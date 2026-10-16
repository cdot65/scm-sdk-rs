//! Explicit Gateway credential planes. Construction performs no network requests.
#[cfg(all(feature = "gateway", feature = "reqwest-transport"))]
fn main() -> scm_sdk::Result<()> {
    use scm_sdk::{
        Client, Endpoint, OriginPolicy, Secret,
        airs::gateway::{AIGatewayClient, AIGatewayModelPricingClient, RuntimeClient},
        auth::{AuthContext, Authentication},
    };
    use std::sync::Arc;

    let issuer = Endpoint::parse("https://auth.apps.paloaltonetworks.com/oauth2/access_token")?;
    let data = Endpoint::parse("https://api.strata.paloaltonetworks.com/ai_gw/v2")?;
    let admin = Endpoint::parse("https://api.strata.paloaltonetworks.com/ai_gw/admin/v2")?;
    let iam = Endpoint::parse("https://api.strata.paloaltonetworks.com/iam/v1")?;
    let policy = OriginPolicy::new([issuer.clone(), data.clone(), admin.clone(), iam.clone()]);
    let identity = AuthContext::new(issuer, "example-client", 1, "1234567890")?;
    let control = Client::builder(data)
        .destination_policy(Arc::new(policy))
        .oauth_credentials(identity, Secret::new("synthetic-client-secret")?, true)
        .build()?;

    // Applications supply their own admitted runtime deployment and runtime key.
    let runtime = RuntimeClient::new(
        Client::builder(Endpoint::parse("https://runtime.example.com/v1")?)
            .authentication(Authentication::runtime_key(Secret::new(
                "synthetic-runtime-key",
            )?)?)
            .build()?,
    )?;
    let gateway = AIGatewayClient::new(control, admin, iam)?.with_runtime(runtime);
    let _inference = gateway.inference()?;
    let _workspaces = gateway.workspaces();

    // Pricing receives its own credential-free client, with no tenant identity.
    let _pricing = AIGatewayModelPricingClient::new(
        Client::builder(Endpoint::parse("https://pricing.example.com")?).build()?,
    )?;
    Ok(())
}

#[cfg(not(all(feature = "gateway", feature = "reqwest-transport")))]
fn main() {}
