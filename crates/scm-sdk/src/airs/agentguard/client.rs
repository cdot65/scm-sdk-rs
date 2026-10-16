use super::models::*;
use crate::{
    Client, Endpoint, Error, ErrorKind, RawRequest, RequestOptions, Result, engine::AuthKind,
};
use serde::Serialize;
use serde_json::Value;

/// Read-only observed contracts, using explicit admitted data and management planes.
#[derive(Clone, Debug)]
pub struct AgentGuardClient {
    data: Client,
    management: Client,
}
impl AgentGuardClient {
    pub fn new(client: Client, data: Endpoint, management: Endpoint) -> Result<Self> {
        client.require_auth(AuthKind::OAuth)?;
        let client = client.with_tsg_header(true)?;
        Ok(Self {
            data: client.with_endpoint(data)?,
            management: client.with_endpoint(management)?,
        })
    }
    pub fn data_client(&self) -> &Client {
        &self.data
    }
    pub fn management_client(&self) -> &Client {
        &self.management
    }
    pub async fn list_scans(
        &self,
        query: &AgentGuardScanListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<AgentGuardScanList> {
        let options = self.data.operation_options(options)?;
        let query = query_values(
            &self.data,
            query,
            "AgentGuardScanListOptionsSchema",
            &[
                "skip",
                "limit",
                "isBackgroundRefresh",
                "start_time",
                "end_time",
            ],
            &options,
        )
        .await?;
        self.data
            .send_contract(
                RawRequest::read("/v1/scans")
                    .named("airs.agentguard.list_scans")
                    .query(query),
                "AgentGuardScanListSchema",
                Some(&options),
            )
            .await
    }
    pub async fn get_scan_stats(
        &self,
        query: &AgentGuardStatsOptions,
        options: Option<&RequestOptions>,
    ) -> Result<AgentGuardScanStats> {
        let options = self.data.operation_options(options)?;
        let mut query = query_values(
            &self.data,
            query,
            "AgentGuardStatsOptionsSchema",
            &["time_period"],
            &options,
        )
        .await?;
        if query.is_empty() {
            query.push(("time_period".into(), "30_DAYS".into()));
        }
        self.data
            .send_contract(
                RawRequest::read("/v1/stats/scans")
                    .named("airs.agentguard.get_scan_stats")
                    .query(query),
                "AgentGuardScanStatsSchema",
                Some(&options),
            )
            .await
    }
    pub async fn list_scan_vulnerabilities(
        &self,
        scan_uuid: &str,
        options: Option<&RequestOptions>,
    ) -> Result<AgentGuardVulnerabilityList> {
        let valid = scan_uuid.len() == 36
            && scan_uuid.bytes().enumerate().all(|(i, b)| {
                if [8, 13, 18, 23].contains(&i) {
                    b == b'-'
                } else {
                    b.is_ascii_hexdigit()
                }
            });
        if !valid {
            return Err(Error::new(ErrorKind::Validation));
        }
        self.data
            .send_contract(
                RawRequest::read("/v1/scans")
                    .segment(scan_uuid)?
                    .segment("vulnerabilities")?
                    .named("airs.agentguard.list_scan_vulnerabilities"),
                "AgentGuardVulnerabilityListSchema",
                options,
            )
            .await
    }
    /// One observed catalog page. Its reported total is a page count, not a catalog total.
    pub async fn list_rules(
        &self,
        query: &AgentGuardPageOptions,
        options: Option<&RequestOptions>,
    ) -> Result<AgentGuardRuleList> {
        let options = self.management.operation_options(options)?;
        let query = query_values(
            &self.management,
            query,
            "AgentGuardPageOptionsSchema",
            &["skip", "limit"],
            &options,
        )
        .await?;
        self.management
            .send_contract(
                RawRequest::read("/v1/rules")
                    .named("airs.agentguard.list_rules")
                    .query(query),
                "AgentGuardRuleListSchema",
                Some(&options),
            )
            .await
    }
}
async fn query_values<T: Serialize>(
    client: &Client,
    input: &T,
    schema: &str,
    keys: &[&str],
    options: &RequestOptions,
) -> Result<Vec<(String, String)>> {
    let bytes = options
        .run(&client.session(), async {
            crate::engine::serialize_bounded(input, crate::config::MAX_URL_BYTES)
        })
        .await?;
    let value: Value =
        serde_json::from_slice(&bytes).map_err(|_| Error::new(ErrorKind::Validation))?;
    let bytes = client.validated_json(&value, schema, options).await?;
    options
        .run(&client.session(), async {
            let value: Value =
                serde_json::from_slice(&bytes).map_err(|_| Error::new(ErrorKind::Validation))?;
            let object = value
                .as_object()
                .ok_or_else(|| Error::new(ErrorKind::Validation))?;
            if object.keys().any(|key| !keys.contains(&key.as_str())) {
                return Err(Error::new(ErrorKind::Validation));
            }
            let mut query = Vec::new();
            let mut bytes = 0usize;
            for key in keys {
                let Some(value) = object.get(*key) else {
                    continue;
                };
                let value = match value {
                    Value::String(v) => v.clone(),
                    Value::Bool(v) => v.to_string(),
                    Value::Number(v) => ryu_js::Buffer::new()
                        .format_finite(
                            v.as_f64()
                                .ok_or_else(|| Error::new(ErrorKind::Validation))?,
                        )
                        .to_owned(),
                    _ => return Err(Error::new(ErrorKind::Validation)),
                };
                bytes = bytes
                    .checked_add(key.len() + value.len())
                    .filter(|n| *n <= crate::config::MAX_URL_BYTES)
                    .ok_or_else(|| Error::new(ErrorKind::Limit))?;
                query.push(((*key).into(), value));
            }
            Ok(query)
        })
        .await
}
