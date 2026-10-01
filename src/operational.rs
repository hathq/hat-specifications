// Added by the HAT Specifications project, 2026.
// Purpose: define canonical, provider-neutral digital-twin operation records.

use crate::{EvidenceReference, OwnerDecisionReference, SemanticStableRef, SemanticTermReference};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GoalOrigin {
    UserDeclared,
    HatDerived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GoalState {
    Active,
    Satisfied,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DesiredState {
    pub schema: String,
    pub desired_state_id: String,
    pub revision: u64,
    pub subject: SemanticStableRef,
    pub requirements: Vec<SemanticStableRef>,
    pub valid_from_epoch_s: u64,
    pub valid_until_epoch_s: Option<u64>,
    pub evidence_refs: Vec<EvidenceReference>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Goal {
    pub schema: String,
    pub goal_id: String,
    pub revision: u64,
    pub owner_subject: SemanticStableRef,
    pub origin: GoalOrigin,
    pub parent_goal: Option<SemanticStableRef>,
    pub proposed_by_hat: Option<SemanticStableRef>,
    pub desired_state: SemanticStableRef,
    pub constraint_refs: Vec<SemanticStableRef>,
    pub state: GoalState,
    pub evidence_refs: Vec<EvidenceReference>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CommitmentState {
    Active,
    Fulfilled,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Commitment {
    pub schema: String,
    pub commitment_id: String,
    pub revision: u64,
    pub goal: SemanticStableRef,
    pub desired_state: SemanticStableRef,
    pub committed_by: SemanticStableRef,
    pub decision: OwnerDecisionReference,
    pub valid_from_epoch_s: u64,
    pub valid_until_epoch_s: u64,
    pub state: CommitmentState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TaskState {
    Candidate,
    AwaitingDecision,
    Ready,
    Blocked,
    Executing,
    AwaitingVerification,
    Completed,
    Exception,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Task {
    pub schema: String,
    pub task_id: String,
    pub revision: u64,
    pub goal: SemanticStableRef,
    pub commitment: SemanticStableRef,
    pub derived_by_hat: SemanticStableRef,
    pub derived_from_requirement: SemanticStableRef,
    pub derivation_rule: SemanticStableRef,
    pub observation_refs: Vec<SemanticStableRef>,
    pub operation: SemanticTermReference,
    pub capability: SemanticTermReference,
    pub scope_refs: Vec<SemanticStableRef>,
    pub dependency_refs: Vec<SemanticStableRef>,
    pub constraint_refs: Vec<SemanticStableRef>,
    pub state: TaskState,
    pub delegation: Option<SemanticStableRef>,
    pub verified_by: Option<SemanticStableRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DelegationState {
    Active,
    Revoked,
    Expired,
    Exhausted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Delegation {
    pub schema: String,
    pub delegation_id: String,
    pub revision: u64,
    pub task: SemanticStableRef,
    pub delegator: SemanticStableRef,
    pub delegate: SemanticStableRef,
    pub capability: SemanticTermReference,
    pub scope_refs: Vec<SemanticStableRef>,
    pub condition_refs: Vec<SemanticStableRef>,
    pub valid_from_epoch_s: u64,
    pub valid_until_epoch_s: u64,
    pub maximum_executions: u32,
    pub state: DelegationState,
    pub decision: OwnerDecisionReference,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExecutionOutcome {
    Succeeded,
    Failed,
    Denied,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Execution {
    pub schema: String,
    pub execution_id: String,
    pub revision: u64,
    pub task: SemanticStableRef,
    pub delegation: SemanticStableRef,
    pub provider: SemanticStableRef,
    pub capability: SemanticTermReference,
    pub operation: SemanticTermReference,
    pub request: SemanticStableRef,
    pub result: Option<SemanticStableRef>,
    pub started_at_epoch_s: u64,
    pub finished_at_epoch_s: Option<u64>,
    pub outcome: ExecutionOutcome,
    pub evidence_refs: Vec<EvidenceReference>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VerificationOutcome {
    Satisfied,
    Unsatisfied,
    Inconclusive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Verification {
    pub schema: String,
    pub verification_id: String,
    pub revision: u64,
    pub task: SemanticStableRef,
    pub execution: SemanticStableRef,
    pub desired_state: SemanticStableRef,
    pub verifier: SemanticStableRef,
    pub observation_refs: Vec<SemanticStableRef>,
    pub outcome: VerificationOutcome,
    pub verified_at_epoch_s: u64,
    pub evidence_refs: Vec<EvidenceReference>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IntelligenceArtifactKind {
    Observation,
    Candidate,
    Projection,
    Proposal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntelligenceArtifact {
    pub schema: String,
    pub artifact_id: String,
    pub revision: u64,
    pub kind: IntelligenceArtifactKind,
    pub producer: SemanticStableRef,
    pub subject: SemanticStableRef,
    pub payload: SemanticStableRef,
    pub evidence_refs: Vec<EvidenceReference>,
}

impl IntelligenceArtifact {
    #[must_use]
    pub const fn authorizes_effect(&self) -> bool {
        false
    }
}
