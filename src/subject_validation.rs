// Added by the HAT Specifications project, 2026.
// Purpose: reject ambiguous, aliased, or malformed subject references.

use crate::model::validation;
use crate::tokens::stable_token;
use crate::{SUBJECT_REFERENCE_SCHEMA, SubjectReference, Validation};

#[must_use]
pub fn validate_subject_reference(subject: &SubjectReference) -> Validation {
    let mut findings = Vec::new();
    if subject.schema != SUBJECT_REFERENCE_SCHEMA
        || !stable_token(&subject.subject_id)
        || !stable_token(&subject.identity_authority_id)
        || subject.identity_revision == 0
    {
        findings.push("subject identity is invalid".to_owned());
    }
    if !opaque_reference(&subject.identity_reference) {
        findings.push("subject identity reference is invalid".to_owned());
    }
    validation(findings)
}

/// Validates that two references cannot name the same subject through an alias.
#[must_use]
pub fn validate_distinct_subjects(left: &SubjectReference, right: &SubjectReference) -> Validation {
    let mut findings = validate_subject_reference(left).findings;
    findings.extend(validate_subject_reference(right).findings);
    if left.subject_id == right.subject_id {
        findings.push("distinct subjects reuse one subject_id".to_owned());
    }
    if left.identity_authority_id == right.identity_authority_id
        && left.identity_reference == right.identity_reference
    {
        findings.push("distinct subjects alias one authority identity".to_owned());
    }
    validation(findings)
}

fn opaque_reference(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 256
        && value.trim() == value
        && value.bytes().all(|byte| !byte.is_ascii_control())
}
