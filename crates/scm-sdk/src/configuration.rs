//! Isolated scan initialization. Existing clients never consult mutable configuration state.
use crate::environment::{EnvironmentSnapshot, js_trim};
use crate::{
    Client, ClientBuilder, Endpoint, Error, ErrorKind, Limits, Result, Secret, auth::Authentication,
};
const DEFAULT_ENDPOINT: &str = "https://service.api.aisecurity.paloaltonetworks.com";
/// Credentials are explicit secrets. Environment resolution is a separately named operation.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct InitOptions {
    pub api_key: Option<Secret>,
    pub api_token: Option<Secret>,
    pub api_endpoint: Option<Endpoint>,
    pub num_retries: Option<u32>,
}
crate::redaction::redacted_debug!(InitOptions);
#[derive(Clone)]
pub struct ScanConfiguration {
    api_key: Option<Secret>,
    api_token: Option<Secret>,
    api_endpoint: Endpoint,
    num_retries: u32,
    initialized: bool,
}
crate::redaction::redacted_debug!(ScanConfiguration);
fn credential(explicit: Option<Secret>, environment: Option<&str>) -> Result<Option<Secret>> {
    let raw = explicit.as_ref().map(Secret::expose).or(environment);
    let Some(raw) = raw else { return Ok(None) };
    let value = js_trim(raw);
    if value.is_empty() {
        return Ok(None);
    };
    if value.encode_utf16().take(2049).count() > 2048 {
        return Err(Error::new(ErrorKind::Configuration));
    }
    Secret::new(value).map(Some)
}
impl ScanConfiguration {
    pub fn new() -> Result<Self> {
        Ok(Self {
            api_key: None,
            api_token: None,
            api_endpoint: Endpoint::parse(DEFAULT_ENDPOINT)?,
            num_retries: 5,
            initialized: false,
        })
    }
    pub fn api_key(&self) -> Option<&Secret> {
        self.api_key.as_ref()
    }
    pub fn api_token(&self) -> Option<&Secret> {
        self.api_token.as_ref()
    }
    pub fn api_endpoint(&self) -> &Endpoint {
        &self.api_endpoint
    }
    pub fn num_retries(&self) -> u32 {
        self.num_retries
    }
    pub fn initialized(&self) -> bool {
        self.initialized
    }
    /// Validate a complete replacement before committing any state.
    pub fn init(&mut self, options: InitOptions) -> Result<()> {
        self.init_from_environment(options, &EnvironmentSnapshot::empty())
    }
    pub fn init_from_environment(
        &mut self,
        options: InitOptions,
        environment: &EnvironmentSnapshot,
    ) -> Result<()> {
        let api_key = credential(options.api_key, environment.get("PANW_AI_SEC_API_KEY"))?;
        let api_token = credential(options.api_token, environment.get("PANW_AI_SEC_API_TOKEN"))?;
        Authentication::scan(api_key.clone(), api_token.clone())?;
        let api_endpoint = match options.api_endpoint {
            Some(endpoint) => endpoint,
            None => Endpoint::parse(
                environment
                    .get("PANW_AI_SEC_API_ENDPOINT")
                    .unwrap_or(DEFAULT_ENDPOINT),
            )?,
        };
        // Match source trailing-slash normalization while preserving an explicit loopback opt-in.
        let normalized = api_endpoint.url().as_str().trim_end_matches('/');
        let api_endpoint = if api_endpoint.url().scheme() == "http" {
            Endpoint::loopback(normalized)?
        } else {
            Endpoint::parse(normalized)?
        };
        let num_retries = options.num_retries.unwrap_or(5);
        if num_retries > 5 {
            return Err(Error::new(ErrorKind::Configuration));
        }
        *self = Self {
            api_key,
            api_token,
            api_endpoint,
            num_retries,
            initialized: true,
        };
        Ok(())
    }
    pub fn reset(&mut self) -> Result<()> {
        *self = Self::new()?;
        Ok(())
    }
    pub fn client_builder(&self) -> Result<ClientBuilder> {
        if !self.initialized {
            return Err(Error::new(ErrorKind::Configuration));
        }
        let limits = Limits {
            max_retries: self.num_retries,
            ..Limits::default()
        };
        Ok(Client::builder(self.api_endpoint.clone())
            .limits(limits)
            .authentication(Authentication::scan(
                self.api_key.clone(),
                self.api_token.clone(),
            )?))
    }
}
pub fn init(options: InitOptions) -> Result<ScanConfiguration> {
    let mut configuration = ScanConfiguration::new()?;
    configuration.init(options)?;
    Ok(configuration)
}
pub fn init_from_environment(
    options: InitOptions,
    environment: &EnvironmentSnapshot,
) -> Result<ScanConfiguration> {
    let mut configuration = ScanConfiguration::new()?;
    configuration.init_from_environment(options, environment)?;
    Ok(configuration)
}
