use hat_specifications::{
    ACTION_RESULT_SCHEMA, ACTION_STATUS_SCHEMA, ActionReference, ContextPartition, HatActionResult,
    HatActionStatus, HatBinding, HatInvocation, HatInvocationControl, HatInvocationControlKind,
    HatInvocationOutcome, HatInvocationPhase, INVOCATION_CONTROL_SCHEMA, INVOCATION_SCHEMA,
    parse_package, validate_action_result, validate_action_status,
    validate_invocation_against_package, validate_invocation_control,
};

const DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const PACKAGE: &str = include_str!("../examples/source-curator.package.json");

fn reference(schema_id: &str) -> ActionReference {
    ActionReference {
        owner_id: "zixcel".to_owned(),
        reference: "artifact-1".to_owned(),
        schema_id: schema_id.to_owned(),
        digest_sha256: DIGEST.to_owned(),
    }
}

fn invocation() -> HatInvocation {
    let package = parse_package(PACKAGE).expect("package");
    let operation = &package.operations[0];
    HatInvocation {
        schema: INVOCATION_SCHEMA.to_owned(),
        invocation_id: "invoke-1".to_owned(),
        binding: HatBinding {
            schema: hat_specifications::BINDING_SCHEMA.to_owned(),
            package_id: package.package_id,
            package_digest_sha256: DIGEST.to_owned(),
            catalog_digest_sha256: DIGEST.to_owned(),
            fitting_digest_sha256: DIGEST.to_owned(),
            subject_ref: "subject:test".to_owned(),
            scope_ref: "scope:personal".to_owned(),
        },
        context_partition: ContextPartition {
            context_partition_id: "context-partition-1".to_owned(),
            revision: 7,
            policy_digest_sha256: DIGEST.to_owned(),
        },
        operation_id: operation.id.clone(),
        expected_projection_revision: 4,
        idempotency_key: "request-1".to_owned(),
        input: reference(&operation.input_schema),
        effective_grant: reference("hathq://hat/effective-grant/v1"),
        placement: reference("hathq://hat/placement/v1"),
    }
}

#[test]
fn invocation_is_bound_to_installed_package_operation_and_input_schema() {
    let package = parse_package(PACKAGE).expect("package");
    let invocation = invocation();
    assert!(validate_invocation_against_package(&invocation, &package).valid);

    let mut substituted = invocation;
    substituted.input.schema_id = "hathq://hat/substituted/v1".to_owned();
    assert!(!validate_invocation_against_package(&substituted, &package).valid);
}

#[test]
fn invocation_rejects_omitted_references_and_unknown_inline_payloads() {
    let value = serde_json::to_value(invocation()).expect("serialize");
    for field in ["input", "effective_grant", "placement"] {
        let mut missing = value.clone();
        missing.as_object_mut().expect("object").remove(field);
        assert!(
            serde_json::from_value::<HatInvocation>(missing).is_err(),
            "{field}"
        );
    }
    let mut inline = value;
    inline
        .as_object_mut()
        .expect("object")
        .insert("raw_input".to_owned(), serde_json::json!({"secret": true}));
    assert!(serde_json::from_value::<HatInvocation>(inline).is_err());
}

#[test]
fn lifecycle_status_result_and_control_are_closed_and_revision_bound() {
    let status = HatActionStatus {
        schema: ACTION_STATUS_SCHEMA.to_owned(),
        invocation_id: "invoke-1".to_owned(),
        context_partition_id: "context-partition-1".to_owned(),
        state_revision: 2,
        phase: HatInvocationPhase::Running,
        reason_id: None,
    };
    assert!(validate_action_status(&status).valid);

    let result = HatActionResult {
        schema: ACTION_RESULT_SCHEMA.to_owned(),
        invocation_id: "invoke-1".to_owned(),
        operation_id: "hathq://vocabulary/action/review-source/v1".to_owned(),
        state_revision: 3,
        projection_revision: 5,
        outcome: HatInvocationOutcome::Completed,
        output: Some(reference("hathq://hat/source-review-output/v1")),
        reason_id: None,
        evidence_refs: Vec::new(),
    };
    assert!(validate_action_result(&result).valid);

    let mut invalid = result;
    invalid.outcome = HatInvocationOutcome::Failed;
    assert!(!validate_action_result(&invalid).valid);

    for kind in [
        HatInvocationControlKind::ReadStatus,
        HatInvocationControlKind::Cancel,
        HatInvocationControlKind::Recover,
    ] {
        let control = HatInvocationControl {
            schema: INVOCATION_CONTROL_SCHEMA.to_owned(),
            request_id: format!("request-{kind:?}").to_ascii_lowercase(),
            invocation_id: "invoke-1".to_owned(),
            context_partition_id: "context-partition-1".to_owned(),
            expected_state_revision: 2,
            idempotency_key: "control-1".to_owned(),
            kind,
        };
        assert!(validate_invocation_control(&control).valid);
    }
}
