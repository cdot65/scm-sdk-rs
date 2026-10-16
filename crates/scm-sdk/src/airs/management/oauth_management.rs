//! Token administration endpoints, distinct from the shared service-token cache.
use super::models::{ClientIdAndCustomerApp, Oauth2Token};
use crate::{
    Client, RawRequest, RequestOptions, Result, Secret, data::FiniteNumber, engine::AuthKind,
};
use http::Method;
use std::fmt;

/// Optional token TTL query values. The service owns units and permitted ranges.
#[derive(Clone)]
#[non_exhaustive]
pub struct GetTokenOptions {
    pub body: ClientIdAndCustomerApp,
    pub token_ttl_interval: Option<FiniteNumber>,
    pub token_ttl_unit: Option<String>,
}
impl GetTokenOptions {
    pub fn new(body: ClientIdAndCustomerApp) -> Self {
        Self {
            body,
            token_ttl_interval: None,
            token_ttl_unit: None,
        }
    }
    pub fn with_token_ttl_interval(mut self, value: FiniteNumber) -> Self {
        self.token_ttl_interval = Some(value);
        self
    }
    pub fn with_token_ttl_unit(mut self, value: impl Into<String>) -> Self {
        self.token_ttl_unit = Some(value.into());
        self
    }
}
impl fmt::Debug for GetTokenOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GetTokenOptions([REDACTED])")
    }
}

#[derive(Clone, Debug)]
pub struct OAuthManagementClient {
    client: Client,
}
impl OAuthManagementClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::OAuth)?;
        // Frozen management operations use bearer authentication without tenant routing.
        Ok(Self {
            client: client.with_tsg_header(false)?,
        })
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
    /// Create an application token; its payload remains explicit and Debug-redacted.
    /// This POST is never replayed after an uncertain outcome.
    pub async fn get_access_token(
        &self,
        input: &GetTokenOptions,
        options: Option<&RequestOptions>,
    ) -> Result<Oauth2Token> {
        let mut query = Vec::new();
        if let Some(interval) = &input.token_ttl_interval {
            let number = interval
                .as_f64()
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?;
            let mut buffer = ryu_js::Buffer::new();
            query.push(("tokenTtlInterval".into(), buffer.format(number).into()));
        }
        if let Some(unit) = &input.token_ttl_unit {
            query.push(("tokenTtlUnit".into(), unit.clone()));
        }
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/mgmt/oauth/client_credential/accesstoken")
                    .named("airs.management.oauth.get_access_token")
                    .query(query),
                &input.body,
                "ClientIdAndCustomerAppSchema",
                "Oauth2TokenSchema",
                options,
            )
            .await
    }
    /// Invalidate an application token. The token is query data, never a path or log.
    /// Return the endpoint's JSON string confirmation without treating it as plain text.
    pub async fn invalidate_token(
        &self,
        token: &Secret,
        body: &ClientIdAndCustomerApp,
        options: Option<&RequestOptions>,
    ) -> Result<String> {
        let options = self.client.operation_options(options)?;
        let bytes = self
            .client
            .validated_json(body, "ClientIdAndCustomerAppSchema", &options)
            .await?;
        self.client
            .send_json(
                RawRequest::new(Method::POST, "/v1/mgmt/oauth/invalidateToken")
                    .named("airs.management.oauth.invalidate_token")
                    .query([("token".into(), token.expose().into())])
                    .body(bytes, "application/json")?,
                Some(&options),
            )
            .await
    }
}
