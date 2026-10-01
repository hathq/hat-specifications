use hat_specifications::{
    ACTION_RESULT_SCHEMA, FAILURE_ENVELOPE_SCHEMA, HatActionResult, HatFailureClass,
    HatFailureEnvelope, HatFailureRecovery, HatFailureResponsibility, HatInvocationOutcome,
    SemanticCatalog, SemanticTermKind, VocabularyIcon, parse_package, semantic_catalog_digest,
    semantic_term_definition_digest, validate_action_failure, validate_action_failure_with_catalog,
    validate_failure_envelope, validate_failure_envelope_with_catalog,
};
use std::collections::BTreeMap;

fn failure() -> HatFailureEnvelope {
    HatFailureEnvelope {
        schema: FAILURE_ENVELOPE_SCHEMA.to_owned(),
        failure_id: "failure-1".to_owned(),
        invocation_id: "invocation-1".to_owned(),
        component_id: "source-curator".to_owned(),
        operation_id: "hathq://vocabulary/action/review-source/v1".to_owned(),
        reason_id: "hathq://vocabulary/reason/source-unavailable/v1".to_owned(),
        class: HatFailureClass::Availability,
        recovery: HatFailureRecovery::ExternalChange,
        responsibility: HatFailureResponsibility::ExternalService,
        state_revision: 4,
        parameters: BTreeMap::from([("source-kind".to_owned(), "catalog".to_owned())]),
        next_action_id: None,
        evidence_refs: Vec::new(),
    }
}

fn catalog() -> SemanticCatalog {
    let mut catalog = parse_package(include_str!("../examples/source-curator.package.json"))
        .expect("package")
        .catalog;
    let mut reason = catalog.terms[0].clone();
    "hathq://vocabulary/reason/source-unavailable/v1".clone_into(&mut reason.reference.term_id);
    reason.reference.kind = SemanticTermKind::Reason;
    reason.reference.definition_digest_sha256.clear();
    reason.wire_schema = Some(FAILURE_ENVELOPE_SCHEMA.to_owned());
    reason.semantic_icon = VocabularyIcon::Reason;
    reason.reference.definition_digest_sha256 =
        semantic_term_definition_digest(&reason).expect("reason digest");
    catalog.terms.push(reason);
    catalog.identity.digest_sha256.clear();
    let digest = semantic_catalog_digest(&catalog).expect("catalog digest");
    catalog.identity.digest_sha256.clone_from(&digest);
    for term in &mut catalog.terms {
        term.reference.catalog_digest_sha256.clone_from(&digest);
    }
    for frame in &mut catalog.frames {
        frame.predicate.catalog_digest_sha256.clone_from(&digest);
    }
    catalog
}

#[test]
fn accepts_bounded_semantic_parameters_without_provider_error_text() {
    assert!(validate_failure_envelope(&failure()).valid);
    assert!(validate_failure_envelope_with_catalog(&failure(), &catalog()).valid);
    let value = serde_json::to_value(failure()).expect("failure");
    assert!(value.get("message").is_none());
    assert!(value.get("cause").is_none());
    assert!(value.get("stack").is_none());
}

#[test]
fn owner_action_requires_an_exact_action_and_owner_responsibility() {
    let mut value = failure();
    value.recovery = HatFailureRecovery::OwnerAction;
    assert!(!validate_failure_envelope(&value).valid);
    value.responsibility = HatFailureResponsibility::Owner;
    value.next_action_id = Some("hathq://vocabulary/action/review-source/v1".to_owned());
    assert!(validate_failure_envelope(&value).valid);
}

#[test]
fn rejects_unknown_fields_unbounded_parameters_and_non_reason_terms() {
    let mut raw = serde_json::to_value(failure()).expect("failure");
    raw.as_object_mut().expect("object").insert(
        "raw_provider_error".to_owned(),
        serde_json::json!("private endpoint and token"),
    );
    assert!(serde_json::from_value::<HatFailureEnvelope>(raw).is_err());

    let mut value = failure();
    value
        .parameters
        .insert("detail".to_owned(), "x".repeat(129));
    assert!(!validate_failure_envelope(&value).valid);
    value.parameters.clear();
    value.reason_id = "hathq://vocabulary/entity/source/v1".to_owned();
    assert!(!validate_failure_envelope(&value).valid);

    let mut undeclared = failure();
    undeclared.reason_id = "hathq://vocabulary/reason/undeclared/v1".to_owned();
    assert!(validate_failure_envelope(&undeclared).valid);
    assert!(!validate_failure_envelope_with_catalog(&undeclared, &catalog()).valid);

    let mut private_value = failure();
    private_value
        .parameters
        .insert("detail".to_owned(), "/home/owner/private".to_owned());
    assert!(!validate_failure_envelope(&private_value).valid);
}

#[test]
fn correlates_a_terminal_result_without_copying_failure_text() {
    let failure = failure();
    let result = HatActionResult {
        schema: ACTION_RESULT_SCHEMA.to_owned(),
        invocation_id: failure.invocation_id.clone(),
        operation_id: failure.operation_id.clone(),
        state_revision: failure.state_revision,
        projection_revision: 2,
        outcome: HatInvocationOutcome::Failed,
        output: None,
        reason_id: Some(failure.reason_id.clone()),
        evidence_refs: Vec::new(),
    };
    assert!(validate_action_failure(&result, &failure).valid);
    assert!(validate_action_failure_with_catalog(&result, &failure, &catalog()).valid);
    let mut substituted = failure;
    substituted.invocation_id = "invocation-2".to_owned();
    assert!(!validate_action_failure(&result, &substituted).valid);
}
