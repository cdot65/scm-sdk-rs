#![warn(missing_docs)]

use super::{Content, content::validate_content, models::*};
use crate::{
    Client, Endpoint, Error, ErrorKind, RawRequest, RequestOptions, Result, Secret,
    auth::Authentication, engine::AuthKind,
};
use http::Method;
use serde::Serialize;
use std::fmt;

/// Regional AIRS scan service bases; choosing a region does not select credentials.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Region {
    /// United States scan service.
    Us,
    /// European scan service.
    Eu,
    /// India scan service.
    India,
    /// Singapore scan service.
    Singapore,
}
impl Region {
    /// Return the HTTPS base used by [`Scanner::for_region`].
    pub const fn endpoint(self) -> &'static str {
        match self {
            Self::Us => "https://service.api.aisecurity.paloaltonetworks.com",
            Self::Eu => "https://service-de.api.aisecurity.paloaltonetworks.com",
            Self::India => "https://service-in.api.aisecurity.paloaltonetworks.com",
            Self::Singapore => "https://service-sg.api.aisecurity.paloaltonetworks.com",
        }
    }
}
/// Optional correlation metadata and request budget for one synchronous scan.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct SyncScanOptions {
    /// Caller transaction identifier; an empty string is omitted.
    pub tr_id: Option<String>,
    /// Caller session identifier; an empty string is omitted.
    pub session_id: Option<String>,
    /// Typed application metadata sent with the scan request.
    pub metadata: Option<Metadata>,
    /// Per-call deadline, cancellation and retry settings.
    pub request: Option<RequestOptions>,
}
impl fmt::Debug for SyncScanOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SyncScanOptions([REDACTED])")
    }
}

/// Typed synchronous scans, asynchronous batches and correlated result queries.
///
/// Enable `scan`; the application owns credentials, its async runtime and policy
/// decisions based on returned actions. Unfamiliar actions require explicit handling.
#[derive(Clone, Debug)]
pub struct Scanner {
    client: Client,
}
impl Scanner {
    /// Require scan authentication while retaining the caller's transport and session.
    ///
    /// The configured client supplies the endpoint, credentials and destination policy.
    /// Construction performs no request and reads no environment variables.
    ///
    /// ```no_run
    /// use scm_sdk::{Client, Result, airs::scan::{Content, Scanner, models::AiProfile}};
    ///
    /// // Poll this future from the application's runtime.
    /// async fn scan_prompt(configured: Client, profile: &AiProfile) -> Result<()> {
    ///     let scanner = Scanner::new(configured)?;
    ///     let content = Content::builder().prompt("A harmless example prompt").build()?;
    ///     let response = scanner.sync_scan(profile, &content, None).await?;
    ///     let _action = &response.action; // The application decides how to enforce it.
    ///     Ok(())
    /// }
    /// ```
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::Scan)?;
        Ok(Self { client })
    }
    /// Build a regional scanner with explicit scan credentials and the default HTTP transport.
    ///
    /// Requires `reqwest-transport`. For an injected transport or custom destination
    /// policy, configure a [`Client`] and call [`Scanner::new`] instead.
    ///
    /// ```no_run
    /// use scm_sdk::{Result, Secret, airs::scan::{Region, Scanner}};
    /// fn regional_scanner(api_key: Secret) -> Result<Scanner> {
    ///     Scanner::for_region(Region::Eu, Some(api_key), None)
    /// }
    /// ```
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
    /// Borrow the underlying client, including its shared session cancellation handle.
    pub fn client(&self) -> &Client {
        &self.client
    }
    /// Scan one validated content object with a profile and optional correlation metadata.
    ///
    /// Request validation finishes before authentication. Submissions do not replay
    /// automatically; inspect the error's remote outcome before reconciling a failure.
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
    /// Send a complete typed synchronous request, validating every content item's limits.
    ///
    /// Use this form when the request already contains profile and correlation fields.
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
    /// Submit 1–20 objects and return the asynchronous batch receipt without polling.
    ///
    /// Preserve the receipt; one scan ID may produce many unordered result rows.
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
    /// Query 1–5 scan UUIDs and return result rows in server order.
    ///
    /// Input order and duplicate IDs are preserved; results are not sorted or deduplicated.
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
    /// Query 1–5 report identifiers and return threat reports in server order.
    ///
    /// Report IDs may be opaque strings, but commas and control characters are rejected.
    /// Query encoding preserves the supplied order and duplicate identifiers.
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
    crate::data::is_uuid(value)
}
