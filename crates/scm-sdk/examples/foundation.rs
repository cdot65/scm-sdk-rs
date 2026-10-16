//! Raw-client example. Product-specific interfaces follow in later phases.
//! Supply credentials explicitly; this example performs no network requests.
use scm_sdk::{Client, Endpoint, Secret, auth::AuthContext};

fn main() -> scm_sdk::Result<()> {
    let issuer = Endpoint::parse("https://auth.apps.paloaltonetworks.com/oauth2/access_token")?;
    let identity = AuthContext::new(issuer, "example-client", 1, "1234567890")?;
    let client = Client::builder(Endpoint::parse(
        "https://api.sase.paloaltonetworks.com/aisec",
    )?)
    .oauth_credentials(identity, Secret::new("synthetic-example-secret")?, false)
    .build()?;
    assert!(!client.session().is_cancelled());
    Ok(())
}
