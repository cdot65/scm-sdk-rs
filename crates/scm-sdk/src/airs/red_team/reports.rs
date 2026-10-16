use super::{RedTeamListOptions, common, models::*};
use crate::{Client, RawRequest, RequestOptions, Result, data::JsonValue, engine::AuthKind};
use http::Method;
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct AttackListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub attack_status: Option<String>,
    pub compliance: Option<String>,
    pub attack_modality: Option<String>,
    pub status: Option<String>,
    pub severity: Option<String>,
    pub category: Option<String>,
    pub sub_category: Option<String>,
    pub attack_type: Option<String>,
    pub threat: Option<bool>,
}

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct GoalListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub goal_category: Option<String>,
    pub goal_type: Option<String>,
    pub status: Option<String>,
    pub count: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct RedTeamReportsClient {
    pub(super) client: Client,
}
impl RedTeamReportsClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::OAuth)?;
        Ok(Self {
            client: client.with_tsg_header(false)?,
        })
    }
    pub async fn list_attacks(
        &self,
        job_id: &str,
        params: &AttackListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<AttackListResponse> {
        common::uuid(job_id)?;
        let mut query = common::listing(params.skip, params.limit, params.search.as_deref())?;
        for (key, value) in [
            ("attack_status", &params.attack_status),
            ("compliance", &params.compliance),
            ("attack_modality", &params.attack_modality),
            ("status", &params.status),
            ("severity", &params.severity),
            ("category", &params.category),
            ("sub_category", &params.sub_category),
            ("attack_type", &params.attack_type),
        ] {
            common::push_filter(&mut query, key, value.as_deref())?;
        }
        if let Some(value) = params.threat {
            query.push(("threat".into(), value.to_string()));
        }
        self.client
            .send_contract(
                RawRequest::read("/v1/report/static")
                    .segment(job_id)?
                    .segment("list-attacks")?
                    .query(query)
                    .named("airs.red_team.reports.list_attacks"),
                "AttackListResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_attack_detail(
        &self,
        job_id: &str,
        attack_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<AttackDetailResponse> {
        common::uuid(job_id)?;
        common::uuid(attack_id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/report/static")
                    .segment(job_id)?
                    .segment("attack")?
                    .segment(attack_id)?
                    .named("airs.red_team.reports.get_attack_detail"),
                "AttackDetailResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_multi_turn_attack_detail(
        &self,
        job_id: &str,
        attack_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<AttackMultiTurnDetailResponse> {
        common::uuid(job_id)?;
        common::uuid(attack_id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/report/static")
                    .segment(job_id)?
                    .segment("attack-multi-turn")?
                    .segment(attack_id)?
                    .named("airs.red_team.reports.get_multi_turn_attack_detail"),
                "AttackMultiTurnDetailResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_static_report(
        &self,
        job_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<StaticJobReport> {
        common::uuid(job_id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/report/static")
                    .segment(job_id)?
                    .segment("report")?
                    .named("airs.red_team.reports.get_static_report"),
                "StaticJobReportSchema",
                options,
            )
            .await
    }
    pub async fn get_static_runtime_policy(
        &self,
        job_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<RuntimeSecurityProfileResponse> {
        common::uuid(job_id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/report/static")
                    .segment(job_id)?
                    .segment("runtime-policy-config")?
                    .named("airs.red_team.reports.get_static_runtime_policy"),
                "RuntimeSecurityProfileResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_static_remediation(
        &self,
        job_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<RemediationResponse> {
        common::uuid(job_id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/report/static")
                    .segment(job_id)?
                    .segment("remediation")?
                    .named("airs.red_team.reports.get_static_remediation"),
                "RemediationResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_dynamic_report(
        &self,
        job_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<DynamicJobReport> {
        common::uuid(job_id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/report/dynamic")
                    .segment(job_id)?
                    .segment("report")?
                    .named("airs.red_team.reports.get_dynamic_report"),
                "DynamicJobReportSchema",
                options,
            )
            .await
    }
    pub async fn get_dynamic_runtime_policy(
        &self,
        job_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<RuntimeSecurityProfileResponse> {
        common::uuid(job_id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/report/dynamic")
                    .segment(job_id)?
                    .segment("runtime-policy-config")?
                    .named("airs.red_team.reports.get_dynamic_runtime_policy"),
                "RuntimeSecurityProfileResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_dynamic_remediation(
        &self,
        job_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<RemediationResponse> {
        common::uuid(job_id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/report/dynamic")
                    .segment(job_id)?
                    .segment("remediation")?
                    .named("airs.red_team.reports.get_dynamic_remediation"),
                "RemediationResponseSchema",
                options,
            )
            .await
    }
    pub async fn list_goals(
        &self,
        job_id: &str,
        params: &GoalListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GoalListResponse> {
        common::uuid(job_id)?;
        let mut query = common::listing(params.skip, params.limit, params.search.as_deref())?;
        for (key, value) in [
            ("goal_category", &params.goal_category),
            ("goal_type", &params.goal_type),
            ("status", &params.status),
        ] {
            common::push_filter(&mut query, key, value.as_deref())?;
        }
        if let Some(value) = params.count {
            query.push(("count".into(), value.to_string()));
        }
        self.client
            .send_contract(
                RawRequest::read("/v1/report/dynamic")
                    .segment(job_id)?
                    .segment("list-goals")?
                    .query(query)
                    .named("airs.red_team.reports.list_goals"),
                "GoalListResponseSchema",
                options,
            )
            .await
    }
    pub async fn list_goal_streams(
        &self,
        job_id: &str,
        goal_id: &str,
        params: &RedTeamListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<StreamListResponse> {
        common::uuid(job_id)?;
        common::uuid(goal_id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/report/dynamic")
                    .segment(job_id)?
                    .segment("goal")?
                    .segment(goal_id)?
                    .segment("list-streams")?
                    .query(params.query()?)
                    .named("airs.red_team.reports.list_goal_streams"),
                "StreamListResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_stream_detail(
        &self,
        stream_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<StreamDetailResponse> {
        common::uuid(stream_id)?;
        self.client
            .send_contract(
                RawRequest::read("/v1/report/dynamic/stream")
                    .segment(stream_id)?
                    .named("airs.red_team.reports.get_stream_detail"),
                "StreamDetailResponseSchema",
                options,
            )
            .await
    }
    /// Frozen contract returns JSON (which may contain a download URL). It never
    /// follows that URL or changes authentication destinations implicitly.
    pub async fn download_report(
        &self,
        job_id: &str,
        format: &str,
        options: Option<&RequestOptions>,
    ) -> Result<JsonValue> {
        common::uuid(job_id)?;
        common::filter(format)?;
        self.client
            .send_json(
                RawRequest::read("/v1/report")
                    .segment(job_id)?
                    .segment("download")?
                    .query([("file_format".into(), format.into())])
                    .named("airs.red_team.reports.download_report"),
                options,
            )
            .await
    }
    pub async fn generate_partial_report(
        &self,
        job_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<JsonValue> {
        common::uuid(job_id)?;
        self.client
            .send_json(
                RawRequest::new(Method::POST, "/v1/report")
                    .segment(job_id)?
                    .segment("generate-partial-report")?
                    .named("airs.red_team.reports.generate_partial_report"),
                options,
            )
            .await
    }
}

crate::redaction::redacted_debug!(AttackListOptions, GoalListOptions);
