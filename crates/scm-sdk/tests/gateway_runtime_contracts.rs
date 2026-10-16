//! Runtime wire expectations use independently frozen TypeScript fixtures plus
//! hand-authored multipart, stream, routing, validation and lifecycle cases.
#![cfg(feature = "gateway")]
use bytes::Bytes;
use futures_util::future::BoxFuture;
use http::{HeaderMap, HeaderValue, StatusCode};
use scm_sdk::{
    Client, Endpoint, Error, ErrorKind, Limits, RemoteOutcome, RequestOptions, Result, Secret,
    airs::gateway::{
        GatewayRouting, RuntimeClient, RuntimeReplay, TranscriptionResult, TranslationResult,
        models::*,
    },
    auth::Authentication,
    transport::{Transport, TransportRequest, TransportResponse},
};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::Duration,
};
const KEY: &str = "synthetic-runtime-secret";
const ID: &str = "550e8400-e29b-41d4-a716-446655440000";
const FROZEN: &str = include_str!(
    "../../../contracts/fixtures/typescript/test/openapi/fixtures/gateway-runtime.json"
);
enum Reply {
    Json(Value),
    Raw(Vec<u8>),
    Status(u16),
    Failure,
    Sse(String),
    BrokenSse(String),
}
struct Wire {
    requests: Mutex<Vec<TransportRequest>>,
    replies: Mutex<VecDeque<Reply>>,
}
impl Transport for Wire {
    fn send<'a>(&'a self, request: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            self.requests.lock().unwrap().push(request);
            let reply = self
                .replies
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected runtime dispatch");
            let mut headers = HeaderMap::new();
            headers.insert("x-request-id", HeaderValue::from_static("safe-runtime-id"));
            match reply {
                Reply::Json(value) => Ok(TransportResponse::from_bytes(
                    StatusCode::OK,
                    headers,
                    Bytes::from(serde_json::to_vec(&value).unwrap()),
                )),
                Reply::Raw(bytes) => Ok(TransportResponse::from_bytes(
                    StatusCode::OK,
                    headers,
                    Bytes::from(bytes),
                )),
                Reply::Status(code) => Ok(TransportResponse::from_bytes(
                    StatusCode::from_u16(code).unwrap(),
                    headers,
                    Bytes::from_static(b"upstream-secret"),
                )),
                Reply::Failure => Err(Error::new(ErrorKind::Transport)),
                Reply::Sse(value) => {
                    headers.insert(
                        "content-type",
                        HeaderValue::from_static("text/event-stream"),
                    );
                    Ok(TransportResponse::from_bytes(
                        StatusCode::OK,
                        headers,
                        Bytes::from(value),
                    ))
                }
                Reply::BrokenSse(value) => {
                    headers.insert(
                        "content-type",
                        HeaderValue::from_static("text/event-stream"),
                    );
                    Ok(TransportResponse::new(
                        StatusCode::OK,
                        headers,
                        Box::pin(futures_util::stream::iter([
                            Ok(Bytes::from(value)),
                            Err(Error::new(ErrorKind::Transport)),
                        ])),
                    ))
                }
            }
        })
    }
}
fn setup(replies: Vec<Reply>) -> (RuntimeClient, Arc<Wire>) {
    setup_limits(replies, Limits::default())
}
fn setup_limits(replies: Vec<Reply>, limits: Limits) -> (RuntimeClient, Arc<Wire>) {
    let wire = Arc::new(Wire {
        requests: Mutex::new(Vec::new()),
        replies: Mutex::new(replies.into()),
    });
    let client = Client::builder(Endpoint::parse("https://runtime.example.test/v1").unwrap())
        .transport(wire.clone())
        .limits(limits)
        .authentication(Authentication::runtime_key(Secret::new(KEY).unwrap()).unwrap())
        .build()
        .unwrap();
    (RuntimeClient::new(client).unwrap(), wire)
}
fn model<T: DeserializeOwned>(value: Value) -> T {
    serde_json::from_value(value).unwrap()
}
fn fixture(name: &str) -> Value {
    let source: Value = serde_json::from_str(FROZEN).unwrap();
    source["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["call"]["member"] == name)
        .unwrap()
        .clone()
}
fn response(name: &str) -> Value {
    fixture(name)["responseBody"].clone()
}
fn request_body(name: &str) -> Value {
    fixture(name)["body"].clone()
}
fn check(
    request: &TransportRequest,
    method: &str,
    path: &str,
    query: &[(String, String)],
    body: Option<&Value>,
    routed: bool,
) {
    assert_eq!(request.method().as_str(), method);
    assert_eq!(request.url().host_str(), Some("runtime.example.test"));
    assert_eq!(request.url().path(), path);
    assert_eq!(
        request
            .url()
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect::<Vec<_>>(),
        query
    );
    let mut expected = vec!["service-name", "user-agent", "x-portkey-api-key"];
    if body.is_some() {
        expected.push("content-type");
    }
    if routed {
        expected.extend(["openai-beta", "x-portkey-provider"]);
    }
    expected.sort_unstable();
    let mut actual = request
        .headers()
        .keys()
        .map(|k| k.as_str())
        .collect::<Vec<_>>();
    actual.sort_unstable();
    assert_eq!(actual, expected);
    for name in expected {
        assert_eq!(request.headers().get_all(name).iter().count(), 1);
    }
    assert!(request.headers().values().all(|v| v.is_sensitive()));
    assert_eq!(request.headers()["x-portkey-api-key"], KEY);
    assert_eq!(request.headers()["service-name"], "api");
    assert_eq!(
        request.headers()["user-agent"],
        concat!("PAN-AIRS/", env!("CARGO_PKG_VERSION"), "-rust-sdk")
    );
    if routed {
        assert_eq!(
            request.headers()["x-portkey-provider"],
            "@synthetic-provider"
        );
        assert_eq!(request.headers()["openai-beta"], "assistants=v2");
    }
    if let Some(expected) = body {
        assert_eq!(request.headers()["content-type"], "application/json");
        assert_eq!(
            serde_json::from_slice::<Value>(request.body()).unwrap(),
            *expected
        );
    } else {
        assert!(request.body().is_empty());
    }
    assert!(!format!("{request:?} {:?}", request.headers()).contains(KEY));
}

async fn invoke(
    client: &RuntimeClient,
    case: &Value,
    options: Option<&RequestOptions>,
) -> Result<Value> {
    let body = case["body"].clone();
    let query = case["query"].clone();
    macro_rules! json_result {
        ($call:expr) => {
            serde_json::to_value($call.await?).map_err(|_| Error::new(ErrorKind::Decode))
        };
    }
    match case["call"]["member"].as_str().unwrap() {
        "createImage" => json_result!(client.create_image(&model(body), options)),
        "createRerank" => json_result!(client.create_rerank(&model(body), options)),
        "createOcr" => json_result!(client.create_ocr(&model(body), options)),
        "createFineTuningJob" => json_result!(client.create_fine_tuning_job(&model(body), options)),
        "createModeration" => json_result!(client.create_moderation(&model(body), options)),
        "createVectorStore" => json_result!(client.create_vector_store(&model(body), options)),
        "modifyVectorStore" => json_result!(client.modify_vector_store(ID, &model(body), options)),
        "createVectorStoreFile" => {
            json_result!(client.create_vector_store_file(ID, &model(body), options))
        }
        "createVectorStoreFileBatch" => {
            json_result!(client.create_vector_store_file_batch(ID, &model(body), options))
        }
        "createBatch" => json_result!(client.create_batch(&model(body), options)),
        "createFeedback" => json_result!(client.create_feedback(&model(body), options)),
        "updateFeedback" => json_result!(client.update_feedback(ID, &model(body), options)),
        "retrieveFile" => json_result!(client.retrieve_file(ID, options)),
        "deleteFile" => json_result!(client.delete_file(ID, options)),
        "downloadFile" => {
            let bytes = client.download_file(ID, options).await?;
            Ok(Value::String(String::from_utf8(bytes.to_vec()).unwrap()))
        }
        "retrieveFineTuningJob" => json_result!(client.retrieve_fine_tuning_job(ID, options)),
        "cancelFineTuningJob" => json_result!(client.cancel_fine_tuning_job(ID, options)),
        "retrieveModel" => json_result!(client.retrieve_model(ID, options)),
        "deleteModel" => json_result!(client.delete_model(ID, options)),
        "deleteResponse" => {
            client.delete_response(ID, options).await?;
            Ok(Value::Null)
        }
        "getVectorStore" => json_result!(client.get_vector_store(ID, options)),
        "deleteVectorStore" => json_result!(client.delete_vector_store(ID, options)),
        "getVectorStoreFile" => json_result!(client.get_vector_store_file(ID, ID, options)),
        "deleteVectorStoreFile" => json_result!(client.delete_vector_store_file(ID, ID, options)),
        "getVectorStoreFileBatch" => {
            json_result!(client.get_vector_store_file_batch(ID, ID, options))
        }
        "cancelVectorStoreFileBatch" => {
            json_result!(client.cancel_vector_store_file_batch(ID, ID, options))
        }
        "retrieveBatch" => json_result!(client.retrieve_batch(ID, options)),
        "cancelBatch" => json_result!(client.cancel_batch(ID, options)),
        "getBatchOutput" => {
            let bytes = client.get_batch_output(ID, options).await?;
            Ok(Value::String(String::from_utf8(bytes.to_vec()).unwrap()))
        }
        "listFiles" => json_result!(client.list_files(&model(query), options)),
        "listPaginatedFineTuningJobs" => {
            json_result!(client.list_paginated_fine_tuning_jobs(&model(query), options))
        }
        "listFineTuningEvents" => {
            json_result!(client.list_fine_tuning_events(ID, &model(query), options))
        }
        "listFineTuningJobCheckpoints" => {
            json_result!(client.list_fine_tuning_job_checkpoints(ID, &model(query), options))
        }
        "listModels" => json_result!(client.list_models(&model(query), options)),
        "getResponse" => json_result!(client.get_response(ID, &model(query), options)),
        "listInputItems" => json_result!(client.list_input_items(ID, &model(query), options)),
        "listVectorStores" => json_result!(client.list_vector_stores(&model(query), options)),
        "listVectorStoreFiles" => {
            json_result!(client.list_vector_store_files(ID, &model(query), options))
        }
        "listFilesInVectorStoreBatch" => {
            json_result!(client.list_files_in_vector_store_batch(ID, ID, &model(query), options))
        }
        "listBatches" => json_result!(client.list_batches(&model(query), options)),
        "getLog" => json_result!(client.get_log(ID, &model(query), options)),
        "createChatCompletion" => {
            json_result!(client.create_chat_completion(&model(body), options))
        }
        "createCompletion" => json_result!(client.create_completion(&model(body), options)),
        "createPromptCompletion" => {
            json_result!(client.create_prompt_completion(ID, &model(body), options))
        }
        "createPromptRender" => {
            json_result!(client.create_prompt_render(ID, &model(body), options))
        }
        "createEmbedding" => json_result!(client.create_embedding(&model(body), options)),
        "createResponse" => json_result!(client.create_response(&model(body), options)),
        "createSpeech" => {
            let bytes = client.create_speech(&model(body), options).await?;
            Ok(Value::String(String::from_utf8(bytes.to_vec()).unwrap()))
        }
        "createLogs" => client
            .create_logs(&model(body), options)
            .await
            .map(Value::String),
        _ => panic!("unmapped frozen runtime method"),
    }
}

#[tokio::test]
async fn all_forty_nine_frozen_json_text_and_binary_methods_match_wire_contracts() {
    let source: Value = serde_json::from_str(FROZEN).unwrap();
    let cases = source["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 49);
    for case in cases {
        let method = case["call"]["member"].as_str().unwrap();
        let mut expected = case["responseBody"].clone();
        if let Some(object) = expected.as_object_mut() {
            object.insert(
                "future_response".into(),
                json!({"private":"response-secret"}),
            );
        }
        let reply = if case["responseIsText"] == true {
            Reply::Raw(expected.as_str().unwrap().as_bytes().to_vec())
        } else {
            Reply::Json(expected.clone())
        };
        let (client, wire) = setup(vec![reply]);
        let routing = GatewayRouting::new()
            .header("X-Portkey-Provider", "@synthetic-provider")
            .unwrap()
            .header("OpenAI-Beta", "assistants=v2")
            .unwrap();
        let client = client.with_routing(routing).unwrap();
        let result = invoke(&client, case, None)
            .await
            .unwrap_or_else(|error| panic!("{method}: {error:?}"));
        if method == "deleteResponse" {
            // The frozen method returns void and the native method returns unit;
            // both ignore a successful response body. The adapter represents unit as null.
            assert_eq!(result, Value::Null, "{method}");
        } else {
            assert_eq!(result, expected, "{method}");
        }
        let template = case["path"].as_str().unwrap();
        let mut path = String::from("/v1");
        let mut in_parameter = false;
        for c in template.chars() {
            if c == '{' {
                in_parameter = true;
                path.push_str(ID);
            } else if c == '}' {
                in_parameter = false;
            } else if !in_parameter {
                path.push(c);
            }
        }
        // The frozen query object preserves its independently recorded wire keys;
        // assert explicit source order for each typed query below as well.
        let query = case["queryWire"].as_object().unwrap();
        let actual = wire.requests.lock().unwrap();
        assert_eq!(actual.len(), 1, "{method}");
        let pairs = actual[0]
            .url()
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect::<Vec<_>>();
        let mut expected_pairs = Vec::new();
        let order = match method {
            "listFiles" => vec!["purpose"],
            "listModels" => vec!["ai_service", "provider", "limit", "offset", "sort", "order"],
            "getResponse" => vec!["include"],
            "listInputItems" | "listVectorStores" => vec!["limit", "order", "after", "before"],
            "listVectorStoreFiles" | "listFilesInVectorStoreBatch" => {
                vec!["limit", "order", "after", "before", "filter"]
            }
            "getLog" => vec!["path_format", "created_at", "type"],
            _ => vec!["after", "limit"],
        };
        for key in order {
            if let Some(values) = query.get(key) {
                for value in values.as_array().unwrap() {
                    expected_pairs.push((key.to_owned(), value.as_str().unwrap().to_owned()));
                }
            }
        }
        assert_eq!(pairs, expected_pairs, "{method}");
        let body = case.get("body").filter(|v| !v.is_null());
        check(
            &actual[0],
            case["method"].as_str().unwrap(),
            &path,
            &expected_pairs,
            body,
            true,
        );
    }
}

enum Part<'a> {
    Text(&'a str, &'a str),
    File(&'a str, &'a str, &'a str, &'a [u8]),
}
fn check_form(request: &TransportRequest, path: &str, parts: &[Part<'_>]) {
    assert_eq!(request.method(), "POST");
    assert_eq!(request.url().host_str(), Some("runtime.example.test"));
    assert_eq!(request.url().path(), path);
    assert!(request.url().query().is_none());
    let mut names = request
        .headers()
        .keys()
        .map(|k| k.as_str())
        .collect::<Vec<_>>();
    names.sort_unstable();
    assert_eq!(
        names,
        [
            "content-type",
            "service-name",
            "user-agent",
            "x-portkey-api-key"
        ]
    );
    assert!(request.headers().values().all(|v| v.is_sensitive()));
    assert_eq!(request.headers()["x-portkey-api-key"], KEY);
    let boundary = request.headers()["content-type"]
        .to_str()
        .unwrap()
        .strip_prefix("multipart/form-data; boundary=")
        .unwrap();
    let mut expected = Vec::new();
    for part in parts {
        expected.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
        match part {
            Part::Text(name, value) => {
                expected.extend_from_slice(
                    format!("Content-Disposition: form-data; name=\"{name}\"\r\n\r\n").as_bytes(),
                );
                expected.extend_from_slice(value.as_bytes());
            }
            Part::File(name, filename, mime, bytes) => {
                expected.extend_from_slice(
                    format!(
                        concat!(
                            "Content-Disposition: form-data; name=\"{name}\"; ",
                            "filename=\"{filename}\"\r\nContent-Type: {mime}\r\n\r\n"
                        ),
                        name = name,
                        filename = filename,
                        mime = mime
                    )
                    .as_bytes(),
                );
                expected.extend_from_slice(bytes);
            }
        }
        expected.extend_from_slice(b"\r\n");
    }
    expected.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    assert_eq!(request.body().as_ref(), expected);
    assert!(!format!("{request:?}").contains("file-secret"));
}
fn file() -> BinaryFile {
    BinaryFile::new(Bytes::from_static(b"file-secret\0\xff")).unwrap()
}
fn edit() -> GatewayInferenceInputCreateImageEditRequest {
    GatewayInferenceInputCreateImageEditRequest::builder()
        .image(
            file()
                .with_filename("image.png")
                .unwrap()
                .with_content_type("image/png")
                .unwrap(),
        )
        .prompt("synthetic prompt".into())
        .mask(Optional::Value(file()))
        .model(Patch::Null)
        .n(Patch::Value(1.into()))
        .size(Patch::Value(model(json!("256x256"))))
        .response_format(Patch::Null)
        .user(Optional::Value(String::new()))
        .build()
        .unwrap()
}
fn variation() -> GatewayInferenceInputCreateImageVariationRequest {
    GatewayInferenceInputCreateImageVariationRequest::builder()
        .image(file())
        .model(Patch::Value(model(json!("@provider/image"))))
        .n(Patch::Value(10.into()))
        .response_format(Patch::Value(model(json!("b64_json"))))
        .size(Patch::Value(model(json!("512x512"))))
        .user(Optional::Value("synthetic user".into()))
        .build()
        .unwrap()
}
fn transcription(format: Option<&str>) -> GatewayInferenceInputCreateTranscriptionRequest {
    GatewayInferenceInputCreateTranscriptionRequest::builder()
        .file(file())
        .model(model(json!("@provider/audio")))
        .language(Optional::Value("en".into()))
        .prompt(Optional::Value(String::new()))
        .response_format(format.map_or(Optional::Unset, |value| {
            Optional::Value(model(json!(value)))
        }))
        .temperature(Optional::Value(0.into()))
        .timestamp_granularities__(Optional::Value(vec![
            model(json!("word")),
            model(json!("segment")),
            model(json!("word")),
        ]))
        .build()
        .unwrap()
}
fn translation(format: Option<&str>) -> GatewayInferenceInputCreateTranslationRequest {
    GatewayInferenceInputCreateTranslationRequest::builder()
        .file(file())
        .model(model(json!("@provider/audio")))
        .prompt(Optional::Value(String::new()))
        .response_format(format.map_or(Optional::Unset, |value| Optional::Value(value.into())))
        .temperature(Optional::Value(0.into()))
        .build()
        .unwrap()
}
fn upload() -> GatewayInferenceInputCreateFileRequest {
    GatewayInferenceInputCreateFileRequest::builder()
        .file(file())
        .purpose(model(json!("assistants")))
        .build()
        .unwrap()
}

#[tokio::test]
async fn all_five_native_multipart_methods_match_exact_parts_bytes_defaults_and_order() {
    let (client, wire) = setup(vec![
        Reply::Json(response("createImage")),
        Reply::Json(response("createImage")),
        Reply::Json(response("retrieveFile")),
        Reply::Json(json!({"text":"transcribed","future":"response-secret"})),
        Reply::Json(json!({"text":"translated","future":"response-secret"})),
    ]);
    client.create_image_edit(&edit(), None).await.unwrap();
    client
        .create_image_variation(&variation(), None)
        .await
        .unwrap();
    client.create_file(&upload(), None).await.unwrap();
    let result = client
        .create_transcription(&transcription(None), None)
        .await
        .unwrap();
    assert!(matches!(result, TranscriptionResult::Json(_)));
    let result = client
        .create_translation(&translation(Some("future-format")), None)
        .await
        .unwrap();
    assert!(matches!(result, TranslationResult::Json(_)));
    assert!(!format!("{result:?}").contains("response-secret"));
    let requests = wire.requests.lock().unwrap();
    let bytes = b"file-secret\0\xff";
    check_form(
        &requests[0],
        "/v1/images/edits",
        &[
            Part::File("image", "image.png", "image/png", bytes),
            Part::Text("prompt", "synthetic prompt"),
            Part::File("mask", "blob", "application/octet-stream", bytes),
            Part::Text("n", "1"),
            Part::Text("size", "256x256"),
            Part::Text("user", ""),
        ],
    );
    check_form(
        &requests[1],
        "/v1/images/variations",
        &[
            Part::File("image", "blob", "application/octet-stream", bytes),
            Part::Text("model", "@provider/image"),
            Part::Text("n", "10"),
            Part::Text("response_format", "b64_json"),
            Part::Text("size", "512x512"),
            Part::Text("user", "synthetic user"),
        ],
    );
    check_form(
        &requests[2],
        "/v1/files",
        &[
            Part::File("file", "blob", "application/octet-stream", bytes),
            Part::Text("purpose", "assistants"),
        ],
    );
    check_form(
        &requests[3],
        "/v1/audio/transcriptions",
        &[
            Part::File("file", "blob", "application/octet-stream", bytes),
            Part::Text("model", "@provider/audio"),
            Part::Text("language", "en"),
            Part::Text("prompt", ""),
            Part::Text("temperature", "0"),
            Part::Text("timestamp_granularities[]", "word"),
            Part::Text("timestamp_granularities[]", "segment"),
            Part::Text("timestamp_granularities[]", "word"),
        ],
    );
    check_form(
        &requests[4],
        "/v1/audio/translations",
        &[
            Part::File("file", "blob", "application/octet-stream", bytes),
            Part::Text("model", "@provider/audio"),
            Part::Text("prompt", ""),
            Part::Text("response_format", "future-format"),
            Part::Text("temperature", "0"),
        ],
    );
}

#[tokio::test]
async fn audio_response_format_controls_decoding_without_content_type_guessing() {
    for format in [
        None,
        Some("json"),
        Some("verbose_json"),
        Some("text"),
        Some("srt"),
        Some("vtt"),
    ] {
        let expected = json!({"language":"en","duration":1,"text":"synthetic text","future":"response-secret"});
        let textual = matches!(format, Some("text" | "srt" | "vtt"));
        let reply = || {
            if textual {
                Reply::Raw(b"\xef\xbb\xbfsynthetic\xfftext".to_vec())
            } else {
                Reply::Json(expected.clone())
            }
        };
        let (client, wire) = setup(vec![reply(), reply()]);
        let transcribed = client
            .create_transcription(&transcription(format), None)
            .await
            .unwrap();
        let translated = client
            .create_translation(&translation(format), None)
            .await
            .unwrap();
        match (format, transcribed, translated) {
            (
                Some("verbose_json"),
                TranscriptionResult::VerboseJson(a),
                TranslationResult::VerboseJson(b),
            ) => {
                assert_eq!(serde_json::to_value(a).unwrap(), expected);
                assert_eq!(serde_json::to_value(b).unwrap(), expected);
            }
            (
                Some("text" | "srt" | "vtt"),
                TranscriptionResult::Text(a),
                TranslationResult::Text(b),
            ) => {
                assert_eq!(a, "synthetic\u{fffd}text");
                assert_eq!(b, a);
            }
            (_, TranscriptionResult::Json(a), TranslationResult::Json(b)) => {
                assert_eq!(serde_json::to_value(a).unwrap(), expected);
                assert_eq!(serde_json::to_value(b).unwrap(), expected);
            }
            _ => panic!("incorrect audio response branch"),
        }
        assert_eq!(wire.requests.lock().unwrap().len(), 2);
    }
}

#[tokio::test]
async fn every_native_file_shape_rejects_invalid_fields_collisions_and_bounds_before_dispatch() {
    let (client, wire) = setup(vec![]);
    let mut body = edit();
    body.n = Patch::Value(0.into());
    assert!(client.create_image_edit(&body, None).await.is_err());
    let mut body = variation();
    body.n = Patch::Value(11.into());
    assert!(client.create_image_variation(&body, None).await.is_err());
    let mut body = transcription(Some("future-format"));
    assert!(client.create_transcription(&body, None).await.is_err());
    body = transcription(None);
    body.timestamp_granularities__ = Optional::Value(vec![model(json!("future-granularity"))]);
    assert!(client.create_transcription(&body, None).await.is_err());
    for key in [
        "image",
        "prompt",
        "mask",
        "model",
        "n",
        "size",
        "response_format",
        "user",
    ] {
        let mut body = edit();
        body.mask = Optional::Unset;
        body.extensions
            .insert(key.into(), JsonValue::new(Value::Null));
        assert!(client.create_image_edit(&body, None).await.is_err());
    }
    for key in ["image", "model", "n", "size", "response_format", "user"] {
        let mut body = variation();
        body.extensions
            .insert(key.into(), JsonValue::new(Value::Null));
        assert!(client.create_image_variation(&body, None).await.is_err());
    }
    for key in [
        "file",
        "model",
        "language",
        "prompt",
        "response_format",
        "temperature",
        "timestamp_granularities[]",
    ] {
        let mut body = transcription(None);
        body.extensions
            .insert(key.into(), JsonValue::new(Value::Null));
        assert!(client.create_transcription(&body, None).await.is_err());
    }
    for key in ["file", "model", "prompt", "response_format", "temperature"] {
        let mut body = translation(None);
        body.extensions
            .insert(key.into(), JsonValue::new(Value::Null));
        assert!(client.create_translation(&body, None).await.is_err());
    }
    for key in ["file", "purpose", "future"] {
        let mut body = upload();
        body.extensions
            .insert(key.into(), JsonValue::new(Value::Null));
        assert!(client.create_file(&body, None).await.is_err());
    }
    let limits = scm_sdk::validation::ValidationLimits::default();
    for (schema, metadata) in [
        (
            "GatewayInferenceInputCreateImageEditRequestSchema",
            json!({"prompt":"synthetic"}),
        ),
        (
            "GatewayInferenceInputCreateImageVariationRequestSchema",
            json!({}),
        ),
        (
            "GatewayInferenceInputCreateTranscriptionRequestSchema",
            json!({"model":"synthetic"}),
        ),
        (
            "GatewayInferenceInputCreateTranslationRequestSchema",
            json!({"model":"synthetic"}),
        ),
        (
            "GatewayInferenceInputCreateFileRequestSchema",
            json!({"purpose":"assistants"}),
        ),
    ] {
        assert!(
            scm_sdk::validation::parse_multipart_with_limits(
                schema,
                metadata.clone(),
                &[],
                &limits,
                1024
            )
            .is_err()
        );
        let mut spoof = metadata;
        spoof[if schema.contains("Image") {
            "image"
        } else {
            "file"
        }] = json!({"bytes":"fake"});
        assert!(
            scm_sdk::validation::parse(schema, spoof.clone(), scm_sdk::validation::Mode::Request)
                .is_err()
        );
        assert!(
            scm_sdk::validation::parse_multipart_with_limits(schema, spoof, &[], &limits, 1024)
                .is_err()
        );
    }
    assert!(wire.requests.lock().unwrap().is_empty());
    let mut limits = Limits::default();
    limits.max_request_bytes = 1024;
    let (client, wire) = setup_limits(vec![], limits);
    for length in [1025, 950] {
        let huge = BinaryFile::new(Bytes::from(vec![b'x'; length])).unwrap();
        let mut a = edit();
        a.image = huge.clone();
        let mut b = variation();
        b.image = huge.clone();
        let mut c = transcription(None);
        c.file = huge.clone();
        let mut d = translation(None);
        d.file = huge.clone();
        let mut e = upload();
        e.file = huge;
        for error in [
            client.create_image_edit(&a, None).await.unwrap_err(),
            client.create_image_variation(&b, None).await.unwrap_err(),
            client.create_transcription(&c, None).await.unwrap_err(),
            client.create_translation(&d, None).await.unwrap_err(),
            client.create_file(&e, None).await.unwrap_err(),
        ] {
            assert_eq!(error.kind(), ErrorKind::Limit);
        }
    }
    assert!(wire.requests.lock().unwrap().is_empty());
}

#[tokio::test]
async fn runtime_default_never_replays_any_frozen_method_even_with_safe_read_retry_override() {
    let source: Value = serde_json::from_str(FROZEN).unwrap();
    let options = RequestOptions::with_timeout(Duration::from_secs(30))
        .unwrap()
        .with_retries(5)
        .unwrap();
    for case in source["cases"].as_array().unwrap() {
        let (client, wire) = setup(vec![Reply::Failure]);
        let error = invoke(&client, case, Some(&options)).await.unwrap_err();
        assert_eq!(
            (error.kind(), error.remote_outcome()),
            (ErrorKind::Transport, RemoteOutcome::Unknown)
        );
        assert_eq!(wire.requests.lock().unwrap().len(), 1);
    }
    let (client, wire) = setup((0..5).map(|_| Reply::Failure).collect());
    for error in [
        client
            .create_image_edit(&edit(), Some(&options))
            .await
            .unwrap_err(),
        client
            .create_image_variation(&variation(), Some(&options))
            .await
            .unwrap_err(),
        client
            .create_transcription(&transcription(None), Some(&options))
            .await
            .unwrap_err(),
        client
            .create_translation(&translation(None), Some(&options))
            .await
            .unwrap_err(),
        client
            .create_file(&upload(), Some(&options))
            .await
            .unwrap_err(),
    ] {
        assert_eq!(
            (error.kind(), error.remote_outcome()),
            (ErrorKind::Transport, RemoteOutcome::Unknown)
        );
    }
    assert_eq!(wire.requests.lock().unwrap().len(), 5);
}

fn chunk() -> Value {
    json!({"id":"chat-synthetic","object":"chat.completion.chunk","created":1,"model":"@synthetic/model","choices":[{"index":0,"delta":{"role":"assistant","content":"synthetic"},"finish_reason":null}],"future":"response-secret"})
}
fn streamed(name: &str) -> Value {
    let mut body = request_body(name);
    body["stream"] = json!(true);
    body
}
#[tokio::test]
async fn four_inference_streams_preserve_typed_events_terminal_policies_and_metadata() {
    let chat = chunk();
    let completion = response("createCompletion");
    let terminal = json!({"type":"response.completed","response":response("createResponse"),"future":"response-secret"});
    let (client, wire) = setup(vec![
        Reply::Sse(format!("data: {chat}\n\nevent: error\ndata: [DONE]\n\n")),
        Reply::Sse(format!("data: {completion}\n\ndata: [DONE]\n\n")),
        Reply::Sse(format!("data: {chat}\n\ndata: [DONE]\n\n")),
        Reply::Sse(format!("event: ignored\ndata: {terminal}\n\n")),
    ]);
    let mut stream = client
        .create_chat_completion_stream(&model(streamed("createChatCompletion")), None)
        .await
        .unwrap();
    assert_eq!(
        serde_json::to_value(stream.next().await.unwrap().unwrap()).unwrap(),
        chat
    );
    assert!(stream.next().await.unwrap().is_none());
    let mut stream = client
        .create_completion_stream(&model(streamed("createCompletion")), None)
        .await
        .unwrap();
    assert_eq!(
        serde_json::to_value(stream.next().await.unwrap().unwrap()).unwrap(),
        completion
    );
    assert!(stream.next().await.unwrap().is_none());
    let mut stream = client
        .create_prompt_completion_stream(ID, &model(streamed("createPromptCompletion")), None)
        .await
        .unwrap();
    assert_eq!(
        serde_json::to_value(stream.next().await.unwrap().unwrap()).unwrap(),
        chat
    );
    assert!(stream.next().await.unwrap().is_none());
    let mut stream = client
        .create_response_stream(&model(streamed("createResponse")), None)
        .await
        .unwrap();
    let event = stream.next().await.unwrap().unwrap();
    assert!(!format!("{event:?}").contains("response-secret"));
    assert_eq!(serde_json::to_value(event).unwrap(), terminal);
    assert!(stream.next().await.unwrap().is_none());
    let requests = wire.requests.lock().unwrap();
    for (request, path, name) in [
        (&requests[0], "/v1/chat/completions", "createChatCompletion"),
        (&requests[1], "/v1/completions", "createCompletion"),
        (
            &requests[2],
            &format!("/v1/prompts/{ID}/completions"),
            "createPromptCompletion",
        ),
        (&requests[3], "/v1/responses", "createResponse"),
    ] {
        check(request, "POST", path, &[], Some(&streamed(name)), false);
    }
}

#[tokio::test]
async fn streaming_failures_keep_safe_metadata_and_never_replay_after_success_headers() {
    for reply in [
        Reply::BrokenSse(String::new()),
        Reply::Sse("data: {invalid}\n\n".into()),
        Reply::Sse("event: error\ndata: {\"private\":\"upstream-secret\"}\n\n".into()),
        Reply::Sse("data: {\"error\":{\"message\":\"upstream-secret\"}}\n\n".into()),
        Reply::Sse("data: {}\n\n".into()),
    ] {
        let (client, wire) = setup(vec![reply]);
        let client = client
            .with_replay(RuntimeReplay::bounded(5).unwrap())
            .unwrap();
        let mut stream = client
            .create_chat_completion_stream(&model(streamed("createChatCompletion")), None)
            .await
            .unwrap();
        let error = stream.next().await.unwrap_err();
        assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
        assert_eq!(error.status(), Some(200));
        assert_eq!(error.request_id(), Some("safe-runtime-id"));
        assert!(!format!("{error:?} {error}").contains("upstream-secret"));
        assert_eq!(wire.requests.lock().unwrap().len(), 1);
    }
    for kind in ["response.failed", "response.incomplete"] {
        let terminal = json!({"type":kind,"response":response("createResponse")});
        let (client, _) = setup(vec![Reply::Sse(format!("data: {terminal}\n\n"))]);
        let mut stream = client
            .create_response_stream(&model(streamed("createResponse")), None)
            .await
            .unwrap();
        assert_eq!(
            serde_json::to_value(stream.next().await.unwrap().unwrap()).unwrap(),
            terminal
        );
        assert!(stream.next().await.unwrap().is_none());
    }
    let (client, wire) = setup(vec![Reply::Sse("data: [DONE]\n\n".into())]);
    let mut stream = client
        .create_response_stream(&model(streamed("createResponse")), None)
        .await
        .unwrap();
    assert!(stream.next().await.is_err());
    assert_eq!(wire.requests.lock().unwrap().len(), 1);
}

#[tokio::test(start_paused = true)]
async fn explicit_runtime_replay_is_bounded_and_only_network_or_source_five_xx_statuses_qualify() {
    for status in [500, 502, 503, 504] {
        let (client, wire) = setup(vec![
            Reply::Status(status),
            Reply::Json(response("createChatCompletion")),
        ]);
        let client = client
            .with_replay(RuntimeReplay::bounded(1).unwrap())
            .unwrap();
        client
            .create_chat_completion(&model(request_body("createChatCompletion")), None)
            .await
            .unwrap();
        let requests = wire.requests.lock().unwrap();
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0].body(), requests[1].body());
        assert_eq!(requests[0].headers(), requests[1].headers());
    }
    let (client, wire) = setup(vec![
        Reply::Failure,
        Reply::Json(response("createChatCompletion")),
    ]);
    let client = client
        .with_replay(RuntimeReplay::bounded(1).unwrap())
        .unwrap();
    client
        .create_chat_completion(&model(request_body("createChatCompletion")), None)
        .await
        .unwrap();
    assert_eq!(wire.requests.lock().unwrap().len(), 2);
    let (client, wire) = setup(vec![Reply::Status(429)]);
    let client = client
        .with_replay(RuntimeReplay::bounded(5).unwrap())
        .unwrap();
    assert_eq!(
        client
            .create_chat_completion(&model(request_body("createChatCompletion")), None)
            .await
            .unwrap_err()
            .status(),
        Some(429)
    );
    assert_eq!(wire.requests.lock().unwrap().len(), 1);
    let (client, wire) = setup(vec![Reply::Status(503)]);
    assert!(
        client
            .create_chat_completion(&model(request_body("createChatCompletion")), None)
            .await
            .is_err()
    );
    assert_eq!(wire.requests.lock().unwrap().len(), 1);
    let (client, wire) = setup(vec![
        Reply::Status(503),
        Reply::Sse(format!("data: {}\n\ndata: [DONE]\n\n", chunk())),
    ]);
    let client = client
        .with_replay(RuntimeReplay::bounded(1).unwrap())
        .unwrap();
    let mut stream = client
        .create_chat_completion_stream(&model(streamed("createChatCompletion")), None)
        .await
        .unwrap();
    assert!(stream.next().await.unwrap().is_some());
    assert!(stream.next().await.unwrap().is_none());
    assert_eq!(wire.requests.lock().unwrap().len(), 2);
}

#[tokio::test]
async fn invalid_stream_modes_queries_routing_and_identifiers_fail_without_dispatch() {
    let (client, wire) = setup(vec![]);
    assert!(
        client
            .create_chat_completion(&model(streamed("createChatCompletion")), None)
            .await
            .is_err()
    );
    assert!(
        client
            .create_chat_completion_stream(&model(request_body("createChatCompletion")), None)
            .await
            .is_err()
    );
    assert!(
        client
            .create_completion(&model(streamed("createCompletion")), None)
            .await
            .is_err()
    );
    assert!(
        client
            .create_completion_stream(&model(request_body("createCompletion")), None)
            .await
            .is_err()
    );
    assert!(
        client
            .create_prompt_completion(ID, &model(streamed("createPromptCompletion")), None)
            .await
            .is_err()
    );
    assert!(
        client
            .create_prompt_completion_stream(
                ID,
                &model(request_body("createPromptCompletion")),
                None
            )
            .await
            .is_err()
    );
    assert!(
        client
            .create_response(&model(streamed("createResponse")), None)
            .await
            .is_err()
    );
    assert!(
        client
            .create_response_stream(&model(request_body("createResponse")), None)
            .await
            .is_err()
    );
    let invalid =
        GatewayInferenceInputGetLogQuery::Unknown(JsonValue::new(json!({"path_format":"v2"})));
    assert!(client.get_log("log", &invalid, None).await.is_err());
    for id in [
        "",
        " ",
        ".",
        "..",
        "../escape",
        "%252e%252e",
        "bad\nidentifier",
    ] {
        assert!(client.retrieve_file(id, None).await.is_err());
        assert!(client.download_file(id, None).await.is_err());
        assert!(client.get_batch_output(id, None).await.is_err());
        assert!(
            client
                .create_prompt_render(id, &model(request_body("createPromptRender")), None)
                .await
                .is_err()
        );
    }
    assert!(
        client
            .update_feedback("not-uuid", &model(request_body("updateFeedback")), None)
            .await
            .is_err()
    );
    for name in [
        "authorization",
        "host",
        "x-tsg-id",
        "X-Portkey-Api-Key",
        "x-portkey-",
        "x-portkey-_",
        "cookie",
    ] {
        assert!(GatewayRouting::new().header(name, "private").is_err());
    }
    assert!(
        GatewayRouting::new()
            .header("openai-beta", "private\r\nInjected: yes")
            .is_err()
    );
    assert!(
        GatewayRouting::new()
            .header("x-portkey-provider", "private")
            .unwrap()
            .header("X-Portkey-Provider", "other")
            .is_err()
    );
    assert!(RuntimeReplay::bounded(6).is_err());
    assert!(wire.requests.lock().unwrap().is_empty());
}

#[tokio::test]
async fn identifiers_routing_clones_binary_content_and_scalar_query_states_remain_exact() {
    let (client, wire) = setup(vec![
        Reply::Raw(vec![0, 255, 10]),
        Reply::Raw(vec![0, 255, 10]),
        Reply::Json(response("listModels")),
        Reply::Json(response("getResponse")),
    ]);
    let routing = GatewayRouting::new()
        .header("x-portkey-provider", "@synthetic-provider")
        .unwrap()
        .header("openai-beta", "assistants=v2")
        .unwrap();
    let routed = client.with_routing(routing).unwrap();
    let id = "file/a?b#c%literal";
    assert_eq!(
        routed.download_file(id, None).await.unwrap(),
        Bytes::from_static(&[0, 255, 10])
    );
    let long = "x".repeat(1024);
    assert_eq!(
        client.download_file(&long, None).await.unwrap(),
        Bytes::from_static(&[0, 255, 10])
    );
    let query: GatewayInferenceInputListModelsQuery =
        model(json!({"ai_service":"","provider":"","limit":0,"offset":-1,"order":"asc"}));
    client.list_models(&query, None).await.unwrap();
    let query: GatewayInferenceInputGetResponseQuery =
        model(json!({"include":["message.input_image.image_url","message.input_image.image_url"]}));
    client
        .get_response("response/id", &query, None)
        .await
        .unwrap();
    let requests = wire.requests.lock().unwrap();
    check(
        &requests[0],
        "GET",
        "/v1/files/file%2Fa%3Fb%23c%25literal/content",
        &[],
        None,
        true,
    );
    check(
        &requests[1],
        "GET",
        &format!("/v1/files/{long}/content"),
        &[],
        None,
        false,
    );
    assert_eq!(
        requests[2]
            .url()
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect::<Vec<_>>(),
        vec![
            ("ai_service".into(), "".into()),
            ("provider".into(), "".into()),
            ("limit".into(), "0".into()),
            ("offset".into(), "-1".into()),
            ("order".into(), "asc".into())
        ]
    );
    assert_eq!(
        requests[3].url().query_pairs().collect::<Vec<_>>(),
        vec![
            ("include".into(), "message.input_image.image_url".into()),
            ("include".into(), "message.input_image.image_url".into())
        ]
    );
}

#[tokio::test(start_paused = true)]
async fn cancellation_deadlines_and_request_limits_apply_before_dispatch_and_to_buffered_events() {
    let (client, wire) = setup(vec![]);
    let options = RequestOptions::with_timeout(Duration::from_secs(5)).unwrap();
    options.cancel();
    assert_eq!(
        client
            .create_file(&upload(), Some(&options))
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(
        client
            .create_chat_completion(&model(request_body("createChatCompletion")), Some(&options))
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(
        client
            .list_files(&model(json!({})), Some(&options))
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Cancelled
    );
    let options = RequestOptions::with_timeout(Duration::from_millis(1)).unwrap();
    tokio::time::advance(Duration::from_millis(2)).await;
    assert_eq!(
        client
            .create_image_edit(&edit(), Some(&options))
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Deadline
    );
    assert!(wire.requests.lock().unwrap().is_empty());
    let (client, wire) = setup(vec![Reply::Sse(format!(
        "data: {}\n\ndata: {}\n\ndata: [DONE]\n\n",
        chunk(),
        chunk()
    ))]);
    let options = RequestOptions::with_timeout(Duration::from_secs(5)).unwrap();
    let mut stream = client
        .create_chat_completion_stream(&model(streamed("createChatCompletion")), Some(&options))
        .await
        .unwrap();
    assert!(stream.next().await.unwrap().is_some());
    options.cancel();
    assert_eq!(
        stream.next().await.unwrap_err().kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(wire.requests.lock().unwrap().len(), 1);
    let mut limits = Limits::default();
    limits.max_request_bytes = 1024;
    let (client, wire) = setup_limits(vec![], limits);
    let mut body = request_body("createChatCompletion");
    body["messages"] = json!([{"role":"user","content":"x".repeat(2048)}]);
    assert_eq!(
        client
            .create_chat_completion(&model(body), None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Limit
    );
    assert!(wire.requests.lock().unwrap().is_empty());
}
