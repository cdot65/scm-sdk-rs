use crate::{Error, ErrorKind, Result, pagination::CollectOptions};
/// Product-private query construction bounds allocations before authentication.
pub(super) struct Query {
    values: Vec<(String, String)>,
    bytes: usize,
}
impl Query {
    pub(super) fn new() -> Self {
        Self {
            values: Vec::new(),
            bytes: 0,
        }
    }
    pub(super) fn text(&mut self, key: &str, value: &str) -> Result<()> {
        self.bytes = self
            .bytes
            .checked_add(key.len())
            .and_then(|n| n.checked_add(value.len()))
            .filter(|n| *n <= crate::config::MAX_URL_BYTES)
            .ok_or_else(|| Error::new(ErrorKind::Limit))?;
        if self.values.len() >= 256 {
            return Err(Error::new(ErrorKind::Limit));
        }
        self.values.push((key.into(), value.into()));
        Ok(())
    }
    fn string(&mut self, key: &str, value: Option<&str>) -> Result<()> {
        if let Some(value) = value {
            self.text(key, value)?;
        }
        Ok(())
    }
    fn number(&mut self, key: &str, value: Option<u64>) -> Result<()> {
        if let Some(value) = value {
            if value > 9_007_199_254_740_991 || key == "limit" && value == 0 {
                return Err(Error::new(ErrorKind::Validation));
            }
            self.text(key, &value.to_string())?;
        }
        Ok(())
    }
    fn boolean(&mut self, key: &str, value: Option<bool>) -> Result<()> {
        if let Some(value) = value {
            self.text(key, if value { "true" } else { "false" })?;
        }
        Ok(())
    }
    fn array(&mut self, key: &str, value: Option<&[String]>) -> Result<()> {
        if let Some(values) = value {
            for value in values {
                self.text(key, value)?;
            }
        }
        Ok(())
    }
    pub(super) fn finish(self) -> Vec<(String, String)> {
        self.values
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ModelSecurityScanListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub is_background_refresh: Option<bool>,
    pub model_version_uuid: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub search_query: Option<String>,
    pub eval_outcomes: Option<Vec<String>>,
    pub source_types: Option<Vec<String>>,
    pub security_group_uuid: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub labels_query: Option<String>,
}
impl std::fmt::Debug for ModelSecurityScanListOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelSecurityScanListOptions([REDACTED])")
    }
}
impl ModelSecurityScanListOptions {
    pub(super) fn query(&self) -> Result<Vec<(String, String)>> {
        let mut query = Query::new();
        query.number("skip", self.skip)?;
        query.number("limit", self.limit.map(u64::from))?;
        query.string("search", self.search.as_deref())?;
        query.boolean("isBackgroundRefresh", self.is_background_refresh)?;
        query.string("model_version_uuid", self.model_version_uuid.as_deref())?;
        query.string("sort_by", self.sort_by.as_deref())?;
        query.string("sort_order", self.sort_order.as_deref())?;
        query.string("search_query", self.search_query.as_deref())?;
        query.array("eval_outcomes", self.eval_outcomes.as_deref())?;
        query.array("source_types", self.source_types.as_deref())?;
        query.string("security_group_uuid", self.security_group_uuid.as_deref())?;
        query.string("start_time", self.start_time.as_deref())?;
        query.string("end_time", self.end_time.as_deref())?;
        query.string("labels_query", self.labels_query.as_deref())?;
        Ok(query.finish())
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ModelSecurityEvaluationListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub sort_field: Option<String>,
    pub sort_order: Option<String>,
    pub result: Option<String>,
    pub rule_instance_uuid: Option<String>,
}
impl std::fmt::Debug for ModelSecurityEvaluationListOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelSecurityEvaluationListOptions([REDACTED])")
    }
}
impl ModelSecurityEvaluationListOptions {
    pub(super) fn query(&self) -> Result<Vec<(String, String)>> {
        let mut query = Query::new();
        query.number("skip", self.skip)?;
        query.number("limit", self.limit.map(u64::from))?;
        query.string("search", self.search.as_deref())?;
        query.string("sort_field", self.sort_field.as_deref())?;
        query.string("sort_order", self.sort_order.as_deref())?;
        query.string("result", self.result.as_deref())?;
        query.string("rule_instance_uuid", self.rule_instance_uuid.as_deref())?;
        Ok(query.finish())
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ModelSecurityFileListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub recursive: Option<bool>,
    pub sort_field: Option<String>,
    pub sort_dir: Option<String>,
    pub type_: Option<String>,
    pub result: Option<String>,
    pub query_path: Option<String>,
}
impl std::fmt::Debug for ModelSecurityFileListOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelSecurityFileListOptions([REDACTED])")
    }
}
impl ModelSecurityFileListOptions {
    pub(super) fn query(&self) -> Result<Vec<(String, String)>> {
        let mut query = Query::new();
        query.number("skip", self.skip)?;
        query.number("limit", self.limit.map(u64::from))?;
        query.string("search", self.search.as_deref())?;
        query.boolean("recursive", self.recursive)?;
        query.string("sort_field", self.sort_field.as_deref())?;
        query.string("sort_dir", self.sort_dir.as_deref())?;
        query.string("type", self.type_.as_deref())?;
        query.string("result", self.result.as_deref())?;
        query.string("query_path", self.query_path.as_deref())?;
        Ok(query.finish())
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ModelSecurityLabelListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
}
impl std::fmt::Debug for ModelSecurityLabelListOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelSecurityLabelListOptions([REDACTED])")
    }
}
impl ModelSecurityLabelListOptions {
    pub(super) fn query(&self) -> Result<Vec<(String, String)>> {
        let mut query = Query::new();
        query.number("skip", self.skip)?;
        query.number("limit", self.limit.map(u64::from))?;
        query.string("search", self.search.as_deref())?;
        Ok(query.finish())
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ModelSecurityModelListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub is_background_refresh: Option<bool>,
    pub search_query: Option<String>,
    pub sort_field: Option<String>,
    pub sort_order: Option<String>,
    pub latest_version_outcomes: Option<Vec<String>>,
    pub latest_version_formats: Option<Vec<String>>,
    pub latest_version_source_types: Option<Vec<String>>,
    pub latest_version_scan_time_before: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}
impl std::fmt::Debug for ModelSecurityModelListOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelSecurityModelListOptions([REDACTED])")
    }
}
impl ModelSecurityModelListOptions {
    pub(super) fn query(&self) -> Result<Vec<(String, String)>> {
        let mut query = Query::new();
        query.number("skip", self.skip)?;
        query.number("limit", self.limit.map(u64::from))?;
        query.string("search", self.search.as_deref())?;
        query.boolean("isBackgroundRefresh", self.is_background_refresh)?;
        query.string("search_query", self.search_query.as_deref())?;
        query.string("sort_field", self.sort_field.as_deref())?;
        query.string("sort_order", self.sort_order.as_deref())?;
        query.array(
            "latest_version_outcomes",
            self.latest_version_outcomes.as_deref(),
        )?;
        query.array(
            "latest_version_formats",
            self.latest_version_formats.as_deref(),
        )?;
        query.array(
            "latest_version_source_types",
            self.latest_version_source_types.as_deref(),
        )?;
        query.string(
            "latest_version_scan_time_before",
            self.latest_version_scan_time_before.as_deref(),
        )?;
        query.string("start_time", self.start_time.as_deref())?;
        query.string("end_time", self.end_time.as_deref())?;
        Ok(query.finish())
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ModelSecurityModelVersionListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub sort_order: Option<String>,
}
impl std::fmt::Debug for ModelSecurityModelVersionListOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelSecurityModelVersionListOptions([REDACTED])")
    }
}
impl ModelSecurityModelVersionListOptions {
    pub(super) fn query(&self) -> Result<Vec<(String, String)>> {
        let mut query = Query::new();
        query.number("skip", self.skip)?;
        query.number("limit", self.limit.map(u64::from))?;
        query.string("search", self.search.as_deref())?;
        query.string("sort_order", self.sort_order.as_deref())?;
        Ok(query.finish())
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ModelSecurityGroupListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub sort_field: Option<String>,
    pub sort_dir: Option<String>,
    pub source_types: Option<Vec<String>>,
    pub search_query: Option<String>,
    pub enabled_rules: Option<Vec<String>>,
}
impl std::fmt::Debug for ModelSecurityGroupListOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelSecurityGroupListOptions([REDACTED])")
    }
}
impl ModelSecurityGroupListOptions {
    pub(super) fn query(&self) -> Result<Vec<(String, String)>> {
        let mut query = Query::new();
        query.number("skip", self.skip)?;
        query.number("limit", self.limit.map(u64::from))?;
        query.string("search", self.search.as_deref())?;
        query.string("sort_field", self.sort_field.as_deref())?;
        query.string("sort_dir", self.sort_dir.as_deref())?;
        query.array("source_types", self.source_types.as_deref())?;
        query.string("search_query", self.search_query.as_deref())?;
        query.array("enabled_rules", self.enabled_rules.as_deref())?;
        Ok(query.finish())
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ModelSecurityRuleInstanceListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub is_custom: Option<bool>,
    pub generation: Option<u64>,
    pub security_rule_uuid: Option<String>,
    pub state: Option<String>,
}
impl std::fmt::Debug for ModelSecurityRuleInstanceListOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelSecurityRuleInstanceListOptions([REDACTED])")
    }
}
impl ModelSecurityRuleInstanceListOptions {
    pub(super) fn query(&self) -> Result<Vec<(String, String)>> {
        let mut query = Query::new();
        query.number("skip", self.skip)?;
        query.number("limit", self.limit.map(u64::from))?;
        query.string("search", self.search.as_deref())?;
        query.boolean("is_custom", self.is_custom)?;
        query.number("generation", self.generation)?;
        query.string("security_rule_uuid", self.security_rule_uuid.as_deref())?;
        query.string("state", self.state.as_deref())?;
        Ok(query.finish())
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ModelSecurityRuleListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub source_type: Option<String>,
    pub search_query: Option<String>,
    pub generation: Option<u64>,
}
impl std::fmt::Debug for ModelSecurityRuleListOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelSecurityRuleListOptions([REDACTED])")
    }
}
impl ModelSecurityRuleListOptions {
    pub(super) fn query(&self) -> Result<Vec<(String, String)>> {
        let mut query = Query::new();
        query.number("skip", self.skip)?;
        query.number("limit", self.limit.map(u64::from))?;
        query.string("search", self.search.as_deref())?;
        query.string("source_type", self.source_type.as_deref())?;
        query.string("search_query", self.search_query.as_deref())?;
        query.number("generation", self.generation)?;
        Ok(query.finish())
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct CustomRuleListOptions {
    pub limit: Option<u32>,
    pub skip: Option<u64>,
    pub sort_field: Option<String>,
    pub sort_dir: Option<String>,
    pub is_archived: Option<bool>,
    pub search_query: Option<String>,
    pub source_types: Option<Vec<String>>,
    pub generation: Option<u64>,
}
impl std::fmt::Debug for CustomRuleListOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CustomRuleListOptions([REDACTED])")
    }
}
impl CustomRuleListOptions {
    pub(super) fn query(&self) -> Result<Vec<(String, String)>> {
        if self.limit.is_some_and(|v| v > 100) {
            return Err(Error::new(ErrorKind::Validation));
        }
        if self
            .sort_field
            .as_deref()
            .is_some_and(|v| !["created_at", "updated_at"].contains(&v))
            || self
                .sort_dir
                .as_deref()
                .is_some_and(|v| !["asc", "desc"].contains(&v))
        {
            return Err(Error::new(ErrorKind::Validation));
        }
        let mut query = Query::new();
        query.number("limit", self.limit.map(u64::from))?;
        query.number("skip", self.skip)?;
        query.string("sort_field", self.sort_field.as_deref())?;
        query.string("sort_dir", self.sort_dir.as_deref())?;
        query.boolean("is_archived", self.is_archived)?;
        query.string("search_query", self.search_query.as_deref())?;
        query.array("source_types", self.source_types.as_deref())?;
        query.number("generation", self.generation)?;
        Ok(query.finish())
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct SnapshotVersionListOptions {
    pub limit: Option<u32>,
    pub next_token: Option<String>,
}
impl std::fmt::Debug for SnapshotVersionListOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SnapshotVersionListOptions([REDACTED])")
    }
}
impl SnapshotVersionListOptions {
    pub(super) fn query(&self) -> Result<Vec<(String, String)>> {
        if self.limit.is_some_and(|v| v > 100) {
            return Err(Error::new(ErrorKind::Validation));
        }
        let mut query = Query::new();
        query.number("limit", self.limit.map(u64::from))?;
        query.string("next_token", self.next_token.as_deref())?;
        Ok(query.finish())
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct CustomRuleSecurityGroupListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
}
impl std::fmt::Debug for CustomRuleSecurityGroupListOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CustomRuleSecurityGroupListOptions([REDACTED])")
    }
}
impl CustomRuleSecurityGroupListOptions {
    pub(super) fn query(&self) -> Result<Vec<(String, String)>> {
        if self.limit.is_some_and(|v| v > 100) {
            return Err(Error::new(ErrorKind::Validation));
        }
        let mut query = Query::new();
        query.number("skip", self.skip)?;
        query.number("limit", self.limit.map(u64::from))?;
        Ok(query.finish())
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ModelSecurityScanListAllOptions {
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub is_background_refresh: Option<bool>,
    pub model_version_uuid: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub search_query: Option<String>,
    pub eval_outcomes: Option<Vec<String>>,
    pub source_types: Option<Vec<String>>,
    pub security_group_uuid: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub labels_query: Option<String>,
    pub collect: CollectOptions,
}
impl std::fmt::Debug for ModelSecurityScanListAllOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelSecurityScanListAllOptions([REDACTED])")
    }
}
impl ModelSecurityScanListAllOptions {
    pub(super) fn page(&self, skip: u64, limit: u32) -> ModelSecurityScanListOptions {
        ModelSecurityScanListOptions {
            skip: Some(skip),
            limit: Some(limit),
            search: self.search.clone(),
            is_background_refresh: self.is_background_refresh,
            model_version_uuid: self.model_version_uuid.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            search_query: self.search_query.clone(),
            eval_outcomes: self.eval_outcomes.clone(),
            source_types: self.source_types.clone(),
            security_group_uuid: self.security_group_uuid.clone(),
            start_time: self.start_time.clone(),
            end_time: self.end_time.clone(),
            labels_query: self.labels_query.clone(),
        }
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ModelSecurityModelListAllOptions {
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub is_background_refresh: Option<bool>,
    pub search_query: Option<String>,
    pub sort_field: Option<String>,
    pub sort_order: Option<String>,
    pub latest_version_outcomes: Option<Vec<String>>,
    pub latest_version_formats: Option<Vec<String>>,
    pub latest_version_source_types: Option<Vec<String>>,
    pub latest_version_scan_time_before: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub collect: CollectOptions,
}
impl std::fmt::Debug for ModelSecurityModelListAllOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelSecurityModelListAllOptions([REDACTED])")
    }
}
impl ModelSecurityModelListAllOptions {
    pub(super) fn page(&self, skip: u64, limit: u32) -> ModelSecurityModelListOptions {
        ModelSecurityModelListOptions {
            skip: Some(skip),
            limit: Some(limit),
            search: self.search.clone(),
            is_background_refresh: self.is_background_refresh,
            search_query: self.search_query.clone(),
            sort_field: self.sort_field.clone(),
            sort_order: self.sort_order.clone(),
            latest_version_outcomes: self.latest_version_outcomes.clone(),
            latest_version_formats: self.latest_version_formats.clone(),
            latest_version_source_types: self.latest_version_source_types.clone(),
            latest_version_scan_time_before: self.latest_version_scan_time_before.clone(),
            start_time: self.start_time.clone(),
            end_time: self.end_time.clone(),
        }
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ModelSecurityModelVersionListAllOptions {
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub sort_order: Option<String>,
    pub collect: CollectOptions,
}
impl std::fmt::Debug for ModelSecurityModelVersionListAllOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelSecurityModelVersionListAllOptions([REDACTED])")
    }
}
impl ModelSecurityModelVersionListAllOptions {
    pub(super) fn page(&self, skip: u64, limit: u32) -> ModelSecurityModelVersionListOptions {
        ModelSecurityModelVersionListOptions {
            skip: Some(skip),
            limit: Some(limit),
            search: self.search.clone(),
            sort_order: self.sort_order.clone(),
        }
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ModelSecurityModelVersionFileListAllOptions {
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub collect: CollectOptions,
}
impl std::fmt::Debug for ModelSecurityModelVersionFileListAllOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelSecurityModelVersionFileListAllOptions([REDACTED])")
    }
}
impl ModelSecurityModelVersionFileListAllOptions {
    pub(super) fn page(&self, skip: u64, limit: u32) -> ModelSecurityLabelListOptions {
        ModelSecurityLabelListOptions {
            skip: Some(skip),
            limit: Some(limit),
            search: self.search.clone(),
        }
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ModelSecurityGroupListAllOptions {
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub sort_field: Option<String>,
    pub sort_dir: Option<String>,
    pub source_types: Option<Vec<String>>,
    pub search_query: Option<String>,
    pub enabled_rules: Option<Vec<String>>,
    pub collect: CollectOptions,
}
impl std::fmt::Debug for ModelSecurityGroupListAllOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelSecurityGroupListAllOptions([REDACTED])")
    }
}
impl ModelSecurityGroupListAllOptions {
    pub(super) fn page(&self, skip: u64, limit: u32) -> ModelSecurityGroupListOptions {
        ModelSecurityGroupListOptions {
            skip: Some(skip),
            limit: Some(limit),
            search: self.search.clone(),
            sort_field: self.sort_field.clone(),
            sort_dir: self.sort_dir.clone(),
            source_types: self.source_types.clone(),
            search_query: self.search_query.clone(),
            enabled_rules: self.enabled_rules.clone(),
        }
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ModelSecurityRuleListAllOptions {
    pub limit: Option<u32>,
    pub search: Option<String>,
    pub source_type: Option<String>,
    pub search_query: Option<String>,
    pub generation: Option<u64>,
    pub collect: CollectOptions,
}
impl std::fmt::Debug for ModelSecurityRuleListAllOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ModelSecurityRuleListAllOptions([REDACTED])")
    }
}
impl ModelSecurityRuleListAllOptions {
    pub(super) fn page(&self, skip: u64, limit: u32) -> ModelSecurityRuleListOptions {
        ModelSecurityRuleListOptions {
            skip: Some(skip),
            limit: Some(limit),
            search: self.search.clone(),
            source_type: self.source_type.clone(),
            search_query: self.search_query.clone(),
            generation: self.generation,
        }
    }
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct CustomRuleListAllOptions {
    pub limit: Option<u32>,
    pub sort_field: Option<String>,
    pub sort_dir: Option<String>,
    pub is_archived: Option<bool>,
    pub search_query: Option<String>,
    pub source_types: Option<Vec<String>>,
    pub collect: CollectOptions,
}
impl std::fmt::Debug for CustomRuleListAllOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CustomRuleListAllOptions([REDACTED])")
    }
}
impl CustomRuleListAllOptions {
    pub(super) fn page(&self, skip: u64, limit: u32) -> CustomRuleListOptions {
        CustomRuleListOptions {
            skip: Some(skip),
            limit: Some(limit),
            sort_field: self.sort_field.clone(),
            sort_dir: self.sort_dir.clone(),
            is_archived: self.is_archived,
            search_query: self.search_query.clone(),
            source_types: self.source_types.clone(),
            generation: None,
        }
    }
}
pub type ModelSecurityViolationListOptions = ModelSecurityLabelListOptions;
pub type ModelSecurityModelVersionFileListOptions = ModelSecurityLabelListOptions;
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct SnapshotVersionListAllOptions {
    pub limit: Option<u32>,
    pub next_token: Option<String>,
    pub collect: CollectOptions,
}
impl std::fmt::Debug for SnapshotVersionListAllOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SnapshotVersionListAllOptions([REDACTED])")
    }
}
