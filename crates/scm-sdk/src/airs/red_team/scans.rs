use super::{RedTeamListOptions, common, models::*};
use crate::{
    Client, RawRequest, RequestOptions, Result,
    engine::AuthKind,
    pagination::{CollectOptions, CollectionResult, Items, Pages},
};
use http::Method;

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct RedTeamScanListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub status: Option<String>,
    pub job_type: Option<String>,
    pub target_id: Option<String>,
}

impl From<RedTeamListOptions> for RedTeamScanListOptions {
    fn from(value: RedTeamListOptions) -> Self {
        Self {
            skip: value.skip,
            limit: value.limit,
            search: value.search,
            ..Self::default()
        }
    }
}
impl RedTeamScanListOptions {
    fn query(&self) -> Result<Vec<(String, String)>> {
        let mut query = common::listing(self.skip, self.limit, self.search.as_deref())?;
        common::push_filter(&mut query, "status", self.status.as_deref())?;
        common::push_filter(&mut query, "job_type", self.job_type.as_deref())?;
        common::push_filter(&mut query, "target_id", self.target_id.as_deref())?;
        Ok(query)
    }
}
#[derive(Clone, Debug)]
pub struct RedTeamScansClient {
    pub(super) client: Client,
}
impl RedTeamScansClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::OAuth)?;
        Ok(Self {
            client: client.with_tsg_header(false)?,
        })
    }
    pub async fn create(
        &self,
        body: &JobCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<JobResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/scan").named("airs.red_team.scans.create"),
                body,
                "vendor:red-team-dp:0.93.0:create_job_v1_scan_post:body",
                "vendor:red-team-dp:0.93.0:create_job_v1_scan_post:response",
                options,
            )
            .await
    }
    pub async fn list(
        &self,
        params: &RedTeamScanListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<JobListResponse> {
        self.client
            .send_contract(
                RawRequest::read("/v1/scan")
                    .query(common::current_query(
                        "list_jobs_v1_scan_get",
                        params.query()?,
                    )?)
                    .named("airs.red_team.scans.list"),
                "vendor:red-team-dp:0.93.0:list_jobs_v1_scan_get:response",
                options,
            )
            .await
    }
    pub async fn get(&self, job_id: &str, options: Option<&RequestOptions>) -> Result<JobResponse> {
        common::uuid(job_id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/scan")
                    .segment(job_id)?
                    .named("airs.red_team.scans.get"),
                "vendor:red-team-dp:0.93.0:get_job_v1_scan__job_id__get:response",
                options,
            )
            .await
    }
    /// Associate a runtime profile with a job. Ambiguous writes are never replayed.
    pub async fn set_runtime_profile(
        &self,
        job_id: &str,
        body: &RuntimeProfileUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<JobResponse> {
        common::uuid(job_id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/v1/scan")
                    .segment(job_id)?
                    .segment("runtime-profile")?
                    .named("airs.red_team.scans.set_runtime_profile"),
                body,
                "vendor:red-team-dp:0.93.0:set_runtime_profile_v1_scan__job_id__runtime_profile_put:body",
                "vendor:red-team-dp:0.93.0:set_runtime_profile_v1_scan__job_id__runtime_profile_put:response",
                options,
            )
            .await
    }
    pub async fn abort(
        &self,
        job_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<JobAbortResponse> {
        common::uuid(job_id)?;
        self.client
            .send_contract(
                RawRequest::new(Method::POST, "/v1/scan")
                    .segment(job_id)?
                    .segment("abort")?
                    .named("airs.red_team.scans.abort"),
                "vendor:red-team-dp:0.93.0:abort_job_v1_scan__job_id__abort_post:response",
                options,
            )
            .await
    }
    pub async fn get_categories(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<CategoryModel>> {
        self.client
            .send_contract(
                RawRequest::read("/v1/categories").named("airs.red_team.scans.get_categories"),
                "vendor:red-team-dp:0.93.0:get_all_categories_v1_categories_get:response",
                options,
            )
            .await
    }
    pub fn list_pages(
        &self,
        params: &RedTeamScanListOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, JobResponse, u64>> {
        common::current_query("list_jobs_v1_scan_get", params.query()?)?;
        let client = self.clone();
        let mut params = params.clone();
        let initial = params.skip.unwrap_or(0);
        let limit = params.limit.unwrap_or(50);
        params.limit = Some(limit);
        Pages::new(
            initial,
            self.client.operation_options(options)?,
            self.client.session(),
            max_pages,
            move |skip, options| {
                let client = client.clone();
                let mut params = params.clone();
                params.skip = Some(skip);
                async move {
                    let page = client.list(&params, Some(&options)).await?;
                    common::skip_page(page.data, page.pagination, skip, limit)
                }
            },
        )
    }
    pub fn list_items(
        &self,
        params: &RedTeamScanListOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Items<'static, JobResponse, u64>> {
        Ok(self.list_pages(params, max_pages, options)?.items())
    }
    /// Explicit bounds, truncation, partial failures and resume positions replace silent caps.
    pub async fn list_all(
        &self,
        params: &RedTeamScanListOptions,
        collect: CollectOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<JobResponse, u64> {
        self.list_pages(params, collect.max_pages, options)?
            .collect(collect)
            .await
    }
}

crate::redaction::redacted_debug!(RedTeamScanListOptions);
