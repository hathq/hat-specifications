// Added by the HAT Specifications project, 2026.
// Purpose: validate local registry provenance and external-schema semantic mappings fail-closed.

use crate::model::validation;
use crate::tokens::{lower_hex_32, schema_uri, stable_token};
use crate::{
    LOCAL_REGISTRY_LEDGER_SCHEMA, LocalRegistryLedger, LocalRegistryVerificationState,
    SEMANTIC_MAPPING_CONTRACT_SCHEMA, SemanticMappingContract, Validation,
};

#[must_use]
pub fn validate_local_registry_ledger(value: &LocalRegistryLedger) -> Validation {
    let mut findings = Vec::new();
    if value.schema != LOCAL_REGISTRY_LEDGER_SCHEMA
        || !stable_token(&value.ledger_id)
        || value.revision == 0
        || !safe_reference(&value.owner_ref)
    {
        findings.push("local registry ledger identity is invalid".to_owned());
    }
    if value.entries.len() > 8192 {
        findings.push("local registry ledger contains too many entries".to_owned());
    }
    let mut ids = std::collections::BTreeSet::new();
    for entry in &value.entries {
        if !stable_token(&entry.entry_id) || !ids.insert(entry.entry_id.as_str()) {
            findings.push("local registry entry IDs must be unique stable tokens".to_owned());
        }
        if entry.recorded_at_epoch_s == 0 {
            findings.push("local registry entry time is invalid".to_owned());
        }
        if !safe_reference(&entry.source.reference)
            || entry
                .source
                .source_schema
                .as_ref()
                .is_some_and(|schema| !schema_uri(schema))
            || entry
                .source
                .source_digest_sha256
                .as_ref()
                .is_some_and(|digest| !lower_hex_32(digest))
        {
            findings.push("local registry source is invalid".to_owned());
        }
        if !stable_token(&entry.artifact.artifact_id)
            || !lower_hex_32(&entry.artifact.digest_sha256)
            || entry.artifact.size_bytes == 0
            || !safe_reference(&entry.artifact.local_ref)
            || entry
                .artifact
                .media_type
                .as_ref()
                .is_some_and(|value| !media_type(value))
        {
            findings.push("local registry artifact is invalid".to_owned());
        }
        validate_verification(entry, &mut findings);
        if entry.published_as.len() > 16 {
            findings.push("local registry publication list is too large".to_owned());
        }
        for publication in &entry.published_as {
            if !safe_reference(&publication.reference)
                || !lower_hex_32(&publication.digest_sha256)
                || publication.published_at_epoch_s == 0
            {
                findings.push("local registry publication is invalid".to_owned());
            }
        }
    }
    validation(findings)
}

fn validate_verification(entry: &crate::LocalRegistryEntry, findings: &mut Vec<String>) {
    let verification = &entry.verification;
    if !safe_reference(&verification.verifier_ref)
        || verification.reason_id.as_ref().is_some_and(|reason| {
            !schema_uri(reason) || !reason.starts_with("hathq://vocabulary/reason/")
        })
        || verification.evidence_refs.len() > 64
    {
        findings.push("local registry verification is invalid".to_owned());
    }
    let has_verified_time = verification
        .verified_at_epoch_s
        .is_some_and(|value| value > 0);
    if matches!(verification.state, LocalRegistryVerificationState::Verified) != has_verified_time {
        findings
            .push("verified local registry entries require an exact verification time".to_owned());
    }
    if matches!(verification.state, LocalRegistryVerificationState::Rejected)
        && verification.reason_id.is_none()
    {
        findings.push("rejected local registry entries require a reason".to_owned());
    }
}

#[must_use]
pub fn validate_semantic_mapping_contract(value: &SemanticMappingContract) -> Validation {
    let mut findings = Vec::new();
    if value.schema != SEMANTIC_MAPPING_CONTRACT_SCHEMA
        || !stable_token(&value.contract_id)
        || value.revision == 0
        || !stable_token(&value.owner_repository_id)
        || !schema_uri(&value.external_schema)
        || value.target_catalogs.is_empty()
        || value.target_catalogs.len() > 64
        || value.field_mappings.is_empty()
        || value.field_mappings.len() > 512
    {
        findings.push("semantic mapping contract identity is invalid".to_owned());
    }
    for catalog in &value.target_catalogs {
        if let Err(finding) = crate::validate_semantic_catalog_reference(catalog) {
            findings.push(finding);
        }
    }
    let mut ids = std::collections::BTreeSet::new();
    for mapping in &value.field_mappings {
        if !stable_token(&mapping.mapping_id)
            || !ids.insert(mapping.mapping_id.as_str())
            || !json_pointer(&mapping.source_pointer)
            || crate::validate_semantic_term_reference(&mapping.target_term).is_err()
            || !stable_token(&mapping.target_field)
        {
            findings.push("semantic field mapping is invalid".to_owned());
        }
    }
    validation(findings)
}

fn safe_reference(value: &str) -> bool {
    !value.trim().is_empty()
        && value == value.trim()
        && value.len() <= 512
        && !value.chars().any(char::is_control)
        && !contains_secret_word(value)
}

fn contains_secret_word(value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();
    ["token", "secret", "password", "private-key", "access-key"]
        .iter()
        .any(|word| normalized.contains(word))
}

fn media_type(value: &str) -> bool {
    value.len() <= 128
        && value.split_once('/').is_some()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'.' | b'+' | b'-'))
}

fn json_pointer(value: &str) -> bool {
    value == "$"
        || value.strip_prefix("$.").is_some_and(|body| {
            !body.is_empty()
                && body.len() <= 256
                && body
                    .split('.')
                    .all(|segment| !segment.is_empty() && source_field(segment))
        })
}

fn source_field(value: &str) -> bool {
    value.len() <= 96
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}
