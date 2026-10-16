use super::{
    ModelSecurityGroupListAllOptions, ModelSecurityGroupListOptions,
    ModelSecurityRuleInstanceListOptions, SnapshotVersionListOptions, models::*, next_skip, oauth,
    total, uuid,
};
use crate::{
    Client, RawRequest, RequestOptions, Result,
    pagination::{CollectionResult, Page, Pages},
};
use http::Method;
#[derive(Clone, Debug)]
pub struct ModelSecurityGroupsClient {
    pub(super) client: Client,
}
impl ModelSecurityGroupsClient {
    pub fn new(client: Client) -> Result<Self> {
        Ok(Self {
            client: oauth(client)?,
        })
    }
    pub async fn create(
        &self,
        body: &ModelSecurityGroupCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<ModelSecurityGroupResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/security-groups")
                    .named("airs.model_security.groups.create"),
                body,
                "ModelSecurityGroupCreateRequestSchema",
                "ModelSecurityGroupResponseSchema",
                options,
            )
            .await
    }
    pub async fn list(
        &self,
        query: &ModelSecurityGroupListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ListModelSecurityGroupsResponse> {
        self.client
            .send_contract(
                RawRequest::read("/v1/security-groups")
                    .named("airs.model_security.groups.list")
                    .query(query.query()?),
                "ListModelSecurityGroupsResponseSchema",
                options,
            )
            .await
    }
    pub fn pages(
        &self,
        query: ModelSecurityGroupListOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, ModelSecurityGroupResponse, u64>> {
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
                        page.security_groups.len(),
                        limit,
                        total(&page.pagination.total_items)?,
                    )?;
                    Ok(Page {
                        items: page.security_groups,
                        next,
                    })
                }
            },
        )
    }
    pub async fn list_all(
        &self,
        query: ModelSecurityGroupListAllOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<ModelSecurityGroupResponse, u64> {
        let limit = query.limit.unwrap_or(50);
        self.pages(query.page(0, limit), query.collect.max_pages, options)?
            .collect(query.collect)
            .await
    }
    pub async fn get(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<ModelSecurityGroupResponse> {
        uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/security-groups")
                    .segment(id)?
                    .named("airs.model_security.groups.get"),
                "ModelSecurityGroupResponseSchema",
                options,
            )
            .await
    }
    pub async fn update(
        &self,
        id: &str,
        body: &ModelSecurityGroupUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<ModelSecurityGroupResponse> {
        uuid(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/v1/security-groups")
                    .segment(id)?
                    .named("airs.model_security.groups.update"),
                body,
                "ModelSecurityGroupUpdateRequestSchema",
                "ModelSecurityGroupResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete(&self, id: &str, options: Option<&RequestOptions>) -> Result<()> {
        uuid(id)?;
        self.client
            .send_empty(
                RawRequest::new(Method::DELETE, "/v1/security-groups")
                    .segment(id)?
                    .named("airs.model_security.groups.delete"),
                options,
            )
            .await
    }
    pub async fn list_rule_instances(
        &self,
        id: &str,
        query: &ModelSecurityRuleInstanceListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ListModelSecurityRuleInstancesResponse> {
        uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/security-groups")
                    .segment(id)?
                    .segment("rule-instances")?
                    .named("airs.model_security.groups.list_rule_instances")
                    .query(query.query()?),
                "ListModelSecurityRuleInstancesResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_rule_instance(
        &self,
        group: &str,
        instance: &str,
        options: Option<&RequestOptions>,
    ) -> Result<ModelSecurityRuleInstanceResponse> {
        uuid(group)?;
        uuid(instance)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/security-groups")
                    .segment(group)?
                    .segment("rule-instances")?
                    .segment(instance)?
                    .named("airs.model_security.groups.get_rule_instance"),
                "ModelSecurityRuleInstanceResponseSchema",
                options,
            )
            .await
    }
    pub async fn update_rule_instance(
        &self,
        group: &str,
        instance: &str,
        body: &ModelSecurityRuleInstanceUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<ModelSecurityRuleInstanceResponse> {
        uuid(group)?;
        uuid(instance)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/v1/security-groups")
                    .segment(group)?
                    .segment("rule-instances")?
                    .segment(instance)?
                    .named("airs.model_security.groups.update_rule_instance"),
                body,
                "ModelSecurityRuleInstanceUpdateRequestSchema",
                "ModelSecurityRuleInstanceResponseSchema",
                options,
            )
            .await
    }
    pub async fn list_rule_instance_versions(
        &self,
        id: &str,
        query: &SnapshotVersionListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<SnapshotVersionListResponse> {
        uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/security-groups")
                    .segment(id)?
                    .segment("rule-instances")?
                    .segment("versions")?
                    .named("airs.model_security.groups.list_rule_instance_versions")
                    .query(query.query()?),
                "SnapshotVersionListResponseSchema",
                options,
            )
            .await
    }
}
