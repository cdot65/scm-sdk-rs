use super::{PaginationOptions, models::*, next_offset, optional_items, tenant};
use crate::{
    Client, RawRequest, RequestOptions, Result,
    engine::AuthKind,
    pagination::{CollectOptions, CollectionResult, Page, Pages},
};
use http::Method;
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ApiKeyListAllOptions {
    pub limit: u32,
    pub collect: CollectOptions,
}
impl Default for ApiKeyListAllOptions {
    fn default() -> Self {
        Self {
            limit: 100,
            collect: CollectOptions::default(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct ApiKeysClient {
    pub(super) client: Client,
}
impl ApiKeysClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::OAuth)?;
        Ok(Self { client })
    }
    pub async fn list_for_token(
        &self,
        page: PaginationOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ApiKeyListResponse> {
        self.client
            .send_contract(
                RawRequest::read("/v1/mgmt/apikeys")
                    .named("airs.management.api_keys.list_for_token")
                    .query(page.query(false)?),
                "ApiKeyListResponseSchema",
                options,
            )
            .await
    }
    pub async fn create(
        &self,
        body: &ApiKeyCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<ApiKey> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/mgmt/apikey")
                    .named("airs.management.api_keys.create"),
                body,
                "ApiKeyCreateRequestSchema",
                "ApiKeySchema",
                options,
            )
            .await
    }
    pub async fn list(
        &self,
        page: PaginationOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ApiKeyListResponse> {
        self.client
            .send_contract(
                RawRequest::read("/v1/mgmt/apikeys/tsg")
                    .segment(tenant(&self.client)?)?
                    .named("airs.management.api_keys.list")
                    .query(page.query(false)?),
                "ApiKeyListResponseSchema",
                options,
            )
            .await
    }
    pub fn pages(
        &self,
        page: PaginationOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, ApiKey, u64>> {
        page.query(false)?;
        let client = self.clone();
        Pages::new(
            page.offset,
            self.client.operation_options(options)?,
            self.client.session(),
            max_pages,
            move |offset, options| {
                let client = client.clone();
                async move {
                    let response = client
                        .list(PaginationOptions { offset, ..page }, Some(&options))
                        .await?;
                    Ok(Page {
                        items: optional_items(response.api_keys),
                        next: next_offset(response.next_offset)?,
                    })
                }
            },
        )
    }
    pub async fn list_all(
        &self,
        all: ApiKeyListAllOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<ApiKey, u64> {
        self.pages(
            PaginationOptions {
                limit: all.limit,
                ..PaginationOptions::default()
            },
            all.collect.max_pages,
            options,
        )?
        .collect(all.collect)
        .await
    }
    pub async fn delete(
        &self,
        name: &str,
        updated_by: &str,
        options: Option<&RequestOptions>,
    ) -> Result<ApiKeyDeleteResponse> {
        self.client
            .send_contract(
                RawRequest::new(Method::DELETE, "/v1/mgmt/apikey/delete")
                    .segment(name)?
                    .named("airs.management.api_keys.delete")
                    .query([("updated_by".into(), updated_by.into())]),
                "ApiKeyDeleteResponseSchema",
                options,
            )
            .await
    }
    pub async fn regenerate(
        &self,
        id: &str,
        body: &ApiKeyRegenerateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<ApiKey> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/mgmt/apikey/regenerate")
                    .segment(id)?
                    .named("airs.management.api_keys.regenerate"),
                body,
                "ApiKeyRegenerateRequestSchema",
                "ApiKeySchema",
                options,
            )
            .await
    }
}
