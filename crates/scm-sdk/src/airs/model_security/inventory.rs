use super::{
    ModelSecurityModelListAllOptions, ModelSecurityModelListOptions,
    ModelSecurityModelVersionFileListAllOptions, ModelSecurityModelVersionFileListOptions,
    ModelSecurityModelVersionListAllOptions, ModelSecurityModelVersionListOptions,
    ModelSecurityRuleListAllOptions, ModelSecurityRuleListOptions, SnapshotVersionListOptions,
    models::*, next_skip, oauth, total, uuid,
};
use crate::{
    Client, RawRequest, RequestOptions, Result,
    pagination::{CollectionResult, Page, Pages},
};
#[derive(Clone, Debug)]
pub struct ModelSecurityModelsClient {
    pub(super) client: Client,
}
impl ModelSecurityModelsClient {
    pub fn new(client: Client) -> Result<Self> {
        Ok(Self {
            client: oauth(client)?,
        })
    }
    pub async fn list_models(
        &self,
        query: &ModelSecurityModelListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ModelList> {
        self.client
            .send_contract(
                RawRequest::read("/v1/models")
                    .named("airs.model_security.models.list_models")
                    .query(query.query()?),
                "ModelListSchema",
                options,
            )
            .await
    }
    pub fn model_pages(
        &self,
        query: ModelSecurityModelListOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, Model, u64>> {
        query.query()?;
        let limit = query.limit.unwrap_or(50);
        let client = self.clone();
        Pages::new(
            query.skip.unwrap_or(0),
            self.client.operation_options(options)?,
            self.client.session(),
            max_pages,
            move |skip, options| {
                let client = client.clone();
                let mut query = query.clone();
                query.skip = Some(skip);
                query.limit = Some(limit);
                async move {
                    let page = client.list_models(&query, Some(&options)).await?;
                    let next = next_skip(
                        skip,
                        page.models.len(),
                        limit,
                        total(&page.pagination.total_items)?,
                    )?;
                    Ok(Page {
                        items: page.models,
                        next,
                    })
                }
            },
        )
    }
    pub async fn list_all_models(
        &self,
        query: ModelSecurityModelListAllOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<Model, u64> {
        let limit = query.limit.unwrap_or(50);
        self.model_pages(query.page(0, limit), query.collect.max_pages, options)?
            .collect(query.collect)
            .await
    }
    pub async fn get_model(&self, id: &str, options: Option<&RequestOptions>) -> Result<Model> {
        uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/models")
                    .segment(id)?
                    .named("airs.model_security.models.get_model"),
                "ModelResponseSchema",
                options,
            )
            .await
    }
    pub async fn list_model_versions(
        &self,
        id: &str,
        query: &ModelSecurityModelVersionListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ModelVersionList> {
        uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/models")
                    .segment(id)?
                    .segment("model-versions")?
                    .named("airs.model_security.models.list_model_versions")
                    .query(query.query()?),
                "ModelVersionListSchema",
                options,
            )
            .await
    }
    pub fn version_pages(
        &self,
        id: &str,
        query: ModelSecurityModelVersionListOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, ModelVersion, u64>> {
        uuid(id)?;
        query.query()?;
        let id = id.to_owned();
        let limit = query.limit.unwrap_or(50);
        let client = self.clone();
        Pages::new(
            query.skip.unwrap_or(0),
            self.client.operation_options(options)?,
            self.client.session(),
            max_pages,
            move |skip, options| {
                let client = client.clone();
                let id = id.clone();
                let mut query = query.clone();
                query.skip = Some(skip);
                query.limit = Some(limit);
                async move {
                    let page = client
                        .list_model_versions(&id, &query, Some(&options))
                        .await?;
                    let next = next_skip(
                        skip,
                        page.model_versions.len(),
                        limit,
                        total(&page.pagination.total_items)?,
                    )?;
                    Ok(Page {
                        items: page.model_versions,
                        next,
                    })
                }
            },
        )
    }
    pub async fn list_all_model_versions(
        &self,
        id: &str,
        query: ModelSecurityModelVersionListAllOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<ModelVersion, u64> {
        let limit = query.limit.unwrap_or(50);
        self.version_pages(id, query.page(0, limit), query.collect.max_pages, options)?
            .collect(query.collect)
            .await
    }
    pub async fn get_model_version(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<ModelVersion> {
        uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/model-versions")
                    .segment(id)?
                    .named("airs.model_security.models.get_model_version"),
                "ModelVersionResponseSchema",
                options,
            )
            .await
    }
    pub async fn list_model_version_files(
        &self,
        id: &str,
        query: &ModelSecurityModelVersionFileListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<FileList> {
        uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/model-versions")
                    .segment(id)?
                    .segment("files")?
                    .named("airs.model_security.models.list_model_version_files")
                    .query(query.query()?),
                "FileListSchema",
                options,
            )
            .await
    }
    pub fn file_pages(
        &self,
        id: &str,
        query: ModelSecurityModelVersionFileListOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, FileResponse, u64>> {
        uuid(id)?;
        query.query()?;
        let id = id.to_owned();
        let limit = query.limit.unwrap_or(50);
        let client = self.clone();
        Pages::new(
            query.skip.unwrap_or(0),
            self.client.operation_options(options)?,
            self.client.session(),
            max_pages,
            move |skip, options| {
                let client = client.clone();
                let id = id.clone();
                let mut query = query.clone();
                query.skip = Some(skip);
                query.limit = Some(limit);
                async move {
                    let page = client
                        .list_model_version_files(&id, &query, Some(&options))
                        .await?;
                    let next = next_skip(
                        skip,
                        page.files.len(),
                        limit,
                        total(&page.pagination.total_items)?,
                    )?;
                    Ok(Page {
                        items: page.files,
                        next,
                    })
                }
            },
        )
    }
    pub async fn list_all_model_version_files(
        &self,
        id: &str,
        query: ModelSecurityModelVersionFileListAllOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<FileResponse, u64> {
        let limit = query.limit.unwrap_or(50);
        self.file_pages(id, query.page(0, limit), query.collect.max_pages, options)?
            .collect(query.collect)
            .await
    }
}
#[derive(Clone, Debug)]
pub struct ModelSecurityRulesClient {
    pub(super) client: Client,
}
impl ModelSecurityRulesClient {
    pub fn new(client: Client) -> Result<Self> {
        Ok(Self {
            client: oauth(client)?,
        })
    }
    pub async fn list(
        &self,
        query: &ModelSecurityRuleListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ListModelSecurityRulesResponse> {
        self.client
            .send_contract(
                RawRequest::read("/v1/security-rules")
                    .named("airs.model_security.rules.list")
                    .query(query.query()?),
                "ListModelSecurityRulesResponseSchema",
                options,
            )
            .await
    }
    pub fn pages(
        &self,
        query: ModelSecurityRuleListOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, ModelSecurityRuleResponse, u64>> {
        query.query()?;
        let limit = query.limit.unwrap_or(50);
        let client = self.clone();
        Pages::new(
            query.skip.unwrap_or(0),
            self.client.operation_options(options)?,
            self.client.session(),
            max_pages,
            move |skip, options| {
                let client = client.clone();
                let mut query = query.clone();
                query.skip = Some(skip);
                query.limit = Some(limit);
                async move {
                    let page = client.list(&query, Some(&options)).await?;
                    let next = next_skip(
                        skip,
                        page.rules.len(),
                        limit,
                        total(&page.pagination.total_items)?,
                    )?;
                    Ok(Page {
                        items: page.rules,
                        next,
                    })
                }
            },
        )
    }
    pub async fn list_all(
        &self,
        query: ModelSecurityRuleListAllOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<ModelSecurityRuleResponse, u64> {
        let limit = query.limit.unwrap_or(50);
        self.pages(query.page(0, limit), query.collect.max_pages, options)?
            .collect(query.collect)
            .await
    }
    pub async fn get(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<ModelSecurityRuleResponse> {
        uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/security-rules")
                    .segment(id)?
                    .named("airs.model_security.rules.get"),
                "ModelSecurityRuleResponseSchema",
                options,
            )
            .await
    }
    pub async fn list_versions(
        &self,
        query: &SnapshotVersionListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<SnapshotVersionListResponse> {
        self.client
            .send_contract(
                RawRequest::read("/v1/security-rules/versions")
                    .named("airs.model_security.rules.list_versions")
                    .query(query.query()?),
                "SnapshotVersionListResponseSchema",
                options,
            )
            .await
    }
}
