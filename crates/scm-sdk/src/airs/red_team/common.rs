use super::models::RedTeamPagination;
use crate::{Error, ErrorKind, Result, data::Patch, pagination::Page};
pub(super) const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

/// Skip/limit/search values are omitted on direct reads unless explicitly supplied.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct RedTeamListOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
}

impl RedTeamListOptions {
    pub(super) fn query(&self) -> Result<Vec<(String, String)>> {
        listing(self.skip, self.limit, self.search.as_deref())
    }
}
pub(super) fn listing(
    skip: Option<u64>,
    limit: Option<u32>,
    search: Option<&str>,
) -> Result<Vec<(String, String)>> {
    if skip.is_some_and(|value| value > MAX_SAFE_INTEGER) || limit == Some(0) {
        return Err(Error::new(ErrorKind::Validation));
    }
    let mut values = Vec::new();
    if let Some(value) = skip {
        values.push(("skip".into(), value.to_string()));
    }
    if let Some(value) = limit {
        values.push(("limit".into(), value.to_string()));
    }
    push_filter(&mut values, "search", search)?;
    Ok(values)
}
pub(super) fn filter(value: &str) -> Result<()> {
    if value.len() > 4096 || value.chars().any(char::is_control) {
        Err(Error::new(ErrorKind::Validation))
    } else {
        Ok(())
    }
}
pub(super) fn push_filter(
    values: &mut Vec<(String, String)>,
    key: &str,
    value: Option<&str>,
) -> Result<()> {
    if let Some(value) = value {
        filter(value)?;
        values.push((key.into(), value.into()));
    }
    Ok(())
}
pub(super) fn uuid(value: &str) -> Result<()> {
    if crate::data::is_uuid(value) {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Validation))
    }
}
/// Match collectSkipPages: an empty page ends the walk even if totals are stale.
pub(super) fn skip_page<T>(
    items: Vec<T>,
    pagination: RedTeamPagination,
    skip: u64,
    limit: u32,
) -> Result<Page<T, u64>> {
    if items.is_empty() {
        return Ok(Page { items, next: None });
    }
    let next = skip
        .checked_add(u64::try_from(items.len()).map_err(|_| Error::new(ErrorKind::Pagination))?)
        .filter(|value| *value <= MAX_SAFE_INTEGER)
        .ok_or_else(|| Error::new(ErrorKind::Pagination))?;
    let more = match pagination.total_items {
        Patch::Unset | Patch::Null => items.len() == limit as usize,
        Patch::Value(total) => {
            let total = total
                .as_f64()
                .ok_or_else(|| Error::new(ErrorKind::Pagination))?;
            if total < 0.0 || total.fract() != 0.0 || total > MAX_SAFE_INTEGER as f64 {
                return Err(Error::new(ErrorKind::Pagination));
            }
            (next as f64) < total
        }
    };
    Ok(Page {
        items,
        next: more.then_some(next),
    })
}

crate::redaction::redacted_debug!(RedTeamListOptions);
