use std::collections::BTreeSet;

use crate::federation_validation_support::{
    classifications, hat_id, reference, unique_schemas, unique_tokens,
};
use crate::tokens::{lower_hex_32, stable_token};
use crate::{
    EXECUTION_LOCATION_SCHEMA, EXECUTION_LOCATION_SET_SCHEMA, FEDERATION_RECEIPT_SCHEMA,
    HatActionResult, HatExecutionLocation, HatExecutionLocationSet, HatFederationExecutionReceipt,
    HatPlacementSelection, PLACEMENT_SELECTION_SCHEMA, Validation, validate_action_result,
};

#[must_use]
pub fn validate_execution_location(value: &HatExecutionLocation, now: u64) -> Validation {
    let mut findings = Vec::new();
    if value.schema != EXECUTION_LOCATION_SCHEMA
        || !stable_token(&value.location_id)
        || !hat_id(&value.package_id)
        || !lower_hex_32(&value.package_digest_sha256)
        || !stable_token(&value.publisher_id)
        || !stable_token(&value.worker_service_id)
        || !reference(&value.identity_authority_ref, "ihat/")
        || !value.evidence_recovery.valid_declaration()
        || !reference(&value.transport_profile_ref, "crowsi/")
        || !reference(&value.route_ref, "crowsi/")
        || value
            .region
            .as_ref()
            .is_some_and(|item| !stable_token(item))
        || value.assurance != "official" && value.assurance != "verified"
        || value.issued_at_epoch_s > now
        || now >= value.expires_at_epoch_s
        || value.expires_at_epoch_s <= value.issued_at_epoch_s
        || value.expires_at_epoch_s - value.issued_at_epoch_s > 86_400
    {
        findings.push("execution location identity, trust, or lifetime is invalid".into());
    }
    unique_tokens(&value.jurisdictions, 16, &mut findings);
    unique_tokens(&value.data_residencies, 16, &mut findings);
    unique_tokens(&value.capability_ids, 64, &mut findings);
    unique_schemas(&value.operation_ids, 128, &mut findings);
    classifications(&value.accepted_classifications, &mut findings);
    Validation {
        valid: findings.is_empty(),
        findings,
    }
}

#[must_use]
pub fn validate_execution_location_set(value: &HatExecutionLocationSet, now: u64) -> Validation {
    let mut findings = Vec::new();
    if value.schema != EXECUTION_LOCATION_SET_SCHEMA
        || !hat_id(&value.package_id)
        || !lower_hex_32(&value.package_digest_sha256)
        || value.revision == 0
        || value.issued_at_epoch_s > now
        || now >= value.expires_at_epoch_s
        || value.expires_at_epoch_s <= value.issued_at_epoch_s
        || value.expires_at_epoch_s - value.issued_at_epoch_s > 86_400
        || value.locations.is_empty()
        || value.locations.len() > 64
    {
        findings.push("execution location set header is invalid".into());
    }
    let mut ids = BTreeSet::new();
    for location in &value.locations {
        findings.extend(validate_execution_location(location, now).findings);
        if location.package_id != value.package_id
            || location.package_digest_sha256 != value.package_digest_sha256
            || location.issued_at_epoch_s < value.issued_at_epoch_s
            || location.expires_at_epoch_s > value.expires_at_epoch_s
            || !ids.insert(location.location_id.as_str())
        {
            findings.push("execution location set member is invalid".into());
        }
    }
    Validation {
        valid: findings.is_empty(),
        findings,
    }
}

#[must_use]
pub fn validate_placement_selection(value: &HatPlacementSelection) -> Validation {
    let mut findings = Vec::new();
    if value.schema != PLACEMENT_SELECTION_SCHEMA
        || !stable_token(&value.selection_id)
        || !stable_token(&value.context_partition_id)
        || !hat_id(&value.package_id)
        || !stable_token(&value.location_id)
        || value.revision == 0
        || value.selected_at_epoch_s == 0
    {
        findings.push("placement selection identity or revision is invalid".into());
    }
    for digest in [
        &value.package_digest_sha256,
        &value.location_digest_sha256,
        &value.route_policy_digest_sha256,
    ] {
        if !lower_hex_32(digest) {
            findings.push("placement selection digest is invalid".into());
        }
    }
    classifications(&value.permitted_classifications, &mut findings);
    let failover = &value.approved_failover_location_digests;
    let unique = failover.iter().collect::<BTreeSet<_>>();
    if failover.len() > 8
        || unique.len() != failover.len()
        || failover
            .iter()
            .any(|item| !lower_hex_32(item) || item == &value.location_digest_sha256)
    {
        findings.push("placement failover set is invalid".into());
    }
    Validation {
        valid: findings.is_empty(),
        findings,
    }
}

#[must_use]
pub fn validate_federation_execution_receipt(
    value: &HatFederationExecutionReceipt,
    result: &HatActionResult,
) -> Validation {
    let mut findings = validate_action_result(result).findings;
    if value.schema != FEDERATION_RECEIPT_SCHEMA
        || !stable_token(&value.receipt_id)
        || !stable_token(&value.invocation_id)
        || value.invocation_id != result.invocation_id
        || !stable_token(&value.worker_service_id)
        || !reference(&value.worker_identity_ref, "ihat/")
        || value.completed_at_epoch_s == 0
    {
        findings.push("federation execution receipt identity is invalid".into());
    }
    for digest in [
        &value.invocation_digest_sha256,
        &value.placement_selection_digest_sha256,
        &value.location_digest_sha256,
        &value.transport_receipt_digest_sha256,
        &value.result_digest_sha256,
    ] {
        if !lower_hex_32(digest) {
            findings.push("federation execution receipt digest is invalid".into());
        }
    }
    Validation {
        valid: findings.is_empty(),
        findings,
    }
}

#[must_use]
pub fn validate_federation_execution_receipt_against(
    value: &HatFederationExecutionReceipt,
    result: &HatActionResult,
    invocation_digest_sha256: &str,
    placement: &HatPlacementSelection,
    location: &HatExecutionLocation,
) -> Validation {
    let mut findings = validate_federation_execution_receipt(value, result).findings;
    let result_digest = crate::action_result_digest(result).ok();
    let placement_digest = crate::placement_selection_digest(placement).ok();
    let location_digest = crate::execution_location_digest(location).ok();
    if value.invocation_digest_sha256 != invocation_digest_sha256
        || placement_digest.as_deref() != Some(&value.placement_selection_digest_sha256)
        || location_digest.as_deref() != Some(&value.location_digest_sha256)
        || result_digest.as_deref() != Some(&value.result_digest_sha256)
        || value.worker_service_id != location.worker_service_id
        || value.worker_identity_ref != location.identity_authority_ref
    {
        findings.push("federation execution receipt correlation is invalid".into());
    }
    Validation {
        valid: findings.is_empty(),
        findings,
    }
}
