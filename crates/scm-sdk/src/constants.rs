//! Typed constants from the frozen TypeScript baseline. Regenerate with tools/generate_constants.py.
use serde::{Deserialize, Serialize};
pub const BASELINE_SDK_VERSION: &str = "0.34.0";
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AIGatewaySecretOperation {
    #[serde(rename = "telemetry.filterBoundaries")]
    TelemetryFilterBoundaries,
    #[serde(rename = "organisations.getInfo")]
    OrganisationsGetInfo,
    #[serde(rename = "secretReferences.list")]
    SecretReferencesList,
    #[serde(rename = "secretReferences.create")]
    SecretReferencesCreate,
    #[serde(rename = "secretReferences.get")]
    SecretReferencesGet,
    #[serde(rename = "secretReferences.update")]
    SecretReferencesUpdate,
    #[serde(rename = "secretReferences.delete")]
    SecretReferencesDelete,
    #[serde(rename = "apiKeys.createService")]
    ApiKeysCreateService,
    #[serde(rename = "apiKeys.createUser")]
    ApiKeysCreateUser,
    #[serde(rename = "apiKeys.rotateService")]
    ApiKeysRotateService,
    #[serde(rename = "apiKeys.rotateUser")]
    ApiKeysRotateUser,
    #[serde(rename = "deployments.create")]
    DeploymentsCreate,
    #[serde(rename = "deployments.update")]
    DeploymentsUpdate,
    #[serde(rename = "integrations.create")]
    IntegrationsCreate,
    #[serde(rename = "integrations.update")]
    IntegrationsUpdate,
    #[serde(rename = "mcpIntegrations.create")]
    McpIntegrationsCreate,
    #[serde(rename = "mcpIntegrations.update")]
    McpIntegrationsUpdate,
    #[serde(rename = "organisations.getAuthSettings")]
    OrganisationsGetAuthSettings,
    #[serde(rename = "organisations.updateAuthSettings")]
    OrganisationsUpdateAuthSettings,
    #[serde(rename = "plugins.create")]
    PluginsCreate,
    #[serde(rename = "providers.get")]
    ProvidersGet,
}
impl AIGatewaySecretOperation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TelemetryFilterBoundaries => "telemetry.filterBoundaries",
            Self::OrganisationsGetInfo => "organisations.getInfo",
            Self::SecretReferencesList => "secretReferences.list",
            Self::SecretReferencesCreate => "secretReferences.create",
            Self::SecretReferencesGet => "secretReferences.get",
            Self::SecretReferencesUpdate => "secretReferences.update",
            Self::SecretReferencesDelete => "secretReferences.delete",
            Self::ApiKeysCreateService => "apiKeys.createService",
            Self::ApiKeysCreateUser => "apiKeys.createUser",
            Self::ApiKeysRotateService => "apiKeys.rotateService",
            Self::ApiKeysRotateUser => "apiKeys.rotateUser",
            Self::DeploymentsCreate => "deployments.create",
            Self::DeploymentsUpdate => "deployments.update",
            Self::IntegrationsCreate => "integrations.create",
            Self::IntegrationsUpdate => "integrations.update",
            Self::McpIntegrationsCreate => "mcpIntegrations.create",
            Self::McpIntegrationsUpdate => "mcpIntegrations.update",
            Self::OrganisationsGetAuthSettings => "organisations.getAuthSettings",
            Self::OrganisationsUpdateAuthSettings => "organisations.updateAuthSettings",
            Self::PluginsCreate => "plugins.create",
            Self::ProvidersGet => "providers.get",
        }
    }
    pub fn parse(value: &str) -> crate::Result<Self> {
        match value {
            "telemetry.filterBoundaries" => Ok(Self::TelemetryFilterBoundaries),
            "organisations.getInfo" => Ok(Self::OrganisationsGetInfo),
            "secretReferences.list" => Ok(Self::SecretReferencesList),
            "secretReferences.create" => Ok(Self::SecretReferencesCreate),
            "secretReferences.get" => Ok(Self::SecretReferencesGet),
            "secretReferences.update" => Ok(Self::SecretReferencesUpdate),
            "secretReferences.delete" => Ok(Self::SecretReferencesDelete),
            "apiKeys.createService" => Ok(Self::ApiKeysCreateService),
            "apiKeys.createUser" => Ok(Self::ApiKeysCreateUser),
            "apiKeys.rotateService" => Ok(Self::ApiKeysRotateService),
            "apiKeys.rotateUser" => Ok(Self::ApiKeysRotateUser),
            "deployments.create" => Ok(Self::DeploymentsCreate),
            "deployments.update" => Ok(Self::DeploymentsUpdate),
            "integrations.create" => Ok(Self::IntegrationsCreate),
            "integrations.update" => Ok(Self::IntegrationsUpdate),
            "mcpIntegrations.create" => Ok(Self::McpIntegrationsCreate),
            "mcpIntegrations.update" => Ok(Self::McpIntegrationsUpdate),
            "organisations.getAuthSettings" => Ok(Self::OrganisationsGetAuthSettings),
            "organisations.updateAuthSettings" => Ok(Self::OrganisationsUpdateAuthSettings),
            "plugins.create" => Ok(Self::PluginsCreate),
            "providers.get" => Ok(Self::ProvidersGet),
            _ => Err(crate::Error::new(crate::ErrorKind::Validation)),
        }
    }
}
#[derive(Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GatewaySecretDirection {
    #[default]
    Request,
    Response,
}
pub type GatewaySecretPathSegment = String;
#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GatewaySecretRedaction {
    Value,
    Subtree,
}
#[derive(Clone, Copy, Serialize)]
pub struct GatewaySecretRule {
    pub path: &'static [&'static str],
    pub redact: GatewaySecretRedaction,
    #[serde(rename = "oneTime", skip_serializing_if = "Option::is_none")]
    pub one_time: Option<bool>,
}
#[derive(Clone, Copy, Serialize)]
pub struct GatewaySecretRules {
    pub request: &'static [GatewaySecretRule],
    pub response: &'static [GatewaySecretRule],
}
impl GatewaySecretRules {
    pub fn direction(&self, direction: GatewaySecretDirection) -> &'static [GatewaySecretRule] {
        match direction {
            GatewaySecretDirection::Request => self.request,
            GatewaySecretDirection::Response => self.response,
        }
    }
}
crate::redaction::redacted_debug!(
    AIGatewaySecretOperation,
    GatewaySecretDirection,
    GatewaySecretRedaction,
    GatewaySecretRule,
    GatewaySecretRules
);
pub const AGENT_GUARD_DATA_ENDPOINT: &str = "https://api.apps.paloaltonetworks.com/aiag/data";
pub const AGENT_GUARD_MGMT_ENDPOINT: &str = "https://api.apps.paloaltonetworks.com/aiag/mgmt";
pub const AGENT_GUARD_RULES_PATH: &str = "/v1/rules";
pub const AGENT_GUARD_SCANS_PATH: &str = "/v1/scans";
pub const AGENT_GUARD_STATS_PATH: &str = "/v1/stats/scans";
pub const AI_GATEWAY_DEPLOYMENT_STATUSES: &[&str] = &["active", "archived"];
pub const AI_GATEWAY_DEPLOYMENT_TYPES: &[&str] = &["non_production", "production"];
pub const AI_GATEWAY_KNOWN_API_KEY_SCOPES: &[&str] = &[
    "agents.invoke",
    "completions.write",
    "logs.write",
    "mcp.invoke",
    "prompts.render",
];
pub const AI_GATEWAY_KNOWN_CACHE_MODES: &[&str] = &["semantic", "simple"];
pub const AI_GATEWAY_KNOWN_CONFIG_STRATEGIES: &[&str] = &["fallback", "loadbalance", "single"];
pub const AI_GATEWAY_KNOWN_MCP_AUTH_TYPES: &[&str] =
    &["headers", "none", "oauth_auto", "oauth_client_credentials"];
pub const AI_GATEWAY_KNOWN_MCP_TRANSPORTS: &[&str] = &["http", "interactive", "sse"];
pub const AI_GATEWAY_KNOWN_RATE_LIMIT_TYPES: &[&str] = &["requests", "tokens"];
pub const AI_GATEWAY_KNOWN_RATE_LIMIT_UNITS: &[&str] = &["rpd", "rph", "rpm", "rps", "rpw"];
pub const AI_GATEWAY_MUTABLE_MCP_CAPABILITY_TYPES: &[&str] = &["prompt", "resource", "tool"];
pub const AI_GATEWAY_REDACTED: &str = "[REDACTED]";
pub const AI_GATEWAY_SECRET_FIELDS: &[(AIGatewaySecretOperation, GatewaySecretRules)] = &[
    (
        AIGatewaySecretOperation::TelemetryFilterBoundaries,
        GatewaySecretRules {
            request: &[],
            response: &[GatewaySecretRule {
                path: &["data"],
                redact: GatewaySecretRedaction::Subtree,
                one_time: None,
            }],
        },
    ),
    (
        AIGatewaySecretOperation::OrganisationsGetInfo,
        GatewaySecretRules {
            request: &[],
            response: &[GatewaySecretRule {
                path: &["settings"],
                redact: GatewaySecretRedaction::Subtree,
                one_time: None,
            }],
        },
    ),
    (
        AIGatewaySecretOperation::SecretReferencesList,
        GatewaySecretRules {
            request: &[],
            response: &[GatewaySecretRule {
                path: &["data", "*", "auth_config"],
                redact: GatewaySecretRedaction::Subtree,
                one_time: None,
            }],
        },
    ),
    (
        AIGatewaySecretOperation::SecretReferencesCreate,
        GatewaySecretRules {
            request: &[GatewaySecretRule {
                path: &["auth_config"],
                redact: GatewaySecretRedaction::Subtree,
                one_time: None,
            }],
            response: &[GatewaySecretRule {
                path: &["auth_config"],
                redact: GatewaySecretRedaction::Subtree,
                one_time: None,
            }],
        },
    ),
    (
        AIGatewaySecretOperation::SecretReferencesGet,
        GatewaySecretRules {
            request: &[],
            response: &[GatewaySecretRule {
                path: &["auth_config"],
                redact: GatewaySecretRedaction::Subtree,
                one_time: None,
            }],
        },
    ),
    (
        AIGatewaySecretOperation::SecretReferencesUpdate,
        GatewaySecretRules {
            request: &[GatewaySecretRule {
                path: &["auth_config"],
                redact: GatewaySecretRedaction::Subtree,
                one_time: None,
            }],
            response: &[GatewaySecretRule {
                path: &["auth_config"],
                redact: GatewaySecretRedaction::Subtree,
                one_time: None,
            }],
        },
    ),
    (
        AIGatewaySecretOperation::SecretReferencesDelete,
        GatewaySecretRules {
            request: &[],
            response: &[GatewaySecretRule {
                path: &["auth_config"],
                redact: GatewaySecretRedaction::Subtree,
                one_time: None,
            }],
        },
    ),
    (
        AIGatewaySecretOperation::ApiKeysCreateService,
        GatewaySecretRules {
            request: &[],
            response: &[GatewaySecretRule {
                path: &["key"],
                redact: GatewaySecretRedaction::Value,
                one_time: Some(true),
            }],
        },
    ),
    (
        AIGatewaySecretOperation::ApiKeysCreateUser,
        GatewaySecretRules {
            request: &[],
            response: &[GatewaySecretRule {
                path: &["key"],
                redact: GatewaySecretRedaction::Value,
                one_time: Some(true),
            }],
        },
    ),
    (
        AIGatewaySecretOperation::ApiKeysRotateService,
        GatewaySecretRules {
            request: &[],
            response: &[GatewaySecretRule {
                path: &["key"],
                redact: GatewaySecretRedaction::Value,
                one_time: Some(true),
            }],
        },
    ),
    (
        AIGatewaySecretOperation::ApiKeysRotateUser,
        GatewaySecretRules {
            request: &[],
            response: &[GatewaySecretRule {
                path: &["key"],
                redact: GatewaySecretRedaction::Value,
                one_time: Some(true),
            }],
        },
    ),
    (
        AIGatewaySecretOperation::DeploymentsCreate,
        GatewaySecretRules {
            request: &[],
            response: &[
                GatewaySecretRule {
                    path: &["client_auth"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: Some(true),
                },
                GatewaySecretRule {
                    path: &["credentials", "password"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: Some(true),
                },
            ],
        },
    ),
    (
        AIGatewaySecretOperation::DeploymentsUpdate,
        GatewaySecretRules {
            request: &[],
            response: &[
                GatewaySecretRule {
                    path: &["client_auth"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: Some(true),
                },
                GatewaySecretRule {
                    path: &["credentials", "password"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: Some(true),
                },
            ],
        },
    ),
    (
        AIGatewaySecretOperation::IntegrationsCreate,
        GatewaySecretRules {
            request: &[
                GatewaySecretRule {
                    path: &["key"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["configurations", "azure_entra_client_secret"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["configurations", "aws_secret_access_key"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["configurations", "vertex_service_account_json"],
                    redact: GatewaySecretRedaction::Subtree,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["configurations", "custom_headers", "*"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
            ],
            response: &[],
        },
    ),
    (
        AIGatewaySecretOperation::IntegrationsUpdate,
        GatewaySecretRules {
            request: &[
                GatewaySecretRule {
                    path: &["key"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["configurations", "azure_entra_client_secret"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["configurations", "aws_secret_access_key"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["configurations", "vertex_service_account_json"],
                    redact: GatewaySecretRedaction::Subtree,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["configurations", "custom_headers", "*"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
            ],
            response: &[],
        },
    ),
    (
        AIGatewaySecretOperation::McpIntegrationsCreate,
        GatewaySecretRules {
            request: &[
                GatewaySecretRule {
                    path: &["configurations", "custom_headers", "*"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["configurations", "client_secret"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["configurations", "oauth_client_secret"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["configurations", "oauth_metadata"],
                    redact: GatewaySecretRedaction::Subtree,
                    one_time: None,
                },
            ],
            response: &[],
        },
    ),
    (
        AIGatewaySecretOperation::McpIntegrationsUpdate,
        GatewaySecretRules {
            request: &[
                GatewaySecretRule {
                    path: &["configurations", "custom_headers", "*"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["configurations", "client_secret"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["configurations", "oauth_client_secret"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["configurations", "oauth_metadata"],
                    redact: GatewaySecretRedaction::Subtree,
                    one_time: None,
                },
            ],
            response: &[],
        },
    ),
    (
        AIGatewaySecretOperation::OrganisationsGetAuthSettings,
        GatewaySecretRules {
            request: &[],
            response: &[
                GatewaySecretRule {
                    path: &["scim_token"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["data", "scim_token"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
            ],
        },
    ),
    (
        AIGatewaySecretOperation::OrganisationsUpdateAuthSettings,
        GatewaySecretRules {
            request: &[
                GatewaySecretRule {
                    path: &["scim_token"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["auth_settings", "client_secret"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
            ],
            response: &[
                GatewaySecretRule {
                    path: &["scim_token"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["data", "scim_token"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
            ],
        },
    ),
    (
        AIGatewaySecretOperation::PluginsCreate,
        GatewaySecretRules {
            request: &[GatewaySecretRule {
                path: &["credentials", "*"],
                redact: GatewaySecretRedaction::Value,
                one_time: None,
            }],
            response: &[],
        },
    ),
    (
        AIGatewaySecretOperation::ProvidersGet,
        GatewaySecretRules {
            request: &[],
            response: &[
                GatewaySecretRule {
                    path: &["key"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["model_config"],
                    redact: GatewaySecretRedaction::Subtree,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["credentials"],
                    redact: GatewaySecretRedaction::Subtree,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["configurations", "azure_entra_client_secret"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["configurations", "aws_secret_access_key"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["configurations", "vertex_service_account_json"],
                    redact: GatewaySecretRedaction::Subtree,
                    one_time: None,
                },
                GatewaySecretRule {
                    path: &["configurations", "custom_headers", "*"],
                    redact: GatewaySecretRedaction::Value,
                    one_time: None,
                },
            ],
        },
    ),
];
pub const AI_GW_ADMIN_ENDPOINT: &str = "PANW_AI_GW_ADMIN_ENDPOINT";
pub const AI_GW_API_KEYS_SERVICE_PATH: &str = "/api-keys/service";
pub const AI_GW_API_KEYS_USER_PATH: &str = "/api-keys/user";
pub const AI_GW_AUDIT_LOGS_PATH: &str = "/audit-logs";
pub const AI_GW_CHART_METRICS: &[&str] = &[
    "cost",
    "requests",
    "latency",
    "tokens",
    "errors",
    "users",
    "cache-summary",
    "cache-hit-trend",
    "user-trends",
    "error-trends",
    "error-category-trends",
    "grouped-errors",
    "rescued-retries",
    "feedback-trend",
    "feedback-weighted",
    "feedback-score-distribution",
    "feedback-models",
];
pub const AI_GW_CHARTS_PATH: &str = "/logs/charts";
pub const AI_GW_CONFIGS_PATH: &str = "/configs";
pub const AI_GW_DATA_ENDPOINT: &str = "PANW_AI_GW_DATA_ENDPOINT";
pub const AI_GW_DEPLOYMENTS_PATH: &str = "/deployments";
pub const AI_GW_GROUP_COLUMNS: &[&str] = &[
    "cost",
    "avg_latency",
    "avg_tokens",
    "total_tokens",
    "success_rate",
    "last_seen",
];
pub const AI_GW_GROUP_DIMENSIONS: &[&str] = &["ai_service", "model", "api_key", "provider"];
pub const AI_GW_GROUPS_PATH: &str = "/logs/groups";
pub const AI_GW_GUARDRAILS_PATH: &str = "/guardrails";
pub const AI_GW_INTEGRATIONS_PATH: &str = "/integrations";
pub const AI_GW_LOGS_PATH: &str = "/logs";
pub const AI_GW_MCP_INTEGRATIONS_PATH: &str = "/mcp-integrations";
pub const AI_GW_ORGANISATIONS_SELF_PATH: &str = "/organisations/self";
pub const AI_GW_PLUGINS_PATH: &str = "/plugins";
pub const AI_GW_PROVIDER_CATALOG_PATH: &str = "/utils/static-resources/ai-providers";
pub const AI_GW_PROVIDERS_PATH: &str = "/providers";
pub const AI_GW_WORKSPACES_PATH: &str = "/workspaces";
pub const AI_SEC_API_ENDPOINT: &str = "PANW_AI_SEC_API_ENDPOINT";
pub const AI_SEC_API_KEY: &str = "PANW_AI_SEC_API_KEY";
pub const AI_SEC_API_TOKEN: &str = "PANW_AI_SEC_API_TOKEN";
pub const AIRS_ENDPOINTS: &[(&str, &str)] = &[
    ("US", "https://service.api.aisecurity.paloaltonetworks.com"),
    (
        "EU",
        "https://service-de.api.aisecurity.paloaltonetworks.com",
    ),
    (
        "INDIA",
        "https://service-in.api.aisecurity.paloaltonetworks.com",
    ),
    (
        "SINGAPORE",
        "https://service-sg.api.aisecurity.paloaltonetworks.com",
    ),
];
pub const ASYNC_SCAN_PATH: &str = "/v1/scan/async/request";
pub const BEARER: &str = "Bearer ";
pub const DEFAULT_AI_GW_ADMIN_ENDPOINT: &str =
    "https://api.apps.paloaltonetworks.com/ai_gw/admin/v2";
pub const DEFAULT_AI_GW_DATA_ENDPOINT: &str = "https://api.apps.paloaltonetworks.com/ai_gw/v2";
pub const DEFAULT_DLP_ENDPOINT: &str = "https://api.dlp.paloaltonetworks.com";
pub const DEFAULT_ENDPOINT: &str = "https://service.api.aisecurity.paloaltonetworks.com";
pub const DEFAULT_IAM_ENDPOINT: &str = "https://api.apps.paloaltonetworks.com/iam/v1";
pub const DEFAULT_MGMT_ENDPOINT: &str = "https://api.apps.paloaltonetworks.com/aisec";
pub const DEFAULT_MODEL_SEC_DATA_ENDPOINT: &str = "https://api.apps.paloaltonetworks.com/aims/data";
pub const DEFAULT_MODEL_SEC_MGMT_ENDPOINT: &str = "https://api.apps.paloaltonetworks.com/aims/mgmt";
pub const DEFAULT_RED_TEAM_DATA_ENDPOINT: &str =
    "https://api.apps.paloaltonetworks.com/ai-red-teaming/data-plane";
pub const DEFAULT_RED_TEAM_MGMT_ENDPOINT: &str =
    "https://api.apps.paloaltonetworks.com/ai-red-teaming/mgmt-plane";
pub const DEFAULT_RED_TEAM_NETWORK_BROKER_ENDPOINT: &str =
    "https://api.apps.paloaltonetworks.com/ai-red-teaming/data-plane/network-broker";
pub const DEFAULT_TOKEN_ENDPOINT: &str =
    "https://auth.apps.paloaltonetworks.com/oauth2/access_token";
pub const DLP_DATA_FILTERING_PROFILES_PATH: &str = "/v2/api/data-filtering-profiles";
pub const DLP_DATA_PATTERNS_PATH: &str = "/v2/api/data-patterns";
pub const DLP_DATA_PROFILES_PATH: &str = "/v2/api/data-profiles";
pub const DLP_DICTIONARIES_PATH: &str = "/v2/api/dictionaries";
pub const HEADER_API_KEY: &str = "x-pan-token";
pub const HEADER_AUTH_TOKEN: &str = "Authorization";
pub const HTTP_FORCE_RETRY_STATUS_CODES: &[u16] = &[500, 502, 503, 504];
pub const IAM_ENDPOINT: &str = "PANW_IAM_ENDPOINT";
pub const IAM_SCOPES_PATH: &str = "/scopes";
pub const MAX_AI_PROFILE_NAME_LENGTH: usize = 100;
pub const MAX_API_KEY_LENGTH: usize = 2048;
pub const MAX_CONNECTION_POOL_SIZE: usize = 100;
pub const MAX_CONTENT_CONTEXT_LENGTH: usize = 104857600;
pub const MAX_CONTENT_PROMPT_LENGTH: usize = 2097152;
pub const MAX_CONTENT_RESPONSE_LENGTH: usize = 2097152;
pub const MAX_NUMBER_OF_BATCH_SCAN_OBJECTS: usize = 20;
pub const MAX_NUMBER_OF_REPORT_IDS: usize = 5;
pub const MAX_NUMBER_OF_RETRIES: usize = 5;
pub const MAX_NUMBER_OF_SCAN_IDS: usize = 5;
pub const MAX_REPORT_ID_STR_LENGTH: usize = 40;
pub const MAX_SCAN_ID_STR_LENGTH: usize = 36;
pub const MAX_SESSION_ID_STR_LENGTH: usize = 100;
pub const MAX_TOKEN_LENGTH: usize = 2048;
pub const MAX_TRANSACTION_ID_STR_LENGTH: usize = 100;
pub const MGMT_API_KEY_PATH: &str = "/v1/mgmt/apikey";
pub const MGMT_API_KEYS_TSG_PATH: &str = "/v1/mgmt/apikeys/tsg";
pub const MGMT_CLIENT_ID: &str = "PANW_MGMT_CLIENT_ID";
pub const MGMT_CLIENT_SECRET: &str = "PANW_MGMT_CLIENT_SECRET";
pub const MGMT_CUSTOMER_APP_PATH: &str = "/v1/mgmt/customerapp";
pub const MGMT_CUSTOMER_APPS_TSG_PATH: &str = "/v1/mgmt/customerapp/tsg";
pub const MGMT_DASHBOARD_APPLICATION_PATH: &str = "/v1/mgmt/dashboard/v2/apps/application";
pub const MGMT_DASHBOARD_APPLICATION_VIOLATION_BREAKDOWN_PATH: &str =
    "/v1/mgmt/dashboard/v2/apps/applicationviolationbreakdown";
pub const MGMT_DASHBOARD_APPLICATIONS_OVERVIEW_PATH: &str =
    "/v1/mgmt/dashboard/v2/apps/applicationsoverview";
pub const MGMT_DASHBOARD_APPLICATIONS_VIOLATIONS_TREND_PATH: &str =
    "/v1/mgmt/dashboard/v2/apps/applicationsviolationstrend";
pub const MGMT_DASHBOARD_APPS_LIST_PATH: &str = "/v1/mgmt/dashboard/v2/apps/appslist";
pub const MGMT_DASHBOARD_SESSION_PATH: &str = "/v1/mgmt/dashboard/v2/sessions/session";
pub const MGMT_DASHBOARD_SESSION_TRANSACTION_PATH: &str =
    "/v1/mgmt/dashboard/v2/sessions/sessiontransaction";
pub const MGMT_DASHBOARD_SESSIONS_CHART_PATH: &str = "/v1/mgmt/dashboard/v2/sessions/sessionschart";
pub const MGMT_DASHBOARD_SESSIONS_OVERVIEW_PATH: &str =
    "/v1/mgmt/dashboard/v2/sessions/sessionsoverview";
pub const MGMT_DASHBOARD_TOP_APPLICATIONS_VIOLATIONS_PATH: &str =
    "/v1/mgmt/dashboard/v2/apps/topapplicationsviolations";
pub const MGMT_DEPLOYMENT_PROFILES_PATH: &str = "/v1/mgmt/deploymentprofiles";
pub const MGMT_DLP_PROFILES_PATH: &str = "/v1/mgmt/dlpprofiles";
pub const MGMT_ENDPOINT: &str = "PANW_MGMT_ENDPOINT";
pub const MGMT_OAUTH_INVALIDATE_PATH: &str = "/v1/mgmt/oauth/invalidateToken";
pub const MGMT_OAUTH_TOKEN_PATH: &str = "/v1/mgmt/oauth/client_credential/accesstoken";
pub const MGMT_PROFILE_PATH: &str = "/v1/mgmt/profile";
pub const MGMT_PROFILES_TSG_PATH: &str = "/v1/mgmt/profiles/tsg";
pub const MGMT_REPORT_SCAN_CONTENT_PATH: &str = "/v1/mgmt/reports/scancontent";
pub const MGMT_SCAN_LOGS_PATH: &str = "/v1/mgmt/scanlogs";
pub const MGMT_TOKEN_ENDPOINT: &str = "PANW_MGMT_TOKEN_ENDPOINT";
pub const MGMT_TOPIC_FORCE_PATH: &str = "/v1/mgmt/topic/force";
pub const MGMT_TOPIC_PATH: &str = "/v1/mgmt/topic";
pub const MGMT_TOPICS_TSG_PATH: &str = "/v1/mgmt/topics/tsg";
pub const MGMT_TSG_ID: &str = "PANW_MGMT_TSG_ID";
pub const MODEL_SEC_CLIENT_ID: &str = "PANW_MODEL_SEC_CLIENT_ID";
pub const MODEL_SEC_CLIENT_SECRET: &str = "PANW_MODEL_SEC_CLIENT_SECRET";
pub const MODEL_SEC_DATA_ENDPOINT: &str = "PANW_MODEL_SEC_DATA_ENDPOINT";
pub const MODEL_SEC_EVALUATIONS_PATH: &str = "/v1/evaluations";
pub const MODEL_SEC_MGMT_ENDPOINT: &str = "PANW_MODEL_SEC_MGMT_ENDPOINT";
pub const MODEL_SEC_MODEL_VERSIONS_PATH: &str = "/v1/model-versions";
pub const MODEL_SEC_MODELS_PATH: &str = "/v1/models";
pub const MODEL_SEC_PYPI_AUTH_PATH: &str = "/v1/pypi/authenticate";
pub const MODEL_SEC_SCANS_PATH: &str = "/v1/scans";
pub const MODEL_SEC_SECURITY_GROUPS_PATH: &str = "/v1/security-groups";
pub const MODEL_SEC_SECURITY_RULES_PATH: &str = "/v1/security-rules";
pub const MODEL_SEC_TOKEN_ENDPOINT: &str = "PANW_MODEL_SEC_TOKEN_ENDPOINT";
pub const MODEL_SEC_TSG_ID: &str = "PANW_MODEL_SEC_TSG_ID";
pub const MODEL_SEC_VIOLATIONS_PATH: &str = "/v1/violations";
pub const PAYLOAD_HASH: &str = "x-payload-hash";
pub const RED_TEAM_ADAPTER_PATH: &str = "/v1/adapters";
pub const RED_TEAM_ADAPTER_VALIDATE_PATH: &str = "/v1/adapters/validate";
pub const RED_TEAM_CATEGORIES_PATH: &str = "/v1/categories";
pub const RED_TEAM_CHANNELS_PATH: &str = "/v1/channels";
pub const RED_TEAM_CHANNELS_STATS_PATH: &str = "/v1/channels/stats";
pub const RED_TEAM_CLIENT_ID: &str = "PANW_RED_TEAM_CLIENT_ID";
pub const RED_TEAM_CLIENT_SECRET: &str = "PANW_RED_TEAM_CLIENT_SECRET";
pub const RED_TEAM_CUSTOM_ATTACK_PATH: &str = "/v1/custom-attack";
pub const RED_TEAM_CUSTOM_ATTACKS_REPORT_PATH: &str = "/v1/custom-attacks";
pub const RED_TEAM_DASHBOARD_PATH: &str = "/v1/dashboard";
pub const RED_TEAM_DATA_ENDPOINT: &str = "PANW_RED_TEAM_DATA_ENDPOINT";
pub const RED_TEAM_ERROR_LOG_PATH: &str = "/v1/error-log/job";
pub const RED_TEAM_ERROR_LOG_TARGET_PROFILE_PATH: &str = "/v1/error-log/target-profile";
pub const RED_TEAM_EULA_PATH: &str = "/v1/eula";
pub const RED_TEAM_INSTANCES_PATH: &str = "/v1/instances";
pub const RED_TEAM_LANGUAGES_PATH: &str = "/v1/languages";
pub const RED_TEAM_MGMT_DASHBOARD_PATH: &str = "/v1/dashboard/overview";
pub const RED_TEAM_MGMT_ENDPOINT: &str = "PANW_RED_TEAM_MGMT_ENDPOINT";
pub const RED_TEAM_NETWORK_BROKER_ENDPOINT: &str = "PANW_RED_TEAM_NETWORK_BROKER_ENDPOINT";
pub const RED_TEAM_QUOTA_PATH: &str = "/v1/metering/quota";
pub const RED_TEAM_REGISTRY_CREDENTIALS_PATH: &str = "/v1/registry-credentials";
pub const RED_TEAM_REPORT_DYNAMIC_PATH: &str = "/v1/report/dynamic";
pub const RED_TEAM_REPORT_PATH: &str = "/v1/report";
pub const RED_TEAM_REPORT_STATIC_PATH: &str = "/v1/report/static";
pub const RED_TEAM_SCAN_PATH: &str = "/v1/scan";
pub const RED_TEAM_SENTIMENT_PATH: &str = "/v1/sentiment";
pub const RED_TEAM_TARGET_PATH: &str = "/v1/target";
pub const RED_TEAM_TARGET_VALIDATE_AUTH_PATH: &str = "/v1/target/validate-auth";
pub const RED_TEAM_TEMPLATE_PATH: &str = "/v1/template";
pub const RED_TEAM_TOKEN_ENDPOINT: &str = "PANW_RED_TEAM_TOKEN_ENDPOINT";
pub const RED_TEAM_TSG_ID: &str = "PANW_RED_TEAM_TSG_ID";
pub const SCAN_REPORTS_PATH: &str = "/v1/scan/reports";
pub const SCAN_RESULTS_PATH: &str = "/v1/scan/results";
pub const SDK_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SYNC_SCAN_PATH: &str = "/v1/scan/sync/request";
pub const TSG_ID_HEADER: &str = "x-tsg-id";
pub const USER_AGENT: &str = concat!("PAN-AIRS/", env!("CARGO_PKG_VERSION"), "-rust-sdk");
