use super::{
    DataPatternListParams, common,
    models::{
        DataPatternPatchRequest, DataPatternRequest, DataPatternResponse, PageDataPatternResponse,
    },
};
use crate::{
    Client, RawRequest, RequestOptions, Result,
    pagination::{CollectOptions, CollectionResult, Items, Pages},
};
use http::Method;
const PATH: &str = "/v2/api/data-patterns";

#[derive(Clone, Debug)]
pub struct DataPatternsClient {
    pub(super) client: Client,
}
impl DataPatternsClient {
    pub async fn list(
        &self,
        params: &DataPatternListParams,
        options: Option<&RequestOptions>,
    ) -> Result<PageDataPatternResponse> {
        let query = common::query(params.page, params.size, &params.sort)?;
        self.client
            .send_contract(
                RawRequest::read(PATH)
                    .query(query)
                    .named("dlp.data_patterns.list"),
                "PageDataPatternResponseSchema",
                options,
            )
            .await
    }
    pub async fn create(
        &self,
        body: &DataPatternRequest,
        options: Option<&RequestOptions>,
    ) -> Result<DataPatternResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, PATH).named("dlp.data_patterns.create"),
                body,
                "DataPatternRequestSchema",
                "DataPatternResponseSchema",
                options,
            )
            .await
    }
    pub async fn get(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<DataPatternResponse> {
        self.client
            .send_contract(
                RawRequest::read(PATH)
                    .segment(id)?
                    .named("dlp.data_patterns.get"),
                "DataPatternResponseSchema",
                options,
            )
            .await
    }
    pub async fn replace(
        &self,
        id: &str,
        body: &DataPatternRequest,
        options: Option<&RequestOptions>,
    ) -> Result<DataPatternResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::PUT, PATH)
                    .segment(id)?
                    .named("dlp.data_patterns.replace"),
                body,
                "DataPatternRequestSchema",
                "DataPatternResponseSchema",
                options,
            )
            .await
    }
    pub async fn patch(
        &self,
        id: &str,
        body: &DataPatternPatchRequest,
        options: Option<&RequestOptions>,
    ) -> Result<DataPatternResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::PATCH, PATH)
                    .segment(id)?
                    .content_type("application/merge-patch+json")?
                    .named("dlp.data_patterns.patch"),
                body,
                "DataPatternPatchRequestSchema",
                "DataPatternResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete(&self, id: &str, options: Option<&RequestOptions>) -> Result<()> {
        self.client
            .send_empty(
                RawRequest::new(Method::DELETE, PATH)
                    .segment(id)?
                    .named("dlp.data_patterns.delete"),
                options,
            )
            .await
    }
    pub fn list_pages(
        &self,
        params: &DataPatternListParams,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, DataPatternResponse, u64>> {
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
        params: &DataPatternListParams,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Items<'static, DataPatternResponse, u64>> {
        Ok(self.list_pages(params, max_pages, options)?.items())
    }
    /// Collect under explicit item/byte/page caps; incomplete results retain a resume position.
    pub async fn list_all(
        &self,
        params: &DataPatternListParams,
        collect: CollectOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<DataPatternResponse, u64> {
        match self.list_pages(params, collect.max_pages, options) {
            Ok(pages) => pages.collect(collect).await,
            Err(error) => Err(error.into()),
        }
    }
}
