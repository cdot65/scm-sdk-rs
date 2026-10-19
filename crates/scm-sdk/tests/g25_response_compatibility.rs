//! Generated root inventory with independent AJV/Zod expectations.
#![cfg(feature = "red-team")]
use scm_sdk::{
    airs::red_team::models::*,
    validation::{self, Mode},
};
use serde_json::Value;
use sha2::{Digest, Sha256};
fn load(name: &str) -> Value {
    serde_json::from_slice(
        &std::fs::read(
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../contracts/compatibility/g25/"
            )
            .to_owned()
                + name,
        )
        .unwrap(),
    )
    .unwrap()
}
fn assign(doc: &mut Value, path: &[Value], value: Value, omit: bool) {
    if path.is_empty() {
        *doc = value;
        return;
    }
    let mut at = doc;
    for key in &path[..path.len() - 1] {
        at = if let Some(i) = key.as_u64() {
            &mut at[i as usize]
        } else {
            &mut at[key.as_str().unwrap()]
        };
    }
    let key = path.last().unwrap();
    if let Some(i) = key.as_u64() {
        at[i as usize] = value;
    } else if omit {
        at.as_object_mut().unwrap().remove(key.as_str().unwrap());
    } else {
        at[key.as_str().unwrap()] = value;
    }
}
fn preserve(input: &Value, output: &Value) {
    match (input, output) {
        (Value::Object(a), Value::Object(b)) => {
            for (k, v) in a {
                preserve(v, b.get(k).expect("response property preserved"));
            }
        }
        (Value::Array(a), Value::Array(b)) => {
            assert_eq!(a.len(), b.len());
            for (a, b) in a.iter().zip(b) {
                preserve(a, b);
            }
        }
        _ => assert_eq!(input, output, "wire value must not be coerced"),
    }
}
#[test]
fn every_vendor_response_root_matches_independent_compatibility_outcomes() {
    let corpus = load("cases.json");
    let oracle = load("outcomes.json");
    let bytes = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../contracts/compatibility/g25/cases.json"
    ))
    .unwrap();
    assert_eq!(
        oracle["casesSha256"].as_str().unwrap(),
        format!("{:x}", Sha256::digest(bytes))
    );

    let roots = corpus["roots"].as_array().unwrap();
    let cases = corpus["cases"].as_array().unwrap();
    let outcomes = oracle["outcomes"].as_array().unwrap();
    assert_eq!(roots.len(), 46);
    assert_eq!(cases.len(), outcomes.len());
    for (index, (case, expected)) in cases.iter().zip(outcomes).enumerate() {
        let root = &roots[case["root"].as_u64().unwrap() as usize];
        let schema = root["root"].as_str().unwrap();
        let mut value = root["base"].clone();
        for choice in case["choices"].as_array().unwrap() {
            assign(
                &mut value,
                choice[0].as_array().unwrap(),
                choice[1].clone(),
                false,
            );
        }
        // Every base and selected union branch must be a strict-valid starting
        // document, independently of the mutation's expected acceptance.
        assert!(
            validation::parse(
                root["strictRoot"].as_str().unwrap(),
                value.clone(),
                Mode::Request
            )
            .is_ok(),
            "strict starting witness {index} {schema}"
        );
        assign(
            &mut value,
            case["path"].as_array().unwrap(),
            case["value"].clone(),
            case["omit"].as_bool().unwrap(),
        );
        let request = validation::parse(
            root["strictRoot"].as_str().unwrap(),
            value.clone(),
            Mode::Request,
        );
        assert_eq!(
            request.is_ok(),
            expected["strict"].as_bool().unwrap(),
            "strict case {index} {schema} {}",
            case["path"]
        );
        // The public operation root also keeps strict vendor validation in Request mode.
        assert_eq!(
            validation::parse(schema, value.clone(), Mode::Request).is_ok(),
            request.is_ok()
        );
        for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
            let result = validation::parse(schema, value.clone(), mode);
            assert_eq!(
                result.is_ok(),
                expected["response"].as_bool().unwrap(),
                "response case {index} {schema} {} {:?}",
                case["path"],
                mode
            );
            if let Ok(result) = result {
                preserve(&value, &result);
                let roundtrip = roundtrip(schema, result.clone(), mode);
                assert_eq!(roundtrip, result, "typed roundtrip case {index} {schema}");
            }
        }
    }
}
// BEGIN GENERATED OPERATION TYPES
fn roundtrip(schema: &str, value: Value, mode: Mode) -> Value {
    match schema {
        "vendor:red-team-dp:0.93.0:create_job_v1_scan_post:response" => {
            serde_json::to_value(JobResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:list_jobs_v1_scan_get:response" => {
            serde_json::to_value(JobListResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_job_v1_scan__job_id__get:response" => {
            serde_json::to_value(JobResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:abort_job_v1_scan__job_id__abort_post:response" => {
            serde_json::to_value(JobAbortResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:set_runtime_profile_v1_scan__job_id__runtime_profile_put:response" =>
        {
            /* Decoder-exempt: operation vendor contract differs from JobResponse::SCHEMA; raw serde only checks the legacy output shape here. */
            serde_json::to_value(serde_json::from_value::<JobResponse>(value).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_all_categories_v1_categories_get:response" => Value::Array(
            value
                .as_array()
                .unwrap()
                .iter()
                .cloned()
                .map(|item| {
                    serde_json::to_value(CategoryModel::parse(item, mode).unwrap()).unwrap()
                })
                .collect(),
        ),
        "vendor:red-team-dp:0.93.0:get_scan_metadata_v1_scan_scan_metadata_get:response" => {
            /* Decoder-exempt: validated unconstrained JSON has no model parse API. */
            value
        }
        "vendor:red-team-dp:0.93.0:list_attacks_v1_report_static__job_id__list_attacks_get:response" => {
            serde_json::to_value(AttackListResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_attack_detail_v1_report_static__job_id__attack__attack_id__get:response" => {
            serde_json::to_value(AttackDetailResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_attack_report_v1_report_static__job_id__report_get:response" => {
            serde_json::to_value(StaticJobReport::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_static_runtime_security_policy_config_v1_report_static__job_id__runtime_policy_config_get:response" => {
            serde_json::to_value(RuntimeSecurityProfileResponse::parse(value, mode).unwrap())
                .unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_static_asr_breakdown_v1_report_static__job_id__asr_get:response" => {
            serde_json::to_value(AsrResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_static_remediations_v1_report_static__job_id__remediation_get:response" => {
            serde_json::to_value(RemediationResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_dynamic_job_report_v1_report_dynamic__job_id__report_get:response" => {
            serde_json::to_value(DynamicJobReport::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_dynamic_runtime_security_policy_config_v1_report_dynamic__job_id__runtime_policy_config_get:response" => {
            serde_json::to_value(RuntimeSecurityProfileResponse::parse(value, mode).unwrap())
                .unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_dynamic_asr_breakdown_v1_report_dynamic__job_id__asr_get:response" => {
            serde_json::to_value(AsrResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_dynamic_remediations_v1_report_dynamic__job_id__remediation_get:response" => {
            serde_json::to_value(RemediationResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:list_dynamic_job_goals_v1_report_dynamic__job_id__list_goals_get:response" => {
            serde_json::to_value(GoalListResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:list_dynamic_job_streams_v1_report_dynamic__job_id__goal__goal_id__list_streams_get:response" => {
            serde_json::to_value(StreamListResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_stream_detail_v1_report_dynamic_stream__stream_id__get:response" => {
            serde_json::to_value(StreamDetailResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:download_report_v1_report__job_id__download_get:response" => {
            /* Decoder-exempt: validated unconstrained JSON has no model parse API. */
            value
        }
        "vendor:red-team-dp:0.93.0:generate_partial_job_report_v1_report__job_id__generate_partial_report_post:response" =>
        {
            /* Decoder-exempt: validated unconstrained JSON has no model parse API. */
            value
        }
        "vendor:red-team-dp:0.93.0:regenerate_report_v1_report__job_id__regenerate_post:response" => {
            serde_json::to_value(RegenerateResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_report_status_v1_report__job_id__status_get:response" => {
            serde_json::to_value(ReportStatusResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_attack_multi_turn_detail_v1_report_static__job_id__attack_multi_turn__attack_id__get:response" => {
            serde_json::to_value(AttackMultiTurnDetailResponse::parse(value, mode).unwrap())
                .unwrap()
        }
        "vendor:red-team-dp:0.93.0:threat_override_static_attack_v1_report_static__job_id__attack__attack_id__override_post:response" => {
            serde_json::to_value(ThreatOverrideResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:threat_override_static_multi_turn_v1_report_static__job_id__attack_multi_turn__attack_id__override_post:response" => {
            serde_json::to_value(ThreatOverrideResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:threat_override_dynamic_stream_v1_report_dynamic__job_id__stream__stream_id__override_post:response" => {
            serde_json::to_value(ThreatOverrideResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:post_quota_summary_v1_metering_quota_post:response" => {
            serde_json::to_value(QuotaSummary::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_custom_attack_report_v1_custom_attacks_report__job_id__get:response" => {
            serde_json::to_value(CustomAttackReportResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_prompt_sets_v1_custom_attacks_report__job_id__prompt_sets_get:response" => {
            serde_json::to_value(PromptSetsReportResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_prompts_by_set_v1_custom_attacks_report__job_id__prompt_set__prompt_set_id__prompts_get:response" => {
            Value::Array(
                value
                    .as_array()
                    .unwrap()
                    .iter()
                    .cloned()
                    .map(|item| {
                        serde_json::to_value(PromptDetailResponse::parse(item, mode).unwrap())
                            .unwrap()
                    })
                    .collect(),
            )
        }
        "vendor:red-team-dp:0.93.0:get_prompt_detail_v1_custom_attacks_report__job_id__prompt__prompt_id__get:response" => {
            serde_json::to_value(PromptDetailResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:list_custom_attacks_v1_custom_attacks_job__job_id__list_custom_attacks_get:response" => {
            serde_json::to_value(CustomAttacksListResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_attack_outputs_v1_custom_attacks_job__job_id__attack__attack_id__list_outputs_get:response" => {
            Value::Array(
                value
                    .as_array()
                    .unwrap()
                    .iter()
                    .cloned()
                    .map(|item| {
                        serde_json::to_value(CustomAttackOutput::parse(item, mode).unwrap())
                            .unwrap()
                    })
                    .collect(),
            )
        }
        "vendor:red-team-dp:0.93.0:get_property_stats_v1_custom_attacks_job__job_id__property_stats_get:response" => {
            Value::Array(
                value
                    .as_array()
                    .unwrap()
                    .iter()
                    .cloned()
                    .map(|item| {
                        serde_json::to_value(PropertyStatisticResult::parse(item, mode).unwrap())
                            .unwrap()
                    })
                    .collect(),
            )
        }
        "vendor:red-team-dp:0.93.0:threat_override_custom_attack_v1_custom_attacks_job__job_id__attack__attack_id__override_post:response" => {
            serde_json::to_value(ThreatOverrideResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:list_error_logs_for_job_v1_error_log_job__job_id__get:response" => {
            serde_json::to_value(ErrorLogListResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:list_error_logs_for_target_profile_v1_error_log_target_profile__target_id__get:response" => {
            serde_json::to_value(ErrorLogListResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_goal_categories_v1_goal_categories__target_type__get:response" => {
            serde_json::to_value(GoalCategoryListResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_languages_v1_languages_get:response" => {
            serde_json::to_value(TenantLanguagesResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:download_report_v2_report__job_id__download_get:response" => {
            serde_json::to_value(ReportDownload::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_scan_statistics_v1_dashboard_scan_statistics_get:response" => {
            serde_json::to_value(ScanStatisticsResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_score_trend_v1_dashboard_score_trend_get:response" => {
            serde_json::to_value(ScoreTrendResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:create_or_update_sentiment_v1_sentiment_post:response" => {
            serde_json::to_value(SentimentResponse::parse(value, mode).unwrap()).unwrap()
        }
        "vendor:red-team-dp:0.93.0:get_job_sentiment_v1_sentiment__job_id__get:response" => {
            serde_json::to_value(SentimentResponse::parse(value, mode).unwrap()).unwrap()
        }
        // END GENERATED OPERATION TYPES
        _ => panic!("unbound response root {schema}"),
    }
}

#[test]
fn every_string_format_node_is_tolerant_in_both_response_modes_across_the_sdk() {
    let graph: Value = serde_json::from_slice(
        &std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/validation/schema_graph.json"
        ))
        .unwrap(),
    )
    .unwrap();
    let formats = [
        "uuid", "rfc3339", "datetime", "date", "duration", "email", "url", "regex",
    ];
    let mut covered = std::collections::BTreeSet::new();
    let mut request_rejections = 0;
    for (sid, node) in graph["nodes"].as_object().unwrap() {
        if node["kind"] != "ZodString" {
            continue;
        }
        let checks = node["checks"].as_array().unwrap();
        if !checks
            .iter()
            .any(|c| formats.contains(&c["kind"].as_str().unwrap()))
        {
            continue;
        }
        let minimum = checks
            .iter()
            .filter(|c| c["kind"] == "min")
            .map(|c| c["value"].as_u64().unwrap() as usize)
            .max()
            .unwrap_or(1);
        let maximum = checks
            .iter()
            .filter(|c| c["kind"] == "max")
            .map(|c| c["value"].as_u64().unwrap() as usize)
            .min()
            .unwrap_or(1000);
        let input = Value::String("x".repeat(minimum.max(1).min(maximum)));
        for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
            assert_eq!(
                validation::parse(sid, input.clone(), mode).unwrap(),
                input,
                "format node {sid}"
            );
            assert!(
                validation::parse(sid, Value::Bool(false), mode).is_err(),
                "string type remains strict {sid}"
            );
        }
        if validation::parse(sid, input, Mode::Request).is_err() {
            request_rejections += 1;
        }
        for c in checks {
            if let Some(k) = c["kind"].as_str()
                && formats.contains(&k)
            {
                covered.insert(k.to_owned());
            }
        }
    }
    assert_eq!(covered, formats.into_iter().map(str::to_owned).collect());
    assert!(request_rejections > 100);
}

fn assert_projection_policy(row: &Value, input: Value, mode: Mode) {
    let name = row["rustType"].as_str().unwrap();
    let schema = row["schema"].as_str().unwrap();
    let expected = validation::parse(schema, input.clone(), mode).unwrap();
    let actual = projection_roundtrip(name, input.clone(), mode);
    preserve(&input, &actual);
    assert_eq!(actual, expected, "projection policy {name} {mode:?}");
    if mode == Mode::ResponseWithoutDefaults {
        assert_eq!(actual, input, "projection without defaults {name}");
    }
}

#[test]
fn every_vendor_added_projection_preserves_absence_null_and_unrepresentable_values() {
    let projections = load("projections.json");
    let rows = projections.as_array().unwrap();
    assert_eq!(rows.len(), 55);
    for row in rows {
        let field = row["field"].as_str().unwrap();
        let base = row["base"].clone();
        for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
            assert_projection_policy(row, base.clone(), mode);
            for raw in [
                Value::Null,
                serde_json::json!(false),
                serde_json::json!(1),
                serde_json::json!("future"),
                serde_json::json!({"future":[null,1,"exact"]}),
                serde_json::json!([null,{"future":true}]),
            ] {
                let mut input = base.clone();
                input[field] = raw;
                assert_projection_policy(row, input, mode);
            }
        }
    }
}
// BEGIN GENERATED PROJECTION TYPES
fn projection_roundtrip(name: &str, value: Value, mode: Mode) -> Value {
    match name {
        "AttackDetailResponse" => {
            serde_json::to_value(AttackDetailResponse::parse(value, mode).unwrap()).unwrap()
        }
        "AttackMultiTurnDetailResponse" => {
            serde_json::to_value(AttackMultiTurnDetailResponse::parse(value, mode).unwrap())
                .unwrap()
        }
        "AttackMultiTurnOutput" => {
            serde_json::to_value(AttackMultiTurnOutput::parse(value, mode).unwrap()).unwrap()
        }
        "AttackOutput" => serde_json::to_value(AttackOutput::parse(value, mode).unwrap()).unwrap(),
        "CustomAttackOutput" => {
            serde_json::to_value(CustomAttackOutput::parse(value, mode).unwrap()).unwrap()
        }
        "CustomJobMetadata" => {
            serde_json::to_value(CustomJobMetadata::parse(value, mode).unwrap()).unwrap()
        }
        "DynamicJobMetadata" => {
            serde_json::to_value(DynamicJobMetadata::parse(value, mode).unwrap()).unwrap()
        }
        "Goal" => serde_json::to_value(Goal::parse(value, mode).unwrap()).unwrap(),
        "JobCreateRequestJobMetadataVariant1" => {
            serde_json::to_value(JobCreateRequestJobMetadataVariant1::parse(value, mode).unwrap())
                .unwrap()
        }
        "JobCreateRequestJobMetadataVariant2" => {
            serde_json::to_value(JobCreateRequestJobMetadataVariant2::parse(value, mode).unwrap())
                .unwrap()
        }
        "JobCreateRequestJobMetadataVariant3" => {
            serde_json::to_value(JobCreateRequestJobMetadataVariant3::parse(value, mode).unwrap())
                .unwrap()
        }
        "JobResponse" => serde_json::to_value(JobResponse::parse(value, mode).unwrap()).unwrap(),
        "LanguageOption" => {
            serde_json::to_value(LanguageOption::parse(value, mode).unwrap()).unwrap()
        }
        "PromptDetailResponse" => {
            serde_json::to_value(PromptDetailResponse::parse(value, mode).unwrap()).unwrap()
        }
        "PromptSetSummary" => {
            serde_json::to_value(PromptSetSummary::parse(value, mode).unwrap()).unwrap()
        }
        "RemediationDetail" => {
            serde_json::to_value(RemediationDetail::parse(value, mode).unwrap()).unwrap()
        }
        "ScoreTrendResponse" => {
            serde_json::to_value(ScoreTrendResponse::parse(value, mode).unwrap()).unwrap()
        }
        "StaticJobMetadata" => {
            serde_json::to_value(StaticJobMetadata::parse(value, mode).unwrap()).unwrap()
        }
        "StreamDetailResponse" => {
            serde_json::to_value(StreamDetailResponse::parse(value, mode).unwrap()).unwrap()
        }
        "StreamDetailResponseGoalVariant2" => {
            serde_json::to_value(StreamDetailResponseGoalVariant2::parse(value, mode).unwrap())
                .unwrap()
        }
        "StreamIterationData" => {
            serde_json::to_value(StreamIterationData::parse(value, mode).unwrap()).unwrap()
        }
        "SubCategoryModel" => {
            serde_json::to_value(SubCategoryModel::parse(value, mode).unwrap()).unwrap()
        }
        "SubCategoryStats" => {
            serde_json::to_value(SubCategoryStats::parse(value, mode).unwrap()).unwrap()
        }
        "TargetMetadata" => {
            serde_json::to_value(TargetMetadata::parse(value, mode).unwrap()).unwrap()
        }
        "ValidationError" => {
            serde_json::to_value(ValidationError::parse(value, mode).unwrap()).unwrap()
        }
        // END GENERATED PROJECTION TYPES
        _ => panic!("unbound projection {name}"),
    }
}

#[test]
fn typed_replacement_wins_projection_fallback_and_baseline_collisions_still_fail() {
    let rows = load("projections.json");
    let row = rows
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["rustType"] == "AttackOutput")
        .unwrap();
    let mut input = row["base"].clone();
    input["judge_reasoning"] = serde_json::json!({"future": true});
    let mut model: AttackOutput = serde_json::from_value(input.clone()).unwrap();
    assert!(model.judge_reasoning.is_unset());
    assert_eq!(serde_json::to_value(&model).unwrap(), input);
    model.judge_reasoning = scm_sdk::data::Optional::Value("replacement".into());
    input["judge_reasoning"] = serde_json::json!("replacement");
    assert_eq!(serde_json::to_value(&model).unwrap(), input);
    model.extensions.insert(
        "uuid".into(),
        serde_json::from_value(serde_json::json!("collision")).unwrap(),
    );
    assert!(serde_json::to_value(&model).is_err());
}

#[test]
fn projection_models_preserve_large_numbers_through_the_sdk_json_boundary() {
    let rows = load("projections.json");
    let row = rows
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["rustType"] == "AttackOutput")
        .unwrap();
    let mut input = row["base"].clone();
    let number: Value = serde_json::from_str("340282366920938463463374607431768211455").unwrap();
    input["future_number"] = number.clone();
    input["judge_reasoning"] = serde_json::json!({"nested": number});
    for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
        let model = AttackOutput::parse(input.clone(), mode).unwrap();
        assert!(model.judge_reasoning.is_unset());
        assert_eq!(serde_json::to_value(model).unwrap(), input);
    }
}

// BEGIN GENERATED PROJECTION STATES
fn projection_state(name: &str, field: &str, value: Value, mode: Mode) -> (&'static str, bool) {
    match (name, field) {
        ("AttackDetailResponse", "threat_details") => {
            let model: AttackDetailResponse = AttackDetailResponse::parse(value, mode).unwrap();
            let state = match model.threat_details {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("threat_details").is_some())
        }
        ("AttackOutput", "response_audio") => {
            let model: AttackOutput = AttackOutput::parse(value, mode).unwrap();
            let state = match model.response_audio {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("response_audio").is_some())
        }
        ("AttackOutput", "transcript") => {
            let model: AttackOutput = AttackOutput::parse(value, mode).unwrap();
            let state = match model.transcript {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("transcript").is_some())
        }
        ("AttackOutput", "transcript_source") => {
            let model: AttackOutput = AttackOutput::parse(value, mode).unwrap();
            let state = match model.transcript_source {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("transcript_source").is_some())
        }
        ("AttackOutput", "threat_details") => {
            let model: AttackOutput = AttackOutput::parse(value, mode).unwrap();
            let state = match model.threat_details {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("threat_details").is_some())
        }
        ("AttackOutput", "judge_reasoning") => {
            let model: AttackOutput = AttackOutput::parse(value, mode).unwrap();
            let state = match model.judge_reasoning {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("judge_reasoning").is_some())
        }
        ("AttackDetailResponse", "audio") => {
            let model: AttackDetailResponse = AttackDetailResponse::parse(value, mode).unwrap();
            let state = match model.audio {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("audio").is_some())
        }
        ("AttackDetailResponse", "audio_technique") => {
            let model: AttackDetailResponse = AttackDetailResponse::parse(value, mode).unwrap();
            let state = match model.audio_technique {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("audio_technique").is_some())
        }
        ("AttackMultiTurnDetailResponse", "threat_details") => {
            let model: AttackMultiTurnDetailResponse =
                AttackMultiTurnDetailResponse::parse(value, mode).unwrap();
            let state = match model.threat_details {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("threat_details").is_some())
        }
        ("AttackMultiTurnDetailResponse", "audio") => {
            let model: AttackMultiTurnDetailResponse =
                AttackMultiTurnDetailResponse::parse(value, mode).unwrap();
            let state = match model.audio {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("audio").is_some())
        }
        ("AttackMultiTurnDetailResponse", "audio_technique") => {
            let model: AttackMultiTurnDetailResponse =
                AttackMultiTurnDetailResponse::parse(value, mode).unwrap();
            let state = match model.audio_technique {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("audio_technique").is_some())
        }
        ("AttackMultiTurnOutput", "response_audio") => {
            let model: AttackMultiTurnOutput = AttackMultiTurnOutput::parse(value, mode).unwrap();
            let state = match model.response_audio {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("response_audio").is_some())
        }
        ("AttackMultiTurnOutput", "transcript") => {
            let model: AttackMultiTurnOutput = AttackMultiTurnOutput::parse(value, mode).unwrap();
            let state = match model.transcript {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("transcript").is_some())
        }
        ("AttackMultiTurnOutput", "transcript_source") => {
            let model: AttackMultiTurnOutput = AttackMultiTurnOutput::parse(value, mode).unwrap();
            let state = match model.transcript_source {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("transcript_source").is_some())
        }
        ("AttackMultiTurnOutput", "threat_details") => {
            let model: AttackMultiTurnOutput = AttackMultiTurnOutput::parse(value, mode).unwrap();
            let state = match model.threat_details {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("threat_details").is_some())
        }
        ("AttackMultiTurnOutput", "judge_reasoning") => {
            let model: AttackMultiTurnOutput = AttackMultiTurnOutput::parse(value, mode).unwrap();
            let state = match model.judge_reasoning {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("judge_reasoning").is_some())
        }
        ("SubCategoryModel", "audio_supported") => {
            let model: SubCategoryModel = SubCategoryModel::parse(value, mode).unwrap();
            let state = match model.audio_supported {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("audio_supported").is_some())
        }
        ("SubCategoryStats", "audio_supported") => {
            let model: SubCategoryStats = SubCategoryStats::parse(value, mode).unwrap();
            let state = match model.audio_supported {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("audio_supported").is_some())
        }
        ("CustomAttackOutput", "judge_reasoning") => {
            let model: CustomAttackOutput = CustomAttackOutput::parse(value, mode).unwrap();
            let state = match model.judge_reasoning {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("judge_reasoning").is_some())
        }
        ("CustomAttackOutput", "threat_details") => {
            let model: CustomAttackOutput = CustomAttackOutput::parse(value, mode).unwrap();
            let state = match model.threat_details {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("threat_details").is_some())
        }
        ("PromptSetSummary", "errored_attacks") => {
            let model: PromptSetSummary = PromptSetSummary::parse(value, mode).unwrap();
            let state = match model.errored_attacks {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("errored_attacks").is_some())
        }
        ("PromptSetSummary", "has_errors") => {
            let model: PromptSetSummary = PromptSetSummary::parse(value, mode).unwrap();
            let state = match model.has_errors {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("has_errors").is_some())
        }
        ("PromptDetailResponse", "threat_details") => {
            let model: PromptDetailResponse = PromptDetailResponse::parse(value, mode).unwrap();
            let state = match model.threat_details {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("threat_details").is_some())
        }
        ("CustomJobMetadata", "runtime_profile_id") => {
            let model: CustomJobMetadata = CustomJobMetadata::parse(value, mode).unwrap();
            let state = match model.runtime_profile_id {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("runtime_profile_id").is_some())
        }
        ("DynamicJobMetadata", "runtime_profile_id") => {
            let model: DynamicJobMetadata = DynamicJobMetadata::parse(value, mode).unwrap();
            let state = match model.runtime_profile_id {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("runtime_profile_id").is_some())
        }
        ("DynamicJobMetadata", "multi_turn_enabled") => {
            let model: DynamicJobMetadata = DynamicJobMetadata::parse(value, mode).unwrap();
            let state = match model.multi_turn_enabled {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("multi_turn_enabled").is_some())
        }
        ("Goal", "goal_subtype") => {
            let model: Goal = Goal::parse(value, mode).unwrap();
            let state = match model.goal_subtype {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("goal_subtype").is_some())
        }
        ("Goal", "goal_english") => {
            let model: Goal = Goal::parse(value, mode).unwrap();
            let state = match model.goal_english {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("goal_english").is_some())
        }
        ("ValidationError", "input") => {
            let model: ValidationError = ValidationError::parse(value, mode).unwrap();
            let state = match model.input {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("input").is_some())
        }
        ("ValidationError", "ctx") => {
            let model: ValidationError = ValidationError::parse(value, mode).unwrap();
            let state = match model.ctx {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("ctx").is_some())
        }
        ("JobCreateRequestJobMetadataVariant1", "runtime_profile_id") => {
            let model: JobCreateRequestJobMetadataVariant1 =
                JobCreateRequestJobMetadataVariant1::parse(value, mode).unwrap();
            let state = match model.runtime_profile_id {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("runtime_profile_id").is_some())
        }
        ("JobCreateRequestJobMetadataVariant1", "include_audio_attacks") => {
            let model: JobCreateRequestJobMetadataVariant1 =
                JobCreateRequestJobMetadataVariant1::parse(value, mode).unwrap();
            let state = match model.include_audio_attacks {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (
                state,
                model.extensions.get("include_audio_attacks").is_some(),
            )
        }
        ("JobCreateRequestJobMetadataVariant1", "include_text_attacks") => {
            let model: JobCreateRequestJobMetadataVariant1 =
                JobCreateRequestJobMetadataVariant1::parse(value, mode).unwrap();
            let state = match model.include_text_attacks {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (
                state,
                model.extensions.get("include_text_attacks").is_some(),
            )
        }
        ("JobCreateRequestJobMetadataVariant2", "runtime_profile_id") => {
            let model: JobCreateRequestJobMetadataVariant2 =
                JobCreateRequestJobMetadataVariant2::parse(value, mode).unwrap();
            let state = match model.runtime_profile_id {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("runtime_profile_id").is_some())
        }
        ("JobCreateRequestJobMetadataVariant2", "multi_turn_enabled") => {
            let model: JobCreateRequestJobMetadataVariant2 =
                JobCreateRequestJobMetadataVariant2::parse(value, mode).unwrap();
            let state = match model.multi_turn_enabled {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("multi_turn_enabled").is_some())
        }
        ("JobCreateRequestJobMetadataVariant3", "runtime_profile_id") => {
            let model: JobCreateRequestJobMetadataVariant3 =
                JobCreateRequestJobMetadataVariant3::parse(value, mode).unwrap();
            let state = match model.runtime_profile_id {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("runtime_profile_id").is_some())
        }
        ("TargetMetadata", "audio_probe_passed") => {
            let model: TargetMetadata = TargetMetadata::parse(value, mode).unwrap();
            let state = match model.audio_probe_passed {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("audio_probe_passed").is_some())
        }
        ("TargetMetadata", "audio_probe_error") => {
            let model: TargetMetadata = TargetMetadata::parse(value, mode).unwrap();
            let state = match model.audio_probe_error {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("audio_probe_error").is_some())
        }
        ("TargetMetadata", "audio_multi_turn_probe_passed") => {
            let model: TargetMetadata = TargetMetadata::parse(value, mode).unwrap();
            let state = match model.audio_multi_turn_probe_passed {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (
                state,
                model
                    .extensions
                    .get("audio_multi_turn_probe_passed")
                    .is_some(),
            )
        }
        ("TargetMetadata", "audio_multi_turn_probe_error") => {
            let model: TargetMetadata = TargetMetadata::parse(value, mode).unwrap();
            let state = match model.audio_multi_turn_probe_error {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (
                state,
                model
                    .extensions
                    .get("audio_multi_turn_probe_error")
                    .is_some(),
            )
        }
        ("TargetMetadata", "supports_audio") => {
            let model: TargetMetadata = TargetMetadata::parse(value, mode).unwrap();
            let state = match model.supports_audio {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("supports_audio").is_some())
        }
        ("TargetMetadata", "audio_format") => {
            let model: TargetMetadata = TargetMetadata::parse(value, mode).unwrap();
            let state = match model.audio_format {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("audio_format").is_some())
        }
        ("TargetMetadata", "supports_text") => {
            let model: TargetMetadata = TargetMetadata::parse(value, mode).unwrap();
            let state = match model.supports_text {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("supports_text").is_some())
        }
        ("StaticJobMetadata", "runtime_profile_id") => {
            let model: StaticJobMetadata = StaticJobMetadata::parse(value, mode).unwrap();
            let state = match model.runtime_profile_id {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("runtime_profile_id").is_some())
        }
        ("StaticJobMetadata", "include_audio_attacks") => {
            let model: StaticJobMetadata = StaticJobMetadata::parse(value, mode).unwrap();
            let state = match model.include_audio_attacks {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (
                state,
                model.extensions.get("include_audio_attacks").is_some(),
            )
        }
        ("StaticJobMetadata", "include_text_attacks") => {
            let model: StaticJobMetadata = StaticJobMetadata::parse(value, mode).unwrap();
            let state = match model.include_text_attacks {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (
                state,
                model.extensions.get("include_text_attacks").is_some(),
            )
        }
        ("JobResponse", "runtime_metrics") => {
            let model: JobResponse = JobResponse::parse(value, mode).unwrap();
            let state = match model.runtime_metrics {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("runtime_metrics").is_some())
        }
        ("JobResponse", "progress") => {
            let model: JobResponse = JobResponse::parse(value, mode).unwrap();
            let state = match model.progress {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("progress").is_some())
        }
        ("ScoreTrendResponse", "iso_labels") => {
            let model: ScoreTrendResponse = ScoreTrendResponse::parse(value, mode).unwrap();
            let state = match model.iso_labels {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("iso_labels").is_some())
        }
        ("StreamIterationData", "judge_explanation") => {
            let model: StreamIterationData = StreamIterationData::parse(value, mode).unwrap();
            let state = match model.judge_explanation {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("judge_explanation").is_some())
        }
        ("StreamDetailResponse", "threat_details") => {
            let model: StreamDetailResponse = StreamDetailResponse::parse(value, mode).unwrap();
            let state = match model.threat_details {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("threat_details").is_some())
        }
        ("StreamIterationData", "threat_details") => {
            let model: StreamIterationData = StreamIterationData::parse(value, mode).unwrap();
            let state = match model.threat_details {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("threat_details").is_some())
        }
        ("StreamDetailResponseGoalVariant2", "goal_subtype") => {
            let model: StreamDetailResponseGoalVariant2 =
                StreamDetailResponseGoalVariant2::parse(value, mode).unwrap();
            let state = match model.goal_subtype {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("goal_subtype").is_some())
        }
        ("LanguageOption", "country_code") => {
            let model: LanguageOption = LanguageOption::parse(value, mode).unwrap();
            let state = match model.country_code {
                scm_sdk::data::Patch::Unset => "unset",
                scm_sdk::data::Patch::Value(_) => "value",
                scm_sdk::data::Patch::Null => "null",
            };
            (state, model.extensions.get("country_code").is_some())
        }
        ("RemediationDetail", "categories") => {
            let model: RemediationDetail = RemediationDetail::parse(value, mode).unwrap();
            let state = match model.categories {
                scm_sdk::data::Optional::Unset => "unset",
                scm_sdk::data::Optional::Value(_) => "value",
            };
            (state, model.extensions.get("categories").is_some())
        }
        // END GENERATED PROJECTION STATES
        _ => panic!("unbound projection state {name}.{field}"),
    }
}
#[test]
fn every_vendor_projection_populates_typed_values_and_preserves_nullability() {
    let rows = load("projections.json");
    for row in rows.as_array().unwrap() {
        let name = row["rustType"].as_str().unwrap();
        let field = row["field"].as_str().unwrap();
        for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
            let mut input = row["base"].clone();
            assert_eq!(
                projection_state(name, field, input.clone(), mode),
                ("unset", false)
            );
            input[field] = row["conforming"].clone();
            assert_eq!(
                projection_state(name, field, input.clone(), mode),
                ("value", false),
                "{name}.{field} {mode:?}"
            );
            assert_projection_policy(row, input.clone(), mode);
            input[field] = Value::Null;
            let expected = if row["nullable"].as_bool().unwrap() {
                ("null", false)
            } else {
                ("unset", true)
            };
            assert_eq!(
                projection_state(name, field, input.clone(), mode),
                expected,
                "null {name}.{field} {mode:?}"
            );
            assert_projection_policy(row, input, mode);
        }
    }
}

#[test]
fn every_projection_model_preserves_large_unknown_numbers_through_public_parse() {
    let rows = load("projections.json");
    let number: Value = serde_json::from_str("340282366920938463463374607431768211455").unwrap();
    for row in rows.as_array().unwrap() {
        let mut input = row["base"].clone();
        input["future_number"] = number.clone();
        for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
            assert_projection_policy(row, input.clone(), mode);
        }
    }
}

#[test]
fn union_nested_projection_preserves_large_numbers_without_falling_back_to_unknown() {
    let rows = load("projections.json");
    let base = |name: &str| {
        rows.as_array()
            .unwrap()
            .iter()
            .find(|r| r["rustType"] == name)
            .unwrap()["base"]
            .clone()
    };
    let mut nested = base("StreamDetailResponseGoalVariant2");
    nested["future_number"] =
        serde_json::from_str("340282366920938463463374607431768211455").unwrap();
    let mut input = base("StreamDetailResponse");
    input["goal"] = nested;
    for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
        let expected =
            validation::parse(StreamDetailResponse::SCHEMA, input.clone(), mode).unwrap();
        let model = StreamDetailResponse::parse(input.clone(), mode).unwrap();
        assert!(matches!(
            &model.goal,
            scm_sdk::data::Patch::Value(StreamDetailResponseGoal::Variant2(_))
        ));
        let actual = serde_json::to_value(model).unwrap();
        preserve(&input, &actual);
        assert_eq!(actual, expected);
        if mode == Mode::ResponseWithoutDefaults {
            assert_eq!(actual, input);
        }
    }
}
