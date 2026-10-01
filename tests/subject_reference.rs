// Added by the HAT Specifications project, 2026.
// Purpose: prove natural and legal subjects remain closed and distinct.

use hat_specifications::{
    SUBJECT_REFERENCE_SCHEMA, SubjectKind, SubjectReference, validate_distinct_subjects,
    validate_subject_reference,
};

fn subject(id: &str, kind: SubjectKind, identity: &str) -> SubjectReference {
    SubjectReference {
        schema: SUBJECT_REFERENCE_SCHEMA.to_owned(),
        subject_id: id.to_owned(),
        kind,
        identity_authority_id: "ihat".to_owned(),
        identity_reference: identity.to_owned(),
        identity_revision: 1,
    }
}

#[test]
fn natural_and_legal_people_are_distinct_subjects() {
    let person = subject(
        "subject-person-1",
        SubjectKind::NaturalPerson,
        "identity/person/1",
    );
    let company = subject(
        "subject-company-1",
        SubjectKind::LegalPerson,
        "identity/company/1",
    );
    assert!(validate_subject_reference(&person).valid);
    assert!(validate_subject_reference(&company).valid);
    assert!(validate_distinct_subjects(&person, &company).valid);
}

#[test]
fn subject_id_and_authority_identity_cannot_be_aliased() {
    let person = subject("subject-1", SubjectKind::NaturalPerson, "identity/1");
    let mut same_id = subject("subject-1", SubjectKind::LegalPerson, "identity/2");
    assert!(!validate_distinct_subjects(&person, &same_id).valid);
    same_id.subject_id = "subject-2".to_owned();
    same_id.identity_reference = person.identity_reference.clone();
    assert!(!validate_distinct_subjects(&person, &same_id).valid);
}

#[test]
fn subject_wire_is_closed_and_revision_bound() {
    let subject = subject("subject-1", SubjectKind::NaturalPerson, "identity/1");
    let mut value = serde_json::to_value(&subject).expect("serialize");
    value.as_object_mut().expect("object").insert(
        "display_name".to_owned(),
        serde_json::Value::String("not part of identity".to_owned()),
    );
    assert!(serde_json::from_value::<SubjectReference>(value).is_err());

    let mut invalid = subject;
    invalid.identity_revision = 0;
    invalid.identity_reference = " identity/1".to_owned();
    assert!(!validate_subject_reference(&invalid).valid);
}

#[test]
fn unknown_subject_kind_is_rejected() {
    let value = serde_json::json!({
        "schema": SUBJECT_REFERENCE_SCHEMA,
        "subject_id": "subject-1",
        "kind": "household",
        "identity_authority_id": "ihat",
        "identity_reference": "identity/1",
        "identity_revision": 1
    });
    assert!(serde_json::from_value::<SubjectReference>(value).is_err());
}
