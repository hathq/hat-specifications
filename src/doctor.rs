use crate::model::validation;
use crate::{
    ACTION_RESULT_SCHEMA, ACTION_STATUS_SCHEMA, AUTOMATION_POLICY_SCHEMA, BINDING_SCHEMA,
    CHECKPOINT_SCHEMA, COMMITMENT_SCHEMA, COMMUNICATION_CAPABILITY_SCHEMA,
    COMPLETE_DELETION_PREVIEW_SCHEMA, COMPLETE_DELETION_RECEIPT_SCHEMA,
    COMPOSITION_APPROVAL_SCHEMA, COMPOSITION_PROPOSAL_SCHEMA, CONTROL_GRANT_SCHEMA,
    DELEGATION_SCHEMA, DESIRED_STATE_SCHEMA, EXECUTION_LOCATION_SCHEMA,
    EXECUTION_LOCATION_SET_SCHEMA, EXECUTION_SCHEMA, FAILURE_ENVELOPE_SCHEMA,
    FEDERATION_RECEIPT_SCHEMA, GOAL_SCHEMA, INFORMATION_COORDINATE_SCHEMA,
    INTELLIGENCE_ARTIFACT_SCHEMA, INVOCATION_CONTROL_SCHEMA, INVOCATION_SCHEMA,
    LOCAL_REGISTRY_LEDGER_SCHEMA, MANIFEST_SCHEMA, MATTER_CONTRIBUTION_SCHEMA, MATTER_SCHEMA,
    PACKAGE_SCHEMA, PERSON_ASSERTION_SCHEMA, PLACEMENT_SELECTION_SCHEMA, POSITION_RECORD_SCHEMA,
    PROFILE_SCHEMA, PROJECTION_EVENT_SCHEMA, PROJECTION_JOURNAL_SCHEMA, PROJECTION_SCHEMA,
    REPRESENTATIVE_ROLE_ARCHETYPE_SCHEMA, REPRESENTATIVE_ROLE_INSTANCE_SCHEMA, RESULT_SCHEMA,
    RETENTION_POLICY_SCHEMA, SEMANTIC_CATALOG_SCHEMA, SEMANTIC_CLAUSE_SCHEMA,
    SEMANTIC_COMPREHENSION_SCHEMA, SEMANTIC_EXCHANGE_SCHEMA, SEMANTIC_MAPPING_CONTRACT_SCHEMA,
    SITUATION_CONTRIBUTION_SCHEMA, SUBJECT_REFERENCE_SCHEMA, TASK_SCHEMA, VERIFICATION_SCHEMA,
    Validation, fit, parse_manifest, parse_package, parse_profile, validate_manifest,
    validate_package,
};

const MANIFEST_SCHEMA_DOCUMENT: &str = include_str!("../schemas/hat-manifest-v1.schema.json");
const PROFILE_SCHEMA_DOCUMENT: &str = include_str!("../schemas/hat-profile-v1.schema.json");
const RESULT_SCHEMA_DOCUMENT: &str = include_str!("../schemas/result-v1.schema.json");
const PACKAGE_SCHEMA_DOCUMENT: &str = include_str!("../schemas/hat-package-v2.schema.json");
const COMMUNICATION_CAPABILITY_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-communication-capability-v1.schema.json");
const BINDING_SCHEMA_DOCUMENT: &str = include_str!("../schemas/hat-binding-v2.schema.json");
const INVOCATION_SCHEMA_DOCUMENT: &str = include_str!("../schemas/hat-invocation-v4.schema.json");
const ACTION_STATUS_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-action-status-v2.schema.json");
const ACTION_RESULT_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-action-result-v1.schema.json");
const FAILURE_ENVELOPE_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-failure-envelope-v1.schema.json");
const INVOCATION_CONTROL_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-invocation-control-v2.schema.json");
const PROJECTION_EVENT_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-projection-event-v1.schema.json");
const PROJECTION_JOURNAL_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-projection-journal-v1.schema.json");
const SITUATION_CONTRIBUTION_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-situation-contribution-v1.schema.json");
const PROJECTION_SCHEMA_DOCUMENT: &str = include_str!("../schemas/hat-projection-v2.schema.json");
const CHECKPOINT_SCHEMA_DOCUMENT: &str = include_str!("../schemas/hat-checkpoint-v2.schema.json");
const SUBJECT_REFERENCE_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-subject-reference-v1.schema.json");
const POSITION_RECORD_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-position-record-v1.schema.json");
const CONTROL_GRANT_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-control-grant-v1.schema.json");
const INFORMATION_COORDINATE_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-information-coordinate-v1.schema.json");
const PERSON_ASSERTION_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-person-assertion-v2.schema.json");
const RETENTION_POLICY_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-retention-policy-v2.schema.json");
const COMPLETE_DELETION_PREVIEW_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-complete-deletion-preview-v2.schema.json");
const COMPLETE_DELETION_RECEIPT_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-complete-deletion-receipt-v1.schema.json");
const COMPOSITION_PROPOSAL_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-composition-proposal-v1.schema.json");
const COMPOSITION_APPROVAL_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-composition-approval-v1.schema.json");
const EXECUTION_LOCATION_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-execution-location-v1.schema.json");
const EXECUTION_LOCATION_SET_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-execution-location-set-v1.schema.json");
const PLACEMENT_SELECTION_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-placement-selection-v2.schema.json");
const FEDERATION_RECEIPT_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-federation-execution-receipt-v1.schema.json");
const SEMANTIC_CATALOG_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-semantic-catalog-v1.schema.json");
const SEMANTIC_CLAUSE_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-semantic-clause-v1.schema.json");
const SEMANTIC_EXCHANGE_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-semantic-exchange-v1.schema.json");
const SEMANTIC_COMPREHENSION_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-semantic-comprehension-report-v1.schema.json");
const LOCAL_REGISTRY_LEDGER_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-local-registry-ledger-v1.schema.json");
const SEMANTIC_MAPPING_CONTRACT_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-semantic-mapping-contract-v1.schema.json");
const DESIRED_STATE_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-desired-state-v1.schema.json");
const GOAL_SCHEMA_DOCUMENT: &str = include_str!("../schemas/hat-goal-v1.schema.json");
const COMMITMENT_SCHEMA_DOCUMENT: &str = include_str!("../schemas/hat-commitment-v1.schema.json");
const TASK_SCHEMA_DOCUMENT: &str = include_str!("../schemas/hat-task-v2.schema.json");
const DELEGATION_SCHEMA_DOCUMENT: &str = include_str!("../schemas/hat-delegation-v1.schema.json");
const EXECUTION_SCHEMA_DOCUMENT: &str = include_str!("../schemas/hat-execution-v1.schema.json");
const VERIFICATION_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-verification-v1.schema.json");
const INTELLIGENCE_ARTIFACT_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-intelligence-artifact-v1.schema.json");
const REPRESENTATIVE_ROLE_ARCHETYPE_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-representative-role-archetype-v1.schema.json");
const REPRESENTATIVE_ROLE_INSTANCE_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-representative-role-instance-v1.schema.json");
const MATTER_SCHEMA_DOCUMENT: &str = include_str!("../schemas/hat-matter-v1.schema.json");
const MATTER_CONTRIBUTION_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-matter-contribution-v1.schema.json");
const AUTOMATION_POLICY_SCHEMA_DOCUMENT: &str =
    include_str!("../schemas/hat-automation-policy-v1.schema.json");
const EXAMPLE_MANIFEST: &str = include_str!("../examples/source-curator.hat.toml");
const EXAMPLE_PROFILE: &str = include_str!("../examples/editor.profile.toml");
const EXAMPLE_PACKAGE: &str = include_str!("../examples/source-curator.package.json");

/// Runs deterministic self-checks against bundled contracts and examples.
#[must_use]
pub fn doctor() -> Validation {
    let mut findings = Vec::new();
    check_contract_schemas(&mut findings);
    match parse_manifest(EXAMPLE_MANIFEST) {
        Ok(manifest) => findings.extend(validate_manifest(&manifest).findings),
        Err(error) => findings.push(format!("example manifest: {error}")),
    }
    match (
        parse_manifest(EXAMPLE_MANIFEST),
        parse_profile(EXAMPLE_PROFILE),
    ) {
        (Ok(manifest), Ok(profile)) => findings.extend(fit(&manifest, &profile).findings),
        (Err(error), _) | (_, Err(error)) => {
            findings.push(format!("example fitting: {error}"));
        }
    }
    match parse_package(EXAMPLE_PACKAGE) {
        Ok(package) => findings.extend(validate_package(&package).findings),
        Err(error) => findings.push(format!("example package: {error}")),
    }
    validation(findings)
}

fn check_contract_schemas(findings: &mut Vec<String>) {
    check_schema(
        MANIFEST_SCHEMA_DOCUMENT,
        MANIFEST_SCHEMA,
        "manifest schema",
        findings,
    );
    check_runtime_schemas(findings);
    check_schema(
        COMMUNICATION_CAPABILITY_SCHEMA_DOCUMENT,
        COMMUNICATION_CAPABILITY_SCHEMA,
        "communication capability schema",
        findings,
    );
    check_schema(
        PROFILE_SCHEMA_DOCUMENT,
        PROFILE_SCHEMA,
        "profile schema",
        findings,
    );
    check_schema(
        RESULT_SCHEMA_DOCUMENT,
        RESULT_SCHEMA,
        "result schema",
        findings,
    );
    check_person_schemas(findings);
    check_semantic_schemas(findings);
}

fn check_runtime_schemas(findings: &mut Vec<String>) {
    for (source, id, label) in [
        (PACKAGE_SCHEMA_DOCUMENT, PACKAGE_SCHEMA, "package schema"),
        (BINDING_SCHEMA_DOCUMENT, BINDING_SCHEMA, "binding schema"),
        (
            INVOCATION_SCHEMA_DOCUMENT,
            INVOCATION_SCHEMA,
            "invocation schema",
        ),
        (
            ACTION_STATUS_SCHEMA_DOCUMENT,
            ACTION_STATUS_SCHEMA,
            "action status schema",
        ),
        (
            ACTION_RESULT_SCHEMA_DOCUMENT,
            ACTION_RESULT_SCHEMA,
            "action result schema",
        ),
        (
            FAILURE_ENVELOPE_SCHEMA_DOCUMENT,
            FAILURE_ENVELOPE_SCHEMA,
            "failure envelope schema",
        ),
        (
            INVOCATION_CONTROL_SCHEMA_DOCUMENT,
            INVOCATION_CONTROL_SCHEMA,
            "invocation control schema",
        ),
        (
            PROJECTION_EVENT_SCHEMA_DOCUMENT,
            PROJECTION_EVENT_SCHEMA,
            "projection event schema",
        ),
        (
            PROJECTION_JOURNAL_SCHEMA_DOCUMENT,
            PROJECTION_JOURNAL_SCHEMA,
            "projection journal schema",
        ),
        (
            SITUATION_CONTRIBUTION_SCHEMA_DOCUMENT,
            SITUATION_CONTRIBUTION_SCHEMA,
            "situation contribution schema",
        ),
        (
            PROJECTION_SCHEMA_DOCUMENT,
            PROJECTION_SCHEMA,
            "projection schema",
        ),
        (
            CHECKPOINT_SCHEMA_DOCUMENT,
            CHECKPOINT_SCHEMA,
            "checkpoint schema",
        ),
        (
            EXECUTION_LOCATION_SCHEMA_DOCUMENT,
            EXECUTION_LOCATION_SCHEMA,
            "execution location schema",
        ),
        (
            EXECUTION_LOCATION_SET_SCHEMA_DOCUMENT,
            EXECUTION_LOCATION_SET_SCHEMA,
            "execution location set schema",
        ),
        (
            PLACEMENT_SELECTION_SCHEMA_DOCUMENT,
            PLACEMENT_SELECTION_SCHEMA,
            "placement selection schema",
        ),
        (
            FEDERATION_RECEIPT_SCHEMA_DOCUMENT,
            FEDERATION_RECEIPT_SCHEMA,
            "federation receipt schema",
        ),
    ] {
        check_schema(source, id, label, findings);
    }
}

fn check_semantic_schemas(findings: &mut Vec<String>) {
    for (source, id, label) in [
        (
            DESIRED_STATE_SCHEMA_DOCUMENT,
            DESIRED_STATE_SCHEMA,
            "desired state schema",
        ),
        (GOAL_SCHEMA_DOCUMENT, GOAL_SCHEMA, "goal schema"),
        (
            COMMITMENT_SCHEMA_DOCUMENT,
            COMMITMENT_SCHEMA,
            "commitment schema",
        ),
        (TASK_SCHEMA_DOCUMENT, TASK_SCHEMA, "task schema"),
        (
            DELEGATION_SCHEMA_DOCUMENT,
            DELEGATION_SCHEMA,
            "delegation schema",
        ),
        (
            EXECUTION_SCHEMA_DOCUMENT,
            EXECUTION_SCHEMA,
            "execution schema",
        ),
        (
            VERIFICATION_SCHEMA_DOCUMENT,
            VERIFICATION_SCHEMA,
            "verification schema",
        ),
        (
            INTELLIGENCE_ARTIFACT_SCHEMA_DOCUMENT,
            INTELLIGENCE_ARTIFACT_SCHEMA,
            "intelligence artifact schema",
        ),
        (
            REPRESENTATIVE_ROLE_ARCHETYPE_SCHEMA_DOCUMENT,
            REPRESENTATIVE_ROLE_ARCHETYPE_SCHEMA,
            "representative role archetype schema",
        ),
        (
            REPRESENTATIVE_ROLE_INSTANCE_SCHEMA_DOCUMENT,
            REPRESENTATIVE_ROLE_INSTANCE_SCHEMA,
            "representative role instance schema",
        ),
        (MATTER_SCHEMA_DOCUMENT, MATTER_SCHEMA, "matter schema"),
        (
            MATTER_CONTRIBUTION_SCHEMA_DOCUMENT,
            MATTER_CONTRIBUTION_SCHEMA,
            "matter contribution schema",
        ),
        (
            AUTOMATION_POLICY_SCHEMA_DOCUMENT,
            AUTOMATION_POLICY_SCHEMA,
            "automation policy schema",
        ),
        (
            SEMANTIC_CATALOG_SCHEMA_DOCUMENT,
            SEMANTIC_CATALOG_SCHEMA,
            "semantic catalog schema",
        ),
        (
            SEMANTIC_CLAUSE_SCHEMA_DOCUMENT,
            SEMANTIC_CLAUSE_SCHEMA,
            "semantic clause schema",
        ),
        (
            SEMANTIC_EXCHANGE_SCHEMA_DOCUMENT,
            SEMANTIC_EXCHANGE_SCHEMA,
            "semantic exchange schema",
        ),
        (
            SEMANTIC_COMPREHENSION_SCHEMA_DOCUMENT,
            SEMANTIC_COMPREHENSION_SCHEMA,
            "semantic comprehension schema",
        ),
        (
            LOCAL_REGISTRY_LEDGER_SCHEMA_DOCUMENT,
            LOCAL_REGISTRY_LEDGER_SCHEMA,
            "local registry ledger schema",
        ),
        (
            SEMANTIC_MAPPING_CONTRACT_SCHEMA_DOCUMENT,
            SEMANTIC_MAPPING_CONTRACT_SCHEMA,
            "semantic mapping contract schema",
        ),
    ] {
        check_schema(source, id, label, findings);
    }
}

fn check_person_schemas(findings: &mut Vec<String>) {
    for (source, id, label) in [
        (
            SUBJECT_REFERENCE_SCHEMA_DOCUMENT,
            SUBJECT_REFERENCE_SCHEMA,
            "subject reference schema",
        ),
        (
            POSITION_RECORD_SCHEMA_DOCUMENT,
            POSITION_RECORD_SCHEMA,
            "position record schema",
        ),
        (
            CONTROL_GRANT_SCHEMA_DOCUMENT,
            CONTROL_GRANT_SCHEMA,
            "control grant schema",
        ),
        (
            INFORMATION_COORDINATE_SCHEMA_DOCUMENT,
            INFORMATION_COORDINATE_SCHEMA,
            "information coordinate schema",
        ),
        (
            PERSON_ASSERTION_SCHEMA_DOCUMENT,
            PERSON_ASSERTION_SCHEMA,
            "person assertion schema",
        ),
        (
            RETENTION_POLICY_SCHEMA_DOCUMENT,
            RETENTION_POLICY_SCHEMA,
            "retention policy schema",
        ),
        (
            COMPLETE_DELETION_PREVIEW_SCHEMA_DOCUMENT,
            COMPLETE_DELETION_PREVIEW_SCHEMA,
            "complete deletion preview schema",
        ),
        (
            COMPLETE_DELETION_RECEIPT_SCHEMA_DOCUMENT,
            COMPLETE_DELETION_RECEIPT_SCHEMA,
            "complete deletion receipt schema",
        ),
        (
            COMPOSITION_PROPOSAL_SCHEMA_DOCUMENT,
            COMPOSITION_PROPOSAL_SCHEMA,
            "composition proposal schema",
        ),
        (
            COMPOSITION_APPROVAL_SCHEMA_DOCUMENT,
            COMPOSITION_APPROVAL_SCHEMA,
            "composition approval schema",
        ),
    ] {
        check_schema(source, id, label, findings);
    }
}

fn check_schema(source: &str, expected_id: &str, label: &str, findings: &mut Vec<String>) {
    match serde_json::from_str::<serde_json::Value>(source) {
        Ok(schema)
            if schema.get("$id").and_then(serde_json::Value::as_str) == Some(expected_id) => {}
        Ok(_) => findings.push(format!("{label} has the wrong $id")),
        Err(error) => findings.push(format!("{label} is invalid JSON: {error}")),
    }
}
