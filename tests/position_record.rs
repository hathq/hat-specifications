// Added by the HAT Specifications project, 2026.
// Purpose: prove attach, close, supersede, and conflict behavior.

use hat_specifications::{
    EvidenceReference, POSITION_RECORD_SCHEMA, PositionChangeKind, PositionRecord,
    validate_non_conflicting_position_records, validate_position_record,
    validate_position_transition,
};

const DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn record(change: PositionChangeKind, revision: u64) -> PositionRecord {
    PositionRecord {
        schema: POSITION_RECORD_SCHEMA.to_owned(),
        position_id: "position-1".to_owned(),
        revision,
        previous_revision: revision.checked_sub(1).filter(|value| *value > 0),
        change,
        holder_subject_id: "subject-person-1".to_owned(),
        context_subject_id: "subject-company-1".to_owned(),
        position_term_id: "hathq://vocabulary/position/director/v1".to_owned(),
        valid_from_epoch_s: 1_700_000_000,
        valid_until_epoch_s: (change == PositionChangeKind::Close).then_some(1_800_000_000),
        evidence_refs: vec![EvidenceReference {
            owner_id: "registry".to_owned(),
            reference: "evidence/position/1".to_owned(),
            digest_sha256: DIGEST.to_owned(),
        }],
    }
}

#[test]
fn position_can_attach_supersede_and_close_in_revision_order() {
    let attached = record(PositionChangeKind::Attach, 1);
    let mut superseded = record(PositionChangeKind::Supersede, 2);
    superseded.position_term_id = "hathq://vocabulary/position/representative/v1".to_owned();
    let mut closed = record(PositionChangeKind::Close, 3);
    closed.position_term_id = superseded.position_term_id.clone();
    assert!(validate_position_record(&attached).valid);
    assert!(validate_position_transition(&attached, &superseded).valid);
    assert!(validate_position_transition(&superseded, &closed).valid);
}

#[test]
fn close_cannot_change_subject_term_or_reopen() {
    let attached = record(PositionChangeKind::Attach, 1);
    let mut closed = record(PositionChangeKind::Close, 2);
    closed.context_subject_id = "subject-company-2".to_owned();
    assert!(!validate_position_transition(&attached, &closed).valid);

    let closed = record(PositionChangeKind::Close, 2);
    let reopened = record(PositionChangeKind::Supersede, 3);
    assert!(!validate_position_transition(&closed, &reopened).valid);
}

#[test]
fn concurrent_records_for_one_revision_are_conflicts() {
    let left = record(PositionChangeKind::Supersede, 2);
    let mut right = left.clone();
    right.position_term_id = "hathq://vocabulary/position/guardian/v1".to_owned();
    assert!(!validate_non_conflicting_position_records(&left, &right).valid);
}

#[test]
fn position_wire_rejects_unknown_fields_and_subject_self_reference() {
    let mut record = record(PositionChangeKind::Attach, 1);
    record.context_subject_id = record.holder_subject_id.clone();
    assert!(!validate_position_record(&record).valid);

    let mut value = serde_json::to_value(record).expect("serialize");
    value.as_object_mut().expect("object").insert(
        "display_role".to_owned(),
        serde_json::Value::String("director".to_owned()),
    );
    assert!(serde_json::from_value::<PositionRecord>(value).is_err());
}
