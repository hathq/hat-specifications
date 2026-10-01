// Added by the HAT Specifications project, 2026.
// Purpose: digest application catalogs and declarations without self-referential digest fields.

use crate::{SemanticCatalog, SemanticClauseFrame, SemanticExchange, SemanticTermDefinition};
use sha2::{Digest, Sha256};
use std::fmt::Write as _;

/// Computes the canonical exchange digest with its digest field cleared.
///
/// # Errors
///
/// Returns an error when canonical JSON serialization fails.
pub fn semantic_exchange_digest(value: &SemanticExchange) -> Result<String, String> {
    let mut canonical = value.clone();
    canonical.content_digest_sha256.clear();
    digest(&canonical)
}

/// Computes a catalog digest independent of embedded self-references.
///
/// # Errors
///
/// Returns an error when canonical JSON serialization fails.
pub fn semantic_catalog_digest(value: &SemanticCatalog) -> Result<String, String> {
    let mut canonical = value.clone();
    canonical.identity.digest_sha256.clear();
    for definition in &mut canonical.terms {
        clear_definition_catalog_digests(definition);
    }
    for lexicalization in &mut canonical.lexicalizations {
        lexicalization.term.catalog_digest_sha256.clear();
    }
    for frame in &mut canonical.frames {
        frame.predicate.catalog_digest_sha256.clear();
    }
    digest(&canonical)
}

/// Computes a definition digest independent of embedded self-references.
///
/// # Errors
///
/// Returns an error when canonical JSON serialization fails.
pub fn semantic_term_definition_digest(value: &SemanticTermDefinition) -> Result<String, String> {
    let mut canonical = value.clone();
    canonical.reference.definition_digest_sha256.clear();
    clear_definition_catalog_digests(&mut canonical);
    digest(&canonical)
}

/// Computes a clause-frame digest independent of its embedded digest and catalog self-reference.
///
/// # Errors
///
/// Returns an error when canonical JSON serialization fails.
pub fn semantic_clause_frame_digest(value: &SemanticClauseFrame) -> Result<String, String> {
    let mut canonical = value.clone();
    canonical.definition_digest_sha256.clear();
    canonical.predicate.catalog_digest_sha256.clear();
    digest(&canonical)
}

fn clear_definition_catalog_digests(value: &mut SemanticTermDefinition) {
    value.reference.catalog_digest_sha256.clear();
    clear_terms(&mut value.dependencies);
    clear_terms(&mut value.is_a);
    clear_terms(&mut value.domain);
    clear_terms(&mut value.range);
}

fn clear_terms(values: &mut std::collections::BTreeSet<crate::SemanticTermReference>) {
    *values = std::mem::take(values)
        .into_iter()
        .map(|mut term| {
            term.catalog_digest_sha256.clear();
            term
        })
        .collect();
}

fn digest(value: &impl serde::Serialize) -> Result<String, String> {
    let value = serde_json::to_value(value).map_err(|error| error.to_string())?;
    let mut bytes = Vec::new();
    canonical_json(&value, &mut bytes)?;
    let mut result = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        write!(result, "{byte:02x}").map_err(|error| error.to_string())?;
    }
    Ok(result)
}

fn canonical_json(value: &serde_json::Value, output: &mut Vec<u8>) -> Result<(), String> {
    match value {
        serde_json::Value::Null => output.extend_from_slice(b"null"),
        serde_json::Value::Bool(value) => output.extend_from_slice(value.to_string().as_bytes()),
        serde_json::Value::Number(value) => output.extend_from_slice(value.to_string().as_bytes()),
        serde_json::Value::String(value) => output.extend_from_slice(
            serde_json::to_string(value)
                .map_err(|error| error.to_string())?
                .as_bytes(),
        ),
        serde_json::Value::Array(values) => {
            output.push(b'[');
            for (index, value) in values.iter().enumerate() {
                if index > 0 {
                    output.push(b',');
                }
                canonical_json(value, output)?;
            }
            output.push(b']');
        }
        serde_json::Value::Object(values) => {
            output.push(b'{');
            let mut keys = values.keys().collect::<Vec<_>>();
            keys.sort();
            for (index, key) in keys.into_iter().enumerate() {
                if index > 0 {
                    output.push(b',');
                }
                output.extend_from_slice(
                    serde_json::to_string(key)
                        .map_err(|error| error.to_string())?
                        .as_bytes(),
                );
                output.push(b':');
                canonical_json(&values[key], output)?;
            }
            output.push(b'}');
        }
    }
    Ok(())
}
