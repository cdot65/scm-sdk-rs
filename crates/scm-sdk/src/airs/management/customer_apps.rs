use super::{PaginationOptions, models::*, next_offset, optional_items, tenant};
use crate::{
    Client, RawRequest, RequestOptions, Result,
    engine::AuthKind,
    pagination::{CollectOptions, CollectionResult, Page, Pages},
};
use http::Method;
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CustomerAppListAllOptions {
    pub limit: u32,
    pub collect: CollectOptions,
}
impl Default for CustomerAppListAllOptions {
    fn default() -> Self {
        Self {
            limit: 100,
            collect: CollectOptions::default(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct CustomerAppsClient {
    pub(super) client: Client,
}
impl CustomerAppsClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::OAuth)?;
        Ok(Self { client })
    }
    pub async fn list_for_token(
        &self,
        page: PaginationOptions,
        options: Option<&RequestOptions>,
    ) -> Result<CustomerAppListResponse> {
        self.client
            .send_contract(
                RawRequest::read("/v1/mgmt/customerapps")
                    .named("airs.management.customer_apps.list_for_token")
                    .query(page.query(false)?),
                "CustomerAppListResponseSchema",
                options,
            )
            .await
    }
    pub async fn get(&self, name: &str, options: Option<&RequestOptions>) -> Result<CustomerApp> {
        self.client
            .send_contract(
                RawRequest::read("/v1/mgmt/customerapp")
                    .named("airs.management.customer_apps.get")
                    .query([("app_name".into(), name.into())]),
                "CustomerAppSchema",
                options,
            )
            .await
    }
    pub async fn list(
        &self,
        page: PaginationOptions,
        options: Option<&RequestOptions>,
    ) -> Result<CustomerAppListResponse> {
        self.client
            .send_contract(
                RawRequest::read("/v1/mgmt/customerapp/tsg")
                    .segment(tenant(&self.client)?)?
                    .named("airs.management.customer_apps.list")
                    .query(page.query(false)?),
                "CustomerAppListResponseSchema",
                options,
            )
            .await
    }
    pub fn pages(
        &self,
        page: PaginationOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, CustomerAppWithKeys, u64>> {
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
                        items: optional_items(response.customer_apps),
                        next: next_offset(response.next_offset)?,
                    })
                }
            },
        )
    }
    pub async fn list_all(
        &self,
        all: CustomerAppListAllOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<CustomerAppWithKeys, u64> {
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
    pub async fn update(
        &self,
        id: &str,
        body: &CustomerApp,
        options: Option<&RequestOptions>,
    ) -> Result<CustomerApp> {
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/v1/mgmt/customerapp")
                    .named("airs.management.customer_apps.update")
                    .query([("customer_app_id".into(), id.into())]),
                body,
                "CustomerAppSchema",
                "CustomerAppSchema",
                options,
            )
            .await
    }
    pub async fn delete(
        &self,
        name: &str,
        updated_by: &str,
        options: Option<&RequestOptions>,
    ) -> Result<CustomerAppDeleteResponse> {
        self.client
            .send_contract(
                RawRequest::new(Method::DELETE, "/v1/mgmt/customerapp")
                    .named("airs.management.customer_apps.delete")
                    .query([
                        ("app_name".into(), name.into()),
                        ("updated_by".into(), updated_by.into()),
                    ]),
                "CustomerAppDeleteResponseSchema",
                options,
            )
            .await
    }
}
