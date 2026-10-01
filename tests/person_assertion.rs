// Added by the HAT Specifications project, 2026.
// Purpose: prove health, residence, correction, and conflict contracts.

use hat_specifications::{
    ChangePolicy, EvidenceReference, INFORMATION_COORDINATE_SCHEMA, InformationCoordinate,
    InformationSensitivity, KnowledgeState, PERSON_ASSERTION_SCHEMA, PersonAssertion,
    StructuralAxis, validate_person_assertion,
};

const DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn assertion(axis: StructuralAxis, policy: ChangePolicy) -> PersonAssertion {
    PersonAssertion {
        schema: PERSON_ASSERTION_SCHEMA.to_owned(),
        assertion_id: "assertion-1".to_owned(),
        revision: 1,
        scope_ref: "person-self".to_owned(),
        subject_id: "subject-person-1".to_owned(),
        coordinate: InformationCoordinate {
            schema: INFORMATION_COORDINATE_SCHEMA.to_owned(),
            vocabulary_owner_id: "regional-hat".to_owned(),
            catalog_digest_sha256: DIGEST.to_owned(),
            term_id: "hathq://regional/term/value/v1".to_owned(),
            structural_axis: axis,
            change_policy: policy,
            data_domain_term_id: "hathq://vocabulary/data-domain/person/v1".to_owned(),
            sensitivity: InformationSensitivity::RestrictedSensitive,
        },
        producer_id: "regional-hat".to_owned(),
        confirmation_authority_id: None,
        knowledge_state: KnowledgeState::Observed,
        projection_digest_sha256: DIGEST.to_owned(),
        evidence_refs: vec![EvidenceReference {
            owner_id: "provider".to_owned(),
            reference: "source/value/1".to_owned(),
            digest_sha256: DIGEST.to_owned(),
        }],
        recorded_at_epoch_s: 1_800_000_000,
        observed_at_epoch_s: Some(1_700_000_000),
        valid_from_epoch_s: None,
        valid_until_epoch_s: None,
        source_issued_at_epoch_s: Some(1_700_000_000),
        supersedes_assertion_id: None,
        superseded_by_assertion_id: None,
        conflicting_assertion_ids: Vec::new(),
    }
}

#[test]
fn health_observation_and_residence_interval_remain_distinct() {
    let health = assertion(
        StructuralAxis::HealthObservation,
        ChangePolicy::AppendOnlyObservation,
    );
    assert!(validate_person_assertion(&health).valid);
    let mut residence = assertion(StructuralAxis::LocationResidence, ChangePolicy::Interval);
    residence.knowledge_state = KnowledgeState::Confirmed;
    residence.confirmation_authority_id = Some("owner".to_owned());
    residence.valid_from_epoch_s = Some(1_700_000_000);
    residence.valid_until_epoch_s = Some(1_750_000_000);
    assert!(validate_person_assertion(&residence).valid);
}

#[test]
fn correction_and_conflict_links_are_explicit() {
    let mut correction = assertion(
        StructuralAxis::IdentityLifeEvent,
        ChangePolicy::CorrectionOnly,
    );
    correction.assertion_id = "assertion-2".to_owned();
    correction.supersedes_assertion_id = Some("assertion-1".to_owned());
    assert!(validate_person_assertion(&correction).valid);

    let mut conflict = assertion(StructuralAxis::LocationResidence, ChangePolicy::Interval);
    conflict.knowledge_state = KnowledgeState::Conflicted;
    conflict.conflicting_assertion_ids = vec!["assertion-2".to_owned()];
    assert!(validate_person_assertion(&conflict).valid);
}

#[test]
fn copied_body_and_inferred_confirmation_are_rejected() {
    let mut value = serde_json::to_value(assertion(
        StructuralAxis::HealthObservation,
        ChangePolicy::AppendOnlyObservation,
    ))
    .expect("serialize");
    value
        .as_object_mut()
        .expect("object")
        .insert("value".to_owned(), serde_json::json!({"heart_rate": 64}));
    assert!(serde_json::from_value::<PersonAssertion>(value).is_err());

    let mut inferred = assertion(StructuralAxis::LocationResidence, ChangePolicy::Interval);
    inferred.knowledge_state = KnowledgeState::Confirmed;
    assert!(!validate_person_assertion(&inferred).valid);
}

#[test]
fn superseded_assertion_names_its_replacement() {
    let mut old = assertion(
        StructuralAxis::IdentityLifeEvent,
        ChangePolicy::CorrectionOnly,
    );
    old.knowledge_state = KnowledgeState::Superseded;
    old.superseded_by_assertion_id = Some("assertion-2".to_owned());
    assert!(validate_person_assertion(&old).valid);
    old.superseded_by_assertion_id = None;
    assert!(!validate_person_assertion(&old).valid);
}
