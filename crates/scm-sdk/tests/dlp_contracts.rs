//! Hand-authored wire expectations from the frozen DLP client implementations.
//! Dictionary multipart expectations also follow the frozen dictionaries.spec.ts.
#![cfg(feature = "dlp")]
use bytes::Bytes;
use futures_util::future::BoxFuture;
use http::{HeaderMap, StatusCode};
use scm_sdk::{
    Client, Endpoint, Error, ErrorKind, Limits, RemoteOutcome, RequestOptions, Result, Secret,
    auth::{AuthContext, Authentication, TokenLease, TokenProvider},
    dlp::{
        DataFilteringProfileListParams, DictionaryListParams, DictionaryUpload, DlpClient,
        FilteringStatus, SpringListParams, models::*,
    },
    pagination::{CollectOptions, Completion, Truncation},
    transport::{Transport, TransportRequest, TransportResponse},
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

const TOKEN: &str = "synthetic-dlp-bearer-secret";
struct Provider {
    context: AuthContext,
    calls: AtomicUsize,
    deadlines: Mutex<Vec<tokio::time::Instant>>,
}
impl Provider {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            context: AuthContext::new(
                Endpoint::parse("https://issuer.example/token").unwrap(),
                "dlp-client",
                1,
                "12345",
            )
            .unwrap(),
            calls: AtomicUsize::new(0),
            deadlines: Mutex::new(Vec::new()),
        })
    }
}
impl TokenProvider for Provider {
    fn token<'a>(&'a self, options: &'a RequestOptions) -> BoxFuture<'a, Result<TokenLease>> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.deadlines.lock().unwrap().push(options.deadline());
            TokenLease::new(Secret::new(TOKEN).unwrap(), 1, Duration::from_secs(3600))
        })
    }
    fn context(&self) -> &AuthContext {
        &self.context
    }
    fn invalidate(&self, _: u64) -> bool {
        false
    }
}
enum Reply {
    Json(Value),
    Empty,
    Failure,
}
struct Wire {
    requests: Mutex<Vec<TransportRequest>>,
    replies: Mutex<VecDeque<Reply>>,
}
impl Transport for Wire {
    fn send<'a>(&'a self, request: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            self.requests.lock().unwrap().push(request);
            match self
                .replies
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected DLP dispatch")
            {
                Reply::Json(value) => Ok(TransportResponse::from_bytes(
                    StatusCode::OK,
                    HeaderMap::new(),
                    Bytes::from(serde_json::to_vec(&value).unwrap()),
                )),
                Reply::Empty => Ok(TransportResponse::from_bytes(
                    StatusCode::NO_CONTENT,
                    HeaderMap::new(),
                    Bytes::new(),
                )),
                Reply::Failure => Err(Error::new(ErrorKind::Transport)),
            }
        })
    }
}
fn setup(replies: Vec<Reply>) -> (DlpClient, Arc<Wire>, Arc<Provider>) {
    setup_limits(replies, Limits::default())
}
fn setup_limits(replies: Vec<Reply>, limits: Limits) -> (DlpClient, Arc<Wire>, Arc<Provider>) {
    let wire = Arc::new(Wire {
        requests: Mutex::new(Vec::new()),
        replies: Mutex::new(replies.into()),
    });
    let provider = Provider::new();
    let client = Client::builder(Endpoint::parse("https://api.dlp.example.test").unwrap())
        .limits(limits)
        .transport(wire.clone())
        .authentication(Authentication::oauth(provider.clone(), true))
        .build()
        .unwrap();
    (DlpClient::new(client).unwrap(), wire, provider)
}
fn model<T: DeserializeOwned>(value: Value) -> T {
    serde_json::from_value(value).unwrap()
}
fn same<T: Serialize>(actual: T, expected: &Value) {
    assert_eq!(serde_json::to_value(actual).unwrap(), *expected);
}
fn pattern_body() -> Value {
    json!({"name":"SSN","type":"custom","detection_config":{"technique":"regex"},"matching_rules":{"regexes":[{"regex":"[0-9]+","weight":1}]}})
}
fn pattern_patch() -> Value {
    json!({"name":"SSN","type":"custom","detection_config":{"technique":"regex"},"description":null})
}
fn profile_body() -> Value {
    json!({"name":"Confidential","detection_rules":[{"rule_type":"expression_tree","expression_tree":{"operator_type":"and","rule_item":{"detection_technique":"regex","match_type":"include"}}}]})
}
fn profile_patch() -> Value {
    json!({"name":"Confidential","profile_type":"advanced","description":null})
}
fn metadata() -> Value {
    json!({"category":"Confidential","name":"PII","original_file_name":"pii.txt","region_name":"us"})
}
fn dictionary_patch() -> Value {
    json!({"category":"Confidential","name":"PII","original_file_name":"pii.txt","description":null})
}
fn filtering_body() -> Value {
    json!({"file_based":true,"non_file_based":false,"description":"Finance","criteria_details":[{"dataProfileId":42,"fileBased":"true"}]})
}
fn page(items: Vec<Value>, last: Option<bool>, total: Option<u64>) -> Value {
    let mut result = json!({"content":items,"future_page":{"private":"page-secret"}});
    if let Some(last) = last {
        result["last"] = json!(last);
    }
    if let Some(total) = total {
        result["totalPages"] = json!(total);
    }
    result
}
fn check(
    request: &TransportRequest,
    method: &str,
    path: &str,
    query: &[(&str, &str)],
    body: Option<(&Value, &str)>,
) {
    assert_eq!(request.method().as_str(), method);
    assert_eq!(request.url().host_str(), Some("api.dlp.example.test"));
    assert_eq!(request.url().path(), path);
    let actual: Vec<_> = request
        .url()
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect();
    assert_eq!(
        actual,
        query
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect::<Vec<_>>()
    );
    let mut names: Vec<_> = request.headers().keys().map(|key| key.as_str()).collect();
    names.sort_unstable();
    let mut expected = vec!["authorization", "service-name", "user-agent", "x-tsg-id"];
    if body.is_some() {
        expected.push("content-type");
    }
    expected.sort_unstable();
    assert_eq!(names, expected);
    for name in expected {
        assert_eq!(request.headers().get_all(name).iter().count(), 1);
    }
    assert!(request.headers().values().all(|value| value.is_sensitive()));
    assert_eq!(
        request.headers()["authorization"],
        format!("Bearer {TOKEN}")
    );
    assert_eq!(request.headers()["x-tsg-id"], "12345");
    assert_eq!(request.headers()["service-name"], "api");
    assert_eq!(
        request.headers()["user-agent"],
        concat!("PAN-AIRS/", env!("CARGO_PKG_VERSION"), "-rust-sdk")
    );
    if let Some((expected, content_type)) = body {
        assert_eq!(request.headers()["content-type"], content_type);
        assert_eq!(
            serde_json::from_slice::<Value>(request.body()).unwrap(),
            *expected
        );
    } else {
        assert!(request.body().is_empty());
    }
    assert!(!format!("{request:?} {:?}", request.headers()).contains(TOKEN));
}
fn check_form(
    request: &TransportRequest,
    method: &str,
    path: &str,
    query: &[(&str, &str)],
    metadata: &Value,
    file: &[u8],
    mime: &str,
) {
    assert_eq!(request.method().as_str(), method);
    assert_eq!(request.url().path(), path);
    assert_eq!(
        request
            .url()
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect::<Vec<_>>(),
        query
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<Vec<_>>()
    );
    let mut names: Vec<_> = request.headers().keys().map(|key| key.as_str()).collect();
    names.sort_unstable();
    assert_eq!(
        names,
        [
            "authorization",
            "content-type",
            "service-name",
            "user-agent",
            "x-tsg-id"
        ]
    );
    assert_eq!(
        request.headers()["authorization"],
        format!("Bearer {TOKEN}")
    );
    assert!(request.headers().values().all(|value| value.is_sensitive()));
    assert_eq!(request.url().host_str(), Some("api.dlp.example.test"));
    assert_eq!(request.headers()["x-tsg-id"], "12345");
    assert_eq!(request.headers()["service-name"], "api");
    assert_eq!(
        request.headers()["user-agent"],
        concat!("PAN-AIRS/", env!("CARGO_PKG_VERSION"), "-rust-sdk")
    );
    for name in names {
        assert_eq!(request.headers().get_all(name).iter().count(), 1);
    }
    assert!(!format!("{request:?} {:?}", request.headers()).contains(TOKEN));
    let content_type = request.headers()["content-type"].to_str().unwrap();
    let boundary = content_type
        .strip_prefix("multipart/form-data; boundary=")
        .unwrap();
    assert!(
        !file
            .windows(boundary.len())
            .any(|bytes| bytes == boundary.as_bytes())
    );
    let mut expected=format!("--{boundary}\r\nContent-Disposition: form-data; name=\"json\"; filename=\"metadata.json\"\r\nContent-Type: application/json\r\n\r\n").into_bytes();
    expected.extend(serde_json::to_vec(metadata).unwrap());
    expected.extend(format!("\r\n--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"pii.txt\"\r\nContent-Type: {mime}\r\n\r\n").as_bytes());
    expected.extend_from_slice(file);
    expected.extend(format!("\r\n--{boundary}--\r\n").as_bytes());
    assert_eq!(request.body().as_ref(), expected);
}

#[tokio::test]
async fn data_pattern_crud_matches_all_six_frozen_routes_and_merge_patch() {
    let response = json!({"id":"dp-1","name":"SSN","status":"future-status","future_pattern":{"private":"pattern-secret"}});
    let list = page(vec![response.clone()], Some(true), Some(1));
    let (client, wire, _) = setup(vec![
        Reply::Json(list.clone()),
        Reply::Json(response.clone()),
        Reply::Json(response.clone()),
        Reply::Json(response.clone()),
        Reply::Json(response.clone()),
        Reply::Empty,
    ]);
    let resource = client.data_patterns();
    let params = SpringListParams::new().page(1).size(2).sort([
        "name,asc".into(),
        "type,desc".into(),
        "name,asc".into(),
    ]);
    same(resource.list(&params, None).await.unwrap(), &list);
    same(
        resource.create(&model(pattern_body()), None).await.unwrap(),
        &response,
    );
    same(resource.get("dp-1", None).await.unwrap(), &response);
    same(
        resource
            .replace("dp-1", &model(pattern_body()), None)
            .await
            .unwrap(),
        &response,
    );
    same(
        resource
            .patch("dp-1", &model(pattern_patch()), None)
            .await
            .unwrap(),
        &response,
    );
    resource.delete("dp-1", None).await.unwrap();
    let requests = wire.requests.lock().unwrap();
    assert_eq!(requests.len(), 6);
    check(
        &requests[0],
        "GET",
        "/v2/api/data-patterns",
        &[
            ("page", "1"),
            ("size", "2"),
            ("sort", "name,asc"),
            ("sort", "type,desc"),
            ("sort", "name,asc"),
        ],
        None,
    );
    check(
        &requests[1],
        "POST",
        "/v2/api/data-patterns",
        &[],
        Some((&pattern_body(), "application/json")),
    );
    check(&requests[2], "GET", "/v2/api/data-patterns/dp-1", &[], None);
    check(
        &requests[3],
        "PUT",
        "/v2/api/data-patterns/dp-1",
        &[],
        Some((&pattern_body(), "application/json")),
    );
    check(
        &requests[4],
        "PATCH",
        "/v2/api/data-patterns/dp-1",
        &[],
        Some((&pattern_patch(), "application/merge-patch+json")),
    );
    check(
        &requests[5],
        "DELETE",
        "/v2/api/data-patterns/dp-1",
        &[],
        None,
    );
}

#[tokio::test]
async fn dictionary_crud_preserves_multipart_keywords_patch_and_optional_replace() {
    let response = json!({"id":"dict-1","name":"PII","type":"future-dictionary-type","keywords":["alpha","bravo"],"future_dictionary":true});
    let list = page(vec![response.clone()], Some(true), None);
    let (client, wire, _) = setup(vec![
        Reply::Json(list.clone()),
        Reply::Json(response.clone()),
        Reply::Json(response.clone()),
        Reply::Json(response.clone()),
        Reply::Empty,
        Reply::Json(response.clone()),
        Reply::Empty,
    ]);
    let resource = client.dictionaries();
    let mut params = DictionaryListParams::default();
    params.page = Some(1);
    params.size = Some(2);
    params.sort = vec!["name,asc".into(), "category,desc".into()];
    params.keywords = Some(false);
    same(resource.list(&params, None).await.unwrap(), &list);
    let upload = DictionaryUpload::text(model(metadata()), "alpha\nbravo\n")
        .unwrap()
        .include_keywords(true);
    same(resource.create(&upload, None).await.unwrap(), &response);
    same(
        resource.get("dict-1", Some(false), None).await.unwrap(),
        &response,
    );
    same(
        resource
            .replace("dict-1", &upload, None)
            .await
            .unwrap()
            .unwrap(),
        &response,
    );
    let binary = DictionaryUpload::new(
        model(metadata()),
        BinaryFile::new(Bytes::from_static(&[0, 255, 10]))
            .unwrap()
            .with_filename("ignored-hint.bin")
            .unwrap(),
    );
    assert!(
        resource
            .replace("dict-1", &binary, None)
            .await
            .unwrap()
            .is_none()
    );
    same(
        resource
            .patch("dict-1", &model(dictionary_patch()), None)
            .await
            .unwrap(),
        &response,
    );
    resource.delete("dict-1", None).await.unwrap();
    let requests = wire.requests.lock().unwrap();
    assert_eq!(requests.len(), 7);
    check(
        &requests[0],
        "GET",
        "/v2/api/dictionaries",
        &[
            ("page", "1"),
            ("size", "2"),
            ("sort", "name,asc"),
            ("sort", "category,desc"),
            ("keywords", "false"),
        ],
        None,
    );
    check_form(
        &requests[1],
        "POST",
        "/v2/api/dictionaries",
        &[("keywords", "true")],
        &metadata(),
        b"alpha\nbravo\n",
        "text/plain",
    );
    check(
        &requests[2],
        "GET",
        "/v2/api/dictionaries/dict-1",
        &[("keywords", "false")],
        None,
    );
    check_form(
        &requests[3],
        "PUT",
        "/v2/api/dictionaries/dict-1",
        &[("keywords", "true")],
        &metadata(),
        b"alpha\nbravo\n",
        "text/plain",
    );
    check_form(
        &requests[4],
        "PUT",
        "/v2/api/dictionaries/dict-1",
        &[],
        &metadata(),
        &[0, 255, 10],
        "application/octet-stream",
    );
    check(
        &requests[5],
        "PATCH",
        "/v2/api/dictionaries/dict-1",
        &[],
        Some((&dictionary_patch(), "application/merge-patch+json")),
    );
    check(
        &requests[6],
        "DELETE",
        "/v2/api/dictionaries/dict-1",
        &[],
        None,
    );
}

#[tokio::test]
async fn data_profile_crud_matches_all_five_frozen_routes() {
    let response = json!({"id":"prof-1","profile_type":"future-profile-type","description":null,"future_profile":true});
    let list = page(vec![response.clone()], Some(true), None);
    let (client, wire, _) = setup(vec![
        Reply::Json(list.clone()),
        Reply::Json(response.clone()),
        Reply::Json(response.clone()),
        Reply::Json(response.clone()),
        Reply::Json(response.clone()),
    ]);
    let resource = client.data_profiles();
    same(
        resource
            .list(&SpringListParams::default(), None)
            .await
            .unwrap(),
        &list,
    );
    same(
        resource.create(&model(profile_body()), None).await.unwrap(),
        &response,
    );
    same(resource.get("prof-1", None).await.unwrap(), &response);
    same(
        resource
            .replace("prof-1", &model(profile_body()), None)
            .await
            .unwrap(),
        &response,
    );
    same(
        resource
            .patch("prof-1", &model(profile_patch()), None)
            .await
            .unwrap(),
        &response,
    );
    let requests = wire.requests.lock().unwrap();
    assert_eq!(requests.len(), 5);
    check(&requests[0], "GET", "/v2/api/data-profiles", &[], None);
    check(
        &requests[1],
        "POST",
        "/v2/api/data-profiles",
        &[],
        Some((&profile_body(), "application/json")),
    );
    check(
        &requests[2],
        "GET",
        "/v2/api/data-profiles/prof-1",
        &[],
        None,
    );
    check(
        &requests[3],
        "PUT",
        "/v2/api/data-profiles/prof-1",
        &[],
        Some((&profile_body(), "application/json")),
    );
    check(
        &requests[4],
        "PATCH",
        "/v2/api/data-profiles/prof-1",
        &[],
        Some((&profile_patch(), "application/merge-patch+json")),
    );
}

#[tokio::test]
async fn filtering_profiles_match_all_three_frozen_routes_and_filters() {
    let response =
        json!({"id":"dfp-1","scan_type":"future-scan-type","file_based":true,"future_filter":true});
    let list = page(vec![response.clone()], Some(true), None);
    let (client, wire, _) = setup(vec![
        Reply::Json(list.clone()),
        Reply::Json(response.clone()),
        Reply::Json(response.clone()),
    ]);
    let resource = client.data_filtering_profiles();
    let mut params = DataFilteringProfileListParams::default();
    params.page = Some(1);
    params.size = Some(2);
    params.sort = vec!["name,asc".into(), "id,desc".into()];
    params.status = Some(FilteringStatus::Enabled);
    params.name = Some("Finance / global".into());
    same(resource.list(&params, None).await.unwrap(), &list);
    same(resource.get("dfp-1", None).await.unwrap(), &response);
    same(
        resource
            .replace("dfp-1", &model(filtering_body()), None)
            .await
            .unwrap(),
        &response,
    );
    let requests = wire.requests.lock().unwrap();
    assert_eq!(requests.len(), 3);
    check(
        &requests[0],
        "GET",
        "/v2/api/data-filtering-profiles",
        &[
            ("page", "1"),
            ("size", "2"),
            ("sort", "name,asc"),
            ("sort", "id,desc"),
            ("status", "enabled"),
            ("name", "Finance / global"),
        ],
        None,
    );
    check(
        &requests[1],
        "GET",
        "/v2/api/data-filtering-profiles/dfp-1",
        &[],
        None,
    );
    check(
        &requests[2],
        "PUT",
        "/v2/api/data-filtering-profiles/dfp-1",
        &[],
        Some((&filtering_body(), "application/json")),
    );
}

#[tokio::test]
async fn all_four_list_all_methods_use_spring_defaults_and_explicit_completion() {
    let response = page(
        vec![json!({"id":"owned-resource","future_item":true})],
        Some(true),
        Some(1),
    );
    let (client, wire, _) = setup((0..4).map(|_| Reply::Json(response.clone())).collect());
    let patterns = client
        .data_patterns()
        .list_all(
            &SpringListParams::default(),
            CollectOptions::default(),
            None,
        )
        .await
        .unwrap();
    let dictionaries = client
        .dictionaries()
        .list_all(
            &DictionaryListParams::default(),
            CollectOptions::default(),
            None,
        )
        .await
        .unwrap();
    let profiles = client
        .data_profiles()
        .list_all(
            &SpringListParams::default(),
            CollectOptions::default(),
            None,
        )
        .await
        .unwrap();
    let filtering = client
        .data_filtering_profiles()
        .list_all(
            &DataFilteringProfileListParams::default(),
            CollectOptions::default(),
            None,
        )
        .await
        .unwrap();
    for (complete, items) in [
        (
            patterns.is_complete(),
            serde_json::to_value(patterns.items).unwrap(),
        ),
        (
            dictionaries.is_complete(),
            serde_json::to_value(dictionaries.items).unwrap(),
        ),
        (
            profiles.is_complete(),
            serde_json::to_value(profiles.items).unwrap(),
        ),
        (
            filtering.is_complete(),
            serde_json::to_value(filtering.items).unwrap(),
        ),
    ] {
        assert!(complete);
        assert_eq!(items, response["content"]);
    }
    let requests = wire.requests.lock().unwrap();
    assert_eq!(requests.len(), 4);
    for (request, path) in requests.iter().zip([
        "/v2/api/data-patterns",
        "/v2/api/dictionaries",
        "/v2/api/data-profiles",
        "/v2/api/data-filtering-profiles",
    ]) {
        check(request, "GET", path, &[("page", "0"), ("size", "50")], None);
    }
}

#[tokio::test]
async fn spring_walk_honors_last_then_total_pages_then_short_page_and_shared_deadline() {
    let first = json!({"id":"first"});
    let second = json!({"id":"second"});
    // Explicit last wins even over a contradictory totalPages and full content.
    let (client, wire, _) = setup(vec![Reply::Json(page(
        vec![first.clone()],
        Some(true),
        Some(99),
    ))]);
    let params = SpringListParams::new().size(1);
    let collection = client
        .data_patterns()
        .list_all(&params, CollectOptions::default(), None)
        .await
        .unwrap();
    assert!(collection.is_complete());
    assert_eq!(wire.requests.lock().unwrap().len(), 1);
    // totalPages wins over short content, and every page shares one absolute deadline.
    let (client, wire, provider) = setup(vec![
        Reply::Json(page(vec![first.clone()], None, Some(2))),
        Reply::Json(page(vec![second.clone()], None, Some(2))),
    ]);
    let collection = client
        .data_patterns()
        .list_all(
            &SpringListParams::default(),
            CollectOptions::default(),
            None,
        )
        .await
        .unwrap();
    assert!(collection.is_complete());
    same(collection.items, &json!([first, second]));
    {
        let deadlines = provider.deadlines.lock().unwrap();
        assert_eq!(deadlines.len(), 2);
        assert_eq!(deadlines[0], deadlines[1]);
        let requests = wire.requests.lock().unwrap();
        check(
            &requests[0],
            "GET",
            "/v2/api/data-patterns",
            &[("page", "0"), ("size", "50")],
            None,
        );
        check(
            &requests[1],
            "GET",
            "/v2/api/data-patterns",
            &[("page", "1"), ("size", "50")],
            None,
        );
    }
    // Without metadata a full page continues and an empty page is a final short page.
    let (client, wire, _) = setup(vec![
        Reply::Json(page(vec![json!({"id":"only"})], None, None)),
        Reply::Json(page(vec![], None, None)),
    ]);
    let collection = client
        .data_patterns()
        .list_all(&params, CollectOptions::default(), None)
        .await
        .unwrap();
    assert!(collection.is_complete());
    assert_eq!(collection.items.len(), 1);
    assert_eq!(wire.requests.lock().unwrap().len(), 2);
}

#[tokio::test]
async fn empty_nonfinal_and_invalid_total_pages_return_partial_resumable_failures() {
    let (client, wire, _) = setup(vec![Reply::Json(page(vec![], Some(false), Some(2)))]);
    let error = client
        .data_patterns()
        .list_all(
            &SpringListParams::default(),
            CollectOptions::default(),
            None,
        )
        .await
        .unwrap_err();
    assert_eq!(error.error().kind(), ErrorKind::Pagination);
    assert!(error.partial().items.is_empty());
    assert_eq!(error.partial().resume.as_ref().unwrap().cursor, 0);
    assert_eq!(wire.requests.lock().unwrap().len(), 1);
    let mut invalid = page(vec![json!({"id":"bad-page"})], None, None);
    invalid["totalPages"] = json!(-1);
    let (client, _, _) = setup(vec![Reply::Json(invalid)]);
    assert_eq!(
        client
            .data_patterns()
            .list_all(
                &SpringListParams::default(),
                CollectOptions::default(),
                None
            )
            .await
            .unwrap_err()
            .error()
            .kind(),
        ErrorKind::Pagination
    );
    let (client, _, _) = setup(vec![
        Reply::Json(page(vec![json!({"id":"kept"})], Some(false), None)),
        Reply::Failure,
    ]);
    let options = RequestOptions::with_timeout(Duration::from_secs(5))
        .unwrap()
        .with_retries(0)
        .unwrap();
    let error = client
        .data_patterns()
        .list_all(
            &SpringListParams::default(),
            CollectOptions::default(),
            Some(&options),
        )
        .await
        .unwrap_err();
    assert_eq!(error.error().kind(), ErrorKind::Transport);
    same(&error.partial().items, &json!([{"id":"kept"}]));
    assert_eq!(error.partial().resume.as_ref().unwrap().cursor, 1);
}

#[tokio::test]
async fn collection_item_byte_and_page_caps_are_explicit_and_resumable() {
    let response = page(
        vec![json!({"id":"a","private":"row-secret"}), json!({"id":"b"})],
        Some(true),
        None,
    );
    let (client, wire, _) = setup(vec![Reply::Json(response.clone()), Reply::Json(response)]);
    let mut cap = CollectOptions::default();
    cap.max_items = 1;
    let result = client
        .data_patterns()
        .list_all(&SpringListParams::default(), cap, None)
        .await
        .unwrap();
    assert_eq!(result.completion, Completion::Truncated(Truncation::Items));
    assert_eq!(result.items.len(), 1);
    let resume = result.resume.clone().unwrap();
    assert_eq!((resume.cursor, resume.item_offset), (0, 1));
    assert!(!format!("{result:?}").contains("row-secret"));
    let remaining = client
        .data_patterns()
        .list_items(&SpringListParams::default(), 10, None)
        .unwrap()
        .resume_from(resume)
        .unwrap()
        .collect(CollectOptions::default())
        .await
        .unwrap();
    assert!(remaining.is_complete());
    same(remaining.items, &json!([{"id":"b"}]));
    assert_eq!(wire.requests.lock().unwrap().len(), 2);
    let (client, wire, _) = setup(vec![Reply::Json(page(
        vec![json!({"id":"large"})],
        Some(true),
        None,
    ))]);
    let mut cap = CollectOptions::default();
    cap.max_bytes = 2;
    let result = client
        .data_patterns()
        .list_all(&SpringListParams::default(), cap, None)
        .await
        .unwrap();
    assert_eq!(result.completion, Completion::Truncated(Truncation::Bytes));
    assert!(result.items.is_empty());
    assert_eq!(result.resume.unwrap().item_offset, 0);
    assert_eq!(wire.requests.lock().unwrap().len(), 1);
    let (client, wire, _) = setup(vec![Reply::Json(page(
        vec![json!({"id":"first"})],
        Some(false),
        Some(2),
    ))]);
    let mut cap = CollectOptions::default();
    cap.max_pages = 1;
    let result = client
        .data_patterns()
        .list_all(&SpringListParams::default(), cap, None)
        .await
        .unwrap();
    assert_eq!(result.completion, Completion::Truncated(Truncation::Pages));
    assert_eq!(result.resume.unwrap().cursor, 1);
    assert_eq!(wire.requests.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn buffered_items_observe_cancellation_without_another_credential_lookup() {
    let (client, wire, provider) = setup(vec![Reply::Json(page(
        vec![json!({"id":"first"}), json!({"id":"second"})],
        Some(true),
        None,
    ))]);
    let options = RequestOptions::with_timeout(Duration::from_secs(5)).unwrap();
    let mut items = client
        .data_profiles()
        .list_items(&SpringListParams::default(), 10, Some(&options))
        .unwrap();
    assert!(items.next().await.unwrap().is_some());
    options.cancel();
    assert_eq!(items.next().await.unwrap_err().kind(), ErrorKind::Cancelled);
    assert_eq!(provider.calls.load(Ordering::SeqCst), 1);
    assert_eq!(wire.requests.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn patch_omission_null_and_value_produce_exact_merge_patch_bytes() {
    let (client, wire, _) = setup((0..3).map(|_| Reply::Json(json!({}))).collect());
    let mut omitted = pattern_patch();
    omitted.as_object_mut().unwrap().remove("description");
    let mut replacement = omitted.clone();
    replacement["description"] = json!("new value");
    for body in [omitted.clone(), pattern_patch(), replacement.clone()] {
        client
            .data_patterns()
            .patch("id", &model(body), None)
            .await
            .unwrap();
    }
    let requests = wire.requests.lock().unwrap();
    for (request, body) in requests.iter().zip([omitted, pattern_patch(), replacement]) {
        check(
            request,
            "PATCH",
            "/v2/api/data-patterns/id",
            &[],
            Some((&body, "application/merge-patch+json")),
        );
        assert_eq!(request.body().as_ref(), serde_json::to_vec(&body).unwrap());
    }
}

#[tokio::test]
async fn invalid_queries_bodies_uploads_and_ids_precede_credentials_and_dispatch() {
    let (client, wire, provider) = setup(vec![]);
    let invalid = SpringListParams::new().size(0);
    assert_eq!(
        client
            .data_patterns()
            .list(&invalid, None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Validation
    );
    assert_eq!(
        client
            .data_profiles()
            .list(&invalid, None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Validation
    );
    assert!(
        client
            .data_patterns()
            .list_pages(
                &SpringListParams::new().page(9_007_199_254_740_992),
                10,
                None
            )
            .is_err()
    );
    assert!(
        client
            .data_patterns()
            .list_pages(&SpringListParams::default(), 0, None)
            .is_err()
    );
    let mut invalid = DictionaryListParams::default();
    invalid.size = Some(0);
    assert!(client.dictionaries().list(&invalid, None).await.is_err());
    let mut invalid = DataFilteringProfileListParams::default();
    invalid.name = Some("bad\r\nsecret".into());
    assert!(
        client
            .data_filtering_profiles()
            .list(&invalid, None)
            .await
            .is_err()
    );
    let mut body = pattern_body();
    body["name"] = json!("");
    assert!(
        client
            .data_patterns()
            .create(&model(body), None)
            .await
            .is_err()
    );
    let mut body = profile_body();
    body["name"] = json!("");
    assert!(
        client
            .data_profiles()
            .create(&model(body), None)
            .await
            .is_err()
    );
    let mut body = filtering_body();
    body["direction"] = json!("future-direction");
    assert!(
        client
            .data_filtering_profiles()
            .replace("id", &model(body), None)
            .await
            .is_err()
    );
    let mut invalid = metadata();
    invalid["category"] = json!("future-category");
    let upload = DictionaryUpload::text(model(invalid), "data").unwrap();
    assert!(client.dictionaries().create(&upload, None).await.is_err());
    let mut invalid = metadata();
    invalid["original_file_name"] = json!("bad\r\nInjected: secret");
    let upload = DictionaryUpload::text(model(invalid), "data").unwrap();
    let error = client
        .dictionaries()
        .create(&upload, None)
        .await
        .unwrap_err();
    assert!(!format!("{error:?}").contains("Injected"));
    for id in ["", "..", "../escape", "%252e%252e"] {
        assert!(client.data_patterns().get(id, None).await.is_err());
        assert!(client.dictionaries().get(id, None, None).await.is_err());
        assert!(client.data_profiles().get(id, None).await.is_err());
        assert!(
            client
                .data_filtering_profiles()
                .get(id, None)
                .await
                .is_err()
        );
    }
    let mut cap = CollectOptions::default();
    cap.max_items = 0;
    assert!(
        client
            .dictionaries()
            .list_all(&DictionaryListParams::default(), cap, None)
            .await
            .is_err()
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    assert!(wire.requests.lock().unwrap().is_empty());
}

#[tokio::test]
async fn dictionary_file_and_multipart_envelope_bounds_precede_credentials() {
    let mut limits = Limits::default();
    limits.max_request_bytes = 1024;
    let (client, wire, provider) = setup_limits(vec![], limits);
    for length in [1025, 800] {
        let upload = DictionaryUpload::new(
            model(metadata()),
            BinaryFile::new(Bytes::from(vec![b'x'; length])).unwrap(),
        );
        assert_eq!(
            client
                .dictionaries()
                .create(&upload, None)
                .await
                .unwrap_err()
                .kind(),
            ErrorKind::Limit
        );
    }
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    assert!(wire.requests.lock().unwrap().is_empty());
}

#[tokio::test]
async fn resource_identifiers_are_encoded_once_for_each_family() {
    let (client, wire, _) = setup((0..4).map(|_| Reply::Json(json!({}))).collect());
    let id = "weird id/slash%literal";
    client.data_patterns().get(id, None).await.unwrap();
    client.dictionaries().get(id, None, None).await.unwrap();
    client.data_profiles().get(id, None).await.unwrap();
    client
        .data_filtering_profiles()
        .get(id, None)
        .await
        .unwrap();
    let requests = wire.requests.lock().unwrap();
    for (request, prefix) in requests.iter().zip([
        "data-patterns",
        "dictionaries",
        "data-profiles",
        "data-filtering-profiles",
    ]) {
        assert_eq!(
            request.url().path(),
            format!("/v2/api/{prefix}/weird%20id%2Fslash%25literal")
        );
    }
}

#[tokio::test]
async fn all_twelve_dlp_mutations_preserve_uncertain_outcomes_without_replay() {
    let (client, wire, _) = setup((0..12).map(|_| Reply::Failure).collect());
    let options = RequestOptions::with_timeout(Duration::from_secs(10))
        .unwrap()
        .with_retries(5)
        .unwrap();
    let options = Some(&options);
    let patterns = client.data_patterns();
    let dictionaries = client.dictionaries();
    let profiles = client.data_profiles();
    let filtering = client.data_filtering_profiles();
    let upload = DictionaryUpload::text(model(metadata()), "alpha\n").unwrap();
    let errors = [
        patterns
            .create(&model(pattern_body()), options)
            .await
            .unwrap_err(),
        patterns
            .replace("id", &model(pattern_body()), options)
            .await
            .unwrap_err(),
        patterns
            .patch("id", &model(pattern_patch()), options)
            .await
            .unwrap_err(),
        patterns.delete("id", options).await.unwrap_err(),
        dictionaries.create(&upload, options).await.unwrap_err(),
        dictionaries
            .replace("id", &upload, options)
            .await
            .unwrap_err(),
        dictionaries
            .patch("id", &model(dictionary_patch()), options)
            .await
            .unwrap_err(),
        dictionaries.delete("id", options).await.unwrap_err(),
        profiles
            .create(&model(profile_body()), options)
            .await
            .unwrap_err(),
        profiles
            .replace("id", &model(profile_body()), options)
            .await
            .unwrap_err(),
        profiles
            .patch("id", &model(profile_patch()), options)
            .await
            .unwrap_err(),
        filtering
            .replace("id", &model(filtering_body()), options)
            .await
            .unwrap_err(),
    ];
    for error in errors {
        assert_eq!(
            (error.kind(), error.remote_outcome()),
            (ErrorKind::Transport, RemoteOutcome::Unknown)
        );
        assert!(!format!("{error:?} {error}").contains(TOKEN));
    }
    assert_eq!(wire.requests.lock().unwrap().len(), 12);
}

#[tokio::test]
async fn native_page_and_item_iterators_work_for_all_four_resource_families() {
    let response = page(vec![json!({"id":"iterator-resource"})], Some(true), None);
    let (client, wire, _) = setup((0..8).map(|_| Reply::Json(response.clone())).collect());
    let params = SpringListParams::default();
    let dictionary = DictionaryListParams::default();
    let filtering = DataFilteringProfileListParams::default();
    let mut pages = client
        .data_patterns()
        .list_pages(&params, 10, None)
        .unwrap();
    assert_eq!(pages.next_page().await.unwrap().unwrap().items.len(), 1);
    assert!(pages.next_page().await.unwrap().is_none());
    let mut pages = client
        .dictionaries()
        .list_pages(&dictionary, 10, None)
        .unwrap();
    assert_eq!(pages.next_page().await.unwrap().unwrap().items.len(), 1);
    assert!(pages.next_page().await.unwrap().is_none());
    let mut pages = client
        .data_profiles()
        .list_pages(&params, 10, None)
        .unwrap();
    assert_eq!(pages.next_page().await.unwrap().unwrap().items.len(), 1);
    assert!(pages.next_page().await.unwrap().is_none());
    let mut pages = client
        .data_filtering_profiles()
        .list_pages(&filtering, 10, None)
        .unwrap();
    assert_eq!(pages.next_page().await.unwrap().unwrap().items.len(), 1);
    assert!(pages.next_page().await.unwrap().is_none());
    let mut items = client
        .data_patterns()
        .list_items(&params, 10, None)
        .unwrap();
    assert!(items.next().await.unwrap().is_some());
    assert!(items.next().await.unwrap().is_none());
    let mut items = client
        .dictionaries()
        .list_items(&dictionary, 10, None)
        .unwrap();
    assert!(items.next().await.unwrap().is_some());
    assert!(items.next().await.unwrap().is_none());
    let mut items = client
        .data_profiles()
        .list_items(&params, 10, None)
        .unwrap();
    assert!(items.next().await.unwrap().is_some());
    assert!(items.next().await.unwrap().is_none());
    let mut items = client
        .data_filtering_profiles()
        .list_items(&filtering, 10, None)
        .unwrap();
    assert!(items.next().await.unwrap().is_some());
    assert!(items.next().await.unwrap().is_none());
    assert_eq!(wire.requests.lock().unwrap().len(), 8);
}

#[test]
fn dlp_constructor_rejects_scan_runtime_and_missing_authentication() {
    let wire = Arc::new(Wire {
        requests: Mutex::new(Vec::new()),
        replies: Mutex::new(VecDeque::new()),
    });
    for authentication in [
        Authentication::None,
        Authentication::scan(Some(Secret::new("synthetic-scan-key").unwrap()), None).unwrap(),
        Authentication::runtime_key(Secret::new("synthetic-runtime-key").unwrap()).unwrap(),
    ] {
        let client = Client::builder(Endpoint::parse("https://api.dlp.example.test").unwrap())
            .transport(wire.clone())
            .authentication(authentication)
            .build()
            .unwrap();
        assert_eq!(
            DlpClient::new(client).unwrap_err().kind(),
            ErrorKind::Configuration
        );
    }
    assert!(wire.requests.lock().unwrap().is_empty());
}

#[tokio::test]
async fn caller_selected_tenant_header_is_preserved_for_all_canonical_dlp_lists() {
    let (enabled, wire, _) = setup(
        (0..8)
            .map(|_| Reply::Json(page(vec![], Some(true), None)))
            .collect(),
    );
    let disabled = DlpClient::new(enabled.client().with_tsg_header(false).unwrap()).unwrap();
    for client in [&enabled, &disabled] {
        client
            .data_patterns()
            .list(&SpringListParams::default(), None)
            .await
            .unwrap();
        client
            .dictionaries()
            .list(&DictionaryListParams::default(), None)
            .await
            .unwrap();
        client
            .data_profiles()
            .list(&SpringListParams::default(), None)
            .await
            .unwrap();
        client
            .data_filtering_profiles()
            .list(&DataFilteringProfileListParams::default(), None)
            .await
            .unwrap();
    }
    let requests = wire.requests.lock().unwrap();
    for (index, request) in requests.iter().enumerate() {
        let mut names: Vec<_> = request.headers().keys().map(|name| name.as_str()).collect();
        names.sort_unstable();
        let expected = if index < 4 {
            vec!["authorization", "service-name", "user-agent", "x-tsg-id"]
        } else {
            vec!["authorization", "service-name", "user-agent"]
        };
        assert_eq!(names, expected);
        assert_eq!(
            request.headers()["authorization"],
            format!("Bearer {TOKEN}")
        );
        assert_eq!(request.headers()["service-name"], "api");
        assert_eq!(
            request.headers()["user-agent"],
            concat!("PAN-AIRS/", env!("CARGO_PKG_VERSION"), "-rust-sdk")
        );
        assert_eq!(
            request
                .headers()
                .get("x-tsg-id")
                .map(|v| v.to_str().unwrap()),
            (index < 4).then_some("12345")
        );
        assert!(request.headers().values().all(|value| value.is_sensitive()));
    }
}
