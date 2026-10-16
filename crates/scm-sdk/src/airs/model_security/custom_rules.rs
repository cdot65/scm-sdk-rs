use super::{
    CustomRuleListAllOptions, CustomRuleListOptions, CustomRuleSecurityGroupListOptions,
    SnapshotVersionListAllOptions, SnapshotVersionListOptions, models::*, next_skip, oauth, total,
    uuid,
};
use crate::{
    Client, RawRequest, RequestOptions, Result,
    data::Patch,
    pagination::{CollectionResult, Page, Pages},
};
use http::Method;
#[derive(Clone, Debug)]
pub struct ModelSecurityCustomRulesClient {
    pub(super) client: Client,
}
impl ModelSecurityCustomRulesClient {
    pub fn new(client: Client) -> Result<Self> {
        Ok(Self {
            client: oauth(client)?,
        })
    }
    pub async fn list(
        &self,
        query: &CustomRuleListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ListCustomRulesResponse> {
        self.client
            .send_contract(
                RawRequest::read("/v1/custom-rules")
                    .named("airs.model_security.custom_rules.list")
                    .query(query.query()?),
                "ListCustomRulesResponseSchema",
                options,
            )
            .await
    }
    pub fn pages(
        &self,
        query: CustomRuleListOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, CustomRuleListItem, u64>> {
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
                        page.custom_rules.len(),
                        limit,
                        total(&page.pagination.total_items)?,
                    )?;
                    Ok(Page {
                        items: page.custom_rules,
                        next,
                    })
                }
            },
        )
    }
    pub async fn list_all(
        &self,
        query: CustomRuleListAllOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<CustomRuleListItem, u64> {
        let limit = query.limit.unwrap_or(50);
        self.pages(query.page(0, limit), query.collect.max_pages, options)?
            .collect(query.collect)
            .await
    }
    pub async fn create(
        &self,
        body: &CustomRuleCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<CustomRuleResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/custom-rules")
                    .named("airs.model_security.custom_rules.create"),
                body,
                "CustomRuleCreateRequestSchema",
                "CustomRuleResponseSchema",
                options,
            )
            .await
    }
    pub async fn get(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<CustomRuleResponse> {
        uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/custom-rules")
                    .segment(id)?
                    .named("airs.model_security.custom_rules.get"),
                "CustomRuleResponseSchema",
                options,
            )
            .await
    }
    pub async fn update(
        &self,
        id: &str,
        body: &CustomRuleUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<CustomRuleResponse> {
        uuid(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/v1/custom-rules")
                    .segment(id)?
                    .named("airs.model_security.custom_rules.update"),
                body,
                "CustomRuleUpdateRequestSchema",
                "CustomRuleResponseSchema",
                options,
            )
            .await
    }
    pub async fn archive(&self, id: &str, options: Option<&RequestOptions>) -> Result<()> {
        uuid(id)?;
        self.client
            .send_empty(
                RawRequest::new(Method::POST, "/v1/custom-rules")
                    .segment(id)?
                    .segment("archive")?
                    .named("airs.model_security.custom_rules.archive"),
                options,
            )
            .await
    }
    pub async fn unarchive(&self, id: &str, options: Option<&RequestOptions>) -> Result<()> {
        uuid(id)?;
        self.client
            .send_empty(
                RawRequest::new(Method::POST, "/v1/custom-rules")
                    .segment(id)?
                    .segment("unarchive")?
                    .named("airs.model_security.custom_rules.unarchive"),
                options,
            )
            .await
    }
    pub async fn list_security_groups(
        &self,
        id: &str,
        query: &CustomRuleSecurityGroupListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ListCustomRuleSecurityGroupsResponse> {
        uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/custom-rules")
                    .segment(id)?
                    .segment("security-groups")?
                    .named("airs.model_security.custom_rules.list_security_groups")
                    .query(query.query()?),
                "ListCustomRuleSecurityGroupsResponseSchema",
                options,
            )
            .await
    }
    /// HTTP 207 is successful transport; inspect every returned assignment status.
    pub async fn assign_security_groups(
        &self,
        id: &str,
        body: &BatchAssignCustomRuleRequest,
        options: Option<&RequestOptions>,
    ) -> Result<BatchAssignCustomRuleResponse> {
        uuid(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/custom-rules")
                    .segment(id)?
                    .segment("security-groups")?
                    .named("airs.model_security.custom_rules.assign_security_groups"),
                body,
                "BatchAssignCustomRuleRequestSchema",
                "BatchAssignCustomRuleResponseSchema",
                options,
            )
            .await
    }
    pub async fn remove_assignment(
        &self,
        id: &str,
        group: &str,
        options: Option<&RequestOptions>,
    ) -> Result<()> {
        uuid(id)?;
        uuid(group)?;
        self.client
            .send_empty(
                RawRequest::new(Method::DELETE, "/v1/custom-rules")
                    .segment(id)?
                    .segment("security-groups")?
                    .segment(group)?
                    .named("airs.model_security.custom_rules.remove_assignment"),
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
                RawRequest::read("/v1/custom-rules/versions")
                    .named("airs.model_security.custom_rules.list_versions")
                    .query(query.query()?),
                "SnapshotVersionListResponseSchema",
                options,
            )
            .await
    }
    pub fn version_pages(
        &self,
        query: SnapshotVersionListOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, SnapshotVersion, Option<String>>> {
        query.query()?;
        let client = self.clone();
        Pages::new(
            query.next_token.clone(),
            self.client.operation_options(options)?,
            self.client.session(),
            max_pages,
            move |next_token, options| {
                let client = client.clone();
                let query = SnapshotVersionListOptions {
                    limit: query.limit,
                    next_token,
                };
                async move {
                    let page = client.list_versions(&query, Some(&options)).await?;
                    let next = match page.pagination.next_token {
                        Patch::Unset | Patch::Null => None,
                        Patch::Value(token) => Some(Some(token)),
                    };
                    Ok(Page {
                        items: page.versions,
                        next,
                    })
                }
            },
        )
    }
    pub async fn list_all_versions(
        &self,
        query: SnapshotVersionListAllOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<SnapshotVersion, Option<String>> {
        self.version_pages(
            SnapshotVersionListOptions {
                limit: query.limit,
                next_token: query.next_token,
            },
            query.collect.max_pages,
            options,
        )?
        .collect(query.collect)
        .await
    }
}
