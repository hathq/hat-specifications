// Added by the HAT Specifications project, 2026.
// Purpose: define selectable retention and complete local deletion records.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatRetentionMinimum {
    pub package_id: String,
    pub package_digest_sha256: String,
    pub minimum_retention_s: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetentionPolicy {
    pub schema: String,
    pub policy_id: String,
    pub revision: u64,
    pub scope_ref: String,
    pub subject_id: String,
    pub schema_minimum_retention_s: u64,
    pub hat_minimums: Vec<HatRetentionMinimum>,
    pub selected_retention_s: u64,
    pub complete_deletion_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalDeletionClass {
    References,
    Projections,
    EvidenceLinks,
    Digests,
    CorrectionTombstones,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LostLocalCapability {
    Recovery,
    DuplicateDetection,
    Provenance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalOwnerDisclosure {
    pub owner_id: String,
    pub deletion_action_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompleteDeletionPreview {
    pub schema: String,
    pub deletion_id: String,
    pub policy_id: String,
    pub policy_revision: u64,
    pub scope_ref: String,
    pub subject_id: String,
    pub through_revision: u64,
    pub checkpoint_id: String,
    pub checkpoint_digest_sha256: String,
    pub eligible_at_epoch_s: u64,
    pub previewed_at_epoch_s: u64,
    pub local_store_ids: Vec<String>,
    pub local_deletion_classes: Vec<LocalDeletionClass>,
    pub lost_local_capabilities: Vec<LostLocalCapability>,
    pub external_owners: Vec<ExternalOwnerDisclosure>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StorePurgeReceipt {
    pub store_id: String,
    pub store_revision: u64,
    pub purge_digest_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompleteDeletionReceipt {
    pub schema: String,
    pub deletion_id: String,
    pub policy_digest_sha256: String,
    pub deletion_request_digest_sha256: String,
    pub through_revision: u64,
    pub completed_at_epoch_s: u64,
    pub stores: Vec<StorePurgeReceipt>,
}
