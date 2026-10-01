// Hatter downstream 2026: bounded operational input wiring, not a language compiler.
use crate::SemanticStableRef;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

/// An enabled connection is a routing declaration, never proof of authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerceptionState {
    Enabled,
    Disabled,
}

/// Each perspective restores its own exact sem-lang binding and `RuntimeRole`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InterpretationRoute {
    pub hat_binding_ref: SemanticStableRef,
    pub runtime_role_ref: SemanticStableRef,
    pub semantic_binding_ref: SemanticStableRef,
}

/// Hatter-owned wiring of an adopted sense to an external input capability.
/// Sense meaning (reading, hearing, etc.) belongs to sem-lang, not an enum here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PerceptionBinding {
    pub binding_id: String,
    pub revision: u64,
    pub role_ref: SemanticStableRef,
    pub sense_ref: SemanticStableRef,
    pub provider_ref: SemanticStableRef,
    pub account_ref: SemanticStableRef,
    pub source_scope_ref: SemanticStableRef,
    pub input_contract_ref: SemanticStableRef,
    pub state: PerceptionState,
    pub interpretations: Vec<InterpretationRoute>,
}

/// Exact owner-held observation; no credential, message body or copied memory.
/// Occurrence and place are independent evidence, not browser camera coordinates
/// or the sender's domain geocoded as the user's residence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PerceptionObservation {
    pub source_ref: SemanticStableRef,
    pub provider_ref: SemanticStableRef,
    pub account_ref: SemanticStableRef,
    pub source_scope_ref: SemanticStableRef,
    pub input_contract_ref: SemanticStableRef,
    pub received_at_epoch_ms: u64,
    pub occurrence_ref: Option<SemanticStableRef>,
    pub location_ref: Option<SemanticStableRef>,
}

/// Deterministic proposal for the existing Work admission boundary. This does
/// not schedule work, validate semantic meaning, grant access or promote results.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PerceptionPlan {
    pub request_key: String,
    pub role_ref: SemanticStableRef,
    pub sense_ref: SemanticStableRef,
    pub source_ref: SemanticStableRef,
    pub received_at_epoch_ms: u64,
    pub occurrence_ref: Option<SemanticStableRef>,
    pub location_ref: Option<SemanticStableRef>,
    pub routes: Vec<InterpretationRoute>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerceptionError {
    InvalidReference,
    InvalidBinding,
    Disabled,
    SourceMismatch,
    DuplicateRoute,
}

fn check_ref(value: &SemanticStableRef) -> Result<(), PerceptionError> {
    if value.revision == 0
        || crate::semantic_validation::reference(value).is_err()
        || value.id.chars().any(char::is_control)
        || value.schema.chars().any(char::is_control)
    {
        return Err(PerceptionError::InvalidReference);
    }
    Ok(())
}

/// Validates and deterministically orders an input-to-perspectives proposal.
///
/// The caller must resolve every reference against its original owner, retain
/// the observation, validate installed HAT/Role state and obtain current Grants
/// at normal Work admission. Neither schema equality nor this result proves it.
/// Retries require the same immutable observation metadata. Receipt storage and
/// exactly-once effects remain the existing Work owner's responsibility.
///
/// # Errors
/// Rejects invalid/excessive references, disabled bindings, cross-account/scope
/// observations and duplicate routes. There is no default interpreter or model.
pub fn plan_perception(
    binding: &PerceptionBinding,
    observation: &PerceptionObservation,
) -> Result<PerceptionPlan, PerceptionError> {
    if binding.revision == 0
        || binding.binding_id.len() > 256
        || binding.binding_id.is_empty()
        || binding.binding_id.trim() != binding.binding_id
        || binding.binding_id.chars().any(char::is_control)
        || binding.interpretations.is_empty()
        || binding.interpretations.len() > 32
    {
        return Err(PerceptionError::InvalidBinding);
    }
    if binding.state != PerceptionState::Enabled {
        return Err(PerceptionError::Disabled);
    }
    for value in [
        &binding.role_ref,
        &binding.sense_ref,
        &binding.provider_ref,
        &binding.account_ref,
        &binding.source_scope_ref,
        &binding.input_contract_ref,
        &observation.source_ref,
        &observation.provider_ref,
        &observation.account_ref,
        &observation.source_scope_ref,
        &observation.input_contract_ref,
    ]
    .into_iter()
    .chain(observation.occurrence_ref.iter())
    .chain(observation.location_ref.iter())
    {
        check_ref(value)?;
    }
    if binding.provider_ref != observation.provider_ref
        || binding.account_ref != observation.account_ref
        || binding.source_scope_ref != observation.source_scope_ref
        || binding.input_contract_ref != observation.input_contract_ref
    {
        return Err(PerceptionError::SourceMismatch);
    }
    let mut routes = Vec::with_capacity(binding.interpretations.len());
    let mut unique = BTreeSet::new();
    for route in &binding.interpretations {
        for value in [
            &route.hat_binding_ref,
            &route.runtime_role_ref,
            &route.semantic_binding_ref,
        ] {
            check_ref(value)?;
        }
        let bytes = serde_json::to_vec(route).map_err(|_| PerceptionError::InvalidBinding)?;
        if !unique.insert(bytes.clone()) {
            return Err(PerceptionError::DuplicateRoute);
        }
        routes.push((bytes, route.clone()));
    }
    routes.sort_by(|a, b| a.0.cmp(&b.0));
    let routes: Vec<_> = routes.into_iter().map(|(_, route)| route).collect();
    let mut canonical = binding.clone();
    canonical.interpretations.clone_from(&routes);
    let bytes = serde_json::to_vec(&("hat/perception/plan/1", canonical, observation))
        .map_err(|_| PerceptionError::InvalidBinding)?;
    let request_key = format!("{:x}", Sha256::digest(bytes));
    Ok(PerceptionPlan {
        request_key,
        role_ref: binding.role_ref.clone(),
        sense_ref: binding.sense_ref.clone(),
        source_ref: observation.source_ref.clone(),
        received_at_epoch_ms: observation.received_at_epoch_ms,
        occurrence_ref: observation.occurrence_ref.clone(),
        location_ref: observation.location_ref.clone(),
        routes,
    })
}
