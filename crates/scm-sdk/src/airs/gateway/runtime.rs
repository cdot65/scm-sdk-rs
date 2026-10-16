use super::models::{
    GatewayChatCompletion, GatewayChatCompletionChunk, GatewayChatCompletionRequest,
};
use crate::{
    Client, Error, ErrorKind, RawRequest, RequestOptions, Result,
    data::Patch,
    engine::AuthKind,
    streaming::{ContractStream, TerminalPolicy},
};
use http::Method;

#[derive(Clone, Debug)]
pub struct RuntimeClient {
    client: Client,
}
impl RuntimeClient {
    /// Accept only runtime API-key authentication; SCM OAuth never enters this plane.
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::Runtime)?;
        Ok(Self { client })
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
    /// Fetch the original file bytes without UTF-8 or JSON conversion.
    pub async fn download_file(
        &self,
        file_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<bytes::Bytes> {
        self.client
            .send_bytes(
                RawRequest::read("/files")
                    .segment(file_id)?
                    .segment("content")?
                    .named("airs.gateway.files.content"),
                options,
            )
            .await
    }
    pub async fn create_chat_completion(
        &self,
        body: &GatewayChatCompletionRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayChatCompletion> {
        if matches!(body.stream, Patch::Value(true)) {
            return Err(Error::new(ErrorKind::Validation));
        }
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/chat/completions").named("airs.gateway.chat"),
                body,
                "GatewayInferenceInputCreateChatCompletionRequestSchema",
                "GatewayInferenceCreateChatCompletionResponseSchema",
                options,
            )
            .await
    }
    /// Requires an explicit `stream: true`; no billable inference is ever replayed.
    pub async fn create_chat_completion_stream(
        &self,
        body: &GatewayChatCompletionRequest,
        options: Option<&RequestOptions>,
    ) -> Result<ContractStream<GatewayChatCompletionChunk>> {
        if !matches!(body.stream, Patch::Value(true)) {
            return Err(Error::new(ErrorKind::Validation));
        }
        self.client
            .send_model_stream(
                RawRequest::new(Method::POST, "/chat/completions")
                    .named("airs.gateway.chat_stream"),
                body,
                "GatewayInferenceInputCreateChatCompletionRequestSchema",
                (
                    "GatewayInferenceCreateChatCompletionStreamResponseSchema",
                    TerminalPolicy::DoneMarker,
                ),
                options,
            )
            .await
    }
}
