//! Typed target configuration, profiling and Copilot authorization operations.
use super::{common, models::*};
use crate::{
    Client, RawRequest, RequestOptions, Result,
    data::{JsonObject, Optional},
    pagination::{CollectOptions, CollectionResult, Items, Pages},
};
use http::Method;
use std::fmt;
const PATH: &str = "/v1/target";
// The inline getTargetMetadata z.record(z.unknown()) has this exact frozen shape:
// TargetCreateRequestSchema.extra_info -> Optional s012085 -> Nullable s012086
// -> Record s012087 (unconstrained string key s012088, unknown value s012089).
const METADATA_SCHEMA: &str = "s012087";

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct TargetListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub adapter_uuid: Option<String>,
    pub target_type: Option<String>,
    pub status: Option<String>,
}
impl fmt::Debug for TargetListOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetListOptions([REDACTED])")
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct TargetOperationOptions {
    /// Omitted by default. Explicit false and true are transmitted.
    pub validate: Option<bool>,
}
impl fmt::Debug for TargetOperationOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetOperationOptions([REDACTED])")
    }
}
#[derive(Clone, Debug)]
pub struct RedTeamTargetsClient {
    pub(super) client: Client,
}
impl RedTeamTargetsClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(crate::engine::AuthKind::OAuth)?;
        Ok(Self {
            client: client.with_tsg_header(false)?,
        })
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
    fn query(params: &TargetListOptions) -> Result<Vec<(String, String)>> {
        let mut query = common::listing(params.skip, params.limit, params.search.as_deref())?;
        if let Some(value) = &params.adapter_uuid {
            common::uuid(value)?;
            query.push(("adapter_uuid".into(), value.clone()));
        }
        if let Some(value) = &params.target_type {
            common::filter(value)?;
            query.push(("target_type".into(), value.clone()));
        }
        if let Some(value) = &params.status {
            common::filter(value)?;
            query.push(("status".into(), value.clone()));
        }
        Ok(query)
    }
    fn operation(operation: Option<&TargetOperationOptions>) -> Vec<(String, String)> {
        operation
            .and_then(|o| o.validate)
            .map(|v| vec![("validate".into(), v.to_string())])
            .unwrap_or_default()
    }
    pub async fn start_profiling(
        &self,
        uuid: &str,
        options: Option<&RequestOptions>,
    ) -> Result<StartProfilingResponse> {
        common::uuid(uuid)?;
        self.client
            .send_contract(
                RawRequest::new(Method::POST, PATH)
                    .segment(uuid)?
                    .segment("profile")?
                    .named("red_team.targets.start_profiling"),
                "StartProfilingResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_copilot_auth_url(
        &self,
        body: &MSCopilotStudioAuthUrlRequest,
        options: Option<&RequestOptions>,
    ) -> Result<MSCopilotStudioAuthUrlResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/target/ms-copilot-studio/auth-url")
                    .named("red_team.targets.get_copilot_auth_url"),
                body,
                "MSCopilotStudioAuthUrlRequestSchema",
                "MSCopilotStudioAuthUrlResponseSchema",
                options,
            )
            .await
    }
    pub async fn exchange_copilot_token(
        &self,
        body: &MSCopilotStudioTokenRequest,
        options: Option<&RequestOptions>,
    ) -> Result<MSCopilotStudioTokenResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/target/ms-copilot-studio/token")
                    .named("red_team.targets.exchange_copilot_token"),
                body,
                "MSCopilotStudioTokenRequestSchema",
                "MSCopilotStudioTokenResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete_copilot_token(
        &self,
        uuid: &str,
        options: Option<&RequestOptions>,
    ) -> Result<()> {
        common::uuid(uuid)?;
        self.client
            .send_empty(
                RawRequest::new(Method::DELETE, "/v1/target/ms-copilot-studio/token")
                    .segment(uuid)?
                    .named("red_team.targets.delete_copilot_token"),
                options,
            )
            .await
    }
    pub async fn create(
        &self,
        body: &TargetCreateRequest,
        operation: Option<&TargetOperationOptions>,
        options: Option<&RequestOptions>,
    ) -> Result<TargetResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, PATH)
                    .query(Self::operation(operation))
                    .named("red_team.targets.create"),
                body,
                "TargetCreateRequestSchema",
                "TargetResponseSchema",
                options,
            )
            .await
    }
    pub async fn list(
        &self,
        params: &TargetListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<TargetList> {
        self.client
            .send_contract(
                RawRequest::read(PATH)
                    .query(Self::query(params)?)
                    .named("red_team.targets.list"),
                "TargetListSchema",
                options,
            )
            .await
    }
    pub async fn get(
        &self,
        uuid: &str,
        options: Option<&RequestOptions>,
    ) -> Result<TargetResponse> {
        common::uuid(uuid)?;
        self.client
            .send_contract(
                RawRequest::read(PATH)
                    .segment(uuid)?
                    .named("red_team.targets.get"),
                "TargetResponseSchema",
                options,
            )
            .await
    }
    pub async fn update(
        &self,
        uuid: &str,
        body: &TargetUpdateRequest,
        operation: Option<&TargetOperationOptions>,
        options: Option<&RequestOptions>,
    ) -> Result<TargetResponse> {
        common::uuid(uuid)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, PATH)
                    .segment(uuid)?
                    .query(Self::operation(operation))
                    .named("red_team.targets.update"),
                body,
                "TargetUpdateRequestSchema",
                "TargetResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete(
        &self,
        uuid: &str,
        options: Option<&RequestOptions>,
    ) -> Result<Option<BaseResponse>> {
        common::uuid(uuid)?;
        self.client
            .send_optional_contract(
                RawRequest::new(Method::DELETE, PATH)
                    .segment(uuid)?
                    .named("red_team.targets.delete"),
                "BaseResponseSchema",
                options,
            )
            .await
    }
    pub async fn probe(
        &self,
        body: &TargetProbeRequest,
        options: Option<&RequestOptions>,
    ) -> Result<TargetResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/target/probe").named("red_team.targets.probe"),
                body,
                "TargetProbeRequestSchema",
                "TargetResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_profile(
        &self,
        uuid: &str,
        options: Option<&RequestOptions>,
    ) -> Result<TargetProfileResponse> {
        common::uuid(uuid)?;
        self.client
            .send_contract(
                RawRequest::read(PATH)
                    .segment(uuid)?
                    .segment("profile")?
                    .named("red_team.targets.get_profile"),
                "TargetProfileResponseSchema",
                options,
            )
            .await
    }
    pub async fn update_profile(
        &self,
        uuid: &str,
        body: &TargetContextUpdate,
        options: Option<&RequestOptions>,
    ) -> Result<TargetResponse> {
        common::uuid(uuid)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, PATH)
                    .segment(uuid)?
                    .segment("profile")?
                    .named("red_team.targets.update_profile"),
                body,
                "TargetContextUpdateSchema",
                "TargetResponseSchema",
                options,
            )
            .await
    }
    pub async fn validate_auth(
        &self,
        body: &TargetAuthValidationRequest,
        options: Option<&RequestOptions>,
    ) -> Result<TargetAuthValidationResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/target/validate-auth")
                    .named("red_team.targets.validate_auth"),
                body,
                "TargetAuthValidationRequestSchema",
                "TargetAuthValidationResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_target_metadata(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<JsonObject> {
        self.client
            .send_contract(
                RawRequest::read("/v1/template/target-metadata")
                    .named("red_team.targets.get_target_metadata"),
                METADATA_SCHEMA,
                options,
            )
            .await
    }
    pub async fn get_target_templates(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<TargetTemplateCollection> {
        self.client
            .send_contract(
                RawRequest::read("/v1/template/target-templates")
                    .named("red_team.targets.get_target_templates"),
                "TargetTemplateCollectionSchema",
                options,
            )
            .await
    }
    pub fn list_pages(
        &self,
        params: &TargetListOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Pages<'static, TargetListItem, u64>> {
        Self::query(params)?;
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
                    let page = client.list(&params, Some(&options)).await?;
                    let items = match page.data {
                        Optional::Unset => Vec::new(),
                        Optional::Value(items) => items,
                    };
                    common::skip_page(items, page.pagination, skip, limit)
                }
            },
        )
    }
    pub fn list_items(
        &self,
        params: &TargetListOptions,
        max_pages: usize,
        options: Option<&RequestOptions>,
    ) -> Result<Items<'static, TargetListItem, u64>> {
        Ok(self.list_pages(params, max_pages, options)?.items())
    }
    pub async fn list_all(
        &self,
        params: &TargetListOptions,
        collect: CollectOptions,
        options: Option<&RequestOptions>,
    ) -> CollectionResult<TargetListItem, u64> {
        match self.list_pages(params, collect.max_pages, options) {
            Ok(pages) => pages.collect(collect).await,
            Err(error) => Err(error.into()),
        }
    }
}
