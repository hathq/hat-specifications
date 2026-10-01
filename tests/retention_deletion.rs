// Added by the HAT Specifications project, 2026.
// Purpose: prove selectable retention and exact local purge coverage.

use hat_specifications::{
    COMPLETE_DELETION_PREVIEW_SCHEMA, COMPLETE_DELETION_RECEIPT_SCHEMA, CompleteDeletionPreview,
    CompleteDeletionReceipt, ExternalOwnerDisclosure, HatRetentionMinimum, LocalDeletionClass,
    LostLocalCapability, RETENTION_POLICY_SCHEMA, RetentionPolicy, StorePurgeReceipt,
    validate_complete_deletion_preview, validate_complete_deletion_receipt,
    validate_retention_policy,
};

const DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn policy() -> RetentionPolicy {
    RetentionPolicy {
        schema: RETENTION_POLICY_SCHEMA.to_owned(),
        policy_id: "retention-1".to_owned(),
        revision: 2,
        scope_ref: "project-example".to_owned(),
        subject_id: "subject-person-1".to_owned(),
        schema_minimum_retention_s: 600,
        hat_minimums: vec![HatRetentionMinimum {
            package_id: "hat/health".to_owned(),
            package_digest_sha256: DIGEST.to_owned(),
            minimum_retention_s: 1_200,
        }],
        selected_retention_s: 3_600,
        complete_deletion_enabled: true,
    }
}

fn preview() -> CompleteDeletionPreview {
    CompleteDeletionPreview {
        schema: COMPLETE_DELETION_PREVIEW_SCHEMA.to_owned(),
        deletion_id: "deletion-1".to_owned(),
        policy_id: "retention-1".to_owned(),
        policy_revision: 2,
        scope_ref: "project-example".to_owned(),
        subject_id: "subject-person-1".to_owned(),
        through_revision: 42,
        checkpoint_id: "checkpoint-42".to_owned(),
        checkpoint_digest_sha256: DIGEST.to_owned(),
        eligible_at_epoch_s: 1_800_000_000,
        previewed_at_epoch_s: 1_800_000_001,
        local_store_ids: vec!["person-store".to_owned(), "timeline-store".to_owned()],
        local_deletion_classes: vec![
            LocalDeletionClass::References,
            LocalDeletionClass::Projections,
            LocalDeletionClass::EvidenceLinks,
            LocalDeletionClass::Digests,
            LocalDeletionClass::CorrectionTombstones,
        ],
        lost_local_capabilities: vec![
            LostLocalCapability::Recovery,
            LostLocalCapability::DuplicateDetection,
            LostLocalCapability::Provenance,
        ],
        external_owners: vec![ExternalOwnerDisclosure {
            owner_id: "health-provider".to_owned(),
            deletion_action_id: "hathq://vocabulary/action/delete-health-source/v1".to_owned(),
        }],
    }
}

fn receipt() -> CompleteDeletionReceipt {
    CompleteDeletionReceipt {
        schema: COMPLETE_DELETION_RECEIPT_SCHEMA.to_owned(),
        deletion_id: "deletion-1".to_owned(),
        policy_digest_sha256: DIGEST.to_owned(),
        deletion_request_digest_sha256: DIGEST.to_owned(),
        through_revision: 42,
        completed_at_epoch_s: 1_800_000_002,
        stores: vec![
            StorePurgeReceipt {
                store_id: "person-store".to_owned(),
                store_revision: 43,
                purge_digest_sha256: DIGEST.to_owned(),
            },
            StorePurgeReceipt {
                store_id: "timeline-store".to_owned(),
                store_revision: 43,
                purge_digest_sha256: DIGEST.to_owned(),
            },
        ],
    }
}

#[test]
fn user_selection_cannot_undercut_schema_or_hat_minimum() {
    let mut policy = policy();
    assert!(validate_retention_policy(&policy).valid);
    policy.selected_retention_s = 1_199;
    assert!(!validate_retention_policy(&policy).valid);
}

#[test]
fn preview_discloses_every_local_class_loss_and_external_owner() {
    let policy = policy();
    let mut deletion_preview = preview();
    assert!(validate_complete_deletion_preview(&deletion_preview, &policy).valid);
    deletion_preview.local_deletion_classes.pop();
    assert!(!validate_complete_deletion_preview(&deletion_preview, &policy).valid);
    let mut disabled = policy;
    disabled.complete_deletion_enabled = false;
    assert!(!validate_complete_deletion_preview(&preview(), &disabled).valid);
}

#[test]
fn receipt_covers_every_registered_local_store_exactly() {
    let preview = preview();
    let mut receipt = receipt();
    assert!(validate_complete_deletion_receipt(&receipt, &preview).valid);
    receipt.stores.pop();
    assert!(!validate_complete_deletion_receipt(&receipt, &preview).valid);
}

#[test]
fn receipt_is_subject_neutral_and_cannot_claim_external_deletion() {
    let mut value = serde_json::to_value(receipt()).expect("serialize");
    value.as_object_mut().expect("object").insert(
        "subject_id".to_owned(),
        serde_json::Value::String("subject-person-1".to_owned()),
    );
    assert!(serde_json::from_value::<CompleteDeletionReceipt>(value).is_err());

    let mut external = serde_json::to_value(receipt()).expect("serialize");
    external.as_object_mut().expect("object").insert(
        "external_provider_deleted".to_owned(),
        serde_json::Value::Bool(true),
    );
    assert!(serde_json::from_value::<CompleteDeletionReceipt>(external).is_err());
}
