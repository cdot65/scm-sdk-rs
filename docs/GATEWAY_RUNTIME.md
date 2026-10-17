# Gateway runtime

Enable `gateway` for `airs::gateway::RuntimeClient`. It implements the 48 frozen runtime resource methods and six HTTP inference methods, with separate typed streaming variants for four inference methods. Typed request/response models live under `airs::gateway::models`. Phase 7 also implemented the separately gated `connect_realtime` method; see [REALTIME.md](REALTIME.md). Offline contracts do not establish tenant, provider or universal live service availability. Current per-method status is in [LIVE_ACCEPTANCE.md](LIVE_ACCEPTANCE.md) and the [operation matrix](../contracts/native-operations.json).

## Explicit runtime configuration

Construct a core `Client` with an explicit admitted runtime `Endpoint`, `Authentication::runtime_key`, the caller's transport and limits, then pass it to `RuntimeClient::new`. The constructor rejects SCM OAuth, scan credentials and unauthenticated clients. Runtime requests send the key only as sensitive `x-portkey-api-key`; they do not inherit bearer authorization, scan HMAC or `x-tsg-id`. SDK defaults add `service-name: api` and the versioned `PAN-AIRS/<crate-version>-rust-sdk` User-Agent. The Gateway facade accepts this separately configured client through `with_runtime`.

`with_routing(GatewayRouting)` returns an immutable configured clone sharing the original client, session, transport and concurrency bound. It accepts `OpenAI-Beta` and nonempty `x-portkey-*` header names matching the frozen grammar, except `x-portkey-api-key`. Authentication cannot be overridden. Duplicate names are rejected case insensitively; values and formatting are redacted. The header collection has entry, name, value and aggregate byte bounds. A clone's routing does not alter its original client.

All async methods take `Option<&RequestOptions>` last. One absolute deadline covers request normalization, multipart encoding, credential preparation, dispatch and response or stream consumption. Cancellation also applies to buffered stream events. The default operation timeout is 60 seconds; applications can supply a different total budget. The default request and buffered response bounds are 8 MiB, and the default SSE event bound is 1 MiB. Stream idle/session bounds and the shared concurrency limit also apply. Consumer applications own the async runtime and can supply their own admitted transport.

## Runtime surface

| Family | Native methods |
| --- | --- |
| Inference | `create_completion`, `create_prompt_completion`, `create_prompt_render`, `create_chat_completion`, `create_embedding`, `create_response` |
| Images and documents | `create_image`, `create_image_edit`, `create_image_variation`, `create_rerank`, `create_ocr`, `create_moderation` |
| Audio | `create_transcription`, `create_translation`, `create_speech` |
| Files | `create_file`, `list_files`, `retrieve_file`, `delete_file`, `download_file` |
| Fine tuning | `create_fine_tuning_job`, `list_paginated_fine_tuning_jobs`, `retrieve_fine_tuning_job`, `cancel_fine_tuning_job`, `list_fine_tuning_events`, `list_fine_tuning_job_checkpoints` |
| Models | `list_models`, `retrieve_model`, `delete_model` |
| Responses | `get_response`, `delete_response`, `list_input_items` |
| Vector stores | `create_vector_store`, `list_vector_stores`, `get_vector_store`, `modify_vector_store`, `delete_vector_store`, `create_vector_store_file`, `list_vector_store_files`, `get_vector_store_file`, `delete_vector_store_file`, `create_vector_store_file_batch`, `get_vector_store_file_batch`, `cancel_vector_store_file_batch`, `list_files_in_vector_store_batch` |
| Batches | `create_batch`, `list_batches`, `retrieve_batch`, `cancel_batch`, `get_batch_output` |
| Logs and feedback | `get_log`, `create_logs`, `create_feedback`, `update_feedback` |
| Experimental realtime (`realtime` feature) | `connect_realtime`; separate WebSocket connector and runtime-key upgrade |

Requests use the exact frozen route literals and schema-specific query contracts. Optional query fields are omitted when unset. Explicit zero, negative values permitted by their own schemas, false and empty strings are retained. Arrays become repeated ordered keys, including duplicates. No unrelated control-plane identifier length limit is imposed on runtime IDs; URL, traversal and control checks still apply. IDs are encoded once as individual path segments. `update_feedback` retains its separate UUID requirement.

JSON requests are normalized and validated before credentials or dispatch. Responses preserve additional fields and unfamiliar enum values under native response validation. Known literal mismatches and malformed known union branches are rejected. Error and model formatting conceals prompts, binary content, arbitrary response values and routing secrets. `download_file`, `get_batch_output` and `create_speech` return faithful `Bytes`; `delete_response` returns `()`.

## Native multipart and audio formats

Five operations accept real `BinaryFile` inputs: image edit, image variation, transcription, translation and file upload. Image edit has separate image and optional mask file slots, accounting for all six frozen Blob boundaries. JSON objects cannot manufacture a Blob. Typed builders expose `build()` for required-field construction; their operations perform complete mixed metadata/file validation under the same request budget. Prerelease review removed these five builders' JSON-only `build_validated` convenience, which could never validate a native file. JSON-capable builders retain it; a generator regression pins the exact exception.

`BinaryFile` construction owns caller bytes without a size ceiling. The operation validates all file slots, their aggregate bytes, filenames and MIME metadata, then bounds the complete multipart envelope. A supplied filename and MIME are retained; absent hints become `blob` and `application/octet-stream`. Source field order is retained, nullable metadata is omitted from the form, and array entries use repeated `[]` names. The transcription key is exactly `timestamp_granularities[]`, without adding another suffix. Defaults and cross-field constraints come from the complete frozen schema. All five request schemas are strict: unknown extensions and collisions with any known field, including an absent field, are rejected before dispatch. Boundaries are generated locally and checked against part headers and bytes.

Transcription and translation return `TranscriptionResult` and `TranslationResult` respectively. `text`, `srt` and `vtt` select `Text`; `verbose_json` selects its distinct typed verbose object; other accepted formats select the typed JSON response. Translation deliberately accepts an unfamiliar format string because its frozen schema does, while transcription retains its request vocabulary constraint. A response Content-Type does not change the chosen decoder. Text follows Fetch decoding: malformed UTF-8 is replaced and a leading UTF-8 BOM is removed. `create_logs` also returns text without JSON decoding or a required Content-Type.

## Streaming and explicit replay

Use `create_completion_stream`, `create_prompt_completion_stream`, `create_chat_completion_stream` or `create_response_stream` with a request containing `stream: true`. Their ordinary counterparts reject that state; stream variants reject a non-streaming body before dispatch. This replaces the TypeScript Promise-or-stream return union with an explicit native return type.

Streams are caller-driven, bounded and cancellable, with typed schema validation for each event. Completion, prompt-completion and chat streams require `[DONE]`; Responses streams require a typed `response.completed`, `response.failed` or `response.incomplete` terminal event. Those terminal events remain visible to the caller. `[DONE]` is not accepted as a Responses terminal. EOF before the required terminal, malformed JSON, invalid known events, provider error events and interrupted transport produce a sanitized error. Allowlisted request IDs and HTTP status remain available on stream errors. Successful headers permanently end replay eligibility.

The default `RuntimeReplay::Never` sends each operation once, including GET resources. `RequestOptions::with_retries` alone cannot enable runtime replay. A transport failure after dispatch reports `RemoteOutcome::Unknown`; applications reconcile before repeating work.

Applications may explicitly opt in with `with_replay(RuntimeReplay::bounded(n)?)`, where `n` is zero through five additional attempts within the original total deadline. Only transport failure before successful headers and HTTP 500, 502, 503 or 504 qualify. HTTP 429, malformed successful responses and interrupted successful bodies do not. Prepared body bytes and routing remain identical across attempts. Explicit replay can duplicate computation, remote mutations and charges when an earlier result was lost. The explicit runtime permit controls this budget; the safe-read retry override does not alter it.

Optional [metadata tracing](OBSERVABILITY.md) records fixed HTTP, decoding, body/typed-stream and realtime lifecycle phases. It does not record model, routing, body, URL or correlation values, and installs no subscriber.

## Deliberate adaptations and evidence

- Explicit endpoint, runtime key, transport and limits replace environment/global fetch defaults. Runtime and SCM credentials remain separate.
- Immutable routing clones and an explicit replay permit replace the TypeScript per-call header/retry bag. Request lifecycle options use a total deadline, including preparation and stream consumption, rather than restarting a per-attempt timeout.
- Native files retain byte fidelity and receive complete operation-level validation. Strict extension checks include absent known keys; multipart Content-Type carries the locally generated boundary.
- Separate typed streaming methods and audio result enums replace dynamic return unions. Byte and text methods preserve their own contracts.
- Existing representative `create_chat_completion`, `create_chat_completion_stream` and `download_file` signatures remain available.

`contracts/native-operation-mappings/gateway-runtime.json` maps all 54 frozen methods to concrete implementations, actual tests and these adaptations. `tests/gateway_runtime_contracts.rs` directly consumes the frozen owned TypeScript fixture for 49 JSON/text/binary cases, checking exact methods, routes, ordered queries, header sets, sensitive flags and semantic JSON payloads. Source-derived expectations authored inside the native tests pin exact bytes, headers, ordering and defaults for all five multipart forms; these are not an independently captured multipart corpus. The suite also exercises response format selection, unknown response fields, all 54 default non-replay paths, four typed streams and their terminals, stream failure metadata, explicit replay eligibility, routing isolation, native-file spoofing/collisions/bounds, cancellation, deadlines and request limits. No live mutation or credential access is part of these tests.
