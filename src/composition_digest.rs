use sha2::{Digest, Sha256};
use std::fmt::Write;

use crate::{HatCompositionProposal, validate_composition_proposal};

/// Returns the domain-separated digest of one valid, field-ordered proposal.
///
/// # Errors
///
/// Rejects a proposal before hashing when its graph is invalid or ambiguous.
pub fn composition_proposal_digest(value: &HatCompositionProposal) -> Result<String, &'static str> {
    if !validate_composition_proposal(value).valid {
        return Err("composition-proposal-invalid");
    }
    let body = serde_json::to_vec(value).map_err(|_| "composition-proposal-invalid")?;
    let mut hasher = Sha256::new();
    hasher.update(b"hathq:hat-composition-proposal:v1\0");
    hasher.update(body);
    let digest = hasher.finalize();
    let mut encoded = String::with_capacity(64);
    for byte in digest {
        write!(&mut encoded, "{byte:02x}").map_err(|_| "composition-proposal-invalid")?;
    }
    Ok(encoded)
}
