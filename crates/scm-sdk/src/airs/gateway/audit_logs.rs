use super::{common, models::GatewayAuditLogsResponse};
use crate::{Client, RawRequest, RequestOptions, Result, validation::NativeDate};

/// Two explicit JavaScript-Date-equivalent instants; no implicit local timezone.
#[derive(Clone, Copy)]
#[non_exhaustive]
pub struct AIGatewayAuditLogListOptions {
    pub start: NativeDate,
    pub end: NativeDate,
}
impl AIGatewayAuditLogListOptions {
    pub fn new(start: NativeDate, end: NativeDate) -> Self {
        Self { start, end }
    }
}
crate::redaction::redacted_debug!(AIGatewayAuditLogListOptions);

#[derive(Clone, Debug)]
pub struct AIGatewayAuditLogsClient {
    pub(super) client: Client,
}
impl AIGatewayAuditLogsClient {
    pub fn new(client: Client) -> Result<Self> {
        Ok(Self {
            client: common::oauth(client)?,
        })
    }
    /// Return audit bodies faithfully. They may include unredacted credentials.
    pub async fn list(
        &self,
        query: &AIGatewayAuditLogListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayAuditLogsResponse> {
        let query = [
            ("start_time".into(), common::date_iso(&query.start)?),
            ("end_time".into(), common::date_iso(&query.end)?),
        ];
        self.client
            .send_contract(
                RawRequest::read("/audit-logs")
                    .named("airs.gateway.audit_logs.list")
                    .query(query),
                "GatewayAuditLogsResponseSchema",
                options,
            )
            .await
    }
}
