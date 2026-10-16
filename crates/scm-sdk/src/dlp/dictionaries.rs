use super::{
    DictionaryListParams, common,
    models::{
        BinaryFile, DictionaryPatchRequest, DictionaryRequest, DictionaryResponse,
        PageDictionaryResponse,
    },
};
use crate::{
    Client, RawRequest, RequestOptions, Result,
    multipart::Multipart,
    pagination::{CollectOptions, CollectionResult, Items, Pages},
};
use bytes::Bytes;
use http::Method;

const PATH: &str = "/v2/api/dictionaries";
/// Owned native multipart input. The metadata's original_file_name controls the file part name.
#[derive(Clone)]
#[non_exhaustive]
pub struct DictionaryUpload {
    pub metadata: DictionaryRequest,
    pub file: BinaryFile,
    pub include_keywords: Option<bool>,
}
impl DictionaryUpload {
    pub fn new(metadata: DictionaryRequest, file: BinaryFile) -> Self {
        Self {
            metadata,
            file,
            include_keywords: None,
        }
    }
    pub fn text(metadata: DictionaryRequest, text: impl Into<String>) -> Result<Self> {
        Ok(Self::new(
            metadata,
            BinaryFile::new(Bytes::from(text.into()))?.with_content_type("text/plain")?,
        ))
    }
    pub fn include_keywords(mut self, value: bool) -> Self {
        self.include_keywords = Some(value);
        self
    }
}
impl std::fmt::Debug for DictionaryUpload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DictionaryUpload([REDACTED])")
    }
}

#[derive(Clone, Debug)]
pub struct DictionariesClient {
    pub(super) client: Client,
}
impl DictionariesClient {
    pub async fn list(
        &self,
        params: &DictionaryListParams,
        options: Option<&RequestOptions>,
    ) -> Result<PageDictionaryResponse> {
        let mut query = common::query(params.page, params.size, &params.sort)?;
        if let Some(value) = params.keywords {
            query.push(("keywords".into(), value.to_string()));
        }
        self.client
            .send_contract(
                RawRequest::read(PATH)
                    .query(query)
                    .named("dlp.dictionaries.list"),
                "PageDictionaryResponseSchema",
                options,
            )
            .await
    }
    pub async fn get(
        &self,
        id: &str,
        include_keywords: Option<bool>,
        options: Option<&RequestOptions>,
    ) -> Result<DictionaryResponse> {
        let query = include_keywords.map(|value| ("keywords".into(), value.to_string()));
        self.client
            .send_contract(
                RawRequest::read(PATH)
                    .segment(id)?
                    .query(query)
                    .named("dlp.dictionaries.get"),
                "DictionaryResponseSchema",
                options,
            )
            .await
    }
    async fn upload_request(
        &self,
        request: RawRequest,
        upload: &DictionaryUpload,
        options: &RequestOptions,
    ) -> Result<RawRequest> {
        let limit = self.client.limits().max_request_bytes;
        upload.file.validate(limit)?;
        let metadata = self
            .client
            .validated_json(&upload.metadata, "DictionaryRequestSchema", options)
            .await?;
        let mut multipart = Multipart::new(limit)?;
        multipart.part(
            "json",
            Some("metadata.json"),
            Some("application/json"),
            metadata,
        )?;
        let file = upload
            .file
            .clone()
            .with_filename(upload.metadata.original_file_name.clone())?;
        multipart.file("file", &file)?;
        let (body, content_type) = multipart.finish()?;
        request
            .query(
                upload
                    .include_keywords
                    .map(|value| ("keywords".into(), value.to_string())),
            )
            .body(body, &content_type)
    }
    pub async fn create(
        &self,
        upload: &DictionaryUpload,
        options: Option<&RequestOptions>,
    ) -> Result<DictionaryResponse> {
        let options = self.client.operation_options(options)?;
        let request = self
            .upload_request(
                RawRequest::new(Method::POST, PATH).named("dlp.dictionaries.create"),
                upload,
                &options,
            )
            .await?;
        self.client
            .send_contract(request, "DictionaryResponseSchema", Some(&options))
            .await
    }
    /// PUT may return a resource or a successful empty (204) response.
    pub async fn replace(
        &self,
        id: &str,
        upload: &DictionaryUpload,
        options: Option<&RequestOptions>,
    ) -> Result<Option<DictionaryResponse>> {
        let request = RawRequest::new(Method::PUT, PATH)
            .segment(id)?
            .named("dlp.dictionaries.replace");
        let options = self.client.operation_options(options)?;
        let request = self.upload_request(request, upload, &options).await?;
        self.client
            .send_optional_contract(request, "DictionaryResponseSchema", Some(&options))
            .await
    }
    pub async fn patch(
        &self,
        id: &str,
        body: &DictionaryPatchRequest,
        options: Option<&RequestOptions>,
    ) -> Result<DictionaryResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::PATCH, PATH)
                    .segment(id)?
                    .content_type("application/merge-patch+json")?
                    .named("dlp.dictionaries.patch"),
                body,
                "DictionaryPatchRequestSchema",
                "DictionaryResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete(&self, id: &str, options: Option<&RequestOptions>) -> Result<()> {
        self.client
            .send_empty(
                RawRequest::new(Method::DELETE, PATH)
                    .segment(id)?
                    .named("dlp.dictionaries.delete"),
                options,
            )
            .await
    }
    pub fn list_pages(
        &self,
        params: &DictionaryListParams,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, DictionaryResponse, u64>> {
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
        params: &DictionaryListParams,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Items<'static, DictionaryResponse, u64>> {
        Ok(self.list_pages(params, max_pages, options)?.items())
    }
    pub async fn list_all(
        &self,
        params: &DictionaryListParams,
        collect: CollectOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<DictionaryResponse, u64> {
        match self.list_pages(params, collect.max_pages, options) {
            Ok(pages) => pages.collect(collect).await,
            Err(error) => Err(error.into()),
        }
    }
}
