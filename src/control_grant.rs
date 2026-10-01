// Added by the HAT Specifications project, 2026.
// Purpose: define one-user control grants for represented subjects.

use crate::EvidenceReference;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ControlGrantState {
    Active,
    Revoked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnerDecisionReference {
    pub decision_id: String,
    pub decided_by_subject_id: String,
    pub decision_revision: u64,
    pub decision_digest_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ControlGrant {
    pub schema: String,
    pub grant_id: String,
    pub revision: u64,
    pub operator_subject_id: String,
    pub represented_subject_id: String,
    pub position_id: String,
    pub position_revision: u64,
    pub capacity_term_id: String,
    pub action_scope_ids: Vec<String>,
    pub valid_from_epoch_s: u64,
    pub valid_until_epoch_s: u64,
    pub state: ControlGrantState,
    pub revoked_at_epoch_s: Option<u64>,
    pub decision: OwnerDecisionReference,
    pub evidence_refs: Vec<EvidenceReference>,
}
