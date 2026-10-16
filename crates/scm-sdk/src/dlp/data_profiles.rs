use super::{
    DataProfileListParams, common,
    models::{
        AdvancedDataProfileRequest, DataProfilePatchRequest, DataProfileResponse,
        PageDataProfileResponse,
    },
};
use crate::{
    Client, RawRequest, RequestOptions, Result,
    pagination::{CollectOptions, CollectionResult, Items, Pages},
};
use http::Method;
const PATH: &str = "/v2/api/data-profiles";

#[derive(Clone, Debug)]
pub struct DataProfilesClient {
    pub(super) client: Client,
}
impl DataProfilesClient {
    pub async fn list(
        &self,
        params: &DataProfileListParams,
        options: Option<&RequestOptions>,
    ) -> Result<PageDataProfileResponse> {
        let query = common::query(params.page, params.size, &params.sort)?;
        self.client
            .send_contract(
                RawRequest::read(PATH)
                    .query(query)
                    .named("dlp.data_profiles.list"),
                "PageDataProfileResponseSchema",
                options,
            )
            .await
    }
    pub async fn create(
        &self,
        body: &AdvancedDataProfileRequest,
        options: Option<&RequestOptions>,
    ) -> Result<DataProfileResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, PATH).named("dlp.data_profiles.create"),
                body,
                "AdvancedDataProfileRequestSchema",
                "DataProfileResponseSchema",
                options,
            )
            .await
    }
    pub async fn get(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<DataProfileResponse> {
        self.client
            .send_contract(
                RawRequest::read(PATH)
                    .segment(id)?
                    .named("dlp.data_profiles.get"),
                "DataProfileResponseSchema",
                options,
            )
            .await
    }
    pub async fn replace(
        &self,
        id: &str,
        body: &AdvancedDataProfileRequest,
        options: Option<&RequestOptions>,
    ) -> Result<DataProfileResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::PUT, PATH)
                    .segment(id)?
                    .named("dlp.data_profiles.replace"),
                body,
                "AdvancedDataProfileRequestSchema",
                "DataProfileResponseSchema",
                options,
            )
            .await
    }
    pub async fn patch(
        &self,
        id: &str,
        body: &DataProfilePatchRequest,
        options: Option<&RequestOptions>,
    ) -> Result<DataProfileResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::PATCH, PATH)
                    .segment(id)?
                    .content_type("application/merge-patch+json")?
                    .named("dlp.data_profiles.patch"),
                body,
                "DataProfilePatchRequestSchema",
                "DataProfileResponseSchema",
                options,
            )
            .await
    }
    pub fn list_pages(
        &self,
        params: &DataProfileListParams,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, DataProfileResponse, u64>> {
        common::query(params.page, params.size, &params.sort)?;
        let client = self.clone();
        let mut params = params.clone();
        let initial = params.page.unwrap_or(0);
        let size = params.size.unwrap_or(50);
        params.size = Some(size);
        let options = self.client.operation_options(options)?;
        Pages::new(
            initial,
            options,
            self.client.session(),
            max_pages,
            move |page, options| {
                let client = client.clone();
                let mut params = params.clone();
                params.page = Some(page);
                async move {
                    let result = client.list(&params, Some(&options)).await?;
                    common::spring(result.content, result.last, result.total_pages, page, size)
                }
            },
        )
    }
    pub fn list_items(
        &self,
        params: &DataProfileListParams,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Items<'static, DataProfileResponse, u64>> {
        Ok(self.list_pages(params, max_pages, options)?.items())
    }
    pub async fn list_all(
        &self,
        params: &DataProfileListParams,
        collect: CollectOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<DataProfileResponse, u64> {
        match self.list_pages(params, collect.max_pages, options) {
            Ok(pages) => pages.collect(collect).await,
            Err(error) => Err(error.into()),
        }
    }
}
