//! Custom prompt sets, prompts and properties on the Red Team management plane.
use super::{common, models::*};
use crate::{
    Client, Error, ErrorKind, RawRequest, RequestOptions, Result,
    data::Optional,
    multipart::Multipart,
    pagination::{CollectOptions, CollectionResult, Items, Pages},
};
use http::Method;
use std::fmt;
const SET_PATH: &str = "/v1/custom-attack/custom-prompt-set";

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct PromptSetListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub language: Option<String>,
    pub status: Option<String>,
    pub active: Option<bool>,
    pub archive: Option<bool>,
}
impl fmt::Debug for PromptSetListOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PromptSetListOptions([REDACTED])")
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct PromptListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub status: Option<String>,
    pub active: Option<bool>,
}
impl fmt::Debug for PromptListOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PromptListOptions([REDACTED])")
    }
}
#[derive(Clone, Debug)]
pub struct RedTeamCustomAttacksClient {
    pub(super) client: Client,
}
impl RedTeamCustomAttacksClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(crate::engine::AuthKind::OAuth)?;
        Ok(Self {
            client: client.with_tsg_header(false)?,
        })
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
    fn set_query(params: &PromptSetListOptions) -> Result<Vec<(String, String)>> {
        let mut query = common::listing(params.skip, params.limit, params.search.as_deref())?;
        if let Some(value) = &params.language {
            common::filter(value)?;
            query.push(("language".into(), value.clone()));
        }
        if let Some(value) = &params.status {
            common::filter(value)?;
            query.push(("status".into(), value.clone()));
        }
        if let Some(value) = params.active {
            query.push(("active".into(), value.to_string()));
        }
        if let Some(value) = params.archive {
            query.push(("archive".into(), value.to_string()));
        }
        Ok(query)
    }
    fn prompt_query(params: &PromptListOptions) -> Result<Vec<(String, String)>> {
        let mut query = common::listing(params.skip, params.limit, params.search.as_deref())?;
        if let Some(value) = &params.status {
            common::filter(value)?;
            query.push(("status".into(), value.clone()));
        }
        if let Some(value) = params.active {
            query.push(("active".into(), value.to_string()));
        }
        Ok(query)
    }
    pub async fn create_prompt_set(
        &self,
        body: &CustomPromptSetCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<CustomPromptSetResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, SET_PATH)
                    .named("red_team.custom_attacks.create_prompt_set"),
                body,
                "CustomPromptSetCreateRequestSchema",
                "CustomPromptSetResponseSchema",
                options,
            )
            .await
    }
    pub async fn list_prompt_sets(
        &self,
        params: &PromptSetListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<CustomPromptSetList> {
        self.client
            .send_contract(
                RawRequest::read("/v1/custom-attack/list-custom-prompt-sets")
                    .query(Self::set_query(params)?)
                    .named("red_team.custom_attacks.list_prompt_sets"),
                "CustomPromptSetListSchema",
                options,
            )
            .await
    }
    pub async fn get_prompt_set(
        &self,
        uuid: &str,
        options: Option<&RequestOptions>,
    ) -> Result<CustomPromptSetResponse> {
        common::uuid(uuid)?;
        self.client
            .send_contract(
                RawRequest::read(SET_PATH)
                    .segment(uuid)?
                    .named("red_team.custom_attacks.get_prompt_set"),
                "CustomPromptSetResponseSchema",
                options,
            )
            .await
    }
    pub async fn update_prompt_set(
        &self,
        uuid: &str,
        body: &CustomPromptSetUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<CustomPromptSetResponse> {
        common::uuid(uuid)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, SET_PATH)
                    .segment(uuid)?
                    .named("red_team.custom_attacks.update_prompt_set"),
                body,
                "CustomPromptSetUpdateRequestSchema",
                "CustomPromptSetResponseSchema",
                options,
            )
            .await
    }
    pub async fn archive_prompt_set(
        &self,
        uuid: &str,
        body: &CustomPromptSetArchiveRequest,
        options: Option<&RequestOptions>,
    ) -> Result<CustomPromptSetResponse> {
        common::uuid(uuid)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, SET_PATH)
                    .segment(uuid)?
                    .segment("archive")?
                    .named("red_team.custom_attacks.archive_prompt_set"),
                body,
                "CustomPromptSetArchiveRequestSchema",
                "CustomPromptSetResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_prompt_set_reference(
        &self,
        uuid: &str,
        options: Option<&RequestOptions>,
    ) -> Result<CustomPromptSetReference> {
        common::uuid(uuid)?;
        self.client
            .send_contract(
                RawRequest::read(SET_PATH)
                    .segment(uuid)?
                    .segment("reference")?
                    .named("red_team.custom_attacks.get_prompt_set_reference"),
                "CustomPromptSetReferenceSchema",
                options,
            )
            .await
    }
    pub async fn get_prompt_set_version_info(
        &self,
        uuid: &str,
        version: Option<&str>,
        options: Option<&RequestOptions>,
    ) -> Result<CustomPromptSetVersionInfo> {
        common::uuid(uuid)?;
        let mut query = Vec::new();
        if let Some(value) = version {
            common::filter(value)?;
            query.push(("version".into(), value.into()));
        }
        self.client
            .send_contract(
                RawRequest::read(SET_PATH)
                    .segment(uuid)?
                    .segment("version-info")?
                    .query(query)
                    .named("red_team.custom_attacks.get_prompt_set_version_info"),
                "CustomPromptSetVersionInfoSchema",
                options,
            )
            .await
    }
    pub async fn list_active_prompt_sets(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<CustomPromptSetListActive> {
        self.client
            .send_contract(
                RawRequest::read("/v1/custom-attack/active-custom-prompt-sets")
                    .named("red_team.custom_attacks.list_active_prompt_sets"),
                "CustomPromptSetListActiveSchema",
                options,
            )
            .await
    }
    pub async fn download_template(
        &self,
        uuid: &str,
        options: Option<&RequestOptions>,
    ) -> Result<String> {
        common::uuid(uuid)?;
        self.client
            .send_text(
                RawRequest::read("/v1/custom-attack/download-template")
                    .segment(uuid)?
                    .named("red_team.custom_attacks.download_template"),
                options,
            )
            .await
    }
    /// Upload caller-owned bytes. The service requires the filename `prompts.csv`.
    pub async fn upload_prompts_csv(
        &self,
        uuid: &str,
        file: &BinaryFile,
        options: Option<&RequestOptions>,
    ) -> Result<BaseResponse> {
        common::uuid(uuid)?;
        let options = self.client.operation_options(options)?;
        let (body, content_type) = options
            .run(&self.client.session(), async {
                file.validate(self.client.limits().max_request_bytes)?;
                let mut form = Multipart::new(self.client.limits().max_request_bytes)?;
                form.part(
                    "file",
                    Some("prompts.csv"),
                    Some(file.content_type().unwrap_or("application/octet-stream")),
                    file.bytes().clone(),
                )?;
                form.finish()
            })
            .await?;
        self.client
            .send_contract(
                RawRequest::new(Method::POST, "/v1/custom-attack/upload-custom-prompts-csv")
                    .query(vec![("prompt_set_uuid".into(), uuid.into())])
                    .body(body, &content_type)?
                    .named("red_team.custom_attacks.upload_prompts_csv"),
                "BaseResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn create_prompt(
        &self,
        body: &CustomPromptCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<CustomPromptResponse> {
        self.client
            .send_model(
                RawRequest::new(
                    Method::POST,
                    "/v1/custom-attack/custom-prompt-set/custom-prompt",
                )
                .named("red_team.custom_attacks.create_prompt"),
                body,
                "CustomPromptCreateRequestSchema",
                "CustomPromptResponseSchema",
                options,
            )
            .await
    }
    pub async fn list_prompts(
        &self,
        uuid: &str,
        params: &PromptListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<CustomPromptList> {
        common::uuid(uuid)?;
        self.client
            .send_contract(
                RawRequest::read(SET_PATH)
                    .segment(uuid)?
                    .segment("list-custom-prompts")?
                    .query(Self::prompt_query(params)?)
                    .named("red_team.custom_attacks.list_prompts"),
                "CustomPromptListSchema",
                options,
            )
            .await
    }
    pub async fn get_prompt(
        &self,
        set_uuid: &str,
        prompt_uuid: &str,
        options: Option<&RequestOptions>,
    ) -> Result<CustomPromptResponse> {
        common::uuid(set_uuid)?;
        common::uuid(prompt_uuid)?;
        self.client
            .send_contract(
                RawRequest::read(SET_PATH)
                    .segment(set_uuid)?
                    .segment("custom-prompt")?
                    .segment(prompt_uuid)?
                    .named("red_team.custom_attacks.get_prompt"),
                "CustomPromptResponseSchema",
                options,
            )
            .await
    }
    pub async fn update_prompt(
        &self,
        set_uuid: &str,
        prompt_uuid: &str,
        body: &CustomPromptUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<CustomPromptResponse> {
        common::uuid(set_uuid)?;
        common::uuid(prompt_uuid)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, SET_PATH)
                    .segment(set_uuid)?
                    .segment("custom-prompt")?
                    .segment(prompt_uuid)?
                    .named("red_team.custom_attacks.update_prompt"),
                body,
                "CustomPromptUpdateRequestSchema",
                "CustomPromptResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete_prompt(
        &self,
        set_uuid: &str,
        prompt_uuid: &str,
        options: Option<&RequestOptions>,
    ) -> Result<Option<BaseResponse>> {
        common::uuid(set_uuid)?;
        common::uuid(prompt_uuid)?;
        self.client
            .send_optional_contract(
                RawRequest::new(Method::DELETE, SET_PATH)
                    .segment(set_uuid)?
                    .segment("custom-prompt")?
                    .segment(prompt_uuid)?
                    .named("red_team.custom_attacks.delete_prompt"),
                "BaseResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_property_names(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<PropertyNamesListResponse> {
        self.client
            .send_contract(
                RawRequest::read("/v1/custom-attack/property-names")
                    .named("red_team.custom_attacks.get_property_names"),
                "PropertyNamesListResponseSchema",
                options,
            )
            .await
    }
    pub async fn create_property_name(
        &self,
        body: &PropertyNameCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<Option<PropertyNameCreateResponse>> {
        self.client
            .send_model_optional(
                RawRequest::new(Method::POST, "/v1/custom-attack/property-names")
                    .named("red_team.custom_attacks.create_property_name"),
                body,
                "PropertyNameCreateRequestSchema",
                "PropertyNameCreateResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_property_values(
        &self,
        name: &str,
        options: Option<&RequestOptions>,
    ) -> Result<PropertyValuesResponse> {
        common::filter(name)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/custom-attack/property-values")
                    .segment(name)?
                    .named("red_team.custom_attacks.get_property_values"),
                "PropertyValuesResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_property_values_multiple(
        &self,
        names: &[String],
        options: Option<&RequestOptions>,
    ) -> Result<PropertyValuesMultipleResponse> {
        if names.len() > 256 {
            return Err(Error::new(ErrorKind::Limit));
        }
        let mut query = Vec::with_capacity(names.len().min(256));
        for value in names {
            common::filter(value)?;
            query.push(("property_names".into(), value.clone()));
        }
        self.client
            .send_contract(
                RawRequest::read("/v1/custom-attack/property-values")
                    .query(query)
                    .named("red_team.custom_attacks.get_property_values_multiple"),
                "PropertyValuesMultipleResponseSchema",
                options,
            )
            .await
    }
    pub async fn create_property_value(
        &self,
        body: &PropertyValueCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<BaseResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/custom-attack/property-values")
                    .named("red_team.custom_attacks.create_property_value"),
                body,
                "PropertyValueCreateRequestSchema",
                "BaseResponseSchema",
                options,
            )
            .await
    }
    pub fn prompt_set_pages(
        &self,
        params: &PromptSetListOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, CustomPromptSetListItem, u64>> {
        Self::set_query(params)?;
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
                    let page = client.list_prompt_sets(&params, Some(&options)).await?;
                    let items = match page.data {
                        Optional::Unset => Vec::new(),
                        Optional::Value(items) => items,
                    };
                    common::skip_page(items, page.pagination, skip, limit)
                }
            },
        )
    }
    pub fn prompt_set_items(
        &self,
        params: &PromptSetListOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Items<'static, CustomPromptSetListItem, u64>> {
        Ok(self.prompt_set_pages(params, max_pages, options)?.items())
    }
    pub async fn list_all_prompt_sets(
        &self,
        params: &PromptSetListOptions,
        collect: CollectOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<CustomPromptSetListItem, u64> {
        match self.prompt_set_pages(params, collect.max_pages, options) {
            Ok(pages) => pages.collect(collect).await,
            Err(error) => Err(error.into()),
        }
    }
    pub fn prompt_pages(
        &self,
        uuid: &str,
        params: &PromptListOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, CustomPromptListItem, u64>> {
        common::uuid(uuid)?;
        Self::prompt_query(params)?;
        let client = self.clone();
        let uuid = uuid.to_owned();
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
                let uuid = uuid.clone();
                let mut params = params.clone();
                params.skip = Some(skip);
                async move {
                    let page = client.list_prompts(&uuid, &params, Some(&options)).await?;
                    let items = match page.data {
                        Optional::Unset => Vec::new(),
                        Optional::Value(items) => items,
                    };
                    common::skip_page(items, page.pagination, skip, limit)
                }
            },
        )
    }
    pub fn prompt_items(
        &self,
        uuid: &str,
        params: &PromptListOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Items<'static, CustomPromptListItem, u64>> {
        Ok(self.prompt_pages(uuid, params, max_pages, options)?.items())
    }
    pub async fn list_all_prompts(
        &self,
        uuid: &str,
        params: &PromptListOptions,
        collect: CollectOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<CustomPromptListItem, u64> {
        match self.prompt_pages(uuid, params, collect.max_pages, options) {
            Ok(pages) => pages.collect(collect).await,
            Err(error) => Err(error.into()),
        }
    }
}
