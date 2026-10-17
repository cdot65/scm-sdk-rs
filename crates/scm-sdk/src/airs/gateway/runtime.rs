//! Runtime-key inference and resources. Routing clones share the caller's transport.
use super::{common, models::*};
use crate::{
    Client, Error, ErrorKind, RawRequest, RequestOptions, Result,
    engine::AuthKind,
    routing::{GatewayRouting, RuntimeReplay},
    streaming::{ContractStream, TerminalPolicy},
};
use http::Method;
use serde::Serialize;

#[path = "runtime_audio.rs"]
mod audio;
#[path = "runtime_resources.rs"]
mod resources;
pub use audio::{TranscriptionResult, TranslationResult};

/// Gateway inference and runtime resources authenticated with a runtime API key.
///
/// Enable `gateway`; caller-driven realtime additionally requires `realtime`.
#[derive(Clone, Debug)]
pub struct RuntimeClient {
    client: Client,
    routing: GatewayRouting,
    replay: RuntimeReplay,
}
impl RuntimeClient {
    /// Accept only runtime API-key authentication; SCM OAuth never enters this plane.
    ///
    /// The caller selects the runtime deployment endpoint and owns its async runtime.
    /// Requests use the default no-replay policy unless explicitly configured otherwise.
    ///
    /// ```no_run
    /// use scm_sdk::{Client, Result, airs::gateway::{RuntimeClient,
    ///     models::GatewayChatCompletionRequest}};
    /// async fn chat(
    ///     configured: Client, request: &GatewayChatCompletionRequest,
    /// ) -> Result<()> {
    ///     let runtime = RuntimeClient::new(configured)?;
    ///     let _completion = runtime.create_chat_completion(request, None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::Runtime)?;
        Ok(Self {
            client,
            routing: GatewayRouting::default(),
            replay: RuntimeReplay::default(),
        })
    }
    /// Borrow the runtime-key client and its shared cancellation session.
    pub fn client(&self) -> &Client {
        &self.client
    }
    /// Experimental realtime. The caller explicitly selects a connector; HTTP retries never apply.
    #[cfg(feature = "realtime")]
    pub async fn connect_realtime(
        &self,
        query: &GatewayRealtimeConnectRequest,
        realtime: &crate::realtime::GatewayRealtimeOptions,
        options: Option<&RequestOptions>,
    ) -> Result<crate::realtime::GatewayRealtimeConnection> {
        realtime.validate()?;
        let options = self.client.operation_options(options)?;
        let query = self
            .client
            .validated_json(query, "GatewayRealtimeConnectRequestSchema", &options)
            .await?;
        let query: serde_json::Value = crate::json_decode::from_slice(&query)
            .map_err(|_| Error::new(ErrorKind::Validation))?;
        let query = query
            .as_object()
            .ok_or_else(|| Error::new(ErrorKind::Validation))?
            .iter()
            .map(|(key, value)| {
                value
                    .as_str()
                    .map(|value| (key.clone(), value.to_owned()))
                    .ok_or_else(|| Error::new(ErrorKind::Validation))
            })
            .collect::<Result<Vec<_>>>()?;
        let mut request = RawRequest::new(Method::GET, "/realtime")
            .query(query)
            .named(crate::realtime::OPERATION);
        self.routing.apply(&mut request)?;
        self.client.open_realtime(request, realtime, &options).await
    }
    /// Configure a clone for one call or a sequence; authentication remains unchanged.
    pub fn with_routing(&self, routing: GatewayRouting) -> Result<Self> {
        Ok(Self {
            client: self.client.clone(),
            routing,
            replay: self.replay,
        })
    }
    /// Explicitly acknowledge runtime replay; RequestOptions alone never grants it.
    pub fn with_replay(&self, replay: RuntimeReplay) -> Result<Self> {
        Ok(Self {
            client: self.client.clone(),
            routing: self.routing.clone(),
            replay,
        })
    }
    fn request(
        &self,
        method: Method,
        path: &str,
        segments: &[&str],
        name: &'static str,
    ) -> Result<RawRequest> {
        let mut request = RawRequest::new(method, path).named(name);
        for segment in segments {
            common::path_segment(segment)?;
            request = request.segment(*segment)?;
        }
        self.routing.apply(&mut request)?;
        self.replay.apply(&mut request)?;
        Ok(request)
    }
    async fn input<T: Serialize>(
        &self,
        request: RawRequest,
        body: &T,
        schema: &str,
        stream: Option<bool>,
        options: Option<&RequestOptions>,
    ) -> Result<(RawRequest, RequestOptions)> {
        let options = self.client.operation_options(options)?;
        let bytes = self.client.validated_json(body, schema, &options).await?;
        if let Some(expected) = stream {
            let value: serde_json::Value = crate::json_decode::from_slice(&bytes)
                .map_err(|_| Error::new(ErrorKind::Validation))?;
            if (value.get("stream") == Some(&serde_json::Value::Bool(true))) != expected {
                return Err(Error::new(ErrorKind::Validation));
            }
        }
        Ok((request.body(bytes, "application/json")?, options))
    }
    pub async fn create_completion(
        &self,
        body: &GatewayInferenceInputCreateCompletionRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceCreateCompletionResponse> {
        let (request, options) = self
            .input(
                self.request(Method::POST, "/completions", &[], "gateway.completion")?,
                body,
                "GatewayInferenceInputCreateCompletionRequestSchema",
                Some(false),
                options,
            )
            .await?;
        self.client
            .send_contract(
                request,
                "GatewayInferenceCreateCompletionResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn create_completion_stream(
        &self,
        body: &GatewayInferenceInputCreateCompletionRequest,
        options: Option<&RequestOptions>,
    ) -> Result<ContractStream<GatewayInferenceCreateCompletionResponse>> {
        let (request, options) = self
            .input(
                self.request(
                    Method::POST,
                    "/completions",
                    &[],
                    "gateway.completion_stream",
                )?,
                body,
                "GatewayInferenceInputCreateCompletionRequestSchema",
                Some(true),
                options,
            )
            .await?;
        self.client
            .send_contract_stream(
                request,
                "GatewayInferenceCreateCompletionResponseSchema",
                TerminalPolicy::DoneMarker,
                Some(&options),
            )
            .await
    }
    pub async fn create_prompt_completion(
        &self,
        id: &str,
        body: &GatewayInferenceInputCreatePromptCompletionRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceCreatePromptCompletionResponse> {
        let (request, options) = self
            .input(
                self.request(
                    Method::POST,
                    "/prompts",
                    &[id, "completions"],
                    "gateway.prompt_completion",
                )?,
                body,
                "GatewayInferenceInputCreatePromptCompletionRequestSchema",
                Some(false),
                options,
            )
            .await?;
        self.client
            .send_contract(
                request,
                "GatewayInferenceCreatePromptCompletionResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn create_prompt_completion_stream(
        &self,
        id: &str,
        body: &GatewayInferenceInputCreatePromptCompletionRequest,
        options: Option<&RequestOptions>,
    ) -> Result<ContractStream<GatewayInferenceCreatePromptCompletionStreamResponse>> {
        let (request, options) = self
            .input(
                self.request(
                    Method::POST,
                    "/prompts",
                    &[id, "completions"],
                    "gateway.prompt_completion_stream",
                )?,
                body,
                "GatewayInferenceInputCreatePromptCompletionRequestSchema",
                Some(true),
                options,
            )
            .await?;
        self.client
            .send_contract_stream(
                request,
                "GatewayInferenceCreatePromptCompletionStreamResponseSchema",
                TerminalPolicy::DoneMarker,
                Some(&options),
            )
            .await
    }
    pub async fn create_prompt_render(
        &self,
        id: &str,
        body: &GatewayInferenceInputCreatePromptRenderRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceCreatePromptRenderResponse> {
        self.client
            .send_model(
                self.request(
                    Method::POST,
                    "/prompts",
                    &[id, "render"],
                    "gateway.prompt_render",
                )?,
                body,
                "GatewayInferenceInputCreatePromptRenderRequestSchema",
                "GatewayInferenceCreatePromptRenderResponseSchema",
                options,
            )
            .await
    }
    pub async fn create_chat_completion(
        &self,
        body: &GatewayChatCompletionRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayChatCompletion> {
        let (request, options) = self
            .input(
                self.request(Method::POST, "/chat/completions", &[], "airs.gateway.chat")?,
                body,
                "GatewayInferenceInputCreateChatCompletionRequestSchema",
                Some(false),
                options,
            )
            .await?;
        self.client
            .send_contract(
                request,
                "GatewayInferenceCreateChatCompletionResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn create_chat_completion_stream(
        &self,
        body: &GatewayChatCompletionRequest,
        options: Option<&RequestOptions>,
    ) -> Result<ContractStream<GatewayChatCompletionChunk>> {
        let (request, options) = self
            .input(
                self.request(
                    Method::POST,
                    "/chat/completions",
                    &[],
                    "airs.gateway.chat_stream",
                )?,
                body,
                "GatewayInferenceInputCreateChatCompletionRequestSchema",
                Some(true),
                options,
            )
            .await?;
        self.client
            .send_contract_stream(
                request,
                "GatewayInferenceCreateChatCompletionStreamResponseSchema",
                TerminalPolicy::DoneMarker,
                Some(&options),
            )
            .await
    }
    pub async fn create_embedding(
        &self,
        body: &GatewayEmbeddingRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayEmbeddingResponse> {
        self.client
            .send_model(
                self.request(Method::POST, "/embeddings", &[], "gateway.embedding")?,
                body,
                "GatewayInferenceInputCreateEmbeddingRequestSchema",
                "GatewayInferenceCreateEmbeddingResponseSchema",
                options,
            )
            .await
    }
    pub async fn create_response(
        &self,
        body: &GatewayInferenceInputCreateResponse,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayInferenceResponse> {
        let (request, options) = self
            .input(
                self.request(Method::POST, "/responses", &[], "gateway.response")?,
                body,
                "GatewayInferenceInputCreateResponseSchema",
                Some(false),
                options,
            )
            .await?;
        self.client
            .send_contract(request, "GatewayInferenceResponseSchema", Some(&options))
            .await
    }
    pub async fn create_response_stream(
        &self,
        body: &GatewayInferenceInputCreateResponse,
        options: Option<&RequestOptions>,
    ) -> Result<ContractStream<GatewayInferenceResponseStreamEvent>> {
        let (request, options) = self
            .input(
                self.request(Method::POST, "/responses", &[], "gateway.response_stream")?,
                body,
                "GatewayInferenceInputCreateResponseSchema",
                Some(true),
                options,
            )
            .await?;
        self.client
            .send_contract_stream(
                request,
                "GatewayInferenceResponseStreamEventSchema",
                TerminalPolicy::Responses,
                Some(&options),
            )
            .await
    }
}
