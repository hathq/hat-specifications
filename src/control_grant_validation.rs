// Added by the HAT Specifications project, 2026.
// Purpose: reject quorum, ambient authority, scope widening, and unbound grants.

use crate::model::validation;
use crate::tokens::{lower_hex_32, schema_uri, stable_token};
use crate::{CONTROL_GRANT_SCHEMA, ControlGrant, ControlGrantState, Validation};
use std::collections::BTreeSet;

#[must_use]
pub fn validate_control_grant(grant: &ControlGrant) -> Validation {
    let mut findings = Vec::new();
    if grant.schema != CONTROL_GRANT_SCHEMA
        || !stable_token(&grant.grant_id)
        || grant.revision == 0
        || !stable_token(&grant.operator_subject_id)
        || !stable_token(&grant.represented_subject_id)
        || grant.operator_subject_id == grant.represented_subject_id
        || !stable_token(&grant.position_id)
        || grant.position_revision == 0
        || !capacity_term(&grant.capacity_term_id)
        || grant.valid_until_epoch_s <= grant.valid_from_epoch_s
    {
        findings.push("control grant identity or interval is invalid".to_owned());
    }
    validate_scope(grant, &mut findings);
    validate_decision(grant, &mut findings);
    validate_state(grant, &mut findings);
    validate_evidence(grant, &mut findings);
    validation(findings)
}

#[must_use]
pub fn validate_control_grant_revocation(
    previous: &ControlGrant,
    revoked: &ControlGrant,
) -> Validation {
    let mut findings = validate_control_grant(previous).findings;
    findings.extend(validate_control_grant(revoked).findings);
    let mut expected = previous.clone();
    expected.revision = previous.revision.saturating_add(1);
    expected.state = ControlGrantState::Revoked;
    expected.revoked_at_epoch_s = revoked.revoked_at_epoch_s;
    if previous.state != ControlGrantState::Active || revoked != &expected {
        findings.push("control grant revocation is not an exact transition".to_owned());
    }
    validation(findings)
}

fn validate_scope(grant: &ControlGrant, findings: &mut Vec<String>) {
    let unique = grant.action_scope_ids.iter().collect::<BTreeSet<_>>();
    if grant.action_scope_ids.is_empty()
        || grant.action_scope_ids.len() > 64
        || unique.len() != grant.action_scope_ids.len()
        || grant
            .action_scope_ids
            .iter()
            .any(|id| !schema_uri(id) || !id.starts_with("hathq://vocabulary/action/"))
    {
        findings.push("control grant action scope is invalid".to_owned());
    }
}

fn validate_decision(grant: &ControlGrant, findings: &mut Vec<String>) {
    let decision = &grant.decision;
    if !stable_token(&decision.decision_id)
        || decision.decided_by_subject_id != grant.operator_subject_id
        || decision.decision_revision == 0
        || !lower_hex_32(&decision.decision_digest_sha256)
    {
        findings.push("control grant owner decision is invalid".to_owned());
    }
}

fn validate_state(grant: &ControlGrant, findings: &mut Vec<String>) {
    match (grant.state, grant.revoked_at_epoch_s) {
        (ControlGrantState::Active, None) => {}
        (ControlGrantState::Revoked, Some(revoked))
            if revoked >= grant.valid_from_epoch_s && revoked <= grant.valid_until_epoch_s => {}
        _ => findings.push("control grant state is invalid".to_owned()),
    }
}

fn validate_evidence(grant: &ControlGrant, findings: &mut Vec<String>) {
    if grant.evidence_refs.is_empty() || grant.evidence_refs.len() > 32 {
        findings.push("control grant evidence count is invalid".to_owned());
    }
    for evidence in &grant.evidence_refs {
        if !stable_token(&evidence.owner_id)
            || evidence.reference.is_empty()
            || evidence.reference.len() > 512
            || !lower_hex_32(&evidence.digest_sha256)
        {
            findings.push("control grant evidence is invalid".to_owned());
        }
    }
}

fn capacity_term(value: &str) -> bool {
    schema_uri(value) && value.starts_with("hathq://vocabulary/capacity/")
}
