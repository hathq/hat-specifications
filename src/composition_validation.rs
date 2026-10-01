use std::collections::{BTreeMap, BTreeSet};

use crate::composition_graph::validate_edges;
use crate::model::validation;
use crate::tokens::{lower_hex_32, schema_uri, stable_token};
use crate::{
    COMPOSITION_APPROVAL_SCHEMA, COMPOSITION_PROPOSAL_SCHEMA, HatCompositionApproval,
    HatCompositionMember, HatCompositionProposal, Validation,
};

#[must_use]
pub fn validate_composition_proposal(value: &HatCompositionProposal) -> Validation {
    let mut findings = Vec::new();
    if value.schema != COMPOSITION_PROPOSAL_SCHEMA
        || !stable_token(&value.proposal_id)
        || !reference(&value.subject_ref)
        || !reference(&value.scope_ref)
        || value.members.is_empty()
        || value.members.len() > 64
    {
        findings.push("composition proposal identity or bounds are invalid".into());
    }
    let mut members = BTreeMap::new();
    let mut operations = BTreeSet::new();
    for member in &value.members {
        validate_member(member, &mut findings);
        if members.insert(&member.repository_id, member).is_some() {
            findings.push("composition repository IDs must be unique".into());
        }
        for operation in &member.operation_ids {
            if !operations.insert(operation) {
                findings.push("composition operation ownership must be unique".into());
            }
        }
    }
    validate_edges(&members, &mut findings);
    validation(findings)
}

#[must_use]
pub fn validate_composition_approval(value: &HatCompositionApproval) -> Validation {
    let mut findings = Vec::new();
    if value.schema != COMPOSITION_APPROVAL_SCHEMA
        || !stable_token(&value.approval_id)
        || !lower_hex_32(&value.proposal_digest_sha256)
        || !reference(&value.approved_by_subject_ref)
    {
        findings.push("composition approval is invalid".into());
    }
    validation(findings)
}

fn validate_member(value: &HatCompositionMember, findings: &mut Vec<String>) {
    if !stable_token(&value.repository_id)
        || !value.package_id.starts_with("hat/")
        || !lower_hex_32(&value.package_digest_sha256)
        || !lower_hex_32(&value.catalog_digest_sha256)
        || !lower_hex_32(&value.fitting_digest_sha256)
        || !lower_hex_32(&value.policy_digest_sha256)
        || value.operation_ids.is_empty()
        || value.operation_ids.len() > 128
    {
        findings.push("composition member is invalid".into());
    }
    sorted_tokens(&value.dependency_repository_ids, findings);
    sorted_tokens(&value.incompatible_repository_ids, findings);
    if !strict_sorted(&value.operation_ids) || value.operation_ids.iter().any(|id| !schema_uri(id))
    {
        findings.push("composition operation IDs must be sorted canonical schemas".into());
    }
}

fn sorted_tokens(values: &[String], findings: &mut Vec<String>) {
    if values.len() > 64 || !strict_sorted(values) || values.iter().any(|id| !stable_token(id)) {
        findings.push("composition relationship IDs must be sorted stable tokens".into());
    }
}

fn strict_sorted(values: &[String]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}

fn reference(value: &str) -> bool {
    !value.is_empty() && value.len() <= 256 && value.trim() == value
}
