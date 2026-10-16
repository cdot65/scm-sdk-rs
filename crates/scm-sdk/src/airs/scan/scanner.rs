use super::{Content, content::validate_content, models::*};
use crate::{
    Client, Endpoint, Error, ErrorKind, RawRequest, RequestOptions, Result, Secret,
    auth::Authentication, engine::AuthKind,
};
use http::Method;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Region {
    Us,
    Eu,
    India,
    Singapore,
}
impl Region {
    pub const fn endpoint(self) -> &'static str {
        match self {
            Self::Us => "https://service.api.aisecurity.paloaltonetworks.com",
            Self::Eu => "https://service-de.api.aisecurity.paloaltonetworks.com",
            Self::India => "https://service-in.api.aisecurity.paloaltonetworks.com",
            Self::Singapore => "https://service-sg.api.aisecurity.paloaltonetworks.com",
        }
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct SyncScanOptions {
    pub tr_id: Option<String>,
    pub session_id: Option<String>,
    pub metadata: Option<Metadata>,
    pub request: Option<RequestOptions>,
}
impl fmt::Debug for SyncScanOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SyncScanOptions([REDACTED])")
    }
}

#[derive(Clone, Debug)]
pub struct Scanner {
    client: Client,
}
impl Scanner {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::Scan)?;
        Ok(Self { client })
    }
    pub fn for_region(
        region: Region,
        api_key: Option<Secret>,
        api_token: Option<Secret>,
    ) -> Result<Self> {
        Self::new(
            Client::builder(Endpoint::parse(region.endpoint())?)
                .authentication(Authentication::scan(api_key, api_token)?)
                .build()?,
        )
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
    pub async fn sync_scan(
        &self,
        profile: &AiProfile,
        content: &Content,
        options: Option<&SyncScanOptions>,
    ) -> Result<ScanResponse> {
        let empty = SyncScanOptions::default();
        let options = options.unwrap_or(&empty);
        #[derive(Serialize)]
        struct Body<'a> {
            ai_profile: &'a AiProfile,
            contents: [&'a ScanRequestContentsInner; 1],
            #[serde(skip_serializing_if = "Option::is_none")]
            tr_id: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            session_id: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            metadata: Option<&'a Metadata>,
        }
        let body = Body {
            ai_profile: profile,
            contents: [content.to_json()],
            tr_id: options.tr_id.as_deref().filter(|v| !v.is_empty()),
            session_id: options.session_id.as_deref().filter(|v| !v.is_empty()),
            metadata: options.metadata.as_ref(),
        };
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/scan/sync/request").named("airs.scan.sync"),
                &body,
                "ScanRequestSchema",
                "ScanResponseSchema",
                options.request.as_ref(),
            )
            .await
    }
    /// Send a complete typed request. Content limits apply to every item, including batches.
    pub async fn scan_request(
        &self,
        request: &ScanRequest,
        options: Option<&RequestOptions>,
    ) -> Result<ScanResponse> {
        for content in &request.contents {
            validate_content(content)?;
        }
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/scan/sync/request").named("airs.scan.sync"),
                request,
                "ScanRequestSchema",
                "ScanResponseSchema",
                options,
            )
            .await
    }
    /// Submit 1–20 objects. Preserve the batch receipt; one scan ID may produce many unordered rows.
    pub async fn async_scan(
        &self,
        requests: &[AsyncScanObject],
        options: Option<&RequestOptions>,
    ) -> Result<AsyncScanResponse> {
        if requests.is_empty() || requests.len() > 20 {
            return Err(Error::new(ErrorKind::Validation));
        }
        for request in requests {
            for content in &request.scan_req.contents {
                validate_content(content)?;
            }
        }
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/scan/async/request").named("airs.scan.async"),
                &requests,
                "AsyncScanObjectSchema[]",
                "AsyncScanResponseSchema",
                options,
            )
            .await
    }
    /// Return rows in server order without sorting, collapsing, or deduplicating.
    pub async fn query_by_scan_ids(
        &self,
        ids: &[String],
        options: Option<&RequestOptions>,
    ) -> Result<Vec<ScanIdResult>> {
        validate_ids(ids, true)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/scan/results")
                    .named("airs.scan.results")
                    .query([("scan_ids".into(), ids.join(","))]),
                "ScanIdResultSchema[]",
                options,
            )
            .await
    }
    pub async fn query_by_report_ids(
        &self,
        ids: &[String],
        options: Option<&RequestOptions>,
    ) -> Result<Vec<ThreatScanReport>> {
        validate_ids(ids, false)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/scan/reports")
                    .named("airs.scan.reports")
                    .query([("report_ids".into(), ids.join(","))]),
                "ThreatScanReportSchema[]",
                options,
            )
            .await
    }
}
fn validate_ids(ids: &[String], uuid: bool) -> Result<()> {
    if ids.is_empty()
        || ids.len() > 5
        || ids.iter().any(|id| {
            id.is_empty()
                || id.len() > 1024
                || id.contains(',')
                || id.chars().any(char::is_control)
                || (uuid && !is_uuid(id))
        })
    {
        return Err(Error::new(ErrorKind::Validation));
    }
    Ok(())
}
pub(crate) fn is_uuid(value: &str) -> bool {
    value.len() == 36
        && value.bytes().enumerate().all(|(i, b)| {
            if [8, 13, 18, 23].contains(&i) {
                b == b'-'
            } else {
                b.is_ascii_hexdigit()
            }
        })
}
