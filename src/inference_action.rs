use crate::ActionReference;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InferenceActionState {
    Prepared,
    Running,
    Waiting,
    Unresolved,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InferenceTriggerKind {
    Event,
    Schedule,
    OwnerRequest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InferenceTrigger {
    pub kind: InferenceTriggerKind,
    pub source: ActionReference,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InspectableValueKind {
    Text,
    Timestamp,
    Reference,
    Count,
    Boolean,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InspectableInferenceInput {
    pub field_id: String,
    pub label_key: String,
    pub value_kind: InspectableValueKind,
    pub canonical_value: String,
    pub source: ActionReference,
    pub purpose_term_id: String,
    pub information_classification: String,
    pub included: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InferenceInputSnapshot {
    pub digest_sha256: String,
    pub items: Vec<InspectableInferenceInput>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InferenceProcedureKind {
    Observe,
    Normalize,
    Infer,
    Validate,
    Propose,
    Execute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InferenceProcedureState {
    Upcoming,
    Current,
    Complete,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InferenceProcedureStep {
    pub step_id: String,
    pub kind: InferenceProcedureKind,
    pub title_key: String,
    pub state: InferenceProcedureState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InferenceDecisionState {
    Confirmed,
    Ambiguous,
    Unresolved,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InferenceDecision {
    pub state: InferenceDecisionState,
    pub candidate_ids: Vec<String>,
    pub selected_candidate_id: Option<String>,
    pub evidence_field_ids: Vec<String>,
    pub reason_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InferenceEffectKind {
    Create,
    Link,
    Update,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InferenceProposedEffect {
    pub effect_id: String,
    pub kind: InferenceEffectKind,
    pub surface_id: String,
    pub payload: ActionReference,
    pub requires_approval: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DelegationMode {
    Observe,
    Propose,
    Conditional,
    Delegated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InferenceDelegation {
    pub mode: DelegationMode,
    pub policy: ActionReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InferenceAction {
    pub schema: String,
    pub action_id: String,
    pub revision: u64,
    pub routine_id: String,
    pub title_key: String,
    pub producer_repository_id: String,
    pub invocation_id: String,
    pub route_id: String,
    pub operation_id: String,
    pub role_id: String,
    pub state: InferenceActionState,
    pub trigger: InferenceTrigger,
    pub input_snapshot: InferenceInputSnapshot,
    pub procedure: Vec<InferenceProcedureStep>,
    pub decision: Option<InferenceDecision>,
    pub proposed_effects: Vec<InferenceProposedEffect>,
    pub delegation: InferenceDelegation,
    pub output: Option<ActionReference>,
    pub reason_id: Option<String>,
    pub created_at_epoch_s: u64,
    pub updated_at_epoch_s: u64,
}
