//! Caller-driven, bounded page and item walks.
//!
//! A walk shares one absolute request deadline. Collections retain a resumable
//! position on truncation or failure. Service listings are not snapshots: resumed
//! pages may have changed, so callers decide whether resource IDs need deduplication.

use crate::{Error, ErrorKind, RequestOptions, Result, Session};
use futures_util::future::BoxFuture;
use serde::Serialize;
use std::{
    collections::{HashSet, VecDeque},
    fmt,
    future::Future,
    hash::Hash,
};

pub struct Page<T, C> {
    pub items: Vec<T>,
    pub next: Option<C>,
}
impl<T, C> fmt::Debug for Page<T, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Page")
            .field("item_count", &self.items.len())
            .field("has_next", &self.next.is_some())
            .finish()
    }
}

/// Re-fetch this page and skip the already-consumed items. Cursor data is redacted.
#[derive(Clone, PartialEq, Eq)]
pub struct Resume<C> {
    pub cursor: C,
    pub item_offset: usize,
}
impl<C> fmt::Debug for Resume<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Resume")
            .field("cursor", &"[REDACTED]")
            .field("item_offset", &self.item_offset)
            .finish()
    }
}

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct CollectOptions {
    pub max_items: usize,
    /// Maximum serialized JSON array size, including commas and brackets.
    pub max_bytes: usize,
    pub max_pages: usize,
}
impl Default for CollectOptions {
    fn default() -> Self {
        Self {
            max_items: 10_000,
            max_bytes: 64 * 1024 * 1024,
            max_pages: 1_000,
        }
    }
}
impl CollectOptions {
    pub fn with_max_items(mut self, value: usize) -> Result<Self> {
        self.max_items = value;
        self.validate()?;
        Ok(self)
    }
    pub fn with_max_bytes(mut self, value: usize) -> Result<Self> {
        self.max_bytes = value;
        self.validate()?;
        Ok(self)
    }
    pub fn with_max_pages(mut self, value: usize) -> Result<Self> {
        self.max_pages = value;
        self.validate()?;
        Ok(self)
    }
    pub fn validate(&self) -> Result<()> {
        if self.max_items == 0
            || self.max_items > 1_000_000
            || self.max_bytes < 2
            || self.max_bytes > 512 * 1024 * 1024
            || self.max_pages == 0
            || self.max_pages > 100_000
        {
            Err(Error::new(ErrorKind::Validation))
        } else {
            Ok(())
        }
    }
}

/// Bounds apply to serialized cursor values retained by a walk.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct CursorLimits {
    pub max_bytes: usize,
    pub max_retained_bytes: usize,
}
impl Default for CursorLimits {
    fn default() -> Self {
        Self {
            max_bytes: 4096,
            max_retained_bytes: 4 * 1024 * 1024,
        }
    }
}
impl CursorLimits {
    pub fn validate(&self) -> Result<()> {
        if self.max_bytes == 0
            || self.max_bytes > 1024 * 1024
            || self.max_retained_bytes < self.max_bytes
            || self.max_retained_bytes > 64 * 1024 * 1024
        {
            Err(Error::new(ErrorKind::Validation))
        } else {
            Ok(())
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Truncation {
    Items,
    Bytes,
    Pages,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Completion {
    Complete,
    Truncated(Truncation),
    Interrupted,
}

pub struct Collection<T, C> {
    pub items: Vec<T>,
    pub completion: Completion,
    pub resume: Option<Resume<C>>,
    pub pages_read: usize,
    pub wire_bytes: usize,
}
impl<T, C> Collection<T, C> {
    pub fn is_complete(&self) -> bool {
        self.completion == Completion::Complete
    }
}
impl<T, C> fmt::Debug for Collection<T, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Collection")
            .field("item_count", &self.items.len())
            .field("completion", &self.completion)
            .field("has_resume", &self.resume.is_some())
            .field("pages_read", &self.pages_read)
            .field("wire_bytes", &self.wire_bytes)
            .finish()
    }
}
pub type CollectionResult<T, C> = std::result::Result<Collection<T, C>, CollectionError<T, C>>;

/// A failed walk retains accepted rows and its resume position separately from the safe error.
pub struct CollectionError<T, C> {
    error: Error,
    partial: Box<Collection<T, C>>,
}
impl<T, C> CollectionError<T, C> {
    pub fn error(&self) -> &Error {
        &self.error
    }
    pub fn partial(&self) -> &Collection<T, C> {
        &self.partial
    }
    pub fn into_error(self) -> Error {
        self.error
    }
    pub fn into_parts(self) -> (Error, Collection<T, C>) {
        (self.error, *self.partial)
    }
}
impl<T, C> From<Error> for CollectionError<T, C> {
    fn from(error: Error) -> Self {
        Self {
            error,
            partial: Box::new(Collection {
                items: Vec::new(),
                completion: Completion::Interrupted,
                resume: None,
                pages_read: 0,
                wire_bytes: 2,
            }),
        }
    }
}
impl<T, C> fmt::Debug for CollectionError<T, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CollectionError")
            .field("error", &self.error)
            .field("partial", &self.partial)
            .finish()
    }
}
impl<T, C> fmt::Display for CollectionError<T, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(f)
    }
}
impl<T, C> std::error::Error for CollectionError<T, C> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

type Fetch<'a, T, C> =
    Box<dyn FnMut(C, RequestOptions) -> BoxFuture<'a, Result<Page<T, C>>> + Send + 'a>;
pub struct Pages<'a, T, C> {
    fetch: Fetch<'a, T, C>,
    cursor: Option<C>,
    seen: HashSet<C>,
    options: RequestOptions,
    session: Session,
    max_pages: usize,
    pages_read: usize,
    failure: Option<Error>,
    cursor_limits: CursorLimits,
    retained_cursor_bytes: usize,
}
impl<T, C> fmt::Debug for Pages<'_, T, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Pages")
            .field("pages_read", &self.pages_read)
            .field("max_pages", &self.max_pages)
            .field("has_resume", &self.cursor.is_some())
            .field("failed", &self.failure.is_some())
            .field("cursor_limits", &self.cursor_limits)
            .field("retained_cursor_bytes", &self.retained_cursor_bytes)
            .finish()
    }
}
impl<'a, T: 'a, C: Clone + Eq + Hash + Serialize + Send + 'a> Pages<'a, T, C> {
    pub fn new<F, Fut>(
        initial: C,
        options: RequestOptions,
        session: Session,
        max_pages: usize,
        fetch: F,
    ) -> Result<Self>
    where
        F: FnMut(C, RequestOptions) -> Fut + Send + 'a,
        Fut: Future<Output = Result<Page<T, C>>> + Send + 'a,
    {
        Self::new_with_cursor_limits(
            initial,
            options,
            session,
            max_pages,
            CursorLimits::default(),
            fetch,
        )
    }
    pub fn new_with_cursor_limits<F, Fut>(
        initial: C,
        options: RequestOptions,
        session: Session,
        max_pages: usize,
        cursor_limits: CursorLimits,
        mut fetch: F,
    ) -> Result<Self>
    where
        F: FnMut(C, RequestOptions) -> Fut + Send + 'a,
        Fut: Future<Output = Result<Page<T, C>>> + Send + 'a,
    {
        if max_pages == 0 || max_pages > 100_000 {
            return Err(Error::new(ErrorKind::Validation));
        }
        cursor_limits.validate()?;
        crate::engine::serialize_bounded(&initial, cursor_limits.max_bytes)?;
        Ok(Self {
            fetch: Box::new(move |cursor, options| Box::pin(fetch(cursor, options))),
            cursor: Some(initial),
            seen: HashSet::new(),
            options,
            session,
            max_pages,
            pages_read: 0,
            failure: None,
            cursor_limits,
            retained_cursor_bytes: 0,
        })
    }
    pub fn pages_read(&self) -> usize {
        self.pages_read
    }
    pub fn resume(&self) -> Option<Resume<C>> {
        self.cursor.clone().map(|cursor| Resume {
            cursor,
            item_offset: 0,
        })
    }
    async fn check_active(&mut self) -> Result<()> {
        if let Some(error) = &self.failure {
            return Err(error.clone());
        }
        let result = self.options.run(&self.session, async { Ok(()) }).await;
        if let Err(error) = &result {
            self.failure = Some(error.clone());
        }
        result
    }
    pub async fn next_page(&mut self) -> Result<Option<Page<T, C>>> {
        if let Some(error) = &self.failure {
            return Err(error.clone());
        }
        let Some(_) = &self.cursor else {
            return Ok(None);
        };
        self.check_active().await?;
        let cursor = self
            .cursor
            .as_ref()
            .ok_or_else(|| Error::new(ErrorKind::Pagination))?;
        if self.pages_read >= self.max_pages || self.seen.contains(cursor) {
            let error = Error::new(ErrorKind::Pagination).with_operation("pagination.pages");
            self.failure = Some(error.clone());
            return Err(error);
        }
        let bytes = match crate::engine::serialize_bounded(cursor, self.cursor_limits.max_bytes) {
            Ok(bytes) => bytes,
            Err(error) => {
                self.failure = Some(error.clone());
                return Err(error);
            }
        };
        if self.retained_cursor_bytes.saturating_add(bytes.len())
            > self.cursor_limits.max_retained_bytes
        {
            let error = Error::new(ErrorKind::Limit).with_operation("pagination.cursor_history");
            self.failure = Some(error.clone());
            return Err(error);
        }
        self.retained_cursor_bytes += bytes.len();
        let cursor = cursor.clone();
        self.seen.insert(cursor.clone());
        let result = self
            .options
            .run(&self.session, (self.fetch)(cursor, self.options.clone()))
            .await;
        let page = match result {
            Ok(page) => page,
            Err(error) => {
                self.failure = Some(error.clone());
                return Err(error);
            }
        };
        if let Some(next) = &page.next
            && let Err(error) = crate::engine::serialize_bounded(next, self.cursor_limits.max_bytes)
        {
            let error = error.with_operation("pagination.cursor");
            self.failure = Some(error.clone());
            return Err(error);
        }
        self.pages_read += 1;
        if page.items.is_empty() && page.next.is_some() {
            let error = Error::new(ErrorKind::Pagination).with_operation("pagination.empty_page");
            self.failure = Some(error.clone());
            return Err(error);
        }
        self.cursor = page.next.clone();
        Ok(Some(page))
    }
    pub fn items(self) -> Items<'a, T, C> {
        Items {
            pages: self,
            pending: VecDeque::new(),
            active_cursor: None,
            item_offset: 0,
            initial_skip: 0,
        }
    }
    pub async fn collect(self, options: CollectOptions) -> CollectionResult<T, C>
    where
        T: Serialize,
    {
        self.items().collect(options).await
    }
}

pub struct Items<'a, T, C> {
    pages: Pages<'a, T, C>,
    pending: VecDeque<T>,
    active_cursor: Option<C>,
    item_offset: usize,
    initial_skip: usize,
}
impl<T, C> fmt::Debug for Items<'_, T, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Items")
            .field("pages", &self.pages)
            .field("buffered_items", &self.pending.len())
            .field("item_offset", &self.item_offset)
            .finish()
    }
}
impl<'a, T: 'a, C: Clone + Eq + Hash + Serialize + Send + 'a> Items<'a, T, C> {
    /// Apply a saved position before this iterator has performed any network work.
    pub fn resume_from(mut self, position: Resume<C>) -> Result<Self> {
        if self.pages.pages_read != 0 || !self.pages.seen.is_empty() || !self.pending.is_empty() {
            return Err(Error::new(ErrorKind::Validation));
        }
        crate::engine::serialize_bounded(&position.cursor, self.pages.cursor_limits.max_bytes)?;
        self.pages.cursor = Some(position.cursor);
        self.initial_skip = position.item_offset;
        Ok(self)
    }
    pub fn resume(&self) -> Option<Resume<C>> {
        if !self.pending.is_empty() {
            self.active_cursor.clone().map(|cursor| Resume {
                cursor,
                item_offset: self.item_offset,
            })
        } else {
            self.pages.cursor.clone().map(|cursor| Resume {
                cursor,
                item_offset: self.initial_skip,
            })
        }
    }
    pub fn pages_read(&self) -> usize {
        self.pages.pages_read
    }
    async fn load_page(&mut self) -> Result<bool> {
        if !self.pending.is_empty() {
            return Ok(true);
        }
        let cursor = self.pages.cursor.clone();
        let Some(page) = self.pages.next_page().await? else {
            return Ok(false);
        };
        if self.initial_skip > page.items.len() {
            let error = Error::new(ErrorKind::Pagination).with_operation("pagination.resume");
            self.pages.failure = Some(error.clone());
            self.pages.cursor = cursor;
            return Err(error);
        }
        self.item_offset = self.initial_skip;
        self.pending = page.items.into_iter().skip(self.initial_skip).collect();
        self.initial_skip = 0;
        self.active_cursor = cursor;
        Ok(true)
    }
    async fn ensure_item(&mut self) -> Result<bool> {
        while self.pending.is_empty() {
            if !self.load_page().await? {
                return Ok(false);
            }
        }
        Ok(true)
    }
    pub async fn next(&mut self) -> Result<Option<T>> {
        // Check cancellation even when the next item is already buffered.
        self.pages.check_active().await?;
        if !self.ensure_item().await? {
            return Ok(None);
        }
        self.item_offset += 1;
        Ok(self.pending.pop_front())
    }
    pub async fn collect(mut self, options: CollectOptions) -> CollectionResult<T, C>
    where
        T: Serialize,
    {
        options.validate()?;
        let mut items = Vec::new();
        let mut wire_bytes = 2usize;
        let max_pages = options.max_pages.min(self.pages.max_pages);
        loop {
            if let Some(error) = &self.pages.failure {
                return Err(CollectionError {
                    error: error.clone(),
                    partial: Box::new(Collection {
                        items,
                        completion: Completion::Interrupted,
                        resume: self.resume(),
                        pages_read: self.pages.pages_read,
                        wire_bytes,
                    }),
                });
            }
            let completion = if self.resume().is_none() {
                Some(Completion::Complete)
            } else if items.len() >= options.max_items {
                Some(Completion::Truncated(Truncation::Items))
            } else if self.pending.is_empty() && self.pages.pages_read >= max_pages {
                Some(Completion::Truncated(Truncation::Pages))
            } else {
                None
            };
            if let Some(completion) = completion {
                return Ok(Collection {
                    items,
                    completion,
                    resume: self.resume(),
                    pages_read: self.pages.pages_read,
                    wire_bytes,
                });
            }
            let active = self.pages.check_active().await;
            let ready = match active {
                Ok(()) => self.load_page().await,
                Err(error) => Err(error),
            };
            match ready {
                Ok(false) => {
                    return Ok(Collection {
                        items,
                        completion: Completion::Complete,
                        resume: None,
                        pages_read: self.pages.pages_read,
                        wire_bytes,
                    });
                }
                Err(error) => {
                    return Err(CollectionError {
                        error,
                        partial: Box::new(Collection {
                            items,
                            completion: Completion::Interrupted,
                            resume: self.resume(),
                            pages_read: self.pages.pages_read,
                            wire_bytes,
                        }),
                    });
                }
                Ok(true) if self.pending.is_empty() => continue,
                Ok(true) => {}
            }
            let comma = usize::from(!items.is_empty());
            let remaining = options
                .max_bytes
                .saturating_sub(wire_bytes)
                .saturating_sub(comma);
            let encoded = crate::engine::serialize_bounded(
                self.pending
                    .front()
                    .ok_or_else(|| Error::new(ErrorKind::Pagination))?,
                remaining,
            );
            match encoded {
                Ok(bytes) => wire_bytes += bytes.len() + comma,
                Err(error) if error.kind() == ErrorKind::Limit => {
                    return Ok(Collection {
                        items,
                        completion: Completion::Truncated(Truncation::Bytes),
                        resume: self.resume(),
                        pages_read: self.pages.pages_read,
                        wire_bytes,
                    });
                }
                Err(error) => {
                    return Err(CollectionError {
                        error,
                        partial: Box::new(Collection {
                            items,
                            completion: Completion::Interrupted,
                            resume: self.resume(),
                            pages_read: self.pages.pages_read,
                            wire_bytes,
                        }),
                    });
                }
            }
            self.item_offset += 1;
            if let Some(item) = self.pending.pop_front() {
                items.push(item);
            }
        }
    }
}

#[cfg(test)]
mod tests;
