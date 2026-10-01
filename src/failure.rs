use crate::EvidenceReference;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HatFailureClass {
    Input,
    Precondition,
    Authority,
    Dependency,
    Availability,
    Conflict,
    Limit,
    Timeout,
    Cancelled,
    Internal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HatFailureRecovery {
    None,
    OwnerAction,
    Retry,
    ExternalChange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HatFailureResponsibility {
    Owner,
    Hatter,
    Hat,
    ExternalService,
}

/// A bounded semantic failure record. It contains no provider error text or source payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatFailureEnvelope {
    pub schema: String,
    pub failure_id: String,
    pub invocation_id: String,
    pub component_id: String,
    pub operation_id: String,
    pub reason_id: String,
    pub class: HatFailureClass,
    pub recovery: HatFailureRecovery,
    pub responsibility: HatFailureResponsibility,
    pub state_revision: u64,
    pub parameters: BTreeMap<String, String>,
    pub next_action_id: Option<String>,
    pub evidence_refs: Vec<EvidenceReference>,
}
