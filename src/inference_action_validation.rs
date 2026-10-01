use crate::model::validation;
use crate::tokens::{classification_rank, lower_hex_32, schema_uri, stable_token};
use crate::{
    ActionReference, INFERENCE_ACTION_SCHEMA, InferenceAction, InferenceActionState,
    InferenceDecisionState, InferenceProcedureState, Validation,
};
use std::collections::BTreeSet;

#[must_use]
pub fn validate_inference_action(value: &InferenceAction) -> Validation {
    let mut findings = Vec::new();
    if value.schema != INFERENCE_ACTION_SCHEMA
        || !stable_token(&value.action_id)
        || !stable_token(&value.routine_id)
        || !label_key(&value.title_key)
        || !repository_id(&value.producer_repository_id)
        || !stable_token(&value.invocation_id)
        || !stable_token(&value.route_id)
        || !schema_uri(&value.operation_id)
        || !schema_uri(&value.role_id)
        || value.revision == 0
        || value.created_at_epoch_s == 0
        || value.updated_at_epoch_s < value.created_at_epoch_s
    {
        findings.push("inference action identity is invalid".to_owned());
    }
    validate_reference(&value.trigger.source, &mut findings);
    validate_snapshot(value, &mut findings);
    validate_procedure(value, &mut findings);
    validate_decision(value, &mut findings);
    validate_effects(value, &mut findings);
    validate_reference(&value.delegation.policy, &mut findings);
    if let Some(output) = &value.output {
        validate_reference(output, &mut findings);
    }
    let terminal = matches!(
        value.state,
        InferenceActionState::Completed
            | InferenceActionState::Failed
            | InferenceActionState::Cancelled
    );
    if matches!(value.state, InferenceActionState::Completed) != value.output.is_some()
        || (terminal && !matches!(value.state, InferenceActionState::Completed))
            != value.reason_id.is_some()
        || value.reason_id.as_ref().is_some_and(|id| !schema_uri(id))
    {
        findings.push("inference action terminal result is invalid".to_owned());
    }
    validation(findings)
}

fn validate_snapshot(value: &InferenceAction, findings: &mut Vec<String>) {
    let snapshot = &value.input_snapshot;
    let mut ids = BTreeSet::new();
    if !lower_hex_32(&snapshot.digest_sha256)
        || snapshot.items.is_empty()
        || snapshot.items.len() > 256
    {
        findings.push("inference input snapshot is invalid".to_owned());
    }
    for item in &snapshot.items {
        if !stable_token(&item.field_id)
            || !ids.insert(item.field_id.as_str())
            || !label_key(&item.label_key)
            || item.canonical_value.is_empty()
            || item.canonical_value.len() > 4096
            || !schema_uri(&item.purpose_term_id)
            || classification_rank(&item.information_classification).is_none()
        {
            findings.push("inspectable inference input is invalid".to_owned());
        }
        validate_reference(&item.source, findings);
    }
    if !snapshot.items.iter().any(|item| item.included) {
        findings.push("inference input snapshot has no included item".to_owned());
    }
}

fn validate_procedure(value: &InferenceAction, findings: &mut Vec<String>) {
    let mut ids = BTreeSet::new();
    let mut current = 0;
    if value.procedure.is_empty() || value.procedure.len() > 16 {
        findings.push("inference procedure size is invalid".to_owned());
    }
    for step in &value.procedure {
        if !stable_token(&step.step_id)
            || !ids.insert(step.step_id.as_str())
            || !label_key(&step.title_key)
        {
            findings.push("inference procedure step is invalid".to_owned());
        }
        if step.state == InferenceProcedureState::Current {
            current += 1;
        }
    }
    if current > 1 {
        findings.push("inference procedure has multiple current steps".to_owned());
    }
}

fn validate_decision(value: &InferenceAction, findings: &mut Vec<String>) {
    let Some(decision) = &value.decision else {
        return;
    };
    let candidates = decision.candidate_ids.iter().collect::<BTreeSet<_>>();
    let fields = value
        .input_snapshot
        .items
        .iter()
        .filter(|item| item.included)
        .map(|item| item.field_id.as_str())
        .collect::<BTreeSet<_>>();
    if decision.candidate_ids.len() > 128
        || candidates.len() != decision.candidate_ids.len()
        || decision.candidate_ids.iter().any(|id| !schema_uri(id))
        || decision.evidence_field_ids.is_empty()
        || decision.evidence_field_ids.len() > 256
        || decision
            .evidence_field_ids
            .iter()
            .any(|id| !fields.contains(id.as_str()))
        || decision
            .reason_id
            .as_ref()
            .is_some_and(|id| !schema_uri(id))
    {
        findings.push("inference decision evidence is invalid".to_owned());
    }
    match decision.state {
        InferenceDecisionState::Confirmed => {
            if decision
                .selected_candidate_id
                .as_ref()
                .is_none_or(|id| !candidates.contains(id))
                || decision.reason_id.is_some()
            {
                findings.push("confirmed inference decision is invalid".to_owned());
            }
        }
        InferenceDecisionState::Ambiguous | InferenceDecisionState::Unresolved => {
            if decision.selected_candidate_id.is_some() || decision.reason_id.is_none() {
                findings.push("unresolved inference decision is invalid".to_owned());
            }
        }
    }
}

fn validate_effects(value: &InferenceAction, findings: &mut Vec<String>) {
    let mut ids = BTreeSet::new();
    if value.proposed_effects.len() > 64 {
        findings.push("inference proposed effect size is invalid".to_owned());
    }
    for effect in &value.proposed_effects {
        if !stable_token(&effect.effect_id)
            || !ids.insert(effect.effect_id.as_str())
            || !schema_uri(&effect.surface_id)
        {
            findings.push("inference proposed effect is invalid".to_owned());
        }
        validate_reference(&effect.payload, findings);
    }
}

fn validate_reference(value: &ActionReference, findings: &mut Vec<String>) {
    if !stable_token(&value.owner_id)
        || value.reference.is_empty()
        || value.reference.len() > 512
        || !schema_uri(&value.schema_id)
        || !lower_hex_32(&value.digest_sha256)
    {
        findings.push("inference action reference is invalid".to_owned());
    }
}

fn label_key(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 160
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'-' | b'_')
        })
}

fn repository_id(value: &str) -> bool {
    value.strip_prefix("hat-").is_some_and(stable_token)
}
