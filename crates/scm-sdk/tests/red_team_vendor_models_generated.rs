// Generated vendor model probes; inputs are independently checked by AJV.
#![cfg(feature = "red-team")]
use scm_sdk::{
    airs::red_team::models::data_plane as vendor,
    validation::{self, Mode},
};
use serde_json::Value;
const CORPUS: &str =
    include_str!("../../../contracts/vendor/red-team-data-plane/0.93.0/model-fixtures.json");
#[test]
fn every_independent_vendor_model_fixture_matches_native_validation_and_typed_decode() {
    let corpus: Value = serde_json::from_str(CORPUS).unwrap();
    for row in corpus["cases"].as_array().unwrap() {
        let value = &row["value"];
        let schema = row["schema"].as_str().unwrap();
        let valid = row["valid"].as_bool().unwrap();
        let result = validation::parse(
            &format!("vendor:red-team-dp:0.93.0:{schema}"),
            value.clone(),
            Mode::Request,
        );
        assert_eq!(result.is_ok(), valid, "fixture {}", row["name"]);
        if !valid {
            continue;
        }
        assert_eq!(
            result.unwrap(),
            *value,
            "vendor normalization preserves caller JSON"
        );
        match schema {
            "AgenticProgressSchema" => {
                let model: vendor::AgenticProgress =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AgenticStageName" => {
                let model: vendor::AgenticStageName =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AgenticStageSchema" => {
                let model: vendor::AgenticStage =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AgenticStatus" => {
                let model: vendor::AgenticStatus =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AgenticSubStepSchema" => {
                let model: vendor::AgenticSubStep =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ApiEndpointType" => {
                let model: vendor::ApiEndpointType =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AsrCategorySchema" => {
                let model: vendor::AsrCategory =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AsrResponseSchema" => {
                let model: vendor::AsrResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AsrStatsSchema" => {
                let model: vendor::AsrStats =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AttackAudioSchema" => {
                let model: vendor::AttackAudio =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AttackDetailResponseSchema" => {
                let model: vendor::AttackDetailResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AttackFileSchema" => {
                let model: vendor::AttackFile =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AttackListResponseSchema" => {
                let model: vendor::AttackListResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AttackListSchema" => {
                let model: vendor::AttackList =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AttackMultiTurnDetailResponseSchema" => {
                let model: vendor::AttackMultiTurnDetailResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AttackMultiTurnOutputResponseSchema" => {
                let model: vendor::AttackMultiTurnOutputResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AttackOutputResponseSchema" => {
                let model: vendor::AttackOutputResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AttackStatus" => {
                let model: vendor::AttackStatus =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AttackType" => {
                let model: vendor::AttackType =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AttemptsUnit" => {
                let model: vendor::AttemptsUnit =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AudioFormat" => {
                let model: vendor::AudioFormat =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "AuthType" => {
                let model: vendor::AuthType =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "BackgroundJobStatus" => {
                let model: vendor::BackgroundJobStatus =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "BaseSubtype" => {
                let model: vendor::BaseSubtype =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "BrandSubCategory" => {
                let model: vendor::BrandSubCategory =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "Category" => {
                let model: vendor::Category =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "CategoryModel" => {
                let model: vendor::CategoryModel =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "CategoryReportSchema" => {
                let model: vendor::CategoryReport =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ClaraJobMetadata" => {
                let model: vendor::ClaraJobMetadata =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ComplianceReportSchema" => {
                let model: vendor::ComplianceReport =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ComplianceSubCategory" => {
                let model: vendor::ComplianceSubCategory =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ComplianceTechniqueSchema" => {
                let model: vendor::ComplianceTechnique =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ComplianceWithTechniqueModel" => {
                let model: vendor::ComplianceWithTechniqueModel =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "CountByNameSchema" => {
                let model: vendor::CountByName =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "CountedQuotaEnum" => {
                let model: vendor::CountedQuotaEnum =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "CustomAttackOutputResponseSchema" => {
                let model: vendor::CustomAttackOutputResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "CustomAttackReportResponse" => {
                let model: vendor::CustomAttackReportResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "CustomAttacksListResponse" => {
                let model: vendor::CustomAttacksListResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "CustomJobMetadata" => {
                let model: vendor::CustomJobMetadata =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "CustomTopicGuardrailsSchema" => {
                let model: vendor::CustomTopicGuardrails =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "DateRangeFilter" => {
                let model: vendor::DateRangeFilter =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "DynamicJobMetadata" => {
                let model: vendor::DynamicJobMetadata =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "DynamicJobReportSchema" => {
                let model: vendor::DynamicJobReport =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "DynamicJobReportStats" => {
                let model: vendor::DynamicJobReportStats =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "DynamicRemediationDetailSchema" => {
                let model: vendor::DynamicRemediationDetail =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "DynamicRemediationResponseSchema" => {
                let model: vendor::DynamicRemediationResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ErrorLogListResponseSchema" => {
                let model: vendor::ErrorLogListResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ErrorLogListSchema" => {
                let model: vendor::ErrorLogList =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ErrorSource" => {
                let model: vendor::ErrorSource =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ErrorType" => {
                let model: vendor::ErrorType =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "EtaState" => {
                let model: vendor::EtaState =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "FileFormat" => {
                let model: vendor::FileFormat =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "GoalCategory" => {
                let model: vendor::GoalCategory =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "GoalCategoryDetail" => {
                let model: vendor::GoalCategoryDetail =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "GoalCategoryListResponseSchema" => {
                let model: vendor::GoalCategoryListResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "GoalCategoryOptionSchema" => {
                let model: vendor::GoalCategoryOption =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "GoalListResponseSchema" => {
                let model: vendor::GoalListResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "GoalManipulationSubtype" => {
                let model: vendor::GoalManipulationSubtype =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "GoalResponseSchema" => {
                let model: vendor::GoalResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "GoalSchema" => {
                let model: vendor::Goal =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "GoalType" => {
                let model: vendor::GoalType =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "GoalTypeQueryParam" => {
                let model: vendor::GoalTypeQueryParam =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "GuardrailAction" => {
                let model: vendor::GuardrailAction =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "HTTPValidationError" => {
                let model: vendor::HTTPValidationError =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "JobAbortResponseSchema" => {
                let model: vendor::JobAbortResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "JobCreateRequestSchema" => {
                let model: vendor::JobCreateRequest =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "JobListResponseSchema" => {
                let model: vendor::JobListResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "JobResponseSchema" => {
                let model: vendor::JobResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "JobRuntimeMetricsSchema" => {
                let model: vendor::JobRuntimeMetrics =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "JobStatus" => {
                let model: vendor::JobStatus =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "JobStatusFilter" => {
                let model: vendor::JobStatusFilter =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "JobTimeRecord" => {
                let model: vendor::JobTimeRecord =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "JobType" => {
                let model: vendor::JobType =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "LanguageDropdownOptionSchema" => {
                let model: vendor::LanguageDropdownOption =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "LanguageOptionSchema" => {
                let model: vendor::LanguageOption =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "MaliciousCodeDetectionSchema" => {
                let model: vendor::MaliciousCodeDetection =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "MaliciousURLDetectionSchema" => {
                let model: vendor::MaliciousURLDetection =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "MemoryPoisoningSubtype" => {
                let model: vendor::MemoryPoisoningSubtype =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "Modality" => {
                let model: vendor::Modality =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "PaginationSchema" => {
                let model: vendor::Pagination =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "PolicyType" => {
                let model: vendor::PolicyType =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "PrerequisiteModel" => {
                let model: vendor::PrerequisiteModel =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "PrivilegeMisuseSubtype" => {
                let model: vendor::PrivilegeMisuseSubtype =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ProfilingStatus" => {
                let model: vendor::ProfilingStatus =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "PromptDetailResponse" => {
                let model: vendor::PromptDetailResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "PromptInjectionGuardrailSchema" => {
                let model: vendor::PromptInjectionGuardrail =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "PromptSetSummary" => {
                let model: vendor::PromptSetSummary =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "PromptSetsReportResponse" => {
                let model: vendor::PromptSetsReportResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "PropertyAssignment" => {
                let model: vendor::PropertyAssignment =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "PropertyStatistic" => {
                let model: vendor::PropertyStatistic =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "PropertyValueStatistic" => {
                let model: vendor::PropertyValueStatistic =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "QuotaDetails" => {
                let model: vendor::QuotaDetails =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "QuotaSummarySchema" => {
                let model: vendor::QuotaSummary =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "RegenerateResponse" => {
                let model: vendor::RegenerateResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "RemediationDetailSchema" => {
                let model: vendor::RemediationDetail =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "RemediationResponseSchema" => {
                let model: vendor::RemediationResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ReportDownloadSchema" => {
                let model: vendor::ReportDownload =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ReportState" => {
                let model: vendor::ReportState =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ReportStatusResponse" => {
                let model: vendor::ReportStatusResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ResponseMode" => {
                let model: vendor::ResponseMode =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "RiskLevelSchema" => {
                let model: vendor::RiskLevel =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "RiskRating" => {
                let model: vendor::RiskRating =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "RuntimeProfileUpdateRequestSchema" => {
                let model: vendor::RuntimeProfileUpdateRequest =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "RuntimeSecurityPolicySchema" => {
                let model: vendor::RuntimeSecurityPolicy =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "RuntimeSecurityProfileResponseSchema" => {
                let model: vendor::RuntimeSecurityProfileResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "SafetySubCategory" => {
                let model: vendor::SafetySubCategory =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ScanProgressBreakdownSchema" => {
                let model: vendor::ScanProgressBreakdown =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ScanRowStatus" => {
                let model: vendor::ScanRowStatus =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ScanStatisticsResponseSchema" => {
                let model: vendor::ScanStatisticsResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ScoreTrendResponseSchema" => {
                let model: vendor::ScoreTrendResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ScoreTrendSeriesSchema" => {
                let model: vendor::ScoreTrendSeries =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "SecuritySubCategory" => {
                let model: vendor::SecuritySubCategory =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "SensitiveDataProtectionSchema" => {
                let model: vendor::SensitiveDataProtection =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "SentimentRequestSchema" => {
                let model: vendor::SentimentRequest =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "SentimentResponseSchema" => {
                let model: vendor::SentimentResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "SeverityFilter" => {
                let model: vendor::SeverityFilter =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "SeverityReportSchema" => {
                let model: vendor::SeverityReport =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "SeverityStatsSchema" => {
                let model: vendor::SeverityStats =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "StandardProgressSchema" => {
                let model: vendor::StandardProgress =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "StaticJobMetadata" => {
                let model: vendor::StaticJobMetadata =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "StaticJobRemediationRecommendationSchema" => {
                let model: vendor::StaticJobRemediationRecommendation =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "StaticJobRemediationSchema" => {
                let model: vendor::StaticJobRemediation =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "StaticJobReportSchema" => {
                let model: vendor::StaticJobReport =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "StaticJobReportStats" => {
                let model: vendor::StaticJobReportStats =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "StatusQueryParam" => {
                let model: vendor::StatusQueryParam =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "StreamDetailResponseSchema" => {
                let model: vendor::StreamDetailResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "StreamIterationDataResponseSchema" => {
                let model: vendor::StreamIterationDataResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "StreamIterationDataSchema" => {
                let model: vendor::StreamIterationData =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "StreamListResponseSchema" => {
                let model: vendor::StreamListResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "StreamSchema" => {
                let model: vendor::Stream =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "StreamType" => {
                let model: vendor::StreamType =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "SubCategoryModel" => {
                let model: vendor::SubCategoryModel =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "SubCategoryStatsSchema" => {
                let model: vendor::SubCategoryStats =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "TargetAdditionalContext" => {
                let model: vendor::TargetAdditionalContext =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "TargetBackground" => {
                let model: vendor::TargetBackground =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "TargetConnectionType" => {
                let model: vendor::TargetConnectionType =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "TargetJobRequest" => {
                let model: vendor::TargetJobRequest =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "TargetMetadata" => {
                let model: vendor::TargetMetadata =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "TargetReferenceSchema" => {
                let model: vendor::TargetReference =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "TargetStatus" => {
                let model: vendor::TargetStatus =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "TargetType" => {
                let model: vendor::TargetType =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "TechniqueModel" => {
                let model: vendor::TechniqueModel =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "TenantLanguagesResponseSchema" => {
                let model: vendor::TenantLanguagesResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ThreatDetails" => {
                let model: vendor::ThreatDetails =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ThreatOverrideItem" => {
                let model: vendor::ThreatOverrideItem =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ThreatOverrideRequest" => {
                let model: vendor::ThreatOverrideRequest =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ThreatOverrideResponse" => {
                let model: vendor::ThreatOverrideResponse =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ToolChainingSubtype" => {
                let model: vendor::ToolChainingSubtype =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ToolMisuseSubtype" => {
                let model: vendor::ToolMisuseSubtype =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ToxicContentGuardrailSchema" => {
                let model: vendor::ToxicContentGuardrail =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "TranscriptSource" => {
                let model: vendor::TranscriptSource =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            "ValidationError" => {
                let model: vendor::ValidationError =
                    serde_json::from_value(value.clone()).expect("typed vendor decode");
                assert_eq!(
                    serde_json::to_value(&model).unwrap(),
                    *value,
                    "typed model preserves wire values"
                );
                assert!(!format!("{model:?}").contains("example"));
            }
            _ => panic!("unbound fixture schema"),
        }
    }
}
