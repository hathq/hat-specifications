// Added by the HAT Specifications project, 2026.
// Purpose: define evidence-linked assertions without copied source bodies.

use crate::{EvidenceReference, InformationCoordinate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum KnowledgeState {
    Observed,
    Proposed,
    Confirmed,
    Conflicted,
    Stale,
    Unavailable,
    Withdrawn,
    Superseded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersonAssertion {
    pub schema: String,
    pub assertion_id: String,
    pub revision: u64,
    pub scope_ref: String,
    pub subject_id: String,
    pub coordinate: InformationCoordinate,
    pub producer_id: String,
    pub confirmation_authority_id: Option<String>,
    pub knowledge_state: KnowledgeState,
    pub projection_digest_sha256: String,
    pub evidence_refs: Vec<EvidenceReference>,
    pub recorded_at_epoch_s: u64,
    pub observed_at_epoch_s: Option<u64>,
    pub valid_from_epoch_s: Option<u64>,
    pub valid_until_epoch_s: Option<u64>,
    pub source_issued_at_epoch_s: Option<u64>,
    pub supersedes_assertion_id: Option<String>,
    pub superseded_by_assertion_id: Option<String>,
    pub conflicting_assertion_ids: Vec<String>,
}
