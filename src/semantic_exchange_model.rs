// Added by the HAT Specifications project, 2026.
// Purpose: define a bounded semantic result exchange without graph synchronization.

use crate::{SemanticClause, SemanticStableRef, SemanticTermReference};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticExchange {
    pub schema: String,
    pub exchange_id: String,
    pub sender: SemanticStableRef,
    pub receiver: SemanticStableRef,
    pub clause: SemanticClause,
    pub terms: Vec<SemanticTermReference>,
    pub frame_digest_sha256: String,
    pub output_schema_digest_sha256: String,
    pub source_revision: u64,
    pub evidence: Vec<SemanticStableRef>,
    pub information_classification: String,
    pub purpose: String,
    pub retention: SemanticStableRef,
    pub issued_at_epoch_s: u64,
    pub expires_at_epoch_s: u64,
    pub nonce: String,
    pub content_digest_sha256: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SemanticTermComprehension {
    Understood,
    HatRequired,
    VersionConflict,
    PolicyDenied,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticComprehensionItem {
    pub term: SemanticTermReference,
    pub state: SemanticTermComprehension,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticComprehensionReport {
    pub schema: String,
    pub exchange_id: String,
    pub receiver: SemanticStableRef,
    pub terms: Vec<SemanticComprehensionItem>,
    pub accepted: bool,
}
