// Added by the HAT Specifications project, 2026.
// Purpose: validate evidence, validity, correction, and conflict invariants.

use crate::model::validation;
use crate::tokens::{lower_hex_32, stable_token};
use crate::{
    PERSON_ASSERTION_SCHEMA, PersonAssertion, Validation, validate_information_coordinate,
};
use std::collections::BTreeSet;

#[must_use]
pub fn validate_person_assertion(assertion: &PersonAssertion) -> Validation {
    let mut findings = validate_information_coordinate(&assertion.coordinate).findings;
    if assertion.schema != PERSON_ASSERTION_SCHEMA
        || !stable_token(&assertion.assertion_id)
        || assertion.revision == 0
        || !stable_token(&assertion.scope_ref)
        || !stable_token(&assertion.subject_id)
        || !stable_token(&assertion.producer_id)
        || !lower_hex_32(&assertion.projection_digest_sha256)
        || assertion.recorded_at_epoch_s == 0
    {
        findings.push("person assertion identity is invalid".to_owned());
    }
    validate_times(assertion, &mut findings);
    validate_links(assertion, &mut findings);
    validate_authority(assertion, &mut findings);
    validate_evidence(assertion, &mut findings);
    validation(findings)
}

fn validate_times(assertion: &PersonAssertion, findings: &mut Vec<String>) {
    if assertion
        .observed_at_epoch_s
        .is_some_and(|value| value > assertion.recorded_at_epoch_s)
        || assertion
            .source_issued_at_epoch_s
            .is_some_and(|value| value > assertion.recorded_at_epoch_s)
        || matches!(
            (assertion.valid_from_epoch_s, assertion.valid_until_epoch_s),
            (None, Some(_))
        )
        || matches!(
            (assertion.valid_from_epoch_s, assertion.valid_until_epoch_s),
            (Some(start), Some(end)) if end < start
        )
    {
        findings.push("person assertion time is invalid".to_owned());
    }
}

fn validate_links(assertion: &PersonAssertion, findings: &mut Vec<String>) {
    let conflicts = assertion
        .conflicting_assertion_ids
        .iter()
        .collect::<BTreeSet<_>>();
    let invalid_conflicts = assertion.conflicting_assertion_ids.len() > 32
        || conflicts.len() != assertion.conflicting_assertion_ids.len()
        || assertion
            .conflicting_assertion_ids
            .iter()
            .any(|id| !stable_token(id) || id == &assertion.assertion_id);
    if invalid_conflicts
        || (matches!(assertion.knowledge_state, crate::KnowledgeState::Conflicted)
            != !assertion.conflicting_assertion_ids.is_empty())
    {
        findings.push("person assertion conflict links are invalid".to_owned());
    }
    for link in [
        &assertion.supersedes_assertion_id,
        &assertion.superseded_by_assertion_id,
    ] {
        if link
            .as_ref()
            .is_some_and(|id| !stable_token(id) || id == &assertion.assertion_id)
        {
            findings.push("person assertion correction link is invalid".to_owned());
        }
    }
    let superseded = matches!(assertion.knowledge_state, crate::KnowledgeState::Superseded);
    if superseded != assertion.superseded_by_assertion_id.is_some()
        || (superseded && assertion.supersedes_assertion_id.is_some())
    {
        findings.push("person assertion supersession state is invalid".to_owned());
    }
}

fn validate_authority(assertion: &PersonAssertion, findings: &mut Vec<String>) {
    let confirmed = matches!(assertion.knowledge_state, crate::KnowledgeState::Confirmed);
    if confirmed != assertion.confirmation_authority_id.is_some()
        || assertion
            .confirmation_authority_id
            .as_ref()
            .is_some_and(|id| !stable_token(id))
    {
        findings.push("person assertion confirmation authority is invalid".to_owned());
    }
}

fn validate_evidence(assertion: &PersonAssertion, findings: &mut Vec<String>) {
    if assertion.evidence_refs.is_empty() || assertion.evidence_refs.len() > 64 {
        findings.push("person assertion evidence count is invalid".to_owned());
    }
    for evidence in &assertion.evidence_refs {
        if !stable_token(&evidence.owner_id)
            || evidence.reference.is_empty()
            || evidence.reference.len() > 512
            || !lower_hex_32(&evidence.digest_sha256)
        {
            findings.push("person assertion evidence is invalid".to_owned());
        }
    }
}
