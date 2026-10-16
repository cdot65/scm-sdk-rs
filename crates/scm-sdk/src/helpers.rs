//! Generic, bounded native equivalents of the public SDK helpers.
use crate::data::{FiniteNumber, Patch};
use crate::pagination::{
    CollectOptions, CollectionResult, Completion, Items, Page, Pages, Truncation,
};
use crate::{Client, ClientBuilder, Error, ErrorKind, RequestOptions, Result, Session};
use futures_util::{Stream, StreamExt};
use serde::Serialize;
use serde_json::Value;
use std::{fmt, future::Future, hash::Hash, pin::Pin};

const SAFE_INTEGER: u64 = 9_007_199_254_740_991;
pub(crate) const MAX_HELPER_BYTES: usize = 8 * 1024 * 1024;
pub(crate) const MAX_HELPER_NODES: usize = 100_000;
pub(crate) const MAX_HELPER_DEPTH: usize = 64;

pub(crate) fn bounded_json(input: &Value) -> Result<()> {
    let mut pending = vec![(input, 0usize)];
    let mut nodes = 0usize;
    while let Some((value, depth)) = pending.pop() {
        nodes += 1;
        if depth > MAX_HELPER_DEPTH || nodes > MAX_HELPER_NODES {
            return Err(Error::new(ErrorKind::Limit));
        }
        let children = match value {
            Value::Array(v) => v.len(),
            Value::Object(v) => v.len(),
            _ => 0,
        };
        if nodes.saturating_add(pending.len()).saturating_add(children) > MAX_HELPER_NODES {
            return Err(Error::new(ErrorKind::Limit));
        }
        match value {
            Value::Array(values) => pending.extend(values.iter().map(|v| (v, depth + 1))),
            Value::Object(values) => pending.extend(values.values().map(|v| (v, depth + 1))),
            Value::Number(number) if number.as_f64().is_none_or(|v| !v.is_finite()) => {
                return Err(Error::new(ErrorKind::Validation));
            }
            _ => {}
        }
    }
    crate::engine::serialize_bounded(input, MAX_HELPER_BYTES)?;
    Ok(())
}

/// Build an immutable client configuration. No global singleton is modified.
pub fn init(builder: ClientBuilder) -> Result<Client> {
    builder.build()
}

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ListingOptions {
    pub skip: Option<u64>,
    pub limit: Option<u32>,
    pub search: Option<String>,
}
impl fmt::Debug for ListingOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ListingOptions([REDACTED])")
    }
}
/// Preserve canonical key order and explicit zero/empty values; omit only unset fields.
pub fn serialize_listing(options: Option<&ListingOptions>) -> Result<Vec<(String, String)>> {
    let mut query = Vec::new();
    if let Some(options) = options {
        if let Some(skip) = options.skip {
            if skip > SAFE_INTEGER {
                return Err(Error::new(ErrorKind::Validation));
            }
            query.push(("skip".into(), skip.to_string()));
        }
        if let Some(limit) = options.limit {
            query.push(("limit".into(), limit.to_string()));
        }
        if let Some(search) = &options.search {
            if search.len() > crate::config::MAX_URL_BYTES || search.chars().any(char::is_control) {
                return Err(Error::new(ErrorKind::Validation));
            }
            query.push(("search".into(), search.clone()));
        }
    }
    Ok(query)
}

/// Cursor-based item iteration with a caller-owned session and one absolute budget.
pub fn paginate<'a, T: 'a, C, F, Fut>(
    initial: C,
    fetch: F,
    max_pages: usize,
    session: Session,
    options: RequestOptions,
) -> Result<Items<'a, T, C>>
where
    C: Clone + Eq + Hash + Serialize + Send + 'a,
    F: FnMut(C, RequestOptions) -> Fut + Send + 'a,
    Fut: Future<Output = Result<Page<T, C>>> + Send + 'a,
{
    Ok(Pages::new(initial, options, session, max_pages, fetch)?.items())
}

#[derive(Clone, Copy)]
#[non_exhaustive]
pub struct WalkAllOptions {
    pub limit: u32,
    pub collect: CollectOptions,
}
impl Default for WalkAllOptions {
    fn default() -> Self {
        Self {
            limit: 50,
            collect: CollectOptions::default(),
        }
    }
}
#[derive(Clone, Copy)]
#[non_exhaustive]
pub struct SpringWalkOptions {
    pub size: u32,
    pub collect: CollectOptions,
}
impl Default for SpringWalkOptions {
    fn default() -> Self {
        Self {
            size: 50,
            collect: CollectOptions::default(),
        }
    }
}
pub struct SkipPage<T> {
    pub items: Vec<T>,
    pub total: Option<FiniteNumber>,
}
pub struct SpringPage<T> {
    pub items: Vec<T>,
    pub last: bool,
}
crate::redaction::redacted_debug!(WalkAllOptions, SpringWalkOptions);
impl<T> fmt::Debug for SkipPage<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SkipPage([REDACTED])")
    }
}
impl<T> fmt::Debug for SpringPage<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SpringPage([REDACTED])")
    }
}

pub async fn collect_skip_pages<'a, T: Serialize + Send + 'a, F, Fut>(
    mut fetch: F,
    walk: WalkAllOptions,
    session: Session,
    options: RequestOptions,
) -> CollectionResult<T, u64>
where
    F: FnMut(u64, u32, RequestOptions) -> Fut + Send + 'a,
    Fut: Future<Output = Result<SkipPage<T>>> + Send + 'a,
{
    walk.collect.validate()?;
    if walk.limit == 0 {
        return Err(Error::new(ErrorKind::Validation).into());
    }
    let pages = Pages::new(
        0u64,
        options,
        session,
        walk.collect.max_pages,
        move |skip, options| {
            let future = fetch(skip, walk.limit, options);
            async move {
                let page = future.await?;
                let next = skip
                    .checked_add(page.items.len() as u64)
                    .filter(|v| *v <= SAFE_INTEGER)
                    .ok_or_else(|| Error::new(ErrorKind::Pagination))?;
                let total = page
                    .total
                    .map(|number| {
                        let value = number
                            .as_f64()
                            .ok_or_else(|| Error::new(ErrorKind::Pagination))?;
                        if value < 0.0 || value > SAFE_INTEGER as f64 || value.fract() != 0.0 {
                            return Err(Error::new(ErrorKind::Pagination));
                        }
                        Ok(value as u64)
                    })
                    .transpose()?;
                let more = !page.items.is_empty()
                    && total.map_or(page.items.len() == walk.limit as usize, |total| {
                        next < total
                    });
                Ok(Page {
                    items: page.items,
                    next: more.then_some(next),
                })
            }
        },
    )?;
    pages.collect(walk.collect).await
}
pub async fn collect_spring_pages<'a, T: Serialize + Send + 'a, F, Fut>(
    mut fetch: F,
    walk: SpringWalkOptions,
    session: Session,
    options: RequestOptions,
) -> CollectionResult<T, u64>
where
    F: FnMut(u64, u32, RequestOptions) -> Fut + Send + 'a,
    Fut: Future<Output = Result<SpringPage<T>>> + Send + 'a,
{
    walk.collect.validate()?;
    if walk.size == 0 {
        return Err(Error::new(ErrorKind::Validation).into());
    }
    let pages = Pages::new(
        0u64,
        options,
        session,
        walk.collect.max_pages,
        move |number, options| {
            let future = fetch(number, walk.size, options);
            async move {
                let page = future.await?;
                if !page.last && page.items.is_empty() {
                    return Err(Error::new(ErrorKind::Pagination));
                }
                let next = if page.last {
                    None
                } else {
                    Some(
                        number
                            .checked_add(1)
                            .filter(|v| *v <= SAFE_INTEGER)
                            .ok_or_else(|| Error::new(ErrorKind::Pagination))?,
                    )
                };
                Ok(Page {
                    items: page.items,
                    next,
                })
            }
        },
    )?;
    pages.collect(walk.collect).await
}

/// Retains a pulled item when a byte cap prevents accepting it; collection can resume.
pub struct StreamListing<S: Stream> {
    stream: Pin<Box<S>>,
    pending: Option<S::Item>,
    ended: bool,
}
impl<S: Stream> StreamListing<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream: Box::pin(stream),
            pending: None,
            ended: false,
        }
    }
}
impl<S: Stream> fmt::Debug for StreamListing<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamListing([REDACTED])")
    }
}
pub struct StreamCollection<T> {
    pub items: Vec<T>,
    pub completion: Completion,
    pub wire_bytes: usize,
}
impl<T> StreamCollection<T> {
    pub fn is_complete(&self) -> bool {
        self.completion == Completion::Complete
    }
}
impl<T> fmt::Debug for StreamCollection<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamCollection")
            .field("item_count", &self.items.len())
            .field("completion", &self.completion)
            .finish()
    }
}
pub struct StreamCollectionError<T> {
    error: Error,
    partial: Box<StreamCollection<T>>,
}
impl<T> StreamCollectionError<T> {
    pub fn error(&self) -> &Error {
        &self.error
    }
    pub fn partial(&self) -> &StreamCollection<T> {
        &self.partial
    }
    pub fn into_error(self) -> Error {
        self.error
    }
    pub fn into_parts(self) -> (Error, StreamCollection<T>) {
        (self.error, *self.partial)
    }
    fn interrupted(error: Error, mut partial: StreamCollection<T>) -> Self {
        partial.completion = Completion::Interrupted;
        Self {
            error,
            partial: Box::new(partial),
        }
    }
}
impl<T> From<Error> for StreamCollectionError<T> {
    fn from(error: Error) -> Self {
        Self::interrupted(
            error,
            StreamCollection {
                items: Vec::new(),
                completion: Completion::Interrupted,
                wire_bytes: 2,
            },
        )
    }
}
impl<T> fmt::Debug for StreamCollectionError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamCollectionError")
            .field("error", &self.error)
            .field("partial", &self.partial)
            .finish()
    }
}
impl<T> fmt::Display for StreamCollectionError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(f)
    }
}
impl<T> std::error::Error for StreamCollectionError<T> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}
pub type StreamCollectionResult<T> =
    std::result::Result<StreamCollection<T>, StreamCollectionError<T>>;
/// Collect any fallible Rust stream. Item/byte caps retain the iterator and partial rows.
/// The page cap applies to page iterators; an arbitrary item stream has no pages.
pub async fn collect_all<S, T>(
    listing: &mut StreamListing<S>,
    caps: CollectOptions,
    session: &Session,
    options: &RequestOptions,
) -> StreamCollectionResult<T>
where
    S: Stream<Item = Result<T>>,
    T: Serialize,
{
    caps.validate()?;
    let mut result = StreamCollection {
        items: Vec::new(),
        completion: Completion::Complete,
        wire_bytes: 2,
    };
    loop {
        if let Err(error) = options.run(session, async { Ok(()) }).await {
            return Err(StreamCollectionError::interrupted(error, result));
        }
        if listing.ended {
            return Ok(result);
        }
        if result.items.len() >= caps.max_items {
            result.completion = Completion::Truncated(Truncation::Items);
            return Ok(result);
        }
        let item = if let Some(item) = listing.pending.take() {
            Some(item)
        } else {
            match options
                .run(session, async { Ok(listing.stream.next().await) })
                .await
            {
                Ok(item) => item,
                Err(error) => {
                    return Err(StreamCollectionError::interrupted(error, result));
                }
            }
        };
        let Some(item) = item else {
            listing.ended = true;
            return Ok(result);
        };
        let item = match item {
            Ok(item) => item,
            Err(error) => {
                return Err(StreamCollectionError::interrupted(error, result));
            }
        };
        let comma = usize::from(!result.items.is_empty());
        let remaining = caps
            .max_bytes
            .saturating_sub(result.wire_bytes)
            .saturating_sub(comma);
        match crate::engine::serialize_bounded(&item, remaining) {
            Ok(bytes) => {
                result.wire_bytes += bytes.len() + comma;
                result.items.push(item);
            }
            Err(error) if error.kind() == ErrorKind::Limit => {
                listing.pending = Some(Ok(item));
                result.completion = Completion::Truncated(Truncation::Bytes);
                return Ok(result);
            }
            Err(error) => {
                listing.pending = Some(Ok(item));
                return Err(StreamCollectionError::interrupted(error, result));
            }
        }
    }
}

/// Native schema-composition seam: only present nonnull values reach the inner parser.
pub fn json_nullable<T>(
    input: Option<&Value>,
    inner: impl FnOnce(&Value) -> Result<T>,
) -> Result<Patch<T>> {
    match input {
        None => Ok(Patch::Unset),
        Some(Value::Null) => Ok(Patch::Null),
        Some(value) => {
            bounded_json(value)?;
            inner(value).map(Patch::Value)
        }
    }
}

/// Decode a generic Spring envelope and validate each content item with the caller's parser.
#[cfg(feature = "models")]
pub fn page_schema<T>(
    input: &Value,
    mut inner: impl FnMut(&Value) -> Result<T>,
) -> Result<crate::dlp::models::Page<T>> {
    bounded_json(input)?;
    let source: crate::dlp::models::Page<crate::data::JsonValue> =
        serde_json::from_value(input.clone()).map_err(|_| Error::new(ErrorKind::Validation))?;
    let mut page = crate::dlp::models::Page::new(
        source
            .content
            .iter()
            .map(|item| inner(item.as_value()))
            .collect::<Result<_>>()?,
    );
    page.empty = source.empty;
    page.first = source.first;
    page.last = source.last;
    page.number = source.number;
    page.number_of_elements = source.number_of_elements;
    page.pageable = source.pageable;
    page.size = source.size;
    page.sort = source.sort;
    page.total_elements = source.total_elements;
    page.total_pages = source.total_pages;
    page.extensions = source.extensions;
    Ok(page)
}

/// Generate a scope name locally; its random suffix is not an authentication secret.
#[cfg(feature = "iam")]
pub fn generate_workspace_scope_name(name: &str) -> Result<String> {
    if name.len() > 4096 {
        return Err(Error::new(ErrorKind::Limit));
    }
    let mut stem = String::new();
    let mut separator = false;
    for ch in name.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_lowercase() || ch.is_ascii_digit() {
            if separator && !stem.is_empty() {
                stem.push('_');
            }
            stem.push(ch);
            separator = false;
        } else {
            separator = true;
        }
    }
    if stem.is_empty() {
        stem.push_str("workspace");
    }
    const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut suffix = String::new();
    // Rejection sampling removes modulo bias while bounding a failing entropy source.
    for _ in 0..16 {
        let mut bytes = [0u8; 16];
        getrandom::fill(&mut bytes).map_err(|_| Error::new(ErrorKind::Io))?;
        for byte in bytes {
            if byte < 252 {
                suffix.push(ALPHABET[usize::from(byte) % 36] as char);
                if suffix.len() == 6 {
                    return Ok(format!("ws_{stem}_{suffix}"));
                }
            }
        }
    }
    Err(Error::new(ErrorKind::Io))
}
