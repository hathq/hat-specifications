use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::{
    HatActionResult, HatExecutionLocation, HatFederationExecutionReceipt, HatInvocation,
    HatPlacementSelection,
};

const LOCATION_DOMAIN: &[u8] = b"hathq/hat-execution-location/v1\0";
const PLACEMENT_DOMAIN: &[u8] = b"hathq/hat-placement-selection/v1\0";
const RECEIPT_DOMAIN: &[u8] = b"hathq/hat-federation-execution-receipt/v1\0";
const INVOCATION_DOMAIN: &[u8] = b"hathq/hat-invocation/v2\0";
const RESULT_DOMAIN: &[u8] = b"hathq/hat-action-result/v1\0";

/// Hash one execution location with a type-specific domain separator.
///
/// # Errors
///
/// Returns an error when the closed contract value cannot be serialized.
pub fn execution_location_digest(
    value: &HatExecutionLocation,
) -> Result<String, serde_json::Error> {
    digest(LOCATION_DOMAIN, value)
}

/// Hash one immutable execution placement selection.
///
/// # Errors
///
/// Returns an error when the closed contract value cannot be serialized.
pub fn placement_selection_digest(
    value: &HatPlacementSelection,
) -> Result<String, serde_json::Error> {
    digest(PLACEMENT_DOMAIN, value)
}

/// Hash one exact federation execution receipt.
///
/// # Errors
///
/// Returns an error when the closed contract value cannot be serialized.
pub fn federation_execution_receipt_digest(
    value: &HatFederationExecutionReceipt,
) -> Result<String, serde_json::Error> {
    digest(RECEIPT_DOMAIN, value)
}

/// Hash one exact HAT invocation for receipt correlation.
///
/// # Errors
///
/// Returns an error when the invocation cannot be serialized.
pub fn invocation_digest(value: &HatInvocation) -> Result<String, serde_json::Error> {
    digest(INVOCATION_DOMAIN, value)
}

/// Hash one exact HAT result for receipt correlation.
///
/// # Errors
///
/// Returns an error when the result cannot be serialized.
pub fn action_result_digest(value: &HatActionResult) -> Result<String, serde_json::Error> {
    digest(RESULT_DOMAIN, value)
}

fn digest<T: Serialize>(domain: &[u8], value: &T) -> Result<String, serde_json::Error> {
    let bytes = serde_json::to_vec(value)?;
    let mut hash = Sha256::new();
    hash.update(domain);
    hash.update(bytes);
    Ok(format!("{:x}", hash.finalize()))
}
