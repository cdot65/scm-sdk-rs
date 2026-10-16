//! Exact SCM telemetry adapters. Monetary values remain in cents.
use super::{common, models::*};
use crate::{Client, Error, ErrorKind, RawRequest, RequestOptions, Result, validation::NativeDate};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
const MAX_SAFE: u64 = 9_007_199_254_740_991;

/// UTC is explicit by default; callers may select a numeric offset without ambient timezone state.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct AIGatewayWindowOptions {
    pub workspace_slug: String,
    pub days: Option<f64>,
    pub start: Option<NativeDate>,
    pub end: Option<NativeDate>,
    pub offset_minutes: i16,
}
impl AIGatewayWindowOptions {
    pub fn new(workspace_slug: impl Into<String>) -> Self {
        Self {
            workspace_slug: workspace_slug.into(),
            ..Self::default()
        }
    }
    fn query(&self, tsg: &str) -> Result<Vec<(String, String)>> {
        common::numeric_id(tsg)?;
        common::workspace_ref(&self.workspace_slug)?;
        if self.days.is_some_and(|n| !n.is_finite()) {
            return Err(common::invalid());
        }
        let end = match self.end {
            Some(end) => end,
            None => {
                let millis = match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(value) => i64::try_from(value.as_millis()).map_err(|_| common::invalid())?,
                    Err(value) => -i64::try_from(value.duration().as_millis())
                        .map_err(|_| common::invalid())?,
                };
                NativeDate::from_epoch_millis(millis)?
            }
        };
        let start = match self.start {
            Some(start) => start,
            None => {
                let millis =
                    (end.epoch_millis() as f64 - self.days.unwrap_or(7.0) * 86_400_000.0).trunc();
                if !millis.is_finite() || millis.abs() > 8_640_000_000_000_000.0 {
                    return Err(common::invalid());
                }
                NativeDate::from_epoch_millis(millis as i64)?
            }
        };
        if start.epoch_millis() > end.epoch_millis() {
            return Err(common::invalid());
        }
        Ok(vec![
            ("organisationId".into(), tsg.into()),
            ("workspaceSlug".into(), self.workspace_slug.clone()),
            (
                "timeOfGenerationMin".into(),
                common::date_offset_iso(&start, self.offset_minutes)?,
            ),
            (
                "timeOfGenerationMax".into(),
                common::date_offset_iso(&end, self.offset_minutes)?,
            ),
        ])
    }
}

#[derive(Clone)]
#[non_exhaustive]
pub struct AIGatewayChartOptions {
    pub window: AIGatewayWindowOptions,
    pub filters: AIGatewayChartFilters,
}
impl AIGatewayChartOptions {
    pub fn new(window: AIGatewayWindowOptions) -> Result<Self> {
        Ok(Self {
            window,
            filters: AIGatewayChartFilters::builder().build()?,
        })
    }
}

pub type AIGatewayRequestChartOptions = AIGatewayChartOptions;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum AIGatewayGroupColumn {
    Cost,
    AvgLatency,
    AvgTokens,
    TotalTokens,
    SuccessRate,
    LastSeen,
}
impl AIGatewayGroupColumn {
    fn as_str(self) -> &'static str {
        match self {
            Self::Cost => "cost",
            Self::AvgLatency => "avg_latency",
            Self::AvgTokens => "avg_tokens",
            Self::TotalTokens => "total_tokens",
            Self::SuccessRate => "success_rate",
            Self::LastSeen => "last_seen",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum AIGatewayGroupDimension {
    AiService,
    Model,
    ApiKey,
    Provider,
}
impl AIGatewayGroupDimension {
    fn as_str(self) -> &'static str {
        match self {
            Self::AiService => "ai_service",
            Self::Model => "model",
            Self::ApiKey => "api_key",
            Self::Provider => "provider",
        }
    }
}
#[derive(Clone)]
#[non_exhaustive]
pub struct AIGatewayGroupOptions {
    pub chart: AIGatewayChartOptions,
    pub columns: Option<Vec<AIGatewayGroupColumn>>,
}
impl AIGatewayGroupOptions {
    pub fn new(chart: AIGatewayChartOptions) -> Self {
        Self {
            chart,
            columns: None,
        }
    }
}

#[derive(Clone)]
#[non_exhaustive]
pub struct AIGatewayLogsOptions {
    pub window: AIGatewayWindowOptions,
    pub page_size: Option<u64>,
    pub current_page: Option<u64>,
    pub trace_id: Option<String>,
    pub status_code: Option<u64>,
}
impl AIGatewayLogsOptions {
    pub fn new(window: AIGatewayWindowOptions) -> Self {
        Self {
            window,
            page_size: None,
            current_page: None,
            trace_id: None,
            status_code: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AIGatewayTelemetryClient {
    pub(super) client: Client,
}
impl AIGatewayTelemetryClient {
    pub fn new(client: Client) -> Result<Self> {
        Ok(Self {
            client: common::oauth(client)?,
        })
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
    fn window_query(&self, window: &AIGatewayWindowOptions) -> Result<Vec<(String, String)>> {
        window.query(
            self.client
                .oauth_context()
                .ok_or_else(common::invalid)?
                .tsg_id(),
        )
    }
    async fn filtered_query(
        &self,
        chart: &AIGatewayChartOptions,
        options: &RequestOptions,
    ) -> Result<Vec<(String, String)>> {
        let mut query = options
            .run(&self.client.session(), async {
                self.window_query(&chart.window)
            })
            .await?;
        let value = options
            .run(&self.client.session(), async {
                let bytes =
                    crate::engine::serialize_bounded(&chart.filters, crate::config::MAX_URL_BYTES)?;
                serde_json::from_slice::<Value>(&bytes).map_err(|_| common::invalid())
            })
            .await?;
        let bytes = self
            .client
            .validated_json(&value, "AIGatewayChartFiltersSchema", options)
            .await?;
        options
            .run(&self.client.session(), async {
                let value: Value = serde_json::from_slice(&bytes).map_err(|_| common::invalid())?;
                for (field, wire) in [
                    ("traceId", "traceId"),
                    ("metadata", "metadata"),
                    ("statusCodes", "statusCode"),
                    ("apiKeyIds", "apiKeyIds"),
                    ("aiOrgModels", "aiOrgModel"),
                    ("totalUnitsMin", "totalUnitsMin"),
                    ("totalUnitsMax", "totalUnitsMax"),
                    ("costMin", "costMin"),
                    ("costMax", "costMax"),
                ] {
                    if let Some(value) = value.get(field) {
                        let encoded = match value {
                            Value::Array(values) => values
                                .iter()
                                .map(common::scalar)
                                .collect::<Result<Vec<_>>>()?
                                .join(","),
                            Value::Object(_) => {
                                serde_json::to_string(value).map_err(|_| common::invalid())?
                            }
                            value => common::scalar(value)?,
                        };
                        query.push((wire.into(), encoded));
                    }
                }
                Ok(query)
            })
            .await
    }
    async fn chart<T: serde::de::DeserializeOwned>(
        &self,
        metric: &'static str,
        window: &AIGatewayWindowOptions,
        schema: &str,
        options: Option<&RequestOptions>,
    ) -> Result<T> {
        let options = self.client.operation_options(options)?;
        let query = options
            .run(&self.client.session(), async { self.window_query(window) })
            .await?;
        self.client
            .send_contract(
                RawRequest::read("/logs/charts")
                    .segment(metric)?
                    .query(query)
                    .named("airs.gateway.telemetry.chart"),
                schema,
                Some(&options),
            )
            .await
    }
    pub async fn error_category_trends(
        &self,
        params: &AIGatewayWindowOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ErrorCategoryTrendsResponse> {
        self.chart(
            "error-category-trends",
            params,
            "ErrorCategoryTrendsResponseSchema",
            options,
        )
        .await
    }
    pub async fn grouped_errors(
        &self,
        params: &AIGatewayWindowOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GroupedErrorsResponse> {
        self.chart(
            "grouped-errors",
            params,
            "GroupedErrorsResponseSchema",
            options,
        )
        .await
    }
    pub async fn errors(
        &self,
        params: &AIGatewayWindowOptions,
        options: Option<&RequestOptions>,
    ) -> Result<CountChartResponse> {
        self.chart("errors", params, "CountChartResponseSchema", options)
            .await
    }
    pub async fn users(
        &self,
        params: &AIGatewayWindowOptions,
        options: Option<&RequestOptions>,
    ) -> Result<CountChartResponse> {
        self.chart("users", params, "CountChartResponseSchema", options)
            .await
    }
    pub async fn cache_summary(
        &self,
        params: &AIGatewayWindowOptions,
        options: Option<&RequestOptions>,
    ) -> Result<CacheSummaryResponse> {
        self.chart(
            "cache-summary",
            params,
            "CacheSummaryResponseSchema",
            options,
        )
        .await
    }
    pub async fn cache_hit_trend(
        &self,
        params: &AIGatewayWindowOptions,
        options: Option<&RequestOptions>,
    ) -> Result<CacheHitTrendResponse> {
        self.chart(
            "cache-hit-trend",
            params,
            "CacheHitTrendResponseSchema",
            options,
        )
        .await
    }
    pub async fn user_trends(
        &self,
        params: &AIGatewayWindowOptions,
        options: Option<&RequestOptions>,
    ) -> Result<UserTrendsResponse> {
        self.chart("user-trends", params, "UserTrendsResponseSchema", options)
            .await
    }
    pub async fn error_trends(
        &self,
        params: &AIGatewayWindowOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ErrorTrendsResponse> {
        self.chart("error-trends", params, "ErrorTrendsResponseSchema", options)
            .await
    }
    pub async fn rescued_retries(
        &self,
        params: &AIGatewayWindowOptions,
        options: Option<&RequestOptions>,
    ) -> Result<RescuedRetriesResponse> {
        self.chart(
            "rescued-retries",
            params,
            "RescuedRetriesResponseSchema",
            options,
        )
        .await
    }
    pub async fn feedback_trend(
        &self,
        params: &AIGatewayWindowOptions,
        options: Option<&RequestOptions>,
    ) -> Result<CountChartResponse> {
        self.chart(
            "feedback-trend",
            params,
            "CountChartResponseSchema",
            options,
        )
        .await
    }
    pub async fn feedback_weighted(
        &self,
        params: &AIGatewayWindowOptions,
        options: Option<&RequestOptions>,
    ) -> Result<CountChartResponse> {
        self.chart(
            "feedback-weighted",
            params,
            "CountChartResponseSchema",
            options,
        )
        .await
    }
    pub async fn feedback_score_distribution(
        &self,
        params: &AIGatewayWindowOptions,
        options: Option<&RequestOptions>,
    ) -> Result<FeedbackScoreDistributionResponse> {
        self.chart(
            "feedback-score-distribution",
            params,
            "FeedbackScoreDistributionResponseSchema",
            options,
        )
        .await
    }
    pub async fn feedback_models(
        &self,
        params: &AIGatewayWindowOptions,
        options: Option<&RequestOptions>,
    ) -> Result<FeedbackModelsResponse> {
        self.chart(
            "feedback-models",
            params,
            "FeedbackModelsResponseSchema",
            options,
        )
        .await
    }
    pub async fn cost(
        &self,
        params: &AIGatewayChartOptions,
        options: Option<&RequestOptions>,
    ) -> Result<CostChartResponse> {
        let options = self.client.operation_options(options)?;
        let query = self.filtered_query(params, &options).await?;
        self.client
            .send_contract(
                RawRequest::read("/logs/charts/cost")
                    .query(query)
                    .named("airs.gateway.telemetry.cost"),
                "CostChartResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn requests(
        &self,
        params: &AIGatewayChartOptions,
        options: Option<&RequestOptions>,
    ) -> Result<CountChartResponse> {
        let options = self.client.operation_options(options)?;
        let query = self.filtered_query(params, &options).await?;
        self.client
            .send_contract(
                RawRequest::read("/logs/charts/requests")
                    .query(query)
                    .named("airs.gateway.telemetry.requests"),
                "CountChartResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn latency(
        &self,
        params: &AIGatewayChartOptions,
        options: Option<&RequestOptions>,
    ) -> Result<LatencyChartResponse> {
        let options = self.client.operation_options(options)?;
        let query = self.filtered_query(params, &options).await?;
        self.client
            .send_contract(
                RawRequest::read("/logs/charts/latency")
                    .query(query)
                    .named("airs.gateway.telemetry.latency"),
                "LatencyChartResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn tokens(
        &self,
        params: &AIGatewayChartOptions,
        options: Option<&RequestOptions>,
    ) -> Result<TokensChartResponse> {
        let options = self.client.operation_options(options)?;
        let query = self.filtered_query(params, &options).await?;
        self.client
            .send_contract(
                RawRequest::read("/logs/charts/tokens")
                    .query(query)
                    .named("airs.gateway.telemetry.tokens"),
                "TokensChartResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn filter_boundaries(
        &self,
        params: &AIGatewayWindowOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayFilterBoundariesResponse> {
        let options = self.client.operation_options(options)?;
        let mut query = options
            .run(&self.client.session(), async { self.window_query(params) })
            .await?;
        query.retain(|(key, _)| key != "organisationId");
        self.client
            .send_contract(
                RawRequest::read("/analytics/filter-boundaries")
                    .query(query)
                    .named("airs.gateway.telemetry.filter_boundaries"),
                "GatewayFilterBoundariesResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn group_by(
        &self,
        dimension: AIGatewayGroupDimension,
        params: &AIGatewayGroupOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GroupListResponse> {
        self.group(dimension.as_str(), params, options).await
    }
    async fn group(
        &self,
        dimension: &str,
        params: &AIGatewayGroupOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GroupListResponse> {
        let options = self.client.operation_options(options)?;
        let mut query = self.filtered_query(&params.chart, &options).await?;
        if let Some(columns) = &params.columns {
            if columns.len() > 256 {
                return Err(Error::new(ErrorKind::Limit));
            }
            if !columns.is_empty() {
                query.push((
                    "columns".into(),
                    columns
                        .iter()
                        .map(|c| c.as_str())
                        .collect::<Vec<_>>()
                        .join(","),
                ));
            }
        }
        self.client
            .send_contract(
                RawRequest::read("/logs/groups")
                    .segment(dimension)?
                    .query(query)
                    .named("airs.gateway.telemetry.group"),
                "GroupListResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn by_status_code(
        &self,
        params: &AIGatewayGroupOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GroupListResponse> {
        self.group("status_code", params, options).await
    }
    pub async fn by_user(
        &self,
        params: &AIGatewayChartOptions,
        options: Option<&RequestOptions>,
    ) -> Result<UserGroupResponse> {
        let options = self.client.operation_options(options)?;
        let query = self.filtered_query(params, &options).await?;
        self.client
            .send_contract(
                RawRequest::read("/logs/groups/users")
                    .query(query)
                    .named("airs.gateway.telemetry.by_user"),
                "UserGroupResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn logs(
        &self,
        params: &AIGatewayLogsOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayLogsResponse> {
        let options = self.client.operation_options(options)?;
        let mut query = options
            .run(&self.client.session(), async {
                let mut query = self.window_query(&params.window)?;
                for (key, value) in [
                    ("pageSize", params.page_size),
                    ("currentPage", params.current_page),
                ] {
                    if let Some(value) = value {
                        if value > MAX_SAFE {
                            return Err(common::invalid());
                        }
                        query.push((key.into(), value.to_string()));
                    }
                }
                if let Some(trace) = &params.trace_id {
                    if common::js_trim(trace).is_empty()
                        || trace.len() > 4096
                        || trace.chars().any(char::is_control)
                    {
                        return Err(common::invalid());
                    }
                    query.push(("traceId".into(), trace.clone()));
                }
                if let Some(status) = params.status_code {
                    if status > MAX_SAFE {
                        return Err(common::invalid());
                    }
                    query.push(("statusCode".into(), status.to_string()));
                }
                Ok(query)
            })
            .await?;
        self.client
            .send_contract(
                RawRequest::read("/logs")
                    .query(std::mem::take(&mut query))
                    .named("airs.gateway.telemetry.logs"),
                "GatewayLogsResponseSchema",
                Some(&options),
            )
            .await
    }
}

crate::redaction::redacted_debug!(
    AIGatewayWindowOptions,
    AIGatewayChartOptions,
    AIGatewayGroupOptions,
    AIGatewayLogsOptions
);
