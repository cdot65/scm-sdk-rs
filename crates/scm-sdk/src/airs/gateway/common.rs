//! Shared source-compatible Gateway query and native date adapters.
use crate::{Client, Error, ErrorKind, RequestOptions, Result, validation::NativeDate};
use serde::Serialize;
use serde_json::Value;

pub(super) fn invalid() -> Error {
    Error::new(ErrorKind::Validation)
}

pub(super) fn oauth(client: Client) -> Result<Client> {
    client.require_auth(crate::engine::AuthKind::OAuth)?;
    client.with_tsg_header(true)
}

/// Validate before authentication, then preserve the schema's declared key order.
/// Arrays become repeated query values in their original order.
pub(super) async fn query<T: Serialize>(
    client: &Client,
    input: &T,
    schema: &str,
    keys: &[&str],
    options: &RequestOptions,
) -> Result<Vec<(String, String)>> {
    let input = options
        .run(&client.session(), async {
            let bytes = crate::engine::serialize_bounded(input, crate::config::MAX_URL_BYTES)?;
            crate::json_decode::from_slice(&bytes).map_err(|_| invalid())
        })
        .await?;
    let bytes = client.validated_json(&input, schema, options).await?;
    options
        .run(&client.session(), async {
            let value: Value = crate::json_decode::from_slice(&bytes).map_err(|_| invalid())?;
            let object = value.as_object().ok_or_else(invalid)?;
            if object.keys().any(|key| !keys.contains(&key.as_str())) {
                return Err(invalid());
            }
            let mut result = Vec::new();
            let mut total_bytes = 0usize;
            for key in keys {
                match object.get(*key) {
                    None | Some(Value::Null) => {}
                    Some(Value::Array(values)) => {
                        if values.len() > 256usize.saturating_sub(result.len()) {
                            return Err(Error::new(ErrorKind::Limit));
                        }
                        for value in values {
                            push_query(&mut result, &mut total_bytes, key, value)?;
                        }
                    }
                    Some(value) => push_query(&mut result, &mut total_bytes, key, value)?,
                }
            }
            Ok(result)
        })
        .await
}

fn push_query(
    result: &mut Vec<(String, String)>,
    total: &mut usize,
    key: &str,
    value: &Value,
) -> Result<()> {
    if result.len() >= 256 {
        return Err(Error::new(ErrorKind::Limit));
    }
    let length = value.as_str().map(str::len).unwrap_or(32);
    *total = total
        .checked_add(key.len())
        .and_then(|n| n.checked_add(length))
        .filter(|n| *n <= crate::config::MAX_URL_BYTES)
        .ok_or_else(|| Error::new(ErrorKind::Limit))?;
    result.push((key.to_owned(), scalar(value)?));
    Ok(())
}

pub(super) fn scalar(value: &Value) -> Result<String> {
    match value {
        Value::String(value) => Ok(value.clone()),
        Value::Bool(value) => Ok(value.to_string()),
        Value::Number(value) => {
            let value = value
                .as_f64()
                .filter(|v| v.is_finite())
                .ok_or_else(invalid)?;
            Ok(ryu_js::Buffer::new().format_finite(value).to_owned())
        }
        _ => Err(invalid()),
    }
}

pub(super) fn uuid(value: &str) -> Result<()> {
    if crate::data::is_uuid(value) {
        Ok(())
    } else {
        Err(invalid())
    }
}
pub(super) fn numeric_id(value: &str) -> Result<()> {
    if !value.is_empty() && value.len() <= 512 && value.bytes().all(|v| v.is_ascii_digit()) {
        Ok(())
    } else {
        Err(invalid())
    }
}
pub(super) fn workspace_ref(value: &str) -> Result<()> {
    if value.len() <= 512
        && value
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_alphanumeric)
        && value
            .bytes()
            .all(|v| v.is_ascii_alphanumeric() || b"_-".contains(&v))
    {
        Ok(())
    } else {
        Err(invalid())
    }
}
pub(super) fn js_whitespace(value: char) -> bool {
    matches!(value, '\u{0009}'..='\u{000d}' | '\u{0020}' | '\u{00a0}' | '\u{1680}' |
        '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' |
        '\u{3000}' | '\u{feff}')
}
pub(super) fn js_trim(value: &str) -> &str {
    value.trim_matches(js_whitespace)
}
pub(super) fn path_segment(value: &str) -> Result<()> {
    if value.len() > crate::config::MAX_URL_BYTES {
        return Err(Error::new(ErrorKind::Limit));
    }
    if value.chars().all(js_whitespace) || matches!(value, "." | "..") {
        Err(invalid())
    } else {
        Ok(())
    }
}
pub(super) fn resource_id(value: &str) -> Result<()> {
    if value.len() > 2048 {
        return Err(invalid());
    }
    path_segment(value)?;
    if value.encode_utf16().count() > 512 {
        Err(invalid())
    } else {
        Ok(())
    }
}

/// Gregorian calendar arithmetic over the entire JavaScript Date range.
/// Euclidean division is essential for instants before the Unix epoch.
fn components(millis: i64) -> (i64, i64, i64, i64, i64, i64, i64) {
    let days = millis.div_euclid(86_400_000);
    let within = millis.rem_euclid(86_400_000);
    let shifted = days + 719_468;
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    (
        year,
        month,
        day,
        within / 3_600_000,
        within / 60_000 % 60,
        within / 1000 % 60,
        within % 1000,
    )
}
pub(super) fn date_iso(value: &NativeDate) -> Result<String> {
    let (year, month, day, hour, minute, second, millis) = components(value.epoch_millis());
    let year = if (0..=9999).contains(&year) {
        format!("{year:04}")
    } else {
        format!(
            "{}{abs:06}",
            if year < 0 { '-' } else { '+' },
            abs = year.unsigned_abs()
        )
    };
    Ok(format!(
        "{year}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{millis:03}Z"
    ))
}
pub(super) fn date_offset_iso(value: &NativeDate, offset_minutes: i16) -> Result<String> {
    if !(-1439..=1439).contains(&offset_minutes) {
        return Err(invalid());
    }
    let shifted = value
        .epoch_millis()
        .checked_add(i64::from(offset_minutes) * 60_000)
        .ok_or_else(invalid)?;
    let (year, month, day, hour, minute, second, _) = components(shifted);
    if !(0..=9999).contains(&year) {
        return Err(invalid());
    }
    let offset = offset_minutes.unsigned_abs();
    Ok(format!(
        "{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}{}{hours:02}:{minutes:02}",
        if offset_minutes < 0 { '-' } else { '+' },
        hours = offset / 60,
        minutes = offset % 60
    ))
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum AIGatewayPlane {
    #[default]
    Data,
    Admin,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum AIGatewayWorkspaceStatus {
    Active,
    Archived,
}
#[derive(Clone, Default, Debug)]
#[non_exhaustive]
pub struct AIGatewayWorkspaceListOptions {
    pub status: Option<AIGatewayWorkspaceStatus>,
    pub plane: AIGatewayPlane,
}
#[derive(Clone, Copy, Default, Debug)]
#[non_exhaustive]
pub struct AIGatewayWorkspaceGetOptions {
    pub plane: AIGatewayPlane,
}
#[derive(Clone)]
#[non_exhaustive]
pub struct AIGatewayWorkspaceScopedListOptions {
    pub workspace_id: String,
}
impl AIGatewayWorkspaceScopedListOptions {
    pub fn new(workspace_id: impl Into<String>) -> Self {
        Self {
            workspace_id: workspace_id.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_dates_match_literal_javascript_iso_vectors_over_the_entire_range() {
        for (epoch, expected) in [
            (0, "1970-01-01T00:00:00.000Z"),
            (-1, "1969-12-31T23:59:59.999Z"),
            (951_782_400_000, "2000-02-29T00:00:00.000Z"),
            (-62_167_219_200_000, "0000-01-01T00:00:00.000Z"),
            (-62_167_219_200_001, "-000001-12-31T23:59:59.999Z"),
            (253_402_300_799_999, "9999-12-31T23:59:59.999Z"),
            (253_402_300_800_000, "+010000-01-01T00:00:00.000Z"),
            (-8_640_000_000_000_000, "-271821-04-20T00:00:00.000Z"),
            (8_640_000_000_000_000, "+275760-09-13T00:00:00.000Z"),
        ] {
            assert_eq!(
                date_iso(&NativeDate::from_epoch_millis(epoch).unwrap()).unwrap(),
                expected
            );
        }
    }

    #[test]
    fn offset_iso_and_js_number_and_whitespace_rules_are_literal() {
        let date = NativeDate::from_epoch_millis(-1).unwrap();
        assert_eq!(
            date_offset_iso(&date, 120).unwrap(),
            "1970-01-01T01:59:59+02:00"
        );
        assert_eq!(
            date_offset_iso(&date, -330).unwrap(),
            "1969-12-31T18:29:59-05:30"
        );
        assert!(date_offset_iso(&date, 1440).is_err());
        assert!(
            date_offset_iso(
                &NativeDate::from_epoch_millis(8_640_000_000_000_000).unwrap(),
                0
            )
            .is_err()
        );
        for (value, expected) in [
            (0.0, "0"),
            (-0.0, "0"),
            (1e-7, "1e-7"),
            (1e-6, "0.000001"),
            (1e21, "1e+21"),
        ] {
            assert_eq!(scalar(&serde_json::json!(value)).unwrap(), expected);
        }
        assert_eq!(js_trim("\u{feff}\u{00a0}x\u{2028}"), "x");
        assert_eq!(js_trim("\u{0085}"), "\u{0085}");
        assert!(path_segment("\u{feff}").is_err());
        assert!(path_segment("\u{0085}").is_ok());
        assert!(resource_id(&"😀".repeat(256)).is_ok());
        assert!(resource_id(&"😀".repeat(257)).is_err());
    }

    #[tokio::test]
    async fn ordered_query_adapter_keeps_repeated_values_zero_and_empty_with_finite_bounds() {
        struct NoNetwork;
        impl crate::transport::Transport for NoNetwork {
            fn send<'a>(
                &'a self,
                _: crate::transport::TransportRequest,
            ) -> futures_util::future::BoxFuture<'a, Result<crate::transport::TransportResponse>>
            {
                panic!("query validation must not dispatch")
            }
        }
        let client = Client::builder(crate::Endpoint::parse("https://query.example.test").unwrap())
            .transport(std::sync::Arc::new(NoNetwork))
            .build()
            .unwrap();
        let options = RequestOptions::with_timeout(std::time::Duration::from_secs(10)).unwrap();
        let values = serde_json::json!({"before":"b", "after":"", "order":"desc", "limit":0});
        assert_eq!(
            query(
                &client,
                &values,
                "GatewayInferenceInputListInputItemsQuerySchema",
                &["limit", "order", "after", "before"],
                &options
            )
            .await
            .unwrap(),
            [
                ("limit", "0"),
                ("order", "desc"),
                ("after", ""),
                ("before", "b")
            ]
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
        );
        let values = serde_json::json!({"include":["file_search_call.results", "message.input_image.image_url", "file_search_call.results"]});
        assert_eq!(
            query(
                &client,
                &values,
                "GatewayInferenceInputGetResponseQuerySchema",
                &["include"],
                &options
            )
            .await
            .unwrap(),
            [
                ("include", "file_search_call.results"),
                ("include", "message.input_image.image_url"),
                ("include", "file_search_call.results")
            ]
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
        );
        for values in [
            serde_json::json!({"include":vec!["file_search_call.results";257]}),
            serde_json::json!({"include":vec!["file_search_call.results";10000]}),
        ] {
            assert_eq!(
                query(
                    &client,
                    &values,
                    "GatewayInferenceInputGetResponseQuerySchema",
                    &["include"],
                    &options
                )
                .await
                .unwrap_err()
                .kind(),
                ErrorKind::Limit
            );
        }
        let cancelled = options.clone();
        cancelled.cancel();
        assert_eq!(
            query(
                &client,
                &serde_json::json!({}),
                "GatewayInferenceInputGetResponseQuerySchema",
                &["include"],
                &cancelled
            )
            .await
            .unwrap_err()
            .kind(),
            ErrorKind::Cancelled
        );
    }
}

crate::redaction::redacted_debug!(AIGatewayWorkspaceScopedListOptions);
