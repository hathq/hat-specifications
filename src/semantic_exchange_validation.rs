// Added by the HAT Specifications project, 2026.
// Purpose: validate and classify exact cross-owner semantic exchanges.

use crate::semantic_validation::{digest, reference, token};
use crate::validate_semantic_term_reference;
use crate::{SEMANTIC_COMPREHENSION_SCHEMA, SEMANTIC_EXCHANGE_SCHEMA, SemanticExchange};
use crate::{SemanticComprehensionItem, SemanticComprehensionReport, SemanticTermComprehension};
use crate::{SemanticTermReference, semantic_exchange_digest, validate_semantic_clause};
use std::collections::{BTreeMap, BTreeSet};

/// Validates a bounded cross-owner semantic exchange.
///
/// # Errors
///
/// Returns an error when identities, terms, evidence, lifetime, classification
/// or the canonical digest are invalid.
pub fn validate_semantic_exchange(value: &SemanticExchange) -> Result<(), String> {
    if value.schema != SEMANTIC_EXCHANGE_SCHEMA {
        return Err("semantic exchange schema is invalid".into());
    }
    token(&value.exchange_id, "semantic exchange id")?;
    reference(&value.sender)?;
    reference(&value.receiver)?;
    if value.sender == value.receiver {
        return Err("semantic exchange owners must be distinct".into());
    }
    validate_semantic_clause(&value.clause)?;
    digest(&value.frame_digest_sha256)?;
    digest(&value.output_schema_digest_sha256)?;
    if value.frame_digest_sha256 != value.clause.frame.definition_digest_sha256
        || value.source_revision == 0
    {
        return Err("semantic exchange frame or source revision is invalid".into());
    }
    if value.evidence.is_empty() || value.evidence.len() > 256 {
        return Err("semantic exchange evidence set is invalid".into());
    }
    for evidence in &value.evidence {
        reference(evidence)?;
    }
    if !matches!(
        value.information_classification.as_str(),
        "public" | "internal" | "internal-confidential" | "restricted-sensitive"
    ) {
        return Err("semantic exchange information classification is invalid".into());
    }
    token(&value.purpose, "semantic exchange purpose")?;
    reference(&value.retention)?;
    token(&value.nonce, "semantic exchange nonce")?;
    if value.issued_at_epoch_s == 0
        || value.expires_at_epoch_s <= value.issued_at_epoch_s
        || value.expires_at_epoch_s - value.issued_at_epoch_s > 86_400
    {
        return Err("semantic exchange lifetime is invalid".into());
    }
    validate_terms(value)?;
    digest(&value.content_digest_sha256)?;
    if semantic_exchange_digest(value)? != value.content_digest_sha256 {
        return Err("semantic exchange content digest differs".into());
    }
    Ok(())
}

/// Classifies every exchanged term against the receiver's exact catalog set.
///
/// # Errors
///
/// Returns an error when the exchange itself is invalid. Unknown, conflicting
/// and denied terms are returned as explicit comprehension states.
pub fn classify_semantic_exchange(
    value: &SemanticExchange,
    available: &[SemanticTermReference],
    denied_term_ids: &BTreeSet<String>,
) -> Result<SemanticComprehensionReport, String> {
    validate_semantic_exchange(value)?;
    let by_id = available
        .iter()
        .map(|term| (term.term_id.as_str(), term))
        .collect::<BTreeMap<_, _>>();
    let terms = value
        .terms
        .iter()
        .map(|term| SemanticComprehensionItem {
            term: term.clone(),
            state: classify(term, &by_id, denied_term_ids),
        })
        .collect::<Vec<_>>();
    let accepted = terms
        .iter()
        .all(|item| item.state == SemanticTermComprehension::Understood);
    Ok(SemanticComprehensionReport {
        schema: SEMANTIC_COMPREHENSION_SCHEMA.into(),
        exchange_id: value.exchange_id.clone(),
        receiver: value.receiver.clone(),
        terms,
        accepted,
    })
}

fn classify(
    term: &SemanticTermReference,
    available: &BTreeMap<&str, &SemanticTermReference>,
    denied: &BTreeSet<String>,
) -> SemanticTermComprehension {
    if denied.contains(&term.term_id) {
        return SemanticTermComprehension::PolicyDenied;
    }
    match available.get(term.term_id.as_str()) {
        None => SemanticTermComprehension::HatRequired,
        Some(candidate) if *candidate == term => SemanticTermComprehension::Understood,
        Some(_) => SemanticTermComprehension::VersionConflict,
    }
}

fn validate_terms(value: &SemanticExchange) -> Result<(), String> {
    if value.terms.is_empty() || value.terms.len() > 256 {
        return Err("semantic exchange term set is invalid".into());
    }
    let mut identities = BTreeSet::new();
    for term in &value.terms {
        validate_semantic_term_reference(term)?;
        if !identities.insert((&term.term_id, term.version, &term.definition_digest_sha256)) {
            return Err("semantic exchange term is duplicated".into());
        }
    }
    let used = value
        .clause
        .bindings
        .iter()
        .filter_map(|binding| binding.term.as_ref())
        .chain(std::iter::once(&value.clause.frame.predicate));
    if used.into_iter().any(|term| !value.terms.contains(term)) {
        return Err("semantic exchange clause uses an undeclared term".into());
    }
    Ok(())
}
