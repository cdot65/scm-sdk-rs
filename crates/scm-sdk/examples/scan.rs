#[cfg(feature = "scan")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use scm_sdk::{
        Secret,
        airs::scan::{Content, Region, Scanner, models::AiProfile},
        data::Optional,
    };
    // The application explicitly chooses its credential and profile inputs.
    let key = Secret::new(std::env::var("AIRS_API_KEY")?)?;
    let profile_name = std::env::var("AIRS_PROFILE_NAME")?;
    let scanner = Scanner::for_region(Region::Us, Some(key), None)?;
    let profile = AiProfile::builder()
        .profile_name(Optional::Value(profile_name))
        .build_validated(scm_sdk::validation::Mode::Request)?;
    let content = Content::builder()
        .prompt("A harmless SDK connectivity check")
        .build()?;
    let response = scanner.sync_scan(&profile, &content, None).await?;
    // The application owns enforcement. Unfamiliar values require an explicit decision.
    let _action = &response.action;
    Ok(())
}
#[cfg(not(feature = "scan"))]
fn main() {}
