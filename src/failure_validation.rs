use crate::model::validation;
use crate::tokens::{lower_hex_32, schema_uri, stable_token};
use crate::{
    FAILURE_ENVELOPE_SCHEMA, HatActionResult, HatFailureEnvelope, HatFailureRecovery,
    HatFailureResponsibility, HatInvocationOutcome, SemanticCatalog, SemanticTermKind, Validation,
    validate_action_result,
};

#[must_use]
pub fn validate_failure_envelope(value: &HatFailureEnvelope) -> Validation {
    let mut findings = Vec::new();
    if value.schema != FAILURE_ENVELOPE_SCHEMA
        || !stable_token(&value.failure_id)
        || !stable_token(&value.invocation_id)
        || !stable_token(&value.component_id)
        || !schema_uri(&value.operation_id)
        || !value.reason_id.starts_with("hathq://vocabulary/reason/")
        || !schema_uri(&value.reason_id)
    {
        findings.push("failure identity is invalid".to_owned());
    }
    if value.parameters.len() > 16
        || value
            .parameters
            .iter()
            .any(|(key, parameter)| !stable_token(key) || !stable_token(parameter))
    {
        findings.push("failure parameters are invalid".to_owned());
    }
    if value.evidence_refs.len() > 64
        || value.evidence_refs.iter().any(|evidence| {
            !stable_token(&evidence.owner_id)
                || !stable_token(&evidence.reference)
                || !lower_hex_32(&evidence.digest_sha256)
        })
    {
        findings.push("failure evidence is invalid".to_owned());
    }
    if value.next_action_id.as_ref().is_some_and(|action| {
        !action.starts_with("hathq://vocabulary/action/") || !schema_uri(action)
    }) || matches!(value.recovery, HatFailureRecovery::OwnerAction)
        != value.next_action_id.is_some()
        || matches!(value.recovery, HatFailureRecovery::OwnerAction)
            != matches!(value.responsibility, HatFailureResponsibility::Owner)
    {
        findings.push("failure recovery is invalid".to_owned());
    }
    validation(findings)
}

/// Validates a failure against the exact vocabulary shipped by its producing HAT.
///
/// A syntactically valid URI is not authority: the reason and optional owner action must be
/// declared with the matching kind in the accepted package catalog.
#[must_use]
pub fn validate_failure_envelope_with_catalog(
    value: &HatFailureEnvelope,
    catalog: &SemanticCatalog,
) -> Validation {
    let mut findings = validate_failure_envelope(value).findings;
    if let Err(finding) = crate::validate_semantic_catalog(catalog) {
        findings.push(finding);
    }
    let reason_matches = catalog
        .terms
        .iter()
        .filter(|term| {
            term.reference.term_id == value.reason_id
                && term.reference.kind == SemanticTermKind::Reason
        })
        .count();
    if reason_matches != 1 {
        findings.push("failure reason is not an exact accepted vocabulary term".to_owned());
    }
    if let Some(action_id) = &value.next_action_id {
        let action_matches = catalog
            .terms
            .iter()
            .filter(|term| {
                term.reference.term_id == *action_id
                    && term.reference.kind == SemanticTermKind::Action
            })
            .count();
        if action_matches != 1 {
            findings.push("failure action is not an exact accepted vocabulary term".to_owned());
        }
    }
    validation(findings)
}

#[must_use]
pub fn validate_action_failure(
    result: &HatActionResult,
    failure: &HatFailureEnvelope,
) -> Validation {
    let mut findings = validate_action_result(result).findings;
    findings.extend(validate_failure_envelope(failure).findings);
    if matches!(result.outcome, HatInvocationOutcome::Completed)
        || result.output.is_some()
        || result.reason_id.as_deref() != Some(failure.reason_id.as_str())
        || result.invocation_id != failure.invocation_id
        || result.operation_id != failure.operation_id
        || result.state_revision != failure.state_revision
    {
        findings.push("action failure correlation is invalid".to_owned());
    }
    crate::model::validation(findings)
}

/// Validates terminal failure correlation and exact package-vocabulary membership together.
#[must_use]
pub fn validate_action_failure_with_catalog(
    result: &HatActionResult,
    failure: &HatFailureEnvelope,
    catalog: &SemanticCatalog,
) -> Validation {
    let mut findings = validate_action_failure(result, failure).findings;
    findings.extend(validate_failure_envelope_with_catalog(failure, catalog).findings);
    findings.sort();
    findings.dedup();
    validation(findings)
}
