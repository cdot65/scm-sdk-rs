//! Independent source-derived environment precedence, isolated initialization and OAuth wiring.
use bytes::Bytes;
use futures_util::future::BoxFuture;
use http::{HeaderMap, Method, StatusCode};
use scm_sdk::{
    Endpoint, ErrorKind, OriginPolicy, Result, Secret, Session,
    auth::RefreshBuffer,
    configuration::{InitOptions, ScanConfiguration, init, init_from_environment},
    environment::{
        EnvironmentSnapshot, ResolveOAuthConfigOptions, SCAN_ENVIRONMENT_NAMES,
        resolve_oauth_config,
    },
    transport::{Transport, TransportRequest, TransportResponse},
};
use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
#[derive(Default)]
struct Wire {
    requests: Mutex<Vec<TransportRequest>>,
    calls: AtomicUsize,
}
impl Transport for Wire {
    fn send<'a>(&'a self, request: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            let token = request.url().host_str() == Some("issuer.example");
            if token {
                assert_eq!(request.method(), Method::POST);
                assert_eq!(
                    request.headers()["content-type"],
                    "application/x-www-form-urlencoded"
                );
                assert!(request.headers()["authorization"].is_sensitive());
            }
            self.requests.lock().unwrap().push(request);
            let body = if token {
                Bytes::from_static(
                    br#"{"access_token":"fixture-token","token_type":"Bearer","expires_in":100}"#,
                )
            } else {
                Bytes::from_static(b"{}")
            };
            Ok(TransportResponse::from_bytes(
                StatusCode::OK,
                HeaderMap::new(),
                body,
            ))
        })
    }
}
fn scan_options(key: &str) -> InitOptions {
    let mut o = InitOptions::default();
    o.api_key = Some(Secret::new(key).unwrap());
    o
}
fn oauth_options() -> ResolveOAuthConfigOptions {
    let mut o = ResolveOAuthConfigOptions::new(
        Endpoint::parse("https://api.example/base").unwrap(),
        "PRIMARY",
    );
    o.fallback_env_prefix = Some("FALLBACK".into());
    o
}
fn oauth_env() -> EnvironmentSnapshot {
    EnvironmentSnapshot::from_pairs([
        ("PRIMARY_CLIENT_ID", "primary-client"),
        ("PRIMARY_CLIENT_SECRET", "primary-secret"),
        ("PRIMARY_TSG_ID", "123"),
        ("PRIMARY_TOKEN_ENDPOINT", "https://issuer.example/token"),
        ("FALLBACK_CLIENT_ID", "fallback-client"),
        ("FALLBACK_CLIENT_SECRET", "fallback-secret"),
        ("FALLBACK_TSG_ID", "456"),
        (
            "FALLBACK_TOKEN_ENDPOINT",
            "https://fallback-issuer.example/token",
        ),
    ])
    .unwrap()
}
#[test]
fn scan_init_reset_and_failed_reinitialization_are_isolated_and_atomic() {
    let mut c = ScanConfiguration::new().unwrap();
    assert!(!c.initialized());
    assert_eq!(c.num_retries(), 5);
    assert_eq!(
        c.api_endpoint().url().as_str(),
        "https://service.api.aisecurity.paloaltonetworks.com/"
    );
    assert!(c.client_builder().is_err());
    c.init(scan_options("first-secret")).unwrap();
    assert!(c.initialized());
    let mut invalid = scan_options("invalid-secret");
    invalid.num_retries = Some(6);
    assert!(c.init(invalid).is_err());
    assert_eq!(c.api_key().unwrap().expose(), "first-secret");
    let other = init(scan_options("other-secret")).unwrap();
    c.reset().unwrap();
    assert!(!c.initialized());
    assert!(c.api_key().is_none());
    assert_eq!(c.num_retries(), 5);
    assert_eq!(other.api_key().unwrap().expose(), "other-secret");
    assert!(!format!("{other:?}").contains("other-secret"));
    assert!(init(InitOptions::default()).is_err());
}
#[test]
fn explicit_scan_environment_priority_blank_suppression_and_js_trim_match_source() {
    assert_eq!(
        SCAN_ENVIRONMENT_NAMES,
        [
            "PANW_AI_SEC_API_KEY",
            "PANW_AI_SEC_API_TOKEN",
            "PANW_AI_SEC_API_ENDPOINT"
        ]
    );
    let env = EnvironmentSnapshot::from_pairs([
        ("PANW_AI_SEC_API_KEY", "\u{feff} env-key \u{2029}"),
        ("PANW_AI_SEC_API_TOKEN", " env-token "),
        ("PANW_AI_SEC_API_ENDPOINT", "https://scan.example/path///"),
    ])
    .unwrap();
    let c = init_from_environment(InitOptions::default(), &env).unwrap();
    assert_eq!(c.api_key().unwrap().expose(), "env-key");
    assert_eq!(c.api_token().unwrap().expose(), "env-token");
    assert_eq!(c.api_endpoint().url().as_str(), "https://scan.example/path");
    let mut opts = scan_options("\u{00a0} explicit-key \u{3000}");
    opts.api_token = Some(Secret::new(" \t").unwrap());
    opts.api_endpoint = Some(Endpoint::parse("https://explicit.example/").unwrap());
    opts.num_retries = Some(0);
    let c = init_from_environment(opts, &env).unwrap();
    assert_eq!(c.api_key().unwrap().expose(), "explicit-key");
    assert!(c.api_token().is_none());
    assert_eq!(c.num_retries(), 0);
    assert_eq!(c.api_endpoint().url().host_str(), Some("explicit.example"));
    let env = EnvironmentSnapshot::from_pairs([("PANW_AI_SEC_API_KEY", "fallback-key")]).unwrap();
    assert!(init_from_environment(scan_options("\u{feff} "), &env).is_err());
    let env = EnvironmentSnapshot::from_pairs([
        ("PANW_AI_SEC_API_KEY", ""),
        ("PANW_AI_SEC_API_TOKEN", "token"),
    ])
    .unwrap();
    assert!(
        init_from_environment(InitOptions::default(), &env)
            .unwrap()
            .api_key()
            .is_none()
    );
    assert_eq!(EnvironmentSnapshot::empty().get("MISSING"), None);
    assert!(!format!("{env:?}").contains("token"));
}
#[test]
fn invalid_credentials_limits_endpoints_and_environment_values_are_safe_errors() {
    for key in ["x".repeat(2049), "a\r\nrequest-secret".into(), "a b".into()] {
        let error = init(scan_options(&key)).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Configuration);
        assert!(!format!("{error:?} {error}").contains("request-secret"));
    }
    assert!(init(scan_options(&"x".repeat(2048))).is_ok());
    let mut local = scan_options("local-key");
    local.api_endpoint = Some(Endpoint::loopback("http://127.0.0.1:8321/scan///").unwrap());
    assert_eq!(
        init(local).unwrap().api_endpoint().url().as_str(),
        "http://127.0.0.1:8321/scan"
    );

    for endpoint in [
        "http://unsafe.example/",
        "https://user:request-secret@host.example/",
        "https://host.example/?token=request-secret",
        "not-a-url",
        "",
    ] {
        let env = EnvironmentSnapshot::from_pairs([
            ("PANW_AI_SEC_API_KEY", "key"),
            ("PANW_AI_SEC_API_ENDPOINT", endpoint),
        ])
        .unwrap();
        assert!(init_from_environment(InitOptions::default(), &env).is_err());
    }
    assert!(EnvironmentSnapshot::from_pairs([("BAD\0NAME", "request-secret")]).is_err());
    assert!(EnvironmentSnapshot::from_pairs([("GOOD", "x".repeat(16 * 1024 + 1))]).is_err());
    assert!(EnvironmentSnapshot::from_pairs((0..129).map(|n| (format!("VAR_{n}"), "x"))).is_err());
    assert!(
        EnvironmentSnapshot::from_pairs(
            (0..128).map(|n| (format!("VAR_{n}"), "x".repeat(16 * 1024)))
        )
        .is_err()
    );
    assert!(EnvironmentSnapshot::capture(&["BAD\0NAME"]).is_err());
    assert!(
        EnvironmentSnapshot::capture(&[])
            .unwrap()
            .get("PANW_AI_SEC_API_KEY")
            .is_none()
    );
}
#[tokio::test]
async fn built_scan_clients_keep_credentials_after_reset_and_both_headers_sign_exact_bytes() {
    let wire = Arc::new(Wire::default());
    let mut options = scan_options("Jefe");
    options.api_token = Some(Secret::new("scan-bearer").unwrap());
    let mut c = init(options).unwrap();
    let first = c
        .client_builder()
        .unwrap()
        .transport(wire.clone())
        .build()
        .unwrap();
    c.reset().unwrap();
    c.init(scan_options("other-key")).unwrap();
    let second = c
        .client_builder()
        .unwrap()
        .transport(wire.clone())
        .build()
        .unwrap();
    first
        .send_empty(
            scm_sdk::RawRequest::new(Method::POST, "/fixture")
                .body(
                    Bytes::from_static(b"what do ya want for nothing?"),
                    "text/plain",
                )
                .unwrap(),
            None,
        )
        .await
        .unwrap();
    second
        .send_empty(scm_sdk::RawRequest::read("/fixture"), None)
        .await
        .unwrap();
    let requests = wire.requests.lock().unwrap();
    assert_eq!(requests[0].headers()["x-pan-token"], "Jefe");
    assert_eq!(requests[0].headers()["authorization"], "Bearer scan-bearer");
    assert_eq!(
        requests[0].headers()["x-payload-hash"],
        "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
    );
    assert_eq!(requests[1].headers()["x-pan-token"], "other-key");
    assert!(!requests[1].headers().contains_key("authorization"));
    assert_eq!(first.limits().max_retries, 5);
}
#[test]
fn oauth_environment_precedence_empty_suppression_no_trim_and_retry_clamping_match_source() {
    let env = oauth_env();
    let primary = resolve_oauth_config(oauth_options(), &env).unwrap();
    assert_eq!(primary.context().client_id(), "primary-client");
    assert_eq!(primary.context().tsg_id(), "123");
    assert_eq!(
        primary.context().issuer().url().host_str(),
        Some("issuer.example")
    );
    assert_eq!(primary.num_retries(), 5);
    let fallback = EnvironmentSnapshot::from_pairs([
        ("FALLBACK_CLIENT_ID", "fallback-client"),
        ("FALLBACK_CLIENT_SECRET", "fallback-secret"),
        ("FALLBACK_TSG_ID", "456"),
    ])
    .unwrap();
    let resolved = resolve_oauth_config(oauth_options(), &fallback).unwrap();
    assert_eq!(resolved.context().client_id(), "fallback-client");
    assert_eq!(
        resolved.context().issuer().url().as_str(),
        "https://auth.apps.paloaltonetworks.com/oauth2/access_token"
    );
    for (n, expected) in [(-10, 0), (0, 0), (3, 3), (10, 5)] {
        let mut o = oauth_options();
        o.num_retries = Some(n);
        assert_eq!(
            resolve_oauth_config(o, &env).unwrap().num_retries(),
            expected
        );
    }
    let mut opts = oauth_options();
    opts.client_id = Some("explicit-client".into());
    opts.client_secret = Some(Secret::new("explicit-secret").unwrap());
    opts.tsg_id = Some("789".into());
    opts.token_endpoint = Some(Endpoint::parse("https://explicit-issuer.example/token").unwrap());
    let explicit = resolve_oauth_config(opts, &env).unwrap();
    assert_eq!(explicit.context().client_id(), "explicit-client");
    assert_eq!(explicit.context().tsg_id(), "789");
    let mut empty = oauth_options();
    empty.client_id = Some(String::new());
    assert!(resolve_oauth_config(empty, &env).is_err());
    let env = EnvironmentSnapshot::from_pairs([
        ("PRIMARY_CLIENT_ID", ""),
        ("FALLBACK_CLIENT_ID", "fallback-client"),
        ("PRIMARY_CLIENT_SECRET", "secret"),
        ("PRIMARY_TSG_ID", "123"),
    ])
    .unwrap();
    assert!(resolve_oauth_config(oauth_options(), &env).is_err());
    let mut no_trim = oauth_options();
    no_trim.tsg_id = Some(" 123 ".into());
    assert!(resolve_oauth_config(no_trim, &oauth_env()).is_err());
    let mut invalid = oauth_options();
    invalid.primary_env_prefix = "BAD/PREFIX".into();
    assert!(invalid.environment_names().is_err());
    assert_eq!(oauth_options().environment_names().unwrap().len(), 8);
    assert!(!format!("{primary:?}").contains("primary-client"));
}
#[tokio::test]
async fn oauth_resolution_propagates_buffer_events_transport_policy_and_session() {
    let mut opts = oauth_options();
    opts.provider_options.refresh_buffer = RefreshBuffer::Fixed(Duration::MAX);
    let c = resolve_oauth_config(opts, &oauth_env()).unwrap();
    let wire = Arc::new(Wire::default());
    let session = Session::new();
    let policy = Arc::new(OriginPolicy::new([
        c.base_url().clone(),
        c.context().issuer().clone(),
    ]));
    let provider = c
        .provider(wire.clone(), policy.clone(), session.clone())
        .unwrap();
    let mut events = provider.subscribe_refresh();
    provider.get_token(None).await.unwrap();
    assert!(events.recv().await.unwrap().is_expiring_soon);
    provider.get_token(None).await.unwrap();
    assert_eq!(wire.calls.load(Ordering::SeqCst), 2);
    let client = c
        .client_builder(true)
        .transport(wire.clone())
        .destination_policy(policy)
        .session(session.clone())
        .build()
        .unwrap();
    client
        .send_empty(scm_sdk::RawRequest::read("/fixture"), None)
        .await
        .unwrap();
    client
        .send_empty(scm_sdk::RawRequest::read("/fixture"), None)
        .await
        .unwrap();
    {
        let requests = wire.requests.lock().unwrap();
        let api: Vec<_> = requests
            .iter()
            .filter(|r| r.url().host_str() == Some("api.example"))
            .collect();
        assert_eq!(api.len(), 2);
        assert_eq!(api[0].headers()["x-tsg-id"], "123");
        assert_eq!(api[0].headers()["authorization"], "Bearer fixture-token");
        assert_eq!(
            requests
                .iter()
                .filter(|r| r.url().host_str() == Some("issuer.example"))
                .count(),
            4
        );
    }
    session.cancel();
    assert_eq!(
        provider.get_token(None).await.unwrap_err().kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(client.limits().max_retries, 5);
    let denied = Arc::new(OriginPolicy::new([c.base_url().clone()]));
    let provider = c.provider(wire.clone(), denied, Session::new()).unwrap();
    assert_eq!(
        provider.get_token(None).await.unwrap_err().kind(),
        ErrorKind::Destination
    );
}
#[test]
fn explicit_process_capture_is_whitelisted_in_a_child_without_global_mutation() {
    const MODE: &str = "SCM_SDK_ENV_CAPTURE_CHILD_MODE";
    const VALUE: &str = "SCM_SDK_ENV_CAPTURE_FIXTURE";
    if std::env::var_os(MODE).is_some() {
        let snapshot = EnvironmentSnapshot::capture(&[VALUE]).unwrap();
        assert_eq!(snapshot.get(VALUE), Some("fixture-secret"));
        assert!(snapshot.get("PATH").is_none());
        assert!(!format!("{snapshot:?}").contains("fixture-secret"));
        return;
    }
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("explicit_process_capture_is_whitelisted_in_a_child_without_global_mutation")
        .env(MODE, "1")
        .env(VALUE, "fixture-secret")
        .output()
        .unwrap();
    assert!(output.status.success());
}
#[cfg(unix)]
#[test]
fn non_utf8_environment_values_fail_safely_in_a_child() {
    use std::os::unix::ffi::OsStringExt;
    const MODE: &str = "SCM_SDK_ENV_NON_UTF8_CHILD_MODE";
    const VALUE: &str = "SCM_SDK_ENV_NON_UTF8_FIXTURE";
    if std::env::var_os(MODE).is_some() {
        let e = EnvironmentSnapshot::capture(&[VALUE]).unwrap_err();
        assert_eq!(e.kind(), ErrorKind::Configuration);
        assert!(!format!("{e:?} {e}").contains("FIXTURE"));
        return;
    }
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("non_utf8_environment_values_fail_safely_in_a_child")
        .env(MODE, "1")
        .env(VALUE, std::ffi::OsString::from_vec(vec![0xff]))
        .output()
        .unwrap();
    assert!(output.status.success());
}
