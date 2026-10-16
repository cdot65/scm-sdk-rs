//! Native multipart projections and format-dependent audio responses.
use super::super::models::*;
use super::{RuntimeClient, common};
use crate::{
    Error, ErrorKind, RawRequest, RequestOptions, Result,
    data::{Optional, Patch},
    multipart::Multipart,
};
use http::Method;
use serde::Serialize;
use serde_json::{Map, Value};

#[derive(Clone, PartialEq)]
#[non_exhaustive]
pub enum TranscriptionResult {
    Json(GatewayInferenceCreateTranscriptionResponseJson),
    VerboseJson(GatewayInferenceCreateTranscriptionResponseVerboseJson),
    Text(String),
}
#[derive(Clone, PartialEq)]
#[non_exhaustive]
pub enum TranslationResult {
    Json(GatewayInferenceCreateTranslationResponseJson),
    VerboseJson(GatewayInferenceCreateTranslationResponseVerboseJson),
    Text(String),
}
crate::redaction::redacted_debug!(TranscriptionResult);
crate::redaction::redacted_debug!(TranslationResult);
struct Metadata {
    map: Map<String, Value>,
    remaining: usize,
}
impl Metadata {
    fn new(extensions: &Extensions<crate::data::JsonValue>, limit: usize) -> Result<Self> {
        // All five multipart schemas are strict, including future extension keys.
        if !extensions.is_empty() {
            return Err(Error::new(ErrorKind::Validation));
        }
        Ok(Self {
            map: Map::new(),
            remaining: limit.saturating_sub(2),
        })
    }
    fn value<T: Serialize>(&mut self, key: &str, value: &T) -> Result<()> {
        let bytes = crate::engine::serialize_bounded(value, self.remaining)?;
        self.remaining = self
            .remaining
            .checked_sub(bytes.len() + key.len() + 4)
            .ok_or_else(|| Error::new(ErrorKind::Limit))?;
        let value =
            serde_json::from_slice(&bytes).map_err(|_| Error::new(ErrorKind::Validation))?;
        if self.map.insert(key.into(), value).is_some() {
            return Err(Error::new(ErrorKind::Validation));
        }
        Ok(())
    }
    fn optional<T: Serialize>(&mut self, key: &str, value: &Optional<T>) -> Result<()> {
        if let Optional::Value(value) = value {
            self.value(key, value)?;
        }
        Ok(())
    }
    fn patch<T: Serialize>(&mut self, key: &str, value: &Patch<T>) -> Result<()> {
        match value {
            Patch::Unset => Ok(()),
            Patch::Null => self.value(key, &Value::Null),
            Patch::Value(value) => self.value(key, value),
        }
    }
}
impl RuntimeClient {
    async fn multipart(
        &self,
        request: RawRequest,
        schema: &str,
        metadata: impl FnOnce() -> Result<Metadata>,
        binaries: &[(&str, &BinaryFile)],
        keys: &[&str],
        options: Option<&RequestOptions>,
    ) -> Result<(RawRequest, RequestOptions)> {
        let options = self.client.operation_options(options)?;
        let (body, mime) = options
            .run(&self.client.session(), async {
                let metadata = metadata()?;
                let metadata = crate::validation::parse_multipart_with_limits(
                    schema,
                    Value::Object(metadata.map),
                    binaries,
                    &self
                        .client
                        .contract_limits(self.client.limits().max_request_bytes)?,
                    self.client.limits().max_request_bytes,
                )?;
                let mut form = Multipart::new(self.client.limits().max_request_bytes)?;
                for key in keys {
                    if let Some((_, file)) = binaries.iter().find(|(name, _)| name == key) {
                        form.file(key, file)?;
                        continue;
                    }
                    let Some(value) = metadata.get(*key) else {
                        continue;
                    };
                    if value.is_null() {
                        continue;
                    }
                    if let Value::Array(values) = value {
                        let array_key = if key.ends_with("[]") {
                            (*key).to_owned()
                        } else {
                            format!("{key}[]")
                        };
                        for value in values {
                            form.text(&array_key, &common::scalar(value)?)?;
                        }
                    } else {
                        form.text(key, &common::scalar(value)?)?;
                    }
                }
                form.finish()
            })
            .await?;
        Ok((request.body(body, &mime)?, options))
    }
    pub async fn create_image_edit(
        &self,
        body: &GatewayInferenceInputCreateImageEditRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceImagesResponse> {
        const KEYS: &[&str] = &[
            "image",
            "prompt",
            "mask",
            "model",
            "n",
            "size",
            "response_format",
            "user",
        ];
        let mut binaries = vec![("image", &body.image)];
        if let Optional::Value(file) = &body.mask {
            binaries.push(("mask", file));
        }
        let (request, options) = self
            .multipart(
                self.request(
                    Method::POST,
                    "/images/edits",
                    &[],
                    "gateway.create_image_edit",
                )?,
                "GatewayInferenceInputCreateImageEditRequestSchema",
                || {
                    let mut metadata =
                        Metadata::new(&body.extensions, self.client.limits().max_request_bytes)?;
                    metadata.value("prompt", &body.prompt)?;
                    metadata.patch("model", &body.model)?;
                    metadata.patch("n", &body.n)?;
                    metadata.patch("size", &body.size)?;
                    metadata.patch("response_format", &body.response_format)?;
                    metadata.optional("user", &body.user)?;
                    Ok(metadata)
                },
                &binaries,
                KEYS,
                options,
            )
            .await?;
        self.client
            .send_contract(
                request,
                "GatewayInferenceImagesResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn create_image_variation(
        &self,
        body: &GatewayInferenceInputCreateImageVariationRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceImagesResponse> {
        const KEYS: &[&str] = &["image", "model", "n", "response_format", "size", "user"];
        let (request, options) = self
            .multipart(
                self.request(
                    Method::POST,
                    "/images/variations",
                    &[],
                    "gateway.create_image_variation",
                )?,
                "GatewayInferenceInputCreateImageVariationRequestSchema",
                || {
                    let mut metadata =
                        Metadata::new(&body.extensions, self.client.limits().max_request_bytes)?;
                    metadata.patch("model", &body.model)?;
                    metadata.patch("n", &body.n)?;
                    metadata.patch("response_format", &body.response_format)?;
                    metadata.patch("size", &body.size)?;
                    metadata.optional("user", &body.user)?;
                    Ok(metadata)
                },
                &[("image", &body.image)],
                KEYS,
                options,
            )
            .await?;
        self.client
            .send_contract(
                request,
                "GatewayInferenceImagesResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn create_file(
        &self,
        body: &GatewayInferenceInputCreateFileRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceOpenAIFile> {
        const KEYS: &[&str] = &["file", "purpose"];
        let (request, options) = self
            .multipart(
                self.request(Method::POST, "/files", &[], "gateway.create_file")?,
                "GatewayInferenceInputCreateFileRequestSchema",
                || {
                    let mut metadata =
                        Metadata::new(&body.extensions, self.client.limits().max_request_bytes)?;
                    metadata.value("purpose", &body.purpose)?;
                    Ok(metadata)
                },
                &[("file", &body.file)],
                KEYS,
                options,
            )
            .await?;
        self.client
            .send_contract(request, "GatewayInferenceOpenAIFileSchema", Some(&options))
            .await
    }
    pub async fn create_transcription(
        &self,
        body: &GatewayInferenceInputCreateTranscriptionRequest,
        options: Option<&RequestOptions>,
    ) -> Result<TranscriptionResult> {
        const KEYS: &[&str] = &[
            "file",
            "model",
            "language",
            "prompt",
            "response_format",
            "temperature",
            "timestamp_granularities[]",
        ];
        let (request, options) = self
            .multipart(
                self.request(
                    Method::POST,
                    "/audio/transcriptions",
                    &[],
                    "gateway.create_transcription",
                )?,
                "GatewayInferenceInputCreateTranscriptionRequestSchema",
                || {
                    let mut metadata =
                        Metadata::new(&body.extensions, self.client.limits().max_request_bytes)?;
                    metadata.value("model", &body.model)?;
                    metadata.optional("language", &body.language)?;
                    metadata.optional("prompt", &body.prompt)?;
                    metadata.optional("response_format", &body.response_format)?;
                    metadata.optional("temperature", &body.temperature)?;
                    metadata
                        .optional("timestamp_granularities[]", &body.timestamp_granularities__)?;
                    Ok(metadata)
                },
                &[("file", &body.file)],
                KEYS,
                options,
            )
            .await?;
        let format = match &body.response_format {
            Optional::Unset => "json",
            Optional::Value(format) => format.as_str(),
        };
        match format {
            "text" | "srt" | "vtt" => self
                .client
                .send_text(request, Some(&options))
                .await
                .map(TranscriptionResult::Text),
            "verbose_json" => self
                .client
                .send_contract(
                    request,
                    "GatewayInferenceCreateTranscriptionResponseVerboseJsonSchema",
                    Some(&options),
                )
                .await
                .map(TranscriptionResult::VerboseJson),
            _ => self
                .client
                .send_contract(
                    request,
                    "GatewayInferenceCreateTranscriptionResponseJsonSchema",
                    Some(&options),
                )
                .await
                .map(TranscriptionResult::Json),
        }
    }
    pub async fn create_translation(
        &self,
        body: &GatewayInferenceInputCreateTranslationRequest,
        options: Option<&RequestOptions>,
    ) -> Result<TranslationResult> {
        const KEYS: &[&str] = &["file", "model", "prompt", "response_format", "temperature"];
        let (request, options) = self
            .multipart(
                self.request(
                    Method::POST,
                    "/audio/translations",
                    &[],
                    "gateway.create_translation",
                )?,
                "GatewayInferenceInputCreateTranslationRequestSchema",
                || {
                    let mut metadata =
                        Metadata::new(&body.extensions, self.client.limits().max_request_bytes)?;
                    metadata.value("model", &body.model)?;
                    metadata.optional("prompt", &body.prompt)?;
                    metadata.optional("response_format", &body.response_format)?;
                    metadata.optional("temperature", &body.temperature)?;
                    Ok(metadata)
                },
                &[("file", &body.file)],
                KEYS,
                options,
            )
            .await?;
        let format = match &body.response_format {
            Optional::Unset => "json",
            Optional::Value(format) => format.as_str(),
        };
        match format {
            "text" | "srt" | "vtt" => self
                .client
                .send_text(request, Some(&options))
                .await
                .map(TranslationResult::Text),
            "verbose_json" => self
                .client
                .send_contract(
                    request,
                    "GatewayInferenceCreateTranslationResponseVerboseJsonSchema",
                    Some(&options),
                )
                .await
                .map(TranslationResult::VerboseJson),
            _ => self
                .client
                .send_contract(
                    request,
                    "GatewayInferenceCreateTranslationResponseJsonSchema",
                    Some(&options),
                )
                .await
                .map(TranslationResult::Json),
        }
    }
    pub async fn create_speech(
        &self,
        body: &GatewayInferenceInputCreateSpeechRequest,
        options: Option<&RequestOptions>,
    ) -> Result<bytes::Bytes> {
        let (request, options) = self
            .input(
                self.request(Method::POST, "/audio/speech", &[], "gateway.create_speech")?,
                body,
                "GatewayInferenceInputCreateSpeechRequestSchema",
                None,
                options,
            )
            .await?;
        self.client.send_bytes(request, Some(&options)).await
    }
    pub async fn create_logs(
        &self,
        body: &GatewayInferenceInputCreateLogsRequest,
        options: Option<&RequestOptions>,
    ) -> Result<String> {
        let (request, options) = self
            .input(
                self.request(Method::POST, "/logs", &[], "gateway.create_logs")?,
                body,
                "GatewayInferenceInputCreateLogsRequestSchema",
                None,
                options,
            )
            .await?;
        self.client.send_text(request, Some(&options)).await
    }
}
