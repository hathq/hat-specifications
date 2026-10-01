// Added by the HAT Specifications project, 2026.
// Purpose: prove subjective one-user, scoped, revisioned control grants.

use hat_specifications::{
    CONTROL_GRANT_SCHEMA, ControlGrant, ControlGrantState, EvidenceReference,
    OwnerDecisionReference, validate_control_grant, validate_control_grant_revocation,
};

const DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn grant() -> ControlGrant {
    ControlGrant {
        schema: CONTROL_GRANT_SCHEMA.to_owned(),
        grant_id: "grant-1".to_owned(),
        revision: 1,
        operator_subject_id: "subject-parent".to_owned(),
        represented_subject_id: "subject-child".to_owned(),
        position_id: "position-parent-1".to_owned(),
        position_revision: 2,
        capacity_term_id: "hathq://vocabulary/capacity/guardian/v1".to_owned(),
        action_scope_ids: vec!["hathq://vocabulary/action/read-health-summary/v1".to_owned()],
        valid_from_epoch_s: 1_700_000_000,
        valid_until_epoch_s: 1_800_000_000,
        state: ControlGrantState::Active,
        revoked_at_epoch_s: None,
        decision: OwnerDecisionReference {
            decision_id: "decision-1".to_owned(),
            decided_by_subject_id: "subject-parent".to_owned(),
            decision_revision: 1,
            decision_digest_sha256: DIGEST.to_owned(),
        },
        evidence_refs: vec![EvidenceReference {
            owner_id: "ihat".to_owned(),
            reference: "evidence/grant/1".to_owned(),
            digest_sha256: DIGEST.to_owned(),
        }],
    }
}

#[test]
fn one_user_decision_is_scope_time_position_and_revision_bound() {
    assert!(validate_control_grant(&grant()).valid);
}

#[test]
fn ambient_or_quorum_shaped_authority_is_rejected() {
    let mut ambient = grant();
    ambient.action_scope_ids.clear();
    assert!(!validate_control_grant(&ambient).valid);

    let mut value = serde_json::to_value(grant()).expect("serialize");
    value.as_object_mut().expect("object").insert(
        "approvers".to_owned(),
        serde_json::json!(["subject-parent", "subject-other"]),
    );
    assert!(serde_json::from_value::<ControlGrant>(value).is_err());
}

#[test]
fn another_subject_cannot_make_the_local_owner_decision() {
    let mut substituted = grant();
    substituted.decision.decided_by_subject_id = "subject-other".to_owned();
    assert!(!validate_control_grant(&substituted).valid);
}

#[test]
fn revocation_is_exact_and_cannot_widen_scope() {
    let active = grant();
    let mut revoked = active.clone();
    revoked.revision = 2;
    revoked.state = ControlGrantState::Revoked;
    revoked.revoked_at_epoch_s = Some(1_750_000_000);
    assert!(validate_control_grant_revocation(&active, &revoked).valid);

    revoked
        .action_scope_ids
        .push("hathq://vocabulary/action/write-health-record/v1".to_owned());
    assert!(!validate_control_grant_revocation(&active, &revoked).valid);
}
