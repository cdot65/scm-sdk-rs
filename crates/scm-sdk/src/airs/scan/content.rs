use super::models::{ScanRequestContentsInner, ToolEvent};
use crate::{Error, ErrorKind, Result, data::Optional};
use std::{fmt, io::Read, path::Path};
use tokio::io::AsyncReadExt;

const TEXT_LIMIT: usize = 2 * 1024 * 1024;
const CONTEXT_LIMIT: usize = 100 * 1024 * 1024;

/// Validated AIRS content. Context supplements a prompt, response, code, or tool event.
#[derive(Clone)]
pub struct Content {
    inner: ScanRequestContentsInner,
}
#[derive(Default)]
pub struct ContentBuilder {
    prompt: Option<String>,
    response: Option<String>,
    context: Option<String>,
    code_prompt: Option<String>,
    code_response: Option<String>,
    tool_event: Option<ToolEvent>,
}
impl ContentBuilder {
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }
    pub fn response(mut self, value: impl Into<String>) -> Self {
        self.response = Some(value.into());
        self
    }
    pub fn context(mut self, value: impl Into<String>) -> Self {
        self.context = Some(value.into());
        self
    }
    pub fn code_prompt(mut self, value: impl Into<String>) -> Self {
        self.code_prompt = Some(value.into());
        self
    }
    pub fn code_response(mut self, value: impl Into<String>) -> Self {
        self.code_response = Some(value.into());
        self
    }
    pub fn tool_event(mut self, value: ToolEvent) -> Self {
        self.tool_event = Some(value);
        self
    }
    pub fn build(self) -> Result<Content> {
        Content::from_json(
            ScanRequestContentsInner::builder()
                .prompt(optional(self.prompt))
                .response(optional(self.response))
                .context(optional(self.context))
                .code_prompt(optional(self.code_prompt))
                .code_response(optional(self.code_response))
                .tool_event(optional(self.tool_event))
                .build()?,
        )
    }
}
fn optional<T>(value: Option<T>) -> Optional<T> {
    value.map_or(Optional::Unset, Optional::Value)
}
fn text(value: &Optional<String>) -> Option<&str> {
    match value {
        Optional::Unset => None,
        Optional::Value(value) => Some(value),
    }
}
impl Content {
    pub fn builder() -> ContentBuilder {
        ContentBuilder::default()
    }
    pub fn from_json(mut value: ScanRequestContentsInner) -> Result<Self> {
        validate_content(&value)?;
        // The baseline Content helper retains its six declared fields only.
        value.extensions = Default::default();
        Ok(Self { inner: value })
    }
    pub fn from_json_slice(value: &[u8]) -> Result<Self> {
        let model = serde_json::from_slice(value).map_err(|_| Error::new(ErrorKind::Validation))?;
        Self::from_json(model)
    }
    /// Read a JSON document within an explicit bound. This synchronous helper
    /// uses caller-owned I/O; async applications can read then call from_json_slice.
    pub fn from_reader(reader: impl Read, max_bytes: usize) -> Result<Self> {
        if max_bytes == 0 || max_bytes > 512 * 1024 * 1024 {
            return Err(Error::new(ErrorKind::Configuration));
        }
        let mut bytes = Vec::new();
        reader
            .take((max_bytes + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(|_| Error::new(ErrorKind::Io))?;
        if bytes.len() > max_bytes {
            return Err(Error::new(ErrorKind::Limit));
        }
        Self::from_json_slice(&bytes)
    }
    /// Explicit async file input, bounded before parsing. The application owns
    /// the runtime; no path or operating-system error text enters SDK formatting.
    pub async fn from_json_file(path: impl AsRef<Path>, max_bytes: usize) -> Result<Self> {
        if max_bytes == 0 || max_bytes > 512 * 1024 * 1024 {
            return Err(Error::new(ErrorKind::Configuration));
        }
        let reader = tokio::fs::File::open(path)
            .await
            .map_err(|_| Error::new(ErrorKind::Io))?;
        let mut bytes = Vec::new();
        reader
            .take((max_bytes + 1) as u64)
            .read_to_end(&mut bytes)
            .await
            .map_err(|_| Error::new(ErrorKind::Io))?;
        if bytes.len() > max_bytes {
            return Err(Error::new(ErrorKind::Limit));
        }
        Self::from_json_slice(&bytes)
    }
    pub fn to_json(&self) -> &ScanRequestContentsInner {
        &self.inner
    }
    pub fn into_json(self) -> ScanRequestContentsInner {
        self.inner
    }
    pub fn prompt(&self) -> Option<&str> {
        text(&self.inner.prompt)
    }
    pub fn response(&self) -> Option<&str> {
        text(&self.inner.response)
    }
    pub fn context(&self) -> Option<&str> {
        text(&self.inner.context)
    }
    pub fn code_prompt(&self) -> Option<&str> {
        text(&self.inner.code_prompt)
    }
    pub fn code_response(&self) -> Option<&str> {
        text(&self.inner.code_response)
    }
    pub fn tool_event(&self) -> Option<&ToolEvent> {
        match &self.inner.tool_event {
            Optional::Unset => None,
            Optional::Value(value) => Some(value),
        }
    }
    pub fn length(&self) -> usize {
        [
            self.prompt(),
            self.response(),
            self.context(),
            self.code_prompt(),
            self.code_response(),
        ]
        .into_iter()
        .flatten()
        .map(str::len)
        .sum()
    }
    pub fn set_prompt(&mut self, value: Option<String>) -> Result<()> {
        self.update(|m| m.prompt = optional(value))
    }
    pub fn set_response(&mut self, value: Option<String>) -> Result<()> {
        self.update(|m| m.response = optional(value))
    }
    pub fn set_context(&mut self, value: Option<String>) -> Result<()> {
        self.update(|m| m.context = optional(value))
    }
    pub fn set_code_prompt(&mut self, value: Option<String>) -> Result<()> {
        self.update(|m| m.code_prompt = optional(value))
    }
    pub fn set_code_response(&mut self, value: Option<String>) -> Result<()> {
        self.update(|m| m.code_response = optional(value))
    }
    pub fn set_tool_event(&mut self, value: Option<ToolEvent>) -> Result<()> {
        self.update(|m| m.tool_event = optional(value))
    }
    fn update(&mut self, change: impl FnOnce(&mut ScanRequestContentsInner)) -> Result<()> {
        let mut candidate = self.inner.clone();
        change(&mut candidate);
        validate_content(&candidate)?;
        self.inner = candidate;
        Ok(())
    }
}
pub(super) fn validate_content(value: &ScanRequestContentsInner) -> Result<()> {
    if [
        text(&value.prompt),
        text(&value.response),
        text(&value.code_prompt),
        text(&value.code_response),
    ]
    .into_iter()
    .flatten()
    .all(str::is_empty)
        && value.tool_event.is_unset()
    {
        return Err(Error::new(ErrorKind::Validation));
    }
    for field in [
        &value.prompt,
        &value.response,
        &value.code_prompt,
        &value.code_response,
    ] {
        if text(field).is_some_and(|value| value.len() > TEXT_LIMIT) {
            return Err(Error::new(ErrorKind::Validation));
        }
    }
    if text(&value.context).is_some_and(|value| value.len() > CONTEXT_LIMIT) {
        return Err(Error::new(ErrorKind::Validation));
    }
    Ok(())
}
impl fmt::Debug for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Content([REDACTED])")
    }
}
impl fmt::Debug for ContentBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ContentBuilder([REDACTED])")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn byte_limits_and_required_content_are_transactional() {
        assert!(Content::builder().build().is_err());
        assert!(Content::builder().context("onlycontext").build().is_err());
        let mut value = Content::builder().prompt("é").build().unwrap();
        assert_eq!(value.length(), 2);
        assert!(value.set_prompt(None).is_err());
        assert_eq!(value.prompt(), Some("é"));
        assert!(
            value
                .set_response(Some("é".repeat(TEXT_LIMIT / 2 + 1)))
                .is_err()
        );
        assert!(value.response().is_none());
        value.set_response(Some("valid".into())).unwrap();
        value.set_prompt(None).unwrap();
        assert_eq!(value.length(), 5);
        assert!(!format!("{value:?}").contains("valid"));
    }
    #[test]
    fn explicit_file_reader_is_bounded_and_safe() {
        assert!(Content::from_reader(br#"{"prompt":"hello"}"#.as_slice(), 64).is_ok());
        assert_eq!(
            Content::from_reader(br#"{"prompt":"hello"}"#.as_slice(), 4)
                .unwrap_err()
                .kind(),
            ErrorKind::Limit
        );
        assert!(
            !format!(
                "{:?}",
                Content::from_json_slice(br#"{"prompt":12,"secret":"echo"}"#).unwrap_err()
            )
            .contains("echo")
        );
    }
}
