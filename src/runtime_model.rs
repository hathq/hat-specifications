use serde::{Deserialize, Serialize};

use crate::EventSemantics;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatBinding {
    pub schema: String,
    pub package_id: String,
    pub package_digest_sha256: String,
    pub catalog_digest_sha256: String,
    pub fitting_digest_sha256: String,
    pub subject_ref: String,
    pub scope_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextPartition {
    pub context_partition_id: String,
    pub revision: u64,
    pub policy_digest_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatInvocation {
    pub schema: String,
    pub invocation_id: String,
    pub binding: HatBinding,
    pub context_partition: ContextPartition,
    pub operation_id: String,
    pub expected_projection_revision: u64,
    pub idempotency_key: String,
    pub input: ActionReference,
    pub effective_grant: ActionReference,
    pub placement: ActionReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionReference {
    pub owner_id: String,
    pub reference: String,
    pub schema_id: String,
    pub digest_sha256: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HatInvocationPhase {
    Queued,
    Running,
    Waiting,
    Unresolved,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatActionStatus {
    pub schema: String,
    pub invocation_id: String,
    pub context_partition_id: String,
    pub state_revision: u64,
    pub phase: HatInvocationPhase,
    pub reason_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HatInvocationOutcome {
    Completed,
    Failed,
    Denied,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatActionResult {
    pub schema: String,
    pub invocation_id: String,
    pub operation_id: String,
    pub state_revision: u64,
    pub projection_revision: u64,
    pub outcome: HatInvocationOutcome,
    pub output: Option<ActionReference>,
    pub reason_id: Option<String>,
    pub evidence_refs: Vec<EvidenceReference>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HatInvocationControlKind {
    ReadStatus,
    Cancel,
    Recover,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatInvocationControl {
    pub schema: String,
    pub request_id: String,
    pub invocation_id: String,
    pub context_partition_id: String,
    pub expected_state_revision: u64,
    pub idempotency_key: String,
    pub kind: HatInvocationControlKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceReference {
    pub owner_id: String,
    pub reference: String,
    pub digest_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatProjectionEvent {
    pub schema: String,
    pub event_id: String,
    #[serde(flatten)]
    pub semantics: EventSemantics,
    pub invocation_id: String,
    pub context_partition_id: String,
    pub operation_id: String,
    pub previous_revision: u64,
    pub next_revision: u64,
    pub evidence_refs: Vec<EvidenceReference>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatProjectionJournal {
    pub schema: String,
    pub context_partition_id: String,
    pub package_id: String,
    pub from_revision: u64,
    pub to_revision: u64,
    pub events: Vec<HatProjectionEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnresolvedTerm {
    pub input_digest_sha256: String,
    pub catalog_digest_sha256: String,
    pub resolution_hat_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatProjection {
    pub schema: String,
    pub context_partition_id: String,
    pub package_id: String,
    pub revision: u64,
    pub timeline_watermark: u64,
    pub projection_digest_sha256: String,
    pub evidence_refs: Vec<EvidenceReference>,
    pub unresolved_terms: Vec<UnresolvedTerm>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatCheckpoint {
    pub schema: String,
    pub context_partition_id: String,
    pub package_id: String,
    pub from_watermark: u64,
    pub to_watermark: u64,
    pub previous_projection_digest_sha256: String,
    pub new_projection_digest_sha256: String,
    pub reducer_version: String,
}
