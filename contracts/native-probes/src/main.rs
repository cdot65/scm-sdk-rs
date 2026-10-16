//! Generated standalone public SDK capability consumer.
#![allow(clippy::let_underscore_future)]
#[rustfmt::skip]
fn main() {
    // Action
    { let _ = std::marker::PhantomData::<scm_sdk::shared::models::Action>; }
    // ActionType
    { let _ = std::marker::PhantomData::<scm_sdk::shared::models::ActionType>; }
    // AdapterConfigResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AdapterConfigResponse>; }
    // AdapterConfigResponseSchema
    { let _ = scm_sdk::validation::parse("AdapterConfigResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AdapterCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AdapterCreateRequest>; }
    // AdapterCreateRequestSchema
    { let _ = scm_sdk::validation::parse("AdapterCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AdapterList
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AdapterList>; }
    // AdapterListAllOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::AdapterListOptions>; }
    // AdapterListAllOptions.include_target_count
    { let _ = |value: &scm_sdk::airs::red_team::AdapterListOptions| { let _ = &value.include_target_count; }; }
    // AdapterListAllOptions.limit
    { let _ = |value: &scm_sdk::airs::red_team::AdapterListOptions| { let _ = &value.limit; }; }
    // AdapterListAllOptions.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // AdapterListAllOptions.search
    { let _ = |value: &scm_sdk::airs::red_team::AdapterListOptions| { let _ = &value.search; }; }
    // AdapterListAllOptions.status
    { let _ = |value: &scm_sdk::airs::red_team::AdapterListOptions| { let _ = &value.status; }; }
    // AdapterListItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AdapterListItem>; }
    // AdapterListItemSchema
    { let _ = scm_sdk::validation::parse("AdapterListItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AdapterListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::AdapterListOptions>; }
    // AdapterListOptions.include_target_count
    { let _ = |value: &scm_sdk::airs::red_team::AdapterListOptions| { let _ = &value.include_target_count; }; }
    // AdapterListOptions.limit
    { let _ = |value: &scm_sdk::airs::red_team::AdapterListOptions| { let _ = &value.limit; }; }
    // AdapterListOptions.search
    { let _ = |value: &scm_sdk::airs::red_team::AdapterListOptions| { let _ = &value.search; }; }
    // AdapterListOptions.skip
    { let _ = |value: &scm_sdk::airs::red_team::AdapterListOptions| { let _ = &value.skip; }; }
    // AdapterListOptions.status
    { let _ = |value: &scm_sdk::airs::red_team::AdapterListOptions| { let _ = &value.status; }; }
    // AdapterListSchema
    { let _ = scm_sdk::validation::parse("AdapterListSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AdapterOperationOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::AdapterOperationOptions>; }
    // AdapterOperationOptions.validate
    { let _ = |value: &scm_sdk::airs::red_team::AdapterOperationOptions| { let _ = &value.validate; }; }
    // AdapterResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AdapterResponse>; }
    // AdapterResponseSchema
    { let _ = scm_sdk::validation::parse("AdapterResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AdapterUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AdapterUpdateRequest>; }
    // AdapterUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("AdapterUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AdapterValidateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AdapterValidateRequest>; }
    // AdapterValidateRequestSchema
    { let _ = scm_sdk::validation::parse("AdapterValidateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AdapterValidateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AdapterValidateResponse>; }
    // AdapterValidateResponseSchema
    { let _ = scm_sdk::validation::parse("AdapterValidateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AdapterVar
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AdapterVar>; }
    // AdapterVarResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AdapterVarResponse>; }
    // AdapterVarResponseSchema
    { let _ = scm_sdk::validation::parse("AdapterVarResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AdapterVarSchema
    { let _ = scm_sdk::validation::parse("AdapterVarSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AdapterVarType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AdapterVarType>; }
    // AdapterVarTypeSchema
    { let _ = scm_sdk::validation::parse("AdapterVarTypeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AdvancedDataProfileRequest
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::AdvancedDataProfileRequest>; }
    // AdvancedDataProfileRequestSchema
    { let _ = scm_sdk::validation::parse("AdvancedDataProfileRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AGENT_GUARD_DATA_ENDPOINT
    { let _ = scm_sdk::constants::AGENT_GUARD_DATA_ENDPOINT; }
    // AGENT_GUARD_MGMT_ENDPOINT
    { let _ = scm_sdk::constants::AGENT_GUARD_MGMT_ENDPOINT; }
    // AGENT_GUARD_RULES_PATH
    { let _ = scm_sdk::constants::AGENT_GUARD_RULES_PATH; }
    // AGENT_GUARD_SCANS_PATH
    { let _ = scm_sdk::constants::AGENT_GUARD_SCANS_PATH; }
    // AGENT_GUARD_STATS_PATH
    { let _ = scm_sdk::constants::AGENT_GUARD_STATS_PATH; }
    // AgentEntry
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::AgentEntry>; }
    // AgentEntrySchema
    { let _ = scm_sdk::validation::parse("AgentEntrySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AgentGuardClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::agentguard::AgentGuardClient>; let _ = scm_sdk::airs::agentguard::AgentGuardClient::new; }
    // AgentGuardClient.getScanStats
    { let _ = scm_sdk::airs::agentguard::AgentGuardClient::get_scan_stats; }
    // AgentGuardClient.listRules
    { let _ = scm_sdk::airs::agentguard::AgentGuardClient::list_rules; }
    // AgentGuardClient.listScans
    { let _ = scm_sdk::airs::agentguard::AgentGuardClient::list_scans; }
    // AgentGuardClient.listScanVulnerabilities
    { let _ = scm_sdk::airs::agentguard::AgentGuardClient::list_scan_vulnerabilities; }
    // AgentGuardClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::ClientBuilder>; let _ = scm_sdk::airs::agentguard::AgentGuardClient::new; }
    // AgentGuardClientOptions.clientId
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer,"fixture",1,"123"); }; }
    // AgentGuardClientOptions.clientSecret
    { let _ = scm_sdk::ClientBuilder::oauth_credentials; }
    // AgentGuardClientOptions.dataEndpoint
    { let _ = scm_sdk::airs::agentguard::AgentGuardClient::new; let _ = scm_sdk::Client::with_endpoint; }
    // AgentGuardClientOptions.mgmtEndpoint
    { let _ = scm_sdk::airs::agentguard::AgentGuardClient::new; let _ = scm_sdk::Client::with_endpoint; }
    // AgentGuardClientOptions.numRetries
    { let _ = scm_sdk::ClientBuilder::limits; let _ = scm_sdk::RequestOptions::with_retries; }
    // AgentGuardClientOptions.tokenEndpoint
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer,"fixture",1,"123"); }; }
    // AgentGuardClientOptions.tsgId
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer,"fixture",1,"123"); }; }
    // AgentGuardPageOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::agentguard::models::AgentGuardPageOptions>; }
    // AgentGuardPageOptionsSchema
    { let _ = scm_sdk::validation::parse("AgentGuardPageOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AgentGuardRule
    { let _ = std::marker::PhantomData::<scm_sdk::airs::agentguard::models::AgentGuardRule>; }
    // AgentGuardRuleList
    { let _ = std::marker::PhantomData::<scm_sdk::airs::agentguard::models::AgentGuardRuleList>; }
    // AgentGuardRuleListSchema
    { let _ = scm_sdk::validation::parse("AgentGuardRuleListSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AgentGuardRuleSchema
    { let _ = scm_sdk::validation::parse("AgentGuardRuleSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AgentGuardScan
    { let _ = std::marker::PhantomData::<scm_sdk::airs::agentguard::models::AgentGuardScan>; }
    // AgentGuardScanList
    { let _ = std::marker::PhantomData::<scm_sdk::airs::agentguard::models::AgentGuardScanList>; }
    // AgentGuardScanListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::agentguard::models::AgentGuardScanListOptions>; }
    // AgentGuardScanListOptionsSchema
    { let _ = scm_sdk::validation::parse("AgentGuardScanListOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AgentGuardScanListSchema
    { let _ = scm_sdk::validation::parse("AgentGuardScanListSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AgentGuardScanSchema
    { let _ = scm_sdk::validation::parse("AgentGuardScanSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AgentGuardScanStats
    { let _ = std::marker::PhantomData::<scm_sdk::airs::agentguard::models::AgentGuardScanStats>; }
    // AgentGuardScanStatsSchema
    { let _ = scm_sdk::validation::parse("AgentGuardScanStatsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AgentGuardStatsOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::agentguard::models::AgentGuardStatsOptions>; }
    // AgentGuardStatsOptionsSchema
    { let _ = scm_sdk::validation::parse("AgentGuardStatsOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AgentGuardVulnerability
    { let _ = std::marker::PhantomData::<scm_sdk::airs::agentguard::models::AgentGuardVulnerability>; }
    // AgentGuardVulnerabilityList
    { let _ = std::marker::PhantomData::<scm_sdk::airs::agentguard::models::AgentGuardVulnerabilityList>; }
    // AgentGuardVulnerabilityListSchema
    { let _ = scm_sdk::validation::parse("AgentGuardVulnerabilityListSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AgentGuardVulnerabilitySchema
    { let _ = scm_sdk::validation::parse("AgentGuardVulnerabilitySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AgentMeta
    { let _ = std::marker::PhantomData::<scm_sdk::shared::models::AgentMeta>; }
    // AgentMetaSchema
    { let _ = scm_sdk::validation::parse("AgentMetaSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AgentProtectionItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::AgentProtectionItem>; }
    // AgentProtectionItemSchema
    { let _ = scm_sdk::validation::parse("AgentProtectionItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AgentReport
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::AgentReport>; }
    // AgentReportSchema
    { let _ = scm_sdk::validation::parse("AgentReportSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AI_GATEWAY_DEPLOYMENT_STATUSES
    { let _ = scm_sdk::constants::AI_GATEWAY_DEPLOYMENT_STATUSES; }
    // AI_GATEWAY_DEPLOYMENT_TYPES
    { let _ = scm_sdk::constants::AI_GATEWAY_DEPLOYMENT_TYPES; }
    // AI_GATEWAY_KNOWN_API_KEY_SCOPES
    { let _ = scm_sdk::constants::AI_GATEWAY_KNOWN_API_KEY_SCOPES; }
    // AI_GATEWAY_KNOWN_CACHE_MODES
    { let _ = scm_sdk::constants::AI_GATEWAY_KNOWN_CACHE_MODES; }
    // AI_GATEWAY_KNOWN_CONFIG_STRATEGIES
    { let _ = scm_sdk::constants::AI_GATEWAY_KNOWN_CONFIG_STRATEGIES; }
    // AI_GATEWAY_KNOWN_MCP_AUTH_TYPES
    { let _ = scm_sdk::constants::AI_GATEWAY_KNOWN_MCP_AUTH_TYPES; }
    // AI_GATEWAY_KNOWN_MCP_TRANSPORTS
    { let _ = scm_sdk::constants::AI_GATEWAY_KNOWN_MCP_TRANSPORTS; }
    // AI_GATEWAY_KNOWN_RATE_LIMIT_TYPES
    { let _ = scm_sdk::constants::AI_GATEWAY_KNOWN_RATE_LIMIT_TYPES; }
    // AI_GATEWAY_KNOWN_RATE_LIMIT_UNITS
    { let _ = scm_sdk::constants::AI_GATEWAY_KNOWN_RATE_LIMIT_UNITS; }
    // AI_GATEWAY_MUTABLE_MCP_CAPABILITY_TYPES
    { let _ = scm_sdk::constants::AI_GATEWAY_MUTABLE_MCP_CAPABILITY_TYPES; }
    // AI_GATEWAY_REDACTED
    { let _ = scm_sdk::constants::AI_GATEWAY_REDACTED; }
    // AI_GATEWAY_SECRET_FIELDS
    { let _ = scm_sdk::constants::AI_GATEWAY_SECRET_FIELDS; }
    // AI_GW_ADMIN_ENDPOINT
    { let _ = scm_sdk::constants::AI_GW_ADMIN_ENDPOINT; }
    // AI_GW_API_KEYS_SERVICE_PATH
    { let _ = scm_sdk::constants::AI_GW_API_KEYS_SERVICE_PATH; }
    // AI_GW_API_KEYS_USER_PATH
    { let _ = scm_sdk::constants::AI_GW_API_KEYS_USER_PATH; }
    // AI_GW_AUDIT_LOGS_PATH
    { let _ = scm_sdk::constants::AI_GW_AUDIT_LOGS_PATH; }
    // AI_GW_CHART_METRICS
    { let _ = scm_sdk::constants::AI_GW_CHART_METRICS; }
    // AI_GW_CHARTS_PATH
    { let _ = scm_sdk::constants::AI_GW_CHARTS_PATH; }
    // AI_GW_CONFIGS_PATH
    { let _ = scm_sdk::constants::AI_GW_CONFIGS_PATH; }
    // AI_GW_DATA_ENDPOINT
    { let _ = scm_sdk::constants::AI_GW_DATA_ENDPOINT; }
    // AI_GW_DEPLOYMENTS_PATH
    { let _ = scm_sdk::constants::AI_GW_DEPLOYMENTS_PATH; }
    // AI_GW_GROUP_COLUMNS
    { let _ = scm_sdk::constants::AI_GW_GROUP_COLUMNS; }
    // AI_GW_GROUP_DIMENSIONS
    { let _ = scm_sdk::constants::AI_GW_GROUP_DIMENSIONS; }
    // AI_GW_GROUPS_PATH
    { let _ = scm_sdk::constants::AI_GW_GROUPS_PATH; }
    // AI_GW_GUARDRAILS_PATH
    { let _ = scm_sdk::constants::AI_GW_GUARDRAILS_PATH; }
    // AI_GW_INTEGRATIONS_PATH
    { let _ = scm_sdk::constants::AI_GW_INTEGRATIONS_PATH; }
    // AI_GW_LOGS_PATH
    { let _ = scm_sdk::constants::AI_GW_LOGS_PATH; }
    // AI_GW_MCP_INTEGRATIONS_PATH
    { let _ = scm_sdk::constants::AI_GW_MCP_INTEGRATIONS_PATH; }
    // AI_GW_ORGANISATIONS_SELF_PATH
    { let _ = scm_sdk::constants::AI_GW_ORGANISATIONS_SELF_PATH; }
    // AI_GW_PLUGINS_PATH
    { let _ = scm_sdk::constants::AI_GW_PLUGINS_PATH; }
    // AI_GW_PROVIDER_CATALOG_PATH
    { let _ = scm_sdk::constants::AI_GW_PROVIDER_CATALOG_PATH; }
    // AI_GW_PROVIDERS_PATH
    { let _ = scm_sdk::constants::AI_GW_PROVIDERS_PATH; }
    // AI_GW_WORKSPACES_PATH
    { let _ = scm_sdk::constants::AI_GW_WORKSPACES_PATH; }
    // AI_SEC_API_ENDPOINT
    { let _ = scm_sdk::constants::AI_SEC_API_ENDPOINT; }
    // AI_SEC_API_KEY
    { let _ = scm_sdk::constants::AI_SEC_API_KEY; }
    // AI_SEC_API_TOKEN
    { let _ = scm_sdk::constants::AI_SEC_API_TOKEN; }
    // AIGatewayAdminGuardrailsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayAdminGuardrailsClient>; let _ = scm_sdk::airs::gateway::AIGatewayAdminGuardrailsClient::new; }
    // AIGatewayAdminGuardrailsClient.create
    { let _ = scm_sdk::airs::gateway::AIGatewayAdminGuardrailsClient::create; }
    // AIGatewayAdminGuardrailsClient.delete
    { let _ = scm_sdk::airs::gateway::AIGatewayAdminGuardrailsClient::delete; }
    // AIGatewayAdminGuardrailsClient.get
    { let _ = scm_sdk::airs::gateway::AIGatewayAdminGuardrailsClient::get; }
    // AIGatewayAdminGuardrailsClient.getMcpServers
    { let _ = scm_sdk::airs::gateway::AIGatewayAdminGuardrailsClient::get_mcp_servers; }
    // AIGatewayAdminGuardrailsClient.list
    { let _ = scm_sdk::airs::gateway::AIGatewayAdminGuardrailsClient::list; }
    // AIGatewayAdminGuardrailsClient.syncMcpServers
    { let _ = scm_sdk::airs::gateway::AIGatewayAdminGuardrailsClient::sync_mcp_servers; }
    // AIGatewayAdminGuardrailsClient.update
    { let _ = scm_sdk::airs::gateway::AIGatewayAdminGuardrailsClient::update; }
    // AIGatewayAdminGuardrailsClient.upsertMcpServer
    { let _ = scm_sdk::airs::gateway::AIGatewayAdminGuardrailsClient::upsert_mcp_server; }
    // AIGatewayApiKeysClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayApiKeysClient>; let _ = scm_sdk::airs::gateway::AIGatewayApiKeysClient::new; }
    // AIGatewayApiKeysClient.createService
    { let _ = scm_sdk::airs::gateway::AIGatewayApiKeysClient::create_service; }
    // AIGatewayApiKeysClient.createUser
    { let _ = scm_sdk::airs::gateway::AIGatewayApiKeysClient::create_user; }
    // AIGatewayApiKeysClient.deleteService
    { let _ = scm_sdk::airs::gateway::AIGatewayApiKeysClient::delete_service; }
    // AIGatewayApiKeysClient.deleteUser
    { let _ = scm_sdk::airs::gateway::AIGatewayApiKeysClient::delete_user; }
    // AIGatewayApiKeysClient.getService
    { let _ = scm_sdk::airs::gateway::AIGatewayApiKeysClient::get_service; }
    // AIGatewayApiKeysClient.getUser
    { let _ = scm_sdk::airs::gateway::AIGatewayApiKeysClient::get_user; }
    // AIGatewayApiKeysClient.listService
    { let _ = scm_sdk::airs::gateway::AIGatewayApiKeysClient::list_service; }
    // AIGatewayApiKeysClient.listUser
    { let _ = scm_sdk::airs::gateway::AIGatewayApiKeysClient::list_user; }
    // AIGatewayApiKeysClient.rotateService
    { let _ = scm_sdk::airs::gateway::AIGatewayApiKeysClient::rotate_service; }
    // AIGatewayApiKeysClient.rotateUser
    { let _ = scm_sdk::airs::gateway::AIGatewayApiKeysClient::rotate_user; }
    // AIGatewayApiKeysClient.updateService
    { let _ = scm_sdk::airs::gateway::AIGatewayApiKeysClient::update_service; }
    // AIGatewayApiKeysClient.updateUser
    { let _ = scm_sdk::airs::gateway::AIGatewayApiKeysClient::update_user; }
    // AIGatewayAuditLogListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayAuditLogListOptions>; }
    // AIGatewayAuditLogListOptions.end
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayAuditLogListOptions| { let _ = &value.end; }; }
    // AIGatewayAuditLogListOptions.start
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayAuditLogListOptions| { let _ = &value.start; }; }
    // AIGatewayAuditLogsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayAuditLogsClient>; let _ = scm_sdk::airs::gateway::AIGatewayAuditLogsClient::new; }
    // AIGatewayAuditLogsClient.list
    { let _ = scm_sdk::airs::gateway::AIGatewayAuditLogsClient::list; }
    // AIGatewayChartFilters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::AIGatewayChartFilters>; }
    // AIGatewayChartFiltersSchema
    { let _ = scm_sdk::validation::parse("AIGatewayChartFiltersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AIGatewayChartOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayChartOptions>; }
    // AIGatewayChartOptions.aiOrgModels
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayChartOptions| { let _ = &value.filters.ai_org_models; }; }
    // AIGatewayChartOptions.apiKeyIds
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayChartOptions| { let _ = &value.filters.api_key_ids; }; }
    // AIGatewayChartOptions.costMax
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayChartOptions| { let _ = &value.filters.cost_max; }; }
    // AIGatewayChartOptions.costMin
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayChartOptions| { let _ = &value.filters.cost_min; }; }
    // AIGatewayChartOptions.days
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayChartOptions| { let _ = &value.window.days; }; }
    // AIGatewayChartOptions.end
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayChartOptions| { let _ = &value.window.end; }; }
    // AIGatewayChartOptions.metadata
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayChartOptions| { let _ = &value.filters.metadata; }; }
    // AIGatewayChartOptions.start
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayChartOptions| { let _ = &value.window.start; }; }
    // AIGatewayChartOptions.statusCodes
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayChartOptions| { let _ = &value.filters.status_codes; }; }
    // AIGatewayChartOptions.totalUnitsMax
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayChartOptions| { let _ = &value.filters.total_units_max; }; }
    // AIGatewayChartOptions.totalUnitsMin
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayChartOptions| { let _ = &value.filters.total_units_min; }; }
    // AIGatewayChartOptions.traceId
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayChartOptions| { let _ = &value.filters.trace_id; }; }
    // AIGatewayChartOptions.workspaceSlug
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayChartOptions| { let _ = &value.window.workspace_slug; }; }
    // AIGatewayClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayClient>; let _ = scm_sdk::airs::gateway::AIGatewayClient::new; }
    // AIGatewayClient.adminGuardrails
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::admin_guardrails; }
    // AIGatewayClient.apiKeys
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::api_keys; }
    // AIGatewayClient.auditLogs
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::audit_logs; }
    // AIGatewayClient.configs
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::configs; }
    // AIGatewayClient.deployments
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::deployments; }
    // AIGatewayClient.guardrails
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::guardrails; }
    // AIGatewayClient.iamScopes
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::iam_scopes; }
    // AIGatewayClient.inference
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::inference; }
    // AIGatewayClient.integrations
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::integrations; }
    // AIGatewayClient.logExports
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::log_exports; }
    // AIGatewayClient.mcpIntegrations
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::mcp_integrations; }
    // AIGatewayClient.mcpServers
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::mcp_servers; }
    // AIGatewayClient.organisations
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::organisations; }
    // AIGatewayClient.plugins
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::plugins; }
    // AIGatewayClient.providers
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::providers; }
    // AIGatewayClient.rateLimits
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::rate_limits; }
    // AIGatewayClient.secretReferences
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::secret_references; }
    // AIGatewayClient.telemetry
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::telemetry; }
    // AIGatewayClient.usageLimits
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::usage_limits; }
    // AIGatewayClient.workspaces
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::workspaces; }
    // AIGatewayClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // AIGatewayClientOptions.adminEndpoint
    { let _ = scm_sdk::Client::with_endpoint; }
    // AIGatewayClientOptions.clientId
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer, "probe", 1, "1"); }; }
    // AIGatewayClientOptions.clientSecret
    { let _ = scm_sdk::ClientBuilder::oauth_credentials; }
    // AIGatewayClientOptions.dataEndpoint
    { let _ = scm_sdk::Client::builder; }
    // AIGatewayClientOptions.iamEndpoint
    { let _ = scm_sdk::Client::with_endpoint; }
    // AIGatewayClientOptions.inference
    { let _ = scm_sdk::airs::gateway::AIGatewayClient::with_runtime; }
    // AIGatewayClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // AIGatewayClientOptions.tokenEndpoint
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer, "probe", 1, "1"); }; }
    // AIGatewayClientOptions.tsgId
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer, "probe", 1, "1"); }; }
    // AIGatewayConfigsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayConfigsClient>; let _ = scm_sdk::airs::gateway::AIGatewayConfigsClient::new; }
    // AIGatewayConfigsClient.create
    { let _ = scm_sdk::airs::gateway::AIGatewayConfigsClient::create; }
    // AIGatewayConfigsClient.delete
    { let _ = scm_sdk::airs::gateway::AIGatewayConfigsClient::delete; }
    // AIGatewayConfigsClient.get
    { let _ = scm_sdk::airs::gateway::AIGatewayConfigsClient::get; }
    // AIGatewayConfigsClient.list
    { let _ = scm_sdk::airs::gateway::AIGatewayConfigsClient::list; }
    // AIGatewayConfigsClient.listVersions
    { let _ = scm_sdk::airs::gateway::AIGatewayConfigsClient::list_versions; }
    // AIGatewayConfigsClient.update
    { let _ = scm_sdk::airs::gateway::AIGatewayConfigsClient::update; }
    // AIGatewayDeploymentsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayDeploymentsClient>; let _ = scm_sdk::airs::gateway::AIGatewayDeploymentsClient::new; }
    // AIGatewayDeploymentsClient.create
    { let _ = scm_sdk::airs::gateway::AIGatewayDeploymentsClient::create; }
    // AIGatewayDeploymentsClient.delete
    { let _ = scm_sdk::airs::gateway::AIGatewayDeploymentsClient::delete; }
    // AIGatewayDeploymentsClient.get
    { let _ = scm_sdk::airs::gateway::AIGatewayDeploymentsClient::get; }
    // AIGatewayDeploymentsClient.list
    { let _ = scm_sdk::airs::gateway::AIGatewayDeploymentsClient::list; }
    // AIGatewayDeploymentsClient.ping
    { let _ = scm_sdk::airs::gateway::AIGatewayDeploymentsClient::ping; }
    // AIGatewayDeploymentsClient.update
    { let _ = scm_sdk::airs::gateway::AIGatewayDeploymentsClient::update; }
    // AIGatewayGroupOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayGroupOptions>; }
    // AIGatewayGroupOptions.aiOrgModels
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayGroupOptions| { let _ = &value.chart.filters.ai_org_models; }; }
    // AIGatewayGroupOptions.apiKeyIds
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayGroupOptions| { let _ = &value.chart.filters.api_key_ids; }; }
    // AIGatewayGroupOptions.columns
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayGroupOptions| { let _ = &value.columns; }; }
    // AIGatewayGroupOptions.costMax
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayGroupOptions| { let _ = &value.chart.filters.cost_max; }; }
    // AIGatewayGroupOptions.costMin
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayGroupOptions| { let _ = &value.chart.filters.cost_min; }; }
    // AIGatewayGroupOptions.days
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayGroupOptions| { let _ = &value.chart.window.days; }; }
    // AIGatewayGroupOptions.end
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayGroupOptions| { let _ = &value.chart.window.end; }; }
    // AIGatewayGroupOptions.metadata
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayGroupOptions| { let _ = &value.chart.filters.metadata; }; }
    // AIGatewayGroupOptions.start
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayGroupOptions| { let _ = &value.chart.window.start; }; }
    // AIGatewayGroupOptions.statusCodes
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayGroupOptions| { let _ = &value.chart.filters.status_codes; }; }
    // AIGatewayGroupOptions.totalUnitsMax
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayGroupOptions| { let _ = &value.chart.filters.total_units_max; }; }
    // AIGatewayGroupOptions.totalUnitsMin
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayGroupOptions| { let _ = &value.chart.filters.total_units_min; }; }
    // AIGatewayGroupOptions.traceId
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayGroupOptions| { let _ = &value.chart.filters.trace_id; }; }
    // AIGatewayGroupOptions.workspaceSlug
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayGroupOptions| { let _ = &value.chart.window.workspace_slug; }; }
    // AIGatewayGuardrailsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayGuardrailsClient>; let _ = scm_sdk::airs::gateway::AIGatewayGuardrailsClient::new; }
    // AIGatewayGuardrailsClient.create
    { let _ = scm_sdk::airs::gateway::AIGatewayGuardrailsClient::create; }
    // AIGatewayGuardrailsClient.delete
    { let _ = scm_sdk::airs::gateway::AIGatewayGuardrailsClient::delete; }
    // AIGatewayGuardrailsClient.get
    { let _ = scm_sdk::airs::gateway::AIGatewayGuardrailsClient::get; }
    // AIGatewayGuardrailsClient.getCatalog
    { let _ = scm_sdk::airs::gateway::AIGatewayGuardrailsClient::get_catalog; }
    // AIGatewayGuardrailsClient.getMcpServers
    { let _ = scm_sdk::airs::gateway::AIGatewayGuardrailsClient::get_mcp_servers; }
    // AIGatewayGuardrailsClient.list
    { let _ = scm_sdk::airs::gateway::AIGatewayGuardrailsClient::list; }
    // AIGatewayGuardrailsClient.syncMcpServers
    { let _ = scm_sdk::airs::gateway::AIGatewayGuardrailsClient::sync_mcp_servers; }
    // AIGatewayGuardrailsClient.update
    { let _ = scm_sdk::airs::gateway::AIGatewayGuardrailsClient::update; }
    // AIGatewayGuardrailsClient.upsertMcpServer
    { let _ = scm_sdk::airs::gateway::AIGatewayGuardrailsClient::upsert_mcp_server; }
    // AIGatewayInferenceClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::RuntimeClient>; let _ = scm_sdk::airs::gateway::RuntimeClient::new; }
    // AIGatewayInferenceClient.cancelBatch
    { let _ = scm_sdk::airs::gateway::RuntimeClient::cancel_batch; }
    // AIGatewayInferenceClient.cancelFineTuningJob
    { let _ = scm_sdk::airs::gateway::RuntimeClient::cancel_fine_tuning_job; }
    // AIGatewayInferenceClient.cancelVectorStoreFileBatch
    { let _ = scm_sdk::airs::gateway::RuntimeClient::cancel_vector_store_file_batch; }
    // AIGatewayInferenceClient.connectRealtime
    { let _ = scm_sdk::airs::gateway::RuntimeClient::connect_realtime; }
    // AIGatewayInferenceClient.createBatch
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_batch; }
    // AIGatewayInferenceClient.createChatCompletion
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_chat_completion; let _ = scm_sdk::airs::gateway::RuntimeClient::create_chat_completion_stream; }
    // AIGatewayInferenceClient.createCompletion
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_completion; let _ = scm_sdk::airs::gateway::RuntimeClient::create_completion_stream; }
    // AIGatewayInferenceClient.createEmbedding
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_embedding; }
    // AIGatewayInferenceClient.createFeedback
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_feedback; }
    // AIGatewayInferenceClient.createFile
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_file; }
    // AIGatewayInferenceClient.createFineTuningJob
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_fine_tuning_job; }
    // AIGatewayInferenceClient.createImage
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_image; }
    // AIGatewayInferenceClient.createImageEdit
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_image_edit; }
    // AIGatewayInferenceClient.createImageVariation
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_image_variation; }
    // AIGatewayInferenceClient.createLogs
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_logs; }
    // AIGatewayInferenceClient.createModeration
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_moderation; }
    // AIGatewayInferenceClient.createOcr
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_ocr; }
    // AIGatewayInferenceClient.createPromptCompletion
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_prompt_completion; let _ = scm_sdk::airs::gateway::RuntimeClient::create_prompt_completion_stream; }
    // AIGatewayInferenceClient.createPromptRender
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_prompt_render; }
    // AIGatewayInferenceClient.createRerank
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_rerank; }
    // AIGatewayInferenceClient.createResponse
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_response; let _ = scm_sdk::airs::gateway::RuntimeClient::create_response_stream; }
    // AIGatewayInferenceClient.createSpeech
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_speech; }
    // AIGatewayInferenceClient.createTranscription
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_transcription; }
    // AIGatewayInferenceClient.createTranslation
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_translation; }
    // AIGatewayInferenceClient.createVectorStore
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_vector_store; }
    // AIGatewayInferenceClient.createVectorStoreFile
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_vector_store_file; }
    // AIGatewayInferenceClient.createVectorStoreFileBatch
    { let _ = scm_sdk::airs::gateway::RuntimeClient::create_vector_store_file_batch; }
    // AIGatewayInferenceClient.deleteFile
    { let _ = scm_sdk::airs::gateway::RuntimeClient::delete_file; }
    // AIGatewayInferenceClient.deleteModel
    { let _ = scm_sdk::airs::gateway::RuntimeClient::delete_model; }
    // AIGatewayInferenceClient.deleteResponse
    { let _ = scm_sdk::airs::gateway::RuntimeClient::delete_response; }
    // AIGatewayInferenceClient.deleteVectorStore
    { let _ = scm_sdk::airs::gateway::RuntimeClient::delete_vector_store; }
    // AIGatewayInferenceClient.deleteVectorStoreFile
    { let _ = scm_sdk::airs::gateway::RuntimeClient::delete_vector_store_file; }
    // AIGatewayInferenceClient.downloadFile
    { let _ = scm_sdk::airs::gateway::RuntimeClient::download_file; }
    // AIGatewayInferenceClient.getBatchOutput
    { let _ = scm_sdk::airs::gateway::RuntimeClient::get_batch_output; }
    // AIGatewayInferenceClient.getLog
    { let _ = scm_sdk::airs::gateway::RuntimeClient::get_log; }
    // AIGatewayInferenceClient.getResponse
    { let _ = scm_sdk::airs::gateway::RuntimeClient::get_response; }
    // AIGatewayInferenceClient.getVectorStore
    { let _ = scm_sdk::airs::gateway::RuntimeClient::get_vector_store; }
    // AIGatewayInferenceClient.getVectorStoreFile
    { let _ = scm_sdk::airs::gateway::RuntimeClient::get_vector_store_file; }
    // AIGatewayInferenceClient.getVectorStoreFileBatch
    { let _ = scm_sdk::airs::gateway::RuntimeClient::get_vector_store_file_batch; }
    // AIGatewayInferenceClient.listBatches
    { let _ = scm_sdk::airs::gateway::RuntimeClient::list_batches; }
    // AIGatewayInferenceClient.listFiles
    { let _ = scm_sdk::airs::gateway::RuntimeClient::list_files; }
    // AIGatewayInferenceClient.listFilesInVectorStoreBatch
    { let _ = scm_sdk::airs::gateway::RuntimeClient::list_files_in_vector_store_batch; }
    // AIGatewayInferenceClient.listFineTuningEvents
    { let _ = scm_sdk::airs::gateway::RuntimeClient::list_fine_tuning_events; }
    // AIGatewayInferenceClient.listFineTuningJobCheckpoints
    { let _ = scm_sdk::airs::gateway::RuntimeClient::list_fine_tuning_job_checkpoints; }
    // AIGatewayInferenceClient.listInputItems
    { let _ = scm_sdk::airs::gateway::RuntimeClient::list_input_items; }
    // AIGatewayInferenceClient.listModels
    { let _ = scm_sdk::airs::gateway::RuntimeClient::list_models; }
    // AIGatewayInferenceClient.listPaginatedFineTuningJobs
    { let _ = scm_sdk::airs::gateway::RuntimeClient::list_paginated_fine_tuning_jobs; }
    // AIGatewayInferenceClient.listVectorStoreFiles
    { let _ = scm_sdk::airs::gateway::RuntimeClient::list_vector_store_files; }
    // AIGatewayInferenceClient.listVectorStores
    { let _ = scm_sdk::airs::gateway::RuntimeClient::list_vector_stores; }
    // AIGatewayInferenceClient.modifyVectorStore
    { let _ = scm_sdk::airs::gateway::RuntimeClient::modify_vector_store; }
    // AIGatewayInferenceClient.retrieveBatch
    { let _ = scm_sdk::airs::gateway::RuntimeClient::retrieve_batch; }
    // AIGatewayInferenceClient.retrieveFile
    { let _ = scm_sdk::airs::gateway::RuntimeClient::retrieve_file; }
    // AIGatewayInferenceClient.retrieveFineTuningJob
    { let _ = scm_sdk::airs::gateway::RuntimeClient::retrieve_fine_tuning_job; }
    // AIGatewayInferenceClient.retrieveModel
    { let _ = scm_sdk::airs::gateway::RuntimeClient::retrieve_model; }
    // AIGatewayInferenceClient.updateFeedback
    { let _ = scm_sdk::airs::gateway::RuntimeClient::update_feedback; }
    // AIGatewayInferenceClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::RuntimeClient>; }
    // AIGatewayInferenceClientOptions.apiKey
    { let _ = scm_sdk::auth::Authentication::runtime_key; }
    // AIGatewayInferenceClientOptions.endpoint
    { let _ = scm_sdk::Client::builder; }
    // AIGatewayInferenceClientOptions.fetch
    { let _ = scm_sdk::ClientBuilder::transport; }
    // AIGatewayInferenceClientOptions.maxEventBytes
    { let _ = |value: &scm_sdk::Limits| { let _ = value.max_event_bytes; }; }
    // AIGatewayInferenceClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // AIGatewayInferenceClientOptions.timeoutMs
    { let _ = |value: &scm_sdk::Limits| { let _ = value.operation_timeout; }; }
    // AIGatewayIntegrationsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayIntegrationsClient>; let _ = scm_sdk::airs::gateway::AIGatewayIntegrationsClient::new; }
    // AIGatewayIntegrationsClient.catalog
    { let _ = scm_sdk::airs::gateway::AIGatewayIntegrationsClient::catalog; }
    // AIGatewayIntegrationsClient.create
    { let _ = scm_sdk::airs::gateway::AIGatewayIntegrationsClient::create; }
    // AIGatewayIntegrationsClient.delete
    { let _ = scm_sdk::airs::gateway::AIGatewayIntegrationsClient::delete; }
    // AIGatewayIntegrationsClient.deleteModels
    { let _ = scm_sdk::airs::gateway::AIGatewayIntegrationsClient::delete_models; }
    // AIGatewayIntegrationsClient.get
    { let _ = scm_sdk::airs::gateway::AIGatewayIntegrationsClient::get; }
    // AIGatewayIntegrationsClient.getModels
    { let _ = scm_sdk::airs::gateway::AIGatewayIntegrationsClient::get_models; }
    // AIGatewayIntegrationsClient.getWorkspaces
    { let _ = scm_sdk::airs::gateway::AIGatewayIntegrationsClient::get_workspaces; }
    // AIGatewayIntegrationsClient.list
    { let _ = scm_sdk::airs::gateway::AIGatewayIntegrationsClient::list; }
    // AIGatewayIntegrationsClient.resolveProviderId
    { let _ = scm_sdk::airs::gateway::AIGatewayIntegrationsClient::resolve_provider_id; }
    // AIGatewayIntegrationsClient.setModels
    { let _ = scm_sdk::airs::gateway::AIGatewayIntegrationsClient::set_models; }
    // AIGatewayIntegrationsClient.setWorkspaces
    { let _ = scm_sdk::airs::gateway::AIGatewayIntegrationsClient::set_workspaces; }
    // AIGatewayIntegrationsClient.update
    { let _ = scm_sdk::airs::gateway::AIGatewayIntegrationsClient::update; }
    // AIGatewayLogExportsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayLogExportsClient>; let _ = scm_sdk::airs::gateway::AIGatewayLogExportsClient::new; }
    // AIGatewayLogExportsClient.cancel
    { let _ = scm_sdk::airs::gateway::AIGatewayLogExportsClient::cancel; }
    // AIGatewayLogExportsClient.create
    { let _ = scm_sdk::airs::gateway::AIGatewayLogExportsClient::create; }
    // AIGatewayLogExportsClient.download
    { let _ = scm_sdk::airs::gateway::AIGatewayLogExportsClient::download; }
    // AIGatewayLogExportsClient.get
    { let _ = scm_sdk::airs::gateway::AIGatewayLogExportsClient::get; }
    // AIGatewayLogExportsClient.list
    { let _ = scm_sdk::airs::gateway::AIGatewayLogExportsClient::list; }
    // AIGatewayLogExportsClient.start
    { let _ = scm_sdk::airs::gateway::AIGatewayLogExportsClient::start; }
    // AIGatewayLogExportsClient.update
    { let _ = scm_sdk::airs::gateway::AIGatewayLogExportsClient::update; }
    // AIGatewayLogsOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayLogsOptions>; }
    // AIGatewayLogsOptions.currentPage
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayLogsOptions| { let _ = &value.current_page; }; }
    // AIGatewayLogsOptions.days
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayLogsOptions| { let _ = &value.window.days; }; }
    // AIGatewayLogsOptions.end
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayLogsOptions| { let _ = &value.window.end; }; }
    // AIGatewayLogsOptions.pageSize
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayLogsOptions| { let _ = &value.page_size; }; }
    // AIGatewayLogsOptions.start
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayLogsOptions| { let _ = &value.window.start; }; }
    // AIGatewayLogsOptions.statusCode
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayLogsOptions| { let _ = &value.status_code; }; }
    // AIGatewayLogsOptions.traceId
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayLogsOptions| { let _ = &value.trace_id; }; }
    // AIGatewayLogsOptions.workspaceSlug
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayLogsOptions| { let _ = &value.window.workspace_slug; }; }
    // AIGatewayMcpIntegrationsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayMcpIntegrationsClient>; let _ = scm_sdk::airs::gateway::AIGatewayMcpIntegrationsClient::new; }
    // AIGatewayMcpIntegrationsClient.create
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpIntegrationsClient::create; }
    // AIGatewayMcpIntegrationsClient.delete
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpIntegrationsClient::delete; }
    // AIGatewayMcpIntegrationsClient.get
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpIntegrationsClient::get; }
    // AIGatewayMcpIntegrationsClient.getCapabilities
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpIntegrationsClient::get_capabilities; }
    // AIGatewayMcpIntegrationsClient.getMetadata
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpIntegrationsClient::get_metadata; }
    // AIGatewayMcpIntegrationsClient.getWorkspaces
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpIntegrationsClient::get_workspaces; }
    // AIGatewayMcpIntegrationsClient.list
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpIntegrationsClient::list; }
    // AIGatewayMcpIntegrationsClient.setCapabilities
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpIntegrationsClient::set_capabilities; }
    // AIGatewayMcpIntegrationsClient.setWorkspaces
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpIntegrationsClient::set_workspaces; }
    // AIGatewayMcpIntegrationsClient.update
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpIntegrationsClient::update; }
    // AIGatewayMcpServersClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayMcpServersClient>; let _ = scm_sdk::airs::gateway::AIGatewayMcpServersClient::new; }
    // AIGatewayMcpServersClient.create
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpServersClient::create; }
    // AIGatewayMcpServersClient.delete
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpServersClient::delete; }
    // AIGatewayMcpServersClient.deleteConnections
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpServersClient::delete_connections; }
    // AIGatewayMcpServersClient.get
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpServersClient::get; }
    // AIGatewayMcpServersClient.getCapabilities
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpServersClient::get_capabilities; }
    // AIGatewayMcpServersClient.getConnections
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpServersClient::get_connections; }
    // AIGatewayMcpServersClient.getUserAccess
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpServersClient::get_user_access; }
    // AIGatewayMcpServersClient.list
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpServersClient::list; }
    // AIGatewayMcpServersClient.test
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpServersClient::test; }
    // AIGatewayMcpServersClient.update
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpServersClient::update; }
    // AIGatewayMcpServersClient.updateCapabilities
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpServersClient::update_capabilities; }
    // AIGatewayMcpServersClient.updateUserAccess
    { let _ = scm_sdk::airs::gateway::AIGatewayMcpServersClient::update_user_access; }
    // AIGatewayModelPricingClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayModelPricingClient>; let _ = scm_sdk::airs::gateway::AIGatewayModelPricingClient::new; }
    // AIGatewayModelPricingClient.get
    { let _ = scm_sdk::airs::gateway::AIGatewayModelPricingClient::get; }
    // AIGatewayModelPricingClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayModelPricingClientOptions>; }
    // AIGatewayModelPricingClientOptions.endpoint
    { let _ = scm_sdk::Client::builder; }
    // AIGatewayModelPricingClientOptions.fetch
    { let _ = scm_sdk::ClientBuilder::transport; }
    // AIGatewayModelPricingClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // AIGatewayModelPricingClientOptions.timeoutMs
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayModelPricingClientOptions| { let _ = value.timeout; }; }
    // AIGatewayOrganisationsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayOrganisationsClient>; let _ = scm_sdk::airs::gateway::AIGatewayOrganisationsClient::new; }
    // AIGatewayOrganisationsClient.getAuthSettings
    { let _ = scm_sdk::airs::gateway::AIGatewayOrganisationsClient::get_auth_settings; }
    // AIGatewayOrganisationsClient.getInfo
    { let _ = scm_sdk::airs::gateway::AIGatewayOrganisationsClient::get_info; }
    // AIGatewayOrganisationsClient.getSelf
    { let _ = scm_sdk::airs::gateway::AIGatewayOrganisationsClient::get_self; }
    // AIGatewayOrganisationsClient.updateAuthSettings
    { let _ = scm_sdk::airs::gateway::AIGatewayOrganisationsClient::update_auth_settings; }
    // AIGatewayOrganisationsClient.updateSelf
    { let _ = scm_sdk::airs::gateway::AIGatewayOrganisationsClient::update_self; }
    // AIGatewayPlane
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayPlane>; }
    // AIGatewayPluginsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayPluginsClient>; let _ = scm_sdk::airs::gateway::AIGatewayPluginsClient::new; }
    // AIGatewayPluginsClient.create
    { let _ = scm_sdk::airs::gateway::AIGatewayPluginsClient::create; }
    // AIGatewayPluginsClient.list
    { let _ = scm_sdk::airs::gateway::AIGatewayPluginsClient::list; }
    // AIGatewayProvidersClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayProvidersClient>; let _ = scm_sdk::airs::gateway::AIGatewayProvidersClient::new; }
    // AIGatewayProvidersClient.create
    { let _ = scm_sdk::airs::gateway::AIGatewayProvidersClient::create; }
    // AIGatewayProvidersClient.delete
    { let _ = scm_sdk::airs::gateway::AIGatewayProvidersClient::delete; }
    // AIGatewayProvidersClient.get
    { let _ = scm_sdk::airs::gateway::AIGatewayProvidersClient::get; }
    // AIGatewayProvidersClient.list
    { let _ = scm_sdk::airs::gateway::AIGatewayProvidersClient::list; }
    // AIGatewayProvidersClient.update
    { let _ = scm_sdk::airs::gateway::AIGatewayProvidersClient::update; }
    // AIGatewayRateLimitsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayRateLimitsClient>; let _ = scm_sdk::airs::gateway::AIGatewayRateLimitsClient::new; }
    // AIGatewayRateLimitsClient.create
    { let _ = scm_sdk::airs::gateway::AIGatewayRateLimitsClient::create; }
    // AIGatewayRateLimitsClient.delete
    { let _ = scm_sdk::airs::gateway::AIGatewayRateLimitsClient::delete; }
    // AIGatewayRateLimitsClient.get
    { let _ = scm_sdk::airs::gateway::AIGatewayRateLimitsClient::get; }
    // AIGatewayRateLimitsClient.list
    { let _ = scm_sdk::airs::gateway::AIGatewayRateLimitsClient::list; }
    // AIGatewayRateLimitsClient.update
    { let _ = scm_sdk::airs::gateway::AIGatewayRateLimitsClient::update; }
    // AIGatewayRequestChartOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayRequestChartOptions>; }
    // AIGatewayRequestChartOptions.aiOrgModels
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayRequestChartOptions| { let _ = &value.filters.ai_org_models; }; }
    // AIGatewayRequestChartOptions.apiKeyIds
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayRequestChartOptions| { let _ = &value.filters.api_key_ids; }; }
    // AIGatewayRequestChartOptions.costMax
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayRequestChartOptions| { let _ = &value.filters.cost_max; }; }
    // AIGatewayRequestChartOptions.costMin
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayRequestChartOptions| { let _ = &value.filters.cost_min; }; }
    // AIGatewayRequestChartOptions.days
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayRequestChartOptions| { let _ = &value.window.days; }; }
    // AIGatewayRequestChartOptions.end
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayRequestChartOptions| { let _ = &value.window.end; }; }
    // AIGatewayRequestChartOptions.metadata
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayRequestChartOptions| { let _ = &value.filters.metadata; }; }
    // AIGatewayRequestChartOptions.start
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayRequestChartOptions| { let _ = &value.window.start; }; }
    // AIGatewayRequestChartOptions.statusCodes
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayRequestChartOptions| { let _ = &value.filters.status_codes; }; }
    // AIGatewayRequestChartOptions.totalUnitsMax
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayRequestChartOptions| { let _ = &value.filters.total_units_max; }; }
    // AIGatewayRequestChartOptions.totalUnitsMin
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayRequestChartOptions| { let _ = &value.filters.total_units_min; }; }
    // AIGatewayRequestChartOptions.traceId
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayRequestChartOptions| { let _ = &value.filters.trace_id; }; }
    // AIGatewayRequestChartOptions.workspaceSlug
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayRequestChartOptions| { let _ = &value.window.workspace_slug; }; }
    // AIGatewaySecretOperation
    { let _ = std::marker::PhantomData::<scm_sdk::constants::AIGatewaySecretOperation>; }
    // AIGatewaySecretReferencesClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewaySecretReferencesClient>; let _ = scm_sdk::airs::gateway::AIGatewaySecretReferencesClient::new; }
    // AIGatewaySecretReferencesClient.create
    { let _ = scm_sdk::airs::gateway::AIGatewaySecretReferencesClient::create; }
    // AIGatewaySecretReferencesClient.delete
    { let _ = scm_sdk::airs::gateway::AIGatewaySecretReferencesClient::delete; }
    // AIGatewaySecretReferencesClient.get
    { let _ = scm_sdk::airs::gateway::AIGatewaySecretReferencesClient::get; }
    // AIGatewaySecretReferencesClient.list
    { let _ = scm_sdk::airs::gateway::AIGatewaySecretReferencesClient::list; }
    // AIGatewaySecretReferencesClient.update
    { let _ = scm_sdk::airs::gateway::AIGatewaySecretReferencesClient::update; }
    // AIGatewaySubClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // AIGatewaySubClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // AIGatewaySubClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // AIGatewaySubClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // AIGatewayTelemetryClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayTelemetryClient>; let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::new; }
    // AIGatewayTelemetryClient.byStatusCode
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::by_status_code; }
    // AIGatewayTelemetryClient.byUser
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::by_user; }
    // AIGatewayTelemetryClient.cacheHitTrend
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::cache_hit_trend; }
    // AIGatewayTelemetryClient.cacheSummary
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::cache_summary; }
    // AIGatewayTelemetryClient.cost
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::cost; }
    // AIGatewayTelemetryClient.errorCategoryTrends
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::error_category_trends; }
    // AIGatewayTelemetryClient.errors
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::errors; }
    // AIGatewayTelemetryClient.errorTrends
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::error_trends; }
    // AIGatewayTelemetryClient.feedbackModels
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::feedback_models; }
    // AIGatewayTelemetryClient.feedbackScoreDistribution
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::feedback_score_distribution; }
    // AIGatewayTelemetryClient.feedbackTrend
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::feedback_trend; }
    // AIGatewayTelemetryClient.feedbackWeighted
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::feedback_weighted; }
    // AIGatewayTelemetryClient.filterBoundaries
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::filter_boundaries; }
    // AIGatewayTelemetryClient.groupBy
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::group_by; }
    // AIGatewayTelemetryClient.groupedErrors
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::grouped_errors; }
    // AIGatewayTelemetryClient.latency
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::latency; }
    // AIGatewayTelemetryClient.logs
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::logs; }
    // AIGatewayTelemetryClient.requests
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::requests; }
    // AIGatewayTelemetryClient.rescuedRetries
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::rescued_retries; }
    // AIGatewayTelemetryClient.tokens
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::tokens; }
    // AIGatewayTelemetryClient.users
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::users; }
    // AIGatewayTelemetryClient.userTrends
    { let _ = scm_sdk::airs::gateway::AIGatewayTelemetryClient::user_trends; }
    // AIGatewayTelemetryClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // AIGatewayTelemetryClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // AIGatewayTelemetryClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // AIGatewayTelemetryClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // AIGatewayTelemetryClientOptions.tsgId
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer, "probe", 1, "1"); }; }
    // AIGatewayUsageLimitsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayUsageLimitsClient>; let _ = scm_sdk::airs::gateway::AIGatewayUsageLimitsClient::new; }
    // AIGatewayUsageLimitsClient.create
    { let _ = scm_sdk::airs::gateway::AIGatewayUsageLimitsClient::create; }
    // AIGatewayUsageLimitsClient.delete
    { let _ = scm_sdk::airs::gateway::AIGatewayUsageLimitsClient::delete; }
    // AIGatewayUsageLimitsClient.get
    { let _ = scm_sdk::airs::gateway::AIGatewayUsageLimitsClient::get; }
    // AIGatewayUsageLimitsClient.list
    { let _ = scm_sdk::airs::gateway::AIGatewayUsageLimitsClient::list; }
    // AIGatewayUsageLimitsClient.listEntities
    { let _ = scm_sdk::airs::gateway::AIGatewayUsageLimitsClient::list_entities; }
    // AIGatewayUsageLimitsClient.resetEntity
    { let _ = scm_sdk::airs::gateway::AIGatewayUsageLimitsClient::reset_entity; }
    // AIGatewayUsageLimitsClient.update
    { let _ = scm_sdk::airs::gateway::AIGatewayUsageLimitsClient::update; }
    // AIGatewayWindowOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayWindowOptions>; }
    // AIGatewayWindowOptions.days
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayWindowOptions| { let _ = &value.days; }; }
    // AIGatewayWindowOptions.end
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayWindowOptions| { let _ = &value.end; }; }
    // AIGatewayWindowOptions.start
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayWindowOptions| { let _ = &value.start; }; }
    // AIGatewayWindowOptions.workspaceSlug
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayWindowOptions| { let _ = &value.workspace_slug; }; }
    // AIGatewayWorkspaceGetOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayWorkspaceGetOptions>; }
    // AIGatewayWorkspaceGetOptions.plane
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayWorkspaceGetOptions| { let _ = &value.plane; }; }
    // AIGatewayWorkspaceListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayWorkspaceListOptions>; }
    // AIGatewayWorkspaceListOptions.plane
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayWorkspaceListOptions| { let _ = &value.plane; }; }
    // AIGatewayWorkspaceListOptions.status
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayWorkspaceListOptions| { let _ = &value.status; }; }
    // AIGatewayWorkspaceProvisionOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayWorkspaceProvisionOptions>; }
    // AIGatewayWorkspaceProvisionOptions.existingScope
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayWorkspaceProvisionOptions| { let _ = &value.existing_scope; }; }
    // AIGatewayWorkspacesClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayWorkspacesClient>; let _ = scm_sdk::airs::gateway::AIGatewayWorkspacesClient::new; }
    // AIGatewayWorkspacesClient.create
    { let _ = scm_sdk::airs::gateway::AIGatewayWorkspacesClient::create; }
    // AIGatewayWorkspacesClient.delete
    { let _ = scm_sdk::airs::gateway::AIGatewayWorkspacesClient::delete; }
    // AIGatewayWorkspacesClient.get
    { let _ = scm_sdk::airs::gateway::AIGatewayWorkspacesClient::get; }
    // AIGatewayWorkspacesClient.list
    { let _ = scm_sdk::airs::gateway::AIGatewayWorkspacesClient::list; }
    // AIGatewayWorkspacesClient.provision
    { let _ = scm_sdk::airs::gateway::AIGatewayWorkspacesClient::provision; }
    // AIGatewayWorkspacesClient.update
    { let _ = scm_sdk::airs::gateway::AIGatewayWorkspacesClient::update; }
    // AIGatewayWorkspacesClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // AIGatewayWorkspacesClientOptions.adminBaseUrl
    { let _ = scm_sdk::Client::with_endpoint; }
    // AIGatewayWorkspacesClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // AIGatewayWorkspacesClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // AIGatewayWorkspacesClientOptions.iamScopes
    { let _ = scm_sdk::airs::gateway::AIGatewayWorkspacesClient::with_iam_endpoint; }
    // AIGatewayWorkspacesClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // AIGatewayWorkspaceScopedListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::AIGatewayWorkspaceScopedListOptions>; }
    // AIGatewayWorkspaceScopedListOptions.workspaceId
    { let _ = |value: &scm_sdk::airs::gateway::AIGatewayWorkspaceScopedListOptions| { let _ = &value.workspace_id; }; }
    // aiGwOrganisationsAuthSettingsPath
    { let _ = scm_sdk::airs::gateway::ai_gw_organisations_auth_settings_path; }
    // AiProfile
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::AiProfile>; }
    // AiProfileSchema
    { let _ = scm_sdk::validation::parse("AiProfileSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AIRS_ENDPOINTS
    { let _ = scm_sdk::constants::AIRS_ENDPOINTS; }
    // AISecSDKException
    { let _ = |message: &str| { let _ = scm_sdk::Error::custom(message, None, scm_sdk::ErrorMetadata::new()); }; }
    // AISecSDKException.cause
    { let _ = |error: &scm_sdk::Error| { let _ = std::error::Error::source(error); }; }
    // AISecSDKException.errorType
    { let _ = scm_sdk::Error::error_type; }
    // AISecSDKException.failureKind
    { let _ = scm_sdk::Error::failure_kind; }
    // AISecSDKException.message
    { let _ = scm_sdk::Error::message; }
    // AISecSDKException.name
    { let _ = scm_sdk::Error::name; }
    // AISecSDKException.problem
    { let _ = scm_sdk::Error::problem; }
    // AISecSDKException.retryAfterMs
    { let _ = scm_sdk::Error::retry_after; }
    // AISecSDKException.stack
    { let _ = scm_sdk::Error::capture_backtrace; let _ = scm_sdk::Error::backtrace; }
    // AISecSDKException.statusCode
    { let _ = scm_sdk::Error::status; }
    // AISecSDKException::captureStackTrace
    { let _ = scm_sdk::Error::capture_backtrace; let _ = scm_sdk::Error::backtrace; }
    // AISecSDKException::prepareStackTrace
    { let _ = scm_sdk::Error::capture_backtrace; let _ = scm_sdk::Error::backtrace; }
    // AISecSDKException::stackTraceLimit
    { let _ = scm_sdk::Error::capture_backtrace; let _ = scm_sdk::Error::backtrace; }
    // AISecSDKExceptionMetadata
    { let _ = std::marker::PhantomData::<scm_sdk::ErrorMetadata>; }
    // AISecSDKExceptionMetadata.failureKind
    { let _ = scm_sdk::ErrorMetadata::failure_kind; }
    // AISecSDKExceptionMetadata.problem
    { let _ = scm_sdk::ErrorMetadata::problem; }
    // AISecSDKExceptionMetadata.retryAfterMs
    { let _ = scm_sdk::ErrorMetadata::retry_after; }
    // AISecSDKExceptionMetadata.statusCode
    { let _ = scm_sdk::ErrorMetadata::status; }
    // AISecSDKProblemDetails
    { let _ = std::marker::PhantomData::<scm_sdk::ProblemDetails>; }
    // AISecSDKProblemDetails.detail
    { let _ = scm_sdk::ProblemDetails::detail; }
    // AISecSDKProblemDetails.errors
    { let _ = scm_sdk::ProblemDetails::errors; }
    // AISecSDKProblemDetails.redacted
    { let _ = scm_sdk::ProblemDetails::redacted; }
    // AISecSDKProblemDetails.title
    { let _ = scm_sdk::ProblemDetails::title; }
    // AiSecurityProfile
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::AiSecurityProfile>; }
    // AiSecurityProfileSchema
    { let _ = scm_sdk::validation::parse("AiSecurityProfileSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ApiEndpointType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ApiEndpointType>; }
    // ApiKey
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::ApiKey>; }
    // ApiKeyCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::ApiKeyCreateRequest>; }
    // ApiKeyCreateRequestSchema
    { let _ = scm_sdk::validation::parse("ApiKeyCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ApiKeyDeleteResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::ApiKeyDeleteResponse>; }
    // ApiKeyDeleteResponseSchema
    { let _ = scm_sdk::validation::parse("ApiKeyDeleteResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ApiKeyDPInfo
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::ApiKeyDPInfo>; }
    // ApiKeyDPInfoSchema
    { let _ = scm_sdk::validation::parse("ApiKeyDPInfoSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ApiKeyListAllOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::ApiKeyListAllOptions>; }
    // ApiKeyListAllOptions.limit
    { let _ = |value: &scm_sdk::airs::management::ApiKeyListAllOptions| { let _ = &value.limit; }; }
    // ApiKeyListAllOptions.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // ApiKeyListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::ApiKeyListResponse>; }
    // ApiKeyListResponseSchema
    { let _ = scm_sdk::validation::parse("ApiKeyListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ApiKeyRegenerateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::ApiKeyRegenerateRequest>; }
    // ApiKeyRegenerateRequestSchema
    { let _ = scm_sdk::validation::parse("ApiKeyRegenerateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ApiKeySchema
    { let _ = scm_sdk::validation::parse("ApiKeySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ApiKeysClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::ApiKeysClient>; let _ = scm_sdk::airs::management::ApiKeysClient::new; }
    // ApiKeysClient.create
    { let _ = scm_sdk::airs::management::ApiKeysClient::create; }
    // ApiKeysClient.delete
    { let _ = scm_sdk::airs::management::ApiKeysClient::delete; }
    // ApiKeysClient.list
    { let _ = scm_sdk::airs::management::ApiKeysClient::list; }
    // ApiKeysClient.listAll
    { let _ = scm_sdk::airs::management::ApiKeysClient::list_all; }
    // ApiKeysClient.listForToken
    { let _ = scm_sdk::airs::management::ApiKeysClient::list_for_token; }
    // ApiKeysClient.regenerate
    { let _ = scm_sdk::airs::management::ApiKeysClient::regenerate; }
    // ApiKeysClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // ApiKeysClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // ApiKeysClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // ApiKeysClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // ApiKeysClientOptions.tsgId
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer, "probe", 1, "1"); }; }
    // AppExclusion
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::AppExclusion>; }
    // AppExclusionSchema
    { let _ = scm_sdk::validation::parse("AppExclusionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ASYNC_SCAN_PATH
    { let _ = scm_sdk::constants::ASYNC_SCAN_PATH; }
    // AsyncScanObject
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::AsyncScanObject>; }
    // AsyncScanObjectSchema
    { let _ = scm_sdk::validation::parse("AsyncScanObjectSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AsyncScanResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::AsyncScanResponse>; }
    // AsyncScanResponseSchema
    { let _ = scm_sdk::validation::parse("AsyncScanResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AttackDetailResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AttackDetailResponse>; }
    // AttackDetailResponseSchema
    { let _ = scm_sdk::validation::parse("AttackDetailResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AttackListItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AttackListItem>; }
    // AttackListItemSchema
    { let _ = scm_sdk::validation::parse("AttackListItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AttackListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::AttackListOptions>; }
    // AttackListOptions.attack_modality
    { let _ = |value: &scm_sdk::airs::red_team::AttackListOptions| { let _ = &value.attack_modality; }; }
    // AttackListOptions.attack_status
    { let _ = |value: &scm_sdk::airs::red_team::AttackListOptions| { let _ = &value.attack_status; }; }
    // AttackListOptions.attack_type
    { let _ = |value: &scm_sdk::airs::red_team::AttackListOptions| { let _ = &value.attack_type; }; }
    // AttackListOptions.category
    { let _ = |value: &scm_sdk::airs::red_team::AttackListOptions| { let _ = &value.category; }; }
    // AttackListOptions.compliance
    { let _ = |value: &scm_sdk::airs::red_team::AttackListOptions| { let _ = &value.compliance; }; }
    // AttackListOptions.limit
    { let _ = |value: &scm_sdk::airs::red_team::AttackListOptions| { let _ = &value.limit; }; }
    // AttackListOptions.search
    { let _ = |value: &scm_sdk::airs::red_team::AttackListOptions| { let _ = &value.search; }; }
    // AttackListOptions.severity
    { let _ = |value: &scm_sdk::airs::red_team::AttackListOptions| { let _ = &value.severity; }; }
    // AttackListOptions.skip
    { let _ = |value: &scm_sdk::airs::red_team::AttackListOptions| { let _ = &value.skip; }; }
    // AttackListOptions.status
    { let _ = |value: &scm_sdk::airs::red_team::AttackListOptions| { let _ = &value.status; }; }
    // AttackListOptions.sub_category
    { let _ = |value: &scm_sdk::airs::red_team::AttackListOptions| { let _ = &value.sub_category; }; }
    // AttackListOptions.threat
    { let _ = |value: &scm_sdk::airs::red_team::AttackListOptions| { let _ = &value.threat; }; }
    // AttackListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AttackListResponse>; }
    // AttackListResponseSchema
    { let _ = scm_sdk::validation::parse("AttackListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AttackMultiTurnDetailResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AttackMultiTurnDetailResponse>; }
    // AttackMultiTurnDetailResponseSchema
    { let _ = scm_sdk::validation::parse("AttackMultiTurnDetailResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AttackMultiTurnOutput
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AttackMultiTurnOutput>; }
    // AttackMultiTurnOutputSchema
    { let _ = scm_sdk::validation::parse("AttackMultiTurnOutputSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AttackOutput
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AttackOutput>; }
    // AttackOutputSchema
    { let _ = scm_sdk::validation::parse("AttackOutputSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AttackStatus
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AttackStatus>; }
    // AttackType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AttackType>; }
    // AuditResponse
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::AuditResponse>; }
    // AuditResponseSchema
    { let _ = scm_sdk::validation::parse("AuditResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AuthConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AuthConfig>; }
    // AuthConfigRequestSchema
    { let _ = scm_sdk::validation::parse("AuthConfigRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AuthConfigSchema
    { let _ = scm_sdk::validation::parse("AuthConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AuthSettingsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::AuthSettingsResponse>; }
    // AuthSettingsResponseSchema
    { let _ = scm_sdk::validation::parse("AuthSettingsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // AuthType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::AuthType>; }
    // BaseResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::BaseResponse>; }
    // BaseResponseSchema
    { let _ = scm_sdk::validation::parse("BaseResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // BasicAuthAuthConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::BasicAuthAuthConfig>; }
    // BasicAuthAuthConfigSchema
    { let _ = scm_sdk::validation::parse("BasicAuthAuthConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // BasicAuthLocation
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::BasicAuthLocation>; }
    // BatchAssignCustomRuleRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::BatchAssignCustomRuleRequest>; }
    // BatchAssignCustomRuleRequestSchema
    { let _ = scm_sdk::validation::parse("BatchAssignCustomRuleRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // BatchAssignCustomRuleResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::BatchAssignCustomRuleResponse>; }
    // BatchAssignCustomRuleResponseSchema
    { let _ = scm_sdk::validation::parse("BatchAssignCustomRuleResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // BEARER
    { let _ = scm_sdk::constants::BEARER; }
    // BedrockAccessConnectionParams
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::BedrockAccessConnectionParams>; }
    // BedrockAccessConnectionParamsSchema
    { let _ = scm_sdk::validation::parse("BedrockAccessConnectionParamsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // BrandSubCategory
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::BrandSubCategory>; }
    // buildDottedObject
    { let _ = scm_sdk::airs::gateway::build_dotted_object; }
    // CacheHitTrendResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::CacheHitTrendResponse>; }
    // CacheHitTrendResponseSchema
    { let _ = scm_sdk::validation::parse("CacheHitTrendResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CacheSummaryResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::CacheSummaryResponse>; }
    // CacheSummaryResponseSchema
    { let _ = scm_sdk::validation::parse("CacheSummaryResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // Category
    { let _ = std::marker::PhantomData::<scm_sdk::shared::models::Category>; }
    // CategoryModel
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CategoryModel>; }
    // CategoryModelSchema
    { let _ = scm_sdk::validation::parse("CategoryModelSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CategoryReport
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CategoryReport>; }
    // CategoryReportSchema
    { let _ = scm_sdk::validation::parse("CategoryReportSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CategoryType
    { let _ = std::marker::PhantomData::<scm_sdk::shared::models::CategoryType>; }
    // CgReport
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::CgReport>; }
    // CgReportSchema
    { let _ = scm_sdk::validation::parse("CgReportSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // Channel
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::Channel>; }
    // ChannelListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::ChannelListOptions>; }
    // ChannelListOptions.include_all_if_empty
    { let _ = |value: &scm_sdk::airs::red_team::ChannelListOptions| { let _ = &value.include_all_if_empty; }; }
    // ChannelListOptions.limit
    { let _ = |value: &scm_sdk::airs::red_team::ChannelListOptions| { let _ = &value.limit; }; }
    // ChannelListOptions.search
    { let _ = |value: &scm_sdk::airs::red_team::ChannelListOptions| { let _ = &value.search; }; }
    // ChannelListOptions.skip
    { let _ = |value: &scm_sdk::airs::red_team::ChannelListOptions| { let _ = &value.skip; }; }
    // ChannelListOptions.status
    { let _ = |value: &scm_sdk::airs::red_team::ChannelListOptions| { let _ = &value.status; }; }
    // ChannelListPagination
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ChannelListPagination>; }
    // ChannelListPaginationSchema
    { let _ = scm_sdk::validation::parse("ChannelListPaginationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ChannelListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ChannelListResponse>; }
    // ChannelListResponseSchema
    { let _ = scm_sdk::validation::parse("ChannelListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ChannelSchema
    { let _ = scm_sdk::validation::parse("ChannelSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ChannelStats
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ChannelStats>; }
    // ChannelStatsSchema
    { let _ = scm_sdk::validation::parse("ChannelStatsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ChannelStatus
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ChannelStatus>; }
    // ChannelStatusSchema
    { let _ = scm_sdk::validation::parse("ChannelStatusSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ChannelStatusType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ChannelStatusType>; }
    // ClaraJobMetadata
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ClaraJobMetadata>; }
    // ClaraJobMetadataSchema
    { let _ = scm_sdk::validation::parse("ClaraJobMetadataSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ClientIdAndCustomerApp
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::ClientIdAndCustomerApp>; }
    // ClientIdAndCustomerAppSchema
    { let _ = scm_sdk::validation::parse("ClientIdAndCustomerAppSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CmdEntry
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::CmdEntry>; }
    // CmdEntrySchema
    { let _ = scm_sdk::validation::parse("CmdEntrySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CmdInjectReport
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::CmdInjectReport>; }
    // CmdInjectReportSchema
    { let _ = scm_sdk::validation::parse("CmdInjectReportSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // collectAll
    { let options = scm_sdk::RequestOptions::with_timeout(std::time::Duration::from_secs(1)).unwrap(); let mut listing = scm_sdk::helpers::StreamListing::new(futures_util::stream::empty::<scm_sdk::Result<String>>()); let _ = scm_sdk::helpers::collect_all(&mut listing, scm_sdk::pagination::CollectOptions::default(), &scm_sdk::Session::new(), &options); }
    // CollectAllOptions
    { let _ = std::marker::PhantomData::<scm_sdk::pagination::CollectOptions>; }
    // CollectAllOptions.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // collectSkipPages
    { let options = scm_sdk::RequestOptions::with_timeout(std::time::Duration::from_secs(1)).unwrap(); let _ = scm_sdk::helpers::collect_skip_pages(|_,_,_| async { Ok(scm_sdk::helpers::SkipPage::<String> {items:vec![],total:None}) }, scm_sdk::helpers::WalkAllOptions::default(), scm_sdk::Session::new(), options); }
    // collectSpringPages
    { let options = scm_sdk::RequestOptions::with_timeout(std::time::Duration::from_secs(1)).unwrap(); let _ = scm_sdk::helpers::collect_spring_pages(|_,_,_| async { Ok(scm_sdk::helpers::SpringPage::<String> {items:vec![],last:true}) }, scm_sdk::helpers::SpringWalkOptions::default(), scm_sdk::Session::new(), options); }
    // ComparisonOperatorType
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::ComparisonOperatorType>; }
    // ComparisonOperatorTypeSchema
    { let _ = scm_sdk::validation::parse("ComparisonOperatorTypeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ComplianceFramework
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ComplianceFramework>; }
    // ComplianceFrameworkSchema
    { let _ = scm_sdk::validation::parse("ComplianceFrameworkSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ComplianceReport
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ComplianceReport>; }
    // ComplianceReportSchema
    { let _ = scm_sdk::validation::parse("ComplianceReportSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ComplianceSubCategory
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ComplianceSubCategory>; }
    // ComplianceTechnique
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ComplianceTechnique>; }
    // ComplianceTechniqueSchema
    { let _ = scm_sdk::validation::parse("ComplianceTechniqueSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ConditionGroup
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ConditionGroup>; }
    // ConditionGroup.conditions
    { let _ = |value: &scm_sdk::airs::model_security::models::ConditionGroup| { let _ = &value.conditions; }; }
    // ConditionGroup.operator
    { let _ = |value: &scm_sdk::airs::model_security::models::ConditionGroup| { let _ = &value.operator; }; }
    // ConditionGroupSchema
    { let _ = scm_sdk::validation::parse("ConditionGroupSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ConnectionParams
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ConnectionParams>; }
    // ConnectionParamsRequestSchema
    { let _ = scm_sdk::validation::parse("ConnectionParamsRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ConnectionParamsSchema
    { let _ = scm_sdk::validation::parse("ConnectionParamsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // Content
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::Content>; let _ = scm_sdk::airs::scan::Content::builder; }
    // Content.codePrompt
    { let _ = scm_sdk::airs::scan::Content::code_prompt; }
    // Content.codeResponse
    { let _ = scm_sdk::airs::scan::Content::code_response; }
    // Content.context
    { let _ = scm_sdk::airs::scan::Content::context; }
    // Content.length
    { let _ = scm_sdk::airs::scan::Content::length; }
    // Content.prompt
    { let _ = scm_sdk::airs::scan::Content::prompt; }
    // Content.response
    { let _ = scm_sdk::airs::scan::Content::response; }
    // Content.toJSON
    { let _ = scm_sdk::airs::scan::Content::to_json; }
    // Content.toolEvent
    { let _ = scm_sdk::airs::scan::Content::tool_event; }
    // Content::fromJSON
    { let _ = scm_sdk::airs::scan::Content::from_json; }
    // Content::fromJSONFile
    { let _ = || { let _future = scm_sdk::airs::scan::Content::from_json_file("probe.json", 1024); }; }
    // ContentError
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::ContentError>; }
    // ContentErrorSchema
    { let _ = scm_sdk::validation::parse("ContentErrorSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ContentErrorType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::ContentErrorType>; }
    // ContentErrorTypeType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::ContentErrorTypeType>; }
    // ContentOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::ContentBuilder>; let _ = scm_sdk::airs::scan::Content::builder; }
    // ContentOptions.codePrompt
    { let _ = |builder: scm_sdk::airs::scan::ContentBuilder| { let _ = builder.code_prompt(String::new()); }; }
    // ContentOptions.codeResponse
    { let _ = |builder: scm_sdk::airs::scan::ContentBuilder| { let _ = builder.code_response(String::new()); }; }
    // ContentOptions.context
    { let _ = |builder: scm_sdk::airs::scan::ContentBuilder| { let _ = builder.context(String::new()); }; }
    // ContentOptions.prompt
    { let _ = |builder: scm_sdk::airs::scan::ContentBuilder| { let _ = builder.prompt(String::new()); }; }
    // ContentOptions.response
    { let _ = |builder: scm_sdk::airs::scan::ContentBuilder| { let _ = builder.response(String::new()); }; }
    // ContentOptions.toolEvent
    { let _ = |builder: scm_sdk::airs::scan::ContentBuilder, event: scm_sdk::airs::scan::models::ToolEvent| { let _ = builder.tool_event(event); }; }
    // CostChartResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::CostChartResponse>; }
    // CostChartResponseSchema
    { let _ = scm_sdk::validation::parse("CostChartResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CountByName
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CountByName>; }
    // CountByNameSchema
    { let _ = scm_sdk::validation::parse("CountByNameSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CountChartResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::CountChartResponse>; }
    // CountChartResponseSchema
    { let _ = scm_sdk::validation::parse("CountChartResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CountedQuotaEnum
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CountedQuotaEnum>; }
    // CreateChannelRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CreateChannelRequest>; }
    // CreateChannelRequestSchema
    { let _ = scm_sdk::validation::parse("CreateChannelRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CreateCustomTopicRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::CreateCustomTopicRequest>; }
    // CreateCustomTopicRequestSchema
    { let _ = scm_sdk::validation::parse("CreateCustomTopicRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CreateSecurityProfileRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::CreateSecurityProfileRequest>; }
    // CreateSecurityProfileRequest.active
    { let _ = |value: &scm_sdk::airs::management::models::CreateSecurityProfileRequest| { let _ = &value.active; }; }
    // CreateSecurityProfileRequest.created_by
    { let _ = |value: &scm_sdk::airs::management::models::CreateSecurityProfileRequest| { let _ = &value.created_by; }; }
    // CreateSecurityProfileRequest.csp_id
    { let _ = |value: &scm_sdk::airs::management::models::CreateSecurityProfileRequest| { let _ = &value.csp_id; }; }
    // CreateSecurityProfileRequest.last_modified_ts
    { let _ = |value: &scm_sdk::airs::management::models::CreateSecurityProfileRequest| { let _ = &value.last_modified_ts; }; }
    // CreateSecurityProfileRequest.policy
    { let _ = |value: &scm_sdk::airs::management::models::CreateSecurityProfileRequest| { let _ = &value.policy; }; }
    // CreateSecurityProfileRequest.profile_id
    { let _ = |value: &scm_sdk::airs::management::models::CreateSecurityProfileRequest| { let _ = &value.profile_id; }; }
    // CreateSecurityProfileRequest.profile_name
    { let _ = |value: &scm_sdk::airs::management::models::CreateSecurityProfileRequest| { let _ = &value.profile_name; }; }
    // CreateSecurityProfileRequest.revision
    { let _ = |value: &scm_sdk::airs::management::models::CreateSecurityProfileRequest| { let _ = &value.revision; }; }
    // CreateSecurityProfileRequest.tsg_id
    { let _ = |value: &scm_sdk::airs::management::models::CreateSecurityProfileRequest| { let _ = &value.tsg_id; }; }
    // CreateSecurityProfileRequest.updated_by
    { let _ = |value: &scm_sdk::airs::management::models::CreateSecurityProfileRequest| { let _ = &value.updated_by; }; }
    // CreateSecurityProfileRequestSchema
    { let _ = scm_sdk::validation::parse("CreateSecurityProfileRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CursorPagination
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::CursorPagination>; }
    // CursorPaginationSchema
    { let _ = scm_sdk::validation::parse("CursorPaginationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomAttackOutput
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CustomAttackOutput>; }
    // CustomAttackOutputSchema
    { let _ = scm_sdk::validation::parse("CustomAttackOutputSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomAttackReportResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CustomAttackReportResponse>; }
    // CustomAttackReportResponseSchema
    { let _ = scm_sdk::validation::parse("CustomAttackReportResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomAttacksListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CustomAttacksListResponse>; }
    // CustomAttacksListResponseSchema
    { let _ = scm_sdk::validation::parse("CustomAttacksListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomAttacksReportListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::CustomAttacksReportListOptions>; }
    // CustomAttacksReportListOptions.limit
    { let _ = |value: &scm_sdk::airs::red_team::CustomAttacksReportListOptions| { let _ = &value.limit; }; }
    // CustomAttacksReportListOptions.prompt_set_id
    { let _ = |value: &scm_sdk::airs::red_team::CustomAttacksReportListOptions| { let _ = &value.prompt_set_id; }; }
    // CustomAttacksReportListOptions.property_value
    { let _ = |value: &scm_sdk::airs::red_team::CustomAttacksReportListOptions| { let _ = &value.property_value; }; }
    // CustomAttacksReportListOptions.search
    { let _ = |value: &scm_sdk::airs::red_team::CustomAttacksReportListOptions| { let _ = &value.search; }; }
    // CustomAttacksReportListOptions.skip
    { let _ = |value: &scm_sdk::airs::red_team::CustomAttacksReportListOptions| { let _ = &value.skip; }; }
    // CustomAttacksReportListOptions.status
    { let _ = |value: &scm_sdk::airs::red_team::CustomAttacksReportListOptions| { let _ = &value.status; }; }
    // CustomAttacksReportListOptions.threat
    { let _ = |value: &scm_sdk::airs::red_team::CustomAttacksReportListOptions| { let _ = &value.threat; }; }
    // CustomerApp
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::CustomerApp>; }
    // CustomerAppDeleteResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::CustomerAppDeleteResponse>; }
    // CustomerAppDeleteResponseSchema
    { let _ = scm_sdk::validation::parse("CustomerAppDeleteResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomerAppListAllOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::CustomerAppListAllOptions>; }
    // CustomerAppListAllOptions.limit
    { let _ = |value: &scm_sdk::airs::management::CustomerAppListAllOptions| { let _ = &value.limit; }; }
    // CustomerAppListAllOptions.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // CustomerAppListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::CustomerAppListResponse>; }
    // CustomerAppListResponseSchema
    { let _ = scm_sdk::validation::parse("CustomerAppListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomerAppSchema
    { let _ = scm_sdk::validation::parse("CustomerAppSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomerAppsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::CustomerAppsClient>; let _ = scm_sdk::airs::management::CustomerAppsClient::new; }
    // CustomerAppsClient.delete
    { let _ = scm_sdk::airs::management::CustomerAppsClient::delete; }
    // CustomerAppsClient.get
    { let _ = scm_sdk::airs::management::CustomerAppsClient::get; }
    // CustomerAppsClient.list
    { let _ = scm_sdk::airs::management::CustomerAppsClient::list; }
    // CustomerAppsClient.listAll
    { let _ = scm_sdk::airs::management::CustomerAppsClient::list_all; }
    // CustomerAppsClient.listForToken
    { let _ = scm_sdk::airs::management::CustomerAppsClient::list_for_token; }
    // CustomerAppsClient.update
    { let _ = scm_sdk::airs::management::CustomerAppsClient::update; }
    // CustomerAppsClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // CustomerAppsClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // CustomerAppsClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // CustomerAppsClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // CustomerAppsClientOptions.tsgId
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer, "probe", 1, "1"); }; }
    // CustomerAppWithKeys
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::CustomerAppWithKeys>; }
    // CustomerAppWithKeysSchema
    { let _ = scm_sdk::validation::parse("CustomerAppWithKeysSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // customHostConfiguration
    { let _ = scm_sdk::airs::gateway::custom_host_configuration; }
    // CustomHostConfigurationOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::CustomHostConfigurationOptions>; }
    // CustomHostConfigurationOptions.headers
    { let _ = |value: &scm_sdk::airs::gateway::CustomHostConfigurationOptions| { let _ = &value.headers; }; }
    // CustomHostConfigurationOptions.host
    { let _ = |value: &scm_sdk::airs::gateway::CustomHostConfigurationOptions| { let _ = &value.host; }; }
    // CustomJobMetadata
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CustomJobMetadata>; }
    // CustomJobMetadataSchema
    { let _ = scm_sdk::validation::parse("CustomJobMetadataSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomPromptCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CustomPromptCreateRequest>; }
    // CustomPromptCreateRequestSchema
    { let _ = scm_sdk::validation::parse("CustomPromptCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomPromptList
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CustomPromptList>; }
    // CustomPromptListItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CustomPromptListItem>; }
    // CustomPromptListItemSchema
    { let _ = scm_sdk::validation::parse("CustomPromptListItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomPromptListSchema
    { let _ = scm_sdk::validation::parse("CustomPromptListSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomPromptResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CustomPromptResponse>; }
    // CustomPromptResponseSchema
    { let _ = scm_sdk::validation::parse("CustomPromptResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomPromptSetArchiveRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CustomPromptSetArchiveRequest>; }
    // CustomPromptSetArchiveRequestSchema
    { let _ = scm_sdk::validation::parse("CustomPromptSetArchiveRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomPromptSetCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CustomPromptSetCreateRequest>; }
    // CustomPromptSetCreateRequestSchema
    { let _ = scm_sdk::validation::parse("CustomPromptSetCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomPromptSetList
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CustomPromptSetList>; }
    // CustomPromptSetListActive
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CustomPromptSetListActive>; }
    // CustomPromptSetListActiveSchema
    { let _ = scm_sdk::validation::parse("CustomPromptSetListActiveSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomPromptSetListItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CustomPromptSetListItem>; }
    // CustomPromptSetListItemSchema
    { let _ = scm_sdk::validation::parse("CustomPromptSetListItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomPromptSetListSchema
    { let _ = scm_sdk::validation::parse("CustomPromptSetListSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomPromptSetReference
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CustomPromptSetReference>; }
    // CustomPromptSetReferenceSchema
    { let _ = scm_sdk::validation::parse("CustomPromptSetReferenceSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomPromptSetResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CustomPromptSetResponse>; }
    // CustomPromptSetResponseSchema
    { let _ = scm_sdk::validation::parse("CustomPromptSetResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomPromptSetUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CustomPromptSetUpdateRequest>; }
    // CustomPromptSetUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("CustomPromptSetUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomPromptSetVersionInfo
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CustomPromptSetVersionInfo>; }
    // CustomPromptSetVersionInfoSchema
    { let _ = scm_sdk::validation::parse("CustomPromptSetVersionInfoSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomPromptUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::CustomPromptUpdateRequest>; }
    // CustomPromptUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("CustomPromptUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomRuleAssignment
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::CustomRuleAssignment>; }
    // CustomRuleAssignmentResult
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::CustomRuleAssignmentResult>; }
    // CustomRuleAssignmentResultSchema
    { let _ = scm_sdk::validation::parse("CustomRuleAssignmentResultSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomRuleAssignmentSchema
    { let _ = scm_sdk::validation::parse("CustomRuleAssignmentSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomRuleCondition
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::CustomRuleCondition>; }
    // CustomRuleConditionSchema
    { let _ = scm_sdk::validation::parse("CustomRuleConditionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomRuleCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::CustomRuleCreateRequest>; }
    // CustomRuleCreateRequestSchema
    { let _ = scm_sdk::validation::parse("CustomRuleCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomRuleListItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::CustomRuleListItem>; }
    // CustomRuleListItemSchema
    { let _ = scm_sdk::validation::parse("CustomRuleListItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomRuleListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::CustomRuleListOptions>; }
    // CustomRuleListOptions.generation
    { let _ = |value: &scm_sdk::airs::model_security::CustomRuleListOptions| { let _ = &value.generation; }; }
    // CustomRuleListOptions.is_archived
    { let _ = |value: &scm_sdk::airs::model_security::CustomRuleListOptions| { let _ = &value.is_archived; }; }
    // CustomRuleListOptions.limit
    { let _ = |value: &scm_sdk::airs::model_security::CustomRuleListOptions| { let _ = &value.limit; }; }
    // CustomRuleListOptions.search_query
    { let _ = |value: &scm_sdk::airs::model_security::CustomRuleListOptions| { let _ = &value.search_query; }; }
    // CustomRuleListOptions.skip
    { let _ = |value: &scm_sdk::airs::model_security::CustomRuleListOptions| { let _ = &value.skip; }; }
    // CustomRuleListOptions.sort_dir
    { let _ = |value: &scm_sdk::airs::model_security::CustomRuleListOptions| { let _ = &value.sort_dir; }; }
    // CustomRuleListOptions.sort_field
    { let _ = |value: &scm_sdk::airs::model_security::CustomRuleListOptions| { let _ = &value.sort_field; }; }
    // CustomRuleListOptions.source_types
    { let _ = |value: &scm_sdk::airs::model_security::CustomRuleListOptions| { let _ = &value.source_types; }; }
    // CustomRuleResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::CustomRuleResponse>; }
    // CustomRuleResponseSchema
    { let _ = scm_sdk::validation::parse("CustomRuleResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomRuleSecurityGroup
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::CustomRuleSecurityGroup>; }
    // CustomRuleSecurityGroupSchema
    { let _ = scm_sdk::validation::parse("CustomRuleSecurityGroupSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomRuleSourceTypeSchema
    { let _ = scm_sdk::validation::parse("CustomRuleSourceTypeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomRuleStateSchema
    { let _ = scm_sdk::validation::parse("CustomRuleStateSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomRuleUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::CustomRuleUpdateRequest>; }
    // CustomRuleUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("CustomRuleUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomTopic
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::CustomTopic>; }
    // CustomTopicListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::CustomTopicListResponse>; }
    // CustomTopicListResponseSchema
    { let _ = scm_sdk::validation::parse("CustomTopicListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // CustomTopicSchema
    { let _ = scm_sdk::validation::parse("CustomTopicSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardApplication
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardApplication>; }
    // DashboardApplicationSchema
    { let _ = scm_sdk::validation::parse("DashboardApplicationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardApplicationSessionsBucket
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardApplicationSessionsBucket>; }
    // DashboardApplicationSessionsBucketSchema
    { let _ = scm_sdk::validation::parse("DashboardApplicationSessionsBucketSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardApplicationsOverview
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardApplicationsOverview>; }
    // DashboardApplicationsOverviewItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardApplicationsOverviewItem>; }
    // DashboardApplicationsOverviewItemSchema
    { let _ = scm_sdk::validation::parse("DashboardApplicationsOverviewItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardApplicationsOverviewQuery
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::DashboardApplicationsOverviewQuery>; }
    // DashboardApplicationsOverviewQuery.limit
    { let _ = |value: &scm_sdk::airs::management::DashboardApplicationsOverviewQuery| { let _ = &value.limit; }; }
    // DashboardApplicationsOverviewQuery.offset
    { let _ = |value: &scm_sdk::airs::management::DashboardApplicationsOverviewQuery| { let _ = &value.offset; }; }
    // DashboardApplicationsOverviewQuery.timeInterval
    { let _ = |value: &scm_sdk::airs::management::DashboardApplicationsOverviewQuery| { let _ = &value.time_interval; }; }
    // DashboardApplicationsOverviewQuery.timeUnit
    { let _ = |value: &scm_sdk::airs::management::DashboardApplicationsOverviewQuery| { let _ = &value.time_unit; }; }
    // DashboardApplicationsOverviewSchema
    { let _ = scm_sdk::validation::parse("DashboardApplicationsOverviewSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardApplicationsViolationsTrend
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardApplicationsViolationsTrend>; }
    // DashboardApplicationsViolationsTrendSchema
    { let _ = scm_sdk::validation::parse("DashboardApplicationsViolationsTrendSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardApplicationViolationBreakdown
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardApplicationViolationBreakdown>; }
    // DashboardApplicationViolationBreakdownSchema
    { let _ = scm_sdk::validation::parse("DashboardApplicationViolationBreakdownSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardAppListItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardAppListItem>; }
    // DashboardAppListItemSchema
    { let _ = scm_sdk::validation::parse("DashboardAppListItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardAppQuery
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::DashboardAppQuery>; }
    // DashboardAppQuery.appId
    { let _ = |value: &scm_sdk::airs::management::DashboardAppQuery| { let _ = &value.app_id; }; }
    // DashboardAppQuery.appName
    { let _ = |value: &scm_sdk::airs::management::DashboardAppQuery| { let _ = &value.app_name; }; }
    // DashboardAppQuery.timeInterval
    { let _ = |value: &scm_sdk::airs::management::DashboardAppQuery| { let _ = &value.time_interval; }; }
    // DashboardAppQuery.timeUnit
    { let _ = |value: &scm_sdk::airs::management::DashboardAppQuery| { let _ = &value.time_unit; }; }
    // DashboardAppsList
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardAppsList>; }
    // DashboardAppsListSchema
    { let _ = scm_sdk::validation::parse("DashboardAppsListSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::DashboardClient>; let _ = scm_sdk::airs::management::DashboardClient::new; }
    // DashboardClient.application
    { let _ = scm_sdk::airs::management::DashboardClient::application; }
    // DashboardClient.applicationRaw
    { let _ = scm_sdk::airs::management::DashboardClient::application_raw; }
    // DashboardClient.applicationsOverview
    { let _ = scm_sdk::airs::management::DashboardClient::applications_overview; }
    // DashboardClient.applicationsOverviewRaw
    { let _ = scm_sdk::airs::management::DashboardClient::applications_overview_raw; }
    // DashboardClient.applicationsViolationsTrend
    { let _ = scm_sdk::airs::management::DashboardClient::applications_violations_trend; }
    // DashboardClient.applicationsViolationsTrendRaw
    { let _ = scm_sdk::airs::management::DashboardClient::applications_violations_trend_raw; }
    // DashboardClient.applicationViolationBreakdown
    { let _ = scm_sdk::airs::management::DashboardClient::application_violation_breakdown; }
    // DashboardClient.applicationViolationBreakdownRaw
    { let _ = scm_sdk::airs::management::DashboardClient::application_violation_breakdown_raw; }
    // DashboardClient.appsList
    { let _ = scm_sdk::airs::management::DashboardClient::apps_list; }
    // DashboardClient.appsListRaw
    { let _ = scm_sdk::airs::management::DashboardClient::apps_list_raw; }
    // DashboardClient.scanContent
    { let _ = scm_sdk::airs::management::DashboardClient::scan_content; }
    // DashboardClient.scanContentRaw
    { let _ = scm_sdk::airs::management::DashboardClient::scan_content_raw; }
    // DashboardClient.session
    { let _ = scm_sdk::airs::management::DashboardClient::session; }
    // DashboardClient.sessionRaw
    { let _ = scm_sdk::airs::management::DashboardClient::session_raw; }
    // DashboardClient.sessionsChart
    { let _ = scm_sdk::airs::management::DashboardClient::sessions_chart; }
    // DashboardClient.sessionsChartRaw
    { let _ = scm_sdk::airs::management::DashboardClient::sessions_chart_raw; }
    // DashboardClient.sessionsOverview
    { let _ = scm_sdk::airs::management::DashboardClient::sessions_overview; }
    // DashboardClient.sessionsOverviewRaw
    { let _ = scm_sdk::airs::management::DashboardClient::sessions_overview_raw; }
    // DashboardClient.sessionTransaction
    { let _ = scm_sdk::airs::management::DashboardClient::session_transaction; }
    // DashboardClient.sessionTransactionRaw
    { let _ = scm_sdk::airs::management::DashboardClient::session_transaction_raw; }
    // DashboardClient.topApplicationsViolations
    { let _ = scm_sdk::airs::management::DashboardClient::top_applications_violations; }
    // DashboardClient.topApplicationsViolationsRaw
    { let _ = scm_sdk::airs::management::DashboardClient::top_applications_violations_raw; }
    // DashboardClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // DashboardClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // DashboardClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // DashboardClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // DashboardClientOptions.tsgId
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer, "probe", 1, "1"); }; }
    // DashboardOverviewResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::DashboardOverviewResponse>; }
    // DashboardOverviewResponseSchema
    { let _ = scm_sdk::validation::parse("DashboardOverviewResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardPagination
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardPagination>; }
    // DashboardPaginationSchema
    { let _ = scm_sdk::validation::parse("DashboardPaginationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardPolicyViolation
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardPolicyViolation>; }
    // DashboardPolicyViolationSchema
    { let _ = scm_sdk::validation::parse("DashboardPolicyViolationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardScanContent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardScanContent>; }
    // DashboardScanContentQuery
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::DashboardScanContentQuery>; }
    // DashboardScanContentQuery.scanId
    { let _ = |value: &scm_sdk::airs::management::DashboardScanContentQuery| { let _ = &value.scan_id; }; }
    // DashboardScanContentQuery.scanSubReqId
    { let _ = |value: &scm_sdk::airs::management::DashboardScanContentQuery| { let _ = &value.scan_sub_req_id; }; }
    // DashboardScanContents
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardScanContents>; }
    // DashboardScanContentSchema
    { let _ = scm_sdk::validation::parse("DashboardScanContentSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardScanContentsSchema
    { let _ = scm_sdk::validation::parse("DashboardScanContentsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardSession
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardSession>; }
    // DashboardSessionAction
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardSessionAction>; }
    // DashboardSessionActionSchema
    { let _ = scm_sdk::validation::parse("DashboardSessionActionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardSessionOverviewItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardSessionOverviewItem>; }
    // DashboardSessionOverviewItemSchema
    { let _ = scm_sdk::validation::parse("DashboardSessionOverviewItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardSessionQuery
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::DashboardSessionQuery>; }
    // DashboardSessionQuery.appId
    { let _ = |value: &scm_sdk::airs::management::DashboardSessionQuery| { let _ = &value.app_id; }; }
    // DashboardSessionQuery.appName
    { let _ = |value: &scm_sdk::airs::management::DashboardSessionQuery| { let _ = &value.app_name; }; }
    // DashboardSessionQuery.limit
    { let _ = |value: &scm_sdk::airs::management::DashboardSessionQuery| { let _ = &value.limit; }; }
    // DashboardSessionQuery.offset
    { let _ = |value: &scm_sdk::airs::management::DashboardSessionQuery| { let _ = &value.offset; }; }
    // DashboardSessionQuery.sessionId
    { let _ = |value: &scm_sdk::airs::management::DashboardSessionQuery| { let _ = &value.session_id; }; }
    // DashboardSessionQuery.timeInterval
    { let _ = |value: &scm_sdk::airs::management::DashboardSessionQuery| { let _ = &value.time_interval; }; }
    // DashboardSessionQuery.timeUnit
    { let _ = |value: &scm_sdk::airs::management::DashboardSessionQuery| { let _ = &value.time_unit; }; }
    // DashboardSessionsChart
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardSessionsChart>; }
    // DashboardSessionsChartBucket
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardSessionsChartBucket>; }
    // DashboardSessionsChartBucketSchema
    { let _ = scm_sdk::validation::parse("DashboardSessionsChartBucketSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardSessionsChartSchema
    { let _ = scm_sdk::validation::parse("DashboardSessionsChartSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardSessionSchema
    { let _ = scm_sdk::validation::parse("DashboardSessionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardSessionsOverview
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardSessionsOverview>; }
    // DashboardSessionsOverviewQuery
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::DashboardSessionsOverviewQuery>; }
    // DashboardSessionsOverviewQuery.limit
    { let _ = |value: &scm_sdk::airs::management::DashboardSessionsOverviewQuery| { let _ = &value.limit; }; }
    // DashboardSessionsOverviewQuery.offset
    { let _ = |value: &scm_sdk::airs::management::DashboardSessionsOverviewQuery| { let _ = &value.offset; }; }
    // DashboardSessionsOverviewQuery.timeInterval
    { let _ = |value: &scm_sdk::airs::management::DashboardSessionsOverviewQuery| { let _ = &value.time_interval; }; }
    // DashboardSessionsOverviewQuery.timeUnit
    { let _ = |value: &scm_sdk::airs::management::DashboardSessionsOverviewQuery| { let _ = &value.time_unit; }; }
    // DashboardSessionsOverviewSchema
    { let _ = scm_sdk::validation::parse("DashboardSessionsOverviewSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardSessionStats
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardSessionStats>; }
    // DashboardSessionStatsSchema
    { let _ = scm_sdk::validation::parse("DashboardSessionStatsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardSessionTransaction
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardSessionTransaction>; }
    // DashboardSessionTransactionQuery
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::DashboardSessionTransactionQuery>; }
    // DashboardSessionTransactionQuery.appId
    { let _ = |value: &scm_sdk::airs::management::DashboardSessionTransactionQuery| { let _ = &value.app_id; }; }
    // DashboardSessionTransactionQuery.appName
    { let _ = |value: &scm_sdk::airs::management::DashboardSessionTransactionQuery| { let _ = &value.app_name; }; }
    // DashboardSessionTransactionQuery.scanId
    { let _ = |value: &scm_sdk::airs::management::DashboardSessionTransactionQuery| { let _ = &value.scan_id; }; }
    // DashboardSessionTransactionQuery.scanSubReqId
    { let _ = |value: &scm_sdk::airs::management::DashboardSessionTransactionQuery| { let _ = &value.scan_sub_req_id; }; }
    // DashboardSessionTransactionQuery.sessionId
    { let _ = |value: &scm_sdk::airs::management::DashboardSessionTransactionQuery| { let _ = &value.session_id; }; }
    // DashboardSessionTransactionQuery.timeInterval
    { let _ = |value: &scm_sdk::airs::management::DashboardSessionTransactionQuery| { let _ = &value.time_interval; }; }
    // DashboardSessionTransactionQuery.timeUnit
    { let _ = |value: &scm_sdk::airs::management::DashboardSessionTransactionQuery| { let _ = &value.time_unit; }; }
    // DashboardSessionTransactionSchema
    { let _ = scm_sdk::validation::parse("DashboardSessionTransactionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardTimeRangeQuery
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::DashboardTimeRangeQuery>; }
    // DashboardTimeRangeQuery.timeInterval
    { let _ = |value: &scm_sdk::airs::management::DashboardTimeRangeQuery| { let _ = &value.time_interval; }; }
    // DashboardTimeRangeQuery.timeUnit
    { let _ = |value: &scm_sdk::airs::management::DashboardTimeRangeQuery| { let _ = &value.time_unit; }; }
    // DashboardTopApplicationsViolations
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardTopApplicationsViolations>; }
    // DashboardTopApplicationsViolationsSchema
    { let _ = scm_sdk::validation::parse("DashboardTopApplicationsViolationsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardTopApplicationViolations
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardTopApplicationViolations>; }
    // DashboardTopApplicationViolationsSchema
    { let _ = scm_sdk::validation::parse("DashboardTopApplicationViolationsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardTransactionAttributes
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardTransactionAttributes>; }
    // DashboardTransactionAttributesSchema
    { let _ = scm_sdk::validation::parse("DashboardTransactionAttributesSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DashboardViolationsTrendBucket
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DashboardViolationsTrendBucket>; }
    // DashboardViolationsTrendBucketSchema
    { let _ = scm_sdk::validation::parse("DashboardViolationsTrendBucketSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DatabaseSecurityItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DatabaseSecurityItem>; }
    // DatabaseSecurityItemSchema
    { let _ = scm_sdk::validation::parse("DatabaseSecurityItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DatabricksConnectionParams
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::DatabricksConnectionParams>; }
    // DatabricksConnectionParamsSchema
    { let _ = scm_sdk::validation::parse("DatabricksConnectionParamsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataFilteringDetails
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataFilteringDetails>; }
    // DataFilteringDetailsSchema
    { let _ = scm_sdk::validation::parse("DataFilteringDetailsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataFilteringProfileListAllParams
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::DataFilteringProfileListParams>; }
    // DataFilteringProfileListAllParams.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // DataFilteringProfileListAllParams.name
    { let _ = |value: &scm_sdk::dlp::DataFilteringProfileListParams| { let _ = &value.name; }; }
    // DataFilteringProfileListAllParams.size
    { let _ = |value: &scm_sdk::dlp::DataFilteringProfileListParams| { let _ = &value.size; }; }
    // DataFilteringProfileListAllParams.sort
    { let _ = |value: &scm_sdk::dlp::DataFilteringProfileListParams| { let _ = &value.sort; }; }
    // DataFilteringProfileListAllParams.status
    { let _ = |value: &scm_sdk::dlp::DataFilteringProfileListParams| { let _ = &value.status; }; }
    // DataFilteringProfileListParams
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::DataFilteringProfileListParams>; }
    // DataFilteringProfileListParams.name
    { let _ = |value: &scm_sdk::dlp::DataFilteringProfileListParams| { let _ = &value.name; }; }
    // DataFilteringProfileListParams.page
    { let _ = |value: &scm_sdk::dlp::DataFilteringProfileListParams| { let _ = &value.page; }; }
    // DataFilteringProfileListParams.size
    { let _ = |value: &scm_sdk::dlp::DataFilteringProfileListParams| { let _ = &value.size; }; }
    // DataFilteringProfileListParams.sort
    { let _ = |value: &scm_sdk::dlp::DataFilteringProfileListParams| { let _ = &value.sort; }; }
    // DataFilteringProfileListParams.status
    { let _ = |value: &scm_sdk::dlp::DataFilteringProfileListParams| { let _ = &value.status; }; }
    // DataFilteringProfileRequest
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataFilteringProfileRequest>; }
    // DataFilteringProfileRequestSchema
    { let _ = scm_sdk::validation::parse("DataFilteringProfileRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataFilteringProfileResponse
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataFilteringProfileResponse>; }
    // DataFilteringProfileResponseSchema
    { let _ = scm_sdk::validation::parse("DataFilteringProfileResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataFilteringProfilesClient
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::DataFilteringProfilesClient>; let _ = scm_sdk::dlp::DlpClient::new; let _ = scm_sdk::dlp::DlpClient::data_filtering_profiles; }
    // DataFilteringProfilesClient.get
    { let _ = scm_sdk::dlp::DataFilteringProfilesClient::get; }
    // DataFilteringProfilesClient.list
    { let _ = scm_sdk::dlp::DataFilteringProfilesClient::list; }
    // DataFilteringProfilesClient.listAll
    { let _ = scm_sdk::dlp::DataFilteringProfilesClient::list_all; let _ = scm_sdk::dlp::DataFilteringProfilesClient::list_pages; let _ = scm_sdk::dlp::DataFilteringProfilesClient::list_items; }
    // DataFilteringProfilesClient.replace
    { let _ = scm_sdk::dlp::DataFilteringProfilesClient::replace; }
    // DataFilteringProfilesClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // DataFilteringProfilesClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // DataFilteringProfilesClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // DataFilteringProfilesClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // DataFilteringRuleDTO
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataFilteringRuleDTO>; }
    // DataFilteringRuleDTOSchema
    { let _ = scm_sdk::validation::parse("DataFilteringRuleDTOSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataLeakDetectionMember
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DataLeakDetectionMember>; }
    // DataLeakDetectionMemberSchema
    { let _ = scm_sdk::validation::parse("DataLeakDetectionMemberSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataPatternConfidenceLevel
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataPatternConfidenceLevel>; }
    // DataPatternConfidenceLevelSchema
    { let _ = scm_sdk::validation::parse("DataPatternConfidenceLevelSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataPatternDetectionConfig
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataPatternDetectionConfig>; }
    // DataPatternDetectionConfigSchema
    { let _ = scm_sdk::validation::parse("DataPatternDetectionConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataPatternLicenseType
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataPatternLicenseType>; }
    // DataPatternLicenseTypeSchema
    { let _ = scm_sdk::validation::parse("DataPatternLicenseTypeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataPatternListAllParams
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::DataPatternListParams>; }
    // DataPatternListAllParams.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // DataPatternListAllParams.size
    { let _ = |value: &scm_sdk::dlp::DataPatternListParams| { let _ = &value.size; }; }
    // DataPatternListAllParams.sort
    { let _ = |value: &scm_sdk::dlp::DataPatternListParams| { let _ = &value.sort; }; }
    // DataPatternListParams
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::DataPatternListParams>; }
    // DataPatternListParams.page
    { let _ = |value: &scm_sdk::dlp::DataPatternListParams| { let _ = &value.page; }; }
    // DataPatternListParams.size
    { let _ = |value: &scm_sdk::dlp::DataPatternListParams| { let _ = &value.size; }; }
    // DataPatternListParams.sort
    { let _ = |value: &scm_sdk::dlp::DataPatternListParams| { let _ = &value.sort; }; }
    // DataPatternMatchingRules
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataPatternMatchingRules>; }
    // DataPatternMatchingRulesSchema
    { let _ = scm_sdk::validation::parse("DataPatternMatchingRulesSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataPatternPatchRequest
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataPatternPatchRequest>; }
    // DataPatternPatchRequestSchema
    { let _ = scm_sdk::validation::parse("DataPatternPatchRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataPatternRequest
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataPatternRequest>; }
    // DataPatternRequestSchema
    { let _ = scm_sdk::validation::parse("DataPatternRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataPatternResponse
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataPatternResponse>; }
    // DataPatternResponseSchema
    { let _ = scm_sdk::validation::parse("DataPatternResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataPatternsClient
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::DataPatternsClient>; let _ = scm_sdk::dlp::DlpClient::new; let _ = scm_sdk::dlp::DlpClient::data_patterns; }
    // DataPatternsClient.create
    { let _ = scm_sdk::dlp::DataPatternsClient::create; }
    // DataPatternsClient.delete
    { let _ = scm_sdk::dlp::DataPatternsClient::delete; }
    // DataPatternsClient.get
    { let _ = scm_sdk::dlp::DataPatternsClient::get; }
    // DataPatternsClient.list
    { let _ = scm_sdk::dlp::DataPatternsClient::list; }
    // DataPatternsClient.listAll
    { let _ = scm_sdk::dlp::DataPatternsClient::list_all; let _ = scm_sdk::dlp::DataPatternsClient::list_pages; let _ = scm_sdk::dlp::DataPatternsClient::list_items; }
    // DataPatternsClient.patch
    { let _ = scm_sdk::dlp::DataPatternsClient::patch; }
    // DataPatternsClient.replace
    { let _ = scm_sdk::dlp::DataPatternsClient::replace; }
    // DataPatternsClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // DataPatternsClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // DataPatternsClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // DataPatternsClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // DataPatternStatus
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataPatternStatus>; }
    // DataPatternStatusSchema
    { let _ = scm_sdk::validation::parse("DataPatternStatusSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataPatternTags
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataPatternTags>; }
    // DataPatternTagsSchema
    { let _ = scm_sdk::validation::parse("DataPatternTagsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataPatternTechnique
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataPatternTechnique>; }
    // DataPatternTechniqueSchema
    { let _ = scm_sdk::validation::parse("DataPatternTechniqueSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataPatternType
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataPatternType>; }
    // DataPatternTypeSchema
    { let _ = scm_sdk::validation::parse("DataPatternTypeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataProfileListAllParams
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::DataProfileListParams>; }
    // DataProfileListAllParams.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // DataProfileListAllParams.size
    { let _ = |value: &scm_sdk::dlp::DataProfileListParams| { let _ = &value.size; }; }
    // DataProfileListAllParams.sort
    { let _ = |value: &scm_sdk::dlp::DataProfileListParams| { let _ = &value.sort; }; }
    // DataProfileListParams
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::DataProfileListParams>; }
    // DataProfileListParams.page
    { let _ = |value: &scm_sdk::dlp::DataProfileListParams| { let _ = &value.page; }; }
    // DataProfileListParams.size
    { let _ = |value: &scm_sdk::dlp::DataProfileListParams| { let _ = &value.size; }; }
    // DataProfileListParams.sort
    { let _ = |value: &scm_sdk::dlp::DataProfileListParams| { let _ = &value.sort; }; }
    // DataProfilePatchRequest
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataProfilePatchRequest>; }
    // DataProfilePatchRequestSchema
    { let _ = scm_sdk::validation::parse("DataProfilePatchRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataProfileResponse
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataProfileResponse>; }
    // DataProfileResponseSchema
    { let _ = scm_sdk::validation::parse("DataProfileResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataProfilesClient
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::DataProfilesClient>; let _ = scm_sdk::dlp::DlpClient::new; let _ = scm_sdk::dlp::DlpClient::data_profiles; }
    // DataProfilesClient.create
    { let _ = scm_sdk::dlp::DataProfilesClient::create; }
    // DataProfilesClient.get
    { let _ = scm_sdk::dlp::DataProfilesClient::get; }
    // DataProfilesClient.list
    { let _ = scm_sdk::dlp::DataProfilesClient::list; }
    // DataProfilesClient.listAll
    { let _ = scm_sdk::dlp::DataProfilesClient::list_all; let _ = scm_sdk::dlp::DataProfilesClient::list_pages; let _ = scm_sdk::dlp::DataProfilesClient::list_items; }
    // DataProfilesClient.patch
    { let _ = scm_sdk::dlp::DataProfilesClient::patch; }
    // DataProfilesClient.replace
    { let _ = scm_sdk::dlp::DataProfilesClient::replace; }
    // DataProfilesClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // DataProfilesClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // DataProfilesClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // DataProfilesClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // DataProfileStatus
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataProfileStatus>; }
    // DataProfileStatusSchema
    { let _ = scm_sdk::validation::parse("DataProfileStatusSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataProfileSubtype
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataProfileSubtype>; }
    // DataProfileSubtypeSchema
    { let _ = scm_sdk::validation::parse("DataProfileSubtypeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataProfileType
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DataProfileType>; }
    // DataProfileTypeSchema
    { let _ = scm_sdk::validation::parse("DataProfileTypeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DataProtection
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DataProtection>; }
    // DataProtectionSchema
    { let _ = scm_sdk::validation::parse("DataProtectionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DateRangeFilter
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::DateRangeFilter>; }
    // DbsEntry
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::DbsEntry>; }
    // DbsEntrySchema
    { let _ = scm_sdk::validation::parse("DbsEntrySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DbsReport
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::DbsReport>; }
    // DbsReportSchema
    { let _ = scm_sdk::validation::parse("DbsReportSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DEFAULT_AI_GW_ADMIN_ENDPOINT
    { let _ = scm_sdk::constants::DEFAULT_AI_GW_ADMIN_ENDPOINT; }
    // DEFAULT_AI_GW_DATA_ENDPOINT
    { let _ = scm_sdk::constants::DEFAULT_AI_GW_DATA_ENDPOINT; }
    // DEFAULT_DLP_ENDPOINT
    { let _ = scm_sdk::constants::DEFAULT_DLP_ENDPOINT; }
    // DEFAULT_ENDPOINT
    { let _ = scm_sdk::constants::DEFAULT_ENDPOINT; }
    // DEFAULT_IAM_ENDPOINT
    { let _ = scm_sdk::constants::DEFAULT_IAM_ENDPOINT; }
    // DEFAULT_MGMT_ENDPOINT
    { let _ = scm_sdk::constants::DEFAULT_MGMT_ENDPOINT; }
    // DEFAULT_MODEL_SEC_DATA_ENDPOINT
    { let _ = scm_sdk::constants::DEFAULT_MODEL_SEC_DATA_ENDPOINT; }
    // DEFAULT_MODEL_SEC_MGMT_ENDPOINT
    { let _ = scm_sdk::constants::DEFAULT_MODEL_SEC_MGMT_ENDPOINT; }
    // DEFAULT_RED_TEAM_DATA_ENDPOINT
    { let _ = scm_sdk::constants::DEFAULT_RED_TEAM_DATA_ENDPOINT; }
    // DEFAULT_RED_TEAM_MGMT_ENDPOINT
    { let _ = scm_sdk::constants::DEFAULT_RED_TEAM_MGMT_ENDPOINT; }
    // DEFAULT_RED_TEAM_NETWORK_BROKER_ENDPOINT
    { let _ = scm_sdk::constants::DEFAULT_RED_TEAM_NETWORK_BROKER_ENDPOINT; }
    // DEFAULT_TOKEN_ENDPOINT
    { let _ = scm_sdk::constants::DEFAULT_TOKEN_ENDPOINT; }
    // DefaultTreeDetectionRule
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DefaultTreeDetectionRule>; }
    // DefaultTreeDetectionRuleSchema
    { let _ = scm_sdk::validation::parse("DefaultTreeDetectionRuleSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DeleteProfileConflict
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DeleteProfileConflict>; }
    // DeleteProfileConflictSchema
    { let _ = scm_sdk::validation::parse("DeleteProfileConflictSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DeleteProfileResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DeleteProfileResponse>; }
    // DeleteProfileResponseSchema
    { let _ = scm_sdk::validation::parse("DeleteProfileResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DeleteTopicConflict
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DeleteTopicConflict>; }
    // DeleteTopicConflictSchema
    { let _ = scm_sdk::validation::parse("DeleteTopicConflictSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DeleteTopicResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DeleteTopicResponse>; }
    // DeleteTopicResponseSchema
    { let _ = scm_sdk::validation::parse("DeleteTopicResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DeploymentProfileAttribute
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::DeploymentProfileAttribute>; }
    // DeploymentProfileAttributeSchema
    { let _ = scm_sdk::validation::parse("DeploymentProfileAttributeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DeploymentProfileEntry
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DeploymentProfileEntry>; }
    // DeploymentProfileEntrySchema
    { let _ = scm_sdk::validation::parse("DeploymentProfileEntrySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DeploymentProfileListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::DeploymentProfileListOptions>; }
    // DeploymentProfileListOptions.unactivated
    { let _ = |value: &scm_sdk::airs::management::DeploymentProfileListOptions| { let _ = &value.unactivated; }; }
    // DeploymentProfileRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::DeploymentProfileRequest>; }
    // DeploymentProfileRequestSchema
    { let _ = scm_sdk::validation::parse("DeploymentProfileRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DeploymentProfilesClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::DeploymentProfilesClient>; let _ = scm_sdk::airs::management::DeploymentProfilesClient::new; }
    // DeploymentProfilesClient.list
    { let _ = scm_sdk::airs::management::DeploymentProfilesClient::list; }
    // DeploymentProfilesClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // DeploymentProfilesClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // DeploymentProfilesClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // DeploymentProfilesClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // DeploymentProfilesResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DeploymentProfilesResponse>; }
    // DeploymentProfilesResponseSchema
    { let _ = scm_sdk::validation::parse("DeploymentProfilesResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DestinationAttributes
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DestinationAttributes>; }
    // DestinationAttributesSchema
    { let _ = scm_sdk::validation::parse("DestinationAttributesSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DetectionRule
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DetectionRule>; }
    // DetectionRuleItem
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DetectionRuleItem>; }
    // DetectionRuleItemSchema
    { let _ = scm_sdk::validation::parse("DetectionRuleItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DetectionRuleSchema
    { let _ = scm_sdk::validation::parse("DetectionRuleSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DetectionServiceName
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::DetectionServiceName>; }
    // DetectionServiceNameType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::DetectionServiceNameType>; }
    // DetectionServiceResult
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::DetectionServiceResult>; }
    // DetectionServiceResultSchema
    { let _ = scm_sdk::validation::parse("DetectionServiceResultSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DetectorViolationBreakdownEntry
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DetectorViolationBreakdownEntry>; }
    // DetectorViolationBreakdownEntrySchema
    { let _ = scm_sdk::validation::parse("DetectorViolationBreakdownEntrySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // Device
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::Device>; }
    // DeviceInstance
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::DeviceInstance>; }
    // DeviceInstanceSchema
    { let _ = scm_sdk::validation::parse("DeviceInstanceSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DeviceLicense
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::DeviceLicense>; }
    // DeviceLicenseSchema
    { let _ = scm_sdk::validation::parse("DeviceLicenseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DeviceRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::DeviceRequest>; }
    // DeviceRequestSchema
    { let _ = scm_sdk::validation::parse("DeviceRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DeviceResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::DeviceResponse>; }
    // DeviceResponseSchema
    { let _ = scm_sdk::validation::parse("DeviceResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DeviceSchema
    { let _ = scm_sdk::validation::parse("DeviceSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DeviceStatus
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::DeviceStatus>; }
    // DeviceStatusSchema
    { let _ = scm_sdk::validation::parse("DeviceStatusSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DictionariesClient
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::DictionariesClient>; let _ = scm_sdk::dlp::DlpClient::new; let _ = scm_sdk::dlp::DlpClient::dictionaries; }
    // DictionariesClient.create
    { let _ = scm_sdk::dlp::DictionariesClient::create; }
    // DictionariesClient.delete
    { let _ = scm_sdk::dlp::DictionariesClient::delete; }
    // DictionariesClient.get
    { let _ = scm_sdk::dlp::DictionariesClient::get; }
    // DictionariesClient.list
    { let _ = scm_sdk::dlp::DictionariesClient::list; }
    // DictionariesClient.listAll
    { let _ = scm_sdk::dlp::DictionariesClient::list_all; let _ = scm_sdk::dlp::DictionariesClient::list_pages; let _ = scm_sdk::dlp::DictionariesClient::list_items; }
    // DictionariesClient.patch
    { let _ = scm_sdk::dlp::DictionariesClient::patch; }
    // DictionariesClient.replace
    { let _ = scm_sdk::dlp::DictionariesClient::replace; }
    // DictionariesClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // DictionariesClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // DictionariesClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // DictionariesClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // DictionaryCategory
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DictionaryCategory>; }
    // DictionaryCategorySchema
    { let _ = scm_sdk::validation::parse("DictionaryCategorySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DictionaryClassification
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DictionaryClassification>; }
    // DictionaryClassificationSchema
    { let _ = scm_sdk::validation::parse("DictionaryClassificationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DictionaryDetectionSubTechnique
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DictionaryDetectionSubTechnique>; }
    // DictionaryDetectionSubTechniqueSchema
    { let _ = scm_sdk::validation::parse("DictionaryDetectionSubTechniqueSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DictionaryDetectionTechnique
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DictionaryDetectionTechnique>; }
    // DictionaryDetectionTechniqueSchema
    { let _ = scm_sdk::validation::parse("DictionaryDetectionTechniqueSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DictionaryFileInput
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::BinaryFile>; }
    // DictionaryGetParams
    { let _ = std::marker::PhantomData::<Option<bool>>; }
    // DictionaryGetParams.includeKeywords
    { let _ = scm_sdk::dlp::DictionariesClient::get; }
    // DictionaryListAllParams
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::DictionaryListParams>; }
    // DictionaryListAllParams.keywords
    { let _ = |value: &scm_sdk::dlp::DictionaryListParams| { let _ = &value.keywords; }; }
    // DictionaryListAllParams.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // DictionaryListAllParams.size
    { let _ = |value: &scm_sdk::dlp::DictionaryListParams| { let _ = &value.size; }; }
    // DictionaryListAllParams.sort
    { let _ = |value: &scm_sdk::dlp::DictionaryListParams| { let _ = &value.sort; }; }
    // DictionaryListParams
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::DictionaryListParams>; }
    // DictionaryListParams.keywords
    { let _ = |value: &scm_sdk::dlp::DictionaryListParams| { let _ = &value.keywords; }; }
    // DictionaryListParams.page
    { let _ = |value: &scm_sdk::dlp::DictionaryListParams| { let _ = &value.page; }; }
    // DictionaryListParams.size
    { let _ = |value: &scm_sdk::dlp::DictionaryListParams| { let _ = &value.size; }; }
    // DictionaryListParams.sort
    { let _ = |value: &scm_sdk::dlp::DictionaryListParams| { let _ = &value.sort; }; }
    // DictionaryMetaDataDTO
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DictionaryMetaDataDTO>; }
    // DictionaryMetaDataDTOSchema
    { let _ = scm_sdk::validation::parse("DictionaryMetaDataDTOSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DictionaryPatchRequest
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DictionaryPatchRequest>; }
    // DictionaryPatchRequestSchema
    { let _ = scm_sdk::validation::parse("DictionaryPatchRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DictionaryRequest
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DictionaryRequest>; }
    // DictionaryRequestSchema
    { let _ = scm_sdk::validation::parse("DictionaryRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DictionaryResponse
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DictionaryResponse>; }
    // DictionaryResponseSchema
    { let _ = scm_sdk::validation::parse("DictionaryResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DictionaryTags
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DictionaryTags>; }
    // DictionaryTagsSchema
    { let _ = scm_sdk::validation::parse("DictionaryTagsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DictionaryType
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DictionaryType>; }
    // DictionaryTypeSchema
    { let _ = scm_sdk::validation::parse("DictionaryTypeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DictionaryUploadParams
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::DictionaryUpload>; }
    // DictionaryUploadParams.file
    { let _ = |value: &scm_sdk::dlp::DictionaryUpload| { let _ = &value.file; }; }
    // DictionaryUploadParams.includeKeywords
    { let _ = |value: &scm_sdk::dlp::DictionaryUpload| { let _ = &value.include_keywords; }; }
    // DictionaryUploadParams.metadata
    { let _ = |value: &scm_sdk::dlp::DictionaryUpload| { let _ = &value.metadata; }; }
    // DLP_DATA_FILTERING_PROFILES_PATH
    { let _ = scm_sdk::constants::DLP_DATA_FILTERING_PROFILES_PATH; }
    // DLP_DATA_PATTERNS_PATH
    { let _ = scm_sdk::constants::DLP_DATA_PATTERNS_PATH; }
    // DLP_DATA_PROFILES_PATH
    { let _ = scm_sdk::constants::DLP_DATA_PROFILES_PATH; }
    // DLP_DICTIONARIES_PATH
    { let _ = scm_sdk::constants::DLP_DICTIONARIES_PATH; }
    // DlpDataProfile
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DlpDataProfile>; }
    // DlpDataProfilePolicy
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DlpDataProfilePolicy>; }
    // DlpDataProfilePolicySchema
    { let _ = scm_sdk::validation::parse("DlpDataProfilePolicySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DlpDataProfileSchema
    { let _ = scm_sdk::validation::parse("DlpDataProfileSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DlpNamespace
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::DlpClient>; let _ = scm_sdk::dlp::DlpClient::new; }
    // DlpNamespace.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // DlpNamespace.baseUrl
    { let _ = scm_sdk::Client::endpoint; }
    // DlpNamespace.dataFilteringProfiles
    { let _ = scm_sdk::dlp::DlpClient::data_filtering_profiles; }
    // DlpNamespace.dataPatterns
    { let _ = scm_sdk::dlp::DlpClient::data_patterns; }
    // DlpNamespace.dataProfiles
    { let _ = scm_sdk::dlp::DlpClient::data_profiles; }
    // DlpNamespace.dictionaries
    { let _ = scm_sdk::dlp::DlpClient::dictionaries; }
    // DlpNamespace.numRetries
    { let _ = scm_sdk::Client::limits; }
    // DlpNamespaceOptions
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::DlpClient>; }
    // DlpNamespaceOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // DlpNamespaceOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // DlpNamespaceOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // DlpPatternDetection
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::DlpPatternDetection>; }
    // DlpPatternDetectionSchema
    { let _ = scm_sdk::validation::parse("DlpPatternDetectionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DlpProfileListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DlpProfileListResponse>; }
    // DlpProfileListResponseSchema
    { let _ = scm_sdk::validation::parse("DlpProfileListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DlpProfilesClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::DlpProfilesClient>; let _ = scm_sdk::airs::management::DlpProfilesClient::new; }
    // DlpProfilesClient.list
    { let _ = scm_sdk::airs::management::DlpProfilesClient::list; }
    // DlpProfilesClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // DlpProfilesClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // DlpProfilesClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // DlpProfilesClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // DlpReport
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::DlpReport>; }
    // DlpReportSchema
    { let _ = scm_sdk::validation::parse("DlpReportSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DlpRule
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::DlpRule>; }
    // DlpRuleSchema
    { let _ = scm_sdk::validation::parse("DlpRuleSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DSDetailResult
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::DSDetailResult>; }
    // DSDetailResultSchema
    { let _ = scm_sdk::validation::parse("DSDetailResultSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DSResultMetadata
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::DSResultMetadata>; }
    // DSResultMetadataSchema
    { let _ = scm_sdk::validation::parse("DSResultMetadataSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DynamicJobMetadata
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::DynamicJobMetadata>; }
    // DynamicJobMetadataSchema
    { let _ = scm_sdk::validation::parse("DynamicJobMetadataSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DynamicJobReport
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::DynamicJobReport>; }
    // DynamicJobReportSchema
    { let _ = scm_sdk::validation::parse("DynamicJobReportSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // DynamicJobReportStats
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::DynamicJobReportStats>; }
    // DynamicJobReportStatsSchema
    { let _ = scm_sdk::validation::parse("DynamicJobReportStatsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ErrorCategoryTrendsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::ErrorCategoryTrendsResponse>; }
    // ErrorCategoryTrendsResponseSchema
    { let _ = scm_sdk::validation::parse("ErrorCategoryTrendsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ErrorCodes
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ErrorCodes>; }
    // ErrorLog
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ErrorLog>; }
    // ErrorLogListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ErrorLogListResponse>; }
    // ErrorLogListResponseSchema
    { let _ = scm_sdk::validation::parse("ErrorLogListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ErrorLogSchema
    { let _ = scm_sdk::validation::parse("ErrorLogSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ErrorResponse
    { let _ = std::marker::PhantomData::<scm_sdk::shared::models::ErrorResponse>; }
    // ErrorResponseSchema
    { let _ = scm_sdk::validation::parse("ErrorResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ErrorSource
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ErrorSource>; }
    // ErrorStatus
    { let _ = std::marker::PhantomData::<scm_sdk::shared::models::ErrorStatus>; }
    // ErrorStatusType
    { let _ = std::marker::PhantomData::<scm_sdk::shared::models::ErrorStatusType>; }
    // ErrorTrendsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::ErrorTrendsResponse>; }
    // ErrorTrendsResponseSchema
    { let _ = scm_sdk::validation::parse("ErrorTrendsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ErrorType
    { assert_eq!(scm_sdk::ErrorType::ServerSideError.as_str(), "AISEC_SERVER_SIDE_ERROR"); assert_eq!(scm_sdk::ErrorType::ClientSideError.as_str(), "AISEC_CLIENT_SIDE_ERROR"); assert_eq!(scm_sdk::ErrorType::UserRequestPayloadError.as_str(), "AISEC_USER_REQUEST_PAYLOAD_ERROR"); assert_eq!(scm_sdk::ErrorType::MissingVariable.as_str(), "AISEC_MISSING_VARIABLE"); assert_eq!(scm_sdk::ErrorType::SdkError.as_str(), "AISEC_SDK_ERROR"); assert_eq!(scm_sdk::ErrorType::OAuthError.as_str(), "AISEC_OAUTH_ERROR"); assert_eq!(scm_sdk::ErrorType::ResponseValidation.as_str(), "AISEC_RESPONSE_VALIDATION"); }
    // EulaAcceptRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::EulaAcceptRequest>; }
    // EulaAcceptRequestSchema
    { let _ = scm_sdk::validation::parse("EulaAcceptRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // EulaContentResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::EulaContentResponse>; }
    // EulaContentResponseSchema
    { let _ = scm_sdk::validation::parse("EulaContentResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // EulaResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::EulaResponse>; }
    // EulaResponseSchema
    { let _ = scm_sdk::validation::parse("EulaResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // EvalOutcome
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::EvalOutcome>; }
    // EvalSummary
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::EvalSummary>; }
    // EvalSummarySchema
    { let _ = scm_sdk::validation::parse("EvalSummarySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ExceptionRuleDTO
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::ExceptionRuleDTO>; }
    // ExceptionRuleDTOSchema
    { let _ = scm_sdk::validation::parse("ExceptionRuleDTOSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // Exclusions
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::Exclusions>; }
    // ExclusionsSchema
    { let _ = scm_sdk::validation::parse("ExclusionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ExpressionOperatorType
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::ExpressionOperatorType>; }
    // ExpressionOperatorTypeSchema
    { let _ = scm_sdk::validation::parse("ExpressionOperatorTypeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ExpressionTreeNode
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::ExpressionTreeNode>; }
    // ExpressionTreeNode.operator_type
    { let _ = |value: &scm_sdk::dlp::models::ExpressionTreeNode| { let _ = &value.operator_type; }; }
    // ExpressionTreeNode.rule_item
    { let _ = |value: &scm_sdk::dlp::models::ExpressionTreeNode| { let _ = &value.rule_item; }; }
    // ExpressionTreeNode.sub_expressions
    { let _ = |value: &scm_sdk::dlp::models::ExpressionTreeNode| { let _ = &value.sub_expressions; }; }
    // ExpressionTreeNodeSchema
    { let _ = scm_sdk::validation::parse("ExpressionTreeNodeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // FeedbackModelsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::FeedbackModelsResponse>; }
    // FeedbackModelsResponseSchema
    { let _ = scm_sdk::validation::parse("FeedbackModelsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // FeedbackScoreDistributionResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::FeedbackScoreDistributionResponse>; }
    // FeedbackScoreDistributionResponseSchema
    { let _ = scm_sdk::validation::parse("FeedbackScoreDistributionResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // FileFormat
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::FileFormat>; }
    // FileList
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::FileList>; }
    // FileListSchema
    { let _ = scm_sdk::validation::parse("FileListSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // FileResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::FileResponse>; }
    // FileResponseSchema
    { let _ = scm_sdk::validation::parse("FileResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // FileScanData
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::FileScanData>; }
    // FileScanDataSchema
    { let _ = scm_sdk::validation::parse("FileScanDataSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // FileScanResult
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::FileScanResult>; }
    // FileType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::FileType>; }
    // GatewayAcuvityScanParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAcuvityScanParameters>; }
    // GatewayAcuvityScanParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayAcuvityScanParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayAdminGuardrail
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAdminGuardrail>; }
    // GatewayAdminGuardrailCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAdminGuardrailCreateRequest>; }
    // GatewayAdminGuardrailCreateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayAdminGuardrailCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayAdminGuardrailCreateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAdminGuardrailCreateResponse>; }
    // GatewayAdminGuardrailCreateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayAdminGuardrailCreateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayAdminGuardrailDetail
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAdminGuardrailDetail>; }
    // GatewayAdminGuardrailDetailSchema
    { let _ = scm_sdk::validation::parse("GatewayAdminGuardrailDetailSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayAdminGuardrailListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAdminGuardrailListOptions>; }
    // GatewayAdminGuardrailListOptionsSchema
    { let _ = scm_sdk::validation::parse("GatewayAdminGuardrailListOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayAdminGuardrailListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAdminGuardrailListResponse>; }
    // GatewayAdminGuardrailListResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayAdminGuardrailListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayAdminGuardrailMcpMappingRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAdminGuardrailMcpMappingRequest>; }
    // GatewayAdminGuardrailMcpMappingRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayAdminGuardrailMcpMappingRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayAdminGuardrailMcpSyncRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAdminGuardrailMcpSyncRequest>; }
    // GatewayAdminGuardrailMcpSyncRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayAdminGuardrailMcpSyncRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayAdminGuardrailSchema
    { let _ = scm_sdk::validation::parse("GatewayAdminGuardrailSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayAdminGuardrailUpdateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAdminGuardrailUpdateResponse>; }
    // GatewayAdminGuardrailUpdateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayAdminGuardrailUpdateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayAllowedRequestTypesParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAllowedRequestTypesParameters>; }
    // GatewayAllowedRequestTypesParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayAllowedRequestTypesParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayApiKey
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayApiKey>; }
    // GatewayApiKeyCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayApiKeyCreateRequest>; }
    // GatewayApiKeyCreateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayApiKeyCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayApiKeyCreateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayApiKeyCreateResponse>; }
    // GatewayApiKeyCreateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayApiKeyCreateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayApiKeyRotateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayApiKeyRotateRequest>; }
    // GatewayApiKeyRotateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayApiKeyRotateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayApiKeyRotateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayApiKeyRotateResponse>; }
    // GatewayApiKeyRotateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayApiKeyRotateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayApiKeyRotationPolicy
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayApiKeyRotationPolicy>; }
    // GatewayApiKeyRotationPolicySchema
    { let _ = scm_sdk::validation::parse("GatewayApiKeyRotationPolicySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayApiKeySchema
    { let _ = scm_sdk::validation::parse("GatewayApiKeySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayApiKeyScope
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayApiKeyScope>; }
    // GatewayApiKeyScopeSchema
    { let _ = scm_sdk::validation::parse("GatewayApiKeyScopeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayApiKeyUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayApiKeyUpdateRequest>; }
    // GatewayApiKeyUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayApiKeyUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayAporiaParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAporiaParameters>; }
    // GatewayAporiaParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayAporiaParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayAuditLogRecord
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAuditLogRecord>; }
    // GatewayAuditLogRecordSchema
    { let _ = scm_sdk::validation::parse("GatewayAuditLogRecordSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayAuditLogsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAuditLogsResponse>; }
    // GatewayAuditLogsResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayAuditLogsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayAzureAIConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAzureAIConfiguration>; }
    // GatewayAzureAIConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayAzureAIConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayAzureContentSafetyParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAzureContentSafetyParameters>; }
    // GatewayAzureContentSafetyParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayAzureContentSafetyParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayAzureDeploymentConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAzureDeploymentConfig>; }
    // GatewayAzureDeploymentConfigSchema
    { let _ = scm_sdk::validation::parse("GatewayAzureDeploymentConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayAzureDeploymentConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAzureDeploymentConfiguration>; }
    // GatewayAzureDeploymentConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayAzureDeploymentConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayAzureOpenAIConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAzureOpenAIConfiguration>; }
    // GatewayAzureOpenAIConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayAzureOpenAIConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayAzurePIIParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayAzurePIIParameters>; }
    // GatewayAzurePIIParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayAzurePIIParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayBasicParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayBasicParameters>; }
    // GatewayBasicParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayBasicParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayBedrockConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayBedrockConfiguration>; }
    // GatewayBedrockConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayBedrockConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayBedrockGuardParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayBedrockGuardParameters>; }
    // GatewayBedrockGuardParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayBedrockGuardParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayBulkSyncMcpServerMappingsRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayBulkSyncMcpServerMappingsRequest>; }
    // GatewayBulkSyncMcpServerMappingsRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayBulkSyncMcpServerMappingsRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayBulkSyncMcpServerMappingsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayBulkSyncMcpServerMappingsResponse>; }
    // GatewayBulkSyncMcpServerMappingsResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayBulkSyncMcpServerMappingsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCatalogAzureAIConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCatalogAzureAIConfiguration>; }
    // GatewayCatalogAzureAIConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayCatalogAzureAIConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCatalogAzureOpenAIConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCatalogAzureOpenAIConfiguration>; }
    // GatewayCatalogAzureOpenAIConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayCatalogAzureOpenAIConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCatalogBedrockConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCatalogBedrockConfiguration>; }
    // GatewayCatalogBedrockConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayCatalogBedrockConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCatalogCortexConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCatalogCortexConfiguration>; }
    // GatewayCatalogCortexConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayCatalogCortexConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCatalogCustomHostConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCatalogCustomHostConfiguration>; }
    // GatewayCatalogCustomHostConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayCatalogCustomHostConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCatalogGuardrailParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCatalogGuardrailParameters>; }
    // GatewayCatalogGuardrailParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayCatalogGuardrailParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCatalogHuggingFaceConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCatalogHuggingFaceConfiguration>; }
    // GatewayCatalogHuggingFaceConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayCatalogHuggingFaceConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCatalogMcpConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCatalogMcpConfiguration>; }
    // GatewayCatalogMcpConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayCatalogMcpConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCatalogOpenAIConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCatalogOpenAIConfiguration>; }
    // GatewayCatalogOpenAIConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayCatalogOpenAIConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCatalogProvider
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCatalogProvider>; }
    // GatewayCatalogProviderConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCatalogProviderConfiguration>; }
    // GatewayCatalogProviderConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayCatalogProviderConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCatalogProviderSchema
    { let _ = scm_sdk::validation::parse("GatewayCatalogProviderSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCatalogSageMakerConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCatalogSageMakerConfiguration>; }
    // GatewayCatalogSageMakerConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayCatalogSageMakerConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCatalogSecretMapping
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCatalogSecretMapping>; }
    // GatewayCatalogSecretMappingSchema
    { let _ = scm_sdk::validation::parse("GatewayCatalogSecretMappingSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCatalogVertexAIConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCatalogVertexAIConfiguration>; }
    // GatewayCatalogVertexAIConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayCatalogVertexAIConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCatalogWorkersAIConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCatalogWorkersAIConfiguration>; }
    // GatewayCatalogWorkersAIConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayCatalogWorkersAIConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCharacterCountParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCharacterCountParameters>; }
    // GatewayCharacterCountParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayCharacterCountParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayChartRecord
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayChartRecord>; }
    // GatewayChartRecordSchema
    { let _ = scm_sdk::validation::parse("GatewayChartRecordSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayChatCompletion
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayChatCompletion>; }
    // GatewayChatCompletionChunk
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayChatCompletionChunk>; }
    // GatewayChatCompletionRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayChatCompletionRequest>; }
    // GatewayCondition
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCondition>; }
    // GatewayConditionSchema
    { let _ = scm_sdk::validation::parse("GatewayConditionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayConfig>; }
    // GatewayConfigCacheMode
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayConfigCacheMode>; }
    // GatewayConfigCacheModeSchema
    { let _ = scm_sdk::validation::parse("GatewayConfigCacheModeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayConfigCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayConfigCreateRequest>; }
    // GatewayConfigCreateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayConfigCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayConfigCreateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayConfigCreateResponse>; }
    // GatewayConfigCreateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayConfigCreateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayConfigDetail
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayConfigDetail>; }
    // GatewayConfigDetailSchema
    { let _ = scm_sdk::validation::parse("GatewayConfigDetailSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayConfigSchema
    { let _ = scm_sdk::validation::parse("GatewayConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayConfigStrategy
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayConfigStrategy>; }
    // GatewayConfigStrategySchema
    { let _ = scm_sdk::validation::parse("GatewayConfigStrategySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayConfigUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayConfigUpdateRequest>; }
    // GatewayConfigUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayConfigUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayConfigVersion
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayConfigVersion>; }
    // GatewayConfigVersionSchema
    { let _ = scm_sdk::validation::parse("GatewayConfigVersionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayContainsCodeParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayContainsCodeParameters>; }
    // GatewayContainsCodeParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayContainsCodeParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayContainsParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayContainsParameters>; }
    // GatewayContainsParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayContainsParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCortexConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCortexConfiguration>; }
    // GatewayCortexConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayCortexConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCreatePolicyResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCreatePolicyResponse>; }
    // GatewayCreatePolicyResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayCreatePolicyResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCreateRateLimitsPolicyRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCreateRateLimitsPolicyRequest>; }
    // GatewayCreateRateLimitsPolicyRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayCreateRateLimitsPolicyRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCreateSecretReferenceRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCreateSecretReferenceRequest>; }
    // GatewayCreateSecretReferenceRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayCreateSecretReferenceRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCreateUsageLimitsPolicyRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCreateUsageLimitsPolicyRequest>; }
    // GatewayCreateUsageLimitsPolicyRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayCreateUsageLimitsPolicyRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayCustomHostConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayCustomHostConfiguration>; }
    // GatewayCustomHostConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayCustomHostConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayDefaultsInput
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayDefaultsInput>; }
    // GatewayDefaultsInputSchema
    { let _ = scm_sdk::validation::parse("GatewayDefaultsInputSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayDeployment
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayDeployment>; }
    // GatewayDeploymentAuthSettingsInput
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayDeploymentAuthSettingsInput>; }
    // GatewayDeploymentAuthSettingsInputSchema
    { let _ = scm_sdk::validation::parse("GatewayDeploymentAuthSettingsInputSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayDeploymentCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayDeploymentCreateRequest>; }
    // GatewayDeploymentCreateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayDeploymentCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayDeploymentCreateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayDeploymentCreateResponse>; }
    // GatewayDeploymentCreateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayDeploymentCreateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayDeploymentDetail
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayDeploymentDetail>; }
    // GatewayDeploymentDetailSchema
    { let _ = scm_sdk::validation::parse("GatewayDeploymentDetailSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayDeploymentPingResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayDeploymentPingResponse>; }
    // GatewayDeploymentPingResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayDeploymentPingResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayDeploymentSchema
    { let _ = scm_sdk::validation::parse("GatewayDeploymentSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayDeploymentStatus
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayDeploymentStatus>; }
    // GatewayDeploymentStatusSchema
    { let _ = scm_sdk::validation::parse("GatewayDeploymentStatusSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayDeploymentTags
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayDeploymentTags>; }
    // GatewayDeploymentTagsSchema
    { let _ = scm_sdk::validation::parse("GatewayDeploymentTagsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayDeploymentType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayDeploymentType>; }
    // GatewayDeploymentTypeSchema
    { let _ = scm_sdk::validation::parse("GatewayDeploymentTypeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayDeploymentUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayDeploymentUpdateRequest>; }
    // GatewayDeploymentUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayDeploymentUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayDottedValueEntry
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayDottedValueEntry>; }
    // GatewayDottedValueEntry.path
    { let _ = |value: &scm_sdk::airs::gateway::models::GatewayDottedValueEntry| { let _ = &value.path; }; }
    // GatewayDottedValueEntry.value
    { let _ = |value: &scm_sdk::airs::gateway::models::GatewayDottedValueEntry| { let _ = &value.value; }; }
    // GatewayDownloadLogsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayDownloadLogsResponse>; }
    // GatewayDownloadLogsResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayDownloadLogsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayEmbeddingRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayEmbeddingRequest>; }
    // GatewayEmbeddingResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayEmbeddingResponse>; }
    // GatewayEndsWithParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayEndsWithParameters>; }
    // GatewayEndsWithParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayEndsWithParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayExportItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayExportItem>; }
    // GatewayExportItemSchema
    { let _ = scm_sdk::validation::parse("GatewayExportItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayExportListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayExportListResponse>; }
    // GatewayExportListResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayExportListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayExportTaskResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayExportTaskResponse>; }
    // GatewayExportTaskResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayExportTaskResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayFilterBoundariesResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayFilterBoundariesResponse>; }
    // GatewayFilterBoundariesResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayFilterBoundariesResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayGenerationsFilterSchema
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayGenerationsFilterSchema>; }
    // GatewayGenerationsFilterSchemaSchema
    { let _ = scm_sdk::validation::parse("GatewayGenerationsFilterSchemaSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayGlobalWorkspaceAccess
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayGlobalWorkspaceAccess>; }
    // GatewayGlobalWorkspaceAccessInput
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayGlobalWorkspaceAccessInput>; }
    // GatewayGlobalWorkspaceAccessInputSchema
    { let _ = scm_sdk::validation::parse("GatewayGlobalWorkspaceAccessInputSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayGlobalWorkspaceAccessSchema
    { let _ = scm_sdk::validation::parse("GatewayGlobalWorkspaceAccessSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayGroupBy
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayGroupBy>; }
    // GatewayGroupBySchema
    { let _ = scm_sdk::validation::parse("GatewayGroupBySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayGroupRow
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayGroupRow>; }
    // GatewayGroupRowSchema
    { let _ = scm_sdk::validation::parse("GatewayGroupRowSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayGuardrail
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayGuardrail>; }
    // GatewayGuardrailActions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayGuardrailActions>; }
    // GatewayGuardrailActionsSchema
    { let _ = scm_sdk::validation::parse("GatewayGuardrailActionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayGuardrailCatalogResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayGuardrailCatalogResponse>; }
    // GatewayGuardrailCatalogResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayGuardrailCatalogResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayGuardrailCheck
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayGuardrailCheck>; }
    // GatewayGuardrailCheckSchema
    { let _ = scm_sdk::validation::parse("GatewayGuardrailCheckSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayGuardrailCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayGuardrailCreateRequest>; }
    // GatewayGuardrailCreateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayGuardrailCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayGuardrailCreateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayGuardrailCreateResponse>; }
    // GatewayGuardrailCreateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayGuardrailCreateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayGuardrailDetail
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayGuardrailDetail>; }
    // GatewayGuardrailDetailSchema
    { let _ = scm_sdk::validation::parse("GatewayGuardrailDetailSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayGuardrailSchema
    { let _ = scm_sdk::validation::parse("GatewayGuardrailSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayGuardrailUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayGuardrailUpdateRequest>; }
    // GatewayGuardrailUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayGuardrailUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayHuggingFaceConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayHuggingFaceConfiguration>; }
    // GatewayHuggingFaceConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayHuggingFaceConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceAnalyticsMetrics
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceAnalyticsMetrics>; }
    // GatewayInferenceAnalyticsMetricsSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceAnalyticsMetricsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceAnnotation
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceAnnotation>; }
    // GatewayInferenceAnnotationSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceAnnotationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceBatch
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceBatch>; }
    // GatewayInferenceBatchSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceBatchSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionFunctionCallOption
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionFunctionCallOption>; }
    // GatewayInferenceChatCompletionFunctionCallOptionSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionFunctionCallOptionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionFunctions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionFunctions>; }
    // GatewayInferenceChatCompletionFunctionsSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionFunctionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionMessageContentBlock
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionMessageContentBlock>; }
    // GatewayInferenceChatCompletionMessageContentBlockSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionMessageContentBlockSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionMessageContentPartRedactedThinking
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionMessageContentPartRedactedThinking>; }
    // GatewayInferenceChatCompletionMessageContentPartRedactedThinkingSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionMessageContentPartRedactedThinkingSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionMessageContentPartThinking
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionMessageContentPartThinking>; }
    // GatewayInferenceChatCompletionMessageContentPartThinkingSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionMessageContentPartThinkingSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionMessageToolCall
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionMessageToolCall>; }
    // GatewayInferenceChatCompletionMessageToolCallChunk
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionMessageToolCallChunk>; }
    // GatewayInferenceChatCompletionMessageToolCallChunkSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionMessageToolCallChunkSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionMessageToolCalls
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionMessageToolCalls>; }
    // GatewayInferenceChatCompletionMessageToolCallSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionMessageToolCallSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionMessageToolCallsSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionMessageToolCallsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionNamedToolChoice
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionNamedToolChoice>; }
    // GatewayInferenceChatCompletionNamedToolChoiceSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionNamedToolChoiceSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionRequestAssistantMessage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionRequestAssistantMessage>; }
    // GatewayInferenceChatCompletionRequestAssistantMessageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionRequestAssistantMessageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionRequestDeveloperMessage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionRequestDeveloperMessage>; }
    // GatewayInferenceChatCompletionRequestDeveloperMessageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionRequestDeveloperMessageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionRequestFunctionMessage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionRequestFunctionMessage>; }
    // GatewayInferenceChatCompletionRequestFunctionMessageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionRequestFunctionMessageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionRequestMessage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionRequestMessage>; }
    // GatewayInferenceChatCompletionRequestMessageContentPart
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionRequestMessageContentPart>; }
    // GatewayInferenceChatCompletionRequestMessageContentPartImage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionRequestMessageContentPartImage>; }
    // GatewayInferenceChatCompletionRequestMessageContentPartImageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionRequestMessageContentPartImageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionRequestMessageContentPartSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionRequestMessageContentPartSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionRequestMessageContentPartText
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionRequestMessageContentPartText>; }
    // GatewayInferenceChatCompletionRequestMessageContentPartTextSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionRequestMessageContentPartTextSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionRequestMessageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionRequestMessageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionRequestSystemMessage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionRequestSystemMessage>; }
    // GatewayInferenceChatCompletionRequestSystemMessageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionRequestSystemMessageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionRequestToolMessage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionRequestToolMessage>; }
    // GatewayInferenceChatCompletionRequestToolMessageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionRequestToolMessageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionRequestUserMessage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionRequestUserMessage>; }
    // GatewayInferenceChatCompletionRequestUserMessageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionRequestUserMessageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionResponseMessage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionResponseMessage>; }
    // GatewayInferenceChatCompletionResponseMessageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionResponseMessageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionStreamOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionStreamOptions>; }
    // GatewayInferenceChatCompletionStreamOptionsSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionStreamOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionStreamResponseDelta
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionStreamResponseDelta>; }
    // GatewayInferenceChatCompletionStreamResponseDeltaSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionStreamResponseDeltaSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionTokenLogprob
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionTokenLogprob>; }
    // GatewayInferenceChatCompletionTokenLogprobSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionTokenLogprobSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionTool
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionTool>; }
    // GatewayInferenceChatCompletionToolChoiceOption
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceChatCompletionToolChoiceOption>; }
    // GatewayInferenceChatCompletionToolChoiceOptionSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionToolChoiceOptionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceChatCompletionToolSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceChatCompletionToolSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceClick
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceClick>; }
    // GatewayInferenceClickSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceClickSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCodeInterpreterFileOutput
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCodeInterpreterFileOutput>; }
    // GatewayInferenceCodeInterpreterFileOutputSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCodeInterpreterFileOutputSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCodeInterpreterTextOutput
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCodeInterpreterTextOutput>; }
    // GatewayInferenceCodeInterpreterTextOutputSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCodeInterpreterTextOutputSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCodeInterpreterToolCall
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCodeInterpreterToolCall>; }
    // GatewayInferenceCodeInterpreterToolCallSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCodeInterpreterToolCallSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCodeInterpreterToolOutput
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCodeInterpreterToolOutput>; }
    // GatewayInferenceCodeInterpreterToolOutputSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCodeInterpreterToolOutputSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceComparisonFilter
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceComparisonFilter>; }
    // GatewayInferenceComparisonFilterSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceComparisonFilterSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCompletionUsage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCompletionUsage>; }
    // GatewayInferenceCompletionUsageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCompletionUsageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCompoundFilter
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCompoundFilter>; }
    // GatewayInferenceCompoundFilterSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCompoundFilterSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceComputerAction
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceComputerAction>; }
    // GatewayInferenceComputerActionSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceComputerActionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceComputerScreenshotImage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceComputerScreenshotImage>; }
    // GatewayInferenceComputerScreenshotImageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceComputerScreenshotImageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceComputerTool
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceComputerTool>; }
    // GatewayInferenceComputerToolCall
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceComputerToolCall>; }
    // GatewayInferenceComputerToolCallOutputResource
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceComputerToolCallOutputResource>; }
    // GatewayInferenceComputerToolCallOutputResourceSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceComputerToolCallOutputResourceSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceComputerToolCallSafetyCheck
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceComputerToolCallSafetyCheck>; }
    // GatewayInferenceComputerToolCallSafetyCheckSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceComputerToolCallSafetyCheckSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceComputerToolCallSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceComputerToolCallSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceComputerToolSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceComputerToolSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCoordinate
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCoordinate>; }
    // GatewayInferenceCoordinateSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCoordinateSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCreateChatCompletionRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCreateChatCompletionRequest>; }
    // GatewayInferenceCreateChatCompletionRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCreateChatCompletionRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCreateChatCompletionResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCreateChatCompletionResponse>; }
    // GatewayInferenceCreateChatCompletionResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCreateChatCompletionResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCreateChatCompletionStreamResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCreateChatCompletionStreamResponse>; }
    // GatewayInferenceCreateChatCompletionStreamResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCreateChatCompletionStreamResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCreateCompletionRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCreateCompletionRequest>; }
    // GatewayInferenceCreateCompletionRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCreateCompletionRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCreateCompletionResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCreateCompletionResponse>; }
    // GatewayInferenceCreateCompletionResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCreateCompletionResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCreateEmbeddingResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCreateEmbeddingResponse>; }
    // GatewayInferenceCreateEmbeddingResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCreateEmbeddingResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCreateModerationResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCreateModerationResponse>; }
    // GatewayInferenceCreateModerationResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCreateModerationResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCreateOcrResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCreateOcrResponse>; }
    // GatewayInferenceCreateOcrResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCreateOcrResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCreatePromptCompletionResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCreatePromptCompletionResponse>; }
    // GatewayInferenceCreatePromptCompletionResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCreatePromptCompletionResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCreatePromptCompletionStreamResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCreatePromptCompletionStreamResponse>; }
    // GatewayInferenceCreatePromptCompletionStreamResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCreatePromptCompletionStreamResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCreatePromptRenderResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCreatePromptRenderResponse>; }
    // GatewayInferenceCreatePromptRenderResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCreatePromptRenderResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCreateRerankResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCreateRerankResponse>; }
    // GatewayInferenceCreateRerankResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCreateRerankResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCreateTranscriptionResponseJson
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCreateTranscriptionResponseJson>; }
    // GatewayInferenceCreateTranscriptionResponseJsonSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCreateTranscriptionResponseJsonSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCreateTranscriptionResponseVerboseJson
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCreateTranscriptionResponseVerboseJson>; }
    // GatewayInferenceCreateTranscriptionResponseVerboseJsonSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCreateTranscriptionResponseVerboseJsonSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCreateTranslationResponseJson
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCreateTranslationResponseJson>; }
    // GatewayInferenceCreateTranslationResponseJsonSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCreateTranslationResponseJsonSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceCreateTranslationResponseVerboseJson
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceCreateTranslationResponseVerboseJson>; }
    // GatewayInferenceCreateTranslationResponseVerboseJsonSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceCreateTranslationResponseVerboseJsonSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceDeleteFileResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceDeleteFileResponse>; }
    // GatewayInferenceDeleteFileResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceDeleteFileResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceDeleteModelResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceDeleteModelResponse>; }
    // GatewayInferenceDeleteModelResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceDeleteModelResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceDeleteVectorStoreFileResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceDeleteVectorStoreFileResponse>; }
    // GatewayInferenceDeleteVectorStoreFileResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceDeleteVectorStoreFileResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceDeleteVectorStoreResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceDeleteVectorStoreResponse>; }
    // GatewayInferenceDeleteVectorStoreResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceDeleteVectorStoreResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceDoubleClick
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceDoubleClick>; }
    // GatewayInferenceDoubleClickSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceDoubleClickSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceDrag
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceDrag>; }
    // GatewayInferenceDragSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceDragSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceEmbedding
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceEmbedding>; }
    // GatewayInferenceEmbeddingSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceEmbeddingSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceFeedbackResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceFeedbackResponse>; }
    // GatewayInferenceFeedbackResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceFeedbackResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceFileCitation
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceFileCitation>; }
    // GatewayInferenceFileCitationSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceFileCitationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceFilePath
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceFilePath>; }
    // GatewayInferenceFilePathSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceFilePathSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceFileSearchTool
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceFileSearchTool>; }
    // GatewayInferenceFileSearchToolCall
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceFileSearchToolCall>; }
    // GatewayInferenceFileSearchToolCallSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceFileSearchToolCallSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceFileSearchToolSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceFileSearchToolSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceFineTuningIntegration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceFineTuningIntegration>; }
    // GatewayInferenceFineTuningIntegrationSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceFineTuningIntegrationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceFineTuningJob
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceFineTuningJob>; }
    // GatewayInferenceFineTuningJobCheckpoint
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceFineTuningJobCheckpoint>; }
    // GatewayInferenceFineTuningJobCheckpointSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceFineTuningJobCheckpointSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceFineTuningJobEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceFineTuningJobEvent>; }
    // GatewayInferenceFineTuningJobEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceFineTuningJobEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceFineTuningJobSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceFineTuningJobSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceFunctionObject
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceFunctionObject>; }
    // GatewayInferenceFunctionObjectSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceFunctionObjectSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceFunctionParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceFunctionParameters>; }
    // GatewayInferenceFunctionParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceFunctionParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceFunctionTool
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceFunctionTool>; }
    // GatewayInferenceFunctionToolCall
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceFunctionToolCall>; }
    // GatewayInferenceFunctionToolCallOutputResource
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceFunctionToolCallOutputResource>; }
    // GatewayInferenceFunctionToolCallOutputResourceSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceFunctionToolCallOutputResourceSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceFunctionToolCallResource
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceFunctionToolCallResource>; }
    // GatewayInferenceFunctionToolCallResourceSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceFunctionToolCallResourceSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceFunctionToolCallSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceFunctionToolCallSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceFunctionToolSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceFunctionToolSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceImage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceImage>; }
    // GatewayInferenceImageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceImageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceImagesResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceImagesResponse>; }
    // GatewayInferenceImagesResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceImagesResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputAnnotation
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputAnnotation>; }
    // GatewayInferenceInputAnnotationSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputAnnotationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputAutoChunkingStrategyRequestParam
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputAutoChunkingStrategyRequestParam>; }
    // GatewayInferenceInputAutoChunkingStrategyRequestParamSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputAutoChunkingStrategyRequestParamSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputBedrockBatchJob
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputBedrockBatchJob>; }
    // GatewayInferenceInputBedrockBatchJobSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputBedrockBatchJobSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputBedrockFinetuneJob
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputBedrockFinetuneJob>; }
    // GatewayInferenceInputBedrockFinetuneJobSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputBedrockFinetuneJobSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputChatCompletionFunctionCallOption
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputChatCompletionFunctionCallOption>; }
    // GatewayInferenceInputChatCompletionFunctionCallOptionSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputChatCompletionFunctionCallOptionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputChatCompletionFunctions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputChatCompletionFunctions>; }
    // GatewayInferenceInputChatCompletionFunctionsSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputChatCompletionFunctionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputChatCompletionMessageToolCall
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputChatCompletionMessageToolCall>; }
    // GatewayInferenceInputChatCompletionMessageToolCalls
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputChatCompletionMessageToolCalls>; }
    // GatewayInferenceInputChatCompletionMessageToolCallSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputChatCompletionMessageToolCallSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputChatCompletionMessageToolCallsSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputChatCompletionMessageToolCallsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputChatCompletionNamedToolChoice
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputChatCompletionNamedToolChoice>; }
    // GatewayInferenceInputChatCompletionNamedToolChoiceSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputChatCompletionNamedToolChoiceSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputChatCompletionRequestAssistantMessage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputChatCompletionRequestAssistantMessage>; }
    // GatewayInferenceInputChatCompletionRequestAssistantMessageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputChatCompletionRequestAssistantMessageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputChatCompletionRequestDeveloperMessage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputChatCompletionRequestDeveloperMessage>; }
    // GatewayInferenceInputChatCompletionRequestDeveloperMessageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputChatCompletionRequestDeveloperMessageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputChatCompletionRequestFunctionMessage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputChatCompletionRequestFunctionMessage>; }
    // GatewayInferenceInputChatCompletionRequestFunctionMessageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputChatCompletionRequestFunctionMessageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputChatCompletionRequestMessage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputChatCompletionRequestMessage>; }
    // GatewayInferenceInputChatCompletionRequestMessageContentPart
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputChatCompletionRequestMessageContentPart>; }
    // GatewayInferenceInputChatCompletionRequestMessageContentPartImage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputChatCompletionRequestMessageContentPartImage>; }
    // GatewayInferenceInputChatCompletionRequestMessageContentPartImageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputChatCompletionRequestMessageContentPartImageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputChatCompletionRequestMessageContentPartSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputChatCompletionRequestMessageContentPartSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputChatCompletionRequestMessageContentPartText
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputChatCompletionRequestMessageContentPartText>; }
    // GatewayInferenceInputChatCompletionRequestMessageContentPartTextSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputChatCompletionRequestMessageContentPartTextSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputChatCompletionRequestMessageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputChatCompletionRequestMessageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputChatCompletionRequestSystemMessage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputChatCompletionRequestSystemMessage>; }
    // GatewayInferenceInputChatCompletionRequestSystemMessageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputChatCompletionRequestSystemMessageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputChatCompletionRequestToolMessage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputChatCompletionRequestToolMessage>; }
    // GatewayInferenceInputChatCompletionRequestToolMessageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputChatCompletionRequestToolMessageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputChatCompletionRequestUserMessage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputChatCompletionRequestUserMessage>; }
    // GatewayInferenceInputChatCompletionRequestUserMessageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputChatCompletionRequestUserMessageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputChatCompletionStreamOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputChatCompletionStreamOptions>; }
    // GatewayInferenceInputChatCompletionStreamOptionsSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputChatCompletionStreamOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputChatCompletionTool
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputChatCompletionTool>; }
    // GatewayInferenceInputChatCompletionToolChoiceOption
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputChatCompletionToolChoiceOption>; }
    // GatewayInferenceInputChatCompletionToolChoiceOptionSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputChatCompletionToolChoiceOptionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputChatCompletionToolSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputChatCompletionToolSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputChunkingStrategyRequestParam
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputChunkingStrategyRequestParam>; }
    // GatewayInferenceInputChunkingStrategyRequestParamSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputChunkingStrategyRequestParamSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputClick
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputClick>; }
    // GatewayInferenceInputClickSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputClickSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputComparisonFilter
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputComparisonFilter>; }
    // GatewayInferenceInputComparisonFilterSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputComparisonFilterSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCompoundFilter
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCompoundFilter>; }
    // GatewayInferenceInputCompoundFilterSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCompoundFilterSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputComputerAction
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputComputerAction>; }
    // GatewayInferenceInputComputerActionSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputComputerActionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputComputerScreenshotImage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputComputerScreenshotImage>; }
    // GatewayInferenceInputComputerScreenshotImageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputComputerScreenshotImageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputComputerTool
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputComputerTool>; }
    // GatewayInferenceInputComputerToolCall
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputComputerToolCall>; }
    // GatewayInferenceInputComputerToolCallOutput
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputComputerToolCallOutput>; }
    // GatewayInferenceInputComputerToolCallOutputSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputComputerToolCallOutputSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputComputerToolCallSafetyCheck
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputComputerToolCallSafetyCheck>; }
    // GatewayInferenceInputComputerToolCallSafetyCheckSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputComputerToolCallSafetyCheckSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputComputerToolCallSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputComputerToolCallSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputComputerToolSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputComputerToolSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputContent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputContent>; }
    // GatewayInferenceInputContentSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputContentSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCoordinate
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCoordinate>; }
    // GatewayInferenceInputCoordinateSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCoordinateSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCreateBatchRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateBatchRequest>; }
    // GatewayInferenceInputCreateBatchRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateBatchRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCreateChatCompletionRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateChatCompletionRequest>; }
    // GatewayInferenceInputCreateChatCompletionRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateChatCompletionRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCreateCompletionRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateCompletionRequest>; }
    // GatewayInferenceInputCreateCompletionRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateCompletionRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCreateEmbeddingRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateEmbeddingRequest>; }
    // GatewayInferenceInputCreateEmbeddingRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateEmbeddingRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCreateFileRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateFileRequest>; }
    // GatewayInferenceInputCreateFileRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateFileRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); let _ = scm_sdk::validation::validate_binary; }
    // GatewayInferenceInputCreateFineTuningJobRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateFineTuningJobRequest>; }
    // GatewayInferenceInputCreateFineTuningJobRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateFineTuningJobRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCreateImageEditRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateImageEditRequest>; }
    // GatewayInferenceInputCreateImageEditRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateImageEditRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); let _ = scm_sdk::validation::validate_binary; }
    // GatewayInferenceInputCreateImageRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateImageRequest>; }
    // GatewayInferenceInputCreateImageRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateImageRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCreateImageVariationRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateImageVariationRequest>; }
    // GatewayInferenceInputCreateImageVariationRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateImageVariationRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); let _ = scm_sdk::validation::validate_binary; }
    // GatewayInferenceInputCreateLogsRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateLogsRequest>; }
    // GatewayInferenceInputCreateLogsRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateLogsRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCreateModerationRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateModerationRequest>; }
    // GatewayInferenceInputCreateModerationRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateModerationRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCreateOcrRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateOcrRequest>; }
    // GatewayInferenceInputCreateOcrRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateOcrRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCreatePromptCompletionRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreatePromptCompletionRequest>; }
    // GatewayInferenceInputCreatePromptCompletionRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreatePromptCompletionRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCreatePromptRenderRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreatePromptRenderRequest>; }
    // GatewayInferenceInputCreatePromptRenderRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreatePromptRenderRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCreateRerankRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateRerankRequest>; }
    // GatewayInferenceInputCreateRerankRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateRerankRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCreateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateResponse>; }
    // GatewayInferenceInputCreateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCreateSpeechRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateSpeechRequest>; }
    // GatewayInferenceInputCreateSpeechRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateSpeechRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCreateTranscriptionRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateTranscriptionRequest>; }
    // GatewayInferenceInputCreateTranscriptionRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateTranscriptionRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); let _ = scm_sdk::validation::validate_binary; }
    // GatewayInferenceInputCreateTranslationRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateTranslationRequest>; }
    // GatewayInferenceInputCreateTranslationRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateTranslationRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); let _ = scm_sdk::validation::validate_binary; }
    // GatewayInferenceInputCreateVectorStoreFileBatchRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateVectorStoreFileBatchRequest>; }
    // GatewayInferenceInputCreateVectorStoreFileBatchRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateVectorStoreFileBatchRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCreateVectorStoreFileRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateVectorStoreFileRequest>; }
    // GatewayInferenceInputCreateVectorStoreFileRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateVectorStoreFileRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCreateVectorStoreRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCreateVectorStoreRequest>; }
    // GatewayInferenceInputCreateVectorStoreRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCreateVectorStoreRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputCustomLog
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputCustomLog>; }
    // GatewayInferenceInputCustomLogSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputCustomLogSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputDoubleClick
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputDoubleClick>; }
    // GatewayInferenceInputDoubleClickSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputDoubleClickSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputDrag
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputDrag>; }
    // GatewayInferenceInputDragSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputDragSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputEasyInputMessage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputEasyInputMessage>; }
    // GatewayInferenceInputEasyInputMessageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputEasyInputMessageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputFeedbackRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputFeedbackRequest>; }
    // GatewayInferenceInputFeedbackRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputFeedbackRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputFeedbackUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputFeedbackUpdateRequest>; }
    // GatewayInferenceInputFeedbackUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputFeedbackUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputFile
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputFile>; }
    // GatewayInferenceInputFileCitation
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputFileCitation>; }
    // GatewayInferenceInputFileCitationSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputFileCitationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputFilePath
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputFilePath>; }
    // GatewayInferenceInputFilePathSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputFilePathSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputFileSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputFileSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputFileSearchTool
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputFileSearchTool>; }
    // GatewayInferenceInputFileSearchToolCall
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputFileSearchToolCall>; }
    // GatewayInferenceInputFileSearchToolCallSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputFileSearchToolCallSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputFileSearchToolSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputFileSearchToolSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputFunctionObject
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputFunctionObject>; }
    // GatewayInferenceInputFunctionObjectSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputFunctionObjectSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputFunctionParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputFunctionParameters>; }
    // GatewayInferenceInputFunctionParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputFunctionParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputFunctionTool
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputFunctionTool>; }
    // GatewayInferenceInputFunctionToolCall
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputFunctionToolCall>; }
    // GatewayInferenceInputFunctionToolCallOutput
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputFunctionToolCallOutput>; }
    // GatewayInferenceInputFunctionToolCallOutputSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputFunctionToolCallOutputSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputFunctionToolCallSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputFunctionToolCallSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputFunctionToolSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputFunctionToolSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputGetLogQuery
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputGetLogQuery>; }
    // GatewayInferenceInputGetLogQuerySchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputGetLogQuerySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputGetResponseQuery
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputGetResponseQuery>; }
    // GatewayInferenceInputGetResponseQuerySchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputGetResponseQuerySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputImage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputImage>; }
    // GatewayInferenceInputImageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputImageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputIncludable
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputIncludable>; }
    // GatewayInferenceInputIncludableSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputIncludableSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputInputContent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputInputContent>; }
    // GatewayInferenceInputInputContentSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputInputContentSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputInputFile
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputInputFile>; }
    // GatewayInferenceInputInputFileSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputInputFileSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputInputImage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputInputImage>; }
    // GatewayInferenceInputInputImageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputInputImageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputInputItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputInputItem>; }
    // GatewayInferenceInputInputItemSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputInputItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputInputMessage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputInputMessage>; }
    // GatewayInferenceInputInputMessageContentList
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputInputMessageContentList>; }
    // GatewayInferenceInputInputMessageContentListSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputInputMessageContentListSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputInputMessageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputInputMessageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputInputText
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputInputText>; }
    // GatewayInferenceInputInputTextSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputInputTextSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputItem>; }
    // GatewayInferenceInputItemReference
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputItemReference>; }
    // GatewayInferenceInputItemReferenceSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputItemReferenceSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputItemSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputKeyPress
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputKeyPress>; }
    // GatewayInferenceInputKeyPressSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputKeyPressSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputListBatchesQuery
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputListBatchesQuery>; }
    // GatewayInferenceInputListBatchesQuerySchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputListBatchesQuerySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputListFilesInVectorStoreBatchQuery
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputListFilesInVectorStoreBatchQuery>; }
    // GatewayInferenceInputListFilesInVectorStoreBatchQuerySchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputListFilesInVectorStoreBatchQuerySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputListFilesQuery
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputListFilesQuery>; }
    // GatewayInferenceInputListFilesQuerySchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputListFilesQuerySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputListFineTuningEventsQuery
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputListFineTuningEventsQuery>; }
    // GatewayInferenceInputListFineTuningEventsQuerySchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputListFineTuningEventsQuerySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputListFineTuningJobCheckpointsQuery
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputListFineTuningJobCheckpointsQuery>; }
    // GatewayInferenceInputListFineTuningJobCheckpointsQuerySchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputListFineTuningJobCheckpointsQuerySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputListInputItemsQuery
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputListInputItemsQuery>; }
    // GatewayInferenceInputListInputItemsQuerySchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputListInputItemsQuerySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputListModelsQuery
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputListModelsQuery>; }
    // GatewayInferenceInputListModelsQuerySchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputListModelsQuerySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputListPaginatedFineTuningJobsQuery
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputListPaginatedFineTuningJobsQuery>; }
    // GatewayInferenceInputListPaginatedFineTuningJobsQuerySchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputListPaginatedFineTuningJobsQuerySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputListVectorStoreFilesQuery
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputListVectorStoreFilesQuery>; }
    // GatewayInferenceInputListVectorStoreFilesQuerySchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputListVectorStoreFilesQuerySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputListVectorStoresQuery
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputListVectorStoresQuery>; }
    // GatewayInferenceInputListVectorStoresQuerySchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputListVectorStoresQuerySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputMessageContentList
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputMessageContentList>; }
    // GatewayInferenceInputMessageContentListSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputMessageContentListSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputMessageResource
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputMessageResource>; }
    // GatewayInferenceInputMessageResourceSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputMessageResourceSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputMetadata
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputMetadata>; }
    // GatewayInferenceInputMetadataSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputMetadataSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputModelIdsResponses
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputModelIdsResponses>; }
    // GatewayInferenceInputModelIdsResponsesSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputModelIdsResponsesSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputMove
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputMove>; }
    // GatewayInferenceInputMoveSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputMoveSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputOpenAIBatchJob
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputOpenAIBatchJob>; }
    // GatewayInferenceInputOpenAIBatchJobSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputOpenAIBatchJobSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputOpenAIFinetuneJob
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputOpenAIFinetuneJob>; }
    // GatewayInferenceInputOpenAIFinetuneJobSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputOpenAIFinetuneJobSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputOutputContent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputOutputContent>; }
    // GatewayInferenceInputOutputContentSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputOutputContentSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputOutputMessage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputOutputMessage>; }
    // GatewayInferenceInputOutputMessageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputOutputMessageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputOutputText
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputOutputText>; }
    // GatewayInferenceInputOutputTextSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputOutputTextSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputParallelToolCalls
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputParallelToolCalls>; }
    // GatewayInferenceInputParallelToolCallsSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputParallelToolCallsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputPortkeyBatchJob
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputPortkeyBatchJob>; }
    // GatewayInferenceInputPortkeyBatchJobSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputPortkeyBatchJobSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputPortkeyFinetuneJob
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputPortkeyFinetuneJob>; }
    // GatewayInferenceInputPortkeyFinetuneJobSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputPortkeyFinetuneJobSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputReasoning
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputReasoning>; }
    // GatewayInferenceInputReasoningEffort
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputReasoningEffort>; }
    // GatewayInferenceInputReasoningEffortSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputReasoningEffortSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputReasoningItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputReasoningItem>; }
    // GatewayInferenceInputReasoningItemSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputReasoningItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputReasoningSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputReasoningSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputRefusal
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputRefusal>; }
    // GatewayInferenceInputRefusalSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputRefusalSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputRerankDocument
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputRerankDocument>; }
    // GatewayInferenceInputRerankDocumentSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputRerankDocumentSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputResponseFormatJsonObject
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputResponseFormatJsonObject>; }
    // GatewayInferenceInputResponseFormatJsonObjectSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputResponseFormatJsonObjectSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputResponseFormatJsonSchema
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputResponseFormatJsonSchema>; }
    // GatewayInferenceInputResponseFormatJsonSchemaSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputResponseFormatJsonSchemaSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputResponseFormatJsonSchemaSchemaSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputResponseFormatJsonSchemaSchemaSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputResponseFormatText
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputResponseFormatText>; }
    // GatewayInferenceInputResponseFormatTextSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputResponseFormatTextSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputScreenshot
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputScreenshot>; }
    // GatewayInferenceInputScreenshotSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputScreenshotSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputScroll
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputScroll>; }
    // GatewayInferenceInputScrollSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputScrollSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputStaticChunkingStrategy
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputStaticChunkingStrategy>; }
    // GatewayInferenceInputStaticChunkingStrategyRequestParam
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputStaticChunkingStrategyRequestParam>; }
    // GatewayInferenceInputStaticChunkingStrategyRequestParamSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputStaticChunkingStrategyRequestParamSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputStaticChunkingStrategySchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputStaticChunkingStrategySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputText
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputText>; }
    // GatewayInferenceInputTextResponseFormatConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputTextResponseFormatConfiguration>; }
    // GatewayInferenceInputTextResponseFormatConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputTextResponseFormatConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputTextResponseFormatJsonSchema
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputTextResponseFormatJsonSchema>; }
    // GatewayInferenceInputTextResponseFormatJsonSchemaSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputTextResponseFormatJsonSchemaSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputTextSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputTextSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputTool
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputTool>; }
    // GatewayInferenceInputToolChoiceFunction
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputToolChoiceFunction>; }
    // GatewayInferenceInputToolChoiceFunctionSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputToolChoiceFunctionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputToolChoiceOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputToolChoiceOptions>; }
    // GatewayInferenceInputToolChoiceOptionsSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputToolChoiceOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputToolChoiceTypes
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputToolChoiceTypes>; }
    // GatewayInferenceInputToolChoiceTypesSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputToolChoiceTypesSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputToolSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputToolSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputType>; }
    // GatewayInferenceInputTypeSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputTypeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputUpdateVectorStoreRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputUpdateVectorStoreRequest>; }
    // GatewayInferenceInputUpdateVectorStoreRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputUpdateVectorStoreRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputUrlCitation
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputUrlCitation>; }
    // GatewayInferenceInputUrlCitationSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputUrlCitationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputVectorStoreExpirationAfter
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputVectorStoreExpirationAfter>; }
    // GatewayInferenceInputVectorStoreExpirationAfterSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputVectorStoreExpirationAfterSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputVectorStoreFileAttributes
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputVectorStoreFileAttributes>; }
    // GatewayInferenceInputVectorStoreFileAttributesSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputVectorStoreFileAttributesSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputVertexBatchJob
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputVertexBatchJob>; }
    // GatewayInferenceInputVertexBatchJobSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputVertexBatchJobSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputWait
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputWait>; }
    // GatewayInferenceInputWaitSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputWaitSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputWebSearchContextSize
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputWebSearchContextSize>; }
    // GatewayInferenceInputWebSearchContextSizeSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputWebSearchContextSizeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputWebSearchTool
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputWebSearchTool>; }
    // GatewayInferenceInputWebSearchToolCall
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceInputWebSearchToolCall>; }
    // GatewayInferenceInputWebSearchToolCallSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputWebSearchToolCallSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceInputWebSearchToolSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceInputWebSearchToolSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceItemResource
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceItemResource>; }
    // GatewayInferenceItemResourceSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceItemResourceSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceKeyPress
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceKeyPress>; }
    // GatewayInferenceKeyPressSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceKeyPressSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceListBatchesResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceListBatchesResponse>; }
    // GatewayInferenceListBatchesResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceListBatchesResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceListFilesResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceListFilesResponse>; }
    // GatewayInferenceListFilesResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceListFilesResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceListFineTuningJobCheckpointsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceListFineTuningJobCheckpointsResponse>; }
    // GatewayInferenceListFineTuningJobCheckpointsResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceListFineTuningJobCheckpointsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceListFineTuningJobEventsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceListFineTuningJobEventsResponse>; }
    // GatewayInferenceListFineTuningJobEventsResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceListFineTuningJobEventsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceListModelsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceListModelsResponse>; }
    // GatewayInferenceListModelsResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceListModelsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceListPaginatedFineTuningJobsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceListPaginatedFineTuningJobsResponse>; }
    // GatewayInferenceListPaginatedFineTuningJobsResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceListPaginatedFineTuningJobsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceListVectorStoreFilesResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceListVectorStoreFilesResponse>; }
    // GatewayInferenceListVectorStoreFilesResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceListVectorStoreFilesResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceListVectorStoresResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceListVectorStoresResponse>; }
    // GatewayInferenceListVectorStoresResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceListVectorStoresResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceLogObject
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceLogObject>; }
    // GatewayInferenceLogObjectSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceLogObjectSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceLogRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceLogRequest>; }
    // GatewayInferenceLogRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceLogRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceLogResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceLogResponse>; }
    // GatewayInferenceLogResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceLogResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceMetadata
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceMetadata>; }
    // GatewayInferenceMetadataSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceMetadataSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceModel
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceModel>; }
    // GatewayInferenceModelIdsResponses
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceModelIdsResponses>; }
    // GatewayInferenceModelIdsResponsesSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceModelIdsResponsesSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceModelSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceModelSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceMove
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceMove>; }
    // GatewayInferenceMoveSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceMoveSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceOcrPage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceOcrPage>; }
    // GatewayInferenceOcrPageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceOcrPageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceOpenAIFile
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceOpenAIFile>; }
    // GatewayInferenceOpenAIFileSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceOpenAIFileSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceOtherChunkingStrategyResponseParam
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceOtherChunkingStrategyResponseParam>; }
    // GatewayInferenceOtherChunkingStrategyResponseParamSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceOtherChunkingStrategyResponseParamSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceOutputContent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceOutputContent>; }
    // GatewayInferenceOutputContentSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceOutputContentSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceOutputItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceOutputItem>; }
    // GatewayInferenceOutputItemSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceOutputItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceOutputMessage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceOutputMessage>; }
    // GatewayInferenceOutputMessageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceOutputMessageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceOutputText
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceOutputText>; }
    // GatewayInferenceOutputTextSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceOutputTextSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceParallelToolCalls
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceParallelToolCalls>; }
    // GatewayInferenceParallelToolCallsSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceParallelToolCallsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferencePromptRenderResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferencePromptRenderResponse>; }
    // GatewayInferencePromptRenderResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferencePromptRenderResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceReasoning
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceReasoning>; }
    // GatewayInferenceReasoningEffort
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceReasoningEffort>; }
    // GatewayInferenceReasoningEffortSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceReasoningEffortSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceReasoningItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceReasoningItem>; }
    // GatewayInferenceReasoningItemSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceReasoningItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceReasoningSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceReasoningSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceRefusal
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceRefusal>; }
    // GatewayInferenceRefusalSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceRefusalSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceRequestOptions
    { let _ = std::marker::PhantomData::<scm_sdk::GatewayRouting>; }
    // GatewayInferenceRequestOptions.headers
    { let _ = scm_sdk::airs::gateway::RuntimeClient::with_routing; }
    // GatewayInferenceRequestOptions.numRetries
    { let _ = scm_sdk::airs::gateway::RuntimeClient::with_replay; }
    // GatewayInferenceRequestOptions.signal
    { let _ = scm_sdk::RequestOptions::cancel; }
    // GatewayInferenceRequestOptions.timeoutMs
    { let _ = scm_sdk::RequestOptions::with_timeout; }
    // GatewayInferenceRequestResponseObject
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceRequestResponseObject>; }
    // GatewayInferenceRequestResponseObjectSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceRequestResponseObjectSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceRerankResult
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceRerankResult>; }
    // GatewayInferenceRerankResultSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceRerankResultSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceRerankUsage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceRerankUsage>; }
    // GatewayInferenceRerankUsageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceRerankUsageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponse>; }
    // GatewayInferenceResponseAudioDeltaEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseAudioDeltaEvent>; }
    // GatewayInferenceResponseAudioDeltaEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseAudioDeltaEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseAudioDoneEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseAudioDoneEvent>; }
    // GatewayInferenceResponseAudioDoneEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseAudioDoneEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseAudioTranscriptDeltaEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseAudioTranscriptDeltaEvent>; }
    // GatewayInferenceResponseAudioTranscriptDeltaEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseAudioTranscriptDeltaEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseAudioTranscriptDoneEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseAudioTranscriptDoneEvent>; }
    // GatewayInferenceResponseAudioTranscriptDoneEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseAudioTranscriptDoneEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseCodeInterpreterCallCodeDeltaEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseCodeInterpreterCallCodeDeltaEvent>; }
    // GatewayInferenceResponseCodeInterpreterCallCodeDeltaEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseCodeInterpreterCallCodeDeltaEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseCodeInterpreterCallCodeDoneEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseCodeInterpreterCallCodeDoneEvent>; }
    // GatewayInferenceResponseCodeInterpreterCallCodeDoneEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseCodeInterpreterCallCodeDoneEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseCodeInterpreterCallCompletedEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseCodeInterpreterCallCompletedEvent>; }
    // GatewayInferenceResponseCodeInterpreterCallCompletedEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseCodeInterpreterCallCompletedEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseCodeInterpreterCallInProgressEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseCodeInterpreterCallInProgressEvent>; }
    // GatewayInferenceResponseCodeInterpreterCallInProgressEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseCodeInterpreterCallInProgressEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseCodeInterpreterCallInterpretingEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseCodeInterpreterCallInterpretingEvent>; }
    // GatewayInferenceResponseCodeInterpreterCallInterpretingEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseCodeInterpreterCallInterpretingEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseCompletedEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseCompletedEvent>; }
    // GatewayInferenceResponseCompletedEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseCompletedEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseContentPartAddedEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseContentPartAddedEvent>; }
    // GatewayInferenceResponseContentPartAddedEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseContentPartAddedEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseContentPartDoneEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseContentPartDoneEvent>; }
    // GatewayInferenceResponseContentPartDoneEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseContentPartDoneEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseCreatedEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseCreatedEvent>; }
    // GatewayInferenceResponseCreatedEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseCreatedEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseError
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseError>; }
    // GatewayInferenceResponseErrorCode
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseErrorCode>; }
    // GatewayInferenceResponseErrorCodeSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseErrorCodeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseErrorEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseErrorEvent>; }
    // GatewayInferenceResponseErrorEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseErrorEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseErrorSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseErrorSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseFailedEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseFailedEvent>; }
    // GatewayInferenceResponseFailedEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseFailedEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseFileSearchCallCompletedEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseFileSearchCallCompletedEvent>; }
    // GatewayInferenceResponseFileSearchCallCompletedEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseFileSearchCallCompletedEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseFileSearchCallInProgressEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseFileSearchCallInProgressEvent>; }
    // GatewayInferenceResponseFileSearchCallInProgressEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseFileSearchCallInProgressEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseFileSearchCallSearchingEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseFileSearchCallSearchingEvent>; }
    // GatewayInferenceResponseFileSearchCallSearchingEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseFileSearchCallSearchingEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseFormatJsonObject
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseFormatJsonObject>; }
    // GatewayInferenceResponseFormatJsonObjectSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseFormatJsonObjectSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseFormatJsonSchema
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseFormatJsonSchema>; }
    // GatewayInferenceResponseFormatJsonSchemaSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseFormatJsonSchemaSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseFormatJsonSchemaSchemaSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseFormatJsonSchemaSchemaSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseFormatText
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseFormatText>; }
    // GatewayInferenceResponseFormatTextSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseFormatTextSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseFunctionCallArgumentsDeltaEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseFunctionCallArgumentsDeltaEvent>; }
    // GatewayInferenceResponseFunctionCallArgumentsDeltaEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseFunctionCallArgumentsDeltaEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseFunctionCallArgumentsDoneEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseFunctionCallArgumentsDoneEvent>; }
    // GatewayInferenceResponseFunctionCallArgumentsDoneEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseFunctionCallArgumentsDoneEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseIncompleteEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseIncompleteEvent>; }
    // GatewayInferenceResponseIncompleteEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseIncompleteEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseInProgressEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseInProgressEvent>; }
    // GatewayInferenceResponseInProgressEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseInProgressEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseItemList
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseItemList>; }
    // GatewayInferenceResponseItemListSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseItemListSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseOutputItemAddedEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseOutputItemAddedEvent>; }
    // GatewayInferenceResponseOutputItemAddedEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseOutputItemAddedEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseOutputItemDoneEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseOutputItemDoneEvent>; }
    // GatewayInferenceResponseOutputItemDoneEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseOutputItemDoneEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseRefusalDeltaEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseRefusalDeltaEvent>; }
    // GatewayInferenceResponseRefusalDeltaEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseRefusalDeltaEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseRefusalDoneEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseRefusalDoneEvent>; }
    // GatewayInferenceResponseRefusalDoneEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseRefusalDoneEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseStreamEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseStreamEvent>; }
    // GatewayInferenceResponseStreamEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseStreamEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseTextAnnotationDeltaEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseTextAnnotationDeltaEvent>; }
    // GatewayInferenceResponseTextAnnotationDeltaEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseTextAnnotationDeltaEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseTextDeltaEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseTextDeltaEvent>; }
    // GatewayInferenceResponseTextDeltaEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseTextDeltaEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseTextDoneEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseTextDoneEvent>; }
    // GatewayInferenceResponseTextDoneEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseTextDoneEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseUsage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseUsage>; }
    // GatewayInferenceResponseUsageSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseUsageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseWebSearchCallCompletedEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseWebSearchCallCompletedEvent>; }
    // GatewayInferenceResponseWebSearchCallCompletedEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseWebSearchCallCompletedEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseWebSearchCallInProgressEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseWebSearchCallInProgressEvent>; }
    // GatewayInferenceResponseWebSearchCallInProgressEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseWebSearchCallInProgressEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceResponseWebSearchCallSearchingEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceResponseWebSearchCallSearchingEvent>; }
    // GatewayInferenceResponseWebSearchCallSearchingEventSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceResponseWebSearchCallSearchingEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceScreenshot
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceScreenshot>; }
    // GatewayInferenceScreenshotSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceScreenshotSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceScroll
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceScroll>; }
    // GatewayInferenceScrollSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceScrollSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceStaticChunkingStrategy
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceStaticChunkingStrategy>; }
    // GatewayInferenceStaticChunkingStrategyResponseParam
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceStaticChunkingStrategyResponseParam>; }
    // GatewayInferenceStaticChunkingStrategyResponseParamSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceStaticChunkingStrategyResponseParamSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceStaticChunkingStrategySchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceStaticChunkingStrategySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceTextResponseFormatConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceTextResponseFormatConfiguration>; }
    // GatewayInferenceTextResponseFormatConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceTextResponseFormatConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceTextResponseFormatJsonSchema
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceTextResponseFormatJsonSchema>; }
    // GatewayInferenceTextResponseFormatJsonSchemaSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceTextResponseFormatJsonSchemaSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceTool
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceTool>; }
    // GatewayInferenceToolChoiceFunction
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceToolChoiceFunction>; }
    // GatewayInferenceToolChoiceFunctionSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceToolChoiceFunctionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceToolChoiceOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceToolChoiceOptions>; }
    // GatewayInferenceToolChoiceOptionsSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceToolChoiceOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceToolChoiceTypes
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceToolChoiceTypes>; }
    // GatewayInferenceToolChoiceTypesSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceToolChoiceTypesSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceToolSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceToolSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceTranscriptionSegment
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceTranscriptionSegment>; }
    // GatewayInferenceTranscriptionSegmentSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceTranscriptionSegmentSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceTranscriptionWord
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceTranscriptionWord>; }
    // GatewayInferenceTranscriptionWordSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceTranscriptionWordSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceType>; }
    // GatewayInferenceTypeSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceTypeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceUrlCitation
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceUrlCitation>; }
    // GatewayInferenceUrlCitationSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceUrlCitationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceVectorStoreExpirationAfter
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceVectorStoreExpirationAfter>; }
    // GatewayInferenceVectorStoreExpirationAfterSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceVectorStoreExpirationAfterSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceVectorStoreFileAttributes
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceVectorStoreFileAttributes>; }
    // GatewayInferenceVectorStoreFileAttributesSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceVectorStoreFileAttributesSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceVectorStoreFileBatchObject
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceVectorStoreFileBatchObject>; }
    // GatewayInferenceVectorStoreFileBatchObjectSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceVectorStoreFileBatchObjectSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceVectorStoreFileObject
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceVectorStoreFileObject>; }
    // GatewayInferenceVectorStoreFileObjectSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceVectorStoreFileObjectSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceVectorStoreObject
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceVectorStoreObject>; }
    // GatewayInferenceVectorStoreObjectSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceVectorStoreObjectSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceWait
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceWait>; }
    // GatewayInferenceWaitSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceWaitSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceWebSearchContextSize
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceWebSearchContextSize>; }
    // GatewayInferenceWebSearchContextSizeSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceWebSearchContextSizeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceWebSearchTool
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceWebSearchTool>; }
    // GatewayInferenceWebSearchToolCall
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayInferenceWebSearchToolCall>; }
    // GatewayInferenceWebSearchToolCallSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceWebSearchToolCallSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayInferenceWebSearchToolSchema
    { let _ = scm_sdk::validation::parse("GatewayInferenceWebSearchToolSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayIntegration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayIntegration>; }
    // GatewayIntegrationConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayIntegrationConfiguration>; }
    // GatewayIntegrationCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayIntegrationCreateRequest>; }
    // GatewayIntegrationCreateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayIntegrationCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayIntegrationModelsBulkUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayIntegrationModelsBulkUpdateRequest>; }
    // GatewayIntegrationModelsBulkUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayIntegrationModelsBulkUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayIntegrationModelsRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayIntegrationModelsRequest>; }
    // GatewayIntegrationModelsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayIntegrationModelsResponse>; }
    // GatewayIntegrationModelsResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayIntegrationModelsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayIntegrationModelUpdate
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayIntegrationModelUpdate>; }
    // GatewayIntegrationModelUpdateSchema
    { let _ = scm_sdk::validation::parse("GatewayIntegrationModelUpdateSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayIntegrationSchema
    { let _ = scm_sdk::validation::parse("GatewayIntegrationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayIntegrationUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayIntegrationUpdateRequest>; }
    // GatewayIntegrationUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayIntegrationUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayIntegrationWorkspace
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayIntegrationWorkspace>; }
    // GatewayIntegrationWorkspacesBulkUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayIntegrationWorkspacesBulkUpdateRequest>; }
    // GatewayIntegrationWorkspacesBulkUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayIntegrationWorkspacesBulkUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayIntegrationWorkspaceSchema
    { let _ = scm_sdk::validation::parse("GatewayIntegrationWorkspaceSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayIntegrationWorkspacesRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayIntegrationWorkspacesRequest>; }
    // GatewayIntegrationWorkspacesResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayIntegrationWorkspacesResponse>; }
    // GatewayIntegrationWorkspacesResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayIntegrationWorkspacesResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayJSONKeysParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayJSONKeysParameters>; }
    // GatewayJSONKeysParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayJSONKeysParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayJsonObject
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayJsonObject>; }
    // GatewayJsonObjectSchema
    { let _ = scm_sdk::validation::parse("GatewayJsonObjectSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayJSONSchemaParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayJSONSchemaParameters>; }
    // GatewayJSONSchemaParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayJSONSchemaParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayJsonValue
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayJsonValue>; }
    // GatewayJsonValueSchema
    { let _ = scm_sdk::validation::parse("GatewayJsonValueSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayJWTParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayJWTParameters>; }
    // GatewayJWTParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayJWTParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayKnownApiKeyScope
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayKnownApiKeyScope>; }
    // GatewayKnownCacheMode
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayKnownCacheMode>; }
    // GatewayKnownConfigStrategy
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayKnownConfigStrategy>; }
    // GatewayKnownMcpAuthType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayKnownMcpAuthType>; }
    // GatewayKnownMcpTransport
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayKnownMcpTransport>; }
    // GatewayKnownRateLimitType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayKnownRateLimitType>; }
    // GatewayKnownRateLimitUnit
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayKnownRateLimitUnit>; }
    // GatewayLogExportsClientCancelResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayLogExportsClientCancelResponse>; }
    // GatewayLogExportsClientCancelResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayLogExportsClientCancelResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayLogExportsClientCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayLogExportsClientCreateRequest>; }
    // GatewayLogExportsClientCreateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayLogExportsClientCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayLogExportsClientCreateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayLogExportsClientCreateResponse>; }
    // GatewayLogExportsClientCreateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayLogExportsClientCreateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayLogExportsClientDownloadResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayLogExportsClientDownloadResponse>; }
    // GatewayLogExportsClientDownloadResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayLogExportsClientDownloadResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayLogExportsClientGetResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayLogExportsClientGetResponse>; }
    // GatewayLogExportsClientGetResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayLogExportsClientGetResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayLogExportsClientListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayLogExportsClientListOptions>; }
    // GatewayLogExportsClientListOptionsSchema
    { let _ = scm_sdk::validation::parse("GatewayLogExportsClientListOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayLogExportsClientListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayLogExportsClientListResponse>; }
    // GatewayLogExportsClientListResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayLogExportsClientListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayLogExportsClientStartResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayLogExportsClientStartResponse>; }
    // GatewayLogExportsClientStartResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayLogExportsClientStartResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayLogExportsClientUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayLogExportsClientUpdateRequest>; }
    // GatewayLogExportsClientUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayLogExportsClientUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayLogExportsClientUpdateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayLogExportsClientUpdateResponse>; }
    // GatewayLogExportsClientUpdateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayLogExportsClientUpdateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayLogExportsRequestedData
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayLogExportsRequestedData>; }
    // GatewayLogExportsRequestedDataSchema
    { let _ = scm_sdk::validation::parse("GatewayLogExportsRequestedDataSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayLogRecord
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayLogRecord>; }
    // GatewayLogRecordSchema
    { let _ = scm_sdk::validation::parse("GatewayLogRecordSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayLogsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayLogsResponse>; }
    // GatewayLogsResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayLogsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpAuthType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpAuthType>; }
    // GatewayMcpAuthTypeSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpAuthTypeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpIntegrationWorkspaceItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpIntegrationWorkspaceItem>; }
    // GatewayMcpIntegrationWorkspaceItemSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpIntegrationWorkspaceItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpIntegrationWorkspacesLegacyResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpIntegrationWorkspacesLegacyResponse>; }
    // GatewayMcpIntegrationWorkspacesLegacyResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpIntegrationWorkspacesLegacyResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpIntegrationWorkspacesListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpIntegrationWorkspacesListResponse>; }
    // GatewayMcpIntegrationWorkspacesListResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpIntegrationWorkspacesListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServer
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServer>; }
    // GatewayMcpServerCapabilitiesBulkUpdateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServerCapabilitiesBulkUpdateResponse>; }
    // GatewayMcpServerCapabilitiesBulkUpdateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServerCapabilitiesBulkUpdateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServerCapabilitiesCounts
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServerCapabilitiesCounts>; }
    // GatewayMcpServerCapabilitiesCountsSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServerCapabilitiesCountsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServerCapabilitiesListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServerCapabilitiesListResponse>; }
    // GatewayMcpServerCapabilitiesListResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServerCapabilitiesListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServerCapabilityItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServerCapabilityItem>; }
    // GatewayMcpServerCapabilityItemSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServerCapabilityItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServerConnectionDeleteResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServerConnectionDeleteResponse>; }
    // GatewayMcpServerConnectionDeleteResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServerConnectionDeleteResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServerConnectionItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServerConnectionItem>; }
    // GatewayMcpServerConnectionItemSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServerConnectionItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServerConnectionsListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServerConnectionsListResponse>; }
    // GatewayMcpServerConnectionsListResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServerConnectionsListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServerCreateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServerCreateResponse>; }
    // GatewayMcpServerCreateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServerCreateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServerListItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServerListItem>; }
    // GatewayMcpServerListItemSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServerListItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServerListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServerListResponse>; }
    // GatewayMcpServerListResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServerListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServerMapping
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServerMapping>; }
    // GatewayMcpServerMappingSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServerMappingSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServerMappingsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServerMappingsResponse>; }
    // GatewayMcpServerMappingsResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServerMappingsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServerSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServerSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientCreateRequest>; }
    // GatewayMcpServersClientCreateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientCreateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientCreateResponse>; }
    // GatewayMcpServersClientCreateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientCreateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientDeleteConnectionsOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientDeleteConnectionsOptions>; }
    // GatewayMcpServersClientDeleteConnectionsOptionsSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientDeleteConnectionsOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientDeleteConnectionsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientDeleteConnectionsResponse>; }
    // GatewayMcpServersClientDeleteConnectionsResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientDeleteConnectionsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientDeleteResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientDeleteResponse>; }
    // GatewayMcpServersClientDeleteResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientDeleteResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientGetCapabilitiesOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientGetCapabilitiesOptions>; }
    // GatewayMcpServersClientGetCapabilitiesOptionsSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientGetCapabilitiesOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientGetCapabilitiesResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientGetCapabilitiesResponse>; }
    // GatewayMcpServersClientGetCapabilitiesResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientGetCapabilitiesResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientGetConnectionsOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientGetConnectionsOptions>; }
    // GatewayMcpServersClientGetConnectionsOptionsSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientGetConnectionsOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientGetConnectionsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientGetConnectionsResponse>; }
    // GatewayMcpServersClientGetConnectionsResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientGetConnectionsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientGetResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientGetResponse>; }
    // GatewayMcpServersClientGetResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientGetResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientGetUserAccessOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientGetUserAccessOptions>; }
    // GatewayMcpServersClientGetUserAccessOptionsSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientGetUserAccessOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientGetUserAccessResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientGetUserAccessResponse>; }
    // GatewayMcpServersClientGetUserAccessResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientGetUserAccessResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientListOptions>; }
    // GatewayMcpServersClientListOptionsSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientListOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientListResponse>; }
    // GatewayMcpServersClientListResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientTestResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientTestResponse>; }
    // GatewayMcpServersClientTestResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientTestResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientUpdateCapabilitiesRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientUpdateCapabilitiesRequest>; }
    // GatewayMcpServersClientUpdateCapabilitiesRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientUpdateCapabilitiesRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientUpdateCapabilitiesResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientUpdateCapabilitiesResponse>; }
    // GatewayMcpServersClientUpdateCapabilitiesResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientUpdateCapabilitiesResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientUpdateRequest>; }
    // GatewayMcpServersClientUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientUpdateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientUpdateResponse>; }
    // GatewayMcpServersClientUpdateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientUpdateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientUpdateUserAccessRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientUpdateUserAccessRequest>; }
    // GatewayMcpServersClientUpdateUserAccessRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientUpdateUserAccessRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServersClientUpdateUserAccessResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServersClientUpdateUserAccessResponse>; }
    // GatewayMcpServersClientUpdateUserAccessResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServersClientUpdateUserAccessResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServerTestResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServerTestResponse>; }
    // GatewayMcpServerTestResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServerTestResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServerUserAccessBulkUpdateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServerUserAccessBulkUpdateResponse>; }
    // GatewayMcpServerUserAccessBulkUpdateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServerUserAccessBulkUpdateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServerUserAccessItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServerUserAccessItem>; }
    // GatewayMcpServerUserAccessItemSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServerUserAccessItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpServerUserAccessListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpServerUserAccessListResponse>; }
    // GatewayMcpServerUserAccessListResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpServerUserAccessListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMcpTransport
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMcpTransport>; }
    // GatewayMcpTransportSchema
    { let _ = scm_sdk::validation::parse("GatewayMcpTransportSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMistralModerationParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMistralModerationParameters>; }
    // GatewayMistralModerationParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayMistralModerationParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayModelCalculateConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayModelCalculateConfig>; }
    // GatewayModelCalculateConfigSchema
    { let _ = scm_sdk::validation::parse("GatewayModelCalculateConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayModelFinetuneConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayModelFinetuneConfig>; }
    // GatewayModelFinetuneConfigSchema
    { let _ = scm_sdk::validation::parse("GatewayModelFinetuneConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayModelImagePricing
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayModelImagePricing>; }
    // GatewayModelImagePricingSchema
    { let _ = scm_sdk::validation::parse("GatewayModelImagePricingSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayModelPayAsYouGo
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayModelPayAsYouGo>; }
    // GatewayModelPayAsYouGoSchema
    { let _ = scm_sdk::validation::parse("GatewayModelPayAsYouGoSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayModelPricingCalculation
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayModelPricingCalculation>; }
    // GatewayModelPricingCalculation.operands
    { let _ = |value: &scm_sdk::airs::gateway::models::GatewayModelPricingCalculation| { let _ = &value.operands; }; }
    // GatewayModelPricingCalculation.operation
    { let _ = |value: &scm_sdk::airs::gateway::models::GatewayModelPricingCalculation| { let _ = &value.operation; }; }
    // GatewayModelPricingCalculation.value
    { let _ = |value: &scm_sdk::airs::gateway::models::GatewayModelPricingCalculation| { let _ = &value.value; }; }
    // GatewayModelPricingCalculationSchema
    { let _ = scm_sdk::validation::parse("GatewayModelPricingCalculationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayModelPricingConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayModelPricingConfig>; }
    // GatewayModelPricingConfigSchema
    { let _ = scm_sdk::validation::parse("GatewayModelPricingConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayModelPricingRequestOptions
    { let _ = std::marker::PhantomData::<scm_sdk::RequestOptions>; }
    // GatewayModelPricingRequestOptions.signal
    { let _ = scm_sdk::RequestOptions::cancel; }
    // GatewayModelPricingRequestOptions.timeoutMs
    { let _ = scm_sdk::RequestOptions::with_timeout; }
    // GatewayModelTokenPrice
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayModelTokenPrice>; }
    // GatewayModelTokenPriceSchema
    { let _ = scm_sdk::validation::parse("GatewayModelTokenPriceSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayModelWhitelistParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayModelWhitelistParameters>; }
    // GatewayModelWhitelistParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayModelWhitelistParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayMutableMcpCapabilityType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayMutableMcpCapabilityType>; }
    // GatewayMutableMcpCapabilityTypeSchema
    { let _ = scm_sdk::validation::parse("GatewayMutableMcpCapabilityTypeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayOpenAIConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayOpenAIConfiguration>; }
    // GatewayOpenAIConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayOpenAIConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayOpenValue
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayOpenValue<serde_json::Value>>; }
    // GatewayOrganisationAuthSettingsUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayOrganisationAuthSettingsUpdateRequest>; }
    // GatewayOrganisationAuthSettingsUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayOrganisationAuthSettingsUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayOrganisationInfo
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayOrganisationInfo>; }
    // GatewayOrganisationInfoSchema
    { let _ = scm_sdk::validation::parse("GatewayOrganisationInfoSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayOrganisationUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayOrganisationUpdateRequest>; }
    // GatewayOrganisationUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayOrganisationUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayPANWPrismaParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayPANWPrismaParameters>; }
    // GatewayPANWPrismaParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayPANWPrismaParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayPatronusCustomParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayPatronusCustomParameters>; }
    // GatewayPatronusCustomParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayPatronusCustomParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayPatronusParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayPatronusParameters>; }
    // GatewayPatronusParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayPatronusParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayPayAsYouGoPricing
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayPayAsYouGoPricing>; }
    // GatewayPayAsYouGoPricingSchema
    { let _ = scm_sdk::validation::parse("GatewayPayAsYouGoPricingSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayPillarScanParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayPillarScanParameters>; }
    // GatewayPillarScanParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayPillarScanParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayPlugin
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayPlugin>; }
    // GatewayPluginCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayPluginCreateRequest>; }
    // GatewayPluginCreateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayPluginCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayPluginSchema
    { let _ = scm_sdk::validation::parse("GatewayPluginSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayPortkeyLanguageParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayPortkeyLanguageParameters>; }
    // GatewayPortkeyLanguageParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayPortkeyLanguageParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayPortkeyModerationParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayPortkeyModerationParameters>; }
    // GatewayPortkeyModerationParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayPortkeyModerationParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayPortkeyPIIParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayPortkeyPIIParameters>; }
    // GatewayPortkeyPIIParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayPortkeyPIIParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayPricingAdjustments
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayPricingAdjustments>; }
    // GatewayPricingAdjustmentsRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayPricingAdjustmentsRequest>; }
    // GatewayPricingAdjustmentsRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayPricingAdjustmentsRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayPricingAdjustmentsSchema
    { let _ = scm_sdk::validation::parse("GatewayPricingAdjustmentsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayPricingConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayPricingConfig>; }
    // GatewayPricingConfigRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayPricingConfigRequest>; }
    // GatewayPricingConfigRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayPricingConfigRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayPricingConfigSchema
    { let _ = scm_sdk::validation::parse("GatewayPricingConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayPricingMultiplier
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayPricingMultiplier>; }
    // GatewayPricingMultiplierSchema
    { let _ = scm_sdk::validation::parse("GatewayPricingMultiplierSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayPromptfooParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayPromptfooParameters>; }
    // GatewayPromptfooParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayPromptfooParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayProvider
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayProvider>; }
    // GatewayProviderCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayProviderCreateRequest>; }
    // GatewayProviderCreateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayProviderCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayProviderCreateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayProviderCreateResponse>; }
    // GatewayProviderCreateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayProviderCreateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayProviderDetail
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayProviderDetail>; }
    // GatewayProviderDetailSchema
    { let _ = scm_sdk::validation::parse("GatewayProviderDetailSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayProviderSchema
    { let _ = scm_sdk::validation::parse("GatewayProviderSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayProviderUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayProviderUpdateRequest>; }
    // GatewayProviderUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayProviderUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRateLimit
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRateLimit>; }
    // GatewayRateLimitInput
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRateLimitInput>; }
    // GatewayRateLimitInputSchema
    { let _ = scm_sdk::validation::parse("GatewayRateLimitInputSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRateLimitSchema
    { let _ = scm_sdk::validation::parse("GatewayRateLimitSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRateLimitsClientCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRateLimitsClientCreateRequest>; }
    // GatewayRateLimitsClientCreateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayRateLimitsClientCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRateLimitsClientCreateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRateLimitsClientCreateResponse>; }
    // GatewayRateLimitsClientCreateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayRateLimitsClientCreateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRateLimitsClientDeleteResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRateLimitsClientDeleteResponse>; }
    // GatewayRateLimitsClientDeleteResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayRateLimitsClientDeleteResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRateLimitsClientGetOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRateLimitsClientGetOptions>; }
    // GatewayRateLimitsClientGetOptionsSchema
    { let _ = scm_sdk::validation::parse("GatewayRateLimitsClientGetOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRateLimitsClientGetResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRateLimitsClientGetResponse>; }
    // GatewayRateLimitsClientGetResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayRateLimitsClientGetResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRateLimitsClientListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRateLimitsClientListOptions>; }
    // GatewayRateLimitsClientListOptionsSchema
    { let _ = scm_sdk::validation::parse("GatewayRateLimitsClientListOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRateLimitsClientListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRateLimitsClientListResponse>; }
    // GatewayRateLimitsClientListResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayRateLimitsClientListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRateLimitsClientUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRateLimitsClientUpdateRequest>; }
    // GatewayRateLimitsClientUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayRateLimitsClientUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRateLimitsClientUpdateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRateLimitsClientUpdateResponse>; }
    // GatewayRateLimitsClientUpdateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayRateLimitsClientUpdateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRateLimitsPolicy
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRateLimitsPolicy>; }
    // GatewayRateLimitsPolicyListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRateLimitsPolicyListResponse>; }
    // GatewayRateLimitsPolicyListResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayRateLimitsPolicyListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRateLimitsPolicyResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRateLimitsPolicyResponse>; }
    // GatewayRateLimitsPolicyResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayRateLimitsPolicyResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRateLimitsPolicySchema
    { let _ = scm_sdk::validation::parse("GatewayRateLimitsPolicySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRateLimitType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRateLimitType>; }
    // GatewayRateLimitTypeSchema
    { let _ = scm_sdk::validation::parse("GatewayRateLimitTypeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRateLimitUnit
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRateLimitUnit>; }
    // GatewayRateLimitUnitSchema
    { let _ = scm_sdk::validation::parse("GatewayRateLimitUnitSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRealtimeConnection
    { let _ = std::marker::PhantomData::<scm_sdk::realtime::GatewayRealtimeConnection>; }
    // GatewayRealtimeConnection.__@asyncIterator@1855
    { let _ = scm_sdk::realtime::GatewayRealtimeConnection::into_stream; }
    // GatewayRealtimeConnection.cancel
    { let _ = scm_sdk::realtime::GatewayRealtimeConnection::cancel; }
    // GatewayRealtimeConnection.next
    { let _ = scm_sdk::realtime::GatewayRealtimeConnection::next; }
    // GatewayRealtimeConnection.return
    { let _ = scm_sdk::realtime::GatewayRealtimeConnection::cancel; }
    // GatewayRealtimeConnection.send
    { let _ = scm_sdk::realtime::GatewayRealtimeConnection::send; }
    // GatewayRealtimeConnection.throw
    { let _ = scm_sdk::realtime::GatewayRealtimeConnection::cancel; }
    // GatewayRealtimeConnectRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRealtimeConnectRequest>; }
    // GatewayRealtimeConnectRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayRealtimeConnectRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRealtimeEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRealtimeEvent>; }
    // GatewayRealtimeEvent.type
    { let _ = |value: &scm_sdk::airs::gateway::models::GatewayRealtimeEvent| { let _ = &value.r#type; }; }
    // GatewayRealtimeEventSchema
    { let _ = scm_sdk::validation::parse("GatewayRealtimeEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRealtimeOptions
    { let _ = std::marker::PhantomData::<scm_sdk::realtime::GatewayRealtimeOptions>; }
    // GatewayRealtimeOptions.handshakeTimeoutMs
    { let _ = |value: &scm_sdk::realtime::GatewayRealtimeOptions| { let _ = value.handshake_timeout; }; }
    // GatewayRealtimeOptions.headers
    { let _ = scm_sdk::airs::gateway::RuntimeClient::with_routing; }
    // GatewayRealtimeOptions.maxBufferedBytes
    { let _ = |value: &scm_sdk::realtime::GatewayRealtimeOptions| { let _ = &value.max_buffered_bytes; }; }
    // GatewayRealtimeOptions.maxBufferedEvents
    { let _ = |value: &scm_sdk::realtime::GatewayRealtimeOptions| { let _ = &value.max_buffered_events; }; }
    // GatewayRealtimeOptions.maxEventBytes
    { let _ = |value: &scm_sdk::realtime::GatewayRealtimeOptions| { let _ = &value.max_event_bytes; }; }
    // GatewayRealtimeOptions.signal
    { let _ = scm_sdk::RequestOptions::cancel; }
    // GatewayRealtimeOptions.timeoutMs
    { let _ = scm_sdk::RequestOptions::with_timeout; }
    // GatewayRealtimeOptions.webSocketFactory
    { let _ = scm_sdk::realtime::GatewayRealtimeOptions::new; }
    // GatewayRegexMatchParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRegexMatchParameters>; }
    // GatewayRegexMatchParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayRegexMatchParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequestAwsAccessKeyAuthConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequestAwsAccessKeyAuthConfig>; }
    // GatewayRequestAwsAccessKeyAuthConfigSchema
    { let _ = scm_sdk::validation::parse("GatewayRequestAwsAccessKeyAuthConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequestAwsAssumedRoleAuthConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequestAwsAssumedRoleAuthConfig>; }
    // GatewayRequestAwsAssumedRoleAuthConfigSchema
    { let _ = scm_sdk::validation::parse("GatewayRequestAwsAssumedRoleAuthConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequestAwsServiceRoleAuthConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequestAwsServiceRoleAuthConfig>; }
    // GatewayRequestAwsServiceRoleAuthConfigSchema
    { let _ = scm_sdk::validation::parse("GatewayRequestAwsServiceRoleAuthConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequestAzureDefaultAuthConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequestAzureDefaultAuthConfig>; }
    // GatewayRequestAzureDefaultAuthConfigSchema
    { let _ = scm_sdk::validation::parse("GatewayRequestAzureDefaultAuthConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequestAzureEntraAuthConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequestAzureEntraAuthConfig>; }
    // GatewayRequestAzureEntraAuthConfigSchema
    { let _ = scm_sdk::validation::parse("GatewayRequestAzureEntraAuthConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequestAzureManagedAuthConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequestAzureManagedAuthConfig>; }
    // GatewayRequestAzureManagedAuthConfigSchema
    { let _ = scm_sdk::validation::parse("GatewayRequestAzureManagedAuthConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequestBulkUpdateMcpServerCapabilities
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequestBulkUpdateMcpServerCapabilities>; }
    // GatewayRequestBulkUpdateMcpServerCapabilitiesSchema
    { let _ = scm_sdk::validation::parse("GatewayRequestBulkUpdateMcpServerCapabilitiesSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequestBulkUpdateMcpServerUserAccess
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequestBulkUpdateMcpServerUserAccess>; }
    // GatewayRequestBulkUpdateMcpServerUserAccessSchema
    { let _ = scm_sdk::validation::parse("GatewayRequestBulkUpdateMcpServerUserAccessSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequestCondition
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequestCondition>; }
    // GatewayRequestConditionSchema
    { let _ = scm_sdk::validation::parse("GatewayRequestConditionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequestCreateMcpServer
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequestCreateMcpServer>; }
    // GatewayRequestCreateMcpServerSchema
    { let _ = scm_sdk::validation::parse("GatewayRequestCreateMcpServerSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequestGroupBy
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequestGroupBy>; }
    // GatewayRequestGroupBySchema
    { let _ = scm_sdk::validation::parse("GatewayRequestGroupBySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequestHashicorpAppRoleAuthConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequestHashicorpAppRoleAuthConfig>; }
    // GatewayRequestHashicorpAppRoleAuthConfigSchema
    { let _ = scm_sdk::validation::parse("GatewayRequestHashicorpAppRoleAuthConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequestHashicorpKubernetesAuthConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequestHashicorpKubernetesAuthConfig>; }
    // GatewayRequestHashicorpKubernetesAuthConfigSchema
    { let _ = scm_sdk::validation::parse("GatewayRequestHashicorpKubernetesAuthConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequestHashicorpTokenAuthConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequestHashicorpTokenAuthConfig>; }
    // GatewayRequestHashicorpTokenAuthConfigSchema
    { let _ = scm_sdk::validation::parse("GatewayRequestHashicorpTokenAuthConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequestLogExportsRequestedData
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequestLogExportsRequestedData>; }
    // GatewayRequestLogExportsRequestedDataSchema
    { let _ = scm_sdk::validation::parse("GatewayRequestLogExportsRequestedDataSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequestPayAsYouGoPricing
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequestPayAsYouGoPricing>; }
    // GatewayRequestPayAsYouGoPricingSchema
    { let _ = scm_sdk::validation::parse("GatewayRequestPayAsYouGoPricingSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequestPricingMultiplier
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequestPricingMultiplier>; }
    // GatewayRequestPricingMultiplierSchema
    { let _ = scm_sdk::validation::parse("GatewayRequestPricingMultiplierSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequestTokenPricing
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequestTokenPricing>; }
    // GatewayRequestTokenPricingSchema
    { let _ = scm_sdk::validation::parse("GatewayRequestTokenPricingSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequestUpdateMcpServer
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequestUpdateMcpServer>; }
    // GatewayRequestUpdateMcpServerSchema
    { let _ = scm_sdk::validation::parse("GatewayRequestUpdateMcpServerSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRequiredMetadataKeysParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRequiredMetadataKeysParameters>; }
    // GatewayRequiredMetadataKeysParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayRequiredMetadataKeysParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRoutingCache
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRoutingCache>; }
    // GatewayRoutingCacheSchema
    { let _ = scm_sdk::validation::parse("GatewayRoutingCacheSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRoutingConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRoutingConfig>; }
    // GatewayRoutingConfigSchema
    { let _ = scm_sdk::validation::parse("GatewayRoutingConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRoutingRetry
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRoutingRetry>; }
    // GatewayRoutingRetrySchema
    { let _ = scm_sdk::validation::parse("GatewayRoutingRetrySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRoutingStrategy
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRoutingStrategy>; }
    // GatewayRoutingStrategySchema
    { let _ = scm_sdk::validation::parse("GatewayRoutingStrategySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayRoutingTarget
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayRoutingTarget>; }
    // GatewayRoutingTargetSchema
    { let _ = scm_sdk::validation::parse("GatewayRoutingTargetSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewaySageMakerConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewaySageMakerConfiguration>; }
    // GatewaySageMakerConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewaySageMakerConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewaySecretDirection
    { let _ = std::marker::PhantomData::<scm_sdk::constants::GatewaySecretDirection>; }
    // GatewaySecretFieldRule
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewaySecretFieldRule>; }
    // GatewaySecretFieldRule.oneTime
    { let _ = |value: &scm_sdk::airs::gateway::models::GatewaySecretFieldRule| { let _ = &value.one_time; }; }
    // GatewaySecretFieldRule.path
    { let _ = |value: &scm_sdk::airs::gateway::models::GatewaySecretFieldRule| { let _ = &value.path; }; }
    // GatewaySecretFieldRule.redact
    { let _ = |value: &scm_sdk::airs::gateway::models::GatewaySecretFieldRule| { let _ = &value.redact; }; }
    // GatewaySecretMapping
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewaySecretMapping>; }
    // GatewaySecretMappingSchema
    { let _ = scm_sdk::validation::parse("GatewaySecretMappingSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewaySecretOperationMetadata
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewaySecretOperationMetadata>; }
    // GatewaySecretOperationMetadata.request
    { let _ = |value: &scm_sdk::airs::gateway::models::GatewaySecretOperationMetadata| { let _ = &value.request; }; }
    // GatewaySecretOperationMetadata.response
    { let _ = |value: &scm_sdk::airs::gateway::models::GatewaySecretOperationMetadata| { let _ = &value.response; }; }
    // GatewaySecretPathSegment
    { let _ = std::marker::PhantomData::<scm_sdk::constants::GatewaySecretPathSegment>; }
    // GatewaySecretReferenceDetailResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewaySecretReferenceDetailResponse>; }
    // GatewaySecretReferenceDetailResponseSchema
    { let _ = scm_sdk::validation::parse("GatewaySecretReferenceDetailResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewaySecretReferenceListItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewaySecretReferenceListItem>; }
    // GatewaySecretReferenceListItemSchema
    { let _ = scm_sdk::validation::parse("GatewaySecretReferenceListItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewaySecretReferencesClientCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewaySecretReferencesClientCreateRequest>; }
    // GatewaySecretReferencesClientCreateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewaySecretReferencesClientCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewaySecretReferencesClientCreateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewaySecretReferencesClientCreateResponse>; }
    // GatewaySecretReferencesClientCreateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewaySecretReferencesClientCreateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewaySecretReferencesClientDeleteResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewaySecretReferencesClientDeleteResponse>; }
    // GatewaySecretReferencesClientDeleteResponseSchema
    { let _ = scm_sdk::validation::parse("GatewaySecretReferencesClientDeleteResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewaySecretReferencesClientGetResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewaySecretReferencesClientGetResponse>; }
    // GatewaySecretReferencesClientGetResponseSchema
    { let _ = scm_sdk::validation::parse("GatewaySecretReferencesClientGetResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewaySecretReferencesClientListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewaySecretReferencesClientListOptions>; }
    // GatewaySecretReferencesClientListOptionsSchema
    { let _ = scm_sdk::validation::parse("GatewaySecretReferencesClientListOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewaySecretReferencesClientListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewaySecretReferencesClientListResponse>; }
    // GatewaySecretReferencesClientListResponseSchema
    { let _ = scm_sdk::validation::parse("GatewaySecretReferencesClientListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewaySecretReferencesClientUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewaySecretReferencesClientUpdateRequest>; }
    // GatewaySecretReferencesClientUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewaySecretReferencesClientUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewaySecretReferencesClientUpdateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewaySecretReferencesClientUpdateResponse>; }
    // GatewaySecretReferencesClientUpdateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewaySecretReferencesClientUpdateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewaySentenceCountParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewaySentenceCountParameters>; }
    // GatewaySentenceCountParametersSchema
    { let _ = scm_sdk::validation::parse("GatewaySentenceCountParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayServiceApiKeyCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayServiceApiKeyCreateRequest>; }
    // GatewayServiceApiKeyCreateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayServiceApiKeyCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayStream
    { let _ = std::marker::PhantomData::<scm_sdk::streaming::ContractStream<scm_sdk::data::JsonValue>>; }
    // GatewayStream.__@asyncIterator@1855
    { let _ = scm_sdk::streaming::ContractStream::<scm_sdk::data::JsonValue>::next; }
    // GatewayStream.cancel
    { let _ = scm_sdk::streaming::ContractStream::<scm_sdk::data::JsonValue>::close; }
    // GatewayStream.next
    { let _ = scm_sdk::streaming::ContractStream::<scm_sdk::data::JsonValue>::next; }
    // GatewayStream.return
    { let _ = scm_sdk::streaming::ContractStream::<scm_sdk::data::JsonValue>::close; }
    // GatewayStream.throw
    { let _ = scm_sdk::streaming::ContractStream::<scm_sdk::data::JsonValue>::close; }
    // GatewaySydeGuardParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewaySydeGuardParameters>; }
    // GatewaySydeGuardParametersSchema
    { let _ = scm_sdk::validation::parse("GatewaySydeGuardParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayTokenPricing
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayTokenPricing>; }
    // GatewayTokenPricingSchema
    { let _ = scm_sdk::validation::parse("GatewayTokenPricingSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUpdateExportResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUpdateExportResponse>; }
    // GatewayUpdateExportResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayUpdateExportResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUpdateRateLimitsPolicyRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUpdateRateLimitsPolicyRequest>; }
    // GatewayUpdateRateLimitsPolicyRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayUpdateRateLimitsPolicyRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUpdateSecretReferenceRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUpdateSecretReferenceRequest>; }
    // GatewayUpdateSecretReferenceRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayUpdateSecretReferenceRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUpdateUsageLimitsPolicyRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUpdateUsageLimitsPolicyRequest>; }
    // GatewayUpdateUsageLimitsPolicyRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayUpdateUsageLimitsPolicyRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUppercaseParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUppercaseParameters>; }
    // GatewayUppercaseParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayUppercaseParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUpsertMcpServerMappingRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUpsertMcpServerMappingRequest>; }
    // GatewayUpsertMcpServerMappingRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayUpsertMcpServerMappingRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUpsertMcpServerMappingResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUpsertMcpServerMappingResponse>; }
    // GatewayUpsertMcpServerMappingResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayUpsertMcpServerMappingResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUsageLimit
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUsageLimit>; }
    // GatewayUsageLimitInput
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUsageLimitInput>; }
    // GatewayUsageLimitInputSchema
    { let _ = scm_sdk::validation::parse("GatewayUsageLimitInputSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUsageLimitSchema
    { let _ = scm_sdk::validation::parse("GatewayUsageLimitSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUsageLimitsClientCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUsageLimitsClientCreateRequest>; }
    // GatewayUsageLimitsClientCreateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayUsageLimitsClientCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUsageLimitsClientCreateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUsageLimitsClientCreateResponse>; }
    // GatewayUsageLimitsClientCreateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayUsageLimitsClientCreateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUsageLimitsClientDeleteResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUsageLimitsClientDeleteResponse>; }
    // GatewayUsageLimitsClientDeleteResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayUsageLimitsClientDeleteResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUsageLimitsClientGetOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUsageLimitsClientGetOptions>; }
    // GatewayUsageLimitsClientGetOptionsSchema
    { let _ = scm_sdk::validation::parse("GatewayUsageLimitsClientGetOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUsageLimitsClientGetResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUsageLimitsClientGetResponse>; }
    // GatewayUsageLimitsClientGetResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayUsageLimitsClientGetResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUsageLimitsClientListEntitiesOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUsageLimitsClientListEntitiesOptions>; }
    // GatewayUsageLimitsClientListEntitiesOptionsSchema
    { let _ = scm_sdk::validation::parse("GatewayUsageLimitsClientListEntitiesOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUsageLimitsClientListEntitiesResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUsageLimitsClientListEntitiesResponse>; }
    // GatewayUsageLimitsClientListEntitiesResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayUsageLimitsClientListEntitiesResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUsageLimitsClientListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUsageLimitsClientListOptions>; }
    // GatewayUsageLimitsClientListOptionsSchema
    { let _ = scm_sdk::validation::parse("GatewayUsageLimitsClientListOptionsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUsageLimitsClientListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUsageLimitsClientListResponse>; }
    // GatewayUsageLimitsClientListResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayUsageLimitsClientListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUsageLimitsClientResetEntityResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUsageLimitsClientResetEntityResponse>; }
    // GatewayUsageLimitsClientResetEntityResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayUsageLimitsClientResetEntityResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUsageLimitsClientUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUsageLimitsClientUpdateRequest>; }
    // GatewayUsageLimitsClientUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayUsageLimitsClientUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUsageLimitsClientUpdateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUsageLimitsClientUpdateResponse>; }
    // GatewayUsageLimitsClientUpdateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayUsageLimitsClientUpdateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUsageLimitsPolicy
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUsageLimitsPolicy>; }
    // GatewayUsageLimitsPolicyEntity
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUsageLimitsPolicyEntity>; }
    // GatewayUsageLimitsPolicyEntityListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUsageLimitsPolicyEntityListResponse>; }
    // GatewayUsageLimitsPolicyEntityListResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayUsageLimitsPolicyEntityListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUsageLimitsPolicyEntitySchema
    { let _ = scm_sdk::validation::parse("GatewayUsageLimitsPolicyEntitySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUsageLimitsPolicyListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUsageLimitsPolicyListResponse>; }
    // GatewayUsageLimitsPolicyListResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayUsageLimitsPolicyListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUsageLimitsPolicyResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUsageLimitsPolicyResponse>; }
    // GatewayUsageLimitsPolicyResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayUsageLimitsPolicyResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUsageLimitsPolicySchema
    { let _ = scm_sdk::validation::parse("GatewayUsageLimitsPolicySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayUserApiKeyCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayUserApiKeyCreateRequest>; }
    // GatewayUserApiKeyCreateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayUserApiKeyCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayValidUrlsParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayValidUrlsParameters>; }
    // GatewayValidUrlsParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayValidUrlsParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayValueKeyUsage
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayValueKeyUsage>; }
    // GatewayValueKeyUsageSchema
    { let _ = scm_sdk::validation::parse("GatewayValueKeyUsageSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayVertexAIConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayVertexAIConfiguration>; }
    // GatewayVertexAIConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayVertexAIConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayWebhookParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayWebhookParameters>; }
    // GatewayWebhookParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayWebhookParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayWebSocket
    { let _ = std::marker::PhantomData::<scm_sdk::realtime::BoxWebSocket>; }
    // GatewayWebSocket.bufferedAmount
    { let _ = |socket: &mut dyn scm_sdk::realtime::GatewayWebSocket, cx: &mut std::task::Context<'_>| { let _ = futures_util::Sink::poll_flush(std::pin::Pin::new(socket), cx); }; }
    // GatewayWebSocket.close
    { let _ = |socket: &mut dyn scm_sdk::realtime::GatewayWebSocket, cx: &mut std::task::Context<'_>| { let _ = futures_util::Sink::poll_close(std::pin::Pin::new(socket), cx); }; }
    // GatewayWebSocket.off
    { let _ = |socket: &mut dyn scm_sdk::realtime::GatewayWebSocket, cx: &mut std::task::Context<'_>| { let _ = futures_util::Stream::poll_next(std::pin::Pin::new(socket), cx); }; }
    // GatewayWebSocket.on
    { let _ = |socket: &mut dyn scm_sdk::realtime::GatewayWebSocket, cx: &mut std::task::Context<'_>| { let _ = futures_util::Stream::poll_next(std::pin::Pin::new(socket), cx); }; }
    // GatewayWebSocket.readyState
    { let _ = |socket: &mut dyn scm_sdk::realtime::GatewayWebSocket, cx: &mut std::task::Context<'_>| { let _ = futures_util::Sink::poll_ready(std::pin::Pin::new(socket), cx); }; }
    // GatewayWebSocket.send
    { let _ = |socket: &mut dyn scm_sdk::realtime::GatewayWebSocket, frame: scm_sdk::realtime::WebSocketFrame| { let _ = futures_util::Sink::start_send(std::pin::Pin::new(socket), frame); }; }
    // GatewayWebSocket.terminate
    { let _ = std::mem::drop::<scm_sdk::realtime::BoxWebSocket>; }
    // GatewayWebSocketFactory
    { let _ = std::marker::PhantomData::<scm_sdk::realtime::GatewayWebSocketFactory>; }
    // GatewayWordCountParameters
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayWordCountParameters>; }
    // GatewayWordCountParametersSchema
    { let _ = scm_sdk::validation::parse("GatewayWordCountParametersSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayWorkersAIConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayWorkersAIConfiguration>; }
    // GatewayWorkersAIConfigurationSchema
    { let _ = scm_sdk::validation::parse("GatewayWorkersAIConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayWorkspace
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayWorkspace>; }
    // GatewayWorkspaceBinding
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayWorkspaceBinding>; }
    // GatewayWorkspaceBindingSchema
    { let _ = scm_sdk::validation::parse("GatewayWorkspaceBindingSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayWorkspaceCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayWorkspaceCreateRequest>; }
    // GatewayWorkspaceCreateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayWorkspaceCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayWorkspaceCreateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayWorkspaceCreateResponse>; }
    // GatewayWorkspaceCreateResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayWorkspaceCreateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayWorkspaceDetail
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayWorkspaceDetail>; }
    // GatewayWorkspaceDetailSchema
    { let _ = scm_sdk::validation::parse("GatewayWorkspaceDetailSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayWorkspaceProvisionRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::GatewayWorkspaceProvisionRequest>; }
    // GatewayWorkspaceProvisionResult
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::GatewayWorkspaceProvisionResult>; }
    // GatewayWorkspaceProvisionResult.scope
    { let _ = |value: &scm_sdk::airs::gateway::GatewayWorkspaceProvisionResult| { let _ = &value.scope; }; }
    // GatewayWorkspaceProvisionResult.scopeCreated
    { let _ = |value: &scm_sdk::airs::gateway::GatewayWorkspaceProvisionResult| { let _ = &value.scope_created; }; }
    // GatewayWorkspaceProvisionResult.workspace
    { let _ = |value: &scm_sdk::airs::gateway::GatewayWorkspaceProvisionResult| { let _ = &value.workspace; }; }
    // GatewayWorkspaceSchema
    { let _ = scm_sdk::validation::parse("GatewayWorkspaceSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayWorkspaceUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayWorkspaceUpdateRequest>; }
    // GatewayWorkspaceUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("GatewayWorkspaceUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GatewayWriteResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GatewayWriteResponse>; }
    // GatewayWriteResponseSchema
    { let _ = scm_sdk::validation::parse("GatewayWriteResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // generateWorkspaceScopeName
    { let _ = scm_sdk::helpers::generate_workspace_scope_name; }
    // GetTokenOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::GetTokenOptions>; }
    // GetTokenOptions.body
    { let _ = |value: &scm_sdk::airs::management::GetTokenOptions| { let _ = &value.body; }; }
    // GetTokenOptions.tokenTtlInterval
    { let _ = |value: &scm_sdk::airs::management::GetTokenOptions| { let _ = &value.token_ttl_interval; }; }
    // GetTokenOptions.tokenTtlUnit
    { let _ = |value: &scm_sdk::airs::management::GetTokenOptions| { let _ = &value.token_ttl_unit; }; }
    // globalConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::configuration::ScanConfiguration>; }
    // globalConfiguration.apiEndpoint
    { let _ = scm_sdk::configuration::ScanConfiguration::api_endpoint; }
    // globalConfiguration.apiKey
    { let _ = scm_sdk::configuration::ScanConfiguration::api_key; }
    // globalConfiguration.apiToken
    { let _ = scm_sdk::configuration::ScanConfiguration::api_token; }
    // globalConfiguration.init
    { let _ = scm_sdk::configuration::ScanConfiguration::init; }
    // globalConfiguration.initialized
    { let _ = scm_sdk::configuration::ScanConfiguration::initialized; }
    // globalConfiguration.numRetries
    { let _ = scm_sdk::configuration::ScanConfiguration::num_retries; }
    // globalConfiguration.reset
    { let _ = scm_sdk::configuration::ScanConfiguration::reset; }
    // Goal
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::Goal>; }
    // GoalCategoryListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::GoalCategoryListResponse>; }
    // GoalCategoryListResponseSchema
    { let _ = scm_sdk::validation::parse("GoalCategoryListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GoalCategoryOption
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::GoalCategoryOption>; }
    // GoalCategoryOptionSchema
    { let _ = scm_sdk::validation::parse("GoalCategoryOptionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GoalListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::GoalListOptions>; }
    // GoalListOptions.count
    { let _ = |value: &scm_sdk::airs::red_team::GoalListOptions| { let _ = &value.count; }; }
    // GoalListOptions.goal_category
    { let _ = |value: &scm_sdk::airs::red_team::GoalListOptions| { let _ = &value.goal_category; }; }
    // GoalListOptions.goal_type
    { let _ = |value: &scm_sdk::airs::red_team::GoalListOptions| { let _ = &value.goal_type; }; }
    // GoalListOptions.limit
    { let _ = |value: &scm_sdk::airs::red_team::GoalListOptions| { let _ = &value.limit; }; }
    // GoalListOptions.search
    { let _ = |value: &scm_sdk::airs::red_team::GoalListOptions| { let _ = &value.search; }; }
    // GoalListOptions.skip
    { let _ = |value: &scm_sdk::airs::red_team::GoalListOptions| { let _ = &value.skip; }; }
    // GoalListOptions.status
    { let _ = |value: &scm_sdk::airs::red_team::GoalListOptions| { let _ = &value.status; }; }
    // GoalListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::GoalListResponse>; }
    // GoalListResponseSchema
    { let _ = scm_sdk::validation::parse("GoalListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GoalSchema
    { let _ = scm_sdk::validation::parse("GoalSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GoalType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::GoalType>; }
    // GoalTypeQueryParam
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::GoalTypeQueryParam>; }
    // GroupedErrorsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GroupedErrorsResponse>; }
    // GroupedErrorsResponseSchema
    { let _ = scm_sdk::validation::parse("GroupedErrorsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GroupListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::GroupListResponse>; }
    // GroupListResponseSchema
    { let _ = scm_sdk::validation::parse("GroupListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // GuardrailAction
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::GuardrailAction>; }
    // HEADER_API_KEY
    { let _ = scm_sdk::constants::HEADER_API_KEY; }
    // HEADER_AUTH_TOKEN
    { let _ = scm_sdk::constants::HEADER_AUTH_TOKEN; }
    // HeadersAuthConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::HeadersAuthConfig>; }
    // HeadersAuthConfigSchema
    { let _ = scm_sdk::validation::parse("HeadersAuthConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // HTTP_FORCE_RETRY_STATUS_CODES
    { let _ = scm_sdk::constants::HTTP_FORCE_RETRY_STATUS_CODES; }
    // HTTPValidationError
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::HTTPValidationError>; }
    // HTTPValidationErrorSchema
    { let _ = scm_sdk::validation::parse("HTTPValidationErrorSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // HuggingfaceConnectionParams
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::HuggingfaceConnectionParams>; }
    // HuggingfaceConnectionParamsSchema
    { let _ = scm_sdk::validation::parse("HuggingfaceConnectionParamsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // IAM_ENDPOINT
    { let _ = scm_sdk::constants::IAM_ENDPOINT; }
    // IAM_SCOPES_PATH
    { let _ = scm_sdk::constants::IAM_SCOPES_PATH; }
    // IamScope
    { let _ = std::marker::PhantomData::<scm_sdk::shared::iam::models::IamScope>; }
    // IamScopeCreateInput
    { let _ = std::marker::PhantomData::<scm_sdk::shared::iam::models::IamScopeCreateInput>; }
    // IamScopeCreateInput.description
    { let _ = |value: &scm_sdk::shared::iam::models::IamScopeCreateInput| { let _ = &value.description; }; }
    // IamScopeCreateInput.name
    { let _ = |value: &scm_sdk::shared::iam::models::IamScopeCreateInput| { let _ = &value.name; }; }
    // IamScopeCreateInput.resources
    { let _ = |value: &scm_sdk::shared::iam::models::IamScopeCreateInput| { let _ = &value.resources; }; }
    // IamScopeCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::shared::iam::models::IamScopeCreateRequest>; }
    // IamScopeCreateRequestSchema
    { let _ = scm_sdk::validation::parse("IamScopeCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // IamScopeListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::shared::iam::models::IamScopeListResponse>; }
    // IamScopeListResponseSchema
    { let _ = scm_sdk::validation::parse("IamScopeListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // IamScopeResource
    { let _ = std::marker::PhantomData::<scm_sdk::shared::iam::models::IamScopeResource>; }
    // IamScopeResourceInput
    { let _ = std::marker::PhantomData::<scm_sdk::shared::iam::models::IamScopeResourceInput>; }
    // IamScopeResourceInputSchema
    { let _ = scm_sdk::validation::parse("IamScopeResourceInputSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // IamScopeResourceSchema
    { let _ = scm_sdk::validation::parse("IamScopeResourceSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // IamScopeSchema
    { let _ = scm_sdk::validation::parse("IamScopeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // IamScopesClient
    { let _ = std::marker::PhantomData::<scm_sdk::shared::iam::IamScopesClient>; let _ = scm_sdk::shared::iam::IamScopesClient::new; }
    // IamScopesClient.bindWorkspace
    { let _ = scm_sdk::shared::iam::IamScopesClient::bind_workspace; }
    // IamScopesClient.create
    { let _ = scm_sdk::shared::iam::IamScopesClient::create; }
    // IamScopesClient.delete
    { let _ = scm_sdk::shared::iam::IamScopesClient::delete; }
    // IamScopesClient.get
    { let _ = scm_sdk::shared::iam::IamScopesClient::get; }
    // IamScopesClient.list
    { let _ = scm_sdk::shared::iam::IamScopesClient::list; }
    // IamScopesClient.update
    { let _ = scm_sdk::shared::iam::IamScopesClient::update; }
    // IamScopesClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // IamScopesClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // IamScopesClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // IamScopesClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // IamScopeUpdateInput
    { let _ = std::marker::PhantomData::<scm_sdk::shared::iam::models::IamScopeUpdateInput>; }
    // IamScopeUpdateInput.description
    { let _ = |value: &scm_sdk::shared::iam::models::IamScopeUpdateInput| { let _ = &value.description; }; }
    // IamScopeUpdateInput.resources
    { let _ = |value: &scm_sdk::shared::iam::models::IamScopeUpdateInput| { let _ = &value.resources; }; }
    // IamScopeUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::shared::iam::models::IamScopeUpdateRequest>; }
    // IamScopeUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("IamScopeUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // init
    { let _ = scm_sdk::configuration::init; let _ = scm_sdk::configuration::init_from_environment; }
    // InitOptions
    { let _ = std::marker::PhantomData::<scm_sdk::configuration::InitOptions>; }
    // InitOptions.apiEndpoint
    { let _ = |x: &scm_sdk::configuration::InitOptions| { let _ = &x.api_endpoint; }; }
    // InitOptions.apiKey
    { let _ = |x: &scm_sdk::configuration::InitOptions| { let _ = &x.api_key; }; }
    // InitOptions.apiToken
    { let _ = |x: &scm_sdk::configuration::InitOptions| { let _ = &x.api_token; }; }
    // InitOptions.numRetries
    { let _ = |x: &scm_sdk::configuration::InitOptions| { let _ = &x.num_retries; }; }
    // InstanceExtraDetails
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::InstanceExtraDetails>; }
    // InstanceExtraDetailsSchema
    { let _ = scm_sdk::validation::parse("InstanceExtraDetailsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // InstanceGetResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::InstanceGetResponse>; }
    // InstanceGetResponseSchema
    { let _ = scm_sdk::validation::parse("InstanceGetResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // InstanceRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::InstanceRequest>; }
    // InstanceRequestSchema
    { let _ = scm_sdk::validation::parse("InstanceRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // InstanceResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::InstanceResponse>; }
    // InstanceResponseSchema
    { let _ = scm_sdk::validation::parse("InstanceResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // IODetected
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::IODetected>; }
    // IODetectedSchema
    { let _ = scm_sdk::validation::parse("IODetectedSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // JobAbortResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::JobAbortResponse>; }
    // JobAbortResponseSchema
    { let _ = scm_sdk::validation::parse("JobAbortResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // JobCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::JobCreateRequest>; }
    // JobCreateRequestSchema
    { let _ = scm_sdk::validation::parse("JobCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // JobListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::JobListResponse>; }
    // JobListResponseSchema
    { let _ = scm_sdk::validation::parse("JobListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // JobResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::JobResponse>; }
    // JobResponseSchema
    { let _ = scm_sdk::validation::parse("JobResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // JobStatus
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::JobStatus>; }
    // JobStatusFilter
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::JobStatusFilter>; }
    // JobTimeRecord
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::JobTimeRecord>; }
    // JobTimeRecordSchema
    { let _ = scm_sdk::validation::parse("JobTimeRecordSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // JobType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::JobType>; }
    // jsonNullable
    { let _ = scm_sdk::helpers::json_nullable::<String>(None, |value| value.as_str().map(str::to_owned).ok_or_else(|| scm_sdk::Error::new(scm_sdk::ErrorKind::Validation))); }
    // JsonNullable
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::JsonNullable<serde_json::Value>>; }
    // Label
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::Label>; }
    // LabelCondition
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::LabelCondition>; }
    // LabelConditionSchema
    { let _ = scm_sdk::validation::parse("LabelConditionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // LabelKeyList
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::LabelKeyList>; }
    // LabelKeyListSchema
    { let _ = scm_sdk::validation::parse("LabelKeyListSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // LabelSchema
    { let _ = scm_sdk::validation::parse("LabelSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // LabelsCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::LabelsCreateRequest>; }
    // LabelsCreateRequestSchema
    { let _ = scm_sdk::validation::parse("LabelsCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // LabelsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::LabelsResponse>; }
    // LabelsResponseSchema
    { let _ = scm_sdk::validation::parse("LabelsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // LabelValueList
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::LabelValueList>; }
    // LabelValueListSchema
    { let _ = scm_sdk::validation::parse("LabelValueListSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // LanguageOption
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::LanguageOption>; }
    // LanguageOptionSchema
    { let _ = scm_sdk::validation::parse("LanguageOptionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // LatencyChartResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::LatencyChartResponse>; }
    // LatencyChartResponseSchema
    { let _ = scm_sdk::validation::parse("LatencyChartResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ListApiKeysResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::ListApiKeysResponse>; }
    // ListApiKeysResponseSchema
    { let _ = scm_sdk::validation::parse("ListApiKeysResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ListCatalogProvidersResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::ListCatalogProvidersResponse>; }
    // ListCatalogProvidersResponseSchema
    { let _ = scm_sdk::validation::parse("ListCatalogProvidersResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ListConfigsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::ListConfigsResponse>; }
    // ListConfigsResponseSchema
    { let _ = scm_sdk::validation::parse("ListConfigsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ListConfigVersionsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::ListConfigVersionsResponse>; }
    // ListConfigVersionsResponseSchema
    { let _ = scm_sdk::validation::parse("ListConfigVersionsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ListCustomRuleSecurityGroupsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ListCustomRuleSecurityGroupsResponse>; }
    // ListCustomRuleSecurityGroupsResponseSchema
    { let _ = scm_sdk::validation::parse("ListCustomRuleSecurityGroupsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ListCustomRulesResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ListCustomRulesResponse>; }
    // ListCustomRulesResponseSchema
    { let _ = scm_sdk::validation::parse("ListCustomRulesResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ListDeploymentsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::ListDeploymentsResponse>; }
    // ListDeploymentsResponseSchema
    { let _ = scm_sdk::validation::parse("ListDeploymentsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ListGuardrailsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::ListGuardrailsResponse>; }
    // ListGuardrailsResponseSchema
    { let _ = scm_sdk::validation::parse("ListGuardrailsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ListingOptions
    { let _ = std::marker::PhantomData::<scm_sdk::helpers::ListingOptions>; }
    // ListingOptions.limit
    { let _ = |value: &scm_sdk::helpers::ListingOptions| { let _ = &value.limit; }; }
    // ListingOptions.search
    { let _ = |value: &scm_sdk::helpers::ListingOptions| { let _ = &value.search; }; }
    // ListingOptions.skip
    { let _ = |value: &scm_sdk::helpers::ListingOptions| { let _ = &value.skip; }; }
    // ListIntegrationsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::ListIntegrationsResponse>; }
    // ListIntegrationsResponseSchema
    { let _ = scm_sdk::validation::parse("ListIntegrationsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ListMcpIntegrationsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::ListMcpIntegrationsResponse>; }
    // ListMcpIntegrationsResponseSchema
    { let _ = scm_sdk::validation::parse("ListMcpIntegrationsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ListModelSecurityGroupsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ListModelSecurityGroupsResponse>; }
    // ListModelSecurityGroupsResponseSchema
    { let _ = scm_sdk::validation::parse("ListModelSecurityGroupsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ListModelSecurityRuleInstancesResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ListModelSecurityRuleInstancesResponse>; }
    // ListModelSecurityRuleInstancesResponseSchema
    { let _ = scm_sdk::validation::parse("ListModelSecurityRuleInstancesResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ListModelSecurityRulesResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ListModelSecurityRulesResponse>; }
    // ListModelSecurityRulesResponseSchema
    { let _ = scm_sdk::validation::parse("ListModelSecurityRulesResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ListPluginsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::ListPluginsResponse>; }
    // ListPluginsResponseSchema
    { let _ = scm_sdk::validation::parse("ListPluginsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ListProvidersResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::ListProvidersResponse>; }
    // ListProvidersResponseSchema
    { let _ = scm_sdk::validation::parse("ListProvidersResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ListWorkspacesResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::ListWorkspacesResponse>; }
    // ListWorkspacesResponseSchema
    { let _ = scm_sdk::validation::parse("ListWorkspacesResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // MaliciousCodeProtection
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::MaliciousCodeProtection>; }
    // MaliciousCodeProtectionSchema
    { let _ = scm_sdk::validation::parse("MaliciousCodeProtectionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // MalwareReport
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::MalwareReport>; }
    // MalwareReportSchema
    { let _ = scm_sdk::validation::parse("MalwareReportSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ManagementClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::ManagementClient>; let _ = scm_sdk::airs::management::ManagementClient::new; }
    // ManagementClient.apiKeys
    { let _ = scm_sdk::airs::management::ManagementClient::api_keys; }
    // ManagementClient.customerApps
    { let _ = scm_sdk::airs::management::ManagementClient::customer_apps; }
    // ManagementClient.dashboard
    { let _ = scm_sdk::airs::management::ManagementClient::dashboard; }
    // ManagementClient.deploymentProfiles
    { let _ = scm_sdk::airs::management::ManagementClient::deployment_profiles; }
    // ManagementClient.dlp
    { let _ = scm_sdk::airs::management::ManagementClient::dlp; }
    // ManagementClient.dlpProfiles
    { let _ = scm_sdk::airs::management::ManagementClient::dlp_profiles; }
    // ManagementClient.oauth
    { let _ = scm_sdk::airs::management::ManagementClient::oauth; }
    // ManagementClient.profiles
    { let _ = scm_sdk::airs::management::ManagementClient::profiles; }
    // ManagementClient.scanLogs
    { let _ = scm_sdk::airs::management::ManagementClient::scan_logs; }
    // ManagementClient.topics
    { let _ = scm_sdk::airs::management::ManagementClient::topics; }
    // ManagementClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // ManagementClientOptions.apiEndpoint
    { let _ = scm_sdk::Client::builder; }
    // ManagementClientOptions.clientId
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer, "probe", 1, "1"); }; }
    // ManagementClientOptions.clientSecret
    { let _ = scm_sdk::ClientBuilder::oauth_credentials; }
    // ManagementClientOptions.dashboardEndpoint
    { let _ = scm_sdk::airs::management::ManagementClient::with_dashboard_endpoint; }
    // ManagementClientOptions.dlpEndpoint
    { let _ = scm_sdk::airs::management::ManagementClient::dlp; }
    // ManagementClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // ManagementClientOptions.tokenEndpoint
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer, "probe", 1, "1"); }; }
    // ManagementClientOptions.tsgId
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer, "probe", 1, "1"); }; }
    // MaskedData
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::MaskedData>; }
    // MaskedDataSchema
    { let _ = scm_sdk::validation::parse("MaskedDataSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // MAX_AI_PROFILE_NAME_LENGTH
    { let _ = scm_sdk::constants::MAX_AI_PROFILE_NAME_LENGTH; }
    // MAX_API_KEY_LENGTH
    { let _ = scm_sdk::constants::MAX_API_KEY_LENGTH; }
    // MAX_CONNECTION_POOL_SIZE
    { let _ = scm_sdk::constants::MAX_CONNECTION_POOL_SIZE; }
    // MAX_CONTENT_CONTEXT_LENGTH
    { let _ = scm_sdk::constants::MAX_CONTENT_CONTEXT_LENGTH; }
    // MAX_CONTENT_PROMPT_LENGTH
    { let _ = scm_sdk::constants::MAX_CONTENT_PROMPT_LENGTH; }
    // MAX_CONTENT_RESPONSE_LENGTH
    { let _ = scm_sdk::constants::MAX_CONTENT_RESPONSE_LENGTH; }
    // MAX_NUMBER_OF_BATCH_SCAN_OBJECTS
    { let _ = scm_sdk::constants::MAX_NUMBER_OF_BATCH_SCAN_OBJECTS; }
    // MAX_NUMBER_OF_REPORT_IDS
    { let _ = scm_sdk::constants::MAX_NUMBER_OF_REPORT_IDS; }
    // MAX_NUMBER_OF_RETRIES
    { let _ = scm_sdk::constants::MAX_NUMBER_OF_RETRIES; }
    // MAX_NUMBER_OF_SCAN_IDS
    { let _ = scm_sdk::constants::MAX_NUMBER_OF_SCAN_IDS; }
    // MAX_REPORT_ID_STR_LENGTH
    { let _ = scm_sdk::constants::MAX_REPORT_ID_STR_LENGTH; }
    // MAX_SCAN_ID_STR_LENGTH
    { let _ = scm_sdk::constants::MAX_SCAN_ID_STR_LENGTH; }
    // MAX_SESSION_ID_STR_LENGTH
    { let _ = scm_sdk::constants::MAX_SESSION_ID_STR_LENGTH; }
    // MAX_TOKEN_LENGTH
    { let _ = scm_sdk::constants::MAX_TOKEN_LENGTH; }
    // MAX_TRANSACTION_ID_STR_LENGTH
    { let _ = scm_sdk::constants::MAX_TRANSACTION_ID_STR_LENGTH; }
    // McEntry
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::McEntry>; }
    // McEntrySchema
    { let _ = scm_sdk::validation::parse("McEntrySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // McpIntegration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::McpIntegration>; }
    // McpIntegrationCapabilitiesBulkUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::McpIntegrationCapabilitiesBulkUpdateRequest>; }
    // McpIntegrationCapabilitiesBulkUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("McpIntegrationCapabilitiesBulkUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // McpIntegrationCapabilitiesResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::McpIntegrationCapabilitiesResponse>; }
    // McpIntegrationCapabilitiesResponseSchema
    { let _ = scm_sdk::validation::parse("McpIntegrationCapabilitiesResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // McpIntegrationCapabilitiesUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::McpIntegrationCapabilitiesUpdateRequest>; }
    // McpIntegrationCapabilitiesUpdateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::McpIntegrationCapabilitiesUpdateResponse>; }
    // McpIntegrationCapabilitiesUpdateResponseSchema
    { let _ = scm_sdk::validation::parse("McpIntegrationCapabilitiesUpdateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // McpIntegrationCapability
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::McpIntegrationCapability>; }
    // McpIntegrationCapabilitySchema
    { let _ = scm_sdk::validation::parse("McpIntegrationCapabilitySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // McpIntegrationCapabilityUpdate
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::McpIntegrationCapabilityUpdate>; }
    // McpIntegrationCapabilityUpdateSchema
    { let _ = scm_sdk::validation::parse("McpIntegrationCapabilityUpdateSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // McpIntegrationCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::McpIntegrationCreateRequest>; }
    // McpIntegrationCreateRequestSchema
    { let _ = scm_sdk::validation::parse("McpIntegrationCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // McpIntegrationDetail
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::McpIntegrationDetail>; }
    // McpIntegrationDetailSchema
    { let _ = scm_sdk::validation::parse("McpIntegrationDetailSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // McpIntegrationMetadata
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::McpIntegrationMetadata>; }
    // McpIntegrationMetadataSchema
    { let _ = scm_sdk::validation::parse("McpIntegrationMetadataSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // McpIntegrationSchema
    { let _ = scm_sdk::validation::parse("McpIntegrationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // McpIntegrationUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::McpIntegrationUpdateRequest>; }
    // McpIntegrationUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("McpIntegrationUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // McpIntegrationWorkspacesBulkUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::McpIntegrationWorkspacesBulkUpdateRequest>; }
    // McpIntegrationWorkspacesBulkUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("McpIntegrationWorkspacesBulkUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // McpIntegrationWorkspacesRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::McpIntegrationWorkspacesRequest>; }
    // McpIntegrationWorkspacesUpdateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::McpIntegrationWorkspacesUpdateResponse>; }
    // McpIntegrationWorkspacesUpdateResponseSchema
    { let _ = scm_sdk::validation::parse("McpIntegrationWorkspacesUpdateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // McReport
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::McReport>; }
    // McReportSchema
    { let _ = scm_sdk::validation::parse("McReportSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // Metadata
    { let _ = std::marker::PhantomData::<scm_sdk::shared::models::Metadata>; }
    // MetadataCriterion
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::MetadataCriterion>; }
    // MetadataCriterionSchema
    { let _ = scm_sdk::validation::parse("MetadataCriterionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // MetadataSchema
    { let _ = scm_sdk::validation::parse("MetadataSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // MGMT_API_KEY_PATH
    { let _ = scm_sdk::constants::MGMT_API_KEY_PATH; }
    // MGMT_API_KEYS_TSG_PATH
    { let _ = scm_sdk::constants::MGMT_API_KEYS_TSG_PATH; }
    // MGMT_CLIENT_ID
    { let _ = scm_sdk::constants::MGMT_CLIENT_ID; }
    // MGMT_CLIENT_SECRET
    { let _ = scm_sdk::constants::MGMT_CLIENT_SECRET; }
    // MGMT_CUSTOMER_APP_PATH
    { let _ = scm_sdk::constants::MGMT_CUSTOMER_APP_PATH; }
    // MGMT_CUSTOMER_APPS_TSG_PATH
    { let _ = scm_sdk::constants::MGMT_CUSTOMER_APPS_TSG_PATH; }
    // MGMT_DASHBOARD_APPLICATION_PATH
    { let _ = scm_sdk::constants::MGMT_DASHBOARD_APPLICATION_PATH; }
    // MGMT_DASHBOARD_APPLICATION_VIOLATION_BREAKDOWN_PATH
    { let _ = scm_sdk::constants::MGMT_DASHBOARD_APPLICATION_VIOLATION_BREAKDOWN_PATH; }
    // MGMT_DASHBOARD_APPLICATIONS_OVERVIEW_PATH
    { let _ = scm_sdk::constants::MGMT_DASHBOARD_APPLICATIONS_OVERVIEW_PATH; }
    // MGMT_DASHBOARD_APPLICATIONS_VIOLATIONS_TREND_PATH
    { let _ = scm_sdk::constants::MGMT_DASHBOARD_APPLICATIONS_VIOLATIONS_TREND_PATH; }
    // MGMT_DASHBOARD_APPS_LIST_PATH
    { let _ = scm_sdk::constants::MGMT_DASHBOARD_APPS_LIST_PATH; }
    // MGMT_DASHBOARD_SESSION_PATH
    { let _ = scm_sdk::constants::MGMT_DASHBOARD_SESSION_PATH; }
    // MGMT_DASHBOARD_SESSION_TRANSACTION_PATH
    { let _ = scm_sdk::constants::MGMT_DASHBOARD_SESSION_TRANSACTION_PATH; }
    // MGMT_DASHBOARD_SESSIONS_CHART_PATH
    { let _ = scm_sdk::constants::MGMT_DASHBOARD_SESSIONS_CHART_PATH; }
    // MGMT_DASHBOARD_SESSIONS_OVERVIEW_PATH
    { let _ = scm_sdk::constants::MGMT_DASHBOARD_SESSIONS_OVERVIEW_PATH; }
    // MGMT_DASHBOARD_TOP_APPLICATIONS_VIOLATIONS_PATH
    { let _ = scm_sdk::constants::MGMT_DASHBOARD_TOP_APPLICATIONS_VIOLATIONS_PATH; }
    // MGMT_DEPLOYMENT_PROFILES_PATH
    { let _ = scm_sdk::constants::MGMT_DEPLOYMENT_PROFILES_PATH; }
    // MGMT_DLP_PROFILES_PATH
    { let _ = scm_sdk::constants::MGMT_DLP_PROFILES_PATH; }
    // MGMT_ENDPOINT
    { let _ = scm_sdk::constants::MGMT_ENDPOINT; }
    // MGMT_OAUTH_INVALIDATE_PATH
    { let _ = scm_sdk::constants::MGMT_OAUTH_INVALIDATE_PATH; }
    // MGMT_OAUTH_TOKEN_PATH
    { let _ = scm_sdk::constants::MGMT_OAUTH_TOKEN_PATH; }
    // MGMT_PROFILE_PATH
    { let _ = scm_sdk::constants::MGMT_PROFILE_PATH; }
    // MGMT_PROFILES_TSG_PATH
    { let _ = scm_sdk::constants::MGMT_PROFILES_TSG_PATH; }
    // MGMT_REPORT_SCAN_CONTENT_PATH
    { let _ = scm_sdk::constants::MGMT_REPORT_SCAN_CONTENT_PATH; }
    // MGMT_SCAN_LOGS_PATH
    { let _ = scm_sdk::constants::MGMT_SCAN_LOGS_PATH; }
    // MGMT_TOKEN_ENDPOINT
    { let _ = scm_sdk::constants::MGMT_TOKEN_ENDPOINT; }
    // MGMT_TOPIC_FORCE_PATH
    { let _ = scm_sdk::constants::MGMT_TOPIC_FORCE_PATH; }
    // MGMT_TOPIC_PATH
    { let _ = scm_sdk::constants::MGMT_TOPIC_PATH; }
    // MGMT_TOPICS_TSG_PATH
    { let _ = scm_sdk::constants::MGMT_TOPICS_TSG_PATH; }
    // MGMT_TSG_ID
    { let _ = scm_sdk::constants::MGMT_TSG_ID; }
    // Model
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::Model>; }
    // MODEL_SEC_CLIENT_ID
    { let _ = scm_sdk::constants::MODEL_SEC_CLIENT_ID; }
    // MODEL_SEC_CLIENT_SECRET
    { let _ = scm_sdk::constants::MODEL_SEC_CLIENT_SECRET; }
    // MODEL_SEC_DATA_ENDPOINT
    { let _ = scm_sdk::constants::MODEL_SEC_DATA_ENDPOINT; }
    // MODEL_SEC_EVALUATIONS_PATH
    { let _ = scm_sdk::constants::MODEL_SEC_EVALUATIONS_PATH; }
    // MODEL_SEC_MGMT_ENDPOINT
    { let _ = scm_sdk::constants::MODEL_SEC_MGMT_ENDPOINT; }
    // MODEL_SEC_MODEL_VERSIONS_PATH
    { let _ = scm_sdk::constants::MODEL_SEC_MODEL_VERSIONS_PATH; }
    // MODEL_SEC_MODELS_PATH
    { let _ = scm_sdk::constants::MODEL_SEC_MODELS_PATH; }
    // MODEL_SEC_PYPI_AUTH_PATH
    { let _ = scm_sdk::constants::MODEL_SEC_PYPI_AUTH_PATH; }
    // MODEL_SEC_SCANS_PATH
    { let _ = scm_sdk::constants::MODEL_SEC_SCANS_PATH; }
    // MODEL_SEC_SECURITY_GROUPS_PATH
    { let _ = scm_sdk::constants::MODEL_SEC_SECURITY_GROUPS_PATH; }
    // MODEL_SEC_SECURITY_RULES_PATH
    { let _ = scm_sdk::constants::MODEL_SEC_SECURITY_RULES_PATH; }
    // MODEL_SEC_TOKEN_ENDPOINT
    { let _ = scm_sdk::constants::MODEL_SEC_TOKEN_ENDPOINT; }
    // MODEL_SEC_TSG_ID
    { let _ = scm_sdk::constants::MODEL_SEC_TSG_ID; }
    // MODEL_SEC_VIOLATIONS_PATH
    { let _ = scm_sdk::constants::MODEL_SEC_VIOLATIONS_PATH; }
    // ModelConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::ModelConfiguration>; }
    // ModelConfigurationSchema
    { let _ = scm_sdk::validation::parse("ModelConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ModelList
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ModelList>; }
    // ModelListSchema
    { let _ = scm_sdk::validation::parse("ModelListSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ModelProtectionItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::ModelProtectionItem>; }
    // ModelProtectionItemSchema
    { let _ = scm_sdk::validation::parse("ModelProtectionItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ModelResponseSchema
    { let _ = scm_sdk::validation::parse("ModelResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ModelScanIssue
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ModelScanIssue>; }
    // ModelScanIssueSchema
    { let _ = scm_sdk::validation::parse("ModelScanIssueSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ModelScanStatus
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ModelScanStatus>; }
    // ModelSecurityClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityClient>; let _ = scm_sdk::airs::model_security::ModelSecurityClient::new; }
    // ModelSecurityClient.customRules
    { let _ = scm_sdk::airs::model_security::ModelSecurityClient::custom_rules; }
    // ModelSecurityClient.getPyPIAuth
    { let _ = scm_sdk::airs::model_security::ModelSecurityClient::get_pypi_auth; }
    // ModelSecurityClient.models
    { let _ = scm_sdk::airs::model_security::ModelSecurityClient::models; }
    // ModelSecurityClient.scans
    { let _ = scm_sdk::airs::model_security::ModelSecurityClient::scans; }
    // ModelSecurityClient.securityGroups
    { let _ = scm_sdk::airs::model_security::ModelSecurityClient::security_groups; }
    // ModelSecurityClient.securityRules
    { let _ = scm_sdk::airs::model_security::ModelSecurityClient::security_rules; }
    // ModelSecurityClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // ModelSecurityClientOptions.clientId
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer, "probe", 1, "1"); }; }
    // ModelSecurityClientOptions.clientSecret
    { let _ = scm_sdk::ClientBuilder::oauth_credentials; }
    // ModelSecurityClientOptions.dataEndpoint
    { let _ = scm_sdk::Client::builder; }
    // ModelSecurityClientOptions.mgmtEndpoint
    { let _ = scm_sdk::Client::with_endpoint; }
    // ModelSecurityClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // ModelSecurityClientOptions.tokenEndpoint
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer, "probe", 1, "1"); }; }
    // ModelSecurityClientOptions.tsgId
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer, "probe", 1, "1"); }; }
    // ModelSecurityCustomRulesClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityCustomRulesClient>; let _ = scm_sdk::airs::model_security::ModelSecurityCustomRulesClient::new; }
    // ModelSecurityCustomRulesClient.archive
    { let _ = scm_sdk::airs::model_security::ModelSecurityCustomRulesClient::archive; }
    // ModelSecurityCustomRulesClient.assignSecurityGroups
    { let _ = scm_sdk::airs::model_security::ModelSecurityCustomRulesClient::assign_security_groups; }
    // ModelSecurityCustomRulesClient.create
    { let _ = scm_sdk::airs::model_security::ModelSecurityCustomRulesClient::create; }
    // ModelSecurityCustomRulesClient.get
    { let _ = scm_sdk::airs::model_security::ModelSecurityCustomRulesClient::get; }
    // ModelSecurityCustomRulesClient.list
    { let _ = scm_sdk::airs::model_security::ModelSecurityCustomRulesClient::list; }
    // ModelSecurityCustomRulesClient.listAll
    { let _ = scm_sdk::airs::model_security::ModelSecurityCustomRulesClient::list_all; }
    // ModelSecurityCustomRulesClient.listAllVersions
    { let _ = scm_sdk::airs::model_security::ModelSecurityCustomRulesClient::list_all_versions; }
    // ModelSecurityCustomRulesClient.listSecurityGroups
    { let _ = scm_sdk::airs::model_security::ModelSecurityCustomRulesClient::list_security_groups; }
    // ModelSecurityCustomRulesClient.listVersions
    { let _ = scm_sdk::airs::model_security::ModelSecurityCustomRulesClient::list_versions; }
    // ModelSecurityCustomRulesClient.removeAssignment
    { let _ = scm_sdk::airs::model_security::ModelSecurityCustomRulesClient::remove_assignment; }
    // ModelSecurityCustomRulesClient.unarchive
    { let _ = scm_sdk::airs::model_security::ModelSecurityCustomRulesClient::unarchive; }
    // ModelSecurityCustomRulesClient.update
    { let _ = scm_sdk::airs::model_security::ModelSecurityCustomRulesClient::update; }
    // ModelSecurityCustomRulesClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // ModelSecurityCustomRulesClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // ModelSecurityCustomRulesClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // ModelSecurityCustomRulesClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // ModelSecurityEvaluationListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityEvaluationListOptions>; }
    // ModelSecurityEvaluationListOptions.limit
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityEvaluationListOptions| { let _ = &value.limit; }; }
    // ModelSecurityEvaluationListOptions.result
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityEvaluationListOptions| { let _ = &value.result; }; }
    // ModelSecurityEvaluationListOptions.rule_instance_uuid
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityEvaluationListOptions| { let _ = &value.rule_instance_uuid; }; }
    // ModelSecurityEvaluationListOptions.search
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityEvaluationListOptions| { let _ = &value.search; }; }
    // ModelSecurityEvaluationListOptions.skip
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityEvaluationListOptions| { let _ = &value.skip; }; }
    // ModelSecurityEvaluationListOptions.sort_field
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityEvaluationListOptions| { let _ = &value.sort_field; }; }
    // ModelSecurityEvaluationListOptions.sort_order
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityEvaluationListOptions| { let _ = &value.sort_order; }; }
    // ModelSecurityFileListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityFileListOptions>; }
    // ModelSecurityFileListOptions.limit
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityFileListOptions| { let _ = &value.limit; }; }
    // ModelSecurityFileListOptions.query_path
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityFileListOptions| { let _ = &value.query_path; }; }
    // ModelSecurityFileListOptions.recursive
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityFileListOptions| { let _ = &value.recursive; }; }
    // ModelSecurityFileListOptions.result
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityFileListOptions| { let _ = &value.result; }; }
    // ModelSecurityFileListOptions.search
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityFileListOptions| { let _ = &value.search; }; }
    // ModelSecurityFileListOptions.skip
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityFileListOptions| { let _ = &value.skip; }; }
    // ModelSecurityFileListOptions.sort_dir
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityFileListOptions| { let _ = &value.sort_dir; }; }
    // ModelSecurityFileListOptions.sort_field
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityFileListOptions| { let _ = &value.sort_field; }; }
    // ModelSecurityFileListOptions.type
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityFileListOptions| { let _ = &value.type_; }; }
    // ModelSecurityGroupCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ModelSecurityGroupCreateRequest>; }
    // ModelSecurityGroupCreateRequestSchema
    { let _ = scm_sdk::validation::parse("ModelSecurityGroupCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ModelSecurityGroupListAllOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityGroupListAllOptions>; }
    // ModelSecurityGroupListAllOptions.enabled_rules
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityGroupListAllOptions| { let _ = &value.enabled_rules; }; }
    // ModelSecurityGroupListAllOptions.limit
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityGroupListAllOptions| { let _ = &value.limit; }; }
    // ModelSecurityGroupListAllOptions.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // ModelSecurityGroupListAllOptions.search
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityGroupListAllOptions| { let _ = &value.search; }; }
    // ModelSecurityGroupListAllOptions.search_query
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityGroupListAllOptions| { let _ = &value.search_query; }; }
    // ModelSecurityGroupListAllOptions.sort_dir
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityGroupListAllOptions| { let _ = &value.sort_dir; }; }
    // ModelSecurityGroupListAllOptions.sort_field
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityGroupListAllOptions| { let _ = &value.sort_field; }; }
    // ModelSecurityGroupListAllOptions.source_types
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityGroupListAllOptions| { let _ = &value.source_types; }; }
    // ModelSecurityGroupListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityGroupListOptions>; }
    // ModelSecurityGroupListOptions.enabled_rules
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityGroupListOptions| { let _ = &value.enabled_rules; }; }
    // ModelSecurityGroupListOptions.limit
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityGroupListOptions| { let _ = &value.limit; }; }
    // ModelSecurityGroupListOptions.search
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityGroupListOptions| { let _ = &value.search; }; }
    // ModelSecurityGroupListOptions.search_query
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityGroupListOptions| { let _ = &value.search_query; }; }
    // ModelSecurityGroupListOptions.skip
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityGroupListOptions| { let _ = &value.skip; }; }
    // ModelSecurityGroupListOptions.sort_dir
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityGroupListOptions| { let _ = &value.sort_dir; }; }
    // ModelSecurityGroupListOptions.sort_field
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityGroupListOptions| { let _ = &value.sort_field; }; }
    // ModelSecurityGroupListOptions.source_types
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityGroupListOptions| { let _ = &value.source_types; }; }
    // ModelSecurityGroupResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ModelSecurityGroupResponse>; }
    // ModelSecurityGroupResponseSchema
    { let _ = scm_sdk::validation::parse("ModelSecurityGroupResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ModelSecurityGroupsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityGroupsClient>; let _ = scm_sdk::airs::model_security::ModelSecurityGroupsClient::new; }
    // ModelSecurityGroupsClient.create
    { let _ = scm_sdk::airs::model_security::ModelSecurityGroupsClient::create; }
    // ModelSecurityGroupsClient.delete
    { let _ = scm_sdk::airs::model_security::ModelSecurityGroupsClient::delete; }
    // ModelSecurityGroupsClient.get
    { let _ = scm_sdk::airs::model_security::ModelSecurityGroupsClient::get; }
    // ModelSecurityGroupsClient.getRuleInstance
    { let _ = scm_sdk::airs::model_security::ModelSecurityGroupsClient::get_rule_instance; }
    // ModelSecurityGroupsClient.list
    { let _ = scm_sdk::airs::model_security::ModelSecurityGroupsClient::list; }
    // ModelSecurityGroupsClient.listAll
    { let _ = scm_sdk::airs::model_security::ModelSecurityGroupsClient::list_all; }
    // ModelSecurityGroupsClient.listRuleInstances
    { let _ = scm_sdk::airs::model_security::ModelSecurityGroupsClient::list_rule_instances; }
    // ModelSecurityGroupsClient.listRuleInstanceVersions
    { let _ = scm_sdk::airs::model_security::ModelSecurityGroupsClient::list_rule_instance_versions; }
    // ModelSecurityGroupsClient.update
    { let _ = scm_sdk::airs::model_security::ModelSecurityGroupsClient::update; }
    // ModelSecurityGroupsClient.updateRuleInstance
    { let _ = scm_sdk::airs::model_security::ModelSecurityGroupsClient::update_rule_instance; }
    // ModelSecurityGroupsClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // ModelSecurityGroupsClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // ModelSecurityGroupsClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // ModelSecurityGroupsClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // ModelSecurityGroupState
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ModelSecurityGroupState>; }
    // ModelSecurityGroupUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ModelSecurityGroupUpdateRequest>; }
    // ModelSecurityGroupUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("ModelSecurityGroupUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ModelSecurityLabelListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityLabelListOptions>; }
    // ModelSecurityModelListAllOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityModelListAllOptions>; }
    // ModelSecurityModelListAllOptions.end_time
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListAllOptions| { let _ = &value.end_time; }; }
    // ModelSecurityModelListAllOptions.isBackgroundRefresh
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListAllOptions| { let _ = &value.is_background_refresh; }; }
    // ModelSecurityModelListAllOptions.latest_version_formats
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListAllOptions| { let _ = &value.latest_version_formats; }; }
    // ModelSecurityModelListAllOptions.latest_version_outcomes
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListAllOptions| { let _ = &value.latest_version_outcomes; }; }
    // ModelSecurityModelListAllOptions.latest_version_scan_time_before
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListAllOptions| { let _ = &value.latest_version_scan_time_before; }; }
    // ModelSecurityModelListAllOptions.latest_version_source_types
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListAllOptions| { let _ = &value.latest_version_source_types; }; }
    // ModelSecurityModelListAllOptions.limit
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListAllOptions| { let _ = &value.limit; }; }
    // ModelSecurityModelListAllOptions.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // ModelSecurityModelListAllOptions.search
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListAllOptions| { let _ = &value.search; }; }
    // ModelSecurityModelListAllOptions.search_query
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListAllOptions| { let _ = &value.search_query; }; }
    // ModelSecurityModelListAllOptions.sort_field
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListAllOptions| { let _ = &value.sort_field; }; }
    // ModelSecurityModelListAllOptions.sort_order
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListAllOptions| { let _ = &value.sort_order; }; }
    // ModelSecurityModelListAllOptions.start_time
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListAllOptions| { let _ = &value.start_time; }; }
    // ModelSecurityModelListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityModelListOptions>; }
    // ModelSecurityModelListOptions.end_time
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListOptions| { let _ = &value.end_time; }; }
    // ModelSecurityModelListOptions.isBackgroundRefresh
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListOptions| { let _ = &value.is_background_refresh; }; }
    // ModelSecurityModelListOptions.latest_version_formats
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListOptions| { let _ = &value.latest_version_formats; }; }
    // ModelSecurityModelListOptions.latest_version_outcomes
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListOptions| { let _ = &value.latest_version_outcomes; }; }
    // ModelSecurityModelListOptions.latest_version_scan_time_before
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListOptions| { let _ = &value.latest_version_scan_time_before; }; }
    // ModelSecurityModelListOptions.latest_version_source_types
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListOptions| { let _ = &value.latest_version_source_types; }; }
    // ModelSecurityModelListOptions.limit
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListOptions| { let _ = &value.limit; }; }
    // ModelSecurityModelListOptions.search
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListOptions| { let _ = &value.search; }; }
    // ModelSecurityModelListOptions.search_query
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListOptions| { let _ = &value.search_query; }; }
    // ModelSecurityModelListOptions.skip
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListOptions| { let _ = &value.skip; }; }
    // ModelSecurityModelListOptions.sort_field
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListOptions| { let _ = &value.sort_field; }; }
    // ModelSecurityModelListOptions.sort_order
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListOptions| { let _ = &value.sort_order; }; }
    // ModelSecurityModelListOptions.start_time
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelListOptions| { let _ = &value.start_time; }; }
    // ModelSecurityModelsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityModelsClient>; let _ = scm_sdk::airs::model_security::ModelSecurityModelsClient::new; }
    // ModelSecurityModelsClient.getModel
    { let _ = scm_sdk::airs::model_security::ModelSecurityModelsClient::get_model; }
    // ModelSecurityModelsClient.getModelVersion
    { let _ = scm_sdk::airs::model_security::ModelSecurityModelsClient::get_model_version; }
    // ModelSecurityModelsClient.listAllModels
    { let _ = scm_sdk::airs::model_security::ModelSecurityModelsClient::list_all_models; }
    // ModelSecurityModelsClient.listAllModelVersionFiles
    { let _ = scm_sdk::airs::model_security::ModelSecurityModelsClient::list_all_model_version_files; }
    // ModelSecurityModelsClient.listAllModelVersions
    { let _ = scm_sdk::airs::model_security::ModelSecurityModelsClient::list_all_model_versions; }
    // ModelSecurityModelsClient.listModels
    { let _ = scm_sdk::airs::model_security::ModelSecurityModelsClient::list_models; }
    // ModelSecurityModelsClient.listModelVersionFiles
    { let _ = scm_sdk::airs::model_security::ModelSecurityModelsClient::list_model_version_files; }
    // ModelSecurityModelsClient.listModelVersions
    { let _ = scm_sdk::airs::model_security::ModelSecurityModelsClient::list_model_versions; }
    // ModelSecurityModelsClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // ModelSecurityModelsClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // ModelSecurityModelsClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // ModelSecurityModelsClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // ModelSecurityModelVersionFileListAllOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityModelVersionFileListAllOptions>; }
    // ModelSecurityModelVersionFileListAllOptions.limit
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelVersionFileListAllOptions| { let _ = &value.limit; }; }
    // ModelSecurityModelVersionFileListAllOptions.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // ModelSecurityModelVersionFileListAllOptions.search
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelVersionFileListAllOptions| { let _ = &value.search; }; }
    // ModelSecurityModelVersionFileListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityModelVersionFileListOptions>; }
    // ModelSecurityModelVersionListAllOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityModelVersionListAllOptions>; }
    // ModelSecurityModelVersionListAllOptions.limit
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelVersionListAllOptions| { let _ = &value.limit; }; }
    // ModelSecurityModelVersionListAllOptions.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // ModelSecurityModelVersionListAllOptions.search
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelVersionListAllOptions| { let _ = &value.search; }; }
    // ModelSecurityModelVersionListAllOptions.sort_order
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelVersionListAllOptions| { let _ = &value.sort_order; }; }
    // ModelSecurityModelVersionListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityModelVersionListOptions>; }
    // ModelSecurityModelVersionListOptions.limit
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelVersionListOptions| { let _ = &value.limit; }; }
    // ModelSecurityModelVersionListOptions.search
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelVersionListOptions| { let _ = &value.search; }; }
    // ModelSecurityModelVersionListOptions.skip
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelVersionListOptions| { let _ = &value.skip; }; }
    // ModelSecurityModelVersionListOptions.sort_order
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityModelVersionListOptions| { let _ = &value.sort_order; }; }
    // ModelSecurityPagination
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ModelSecurityPagination>; }
    // ModelSecurityPaginationSchema
    { let _ = scm_sdk::validation::parse("ModelSecurityPaginationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ModelSecurityRuleInstanceListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityRuleInstanceListOptions>; }
    // ModelSecurityRuleInstanceListOptions.generation
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityRuleInstanceListOptions| { let _ = &value.generation; }; }
    // ModelSecurityRuleInstanceListOptions.is_custom
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityRuleInstanceListOptions| { let _ = &value.is_custom; }; }
    // ModelSecurityRuleInstanceListOptions.limit
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityRuleInstanceListOptions| { let _ = &value.limit; }; }
    // ModelSecurityRuleInstanceListOptions.search
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityRuleInstanceListOptions| { let _ = &value.search; }; }
    // ModelSecurityRuleInstanceListOptions.security_rule_uuid
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityRuleInstanceListOptions| { let _ = &value.security_rule_uuid; }; }
    // ModelSecurityRuleInstanceListOptions.skip
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityRuleInstanceListOptions| { let _ = &value.skip; }; }
    // ModelSecurityRuleInstanceListOptions.state
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityRuleInstanceListOptions| { let _ = &value.state; }; }
    // ModelSecurityRuleInstanceResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ModelSecurityRuleInstanceResponse>; }
    // ModelSecurityRuleInstanceResponseSchema
    { let _ = scm_sdk::validation::parse("ModelSecurityRuleInstanceResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ModelSecurityRuleInstanceUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ModelSecurityRuleInstanceUpdateRequest>; }
    // ModelSecurityRuleInstanceUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("ModelSecurityRuleInstanceUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ModelSecurityRuleListAllOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityRuleListAllOptions>; }
    // ModelSecurityRuleListAllOptions.generation
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityRuleListAllOptions| { let _ = &value.generation; }; }
    // ModelSecurityRuleListAllOptions.limit
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityRuleListAllOptions| { let _ = &value.limit; }; }
    // ModelSecurityRuleListAllOptions.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // ModelSecurityRuleListAllOptions.search
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityRuleListAllOptions| { let _ = &value.search; }; }
    // ModelSecurityRuleListAllOptions.search_query
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityRuleListAllOptions| { let _ = &value.search_query; }; }
    // ModelSecurityRuleListAllOptions.source_type
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityRuleListAllOptions| { let _ = &value.source_type; }; }
    // ModelSecurityRuleListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityRuleListOptions>; }
    // ModelSecurityRuleListOptions.generation
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityRuleListOptions| { let _ = &value.generation; }; }
    // ModelSecurityRuleListOptions.limit
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityRuleListOptions| { let _ = &value.limit; }; }
    // ModelSecurityRuleListOptions.search
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityRuleListOptions| { let _ = &value.search; }; }
    // ModelSecurityRuleListOptions.search_query
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityRuleListOptions| { let _ = &value.search_query; }; }
    // ModelSecurityRuleListOptions.skip
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityRuleListOptions| { let _ = &value.skip; }; }
    // ModelSecurityRuleListOptions.source_type
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityRuleListOptions| { let _ = &value.source_type; }; }
    // ModelSecurityRuleResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ModelSecurityRuleResponse>; }
    // ModelSecurityRuleResponseSchema
    { let _ = scm_sdk::validation::parse("ModelSecurityRuleResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ModelSecurityRulesClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityRulesClient>; let _ = scm_sdk::airs::model_security::ModelSecurityRulesClient::new; }
    // ModelSecurityRulesClient.get
    { let _ = scm_sdk::airs::model_security::ModelSecurityRulesClient::get; }
    // ModelSecurityRulesClient.list
    { let _ = scm_sdk::airs::model_security::ModelSecurityRulesClient::list; }
    // ModelSecurityRulesClient.listAll
    { let _ = scm_sdk::airs::model_security::ModelSecurityRulesClient::list_all; }
    // ModelSecurityRulesClient.listVersions
    { let _ = scm_sdk::airs::model_security::ModelSecurityRulesClient::list_versions; }
    // ModelSecurityRulesClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // ModelSecurityRulesClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // ModelSecurityRulesClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // ModelSecurityRulesClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // ModelSecurityScanListAllOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityScanListAllOptions>; }
    // ModelSecurityScanListAllOptions.end_time
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListAllOptions| { let _ = &value.end_time; }; }
    // ModelSecurityScanListAllOptions.eval_outcomes
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListAllOptions| { let _ = &value.eval_outcomes; }; }
    // ModelSecurityScanListAllOptions.isBackgroundRefresh
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListAllOptions| { let _ = &value.is_background_refresh; }; }
    // ModelSecurityScanListAllOptions.labels_query
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListAllOptions| { let _ = &value.labels_query; }; }
    // ModelSecurityScanListAllOptions.limit
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListAllOptions| { let _ = &value.limit; }; }
    // ModelSecurityScanListAllOptions.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // ModelSecurityScanListAllOptions.model_version_uuid
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListAllOptions| { let _ = &value.model_version_uuid; }; }
    // ModelSecurityScanListAllOptions.search
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListAllOptions| { let _ = &value.search; }; }
    // ModelSecurityScanListAllOptions.search_query
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListAllOptions| { let _ = &value.search_query; }; }
    // ModelSecurityScanListAllOptions.security_group_uuid
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListAllOptions| { let _ = &value.security_group_uuid; }; }
    // ModelSecurityScanListAllOptions.sort_by
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListAllOptions| { let _ = &value.sort_by; }; }
    // ModelSecurityScanListAllOptions.sort_order
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListAllOptions| { let _ = &value.sort_order; }; }
    // ModelSecurityScanListAllOptions.source_types
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListAllOptions| { let _ = &value.source_types; }; }
    // ModelSecurityScanListAllOptions.start_time
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListAllOptions| { let _ = &value.start_time; }; }
    // ModelSecurityScanListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityScanListOptions>; }
    // ModelSecurityScanListOptions.end_time
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListOptions| { let _ = &value.end_time; }; }
    // ModelSecurityScanListOptions.eval_outcomes
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListOptions| { let _ = &value.eval_outcomes; }; }
    // ModelSecurityScanListOptions.isBackgroundRefresh
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListOptions| { let _ = &value.is_background_refresh; }; }
    // ModelSecurityScanListOptions.labels_query
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListOptions| { let _ = &value.labels_query; }; }
    // ModelSecurityScanListOptions.limit
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListOptions| { let _ = &value.limit; }; }
    // ModelSecurityScanListOptions.model_version_uuid
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListOptions| { let _ = &value.model_version_uuid; }; }
    // ModelSecurityScanListOptions.search
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListOptions| { let _ = &value.search; }; }
    // ModelSecurityScanListOptions.search_query
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListOptions| { let _ = &value.search_query; }; }
    // ModelSecurityScanListOptions.security_group_uuid
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListOptions| { let _ = &value.security_group_uuid; }; }
    // ModelSecurityScanListOptions.skip
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListOptions| { let _ = &value.skip; }; }
    // ModelSecurityScanListOptions.sort_by
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListOptions| { let _ = &value.sort_by; }; }
    // ModelSecurityScanListOptions.sort_order
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListOptions| { let _ = &value.sort_order; }; }
    // ModelSecurityScanListOptions.source_types
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListOptions| { let _ = &value.source_types; }; }
    // ModelSecurityScanListOptions.start_time
    { let _ = |value: &scm_sdk::airs::model_security::ModelSecurityScanListOptions| { let _ = &value.start_time; }; }
    // ModelSecurityScansClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityScansClient>; let _ = scm_sdk::airs::model_security::ModelSecurityScansClient::new; }
    // ModelSecurityScansClient.addLabels
    { let _ = scm_sdk::airs::model_security::ModelSecurityScansClient::add_labels; }
    // ModelSecurityScansClient.create
    { let _ = scm_sdk::airs::model_security::ModelSecurityScansClient::create; }
    // ModelSecurityScansClient.deleteLabels
    { let _ = scm_sdk::airs::model_security::ModelSecurityScansClient::delete_labels; }
    // ModelSecurityScansClient.get
    { let _ = scm_sdk::airs::model_security::ModelSecurityScansClient::get; }
    // ModelSecurityScansClient.getEvaluation
    { let _ = scm_sdk::airs::model_security::ModelSecurityScansClient::get_evaluation; }
    // ModelSecurityScansClient.getEvaluations
    { let _ = scm_sdk::airs::model_security::ModelSecurityScansClient::get_evaluations; }
    // ModelSecurityScansClient.getFiles
    { let _ = scm_sdk::airs::model_security::ModelSecurityScansClient::get_files; }
    // ModelSecurityScansClient.getLabelKeys
    { let _ = scm_sdk::airs::model_security::ModelSecurityScansClient::get_label_keys; }
    // ModelSecurityScansClient.getLabelValues
    { let _ = scm_sdk::airs::model_security::ModelSecurityScansClient::get_label_values; }
    // ModelSecurityScansClient.getViolation
    { let _ = scm_sdk::airs::model_security::ModelSecurityScansClient::get_violation; }
    // ModelSecurityScansClient.getViolations
    { let _ = scm_sdk::airs::model_security::ModelSecurityScansClient::get_violations; }
    // ModelSecurityScansClient.list
    { let _ = scm_sdk::airs::model_security::ModelSecurityScansClient::list; }
    // ModelSecurityScansClient.listAll
    { let _ = scm_sdk::airs::model_security::ModelSecurityScansClient::list_all; }
    // ModelSecurityScansClient.setLabels
    { let _ = scm_sdk::airs::model_security::ModelSecurityScansClient::set_labels; }
    // ModelSecurityScansClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // ModelSecurityScansClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // ModelSecurityScansClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // ModelSecurityScansClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // ModelSecurityViolationListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::ModelSecurityViolationListOptions>; }
    // ModelVersion
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ModelVersion>; }
    // ModelVersionList
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ModelVersionList>; }
    // ModelVersionListSchema
    { let _ = scm_sdk::validation::parse("ModelVersionListSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ModelVersionResponseSchema
    { let _ = scm_sdk::validation::parse("ModelVersionResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // MSCopilotStudioAuthUrlRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::MSCopilotStudioAuthUrlRequest>; }
    // MSCopilotStudioAuthUrlRequestSchema
    { let _ = scm_sdk::validation::parse("MSCopilotStudioAuthUrlRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // MSCopilotStudioAuthUrlResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::MSCopilotStudioAuthUrlResponse>; }
    // MSCopilotStudioAuthUrlResponseSchema
    { let _ = scm_sdk::validation::parse("MSCopilotStudioAuthUrlResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // MSCopilotStudioTokenRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::MSCopilotStudioTokenRequest>; }
    // MSCopilotStudioTokenRequestSchema
    { let _ = scm_sdk::validation::parse("MSCopilotStudioTokenRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // MSCopilotStudioTokenResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::MSCopilotStudioTokenResponse>; }
    // MSCopilotStudioTokenResponseSchema
    { let _ = scm_sdk::validation::parse("MSCopilotStudioTokenResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // MultiProfileDataNode
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::MultiProfileDataNode>; }
    // MultiProfileDataNodeSchema
    { let _ = scm_sdk::validation::parse("MultiProfileDataNodeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // MultiProfileDetectionRule
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::MultiProfileDetectionRule>; }
    // MultiProfileDetectionRuleSchema
    { let _ = scm_sdk::validation::parse("MultiProfileDetectionRuleSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // MultiTurnConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::MultiTurnConfig>; }
    // MultiTurnConfigSchema
    { let _ = scm_sdk::validation::parse("MultiTurnConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // MultiTurnStatefulConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::MultiTurnStatefulConfig>; }
    // MultiTurnStatefulConfigSchema
    { let _ = scm_sdk::validation::parse("MultiTurnStatefulConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // MultiTurnStatelessConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::MultiTurnStatelessConfig>; }
    // MultiTurnStatelessConfigSchema
    { let _ = scm_sdk::validation::parse("MultiTurnStatelessConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // OAuth2AuthConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::OAuth2AuthConfig>; }
    // OAuth2AuthConfigSchema
    { let _ = scm_sdk::validation::parse("OAuth2AuthConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // Oauth2Token
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::Oauth2Token>; }
    // Oauth2TokenSchema
    { let _ = scm_sdk::validation::parse("Oauth2TokenSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // OAuthClient
    { let _ = std::marker::PhantomData::<scm_sdk::auth::OAuthProvider>; }
    // OAuthClient.clearToken
    { let _ = scm_sdk::auth::OAuthProvider::clear; }
    // OAuthClient.getToken
    { let _ = scm_sdk::auth::OAuthProvider::get_token; }
    // OAuthClient.getTokenInfo
    { let _ = scm_sdk::auth::OAuthProvider::token_info; }
    // OAuthClient.isTokenExpired
    { let _ = scm_sdk::auth::OAuthProvider::is_token_expired; }
    // OAuthClient.isTokenExpiringSoon
    { let _ = scm_sdk::auth::OAuthProvider::is_token_expiring_soon; }
    // OAuthClient.tokenEndpoint
    { let _ = |x: &scm_sdk::auth::OAuthProvider| { let _ = scm_sdk::auth::TokenProvider::context(x).issuer(); }; let _ = scm_sdk::auth::AuthContext::issuer; }
    // OAuthClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::auth::OAuthProviderOptions>; let _ = scm_sdk::auth::OAuthProvider::new_with_options; let _ = std::marker::PhantomData::<scm_sdk::auth::AuthContext>; }
    // OAuthClientOptions.clientId
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer,"fixture",1,"123"); }; }
    // OAuthClientOptions.clientSecret
    { let _ = scm_sdk::auth::OAuthProvider::new_with_options; let _ = scm_sdk::Secret::new("fixture"); }
    // OAuthClientOptions.onTokenRefresh
    { let _ = scm_sdk::auth::OAuthProvider::subscribe_refresh; }
    // OAuthClientOptions.tokenBufferMs
    { let _ = |x: &scm_sdk::auth::OAuthProviderOptions| { let _ = &x.refresh_buffer; }; }
    // OAuthClientOptions.tokenEndpoint
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer,"fixture",1,"123"); }; }
    // OAuthClientOptions.tsgId
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer,"fixture",1,"123"); }; }
    // OAuthManagementClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::OAuthManagementClient>; let _ = scm_sdk::airs::management::OAuthManagementClient::new; }
    // OAuthManagementClient.getAccessToken
    { let _ = scm_sdk::airs::management::OAuthManagementClient::get_access_token; }
    // OAuthManagementClient.invalidateToken
    { let _ = scm_sdk::airs::management::OAuthManagementClient::invalidate_token; }
    // OAuthManagementClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // OAuthManagementClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // OAuthManagementClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // OAuthManagementClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // Offset
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::Offset>; }
    // OffsetSchema
    { let _ = scm_sdk::validation::parse("OffsetSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // OpenAIConnectionParams
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::OpenAIConnectionParams>; }
    // OpenAIConnectionParamsSchema
    { let _ = scm_sdk::validation::parse("OpenAIConnectionParamsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // OrganisationSelfResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::OrganisationSelfResponse>; }
    // OrganisationSelfResponseSchema
    { let _ = scm_sdk::validation::parse("OrganisationSelfResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // Page
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::Page<serde_json::Value>>; }
    // PageableObject
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::PageableObject>; }
    // PageableObjectSchema
    { let _ = scm_sdk::validation::parse("PageableObjectSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PageDataFilteringProfileResponse
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::PageDataFilteringProfileResponse>; }
    // PageDataFilteringProfileResponseSchema
    { let _ = scm_sdk::validation::parse("PageDataFilteringProfileResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PageDataPatternResponse
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::PageDataPatternResponse>; }
    // PageDataPatternResponseSchema
    { let _ = scm_sdk::validation::parse("PageDataPatternResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PageDataProfileResponse
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::PageDataProfileResponse>; }
    // PageDataProfileResponseSchema
    { let _ = scm_sdk::validation::parse("PageDataProfileResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PageDictionaryResponse
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::PageDictionaryResponse>; }
    // PageDictionaryResponseSchema
    { let _ = scm_sdk::validation::parse("PageDictionaryResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // pageSchema
    { let _ = scm_sdk::helpers::page_schema::<serde_json::Value>(&serde_json::json!({"content": []}), |value| Ok(value.clone())); }
    // paginate
    { let options = scm_sdk::RequestOptions::with_timeout(std::time::Duration::from_secs(1)).unwrap(); let _ = scm_sdk::helpers::paginate(0u64, |_, _| async { Ok(scm_sdk::pagination::Page::<String,u64> { items: vec![], next: None }) }, 1, scm_sdk::Session::new(), options); }
    // PaginatedScanResults
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::PaginatedScanResults>; }
    // PaginatedScanResultsSchema
    { let _ = scm_sdk::validation::parse("PaginatedScanResultsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PaginationOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::PaginationOptions>; }
    // PaginationOptions.latest
    { let _ = |value: &scm_sdk::airs::management::PaginationOptions| { let _ = &value.latest; }; }
    // PaginationOptions.limit
    { let _ = |value: &scm_sdk::airs::management::PaginationOptions| { let _ = &value.limit; }; }
    // PaginationOptions.offset
    { let _ = |value: &scm_sdk::airs::management::PaginationOptions| { let _ = &value.offset; }; }
    // PaginationPage
    { let _ = std::marker::PhantomData::<scm_sdk::pagination::Page<scm_sdk::data::JsonValue,u64>>; }
    // PaginationPage.items
    { let _ = |value: &scm_sdk::pagination::Page<scm_sdk::data::JsonValue,u64>| { let _ = &value.items; }; }
    // PaginationPage.next
    { let _ = |value: &scm_sdk::pagination::Page<scm_sdk::data::JsonValue,u64>| { let _ = &value.next; }; }
    // PatternDetection
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::PatternDetection>; }
    // PatternDetectionSchema
    { let _ = scm_sdk::validation::parse("PatternDetectionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PAYLOAD_HASH
    { let _ = scm_sdk::constants::PAYLOAD_HASH; }
    // Policy
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::Policy>; }
    // PolicyAppProtection
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::PolicyAppProtection>; }
    // PolicyAppProtectionSchema
    { let _ = scm_sdk::validation::parse("PolicyAppProtectionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PolicyLatency
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::PolicyLatency>; }
    // PolicyLatencySchema
    { let _ = scm_sdk::validation::parse("PolicyLatencySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PolicySchema
    { let _ = scm_sdk::validation::parse("PolicySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PolicyType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::PolicyType>; }
    // PrerequisiteModel
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::PrerequisiteModel>; }
    // PrerequisiteModelSchema
    { let _ = scm_sdk::validation::parse("PrerequisiteModelSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ProfileListAllOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::ProfileListAllOptions>; }
    // ProfileListAllOptions.latest
    { let _ = |value: &scm_sdk::airs::management::ProfileListAllOptions| { let _ = &value.latest; }; }
    // ProfileListAllOptions.limit
    { let _ = |value: &scm_sdk::airs::management::ProfileListAllOptions| { let _ = &value.limit; }; }
    // ProfileListAllOptions.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // ProfilesClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::ProfilesClient>; let _ = scm_sdk::airs::management::ProfilesClient::new; }
    // ProfilesClient.create
    { let _ = scm_sdk::airs::management::ProfilesClient::create; }
    // ProfilesClient.delete
    { let _ = scm_sdk::airs::management::ProfilesClient::delete; }
    // ProfilesClient.forceDelete
    { let _ = scm_sdk::airs::management::ProfilesClient::force_delete; }
    // ProfilesClient.get
    { let _ = scm_sdk::airs::management::ProfilesClient::get; }
    // ProfilesClient.getByName
    { let _ = scm_sdk::airs::management::ProfilesClient::get_by_name; }
    // ProfilesClient.list
    { let _ = scm_sdk::airs::management::ProfilesClient::list; }
    // ProfilesClient.listAll
    { let _ = scm_sdk::airs::management::ProfilesClient::list_all; }
    // ProfilesClient.listForToken
    { let _ = scm_sdk::airs::management::ProfilesClient::list_for_token; }
    // ProfilesClient.update
    { let _ = scm_sdk::airs::management::ProfilesClient::update; }
    // ProfilesClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // ProfilesClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // ProfilesClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // ProfilesClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // ProfilesClientOptions.tsgId
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer, "probe", 1, "1"); }; }
    // ProfilingStatus
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ProfilingStatus>; }
    // PromptDetailResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::PromptDetailResponse>; }
    // PromptDetailResponseSchema
    { let _ = scm_sdk::validation::parse("PromptDetailResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PromptDetected
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::PromptDetected>; }
    // PromptDetectedSchema
    { let _ = scm_sdk::validation::parse("PromptDetectedSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PromptDetectionDetails
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::PromptDetectionDetails>; }
    // PromptDetectionDetailsSchema
    { let _ = scm_sdk::validation::parse("PromptDetectionDetailsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PromptListAllOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::PromptListOptions>; }
    // PromptListAllOptions.active
    { let _ = |value: &scm_sdk::airs::red_team::PromptListOptions| { let _ = &value.active; }; }
    // PromptListAllOptions.limit
    { let _ = |value: &scm_sdk::airs::red_team::PromptListOptions| { let _ = &value.limit; }; }
    // PromptListAllOptions.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // PromptListAllOptions.search
    { let _ = |value: &scm_sdk::airs::red_team::PromptListOptions| { let _ = &value.search; }; }
    // PromptListAllOptions.status
    { let _ = |value: &scm_sdk::airs::red_team::PromptListOptions| { let _ = &value.status; }; }
    // PromptListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::PromptListOptions>; }
    // PromptListOptions.active
    { let _ = |value: &scm_sdk::airs::red_team::PromptListOptions| { let _ = &value.active; }; }
    // PromptListOptions.limit
    { let _ = |value: &scm_sdk::airs::red_team::PromptListOptions| { let _ = &value.limit; }; }
    // PromptListOptions.search
    { let _ = |value: &scm_sdk::airs::red_team::PromptListOptions| { let _ = &value.search; }; }
    // PromptListOptions.skip
    { let _ = |value: &scm_sdk::airs::red_team::PromptListOptions| { let _ = &value.skip; }; }
    // PromptListOptions.status
    { let _ = |value: &scm_sdk::airs::red_team::PromptListOptions| { let _ = &value.status; }; }
    // PromptsBySetListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::PromptsBySetListOptions>; }
    // PromptsBySetListOptions.is_threat
    { let _ = |value: &scm_sdk::airs::red_team::PromptsBySetListOptions| { let _ = &value.is_threat; }; }
    // PromptsBySetListOptions.limit
    { let _ = |value: &scm_sdk::airs::red_team::PromptsBySetListOptions| { let _ = &value.limit; }; }
    // PromptsBySetListOptions.search
    { let _ = |value: &scm_sdk::airs::red_team::PromptsBySetListOptions| { let _ = &value.search; }; }
    // PromptsBySetListOptions.skip
    { let _ = |value: &scm_sdk::airs::red_team::PromptsBySetListOptions| { let _ = &value.skip; }; }
    // PromptSetListAllOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::PromptSetListOptions>; }
    // PromptSetListAllOptions.active
    { let _ = |value: &scm_sdk::airs::red_team::PromptSetListOptions| { let _ = &value.active; }; }
    // PromptSetListAllOptions.archive
    { let _ = |value: &scm_sdk::airs::red_team::PromptSetListOptions| { let _ = &value.archive; }; }
    // PromptSetListAllOptions.language
    { let _ = |value: &scm_sdk::airs::red_team::PromptSetListOptions| { let _ = &value.language; }; }
    // PromptSetListAllOptions.limit
    { let _ = |value: &scm_sdk::airs::red_team::PromptSetListOptions| { let _ = &value.limit; }; }
    // PromptSetListAllOptions.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // PromptSetListAllOptions.search
    { let _ = |value: &scm_sdk::airs::red_team::PromptSetListOptions| { let _ = &value.search; }; }
    // PromptSetListAllOptions.status
    { let _ = |value: &scm_sdk::airs::red_team::PromptSetListOptions| { let _ = &value.status; }; }
    // PromptSetListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::PromptSetListOptions>; }
    // PromptSetListOptions.active
    { let _ = |value: &scm_sdk::airs::red_team::PromptSetListOptions| { let _ = &value.active; }; }
    // PromptSetListOptions.archive
    { let _ = |value: &scm_sdk::airs::red_team::PromptSetListOptions| { let _ = &value.archive; }; }
    // PromptSetListOptions.language
    { let _ = |value: &scm_sdk::airs::red_team::PromptSetListOptions| { let _ = &value.language; }; }
    // PromptSetListOptions.limit
    { let _ = |value: &scm_sdk::airs::red_team::PromptSetListOptions| { let _ = &value.limit; }; }
    // PromptSetListOptions.search
    { let _ = |value: &scm_sdk::airs::red_team::PromptSetListOptions| { let _ = &value.search; }; }
    // PromptSetListOptions.skip
    { let _ = |value: &scm_sdk::airs::red_team::PromptSetListOptions| { let _ = &value.skip; }; }
    // PromptSetListOptions.status
    { let _ = |value: &scm_sdk::airs::red_team::PromptSetListOptions| { let _ = &value.status; }; }
    // PromptSetsReportOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::PromptSetsReportOptions>; }
    // PromptSetsReportOptions.is_threat
    { let _ = |value: &scm_sdk::airs::red_team::PromptSetsReportOptions| { let _ = &value.is_threat; }; }
    // PromptSetsReportOptions.limit
    { let _ = |value: &scm_sdk::airs::red_team::PromptSetsReportOptions| { let _ = &value.limit; }; }
    // PromptSetsReportOptions.property_filters
    { let _ = |value: &scm_sdk::airs::red_team::PromptSetsReportOptions| { let _ = &value.property_filters; }; }
    // PromptSetsReportOptions.search
    { let _ = |value: &scm_sdk::airs::red_team::PromptSetsReportOptions| { let _ = &value.search; }; }
    // PromptSetsReportOptions.skip
    { let _ = |value: &scm_sdk::airs::red_team::PromptSetsReportOptions| { let _ = &value.skip; }; }
    // PromptSetsReportResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::PromptSetsReportResponse>; }
    // PromptSetsReportResponseSchema
    { let _ = scm_sdk::validation::parse("PromptSetsReportResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PromptSetStats
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::PromptSetStats>; }
    // PromptSetStatsSchema
    { let _ = scm_sdk::validation::parse("PromptSetStatsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PromptSetSummary
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::PromptSetSummary>; }
    // PromptSetSummarySchema
    { let _ = scm_sdk::validation::parse("PromptSetSummarySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PropertyAssignment
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::PropertyAssignment>; }
    // PropertyAssignmentSchema
    { let _ = scm_sdk::validation::parse("PropertyAssignmentSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PropertyDefinition
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::PropertyDefinition>; }
    // PropertyDefinitionSchema
    { let _ = scm_sdk::validation::parse("PropertyDefinitionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PropertyNameCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::PropertyNameCreateRequest>; }
    // PropertyNameCreateRequestSchema
    { let _ = scm_sdk::validation::parse("PropertyNameCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PropertyNameCreateResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::PropertyNameCreateResponse>; }
    // PropertyNameCreateResponseSchema
    { let _ = scm_sdk::validation::parse("PropertyNameCreateResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PropertyNamesListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::PropertyNamesListResponse>; }
    // PropertyNamesListResponseSchema
    { let _ = scm_sdk::validation::parse("PropertyNamesListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PropertyStatistic
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::PropertyStatistic>; }
    // PropertyStatisticResult
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::PropertyStatisticResult>; }
    // PropertyStatisticResultSchema
    { let _ = scm_sdk::validation::parse("PropertyStatisticResultSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PropertyStatisticSchema
    { let _ = scm_sdk::validation::parse("PropertyStatisticSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PropertyValueCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::PropertyValueCreateRequest>; }
    // PropertyValueCreateRequestSchema
    { let _ = scm_sdk::validation::parse("PropertyValueCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PropertyValuesMultipleResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::PropertyValuesMultipleResponse>; }
    // PropertyValuesMultipleResponseSchema
    { let _ = scm_sdk::validation::parse("PropertyValuesMultipleResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PropertyValuesResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::PropertyValuesResponse>; }
    // PropertyValuesResponseSchema
    { let _ = scm_sdk::validation::parse("PropertyValuesResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PropertyValueStatistic
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::PropertyValueStatistic>; }
    // PropertyValueStatisticSchema
    { let _ = scm_sdk::validation::parse("PropertyValueStatisticSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // PyPIAuthResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::PyPIAuthResponse>; }
    // PyPIAuthResponseSchema
    { let _ = scm_sdk::validation::parse("PyPIAuthResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // QuotaDetails
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::QuotaDetails>; }
    // QuotaDetailsSchema
    { let _ = scm_sdk::validation::parse("QuotaDetailsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // QuotaSummary
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::QuotaSummary>; }
    // QuotaSummarySchema
    { let _ = scm_sdk::validation::parse("QuotaSummarySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RED_TEAM_ADAPTER_PATH
    { let _ = scm_sdk::constants::RED_TEAM_ADAPTER_PATH; }
    // RED_TEAM_ADAPTER_VALIDATE_PATH
    { let _ = scm_sdk::constants::RED_TEAM_ADAPTER_VALIDATE_PATH; }
    // RED_TEAM_CATEGORIES_PATH
    { let _ = scm_sdk::constants::RED_TEAM_CATEGORIES_PATH; }
    // RED_TEAM_CHANNELS_PATH
    { let _ = scm_sdk::constants::RED_TEAM_CHANNELS_PATH; }
    // RED_TEAM_CHANNELS_STATS_PATH
    { let _ = scm_sdk::constants::RED_TEAM_CHANNELS_STATS_PATH; }
    // RED_TEAM_CLIENT_ID
    { let _ = scm_sdk::constants::RED_TEAM_CLIENT_ID; }
    // RED_TEAM_CLIENT_SECRET
    { let _ = scm_sdk::constants::RED_TEAM_CLIENT_SECRET; }
    // RED_TEAM_CUSTOM_ATTACK_PATH
    { let _ = scm_sdk::constants::RED_TEAM_CUSTOM_ATTACK_PATH; }
    // RED_TEAM_CUSTOM_ATTACKS_REPORT_PATH
    { let _ = scm_sdk::constants::RED_TEAM_CUSTOM_ATTACKS_REPORT_PATH; }
    // RED_TEAM_DASHBOARD_PATH
    { let _ = scm_sdk::constants::RED_TEAM_DASHBOARD_PATH; }
    // RED_TEAM_DATA_ENDPOINT
    { let _ = scm_sdk::constants::RED_TEAM_DATA_ENDPOINT; }
    // RED_TEAM_ERROR_LOG_PATH
    { let _ = scm_sdk::constants::RED_TEAM_ERROR_LOG_PATH; }
    // RED_TEAM_ERROR_LOG_TARGET_PROFILE_PATH
    { let _ = scm_sdk::constants::RED_TEAM_ERROR_LOG_TARGET_PROFILE_PATH; }
    // RED_TEAM_EULA_PATH
    { let _ = scm_sdk::constants::RED_TEAM_EULA_PATH; }
    // RED_TEAM_INSTANCES_PATH
    { let _ = scm_sdk::constants::RED_TEAM_INSTANCES_PATH; }
    // RED_TEAM_LANGUAGES_PATH
    { let _ = scm_sdk::constants::RED_TEAM_LANGUAGES_PATH; }
    // RED_TEAM_MGMT_DASHBOARD_PATH
    { let _ = scm_sdk::constants::RED_TEAM_MGMT_DASHBOARD_PATH; }
    // RED_TEAM_MGMT_ENDPOINT
    { let _ = scm_sdk::constants::RED_TEAM_MGMT_ENDPOINT; }
    // RED_TEAM_NETWORK_BROKER_ENDPOINT
    { let _ = scm_sdk::constants::RED_TEAM_NETWORK_BROKER_ENDPOINT; }
    // RED_TEAM_QUOTA_PATH
    { let _ = scm_sdk::constants::RED_TEAM_QUOTA_PATH; }
    // RED_TEAM_REGISTRY_CREDENTIALS_PATH
    { let _ = scm_sdk::constants::RED_TEAM_REGISTRY_CREDENTIALS_PATH; }
    // RED_TEAM_REPORT_DYNAMIC_PATH
    { let _ = scm_sdk::constants::RED_TEAM_REPORT_DYNAMIC_PATH; }
    // RED_TEAM_REPORT_PATH
    { let _ = scm_sdk::constants::RED_TEAM_REPORT_PATH; }
    // RED_TEAM_REPORT_STATIC_PATH
    { let _ = scm_sdk::constants::RED_TEAM_REPORT_STATIC_PATH; }
    // RED_TEAM_SCAN_PATH
    { let _ = scm_sdk::constants::RED_TEAM_SCAN_PATH; }
    // RED_TEAM_SENTIMENT_PATH
    { let _ = scm_sdk::constants::RED_TEAM_SENTIMENT_PATH; }
    // RED_TEAM_TARGET_PATH
    { let _ = scm_sdk::constants::RED_TEAM_TARGET_PATH; }
    // RED_TEAM_TARGET_VALIDATE_AUTH_PATH
    { let _ = scm_sdk::constants::RED_TEAM_TARGET_VALIDATE_AUTH_PATH; }
    // RED_TEAM_TEMPLATE_PATH
    { let _ = scm_sdk::constants::RED_TEAM_TEMPLATE_PATH; }
    // RED_TEAM_TOKEN_ENDPOINT
    { let _ = scm_sdk::constants::RED_TEAM_TOKEN_ENDPOINT; }
    // RED_TEAM_TSG_ID
    { let _ = scm_sdk::constants::RED_TEAM_TSG_ID; }
    // redactAIGatewaySecrets
    { let _ = scm_sdk::airs::gateway::redact_ai_gateway_secrets; }
    // RedTeamAdaptersClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::RedTeamAdaptersClient>; let _ = scm_sdk::airs::red_team::RedTeamAdaptersClient::new; }
    // RedTeamAdaptersClient.create
    { let _ = scm_sdk::airs::red_team::RedTeamAdaptersClient::create; }
    // RedTeamAdaptersClient.delete
    { let _ = scm_sdk::airs::red_team::RedTeamAdaptersClient::delete; }
    // RedTeamAdaptersClient.get
    { let _ = scm_sdk::airs::red_team::RedTeamAdaptersClient::get; }
    // RedTeamAdaptersClient.getConfig
    { let _ = scm_sdk::airs::red_team::RedTeamAdaptersClient::get_config; }
    // RedTeamAdaptersClient.list
    { let _ = scm_sdk::airs::red_team::RedTeamAdaptersClient::list; }
    // RedTeamAdaptersClient.listAll
    { let _ = scm_sdk::airs::red_team::RedTeamAdaptersClient::list_all; let _ = scm_sdk::airs::red_team::RedTeamAdaptersClient::list_pages; let _ = scm_sdk::airs::red_team::RedTeamAdaptersClient::list_items; }
    // RedTeamAdaptersClient.update
    { let _ = scm_sdk::airs::red_team::RedTeamAdaptersClient::update; }
    // RedTeamAdaptersClient.validate
    { let _ = scm_sdk::airs::red_team::RedTeamAdaptersClient::validate; }
    // RedTeamAdaptersClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // RedTeamAdaptersClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // RedTeamAdaptersClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // RedTeamAdaptersClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // RedTeamCategory
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::RedTeamCategory>; }
    // RedTeamClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::RedTeamClient>; let _ = scm_sdk::airs::red_team::RedTeamClient::new; }
    // RedTeamClient.adapters
    { let _ = scm_sdk::airs::red_team::RedTeamClient::adapters; }
    // RedTeamClient.customAttackReports
    { let _ = scm_sdk::airs::red_team::RedTeamClient::custom_attack_reports; }
    // RedTeamClient.customAttacks
    { let _ = scm_sdk::airs::red_team::RedTeamClient::custom_attacks; }
    // RedTeamClient.eula
    { let _ = scm_sdk::airs::red_team::RedTeamClient::eula; }
    // RedTeamClient.getDashboardOverview
    { let _ = scm_sdk::airs::red_team::RedTeamClient::get_dashboard_overview; }
    // RedTeamClient.getErrorLogs
    { let _ = scm_sdk::airs::red_team::RedTeamClient::get_error_logs; }
    // RedTeamClient.getGoalCategories
    { let _ = scm_sdk::airs::red_team::RedTeamClient::get_goal_categories; }
    // RedTeamClient.getLanguages
    { let _ = scm_sdk::airs::red_team::RedTeamClient::get_languages; }
    // RedTeamClient.getManagementLanguages
    { let _ = scm_sdk::airs::red_team::RedTeamClient::get_management_languages; }
    // RedTeamClient.getQuota
    { let _ = scm_sdk::airs::red_team::RedTeamClient::get_quota; }
    // RedTeamClient.getQuotaSummary
    { let _ = scm_sdk::airs::red_team::RedTeamClient::get_quota_summary; }
    // RedTeamClient.getScanMetadata
    { let _ = scm_sdk::airs::red_team::RedTeamClient::get_scan_metadata; }
    // RedTeamClient.getScanStatistics
    { let _ = scm_sdk::airs::red_team::RedTeamClient::get_scan_statistics; }
    // RedTeamClient.getScoreTrend
    { let _ = scm_sdk::airs::red_team::RedTeamClient::get_score_trend; }
    // RedTeamClient.getSentiment
    { let _ = scm_sdk::airs::red_team::RedTeamClient::get_sentiment; }
    // RedTeamClient.getTargetProfileErrorLogs
    { let _ = scm_sdk::airs::red_team::RedTeamClient::get_target_profile_error_logs; }
    // RedTeamClient.instances
    { let _ = scm_sdk::airs::red_team::RedTeamClient::instances; }
    // RedTeamClient.networkBroker
    { let _ = scm_sdk::airs::red_team::RedTeamClient::network_broker; }
    // RedTeamClient.reports
    { let _ = scm_sdk::airs::red_team::RedTeamClient::reports; }
    // RedTeamClient.scans
    { let _ = scm_sdk::airs::red_team::RedTeamClient::scans; }
    // RedTeamClient.targets
    { let _ = scm_sdk::airs::red_team::RedTeamClient::targets; }
    // RedTeamClient.updateSentiment
    { let _ = scm_sdk::airs::red_team::RedTeamClient::update_sentiment; }
    // RedTeamClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // RedTeamClientOptions.clientId
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer, "probe", 1, "1"); }; }
    // RedTeamClientOptions.clientSecret
    { let _ = scm_sdk::ClientBuilder::oauth_credentials; }
    // RedTeamClientOptions.dataEndpoint
    { let _ = scm_sdk::Client::builder; }
    // RedTeamClientOptions.mgmtEndpoint
    { let _ = scm_sdk::Client::with_endpoint; }
    // RedTeamClientOptions.networkBrokerEndpoint
    { let _ = scm_sdk::Client::with_endpoint; }
    // RedTeamClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // RedTeamClientOptions.tokenEndpoint
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer, "probe", 1, "1"); }; }
    // RedTeamClientOptions.tsgId
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer, "probe", 1, "1"); }; }
    // RedTeamCustomAttackReportsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::RedTeamCustomAttackReportsClient>; let _ = scm_sdk::airs::red_team::RedTeamCustomAttackReportsClient::new; }
    // RedTeamCustomAttackReportsClient.getAttackOutputs
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttackReportsClient::get_attack_outputs; }
    // RedTeamCustomAttackReportsClient.getPromptDetail
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttackReportsClient::get_prompt_detail; }
    // RedTeamCustomAttackReportsClient.getPromptsBySet
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttackReportsClient::get_prompts_by_set; }
    // RedTeamCustomAttackReportsClient.getPromptSets
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttackReportsClient::get_prompt_sets; }
    // RedTeamCustomAttackReportsClient.getPropertyStats
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttackReportsClient::get_property_stats; }
    // RedTeamCustomAttackReportsClient.getReport
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttackReportsClient::get_report; }
    // RedTeamCustomAttackReportsClient.listCustomAttacks
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttackReportsClient::list_custom_attacks; }
    // RedTeamCustomAttackReportsClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // RedTeamCustomAttackReportsClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // RedTeamCustomAttackReportsClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // RedTeamCustomAttackReportsClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // RedTeamCustomAttacksClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::RedTeamCustomAttacksClient>; let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::new; }
    // RedTeamCustomAttacksClient.archivePromptSet
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::archive_prompt_set; }
    // RedTeamCustomAttacksClient.createPrompt
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::create_prompt; }
    // RedTeamCustomAttacksClient.createPromptSet
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::create_prompt_set; }
    // RedTeamCustomAttacksClient.createPropertyName
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::create_property_name; }
    // RedTeamCustomAttacksClient.createPropertyValue
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::create_property_value; }
    // RedTeamCustomAttacksClient.deletePrompt
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::delete_prompt; }
    // RedTeamCustomAttacksClient.downloadTemplate
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::download_template; }
    // RedTeamCustomAttacksClient.getPrompt
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::get_prompt; }
    // RedTeamCustomAttacksClient.getPromptSet
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::get_prompt_set; }
    // RedTeamCustomAttacksClient.getPromptSetReference
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::get_prompt_set_reference; }
    // RedTeamCustomAttacksClient.getPromptSetVersionInfo
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::get_prompt_set_version_info; }
    // RedTeamCustomAttacksClient.getPropertyNames
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::get_property_names; }
    // RedTeamCustomAttacksClient.getPropertyValues
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::get_property_values; }
    // RedTeamCustomAttacksClient.getPropertyValuesMultiple
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::get_property_values_multiple; }
    // RedTeamCustomAttacksClient.listActivePromptSets
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::list_active_prompt_sets; }
    // RedTeamCustomAttacksClient.listAllPrompts
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::list_all_prompts; let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::prompt_pages; let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::prompt_items; }
    // RedTeamCustomAttacksClient.listAllPromptSets
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::list_all_prompt_sets; let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::prompt_set_pages; let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::prompt_set_items; }
    // RedTeamCustomAttacksClient.listPrompts
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::list_prompts; }
    // RedTeamCustomAttacksClient.listPromptSets
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::list_prompt_sets; }
    // RedTeamCustomAttacksClient.updatePrompt
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::update_prompt; }
    // RedTeamCustomAttacksClient.updatePromptSet
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::update_prompt_set; }
    // RedTeamCustomAttacksClient.uploadPromptsCsv
    { let _ = scm_sdk::airs::red_team::RedTeamCustomAttacksClient::upload_prompts_csv; }
    // RedTeamCustomAttacksClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // RedTeamCustomAttacksClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // RedTeamCustomAttacksClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // RedTeamCustomAttacksClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // RedTeamErrorType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::RedTeamErrorType>; }
    // RedTeamEulaClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::RedTeamEulaClient>; let _ = scm_sdk::airs::red_team::RedTeamEulaClient::new; }
    // RedTeamEulaClient.accept
    { let _ = scm_sdk::airs::red_team::RedTeamEulaClient::accept; }
    // RedTeamEulaClient.getContent
    { let _ = scm_sdk::airs::red_team::RedTeamEulaClient::get_content; }
    // RedTeamEulaClient.getStatus
    { let _ = scm_sdk::airs::red_team::RedTeamEulaClient::get_status; }
    // RedTeamEulaClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // RedTeamEulaClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // RedTeamEulaClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // RedTeamEulaClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // RedTeamInstancesClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::RedTeamInstancesClient>; let _ = scm_sdk::airs::red_team::RedTeamInstancesClient::new; }
    // RedTeamInstancesClient.createDevices
    { let _ = scm_sdk::airs::red_team::RedTeamInstancesClient::create_devices; }
    // RedTeamInstancesClient.createInstance
    { let _ = scm_sdk::airs::red_team::RedTeamInstancesClient::create_instance; }
    // RedTeamInstancesClient.deleteDevices
    { let _ = scm_sdk::airs::red_team::RedTeamInstancesClient::delete_devices; }
    // RedTeamInstancesClient.deleteInstance
    { let _ = scm_sdk::airs::red_team::RedTeamInstancesClient::delete_instance; }
    // RedTeamInstancesClient.getInstance
    { let _ = scm_sdk::airs::red_team::RedTeamInstancesClient::get_instance; }
    // RedTeamInstancesClient.getRegistryCredentials
    { let _ = scm_sdk::airs::red_team::RedTeamInstancesClient::get_registry_credentials; }
    // RedTeamInstancesClient.updateDevices
    { let _ = scm_sdk::airs::red_team::RedTeamInstancesClient::update_devices; }
    // RedTeamInstancesClient.updateInstance
    { let _ = scm_sdk::airs::red_team::RedTeamInstancesClient::update_instance; }
    // RedTeamInstancesClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // RedTeamInstancesClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // RedTeamInstancesClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // RedTeamInstancesClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // RedTeamListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::RedTeamListOptions>; }
    // RedTeamNetworkBrokerClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::RedTeamNetworkBrokerClient>; let _ = scm_sdk::airs::red_team::RedTeamNetworkBrokerClient::new; }
    // RedTeamNetworkBrokerClient.createChannel
    { let _ = scm_sdk::airs::red_team::RedTeamNetworkBrokerClient::create_channel; }
    // RedTeamNetworkBrokerClient.getChannel
    { let _ = scm_sdk::airs::red_team::RedTeamNetworkBrokerClient::get_channel; }
    // RedTeamNetworkBrokerClient.getChannelStats
    { let _ = scm_sdk::airs::red_team::RedTeamNetworkBrokerClient::get_channel_stats; }
    // RedTeamNetworkBrokerClient.listChannels
    { let _ = scm_sdk::airs::red_team::RedTeamNetworkBrokerClient::list_channels; }
    // RedTeamNetworkBrokerClient.updateChannel
    { let _ = scm_sdk::airs::red_team::RedTeamNetworkBrokerClient::update_channel; }
    // RedTeamNetworkBrokerClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // RedTeamNetworkBrokerClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // RedTeamNetworkBrokerClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // RedTeamNetworkBrokerClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // RedTeamPagination
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::RedTeamPagination>; }
    // RedTeamPaginationSchema
    { let _ = scm_sdk::validation::parse("RedTeamPaginationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RedTeamReportsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::RedTeamReportsClient>; let _ = scm_sdk::airs::red_team::RedTeamReportsClient::new; }
    // RedTeamReportsClient.downloadReport
    { let _ = scm_sdk::airs::red_team::RedTeamReportsClient::download_report; }
    // RedTeamReportsClient.generatePartialReport
    { let _ = scm_sdk::airs::red_team::RedTeamReportsClient::generate_partial_report; }
    // RedTeamReportsClient.getAttackDetail
    { let _ = scm_sdk::airs::red_team::RedTeamReportsClient::get_attack_detail; }
    // RedTeamReportsClient.getDynamicRemediation
    { let _ = scm_sdk::airs::red_team::RedTeamReportsClient::get_dynamic_remediation; }
    // RedTeamReportsClient.getDynamicReport
    { let _ = scm_sdk::airs::red_team::RedTeamReportsClient::get_dynamic_report; }
    // RedTeamReportsClient.getDynamicRuntimePolicy
    { let _ = scm_sdk::airs::red_team::RedTeamReportsClient::get_dynamic_runtime_policy; }
    // RedTeamReportsClient.getMultiTurnAttackDetail
    { let _ = scm_sdk::airs::red_team::RedTeamReportsClient::get_multi_turn_attack_detail; }
    // RedTeamReportsClient.getStaticRemediation
    { let _ = scm_sdk::airs::red_team::RedTeamReportsClient::get_static_remediation; }
    // RedTeamReportsClient.getStaticReport
    { let _ = scm_sdk::airs::red_team::RedTeamReportsClient::get_static_report; }
    // RedTeamReportsClient.getStaticRuntimePolicy
    { let _ = scm_sdk::airs::red_team::RedTeamReportsClient::get_static_runtime_policy; }
    // RedTeamReportsClient.getStreamDetail
    { let _ = scm_sdk::airs::red_team::RedTeamReportsClient::get_stream_detail; }
    // RedTeamReportsClient.listAttacks
    { let _ = scm_sdk::airs::red_team::RedTeamReportsClient::list_attacks; }
    // RedTeamReportsClient.listGoals
    { let _ = scm_sdk::airs::red_team::RedTeamReportsClient::list_goals; }
    // RedTeamReportsClient.listGoalStreams
    { let _ = scm_sdk::airs::red_team::RedTeamReportsClient::list_goal_streams; }
    // RedTeamReportsClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // RedTeamReportsClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // RedTeamReportsClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // RedTeamReportsClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // RedTeamScanListAllOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::RedTeamScanListOptions>; }
    // RedTeamScanListAllOptions.job_type
    { let _ = |value: &scm_sdk::airs::red_team::RedTeamScanListOptions| { let _ = &value.job_type; }; }
    // RedTeamScanListAllOptions.limit
    { let _ = |value: &scm_sdk::airs::red_team::RedTeamScanListOptions| { let _ = &value.limit; }; }
    // RedTeamScanListAllOptions.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // RedTeamScanListAllOptions.search
    { let _ = |value: &scm_sdk::airs::red_team::RedTeamScanListOptions| { let _ = &value.search; }; }
    // RedTeamScanListAllOptions.status
    { let _ = |value: &scm_sdk::airs::red_team::RedTeamScanListOptions| { let _ = &value.status; }; }
    // RedTeamScanListAllOptions.target_id
    { let _ = |value: &scm_sdk::airs::red_team::RedTeamScanListOptions| { let _ = &value.target_id; }; }
    // RedTeamScanListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::RedTeamScanListOptions>; }
    // RedTeamScanListOptions.job_type
    { let _ = |value: &scm_sdk::airs::red_team::RedTeamScanListOptions| { let _ = &value.job_type; }; }
    // RedTeamScanListOptions.limit
    { let _ = |value: &scm_sdk::airs::red_team::RedTeamScanListOptions| { let _ = &value.limit; }; }
    // RedTeamScanListOptions.search
    { let _ = |value: &scm_sdk::airs::red_team::RedTeamScanListOptions| { let _ = &value.search; }; }
    // RedTeamScanListOptions.skip
    { let _ = |value: &scm_sdk::airs::red_team::RedTeamScanListOptions| { let _ = &value.skip; }; }
    // RedTeamScanListOptions.status
    { let _ = |value: &scm_sdk::airs::red_team::RedTeamScanListOptions| { let _ = &value.status; }; }
    // RedTeamScanListOptions.target_id
    { let _ = |value: &scm_sdk::airs::red_team::RedTeamScanListOptions| { let _ = &value.target_id; }; }
    // RedTeamScansClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::RedTeamScansClient>; let _ = scm_sdk::airs::red_team::RedTeamScansClient::new; }
    // RedTeamScansClient.abort
    { let _ = scm_sdk::airs::red_team::RedTeamScansClient::abort; }
    // RedTeamScansClient.create
    { let _ = scm_sdk::airs::red_team::RedTeamScansClient::create; }
    // RedTeamScansClient.get
    { let _ = scm_sdk::airs::red_team::RedTeamScansClient::get; }
    // RedTeamScansClient.getCategories
    { let _ = scm_sdk::airs::red_team::RedTeamScansClient::get_categories; }
    // RedTeamScansClient.list
    { let _ = scm_sdk::airs::red_team::RedTeamScansClient::list; }
    // RedTeamScansClient.listAll
    { let _ = scm_sdk::airs::red_team::RedTeamScansClient::list_all; }
    // RedTeamScansClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // RedTeamScansClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // RedTeamScansClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // RedTeamScansClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // RedTeamTargetsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::RedTeamTargetsClient>; let _ = scm_sdk::airs::red_team::RedTeamTargetsClient::new; }
    // RedTeamTargetsClient.create
    { let _ = scm_sdk::airs::red_team::RedTeamTargetsClient::create; }
    // RedTeamTargetsClient.delete
    { let _ = scm_sdk::airs::red_team::RedTeamTargetsClient::delete; }
    // RedTeamTargetsClient.deleteCopilotToken
    { let _ = scm_sdk::airs::red_team::RedTeamTargetsClient::delete_copilot_token; }
    // RedTeamTargetsClient.exchangeCopilotToken
    { let _ = scm_sdk::airs::red_team::RedTeamTargetsClient::exchange_copilot_token; }
    // RedTeamTargetsClient.get
    { let _ = scm_sdk::airs::red_team::RedTeamTargetsClient::get; }
    // RedTeamTargetsClient.getCopilotAuthUrl
    { let _ = scm_sdk::airs::red_team::RedTeamTargetsClient::get_copilot_auth_url; }
    // RedTeamTargetsClient.getProfile
    { let _ = scm_sdk::airs::red_team::RedTeamTargetsClient::get_profile; }
    // RedTeamTargetsClient.getTargetMetadata
    { let _ = scm_sdk::airs::red_team::RedTeamTargetsClient::get_target_metadata; }
    // RedTeamTargetsClient.getTargetTemplates
    { let _ = scm_sdk::airs::red_team::RedTeamTargetsClient::get_target_templates; }
    // RedTeamTargetsClient.list
    { let _ = scm_sdk::airs::red_team::RedTeamTargetsClient::list; }
    // RedTeamTargetsClient.listAll
    { let _ = scm_sdk::airs::red_team::RedTeamTargetsClient::list_all; let _ = scm_sdk::airs::red_team::RedTeamTargetsClient::list_pages; let _ = scm_sdk::airs::red_team::RedTeamTargetsClient::list_items; }
    // RedTeamTargetsClient.probe
    { let _ = scm_sdk::airs::red_team::RedTeamTargetsClient::probe; }
    // RedTeamTargetsClient.startProfiling
    { let _ = scm_sdk::airs::red_team::RedTeamTargetsClient::start_profiling; }
    // RedTeamTargetsClient.update
    { let _ = scm_sdk::airs::red_team::RedTeamTargetsClient::update; }
    // RedTeamTargetsClient.updateProfile
    { let _ = scm_sdk::airs::red_team::RedTeamTargetsClient::update_profile; }
    // RedTeamTargetsClient.validateAuth
    { let _ = scm_sdk::airs::red_team::RedTeamTargetsClient::validate_auth; }
    // RedTeamTargetsClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // RedTeamTargetsClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // RedTeamTargetsClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // RedTeamTargetsClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // RegistryCredentials
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::RegistryCredentials>; }
    // RegistryCredentialsSchema
    { let _ = scm_sdk::validation::parse("RegistryCredentialsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RemediationDetail
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::RemediationDetail>; }
    // RemediationDetailSchema
    { let _ = scm_sdk::validation::parse("RemediationDetailSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RemediationResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::RemediationResponse>; }
    // RemediationResponseSchema
    { let _ = scm_sdk::validation::parse("RemediationResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RescuedRetriesResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::RescuedRetriesResponse>; }
    // RescuedRetriesResponseSchema
    { let _ = scm_sdk::validation::parse("RescuedRetriesResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ResourceModelExtension
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::ResourceModelExtension>; }
    // ResourceModelExtensionSchema
    { let _ = scm_sdk::validation::parse("ResourceModelExtensionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ResponseDetected
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::ResponseDetected>; }
    // ResponseDetectedSchema
    { let _ = scm_sdk::validation::parse("ResponseDetectedSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ResponseDetectionDetails
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::ResponseDetectionDetails>; }
    // ResponseDetectionDetailsSchema
    { let _ = scm_sdk::validation::parse("ResponseDetectionDetailsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ResponseMode
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ResponseMode>; }
    // RestConnectionParams
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::RestConnectionParams>; }
    // RestConnectionParamsSchema
    { let _ = scm_sdk::validation::parse("RestConnectionParamsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RiskLevel
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::RiskLevel>; }
    // RiskLevelSchema
    { let _ = scm_sdk::validation::parse("RiskLevelSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RiskRating
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::RiskRating>; }
    // RuleConfiguration
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::RuleConfiguration>; }
    // RuleConfigurationSchema
    { let _ = scm_sdk::validation::parse("RuleConfigurationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RuleEditableField
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::RuleEditableField>; }
    // RuleEditableFieldDropdown
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::RuleEditableFieldDropdown>; }
    // RuleEditableFieldDropdownSchema
    { let _ = scm_sdk::validation::parse("RuleEditableFieldDropdownSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RuleEditableFieldSchema
    { let _ = scm_sdk::validation::parse("RuleEditableFieldSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RuleEditableFieldType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::RuleEditableFieldType>; }
    // RuleEvaluationList
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::RuleEvaluationList>; }
    // RuleEvaluationListSchema
    { let _ = scm_sdk::validation::parse("RuleEvaluationListSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RuleEvaluationResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::RuleEvaluationResponse>; }
    // RuleEvaluationResponseSchema
    { let _ = scm_sdk::validation::parse("RuleEvaluationResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RuleEvaluationResult
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::RuleEvaluationResult>; }
    // RuleFieldValueKey
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::RuleFieldValueKey>; }
    // RuleItemConfidenceLevel
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::RuleItemConfidenceLevel>; }
    // RuleItemConfidenceLevelSchema
    { let _ = scm_sdk::validation::parse("RuleItemConfidenceLevelSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RuleItemDetectionTechnique
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::RuleItemDetectionTechnique>; }
    // RuleItemDetectionTechniqueSchema
    { let _ = scm_sdk::validation::parse("RuleItemDetectionTechniqueSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RuleItemEdmMatchCriteria
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::RuleItemEdmMatchCriteria>; }
    // RuleItemEdmMatchCriteriaSchema
    { let _ = scm_sdk::validation::parse("RuleItemEdmMatchCriteriaSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RuleItemMatchType
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::RuleItemMatchType>; }
    // RuleItemMatchTypeSchema
    { let _ = scm_sdk::validation::parse("RuleItemMatchTypeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RuleItemOccurrenceOperatorType
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::RuleItemOccurrenceOperatorType>; }
    // RuleItemOccurrenceOperatorTypeSchema
    { let _ = scm_sdk::validation::parse("RuleItemOccurrenceOperatorTypeSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RuleOrigin
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::RuleOrigin>; }
    // RuleRemediation
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::RuleRemediation>; }
    // RuleRemediationSchema
    { let _ = scm_sdk::validation::parse("RuleRemediationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RuleResultCondition
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::RuleResultCondition>; }
    // RuleResultConditionSchema
    { let _ = scm_sdk::validation::parse("RuleResultConditionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RuleState
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::RuleState>; }
    // RuleType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::RuleType>; }
    // RuntimeSecurityPolicy
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::RuntimeSecurityPolicy>; }
    // RuntimeSecurityPolicyConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::RuntimeSecurityPolicyConfig>; }
    // RuntimeSecurityPolicyConfigSchema
    { let _ = scm_sdk::validation::parse("RuntimeSecurityPolicyConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RuntimeSecurityPolicySchema
    { let _ = scm_sdk::validation::parse("RuntimeSecurityPolicySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // RuntimeSecurityProfileResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::RuntimeSecurityProfileResponse>; }
    // RuntimeSecurityProfileResponseSchema
    { let _ = scm_sdk::validation::parse("RuntimeSecurityProfileResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // SafetySubCategory
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::SafetySubCategory>; }
    // SCAN_REPORTS_PATH
    { let _ = scm_sdk::constants::SCAN_REPORTS_PATH; }
    // SCAN_RESULTS_PATH
    { let _ = scm_sdk::constants::SCAN_RESULTS_PATH; }
    // ScanBaseResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ScanBaseResponse>; }
    // ScanBaseResponseSchema
    { let _ = scm_sdk::validation::parse("ScanBaseResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ScanCallOptions
    { let _ = std::marker::PhantomData::<scm_sdk::RequestOptions>; }
    // ScanCallOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // ScanCallOptions.signal
    { let _ = scm_sdk::RequestOptions::cancel; }
    // ScanCallOptions.timeoutMs
    { let _ = scm_sdk::RequestOptions::with_timeout; }
    // ScanCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ScanCreateRequest>; }
    // ScanCreateRequestSchema
    { let _ = scm_sdk::validation::parse("ScanCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ScanDetails
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ScanDetails>; }
    // ScanDetailsSchema
    { let _ = scm_sdk::validation::parse("ScanDetailsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ScanIdResult
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::ScanIdResult>; }
    // ScanIdResult.req_id
    { let _ = |value: &scm_sdk::airs::scan::models::ScanIdResult| { let _ = &value.req_id; }; }
    // ScanIdResult.result
    { let _ = |value: &scm_sdk::airs::scan::models::ScanIdResult| { let _ = &value.result; }; }
    // ScanIdResult.scan_id
    { let _ = |value: &scm_sdk::airs::scan::models::ScanIdResult| { let _ = &value.scan_id; }; }
    // ScanIdResult.source
    { let _ = |value: &scm_sdk::airs::scan::models::ScanIdResult| { let _ = &value.source; }; }
    // ScanIdResult.status
    { let _ = |value: &scm_sdk::airs::scan::models::ScanIdResult| { let _ = &value.status; }; }
    // ScanIdResultSchema
    { let _ = scm_sdk::validation::parse("ScanIdResultSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ScanList
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ScanList>; }
    // ScanListSchema
    { let _ = scm_sdk::validation::parse("ScanListSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ScanLogQueryOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::ScanLogQueryOptions>; }
    // ScanLogQueryOptions.filter
    { let _ = |value: &scm_sdk::airs::management::ScanLogQueryOptions| { let _ = &value.filter; }; }
    // ScanLogQueryOptions.page_token
    { let _ = |value: &scm_sdk::airs::management::ScanLogQueryOptions| { let _ = &value.page_token; }; }
    // ScanLogQueryOptions.pageNumber
    { let _ = |value: &scm_sdk::airs::management::ScanLogQueryOptions| { let _ = &value.page_number; }; }
    // ScanLogQueryOptions.pageSize
    { let _ = |value: &scm_sdk::airs::management::ScanLogQueryOptions| { let _ = &value.page_size; }; }
    // ScanLogQueryOptions.time_interval
    { let _ = |value: &scm_sdk::airs::management::ScanLogQueryOptions| { let _ = &value.time_interval; }; }
    // ScanLogQueryOptions.time_unit
    { let _ = |value: &scm_sdk::airs::management::ScanLogQueryOptions| { let _ = &value.time_unit; }; }
    // ScanLogsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::ScanLogsClient>; let _ = scm_sdk::airs::management::ScanLogsClient::new; }
    // ScanLogsClient.query
    { let _ = scm_sdk::airs::management::ScanLogsClient::query; }
    // ScanLogsClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // ScanLogsClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // ScanLogsClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // ScanLogsClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // Scanner
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::Scanner>; let _ = scm_sdk::airs::scan::Scanner::new; }
    // Scanner.asyncScan
    { let _ = scm_sdk::airs::scan::Scanner::async_scan; }
    // Scanner.queryByReportIds
    { let _ = scm_sdk::airs::scan::Scanner::query_by_report_ids; }
    // Scanner.queryByScanIds
    { let _ = scm_sdk::airs::scan::Scanner::query_by_scan_ids; }
    // Scanner.syncScan
    { let _ = scm_sdk::airs::scan::Scanner::sync_scan; let _ = scm_sdk::airs::scan::Scanner::scan_request; }
    // ScanOrigin
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ScanOrigin>; }
    // ScanRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::ScanRequest>; }
    // ScanRequestContentsInner
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::ScanRequestContentsInner>; }
    // ScanRequestContentsInnerSchema
    { let _ = scm_sdk::validation::parse("ScanRequestContentsInnerSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ScanRequestSchema
    { let _ = scm_sdk::validation::parse("ScanRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ScanResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::ScanResponse>; }
    // ScanResponseSchema
    { let _ = scm_sdk::validation::parse("ScanResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ScanResultEntry
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::ScanResultEntry>; }
    // ScanResultEntrySchema
    { let _ = scm_sdk::validation::parse("ScanResultEntrySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ScanResultForDashboard
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::ScanResultForDashboard>; }
    // ScanResultForDashboardSchema
    { let _ = scm_sdk::validation::parse("ScanResultForDashboardSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ScanStatisticsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ScanStatisticsResponse>; }
    // ScanStatisticsResponseSchema
    { let _ = scm_sdk::validation::parse("ScanStatisticsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ScanSummary
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::ScanSummary>; }
    // ScanSummarySchema
    { let _ = scm_sdk::validation::parse("ScanSummarySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ScoreTrendResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ScoreTrendResponse>; }
    // ScoreTrendResponseSchema
    { let _ = scm_sdk::validation::parse("ScoreTrendResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ScoreTrendSeries
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ScoreTrendSeries>; }
    // ScoreTrendSeriesSchema
    { let _ = scm_sdk::validation::parse("ScoreTrendSeriesSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // SDK_VERSION
    { let _ = scm_sdk::constants::SDK_VERSION; }
    // SecurityProfile
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::SecurityProfile>; }
    // SecurityProfile.active
    { let _ = |value: &scm_sdk::airs::management::models::SecurityProfile| { let _ = &value.active; }; }
    // SecurityProfile.created_by
    { let _ = |value: &scm_sdk::airs::management::models::SecurityProfile| { let _ = &value.created_by; }; }
    // SecurityProfile.csp_id
    { let _ = |value: &scm_sdk::airs::management::models::SecurityProfile| { let _ = &value.csp_id; }; }
    // SecurityProfile.last_modified_ts
    { let _ = |value: &scm_sdk::airs::management::models::SecurityProfile| { let _ = &value.last_modified_ts; }; }
    // SecurityProfile.policy
    { let _ = |value: &scm_sdk::airs::management::models::SecurityProfile| { let _ = &value.policy; }; }
    // SecurityProfile.profile_id
    { let _ = |value: &scm_sdk::airs::management::models::SecurityProfile| { let _ = &value.profile_id; }; }
    // SecurityProfile.profile_name
    { let _ = |value: &scm_sdk::airs::management::models::SecurityProfile| { let _ = &value.profile_name; }; }
    // SecurityProfile.revision
    { let _ = |value: &scm_sdk::airs::management::models::SecurityProfile| { let _ = &value.revision; }; }
    // SecurityProfile.tsg_id
    { let _ = |value: &scm_sdk::airs::management::models::SecurityProfile| { let _ = &value.tsg_id; }; }
    // SecurityProfile.updated_by
    { let _ = |value: &scm_sdk::airs::management::models::SecurityProfile| { let _ = &value.updated_by; }; }
    // SecurityProfileListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::SecurityProfileListResponse>; }
    // SecurityProfileListResponse.ai_profiles
    { let _ = |value: &scm_sdk::airs::management::models::SecurityProfileListResponse| { let _ = &value.ai_profiles; }; }
    // SecurityProfileListResponse.next_offset
    { let _ = |value: &scm_sdk::airs::management::models::SecurityProfileListResponse| { let _ = &value.next_offset; }; }
    // SecurityProfileListResponseSchema
    { let _ = scm_sdk::validation::parse("SecurityProfileListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // SecurityProfileSchema
    { let _ = scm_sdk::validation::parse("SecurityProfileSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // SecuritySubCategory
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::SecuritySubCategory>; }
    // SentimentRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::SentimentRequest>; }
    // SentimentRequestSchema
    { let _ = scm_sdk::validation::parse("SentimentRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // SentimentResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::SentimentResponse>; }
    // SentimentResponseSchema
    { let _ = scm_sdk::validation::parse("SentimentResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // serializeListing
    { let _ = scm_sdk::helpers::serialize_listing; }
    // setDottedValue
    { let _ = scm_sdk::airs::gateway::set_dotted_value; }
    // SeverityByConfidence
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::SeverityByConfidence>; }
    // SeverityByConfidenceSchema
    { let _ = scm_sdk::validation::parse("SeverityByConfidenceSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // SeverityFilter
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::SeverityFilter>; }
    // SeverityReport
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::SeverityReport>; }
    // SeverityReportSchema
    { let _ = scm_sdk::validation::parse("SeverityReportSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // SeverityStats
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::SeverityStats>; }
    // SeverityStatsSchema
    { let _ = scm_sdk::validation::parse("SeverityStatsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // SnapshotVersion
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::SnapshotVersion>; }
    // SnapshotVersionListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::SnapshotVersionListOptions>; }
    // SnapshotVersionListOptions.limit
    { let _ = |value: &scm_sdk::airs::model_security::SnapshotVersionListOptions| { let _ = &value.limit; }; }
    // SnapshotVersionListOptions.next_token
    { let _ = |value: &scm_sdk::airs::model_security::SnapshotVersionListOptions| { let _ = &value.next_token; }; }
    // SnapshotVersionListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::SnapshotVersionListResponse>; }
    // SnapshotVersionListResponseSchema
    { let _ = scm_sdk::validation::parse("SnapshotVersionListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // SnapshotVersionSchema
    { let _ = scm_sdk::validation::parse("SnapshotVersionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // SortByDateField
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::SortByDateField>; }
    // SortByFileField
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::SortByFileField>; }
    // SortDirection
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::SortDirection>; }
    // SortObject
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::SortObject>; }
    // SortObjectSchema
    { let _ = scm_sdk::validation::parse("SortObjectSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // SourceAttributes
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::SourceAttributes>; }
    // SourceAttributesSchema
    { let _ = scm_sdk::validation::parse("SourceAttributesSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // SourceCodeDetection
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::SourceCodeDetection>; }
    // SourceCodeDetectionSchema
    { let _ = scm_sdk::validation::parse("SourceCodeDetectionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // SourceType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::SourceType>; }
    // StartProfilingResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::StartProfilingResponse>; }
    // StartProfilingResponseSchema
    { let _ = scm_sdk::validation::parse("StartProfilingResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // StaticJobMetadata
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::StaticJobMetadata>; }
    // StaticJobMetadataSchema
    { let _ = scm_sdk::validation::parse("StaticJobMetadataSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // StaticJobRemediation
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::StaticJobRemediation>; }
    // StaticJobRemediationRecommendation
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::StaticJobRemediationRecommendation>; }
    // StaticJobRemediationRecommendationSchema
    { let _ = scm_sdk::validation::parse("StaticJobRemediationRecommendationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // StaticJobRemediationSchema
    { let _ = scm_sdk::validation::parse("StaticJobRemediationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // StaticJobReport
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::StaticJobReport>; }
    // StaticJobReportSchema
    { let _ = scm_sdk::validation::parse("StaticJobReportSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // StaticJobReportStats
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::StaticJobReportStats>; }
    // StaticJobReportStatsSchema
    { let _ = scm_sdk::validation::parse("StaticJobReportStatsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // StatusQueryParam
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::StatusQueryParam>; }
    // StreamDetailResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::StreamDetailResponse>; }
    // StreamDetailResponseSchema
    { let _ = scm_sdk::validation::parse("StreamDetailResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // StreamGoal
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::StreamGoal>; }
    // StreamGoalSchema
    { let _ = scm_sdk::validation::parse("StreamGoalSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // StreamingConnectionParams
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::StreamingConnectionParams>; }
    // StreamingConnectionParamsSchema
    { let _ = scm_sdk::validation::parse("StreamingConnectionParamsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // StreamIterationData
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::StreamIterationData>; }
    // StreamIterationDataSchema
    { let _ = scm_sdk::validation::parse("StreamIterationDataSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // StreamListResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::StreamListResponse>; }
    // StreamListResponseSchema
    { let _ = scm_sdk::validation::parse("StreamListResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // StreamType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::StreamType>; }
    // SubCategoryModel
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::SubCategoryModel>; }
    // SubCategoryModelSchema
    { let _ = scm_sdk::validation::parse("SubCategoryModelSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // SubCategoryStats
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::SubCategoryStats>; }
    // SubCategoryStatsSchema
    { let _ = scm_sdk::validation::parse("SubCategoryStatsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // SYNC_SCAN_PATH
    { let _ = scm_sdk::constants::SYNC_SCAN_PATH; }
    // SyncScanOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::SyncScanOptions>; }
    // SyncScanOptions.metadata
    { let _ = |value: &scm_sdk::airs::scan::SyncScanOptions| { let _ = &value.metadata; }; }
    // SyncScanOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // SyncScanOptions.sessionId
    { let _ = |value: &scm_sdk::airs::scan::SyncScanOptions| { let _ = &value.session_id; }; }
    // SyncScanOptions.signal
    { let _ = scm_sdk::RequestOptions::cancel; }
    // SyncScanOptions.timeoutMs
    { let _ = scm_sdk::RequestOptions::with_timeout; }
    // SyncScanOptions.trId
    { let _ = |value: &scm_sdk::airs::scan::SyncScanOptions| { let _ = &value.tr_id; }; }
    // TargetAdditionalContext
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetAdditionalContext>; }
    // TargetAdditionalContextSchema
    { let _ = scm_sdk::validation::parse("TargetAdditionalContextSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TargetAuthType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetAuthType>; }
    // TargetAuthValidationRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetAuthValidationRequest>; }
    // TargetAuthValidationRequestSchema
    { let _ = scm_sdk::validation::parse("TargetAuthValidationRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TargetAuthValidationResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetAuthValidationResponse>; }
    // TargetAuthValidationResponseSchema
    { let _ = scm_sdk::validation::parse("TargetAuthValidationResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TargetBackground
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetBackground>; }
    // TargetBackgroundSchema
    { let _ = scm_sdk::validation::parse("TargetBackgroundSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TargetConnectionConfig
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetConnectionConfig>; }
    // TargetConnectionConfigSchema
    { let _ = scm_sdk::validation::parse("TargetConnectionConfigSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TargetConnectionType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetConnectionType>; }
    // TargetContextUpdate
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetContextUpdate>; }
    // TargetContextUpdateSchema
    { let _ = scm_sdk::validation::parse("TargetContextUpdateSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TargetCreateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetCreateRequest>; }
    // TargetCreateRequestSchema
    { let _ = scm_sdk::validation::parse("TargetCreateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TargetJobRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetJobRequest>; }
    // TargetJobRequestSchema
    { let _ = scm_sdk::validation::parse("TargetJobRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TargetList
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetList>; }
    // TargetListAllOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::TargetListOptions>; }
    // TargetListAllOptions.adapter_uuid
    { let _ = |value: &scm_sdk::airs::red_team::TargetListOptions| { let _ = &value.adapter_uuid; }; }
    // TargetListAllOptions.limit
    { let _ = |value: &scm_sdk::airs::red_team::TargetListOptions| { let _ = &value.limit; }; }
    // TargetListAllOptions.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // TargetListAllOptions.search
    { let _ = |value: &scm_sdk::airs::red_team::TargetListOptions| { let _ = &value.search; }; }
    // TargetListAllOptions.status
    { let _ = |value: &scm_sdk::airs::red_team::TargetListOptions| { let _ = &value.status; }; }
    // TargetListAllOptions.target_type
    { let _ = |value: &scm_sdk::airs::red_team::TargetListOptions| { let _ = &value.target_type; }; }
    // TargetListItem
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetListItem>; }
    // TargetListItemSchema
    { let _ = scm_sdk::validation::parse("TargetListItemSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TargetListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::TargetListOptions>; }
    // TargetListOptions.adapter_uuid
    { let _ = |value: &scm_sdk::airs::red_team::TargetListOptions| { let _ = &value.adapter_uuid; }; }
    // TargetListOptions.limit
    { let _ = |value: &scm_sdk::airs::red_team::TargetListOptions| { let _ = &value.limit; }; }
    // TargetListOptions.search
    { let _ = |value: &scm_sdk::airs::red_team::TargetListOptions| { let _ = &value.search; }; }
    // TargetListOptions.skip
    { let _ = |value: &scm_sdk::airs::red_team::TargetListOptions| { let _ = &value.skip; }; }
    // TargetListOptions.status
    { let _ = |value: &scm_sdk::airs::red_team::TargetListOptions| { let _ = &value.status; }; }
    // TargetListOptions.target_type
    { let _ = |value: &scm_sdk::airs::red_team::TargetListOptions| { let _ = &value.target_type; }; }
    // TargetListSchema
    { let _ = scm_sdk::validation::parse("TargetListSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TargetMetadata
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetMetadata>; }
    // TargetMetadataSchema
    { let _ = scm_sdk::validation::parse("TargetMetadataSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TargetOperationOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::TargetOperationOptions>; }
    // TargetOperationOptions.validate
    { let _ = |value: &scm_sdk::airs::red_team::TargetOperationOptions| { let _ = &value.validate; }; }
    // TargetProbeRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetProbeRequest>; }
    // TargetProbeRequestSchema
    { let _ = scm_sdk::validation::parse("TargetProbeRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TargetProfileResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetProfileResponse>; }
    // TargetProfileResponseSchema
    { let _ = scm_sdk::validation::parse("TargetProfileResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TargetReference
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetReference>; }
    // TargetReferenceSchema
    { let _ = scm_sdk::validation::parse("TargetReferenceSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TargetResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetResponse>; }
    // TargetResponseSchema
    { let _ = scm_sdk::validation::parse("TargetResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TargetStatus
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetStatus>; }
    // TargetTemplateCollection
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetTemplateCollection>; }
    // TargetTemplateCollectionSchema
    { let _ = scm_sdk::validation::parse("TargetTemplateCollectionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TargetType
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetType>; }
    // TargetUpdateRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TargetUpdateRequest>; }
    // TargetUpdateRequestSchema
    { let _ = scm_sdk::validation::parse("TargetUpdateRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TcReport
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::TcReport>; }
    // TcReportSchema
    { let _ = scm_sdk::validation::parse("TcReportSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TenantLanguagesResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::TenantLanguagesResponse>; }
    // TenantLanguagesResponseSchema
    { let _ = scm_sdk::validation::parse("TenantLanguagesResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TgReport
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::TgReport>; }
    // TgReportSchema
    { let _ = scm_sdk::validation::parse("TgReportSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ThreatCategory
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ThreatCategory>; }
    // ThreatScanReport
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::ThreatScanReport>; }
    // ThreatScanReportSchema
    { let _ = scm_sdk::validation::parse("ThreatScanReportSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TokenInfo
    { let _ = std::marker::PhantomData::<scm_sdk::auth::TokenInfo>; }
    // TokenInfo.expiresAt
    { let _ = |x: &scm_sdk::auth::TokenInfo| { let _ = &x.expires_at; }; }
    // TokenInfo.expiresInMs
    { let _ = |x: &scm_sdk::auth::TokenInfo| { let _ = &x.remaining; }; }
    // TokenInfo.hasToken
    { let _ = |x: &scm_sdk::auth::TokenInfo| { let _ = &x.cached; }; }
    // TokenInfo.isExpired
    { let _ = |x: &scm_sdk::auth::TokenInfo| { let _ = &x.is_expired; }; }
    // TokenInfo.isExpiringSoon
    { let _ = |x: &scm_sdk::auth::TokenInfo| { let _ = &x.is_expiring_soon; }; }
    // TokenInfo.isValid
    { let _ = |x: &scm_sdk::auth::TokenInfo| { let _ = &x.is_valid; }; }
    // TokensChartResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::TokensChartResponse>; }
    // TokensChartResponseSchema
    { let _ = scm_sdk::validation::parse("TokensChartResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TokenStats
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::TokenStats>; }
    // TokenStatsSchema
    { let _ = scm_sdk::validation::parse("TokenStatsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ToolDetected
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::ToolDetected>; }
    // ToolDetectedSchema
    { let _ = scm_sdk::validation::parse("ToolDetectedSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ToolDetectionDetails
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::ToolDetectionDetails>; }
    // ToolDetectionDetailsSchema
    { let _ = scm_sdk::validation::parse("ToolDetectionDetailsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ToolDetectionEntry
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::ToolDetectionEntry>; }
    // ToolDetectionEntrySchema
    { let _ = scm_sdk::validation::parse("ToolDetectionEntrySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ToolDetectionFlags
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::ToolDetectionFlags>; }
    // ToolDetectionFlagsSchema
    { let _ = scm_sdk::validation::parse("ToolDetectionFlagsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ToolEvent
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::ToolEvent>; }
    // ToolEventMetadata
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::ToolEventMetadata>; }
    // ToolEventMetadataSchema
    { let _ = scm_sdk::validation::parse("ToolEventMetadataSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ToolEventSchema
    { let _ = scm_sdk::validation::parse("ToolEventSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TopicArray
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::TopicArray>; }
    // TopicArraySchema
    { let _ = scm_sdk::validation::parse("TopicArraySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TopicGuardrailDetails
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::TopicGuardrailDetails>; }
    // TopicGuardrailDetails.allowed_topics
    { let _ = |value: &scm_sdk::airs::scan::models::TopicGuardrailDetails| { let _ = &value.allowed_topics; }; }
    // TopicGuardrailDetails.blocked_topics
    { let _ = |value: &scm_sdk::airs::scan::models::TopicGuardrailDetails| { let _ = &value.blocked_topics; }; }
    // TopicGuardrailDetailsSchema
    { let _ = scm_sdk::validation::parse("TopicGuardrailDetailsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TopicListAllOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::TopicListAllOptions>; }
    // TopicListAllOptions.limit
    { let _ = |value: &scm_sdk::airs::management::TopicListAllOptions| { let _ = &value.limit; }; }
    // TopicListAllOptions.max
    { let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; }; }
    // TopicListOptions
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::TopicListOptions>; }
    // TopicListOptions.latestOnly
    { let _ = |value: &scm_sdk::airs::management::TopicListOptions| { let _ = &value.latest_only; }; }
    // TopicListOptions.limit
    { let _ = |value: &scm_sdk::airs::management::TopicListOptions| { let _ = &value.limit; }; }
    // TopicListOptions.offset
    { let _ = |value: &scm_sdk::airs::management::TopicListOptions| { let _ = &value.offset; }; }
    // TopicObject
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::TopicObject>; }
    // TopicObjectSchema
    { let _ = scm_sdk::validation::parse("TopicObjectSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TopicsClient
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::TopicsClient>; let _ = scm_sdk::airs::management::TopicsClient::new; }
    // TopicsClient.create
    { let _ = scm_sdk::airs::management::TopicsClient::create; }
    // TopicsClient.delete
    { let _ = scm_sdk::airs::management::TopicsClient::delete; }
    // TopicsClient.forceDelete
    { let _ = scm_sdk::airs::management::TopicsClient::force_delete; }
    // TopicsClient.forceDeleteWithAudit
    { let _ = scm_sdk::airs::management::TopicsClient::force_delete_with_audit; }
    // TopicsClient.get
    { let _ = scm_sdk::airs::management::TopicsClient::get; }
    // TopicsClient.getByName
    { let _ = scm_sdk::airs::management::TopicsClient::get_by_name; }
    // TopicsClient.list
    { let _ = scm_sdk::airs::management::TopicsClient::list; }
    // TopicsClient.listAll
    { let _ = scm_sdk::airs::management::TopicsClient::list_all; }
    // TopicsClient.listForToken
    { let _ = scm_sdk::airs::management::TopicsClient::list_for_token; }
    // TopicsClient.update
    { let _ = scm_sdk::airs::management::TopicsClient::update; }
    // TopicsClientOptions
    { let _ = std::marker::PhantomData::<scm_sdk::Client>; }
    // TopicsClientOptions.auth
    { let _ = scm_sdk::ClientBuilder::authentication; }
    // TopicsClientOptions.baseUrl
    { let _ = scm_sdk::Client::builder; }
    // TopicsClientOptions.numRetries
    { let _ = scm_sdk::RequestOptions::with_retries; }
    // TopicsClientOptions.tsgId
    { let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer, "probe", 1, "1"); }; }
    // ToxicCategory
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::ToxicCategory>; }
    // ToxicCategorySchema
    { let _ = scm_sdk::validation::parse("ToxicCategorySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // TSG_ID_HEADER
    { let _ = scm_sdk::constants::TSG_ID_HEADER; }
    // UpdateChannelRequest
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::UpdateChannelRequest>; }
    // UpdateChannelRequestSchema
    { let _ = scm_sdk::validation::parse("UpdateChannelRequestSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // UrlCategory
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::UrlCategory>; }
    // UrlCategorySchema
    { let _ = scm_sdk::validation::parse("UrlCategorySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // URLExclusion
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::URLExclusion>; }
    // URLExclusionSchema
    { let _ = scm_sdk::validation::parse("URLExclusionSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // UrlfEntry
    { let _ = std::marker::PhantomData::<scm_sdk::airs::scan::models::UrlfEntry>; }
    // UrlfEntrySchema
    { let _ = scm_sdk::validation::parse("UrlfEntrySchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // USER_AGENT
    { let _ = scm_sdk::constants::USER_AGENT; }
    // UserGroupResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::UserGroupResponse>; }
    // UserGroupResponseSchema
    { let _ = scm_sdk::validation::parse("UserGroupResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // UserTrendsResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::gateway::models::UserTrendsResponse>; }
    // UserTrendsResponseSchema
    { let _ = scm_sdk::validation::parse("UserTrendsResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ValidationError
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::ValidationError>; }
    // ValidationErrorSchema
    { let _ = scm_sdk::validation::parse("ValidationErrorSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // Verdict
    { let _ = std::marker::PhantomData::<scm_sdk::shared::models::Verdict>; }
    // VerdictType
    { let _ = std::marker::PhantomData::<scm_sdk::shared::models::VerdictType>; }
    // ViolationList
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ViolationList>; }
    // ViolationListSchema
    { let _ = scm_sdk::validation::parse("ViolationListSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ViolationRemediation
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ViolationRemediation>; }
    // ViolationRemediationSchema
    { let _ = scm_sdk::validation::parse("ViolationRemediationSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ViolationResponse
    { let _ = std::marker::PhantomData::<scm_sdk::airs::model_security::models::ViolationResponse>; }
    // ViolationResponseSchema
    { let _ = scm_sdk::validation::parse("ViolationResponseSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // ViolationSeverityCounts
    { let _ = std::marker::PhantomData::<scm_sdk::airs::management::models::ViolationSeverityCounts>; }
    // ViolationSeverityCountsSchema
    { let _ = scm_sdk::validation::parse("ViolationSeverityCountsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // WalkAllOptions
    { let _ = std::marker::PhantomData::<scm_sdk::helpers::WalkAllOptions>; }
    // WalkAllOptions.limit
    { let _ = |value: &scm_sdk::helpers::WalkAllOptions| { let _ = &value.limit; }; }
    // WalkAllOptions.max
    { let _ = |value: &scm_sdk::helpers::WalkAllOptions| { let _ = value.collect.max_items; }; }
    // WebSocketConnectionParams
    { let _ = std::marker::PhantomData::<scm_sdk::airs::red_team::models::WebSocketConnectionParams>; }
    // WebSocketConnectionParamsSchema
    { let _ = scm_sdk::validation::parse("WebSocketConnectionParamsSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
    // WeightedRegex
    { let _ = std::marker::PhantomData::<scm_sdk::dlp::models::WeightedRegex>; }
    // WeightedRegexSchema
    { let _ = scm_sdk::validation::parse("WeightedRegexSchema", serde_json::Value::Null, scm_sdk::validation::Mode::Request); }
}
