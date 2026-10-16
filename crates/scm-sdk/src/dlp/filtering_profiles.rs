use super::{
    DataFilteringProfileListParams, common,
    models::{
        DataFilteringProfileRequest, DataFilteringProfileResponse, PageDataFilteringProfileResponse,
    },
};
use crate::{
    Client, RawRequest, RequestOptions, Result,
    pagination::{CollectOptions, CollectionResult, Items, Pages},
};
use http::Method;
const PATH: &str = "/v2/api/data-filtering-profiles";

#[derive(Clone, Debug)]
pub struct DataFilteringProfilesClient {
    pub(super) client: Client,
}
impl DataFilteringProfilesClient {
    pub async fn list(
        &self,
        params: &DataFilteringProfileListParams,
        options: Option<&RequestOptions>,
    ) -> Result<PageDataFilteringProfileResponse> {
        let mut query = common::query(params.page, params.size, &params.sort)?;
        if let Some(status) = params.status {
            query.push(("status".into(), status.as_str().into()));
        }
        if let Some(name) = &params.name {
            common::filter(name)?;
            query.push(("name".into(), name.clone()));
        }
        self.client
            .send_contract(
                RawRequest::read(PATH)
                    .query(query)
                    .named("dlp.filtering_profiles.list"),
                "PageDataFilteringProfileResponseSchema",
                options,
            )
            .await
    }
    pub async fn get(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<DataFilteringProfileResponse> {
        self.client
            .send_contract(
                RawRequest::read(PATH)
                    .segment(id)?
                    .named("dlp.filtering_profiles.get"),
                "DataFilteringProfileResponseSchema",
                options,
            )
            .await
    }
    pub async fn replace(
        &self,
        id: &str,
        body: &DataFilteringProfileRequest,
        options: Option<&RequestOptions>,
    ) -> Result<DataFilteringProfileResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::PUT, PATH)
                    .segment(id)?
                    .named("dlp.filtering_profiles.replace"),
                body,
                "DataFilteringProfileRequestSchema",
                "DataFilteringProfileResponseSchema",
                options,
            )
            .await
    }
    pub fn list_pages(
        &self,
        params: &DataFilteringProfileListParams,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, DataFilteringProfileResponse, u64>> {
        common::query(params.page, params.size, &params.sort)?;
        if let Some(name) = &params.name {
            common::filter(name)?;
        }
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
        params: &DataFilteringProfileListParams,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Items<'static, DataFilteringProfileResponse, u64>> {
        Ok(self.list_pages(params, max_pages, options)?.items())
    }
    pub async fn list_all(
        &self,
        params: &DataFilteringProfileListParams,
        collect: CollectOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<DataFilteringProfileResponse, u64> {
        match self.list_pages(params, collect.max_pages, options) {
            Ok(pages) => pages.collect(collect).await,
            Err(error) => Err(error.into()),
        }
    }
}
