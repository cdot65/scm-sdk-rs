use super::{common, models::*, *};
use crate::{
    Client, Endpoint, ErrorKind, RawRequest, RequestOptions, Result, data::JsonObject,
    engine::AuthKind,
};
use http::Method;
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ScanStatisticsOptions {
    pub date_range: Option<String>,
    pub target_id: Option<String>,
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ScoreTrendOptions {
    pub date_range: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

/// One immutable OAuth identity across explicitly admitted service endpoints.
#[derive(Clone, Debug)]
pub struct RedTeamClient {
    data: Client,
    management: Client,
    broker: Client,
}
impl RedTeamClient {
    /// Share one OAuth identity across explicitly admitted data, management and broker bases.
    ///
    /// Enable `red-team`. The supplied client selects the data base and transport;
    /// its destination policy must also admit the management and broker endpoints.
    ///
    /// ```no_run
    /// use scm_sdk::{Client, Endpoint, Result,
    ///     airs::red_team::{RedTeamClient, RedTeamScanListOptions}};
    /// async fn scan_page(oauth: Client, management: Endpoint, broker: Endpoint) -> Result<()> {
    ///     let red_team = RedTeamClient::new(oauth, management, broker)?;
    ///     let query = RedTeamScanListOptions::default();
    ///     let _page = red_team.scans().list(&query, None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(
        data: Client,
        management_endpoint: Endpoint,
        broker_endpoint: Endpoint,
    ) -> Result<Self> {
        data.require_auth(AuthKind::OAuth)?;
        let data = data.with_tsg_header(false)?;
        let management = data.with_endpoint(management_endpoint)?;
        let broker = data.with_endpoint(broker_endpoint)?;
        Ok(Self {
            data,
            management,
            broker,
        })
    }
    /// Borrow the data-plane client and its shared cancellation session.
    pub fn client(&self) -> &Client {
        &self.data
    }
    /// Access attack scan submission, status and bounded listing.
    pub fn scans(&self) -> RedTeamScansClient {
        RedTeamScansClient {
            client: self.data.clone(),
        }
    }
    /// Access scan report and attack result reads.
    pub fn reports(&self) -> RedTeamReportsClient {
        RedTeamReportsClient {
            client: self.data.clone(),
        }
    }
    /// Access reports for custom attack workflows.
    pub fn custom_attack_reports(&self) -> RedTeamCustomAttackReportsClient {
        RedTeamCustomAttackReportsClient {
            client: self.data.clone(),
        }
    }
    /// Access management-plane instance capabilities.
    pub fn instances(&self) -> RedTeamInstancesClient {
        RedTeamInstancesClient {
            client: self.management.clone(),
        }
    }
    /// Access attack target configuration.
    pub fn targets(&self) -> RedTeamTargetsClient {
        RedTeamTargetsClient {
            client: self.management.clone(),
        }
    }
    /// Access custom prompt sets, prompts and attack definitions.
    pub fn custom_attacks(&self) -> RedTeamCustomAttacksClient {
        RedTeamCustomAttacksClient {
            client: self.management.clone(),
        }
    }
    /// Access target adapters and connection validation.
    pub fn adapters(&self) -> RedTeamAdaptersClient {
        RedTeamAdaptersClient {
            client: self.management.clone(),
        }
    }
    /// Access explicit license agreement status and acceptance.
    pub fn eula(&self) -> RedTeamEulaClient {
        RedTeamEulaClient {
            client: self.management.clone(),
        }
    }
    /// Access the separately scoped network broker client.
    pub fn network_broker(&self) -> RedTeamNetworkBrokerClient {
        RedTeamNetworkBrokerClient {
            client: self.broker.clone(),
        }
    }

    /// Only an API HTTP 422 on the static data route triggers the observed
    /// management metadata fallback. Both reads use one absolute budget.
    pub async fn get_scan_metadata(&self, options: Option<&RequestOptions>) -> Result<JsonObject> {
        let options = self.data.operation_options(options)?;
        // The frozen inline ZodObject({}).passthrough() accepts arbitrary JSON
        // objects. Reused s012087 is ZodRecord(string, unknown): its JSON-domain
        // behavior is equivalent, with the same central resource bounds.
        match self
            .data
            .send_contract(
                RawRequest::read("/v1/scan/scan-metadata").named("airs.red_team.get_scan_metadata"),
                "s012087",
                Some(&options),
            )
            .await
        {
            Err(error) if error.kind() == ErrorKind::Http && error.status() == Some(422) => {
                self.targets().get_target_metadata(Some(&options)).await
            }
            result => result,
        }
    }
    pub async fn get_goal_categories(
        &self,
        target_type: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GoalCategoryListResponse> {
        self.data
            .send_contract(
                RawRequest::read("/v1/goal-categories")
                    .segment(target_type)?
                    .named("airs.red_team.get_goal_categories"),
                "vendor:red-team-dp:0.93.0:get_goal_categories_v1_goal_categories__target_type__get:response",
                options,
            )
            .await
    }
    pub async fn get_scan_statistics(
        &self,
        params: &ScanStatisticsOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ScanStatisticsResponse> {
        let mut query = Vec::new();
        common::push_filter(&mut query, "date_range", params.date_range.as_deref())?;
        common::push_filter(&mut query, "target_id", params.target_id.as_deref())?;
        self.data
            .send_contract(
                RawRequest::read("/v1/dashboard/scan-statistics")
                    .query(common::current_query("get_scan_statistics_v1_dashboard_scan_statistics_get", query)?)
                    .named("airs.red_team.get_scan_statistics"),
                "vendor:red-team-dp:0.93.0:get_scan_statistics_v1_dashboard_scan_statistics_get:response",
                options,
            )
            .await
    }
    pub async fn get_score_trend(
        &self,
        target_id: &str,
        params: &ScoreTrendOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ScoreTrendResponse> {
        common::uuid(target_id)?;
        let mut query = vec![("target_id".into(), target_id.into())];
        for (key, value) in [
            ("date_range", &params.date_range),
            ("start_date", &params.start_date),
            ("end_date", &params.end_date),
        ] {
            common::push_filter(&mut query, key, value.as_deref())?;
        }
        self.data
            .send_contract(
                RawRequest::read("/v1/dashboard/score-trend")
                    .query(common::current_query(
                        "get_score_trend_v1_dashboard_score_trend_get",
                        query,
                    )?)
                    .named("airs.red_team.get_score_trend"),
                "vendor:red-team-dp:0.93.0:get_score_trend_v1_dashboard_score_trend_get:response",
                options,
            )
            .await
    }
    /// Published POST quota contract, without an automatic GET fallback.
    pub async fn get_quota(&self, options: Option<&RequestOptions>) -> Result<QuotaSummary> {
        self.data
            .send_contract(
                RawRequest::new(Method::POST, "/v1/metering/quota")
                    .named("airs.red_team.get_quota"),
                "vendor:red-team-dp:0.93.0:post_quota_summary_v1_metering_quota_post:response",
                options,
            )
            .await
    }
    pub async fn get_quota_summary(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<QuotaSummary> {
        self.data
            .send_contract(
                RawRequest::read("/v1/metering/quota").named("airs.red_team.get_quota_summary"),
                "QuotaSummarySchema",
                options,
            )
            .await
    }
    pub async fn get_error_logs(
        &self,
        job_id: &str,
        params: &RedTeamListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ErrorLogListResponse> {
        common::uuid(job_id)?;
        self.data
            .send_contract(
                RawRequest::read("/v1/error-log/job")
                    .segment(job_id)?
                    .query(common::current_query("list_error_logs_for_job_v1_error_log_job__job_id__get", params.query()?)?)
                    .named("airs.red_team.get_error_logs"),
                "vendor:red-team-dp:0.93.0:list_error_logs_for_job_v1_error_log_job__job_id__get:response",
                options,
            )
            .await
    }
    pub async fn get_target_profile_error_logs(
        &self,
        target_id: &str,
        params: &RedTeamListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ErrorLogListResponse> {
        common::uuid(target_id)?;
        self.data
            .send_contract(
                RawRequest::read("/v1/error-log/target-profile")
                    .segment(target_id)?
                    .query(common::current_query("list_error_logs_for_target_profile_v1_error_log_target_profile__target_id__get", params.query()?)?)
                    .named("airs.red_team.get_target_profile_error_logs"),
                "vendor:red-team-dp:0.93.0:list_error_logs_for_target_profile_v1_error_log_target_profile__target_id__get:response",
                options,
            )
            .await
    }
    pub async fn get_languages(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<TenantLanguagesResponse> {
        self.data
            .send_contract(
                RawRequest::read("/v1/languages").named("airs.red_team.get_languages"),
                "vendor:red-team-dp:0.93.0:get_languages_v1_languages_get:response",
                options,
            )
            .await
    }
    pub async fn get_management_languages(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<TenantLanguagesResponse> {
        self.management
            .send_contract(
                RawRequest::read("/v1/languages").named("airs.red_team.get_management_languages"),
                "TenantLanguagesResponseSchema",
                options,
            )
            .await
    }
    pub async fn update_sentiment(
        &self,
        body: &SentimentRequest,
        options: Option<&RequestOptions>,
    ) -> Result<SentimentResponse> {
        self.data
            .send_model(
                RawRequest::new(Method::POST, "/v1/sentiment")
                    .named("airs.red_team.update_sentiment"),
                body,
                "vendor:red-team-dp:0.93.0:create_or_update_sentiment_v1_sentiment_post:body",
                "vendor:red-team-dp:0.93.0:create_or_update_sentiment_v1_sentiment_post:response",
                options,
            )
            .await
    }
    pub async fn get_sentiment(
        &self,
        job_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<SentimentResponse> {
        common::uuid(job_id)?;
        self.data
            .send_contract(
                RawRequest::read("/v1/sentiment")
                    .segment(job_id)?
                    .named("airs.red_team.get_sentiment"),
                "vendor:red-team-dp:0.93.0:get_job_sentiment_v1_sentiment__job_id__get:response",
                options,
            )
            .await
    }
    pub async fn get_dashboard_overview(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<DashboardOverviewResponse> {
        self.management
            .send_contract(
                RawRequest::read("/v1/dashboard/overview")
                    .named("airs.red_team.get_dashboard_overview"),
                "DashboardOverviewResponseSchema",
                options,
            )
            .await
    }
}

crate::redaction::redacted_debug!(ScanStatisticsOptions, ScoreTrendOptions);
