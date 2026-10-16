#[cfg(all(
    feature = "model-security",
    feature = "red-team",
    feature = "reqwest-transport"
))]
#[tokio::main(flavor = "current_thread")]
async fn main() -> scm_sdk::Result<()> {
    use scm_sdk::{
        Client, Endpoint, Error, ErrorKind, OriginPolicy, Secret,
        airs::{
            model_security::{ModelSecurityClient, ModelSecurityScanListOptions},
            red_team::{RedTeamClient, RedTeamScanListOptions},
        },
        auth::AuthContext,
    };
    use std::sync::Arc;
    // This application explicitly selects credentials and service endpoints.
    let read = |name: &str| std::env::var(name).map_err(|_| Error::new(ErrorKind::Configuration));
    let issuer = Endpoint::parse("https://auth.apps.paloaltonetworks.com/oauth2/access_token")?;
    let model_data = Endpoint::parse(&read("PANW_MODEL_SEC_DATA_ENDPOINT")?)?;
    let model_management = Endpoint::parse(&read("PANW_MODEL_SEC_MGMT_ENDPOINT")?)?;
    let red_data = Endpoint::parse(&read("PANW_RED_TEAM_DATA_ENDPOINT")?)?;
    let red_management = Endpoint::parse(&read("PANW_RED_TEAM_MGMT_ENDPOINT")?)?;
    let broker = Endpoint::parse(&read("PANW_RED_TEAM_NETWORK_BROKER_ENDPOINT")?)?;
    let context = AuthContext::new(
        issuer.clone(),
        read("PANW_MGMT_CLIENT_ID")?,
        0,
        read("PANW_MGMT_TSG_ID")?,
    )?;
    let policy = OriginPolicy::new([
        issuer,
        model_data.clone(),
        model_management.clone(),
        red_data.clone(),
        red_management.clone(),
        broker.clone(),
    ]);
    let core = Client::builder(model_data.clone())
        .destination_policy(Arc::new(policy))
        .oauth_credentials(
            context,
            Secret::new(read("PANW_MGMT_CLIENT_SECRET")?)?,
            false,
        )
        .build()?;
    let model = ModelSecurityClient::new(core.clone(), model_data, model_management)?;
    let red = RedTeamClient::new(core.with_endpoint(red_data)?, red_management, broker)?;
    let mut model_query = ModelSecurityScanListOptions::default();
    model_query.limit = Some(1);
    let mut red_query = RedTeamScanListOptions::default();
    red_query.limit = Some(1);
    let model_page = model.scans().list(&model_query, None).await?;
    let red_page = red.scans().list(&red_query, None).await?;
    println!(
        "model_scans={} red_team_scans={}",
        model_page.scans.len(),
        red_page.data.len()
    );
    Ok(())
}
#[cfg(not(all(
    feature = "model-security",
    feature = "red-team",
    feature = "reqwest-transport"
)))]
fn main() {}
