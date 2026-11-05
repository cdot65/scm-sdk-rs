//! Portkey 8e5da0e MCP tag filters. Pure helpers; no service calls are made.
use crate::{Error, ErrorKind, Result};
use std::collections::BTreeMap;

/// Upstream bound on tag filters: the encoded query is carried in the URL.
const MAX_FILTER_BYTES: usize = crate::config::MAX_URL_BYTES;

/// Encode an MCP list `tags` filter as the upstream JSON object string.
///
/// Keys are sorted, so equal tag sets always produce identical query bytes. Pass
/// the result as `tags` on `GatewayMcpServersClientListOptions` or
/// `GatewayMcpIntegrationsClientListOptions`. Only resources whose tags match
/// every pair are returned by the service.
///
/// ```
/// # fn main() -> scm_sdk::Result<()> {
/// let filter = scm_sdk::airs::gateway::mcp_tags_filter([("env", "prod"), ("team", "sec")])?;
/// assert_eq!(filter, r#"{"env":"prod","team":"sec"}"#);
/// # Ok(()) }
/// ```
pub fn mcp_tags_filter<I, K, V>(tags: I) -> Result<String>
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
{
    let mut sorted = BTreeMap::new();
    for (key, value) in tags {
        if sorted
            .insert(key.as_ref().to_owned(), value.as_ref().to_owned())
            .is_some()
        {
            return Err(Error::new(ErrorKind::Validation));
        }
    }
    let encoded = serde_json::to_string(&sorted).map_err(|_| Error::new(ErrorKind::Validation))?;
    validate_mcp_tags_filter(&encoded)?;
    Ok(encoded)
}

/// Reject a `tags` filter that is not a bounded JSON object of unique keys with
/// string values. Runs before authentication; errors never echo the filter.
pub(super) fn validate_mcp_tags_filter(filter: &str) -> Result<()> {
    let invalid = || Error::new(ErrorKind::Validation);
    if filter.len() > MAX_FILTER_BYTES {
        return Err(invalid());
    }
    // Shared bounded/protocol-key guard first, then a strict single-pass decode:
    // serde_json would otherwise keep the last of two duplicate keys silently.
    crate::json_decode::from_slice(filter.as_bytes()).map_err(|_| invalid())?;
    serde_json::from_str::<StrictStringMap>(filter)
        .map(|_| ())
        .map_err(|_| invalid())
}

struct StrictStringMap;

impl<'de> serde::Deserialize<'de> for StrictStringMap {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StrictStringMap;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a JSON object of unique string keys and string values")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> std::result::Result<Self::Value, A::Error> {
                let mut seen = std::collections::BTreeSet::new();
                while let Some(key) = map.next_key::<String>()? {
                    map.next_value::<String>()?;
                    if !seen.insert(key) {
                        return Err(serde::de::Error::custom("duplicate tag key"));
                    }
                }
                Ok(StrictStringMap)
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}

/// Validate the effective serialized `tags` filter before any dispatch. The
/// typed field and an extension-supplied `tags` key are both covered, since
/// either can reach the query string.
pub(super) fn check_filter<T: serde::Serialize>(query: &T) -> Result<()> {
    let invalid = || Error::new(ErrorKind::Validation);
    let value = serde_json::to_value(query).map_err(|_| invalid())?;
    match value.as_object().and_then(|object| object.get("tags")) {
        None => Ok(()),
        Some(serde_json::Value::String(filter)) => validate_mcp_tags_filter(filter),
        Some(_) => Err(invalid()),
    }
}
