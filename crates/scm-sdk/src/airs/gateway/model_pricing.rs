use super::{common, models::GatewayModelPricingConfig};
use crate::{Client, RawRequest, RequestOptions, Result, engine::AuthKind};
use std::time::Duration;

/// Public catalog policy. The caller supplies the endpoint/transport through a credential-free Client.
#[derive(Clone)]
#[non_exhaustive]
pub struct AIGatewayModelPricingClientOptions {
    pub timeout: Duration,
    pub read_retries: u32,
}
impl Default for AIGatewayModelPricingClientOptions {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(60),
            read_retries: 0,
        }
    }
}
crate::redaction::redacted_debug!(AIGatewayModelPricingClientOptions);

/// Standalone public pricing; SCM and inference credentials are rejected.
#[derive(Clone, Debug)]
pub struct AIGatewayModelPricingClient {
    client: Client,
    defaults: AIGatewayModelPricingClientOptions,
}
impl AIGatewayModelPricingClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::Public)?;
        Ok(Self {
            client,
            defaults: AIGatewayModelPricingClientOptions::default(),
        })
    }
    pub fn with_options(mut self, defaults: AIGatewayModelPricingClientOptions) -> Result<Self> {
        RequestOptions::with_timeout(defaults.timeout)?.with_retries(defaults.read_retries)?;
        self.defaults = defaults;
        Ok(self)
    }
    /// Return unevaluated, unconverted catalog data without any tenant billing claim.
    pub async fn get(
        &self,
        provider: &str,
        model: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayModelPricingConfig> {
        common::path_segment(provider)?;
        common::path_segment(model)?;
        let options = options
            .cloned()
            .map_or_else(|| RequestOptions::with_timeout(self.defaults.timeout), Ok)?;
        let options = if options.retry_limit().is_none() {
            options.with_retries(self.defaults.read_retries)?
        } else {
            options
        };
        self.client
            .send_contract(
                RawRequest::read("/model-configs/pricing")
                    .segment(provider)?
                    .segment(model)?
                    .header("accept", "application/json")?
                    .named("airs.gateway.model_pricing.get"),
                "GatewayModelPricingConfigSchema",
                Some(&options),
            )
            .await
    }
}
