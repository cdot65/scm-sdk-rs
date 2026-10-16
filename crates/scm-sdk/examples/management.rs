#[cfg(all(feature = "management", feature = "reqwest-transport"))]
#[tokio::main(flavor = "current_thread")]
async fn main() -> scm_sdk::Result<()> {
    use scm_sdk::{
        Client, Endpoint, Error, ErrorKind, Secret,
        airs::management::{ManagementClient, ProfileListAllOptions},
        auth::AuthContext,
    };
    // Environment selection belongs to this application and is explicit.
    let read = |name: &str| std::env::var(name).map_err(|_| Error::new(ErrorKind::Configuration));
    let identity = AuthContext::new(
        Endpoint::parse("https://auth.apps.paloaltonetworks.com/oauth2/access_token")?,
        read("PANW_MGMT_CLIENT_ID")?,
        0,
        read("PANW_MGMT_TSG_ID")?,
    )?;
    let client = Client::builder(Endpoint::parse(
        "https://api.apps.paloaltonetworks.com/aisec",
    )?)
    .oauth_credentials(
        identity,
        Secret::new(read("PANW_MGMT_CLIENT_SECRET")?)?,
        false,
    )
    .build()?;
    let management = ManagementClient::new(client)?;
    let profiles = management
        .profiles()
        .list_all(ProfileListAllOptions::default(), None)
        .await
        .map_err(|failure| failure.into_error())?;
    println!(
        "profiles={} complete={}",
        profiles.items.len(),
        profiles.is_complete()
    );
    Ok(())
}
#[cfg(not(all(feature = "management", feature = "reqwest-transport")))]
fn main() {}
