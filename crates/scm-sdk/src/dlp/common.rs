use crate::{
    Error, ErrorKind, Result,
    data::{FiniteNumber, Optional},
    pagination::Page,
};
use std::fmt;

const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

/// Zero-based Spring paging. Sort entries are emitted separately and in caller order.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct SpringListParams {
    pub page: Option<u64>,
    pub size: Option<u32>,
    pub sort: Vec<String>,
}
impl SpringListParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn page(mut self, value: u64) -> Self {
        self.page = Some(value);
        self
    }
    pub fn size(mut self, value: u32) -> Self {
        self.size = Some(value);
        self
    }
    pub fn sort(mut self, values: impl IntoIterator<Item = String>) -> Self {
        self.sort = values.into_iter().collect();
        self
    }
}
impl fmt::Debug for SpringListParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SpringListParams([REDACTED])")
    }
}
pub type DataPatternListParams = SpringListParams;
pub type DataProfileListParams = SpringListParams;

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct DictionaryListParams {
    pub page: Option<u64>,
    pub size: Option<u32>,
    pub sort: Vec<String>,
    pub keywords: Option<bool>,
}
impl fmt::Debug for DictionaryListParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DictionaryListParams([REDACTED])")
    }
}

/// Request-only lifecycle filter. It cannot express an unfamiliar server status.
#[derive(Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FilteringStatus {
    Enabled,
    Disabled,
}
impl FilteringStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
        }
    }
}
impl fmt::Debug for FilteringStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FilteringStatus([REDACTED])")
    }
}

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct DataFilteringProfileListParams {
    pub page: Option<u64>,
    pub size: Option<u32>,
    pub sort: Vec<String>,
    pub status: Option<FilteringStatus>,
    pub name: Option<String>,
}
impl fmt::Debug for DataFilteringProfileListParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DataFilteringProfileListParams([REDACTED])")
    }
}

pub(super) fn query(
    page: Option<u64>,
    size: Option<u32>,
    sort: &[String],
) -> Result<Vec<(String, String)>> {
    if page.is_some_and(|value| value > MAX_SAFE_INTEGER)
        || size == Some(0)
        || sort.len() > 128
        || sort.iter().any(|value| {
            value.is_empty() || value.len() > 1024 || value.chars().any(char::is_control)
        })
    {
        return Err(Error::new(ErrorKind::Validation));
    }
    let mut query = Vec::new();
    if let Some(value) = page {
        query.push(("page".into(), value.to_string()));
    }
    if let Some(value) = size {
        query.push(("size".into(), value.to_string()));
    }
    query.extend(sort.iter().map(|value| ("sort".into(), value.clone())));
    Ok(query)
}
pub(super) fn filter(value: &str) -> Result<()> {
    if value.len() > 4096 || value.chars().any(char::is_control) {
        Err(Error::new(ErrorKind::Validation))
    } else {
        Ok(())
    }
}
pub(super) fn spring<T>(
    items: Vec<T>,
    last: Optional<bool>,
    total_pages: Optional<FiniteNumber>,
    page: u64,
    size: u32,
) -> Result<Page<T, u64>> {
    let last = match last {
        Optional::Value(value) => value,
        Optional::Unset => match total_pages {
            Optional::Value(value) => {
                let total = value
                    .as_f64()
                    .ok_or_else(|| Error::new(ErrorKind::Pagination))?;
                if total < 0.0 || total.fract() != 0.0 || total > MAX_SAFE_INTEGER as f64 {
                    return Err(Error::new(ErrorKind::Pagination));
                }
                page as f64 + 1.0 >= total
            }
            Optional::Unset => items.len() < (size as usize),
        },
    };
    if !last && items.is_empty() {
        return Err(Error::new(ErrorKind::Pagination));
    }
    let next = if last {
        None
    } else {
        Some(
            page.checked_add(1)
                .filter(|value| *value <= MAX_SAFE_INTEGER)
                .ok_or_else(|| Error::new(ErrorKind::Pagination))?,
        )
    };
    Ok(Page { items, next })
}
