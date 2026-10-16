use super::{models::*, nonblank};
use crate::{
    Client, Error, ErrorKind, RawRequest, RequestOptions, Result,
    data::JsonValue,
    engine::AuthKind,
    validation::{self, Mode},
};
use serde::Serialize;
use serde_json::{Map, Value};
use std::fmt;
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct DashboardAppQuery {
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "appName")]
    pub app_name: String,
    #[serde(rename = "timeInterval", skip_serializing_if = "Option::is_none")]
    pub time_interval: Option<u64>,
    #[serde(rename = "timeUnit", skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<String>,
}
impl DashboardAppQuery {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            app_id: id.into(),
            app_name: name.into(),
            ..Self::default()
        }
    }
}
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct DashboardTimeRangeQuery {
    #[serde(rename = "timeInterval", skip_serializing_if = "Option::is_none")]
    pub time_interval: Option<u64>,
    #[serde(rename = "timeUnit", skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<String>,
}
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct DashboardSessionsOverviewQuery {
    #[serde(rename = "timeInterval", skip_serializing_if = "Option::is_none")]
    pub time_interval: Option<u64>,
    #[serde(rename = "timeUnit", skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
}
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct DashboardSessionQuery {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "appName")]
    pub app_name: String,
    #[serde(rename = "timeInterval", skip_serializing_if = "Option::is_none")]
    pub time_interval: Option<u64>,
    #[serde(rename = "timeUnit", skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
}
impl DashboardSessionQuery {
    pub fn new(
        session: impl Into<String>,
        app: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            session_id: session.into(),
            app_id: app.into(),
            app_name: name.into(),
            ..Self::default()
        }
    }
}
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct DashboardSessionTransactionQuery {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "appName")]
    pub app_name: String,
    #[serde(rename = "scanId")]
    pub scan_id: String,
    #[serde(rename = "scanSubReqId")]
    pub scan_sub_req_id: u64,
    #[serde(rename = "timeInterval", skip_serializing_if = "Option::is_none")]
    pub time_interval: Option<u64>,
    #[serde(rename = "timeUnit", skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<String>,
}
impl DashboardSessionTransactionQuery {
    pub fn new(
        session: impl Into<String>,
        app: impl Into<String>,
        name: impl Into<String>,
        scan: impl Into<String>,
        sub_request: u64,
    ) -> Self {
        Self {
            session_id: session.into(),
            app_id: app.into(),
            app_name: name.into(),
            scan_id: scan.into(),
            scan_sub_req_id: sub_request,
            ..Self::default()
        }
    }
}
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct DashboardScanContentQuery {
    #[serde(rename = "scanId")]
    pub scan_id: String,
    #[serde(rename = "scanSubReqId")]
    pub scan_sub_req_id: u64,
}
impl DashboardScanContentQuery {
    pub fn new(scan: impl Into<String>, sub_request: u64) -> Self {
        Self {
            scan_id: scan.into(),
            scan_sub_req_id: sub_request,
        }
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct DashboardApplicationsOverviewQuery {
    pub time_interval: Option<u64>,
    pub time_unit: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}
macro_rules! redacted_debug{($($ty:ty),+)=>{$(impl fmt::Debug for $ty{fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{f.write_str(concat!(stringify!($ty),"([REDACTED])"))}})+};}
redacted_debug!(
    DashboardAppQuery,
    DashboardTimeRangeQuery,
    DashboardSessionsOverviewQuery,
    DashboardSessionQuery,
    DashboardSessionTransactionQuery,
    DashboardScanContentQuery,
    DashboardApplicationsOverviewQuery
);
#[derive(Clone, Debug)]
pub struct DashboardClient {
    pub(super) client: Client,
}
fn parsed<T: Serialize>(schema: &str, query: &T) -> Result<Map<String, Value>> {
    let bytes = crate::engine::serialize_bounded(query, crate::config::MAX_URL_BYTES)?;
    let value = serde_json::from_slice(&bytes).map_err(|_| Error::new(ErrorKind::Validation))?;
    validation::parse(schema, value, Mode::Request)?
        .as_object()
        .cloned()
        .ok_or_else(|| Error::new(ErrorKind::Validation))
}
fn params(value: &Map<String, Value>, fields: &[(&str, &str)]) -> Result<Vec<(String, String)>> {
    fields
        .iter()
        .map(|(source, destination)| {
            let value = value
                .get(*source)
                .ok_or_else(|| Error::new(ErrorKind::Validation))?;
            let text = match value {
                Value::String(value) => value.clone(),
                Value::Number(value) => value.to_string(),
                _ => return Err(Error::new(ErrorKind::Validation)),
            };
            Ok(((*destination).into(), text))
        })
        .collect()
}
impl DashboardClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::OAuth)?;
        Ok(Self {
            client: client.with_tsg_header(true)?,
        })
    }
    fn application_request(&self, path: &str, query: &DashboardAppQuery) -> Result<RawRequest> {
        let value = parsed("src/management/dashboard.ts#appQuerySchema", query)?;
        Ok(RawRequest::read(path)
            .named("airs.management.dashboard.application")
            .query(params(
                &value,
                &[
                    ("appId", "appid"),
                    ("appName", "appname"),
                    ("timeInterval", "time_interval"),
                    ("timeUnit", "time_unit"),
                ],
            )?))
    }
    fn time_request(
        &self,
        path: &str,
        query: &DashboardTimeRangeQuery,
        session_window: bool,
    ) -> Result<RawRequest> {
        let schema = if session_window {
            "src/management/dashboard.ts#sessionWindowSchema"
        } else {
            "src/management/dashboard.ts#timeRangeSchema"
        };
        let value = parsed(schema, query)?;
        Ok(RawRequest::read(path)
            .named("airs.management.dashboard.time_range")
            .query(params(
                &value,
                &[("timeInterval", "time_interval"), ("timeUnit", "time_unit")],
            )?))
    }
    fn overview_request(&self, query: &DashboardSessionsOverviewQuery) -> Result<RawRequest> {
        let value = parsed(
            "src/management/dashboard.ts#sessionsOverviewQuerySchema",
            query,
        )?;
        Ok(
            RawRequest::read("/v1/mgmt/dashboard/v2/sessions/sessionsoverview")
                .named("airs.management.dashboard.sessions_overview")
                .query(params(
                    &value,
                    &[
                        ("timeInterval", "time_interval"),
                        ("timeUnit", "time_unit"),
                        ("limit", "limit"),
                        ("offset", "offset"),
                    ],
                )?),
        )
    }
    fn session_request(&self, query: &DashboardSessionQuery) -> Result<RawRequest> {
        let value = parsed("src/management/dashboard.ts#sessionQuerySchema", query)?;
        Ok(RawRequest::read("/v1/mgmt/dashboard/v2/sessions/session")
            .named("airs.management.dashboard.session")
            .query(params(
                &value,
                &[
                    ("sessionId", "session_id"),
                    ("appId", "app_id"),
                    ("appName", "app_name"),
                    ("offset", "offset"),
                    ("limit", "limit"),
                    ("timeInterval", "time_interval"),
                    ("timeUnit", "time_unit"),
                ],
            )?))
    }
    fn transaction_request(&self, query: &DashboardSessionTransactionQuery) -> Result<RawRequest> {
        let value = parsed(
            "src/management/dashboard.ts#sessionTransactionQuerySchema",
            query,
        )?;
        Ok(
            RawRequest::read("/v1/mgmt/dashboard/v2/sessions/sessiontransaction")
                .named("airs.management.dashboard.session_transaction")
                .query(params(
                    &value,
                    &[
                        ("sessionId", "session_id"),
                        ("appId", "app_id"),
                        ("appName", "app_name"),
                        ("scanId", "scan_id"),
                        ("scanSubReqId", "scan_sub_req_id"),
                        ("timeInterval", "time_interval"),
                        ("timeUnit", "time_unit"),
                    ],
                )?),
        )
    }
    fn content_request(&self, query: &DashboardScanContentQuery) -> Result<RawRequest> {
        let value = parsed("src/management/dashboard.ts#scanContentQuerySchema", query)?;
        Ok(RawRequest::read("/v1/mgmt/reports/scancontent")
            .named("airs.management.dashboard.scan_content")
            .query(params(
                &value,
                &[("scanId", "scan_id"), ("scanSubReqId", "scan_sub_req_id")],
            )?))
    }
    fn applications_request(
        &self,
        query: &DashboardApplicationsOverviewQuery,
    ) -> Result<RawRequest> {
        let interval = query.time_interval.unwrap_or(30);
        let unit = query.time_unit.as_deref().unwrap_or("days");
        let limit = query.limit.unwrap_or(25);
        let offset = query.offset.unwrap_or(0);
        if ![1, 7, 30, 60].contains(&interval)
            || !["days", "day", "hour"].contains(&unit)
            || limit == 0
            || limit > 9_007_199_254_740_991
            || offset > 9_007_199_254_740_991
        {
            return Err(Error::new(ErrorKind::Validation));
        }
        nonblank(unit)?;
        Ok(
            RawRequest::read("/v1/mgmt/dashboard/v2/apps/applicationsoverview")
                .named("airs.management.dashboard.applications_overview")
                .query(vec![
                    ("time_interval".into(), interval.to_string()),
                    ("time_unit".into(), unit.into()),
                    ("limit".into(), limit.to_string()),
                    ("offset".into(), offset.to_string()),
                ]),
        )
    }
    async fn raw(
        &self,
        request: RawRequest,
        options: Option<&RequestOptions>,
    ) -> Result<Option<JsonValue>> {
        self.client.send_optional_json(request, options).await
    }
    pub async fn application(
        &self,
        query: &DashboardAppQuery,
        options: Option<&RequestOptions>,
    ) -> Result<DashboardApplication> {
        self.client
            .send_contract(
                self.application_request("/v1/mgmt/dashboard/v2/apps/application", query)?
                    .named("airs.management.dashboard.application"),
                "DashboardApplicationSchema",
                options,
            )
            .await
    }
    pub async fn application_raw(
        &self,
        query: &DashboardAppQuery,
        options: Option<&RequestOptions>,
    ) -> Result<Option<JsonValue>> {
        self.raw(
            self.application_request("/v1/mgmt/dashboard/v2/apps/application", query)?
                .named("airs.management.dashboard.application_raw"),
            options,
        )
        .await
    }
    pub async fn application_violation_breakdown(
        &self,
        query: &DashboardAppQuery,
        options: Option<&RequestOptions>,
    ) -> Result<DashboardApplicationViolationBreakdown> {
        self.client
            .send_contract(
                self.application_request(
                    "/v1/mgmt/dashboard/v2/apps/applicationviolationbreakdown",
                    query,
                )?
                .named("airs.management.dashboard.application_violation_breakdown"),
                "DashboardApplicationViolationBreakdownSchema",
                options,
            )
            .await
    }
    pub async fn application_violation_breakdown_raw(
        &self,
        query: &DashboardAppQuery,
        options: Option<&RequestOptions>,
    ) -> Result<Option<JsonValue>> {
        self.raw(
            self.application_request(
                "/v1/mgmt/dashboard/v2/apps/applicationviolationbreakdown",
                query,
            )?
            .named("airs.management.dashboard.application_violation_breakdown_raw"),
            options,
        )
        .await
    }
    pub async fn top_applications_violations(
        &self,
        query: &DashboardTimeRangeQuery,
        options: Option<&RequestOptions>,
    ) -> Result<DashboardTopApplicationsViolations> {
        self.client
            .send_contract(
                self.time_request(
                    "/v1/mgmt/dashboard/v2/apps/topapplicationsviolations",
                    query,
                    false,
                )?
                .named("airs.management.dashboard.top_applications_violations"),
                "DashboardTopApplicationsViolationsSchema",
                options,
            )
            .await
    }
    pub async fn top_applications_violations_raw(
        &self,
        query: &DashboardTimeRangeQuery,
        options: Option<&RequestOptions>,
    ) -> Result<Option<JsonValue>> {
        self.raw(
            self.time_request(
                "/v1/mgmt/dashboard/v2/apps/topapplicationsviolations",
                query,
                false,
            )?
            .named("airs.management.dashboard.top_applications_violations_raw"),
            options,
        )
        .await
    }
    pub async fn applications_violations_trend(
        &self,
        query: &DashboardTimeRangeQuery,
        options: Option<&RequestOptions>,
    ) -> Result<DashboardApplicationsViolationsTrend> {
        self.client
            .send_contract(
                self.time_request(
                    "/v1/mgmt/dashboard/v2/apps/applicationsviolationstrend",
                    query,
                    false,
                )?
                .named("airs.management.dashboard.applications_violations_trend"),
                "DashboardApplicationsViolationsTrendSchema",
                options,
            )
            .await
    }
    pub async fn applications_violations_trend_raw(
        &self,
        query: &DashboardTimeRangeQuery,
        options: Option<&RequestOptions>,
    ) -> Result<Option<JsonValue>> {
        self.raw(
            self.time_request(
                "/v1/mgmt/dashboard/v2/apps/applicationsviolationstrend",
                query,
                false,
            )?
            .named("airs.management.dashboard.applications_violations_trend_raw"),
            options,
        )
        .await
    }
    pub async fn apps_list(
        &self,
        query: &DashboardTimeRangeQuery,
        options: Option<&RequestOptions>,
    ) -> Result<DashboardAppsList> {
        self.client
            .send_contract(
                self.time_request("/v1/mgmt/dashboard/v2/apps/appslist", query, true)?
                    .named("airs.management.dashboard.apps_list"),
                "DashboardAppsListSchema",
                options,
            )
            .await
    }
    pub async fn apps_list_raw(
        &self,
        query: &DashboardTimeRangeQuery,
        options: Option<&RequestOptions>,
    ) -> Result<Option<JsonValue>> {
        self.raw(
            self.time_request("/v1/mgmt/dashboard/v2/apps/appslist", query, true)?
                .named("airs.management.dashboard.apps_list_raw"),
            options,
        )
        .await
    }
    pub async fn sessions_chart(
        &self,
        query: &DashboardTimeRangeQuery,
        options: Option<&RequestOptions>,
    ) -> Result<DashboardSessionsChart> {
        self.client
            .send_contract(
                self.time_request("/v1/mgmt/dashboard/v2/sessions/sessionschart", query, false)?
                    .named("airs.management.dashboard.sessions_chart"),
                "DashboardSessionsChartSchema",
                options,
            )
            .await
    }
    pub async fn sessions_chart_raw(
        &self,
        query: &DashboardTimeRangeQuery,
        options: Option<&RequestOptions>,
    ) -> Result<Option<JsonValue>> {
        self.raw(
            self.time_request("/v1/mgmt/dashboard/v2/sessions/sessionschart", query, false)?
                .named("airs.management.dashboard.sessions_chart_raw"),
            options,
        )
        .await
    }
    pub async fn sessions_overview(
        &self,
        query: &DashboardSessionsOverviewQuery,
        options: Option<&RequestOptions>,
    ) -> Result<DashboardSessionsOverview> {
        self.client
            .send_contract(
                self.overview_request(query)?
                    .named("airs.management.dashboard.sessions_overview"),
                "DashboardSessionsOverviewSchema",
                options,
            )
            .await
    }
    pub async fn sessions_overview_raw(
        &self,
        query: &DashboardSessionsOverviewQuery,
        options: Option<&RequestOptions>,
    ) -> Result<Option<JsonValue>> {
        self.raw(
            self.overview_request(query)?
                .named("airs.management.dashboard.sessions_overview_raw"),
            options,
        )
        .await
    }
    pub async fn session(
        &self,
        query: &DashboardSessionQuery,
        options: Option<&RequestOptions>,
    ) -> Result<DashboardSession> {
        self.client
            .send_contract(
                self.session_request(query)?
                    .named("airs.management.dashboard.session"),
                "DashboardSessionSchema",
                options,
            )
            .await
    }
    pub async fn session_raw(
        &self,
        query: &DashboardSessionQuery,
        options: Option<&RequestOptions>,
    ) -> Result<Option<JsonValue>> {
        self.raw(
            self.session_request(query)?
                .named("airs.management.dashboard.session_raw"),
            options,
        )
        .await
    }
    pub async fn session_transaction(
        &self,
        query: &DashboardSessionTransactionQuery,
        options: Option<&RequestOptions>,
    ) -> Result<DashboardSessionTransaction> {
        self.client
            .send_contract(
                self.transaction_request(query)?
                    .named("airs.management.dashboard.session_transaction"),
                "DashboardSessionTransactionSchema",
                options,
            )
            .await
    }
    pub async fn session_transaction_raw(
        &self,
        query: &DashboardSessionTransactionQuery,
        options: Option<&RequestOptions>,
    ) -> Result<Option<JsonValue>> {
        self.raw(
            self.transaction_request(query)?
                .named("airs.management.dashboard.session_transaction_raw"),
            options,
        )
        .await
    }
    pub async fn scan_content(
        &self,
        query: &DashboardScanContentQuery,
        options: Option<&RequestOptions>,
    ) -> Result<DashboardScanContent> {
        self.client
            .send_contract(
                self.content_request(query)?
                    .named("airs.management.dashboard.scan_content"),
                "DashboardScanContentSchema",
                options,
            )
            .await
    }
    pub async fn scan_content_raw(
        &self,
        query: &DashboardScanContentQuery,
        options: Option<&RequestOptions>,
    ) -> Result<Option<JsonValue>> {
        self.raw(
            self.content_request(query)?
                .named("airs.management.dashboard.scan_content_raw"),
            options,
        )
        .await
    }
    pub async fn applications_overview(
        &self,
        query: &DashboardApplicationsOverviewQuery,
        options: Option<&RequestOptions>,
    ) -> Result<DashboardApplicationsOverview> {
        self.client
            .send_contract(
                self.applications_request(query)?
                    .named("airs.management.dashboard.applications_overview"),
                "DashboardApplicationsOverviewSchema",
                options,
            )
            .await
    }
    pub async fn applications_overview_raw(
        &self,
        query: &DashboardApplicationsOverviewQuery,
        options: Option<&RequestOptions>,
    ) -> Result<Option<JsonValue>> {
        self.raw(
            self.applications_request(query)?
                .named("airs.management.dashboard.applications_overview_raw"),
            options,
        )
        .await
    }
}
