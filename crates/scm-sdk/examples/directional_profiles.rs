//! Offline TypeScript 0.35.0 directional-profile consumer; no service calls.
use scm_sdk::{
    airs::management::models::{
        AiSecurityProfile, ContentTypeConfiguration, ContentTypeConfigurations,
        CreateSecurityProfileRequest, ModelConfiguration, Policy, SecurityProfile,
    },
    data::Optional,
    validation::{Mode, parse},
};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let direction = ContentTypeConfiguration::builder().build_validated(Mode::Request)?;
    let directions = ContentTypeConfigurations::builder()
        .prompt(Optional::Value(direction))
        .build_validated(Mode::Request)?;
    let shared = ModelConfiguration::builder()
        .enable_full_conversation_inspection(Optional::Value(false))
        .build()?;
    let ai = AiSecurityProfile::builder()
        .content_type_mode(Optional::Value("per_content_type".into()))
        .content_type_configurations(Optional::Value(directions))
        .model_configuration(Optional::Value(shared))
        .build()?;
    let policy = Policy::builder()
        .ai_security_profiles(Optional::Value(vec![ai]))
        .build()?;
    let request = CreateSecurityProfileRequest::builder()
        .profile_name("example-directional".into())
        .policy(policy)
        .dlp_tenant_id(Optional::Value("100".into()))
        .build()?;
    let wire = serde_json::to_value(request)?;
    assert_eq!(
        parse(
            "typescript:0.35.0:CreateSecurityProfileRequestSchema",
            wire.clone(),
            Mode::Request
        )?,
        wire
    );
    assert_eq!(
        wire["policy"]["ai-security-profiles"][0]["content-type-configurations"],
        json!({"prompt":{}})
    );
    assert_eq!(
        wire["policy"]["ai-security-profiles"][0]["model-configuration"]["enable-full-conversation-inspection"],
        false
    );
    let unexpected = json!({"profile_name":"example-directional","dlp_tenant_id":null,"policy":{
        "ai-security-profiles":[{"content-type-mode":17,"content-type-configurations":{"prompt":null}}]}});
    for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
        assert_eq!(
            serde_json::to_value(SecurityProfile::parse(unexpected.clone(), mode)?)?,
            unexpected
        );
    }
    println!("Directional typed models and lossless response compatibility verified offline.");
    Ok(())
}
