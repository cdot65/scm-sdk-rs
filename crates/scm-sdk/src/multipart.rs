//! Bounded native multipart encoding. No transport-specific or background uploads.
use crate::{Error, ErrorKind, Result};
use bytes::Bytes;
use std::{
    fmt,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_BOUNDARY: AtomicU64 = AtomicU64::new(0);
struct Part {
    headers: String,
    bytes: Bytes,
}
pub(crate) struct Multipart {
    parts: Vec<Part>,
    max_bytes: usize,
    reserved: usize,
}
impl fmt::Debug for Multipart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Multipart")
            .field("part_count", &self.parts.len())
            .field("max_bytes", &self.max_bytes)
            .finish()
    }
}
impl Multipart {
    pub(crate) fn new(max_bytes: usize) -> Result<Self> {
        if max_bytes == 0 || max_bytes > 512 * 1024 * 1024 {
            return Err(Error::new(ErrorKind::Configuration));
        }
        Ok(Self {
            parts: Vec::new(),
            max_bytes,
            reserved: 30,
        })
    }
    #[cfg(test)]
    fn text(&mut self, name: &str, value: &str) -> Result<()> {
        if value.len() > self.max_bytes {
            return Err(Error::new(ErrorKind::Limit));
        }
        self.part(name, None, None, Bytes::copy_from_slice(value.as_bytes()))
    }
    pub(crate) fn file(&mut self, name: &str, file: &crate::models::BinaryFile) -> Result<()> {
        file.validate(self.max_bytes)?;
        self.part(
            name,
            Some(file.filename().unwrap_or("blob")),
            Some(file.content_type().unwrap_or("application/octet-stream")),
            file.bytes().clone(),
        )
    }
    pub(crate) fn part(
        &mut self,
        name: &str,
        filename: Option<&str>,
        content_type: Option<&str>,
        bytes: Bytes,
    ) -> Result<()> {
        if name.is_empty()
            || name.len() > 128
            || !name
                .bytes()
                .all(|v| v.is_ascii_alphanumeric() || b"_-.[ ]".contains(&v))
        {
            return Err(Error::new(ErrorKind::Validation));
        }
        let mut headers = format!("Content-Disposition: form-data; name=\"{name}\"");
        if let Some(filename) = filename {
            if filename.is_empty()
                || filename.len() > 255
                || filename == "."
                || filename == ".."
                || filename
                    .chars()
                    .any(|v| v.is_control() || matches!(v, '/' | '\\' | '"'))
            {
                return Err(Error::new(ErrorKind::Validation));
            }
            headers.push_str(&format!("; filename=\"{filename}\""));
        }
        headers.push_str("\r\n");
        if let Some(value) = content_type {
            if value.len() > 255
                || !value.is_ascii()
                || value.bytes().any(|v| v < 32 || v == 127)
                || value.parse::<mime::Mime>().is_err()
            {
                return Err(Error::new(ErrorKind::Validation));
            }
            headers.push_str(&format!("Content-Type: {value}\r\n"));
        }
        headers.push_str("\r\n");
        let total = self
            .reserved
            .checked_add(headers.len())
            .and_then(|n| n.checked_add(bytes.len()))
            .and_then(|n| n.checked_add(30))
            .ok_or_else(|| Error::new(ErrorKind::Limit))?;
        if total > self.max_bytes || self.parts.len() >= 128 {
            return Err(Error::new(ErrorKind::Limit));
        }
        self.reserved = total;
        self.parts.push(Part { headers, bytes });
        Ok(())
    }
    pub(crate) fn finish(self) -> Result<(Bytes, String)> {
        if self.parts.is_empty() {
            return Err(Error::new(ErrorKind::Validation));
        }
        let mut selected = None;
        // Delimiters contain no payload-derived information. Scan all parts to guarantee
        // that even caller-controlled bytes cannot inject the chosen delimiter.
        for _ in 0..16 {
            let sequence = NEXT_BOUNDARY.fetch_add(1, Ordering::Relaxed);
            let boundary = format!("scm-sdk-{sequence:016x}");
            if self.parts.iter().all(|part| {
                !part
                    .bytes
                    .windows(boundary.len())
                    .any(|v| v == boundary.as_bytes())
                    && !part.headers.contains(&boundary)
            }) {
                selected = Some(boundary);
                break;
            }
        }
        let boundary = selected.ok_or_else(|| Error::new(ErrorKind::Limit))?;
        let mut result = Vec::with_capacity(self.reserved.min(self.max_bytes));
        for part in self.parts {
            result.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
            result.extend_from_slice(part.headers.as_bytes());
            result.extend_from_slice(&part.bytes);
            result.extend_from_slice(b"\r\n");
        }
        result.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
        if result.len() > self.max_bytes {
            return Err(Error::new(ErrorKind::Limit));
        }
        Ok((
            Bytes::from(result),
            format!("multipart/form-data; boundary={boundary}"),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bytes_headers_and_terminator_are_exact() {
        let mut form = Multipart::new(4096).unwrap();
        form.text("name", "private").unwrap();
        let file = crate::models::BinaryFile::new(Bytes::from_static(&[0, 255, 10]))
            .unwrap()
            .with_filename("file.bin")
            .unwrap();
        form.file("file", &file).unwrap();
        let (body, content_type) = form.finish().unwrap();
        let boundary = content_type
            .strip_prefix("multipart/form-data; boundary=")
            .unwrap();
        let mut expected=format!("--{boundary}\r\nContent-Disposition: form-data; name=\"name\"\r\n\r\nprivate\r\n--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"file.bin\"\r\nContent-Type: application/octet-stream\r\n\r\n").into_bytes();
        expected.extend_from_slice(&[0, 255, 10]);
        expected.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
        assert_eq!(body.as_ref(), expected);
    }
    #[test]
    fn injection_count_and_envelope_limits_are_local() {
        for name in ["", "a\r\nb", "a\"b", "a\\b"] {
            assert!(Multipart::new(4096).unwrap().text(name, "x").is_err());
        }
        for name in ["../file", "x\r\nY: secret", "x\"y", "x\\y"] {
            assert!(
                Multipart::new(4096)
                    .unwrap()
                    .part("file", Some(name), None, Bytes::new())
                    .is_err()
            );
        }
        assert_eq!(
            Multipart::new(100)
                .unwrap()
                .text("a", "b")
                .unwrap_err()
                .kind(),
            ErrorKind::Limit
        );
        assert!(
            Multipart::new(4096)
                .unwrap()
                .part("file", None, Some("text/plain\r\nx:y"), Bytes::new())
                .is_err()
        );
        let mut form = Multipart::new(1024 * 1024).unwrap();
        for _ in 0..128 {
            form.text("a", "b").unwrap();
        }
        assert!(form.text("a", "b").is_err());
        assert!(!format!("{form:?}").contains("Content-Disposition"));
    }
}
