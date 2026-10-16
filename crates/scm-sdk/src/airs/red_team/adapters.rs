//! Custom target adapters. Scripts and variables are data sent to the service.
use super::{
    common,
    models::{
        AdapterConfigResponse, AdapterCreateRequest, AdapterList, AdapterListItem, AdapterResponse,
        AdapterUpdateRequest, AdapterValidateRequest, AdapterValidateResponse, BaseResponse,
    },
};
use crate::{
    Client, RawRequest, RequestOptions, Result,
    data::Optional,
    pagination::{CollectOptions, CollectionResult, Items, Pages},
};
use http::Method;
use std::fmt;
const PATH: &str = "/v1/adapters";

#[derive(Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AdapterStatusFilter {
    Draft,
    Active,
}
impl AdapterStatusFilter {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "DRAFT",
            Self::Active => "ACTIVE",
        }
    }
}
impl fmt::Debug for AdapterStatusFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AdapterStatusFilter([REDACTED])")
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct AdapterListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub status: Option<AdapterStatusFilter>,
    pub include_target_count: Option<bool>,
}
impl fmt::Debug for AdapterListOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AdapterListOptions([REDACTED])")
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct AdapterOperationOptions {
    /// The frozen save contract defaults to true when omitted.
    pub validate: Option<bool>,
}
impl fmt::Debug for AdapterOperationOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AdapterOperationOptions([REDACTED])")
    }
}
#[derive(Clone, Debug)]
pub struct RedTeamAdaptersClient {
    pub(super) client: Client,
}
impl RedTeamAdaptersClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(crate::engine::AuthKind::OAuth)?;
        Ok(Self {
            client: client.with_tsg_header(false)?,
        })
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
    fn query(params: &AdapterListOptions) -> Result<Vec<(String, String)>> {
        let mut query = common::listing(params.skip, params.limit, params.search.as_deref())?;
        if let Some(value) = params.status {
            query.push(("status".into(), value.as_str().into()));
        }
        if let Some(value) = params.include_target_count {
            query.push(("include_target_count".into(), value.to_string()));
        }
        Ok(query)
    }
    pub async fn get_config(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<AdapterConfigResponse> {
        self.client
            .send_contract(
                RawRequest::read("/v1/adapters/config").named("red_team.adapters.get_config"),
                "AdapterConfigResponseSchema",
                options,
            )
            .await
    }
    pub async fn create(
        &self,
        body: &AdapterCreateRequest,
        operation: Option<&AdapterOperationOptions>,
        options: Option<&RequestOptions>,
    ) -> Result<AdapterResponse> {
        let validate = operation.and_then(|o| o.validate).unwrap_or(true);
        self.client
            .send_model(
                RawRequest::new(Method::POST, PATH)
                    .query(vec![("validate".into(), validate.to_string())])
                    .named("red_team.adapters.create"),
                body,
                "AdapterCreateRequestSchema",
                "AdapterResponseSchema",
                options,
            )
            .await
    }
    pub async fn list(
        &self,
        params: &AdapterListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<AdapterList> {
        self.client
            .send_contract(
                RawRequest::read(PATH)
                    .query(Self::query(params)?)
                    .named("red_team.adapters.list"),
                "AdapterListSchema",
                options,
            )
            .await
    }
    pub async fn get(
        &self,
        uuid: &str,
        options: Option<&RequestOptions>,
    ) -> Result<AdapterResponse> {
        common::uuid(uuid)?;
        self.client
            .send_contract(
                RawRequest::read(PATH)
                    .segment(uuid)?
                    .named("red_team.adapters.get"),
                "AdapterResponseSchema",
                options,
            )
            .await
    }
    /// Full replacement. Missing variable keys delete them; null preserves a stored secret.
    pub async fn update(
        &self,
        uuid: &str,
        body: &AdapterUpdateRequest,
        operation: Option<&AdapterOperationOptions>,
        options: Option<&RequestOptions>,
    ) -> Result<AdapterResponse> {
        common::uuid(uuid)?;
        let validate = operation.and_then(|o| o.validate).unwrap_or(true);
        self.client
            .send_model(
                RawRequest::new(Method::PUT, PATH)
                    .segment(uuid)?
                    .query(vec![("validate".into(), validate.to_string())])
                    .named("red_team.adapters.update"),
                body,
                "AdapterUpdateRequestSchema",
                "AdapterResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete(
        &self,
        uuid: &str,
        options: Option<&RequestOptions>,
    ) -> Result<Option<BaseResponse>> {
        common::uuid(uuid)?;
        self.client
            .send_optional_contract(
                RawRequest::new(Method::DELETE, PATH)
                    .segment(uuid)?
                    .named("red_team.adapters.delete"),
                "BaseResponseSchema",
                options,
            )
            .await
    }
    pub async fn validate(
        &self,
        body: &AdapterValidateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<AdapterValidateResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/adapters/validate")
                    .named("red_team.adapters.validate"),
                body,
                "AdapterValidateRequestSchema",
                "AdapterValidateResponseSchema",
                options,
            )
            .await
    }
    pub fn list_pages(
        &self,
        params: &AdapterListOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, AdapterListItem, u64>> {
        Self::query(params)?;
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
                    let items = match page.data {
                        Optional::Unset => Vec::new(),
                        Optional::Value(items) => items,
                    };
                    common::skip_page(items, page.pagination, skip, limit)
                }
            },
        )
    }
    pub fn list_items(
        &self,
        params: &AdapterListOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Items<'static, AdapterListItem, u64>> {
        Ok(self.list_pages(params, max_pages, options)?.items())
    }
    pub async fn list_all(
        &self,
        params: &AdapterListOptions,
        collect: CollectOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<AdapterListItem, u64> {
        match self.list_pages(params, collect.max_pages, options) {
            Ok(pages) => pages.collect(collect).await,
            Err(error) => Err(error.into()),
        }
    }
}
