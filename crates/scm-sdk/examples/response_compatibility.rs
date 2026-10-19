//! Offline response compatibility check using only the published crate's public API.
//! No credentials, network access, repository fixtures, or async runtime are needed.
use scm_sdk::{
    airs::red_team::models::{AttackListResponse, AttackOutput, ErrorLogListResponse},
    validation::{self, Mode},
};
use serde_json::{Value, json};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sent: Value = serde_json::from_str(
        r#"{
    "pagination": {
        "total_items": 0
    },
    "data": [
        {
            "uuid": "550e8400-e29b-41d4-a716-446655440000",
            "tsg_id": "example",
            "job_id": "job-1",
            "target_id": "550e8400-e29b-41d4-a716-446655440000",
            "prompt": "example",
            "status": "FUTURE_STATUS",
            "marked_safe": false,
            "extra_info": {},
            "threat": false,
            "attack_type": "NORMAL",
            "multi_turn": false,
            "asr": 0,
            "version": 0,
            "error": false,
            "prompt_mapping_id": "map-1",
            "prompt_id": "p-1",
            "category": "FUTURE_CATEGORY",
            "sub_category": "FUTURE_SUB_CATEGORY",
            "severity": "example",
            "category_display_name": "example",
            "sub_category_display_name": "example",
            "attack_modality": "TEXT",
            "future_field": {
                "nullable": null,
                "nested": [
                    "preserved",
                    1
                ]
            }
        }
    ]
}"#,
    )?;
    for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
        let parsed = validation::parse(
            "vendor:red-team-dp:0.93.0:list_attacks_v1_report_static__job_id__list_attacks_get:response",
            sent.clone(),
            mode,
        )?;
        let typed = AttackListResponse::parse(parsed.clone(), mode)?;
        assert_eq!(serde_json::to_value(typed)?, parsed);
        assert_eq!(parsed, sent);
    }
    // These three structural restrictions are also enforced by frozen TypeScript.
    for change in 0..3 {
        let mut invalid = sent.clone();
        let row = invalid["data"][0].as_object_mut().expect("object fixture");
        match change {
            0 => {
                row.remove("category_display_name");
            }
            1 => {
                row.insert("tsg_id".into(), json!(1));
            }
            _ => {
                row.insert("sub_category".into(), Value::Null);
            }
        }
        for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
            assert!(validation::parse("vendor:red-team-dp:0.93.0:list_attacks_v1_report_static__job_id__list_attacks_get:response", invalid.clone(), mode).is_err());
            assert!(AttackListResponse::parse(invalid.clone(), mode).is_err());
        }
    }
    assert!(validation::parse("vendor:red-team-dp:0.93.0:list_attacks_v1_report_static__job_id__list_attacks_get:response", sent, Mode::Request).is_err());
    let sent: Value = serde_json::from_str(
        r#"{
    "pagination": {
        "total_items": 0
    },
    "data": [
        {
            "job_id": "job-1",
            "target_id": "550e8400-e29b-41d4-a716-446655440000",
            "target_version": 0,
            "attack_id": "550e8400-e29b-41d4-a716-446655440000",
            "error_type": "PROFILING",
            "error_source": "NEW_SOURCE",
            "error_message": "example",
            "target_object": {},
            "extra_info": {},
            "version": 0,
            "created_at": "2026-01-05 00:00:00",
            "updated_at": "2026-01-05T00:00:00.123456",
            "future_field": {
                "nullable": null,
                "nested": [
                    "preserved",
                    1
                ]
            }
        }
    ]
}"#,
    )?;
    for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
        let parsed = validation::parse(
            "vendor:red-team-dp:0.93.0:list_error_logs_for_target_profile_v1_error_log_target_profile__target_id__get:response",
            sent.clone(),
            mode,
        )?;
        let typed = ErrorLogListResponse::parse(parsed.clone(), mode)?;
        assert_eq!(serde_json::to_value(typed)?, parsed);
        assert_eq!(parsed, sent);
    }
    assert!(validation::parse("vendor:red-team-dp:0.93.0:list_error_logs_for_target_profile_v1_error_log_target_profile__target_id__get:response", sent, Mode::Request).is_err());
    // u64::MAX is exactly representable by serde_json in both its default and
    // arbitrary_precision configurations. The SDK must preserve that input.
    let sent = json!({
        "uuid": "output-1", "tsg_id": "example", "attack_id": "attack-1",
        "job_id": "job-1", "target_id": "target-1", "output": "synthetic",
        "response_audio": 17,
        "judge_reasoning": {"future_number": u64::MAX, "nullable": null},
        "future_number": u64::MAX
    });
    for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
        let typed = AttackOutput::parse(sent.clone(), mode)?;
        assert!(typed.response_audio.is_unset());
        assert!(typed.judge_reasoning.is_unset());
        assert_eq!(serde_json::to_value(typed)?, sent);
    }
    println!(
        "G25 response formats, unknown values and typed round-trips pass; requests remain strict."
    );
    Ok(())
}
