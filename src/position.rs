// Added by the HAT Specifications project, 2026.
// Purpose: define revisioned subject positions and relationship changes.

use crate::EvidenceReference;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PositionChangeKind {
    Attach,
    Close,
    Supersede,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PositionRecord {
    pub schema: String,
    pub position_id: String,
    pub revision: u64,
    pub previous_revision: Option<u64>,
    pub change: PositionChangeKind,
    pub holder_subject_id: String,
    pub context_subject_id: String,
    pub position_term_id: String,
    pub valid_from_epoch_s: u64,
    pub valid_until_epoch_s: Option<u64>,
    pub evidence_refs: Vec<EvidenceReference>,
}
