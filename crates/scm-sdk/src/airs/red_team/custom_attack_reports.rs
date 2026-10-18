use super::{common, models::*};
use crate::{Client, RawRequest, RequestOptions, Result, engine::AuthKind};
use http::Method;
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct PromptsBySetListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub is_threat: Option<bool>,
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct PromptSetsReportOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub property_filters: Option<String>,
    pub is_threat: Option<bool>,
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct CustomAttacksReportListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub status: Option<String>,
    pub threat: Option<bool>,
    pub prompt_set_id: Option<String>,
    pub property_value: Option<String>,
}
crate::redaction::redacted_debug!(
    PromptsBySetListOptions,
    PromptSetsReportOptions,
    CustomAttacksReportListOptions
);
#[derive(Clone, Debug)]
pub struct RedTeamCustomAttackReportsClient {
    pub(super) client: Client,
}
impl RedTeamCustomAttackReportsClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::OAuth)?;
        Ok(Self {
            client: client.with_tsg_header(false)?,
        })
    }
    pub async fn get_report(
        &self,
        job_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<CustomAttackReportResponse> {
        common::uuid(job_id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/custom-attacks/report")
                    .segment(job_id)?
                    .named("airs.red_team.custom_attack_reports.get_report"),
                "vendor:red-team-dp:0.93.0:get_custom_attack_report_v1_custom_attacks_report__job_id__get:response",
                options,
            )
            .await
    }
    pub async fn get_prompt_sets(
        &self,
        job_id: &str,
        params: &PromptSetsReportOptions,
        options: Option<&RequestOptions>,
    ) -> Result<PromptSetsReportResponse> {
        common::uuid(job_id)?;
        let mut query = common::listing(params.skip, params.limit, params.search.as_deref())?;
        common::push_filter(
            &mut query,
            "property_filters",
            params.property_filters.as_deref(),
        )?;
        if let Some(value) = params.is_threat {
            query.push(("is_threat".into(), value.to_string()));
        }
        self.client
            .send_contract(
                RawRequest::read("/v1/custom-attacks/report")
                    .segment(job_id)?
                    .segment("prompt-sets")?
                    .query(common::current_query("get_prompt_sets_v1_custom_attacks_report__job_id__prompt_sets_get", query)?)
                    .named("airs.red_team.custom_attack_reports.get_prompt_sets"),
                "vendor:red-team-dp:0.93.0:get_prompt_sets_v1_custom_attacks_report__job_id__prompt_sets_get:response",
                options,
            )
            .await
    }
    pub async fn get_prompts_by_set(
        &self,
        job_id: &str,
        prompt_set_id: &str,
        params: &PromptsBySetListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<PromptDetailResponse>> {
        common::uuid(job_id)?;
        common::uuid(prompt_set_id)?;
        let mut query = common::listing(params.skip, params.limit, params.search.as_deref())?;
        if let Some(value) = params.is_threat {
            query.push(("is_threat".into(), value.to_string()));
        }
        self.client
            .send_contract(
                RawRequest::read("/v1/custom-attacks/report")
                    .segment(job_id)?
                    .segment("prompt-set")?
                    .segment(prompt_set_id)?
                    .segment("prompts")?
                    .query(common::current_query("get_prompts_by_set_v1_custom_attacks_report__job_id__prompt_set__prompt_set_id__prompts_get", query)?)
                    .named("airs.red_team.custom_attack_reports.get_prompts_by_set"),
                "vendor:red-team-dp:0.93.0:get_prompts_by_set_v1_custom_attacks_report__job_id__prompt_set__prompt_set_id__prompts_get:response",
                options,
            )
            .await
    }
    pub async fn get_prompt_detail(
        &self,
        job_id: &str,
        prompt_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<PromptDetailResponse> {
        common::uuid(job_id)?;
        common::uuid(prompt_id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/custom-attacks/report")
                    .segment(job_id)?
                    .segment("prompt")?
                    .segment(prompt_id)?
                    .named("airs.red_team.custom_attack_reports.get_prompt_detail"),
                "vendor:red-team-dp:0.93.0:get_prompt_detail_v1_custom_attacks_report__job_id__prompt__prompt_id__get:response",
                options,
            )
            .await
    }
    pub async fn list_custom_attacks(
        &self,
        job_id: &str,
        params: &CustomAttacksReportListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<CustomAttacksListResponse> {
        common::uuid(job_id)?;
        let mut query = common::listing(params.skip, params.limit, params.search.as_deref())?;
        common::push_filter(&mut query, "status", params.status.as_deref())?;
        if let Some(value) = params.threat {
            query.push(("threat".into(), value.to_string()));
        }
        common::push_filter(&mut query, "prompt_set_id", params.prompt_set_id.as_deref())?;
        common::push_filter(
            &mut query,
            "property_value",
            params.property_value.as_deref(),
        )?;
        self.client
            .send_contract(
                RawRequest::read("/v1/custom-attacks/job")
                    .segment(job_id)?
                    .segment("list-custom-attacks")?
                    .query(common::current_query("list_custom_attacks_v1_custom_attacks_job__job_id__list_custom_attacks_get", query)?)
                    .named("airs.red_team.custom_attack_reports.list_custom_attacks"),
                "vendor:red-team-dp:0.93.0:list_custom_attacks_v1_custom_attacks_job__job_id__list_custom_attacks_get:response",
                options,
            )
            .await
    }
    pub async fn get_attack_outputs(
        &self,
        job_id: &str,
        attack_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<CustomAttackOutput>> {
        common::uuid(job_id)?;
        common::uuid(attack_id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/custom-attacks/job")
                    .segment(job_id)?
                    .segment("attack")?
                    .segment(attack_id)?
                    .segment("list-outputs")?
                    .named("airs.red_team.custom_attack_reports.get_attack_outputs"),
                "vendor:red-team-dp:0.93.0:get_attack_outputs_v1_custom_attacks_job__job_id__attack__attack_id__list_outputs_get:response",
                options,
            )
            .await
    }
    pub async fn get_property_stats(
        &self,
        job_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<PropertyStatisticResult>> {
        common::uuid(job_id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/custom-attacks/job")
                    .segment(job_id)?
                    .segment("property-stats")?
                    .named("airs.red_team.custom_attack_reports.get_property_stats"),
                "vendor:red-team-dp:0.93.0:get_property_stats_v1_custom_attacks_job__job_id__property_stats_get:response",
                options,
            )
            .await
    }
    /// Set or clear human verdicts for outputs in this job and custom attack.
    pub async fn override_attack(
        &self,
        job_id: &str,
        attack_id: &str,
        body: &ThreatOverrideRequest,
        options: Option<&RequestOptions>,
    ) -> Result<ThreatOverrideResponse> {
        common::uuid(job_id)?;
        common::uuid(attack_id)?;
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/custom-attacks/job")
                    .segment(job_id)?
                    .segment("attack")?
                    .segment(attack_id)?
                    .segment("override")?
                    .named("airs.red_team.custom_attack_reports.override_attack"),
                body,
                "vendor:red-team-dp:0.93.0:threat_override_custom_attack_v1_custom_attacks_job__job_id__attack__attack_id__override_post:body",
                "vendor:red-team-dp:0.93.0:threat_override_custom_attack_v1_custom_attacks_job__job_id__attack__attack_id__override_post:response",
                options,
            )
            .await
    }
}
