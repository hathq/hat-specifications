// Added by the HAT Specifications project, 2026.
// Purpose: validate attach, close, supersede, and conflict invariants.

use crate::model::validation;
use crate::tokens::{lower_hex_32, schema_uri, stable_token};
use crate::{POSITION_RECORD_SCHEMA, PositionChangeKind, PositionRecord, Validation};

#[must_use]
pub fn validate_position_record(record: &PositionRecord) -> Validation {
    let mut findings = Vec::new();
    if record.schema != POSITION_RECORD_SCHEMA
        || !stable_token(&record.position_id)
        || record.revision == 0
        || !stable_token(&record.holder_subject_id)
        || !stable_token(&record.context_subject_id)
        || record.holder_subject_id == record.context_subject_id
        || !schema_uri(&record.position_term_id)
        || !record
            .position_term_id
            .starts_with("hathq://vocabulary/position/")
    {
        findings.push("position identity is invalid".to_owned());
    }
    validate_change(record, &mut findings);
    validate_evidence(record, &mut findings);
    validation(findings)
}

#[must_use]
pub fn validate_position_transition(
    previous: &PositionRecord,
    next: &PositionRecord,
) -> Validation {
    let mut findings = validate_position_record(previous).findings;
    findings.extend(validate_position_record(next).findings);
    if previous.change == PositionChangeKind::Close
        || next.change == PositionChangeKind::Attach
        || next.position_id != previous.position_id
        || next.revision != previous.revision.saturating_add(1)
        || next.previous_revision != Some(previous.revision)
        || next.holder_subject_id != previous.holder_subject_id
        || next.context_subject_id != previous.context_subject_id
    {
        findings.push("position transition is invalid".to_owned());
    }
    if next.change == PositionChangeKind::Close
        && next.position_term_id != previous.position_term_id
    {
        findings.push("position close changes the position term".to_owned());
    }
    validation(findings)
}

#[must_use]
pub fn validate_non_conflicting_position_records(
    left: &PositionRecord,
    right: &PositionRecord,
) -> Validation {
    let mut findings = validate_position_record(left).findings;
    findings.extend(validate_position_record(right).findings);
    if left.position_id == right.position_id && left.revision == right.revision && left != right {
        findings.push("position revision has conflicting records".to_owned());
    }
    validation(findings)
}

fn validate_change(record: &PositionRecord, findings: &mut Vec<String>) {
    match record.change {
        PositionChangeKind::Attach => {
            if record.revision != 1
                || record.previous_revision.is_some()
                || record.valid_until_epoch_s.is_some()
            {
                findings.push("position attach is invalid".to_owned());
            }
        }
        PositionChangeKind::Close => {
            if record.revision <= 1
                || record.previous_revision != record.revision.checked_sub(1)
                || record
                    .valid_until_epoch_s
                    .is_none_or(|end| end < record.valid_from_epoch_s)
            {
                findings.push("position close is invalid".to_owned());
            }
        }
        PositionChangeKind::Supersede => {
            if record.revision <= 1
                || record.previous_revision != record.revision.checked_sub(1)
                || record.valid_until_epoch_s.is_some()
            {
                findings.push("position supersede is invalid".to_owned());
            }
        }
    }
}

fn validate_evidence(record: &PositionRecord, findings: &mut Vec<String>) {
    if record.evidence_refs.is_empty() || record.evidence_refs.len() > 32 {
        findings.push("position evidence count is invalid".to_owned());
    }
    for evidence in &record.evidence_refs {
        if !stable_token(&evidence.owner_id)
            || evidence.reference.is_empty()
            || evidence.reference.len() > 512
            || !lower_hex_32(&evidence.digest_sha256)
        {
            findings.push("position evidence is invalid".to_owned());
        }
    }
}
