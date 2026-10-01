// Added by the HAT Specifications project, 2026.
// Purpose: prove retention minima and complete local-store purge coverage.

use crate::model::validation;
use crate::tokens::{lower_hex_32, schema_uri, stable_token};
use crate::{
    COMPLETE_DELETION_PREVIEW_SCHEMA, COMPLETE_DELETION_RECEIPT_SCHEMA, CompleteDeletionPreview,
    CompleteDeletionReceipt, LocalDeletionClass, LostLocalCapability, RETENTION_POLICY_SCHEMA,
    RetentionPolicy, Validation,
};
use std::collections::BTreeSet;

#[must_use]
pub fn validate_retention_policy(policy: &RetentionPolicy) -> Validation {
    let mut findings = Vec::new();
    if policy.schema != RETENTION_POLICY_SCHEMA
        || !stable_token(&policy.policy_id)
        || policy.revision == 0
        || !stable_token(&policy.scope_ref)
        || !stable_token(&policy.subject_id)
    {
        findings.push("retention policy identity is invalid".to_owned());
    }
    let mut packages = BTreeSet::new();
    let mut required = policy.schema_minimum_retention_s;
    for minimum in &policy.hat_minimums {
        if !minimum.package_id.starts_with("hat/")
            || !lower_hex_32(&minimum.package_digest_sha256)
            || !packages.insert(&minimum.package_id)
        {
            findings.push("HAT retention minimum is invalid".to_owned());
        }
        required = required.max(minimum.minimum_retention_s);
    }
    if policy.hat_minimums.len() > 64 || policy.selected_retention_s < required {
        findings.push("selected retention is below a required minimum".to_owned());
    }
    validation(findings)
}

#[must_use]
pub fn validate_complete_deletion_preview(
    preview: &CompleteDeletionPreview,
    policy: &RetentionPolicy,
) -> Validation {
    let mut findings = validate_retention_policy(policy).findings;
    if preview.schema != COMPLETE_DELETION_PREVIEW_SCHEMA
        || !policy.complete_deletion_enabled
        || preview.policy_id != policy.policy_id
        || preview.policy_revision != policy.revision
        || preview.scope_ref != policy.scope_ref
        || preview.subject_id != policy.subject_id
        || !stable_token(&preview.deletion_id)
        || !stable_token(&preview.checkpoint_id)
        || !lower_hex_32(&preview.checkpoint_digest_sha256)
        || preview.previewed_at_epoch_s < preview.eligible_at_epoch_s
    {
        findings.push("complete deletion preview is invalid".to_owned());
    }
    validate_preview_sets(preview, &mut findings);
    validation(findings)
}

#[must_use]
pub fn validate_complete_deletion_receipt(
    receipt: &CompleteDeletionReceipt,
    preview: &CompleteDeletionPreview,
) -> Validation {
    let mut findings = Vec::new();
    if receipt.schema != COMPLETE_DELETION_RECEIPT_SCHEMA
        || receipt.deletion_id != preview.deletion_id
        || receipt.through_revision != preview.through_revision
        || receipt.completed_at_epoch_s < preview.previewed_at_epoch_s
        || !lower_hex_32(&receipt.policy_digest_sha256)
        || !lower_hex_32(&receipt.deletion_request_digest_sha256)
    {
        findings.push("complete deletion receipt is invalid".to_owned());
    }
    let expected = preview.local_store_ids.iter().collect::<BTreeSet<_>>();
    let actual = receipt
        .stores
        .iter()
        .map(|store| &store.store_id)
        .collect::<BTreeSet<_>>();
    if receipt.stores.len() != actual.len() || actual != expected {
        findings.push("complete deletion store coverage is invalid".to_owned());
    }
    for store in &receipt.stores {
        if !stable_token(&store.store_id)
            || store.store_revision == 0
            || !lower_hex_32(&store.purge_digest_sha256)
        {
            findings.push("complete deletion store receipt is invalid".to_owned());
        }
    }
    validation(findings)
}

fn validate_preview_sets(preview: &CompleteDeletionPreview, findings: &mut Vec<String>) {
    let stores = preview.local_store_ids.iter().collect::<BTreeSet<_>>();
    if preview.local_store_ids.is_empty()
        || preview.local_store_ids.len() > 64
        || stores.len() != preview.local_store_ids.len()
        || preview.local_store_ids.iter().any(|id| !stable_token(id))
    {
        findings.push("complete deletion local stores are invalid".to_owned());
    }
    let classes = preview
        .local_deletion_classes
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let required_classes = BTreeSet::from([
        LocalDeletionClass::References,
        LocalDeletionClass::Projections,
        LocalDeletionClass::EvidenceLinks,
        LocalDeletionClass::Digests,
        LocalDeletionClass::CorrectionTombstones,
    ]);
    if classes != required_classes || preview.local_deletion_classes.len() != classes.len() {
        findings.push("complete deletion classes are incomplete".to_owned());
    }
    let losses = preview
        .lost_local_capabilities
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let required_losses = BTreeSet::from([
        LostLocalCapability::Recovery,
        LostLocalCapability::DuplicateDetection,
        LostLocalCapability::Provenance,
    ]);
    if losses != required_losses || preview.lost_local_capabilities.len() != losses.len() {
        findings.push("complete deletion loss preview is incomplete".to_owned());
    }
    validate_external_owners(preview, findings);
}

fn validate_external_owners(preview: &CompleteDeletionPreview, findings: &mut Vec<String>) {
    if preview.external_owners.len() > 64 {
        findings.push("too many external deletion owners".to_owned());
    }
    let mut owners = BTreeSet::new();
    for owner in &preview.external_owners {
        if !stable_token(&owner.owner_id)
            || !owners.insert(&owner.owner_id)
            || !schema_uri(&owner.deletion_action_id)
            || !owner
                .deletion_action_id
                .starts_with("hathq://vocabulary/action/")
        {
            findings.push("external deletion owner is invalid".to_owned());
        }
    }
}
