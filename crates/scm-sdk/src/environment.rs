//! Explicit, bounded environment snapshots. Constructors never read process variables implicitly.
use crate::auth::{AuthContext, OAuthProviderOptions};
use crate::{Client, ClientBuilder, Endpoint, Error, ErrorKind, Limits, Result, Secret};
use std::collections::BTreeMap;
use zeroize::Zeroizing;

const MAX_VARIABLES: usize = 128;
const MAX_VALUE_BYTES: usize = 16 * 1024;
const MAX_SNAPSHOT_BYTES: usize = 256 * 1024;
const DEFAULT_TOKEN_ENDPOINT: &str = "https://auth.apps.paloaltonetworks.com/oauth2/access_token";
/// The only variables used by explicit scan-environment initialization.
pub const SCAN_ENVIRONMENT_NAMES: &[&str] = &[
    "PANW_AI_SEC_API_KEY",
    "PANW_AI_SEC_API_TOKEN",
    "PANW_AI_SEC_API_ENDPOINT",
];
fn invalid() -> Error {
    Error::new(ErrorKind::Configuration)
}
fn name_valid(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 128
        && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}
/// Values, including empty strings, are preserved until service-specific resolution.
/// Formatting hides both variable names and values; each snapshot zeroizes its stored strings on drop.
#[derive(Clone)]
pub struct EnvironmentSnapshot {
    values: BTreeMap<String, Zeroizing<String>>,
}
impl std::fmt::Debug for EnvironmentSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnvironmentSnapshot")
            .field("variables", &self.values.len())
            .finish_non_exhaustive()
    }
}
impl EnvironmentSnapshot {
    pub fn empty() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }
    /// Construct a snapshot from explicit values, including deterministic test fixtures.
    pub fn from_pairs<I, K, V>(pairs: I) -> Result<Self>
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let mut snapshot = Self::empty();
        let mut bytes = 0usize;
        for (index, (name, value)) in pairs.into_iter().enumerate() {
            if index >= MAX_VARIABLES {
                return Err(Error::new(ErrorKind::Limit));
            }
            let name = name.into();
            let value = Zeroizing::new(value.into());
            if !name_valid(&name) {
                return Err(invalid());
            }
            if value.len() > MAX_VALUE_BYTES {
                return Err(Error::new(ErrorKind::Limit));
            }
            let previous = snapshot
                .values
                .get(&name)
                .map_or(0, |v| v.len() + name.len());
            bytes = bytes
                .saturating_sub(previous)
                .checked_add(name.len() + value.len())
                .filter(|n| *n <= MAX_SNAPSHOT_BYTES)
                .ok_or_else(|| Error::new(ErrorKind::Limit))?;
            snapshot.values.insert(name, value);
        }
        Ok(snapshot)
    }
    /// Read only caller-selected names, once. Missing differs from explicitly empty.
    pub fn capture(names: &[&str]) -> Result<Self> {
        if names.len() > MAX_VARIABLES {
            return Err(Error::new(ErrorKind::Limit));
        }
        let mut pairs = Vec::new();
        let mut bytes = 0usize;
        for name in names {
            if !name_valid(name) {
                return Err(invalid());
            }
            if let Some(value) = std::env::var_os(name) {
                let value = Zeroizing::new(value.into_string().map_err(|_| invalid())?);
                if value.len() > MAX_VALUE_BYTES {
                    return Err(Error::new(ErrorKind::Limit));
                }
                bytes = bytes
                    .checked_add(name.len() + value.len())
                    .filter(|n| *n <= MAX_SNAPSHOT_BYTES)
                    .ok_or_else(|| Error::new(ErrorKind::Limit))?;
                pairs.push(((*name).to_owned(), value));
            }
        }
        // Keep intermediate credential copies zeroized, including on failure.
        Self::from_pairs(
            pairs
                .iter()
                .map(|(key, value)| (key.clone(), value.as_str().to_owned())),
        )
    }
    /// Explicit value access. Callers own any resulting copies or disclosure.
    pub fn get(&self, name: &str) -> Option<&str> {
        self.values.get(name).map(|v| v.as_str())
    }
}

pub(crate) fn js_whitespace(value: char) -> bool {
    matches!(value, '\u{0009}'..='\u{000d}' | '\u{0020}' | '\u{00a0}' | '\u{1680}' |
        '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' |
        '\u{3000}' | '\u{feff}')
}
pub(crate) fn js_trim(value: &str) -> &str {
    value.trim_matches(js_whitespace)
}

/// Explicit counterpart to the source's internal OAuth environment resolver.
#[derive(Clone)]
#[non_exhaustive]
pub struct ResolveOAuthConfigOptions {
    pub base_url: Endpoint,
    pub primary_env_prefix: String,
    pub fallback_env_prefix: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<Secret>,
    pub tsg_id: Option<String>,
    pub token_endpoint: Option<Endpoint>,
    /// Source integer semantics: values outside 0..5 are clamped.
    pub num_retries: Option<i64>,
    pub credential_generation: u64,
    pub provider_options: OAuthProviderOptions,
}
impl ResolveOAuthConfigOptions {
    pub fn new(base_url: Endpoint, primary_env_prefix: impl Into<String>) -> Self {
        Self {
            base_url,
            primary_env_prefix: primary_env_prefix.into(),
            fallback_env_prefix: None,
            client_id: None,
            client_secret: None,
            tsg_id: None,
            token_endpoint: None,
            num_retries: None,
            credential_generation: 1,
            provider_options: OAuthProviderOptions::default(),
        }
    }
    /// Enumerate exactly the names a caller may choose to capture from the process.
    pub fn environment_names(&self) -> Result<Vec<String>> {
        let mut names = Vec::new();
        for prefix in
            std::iter::once(&self.primary_env_prefix).chain(self.fallback_env_prefix.iter())
        {
            if !name_valid(prefix) || prefix.len() > 96 {
                return Err(invalid());
            }
            for suffix in ["CLIENT_ID", "CLIENT_SECRET", "TSG_ID", "TOKEN_ENDPOINT"] {
                names.push(format!("{prefix}_{suffix}"));
            }
        }
        Ok(names)
    }
}
crate::redaction::redacted_debug!(ResolveOAuthConfigOptions);
/// Validated identity/configuration snapshot; it has no cache until a client/provider is built.
#[derive(Clone)]
pub struct OAuthServiceConfig {
    base_url: Endpoint,
    context: AuthContext,
    client_secret: Secret,
    num_retries: u32,
    provider_options: OAuthProviderOptions,
}
crate::redaction::redacted_debug!(OAuthServiceConfig);
impl OAuthServiceConfig {
    pub fn base_url(&self) -> &Endpoint {
        &self.base_url
    }
    pub fn context(&self) -> &AuthContext {
        &self.context
    }
    pub fn num_retries(&self) -> u32 {
        self.num_retries
    }
    /// Build an application-owned provider so metadata subscriptions can precede authentication.
    pub fn provider(
        &self,
        transport: std::sync::Arc<dyn crate::transport::Transport>,
        policy: std::sync::Arc<dyn crate::config::DestinationPolicy>,
        session: crate::Session,
    ) -> Result<crate::auth::OAuthProvider> {
        crate::auth::OAuthProvider::new_with_options(
            self.context.clone(),
            self.client_secret.clone(),
            transport,
            policy,
            session,
            self.provider_options.clone(),
        )
    }
    /// Preserve caller-owned transport, policy, session and TLS by configuring the returned builder.
    pub fn client_builder(&self, tsg_header: bool) -> ClientBuilder {
        let limits = Limits {
            max_retries: self.num_retries,
            ..Limits::default()
        };
        Client::builder(self.base_url.clone())
            .limits(limits)
            .oauth_credentials(self.context.clone(), self.client_secret.clone(), tsg_header)
            .oauth_provider_options(self.provider_options.clone())
    }
}
/// Resolve options -> primary snapshot -> fallback snapshot, without ambient reads or credential trimming.
pub fn resolve_oauth_config(
    options: ResolveOAuthConfigOptions,
    environment: &EnvironmentSnapshot,
) -> Result<OAuthServiceConfig> {
    options.environment_names()?;
    options.provider_options.validate()?;
    let lookup = |suffix: &str| {
        environment
            .get(&format!("{}_{}", options.primary_env_prefix, suffix))
            .or_else(|| {
                options
                    .fallback_env_prefix
                    .as_ref()
                    .and_then(|prefix| environment.get(&format!("{prefix}_{suffix}")))
            })
    };
    let client_id = options
        .client_id
        .as_deref()
        .or_else(|| lookup("CLIENT_ID"))
        .ok_or_else(invalid)?;
    let tsg_id = options
        .tsg_id
        .as_deref()
        .or_else(|| lookup("TSG_ID"))
        .ok_or_else(invalid)?;
    let secret = match options.client_secret {
        Some(secret) => secret,
        None => Secret::new(lookup("CLIENT_SECRET").ok_or_else(invalid)?)?,
    };
    if secret.expose().chars().any(char::is_control) {
        return Err(invalid());
    }
    let issuer = match options.token_endpoint {
        Some(endpoint) => endpoint,
        None => Endpoint::parse(lookup("TOKEN_ENDPOINT").unwrap_or(DEFAULT_TOKEN_ENDPOINT))?,
    };
    let context = AuthContext::new(issuer, client_id, options.credential_generation, tsg_id)?;
    Ok(OAuthServiceConfig {
        base_url: options.base_url,
        context,
        client_secret: secret,
        num_retries: options.num_retries.unwrap_or(5).clamp(0, 5) as u32,
        provider_options: options.provider_options,
    })
}
