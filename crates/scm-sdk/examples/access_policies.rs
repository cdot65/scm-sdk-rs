//! Offline example: explicit control-plane key client, versioned writes, and faithful responses.
use bytes::Bytes;
use futures_util::future::BoxFuture;
use http::{HeaderMap, StatusCode};
use scm_sdk::{
    Client, Endpoint, Result, Secret,
    airs::gateway::access_policies::*,
    auth::Authentication,
    transport::{Transport, TransportRequest, TransportResponse},
};
use std::sync::Arc;
struct Offline;
impl Transport for Offline {
    fn send<'a>(&'a self, _: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async {
            Ok(TransportResponse::from_bytes(
                StatusCode::OK,
                HeaderMap::new(),
                Bytes::from_static(b"{}"),
            ))
        })
    }
}
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // Replace the synthetic transport/key with your explicitly admitted compatible
    // control-plane deployment. Never reuse SCM OAuth or infer an inference URL.
    let core = Client::builder(Endpoint::parse("https://control.example/v1")?)
        .authentication(Authentication::runtime_key(Secret::new(
            "offline-fixture",
        )?)?)
        .transport(Arc::new(Offline))
        .build()?;
    let client = AccessPoliciesClient::new(core)?;
    let create = CreateAccessPolicy::new(
        "example",
        vec![AccessPolicySubject::new(
            AccessPolicySubjectKind::User,
            "example-user",
        )],
        vec![AccessPolicyTarget::provider("example-provider")],
    );
    let _created = client.create(&create, None).await?;
    let _page = client
        .list(&AccessPolicyListOptions::default(), None)
        .await?;
    let id = "550e8400-e29b-41d4-a716-446655440000";
    let policy = client.get(id, None).await?;
    assert_eq!(serde_json::to_value(policy).unwrap(), serde_json::json!({}));
    // In production, use the version actually returned by get; 409 is not retried.
    let update = UpdateAccessPolicy::new(1, create.subjects, create.targets);
    let _updated = client.update(id, &update, None).await?;
    let _deleted = client.delete(id, None).await?;
    Ok(())
}
