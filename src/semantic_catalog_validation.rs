// Added by the HAT Specifications project, 2026.
// Purpose: validate exact operational catalog identities, terms and display metadata.

use crate::SemanticTermReference;
use crate::semantic_validation::{digest, token};
use crate::{SEMANTIC_CATALOG_SCHEMA, SemanticCatalog, SemanticCatalogReference};
use crate::{SemanticLexicalization, SemanticTermDefinition};
use std::collections::BTreeSet;

/// Validates a catalog identity and digest.
///
/// # Errors
///
/// Returns an error when the identifier, version or digest is invalid.
pub fn validate_semantic_catalog_reference(value: &SemanticCatalogReference) -> Result<(), String> {
    token(&value.catalog_id, "semantic catalog id")?;
    if value.version == 0 {
        return Err("semantic catalog version is invalid".into());
    }
    digest(&value.digest_sha256)
}

/// Validates an exact catalog-bound semantic term reference.
///
/// # Errors
///
/// Returns an error when any identity, version, kind-bound digest or definition
/// digest field is invalid.
pub fn validate_semantic_term_reference(value: &SemanticTermReference) -> Result<(), String> {
    token(&value.catalog_id, "semantic term catalog id")?;
    token(&value.term_id, "semantic term id")?;
    if value.version == 0 {
        return Err("semantic term version is invalid".into());
    }
    digest(&value.catalog_digest_sha256)?;
    digest(&value.definition_digest_sha256)
}

/// Validates one non-operational locale lexicalization.
///
/// # Errors
///
/// Returns an error for an invalid term, locale, preferred label or alias set.
pub fn validate_semantic_lexicalization(value: &SemanticLexicalization) -> Result<(), String> {
    validate_semantic_term_reference(&value.term)?;
    if !locale(&value.locale) || !text(&value.preferred, 160) || value.search_aliases.len() > 32 {
        return Err("semantic lexicalization is invalid".into());
    }
    if value
        .search_aliases
        .iter()
        .any(|alias| !text(alias, 160) || alias == &value.preferred)
    {
        return Err("semantic lexicalization alias is invalid".into());
    }
    Ok(())
}

/// Validates a complete semantic catalog and all of its exact relations.
///
/// # Errors
///
/// Returns an error when the catalog is malformed, conflicted, out of closure
/// or differs from its canonical digest.
pub fn validate_semantic_catalog(value: &SemanticCatalog) -> Result<(), String> {
    if value.schema != SEMANTIC_CATALOG_SCHEMA {
        return Err("semantic catalog schema is invalid".into());
    }
    validate_semantic_catalog_reference(&value.identity)?;
    token(&value.owner_repository_id, "semantic catalog owner")?;
    if value.foundation && (!value.dependencies.is_empty() || !value.incompatibilities.is_empty()) {
        return Err("foundation semantic catalog cannot depend on an extension".into());
    }
    let mut catalogs = BTreeSet::from([(
        value.identity.catalog_id.as_str(),
        value.identity.digest_sha256.as_str(),
    )]);
    for dependency in &value.dependencies {
        validate_semantic_catalog_reference(dependency)?;
        if !catalogs.insert((&dependency.catalog_id, &dependency.digest_sha256)) {
            return Err("semantic catalog dependency is duplicated".into());
        }
    }
    let mut incompatibilities = BTreeSet::new();
    for incompatibility in &value.incompatibilities {
        validate_semantic_catalog_reference(incompatibility)?;
        if catalogs.contains(&(
            incompatibility.catalog_id.as_str(),
            incompatibility.digest_sha256.as_str(),
        )) || !incompatibilities.insert((
            incompatibility.catalog_id.as_str(),
            incompatibility.digest_sha256.as_str(),
        )) {
            return Err("semantic catalog incompatibility is duplicated or required".into());
        }
    }
    if value.terms.is_empty() || value.terms.len() > 4096 {
        return Err("semantic catalog term set is invalid".into());
    }
    let mut terms = BTreeSet::new();
    for definition in &value.terms {
        validate_definition(definition, &value.identity, &catalogs)?;
        if !terms.insert(identity(&definition.reference)) {
            return Err("semantic catalog term is duplicated".into());
        }
    }
    let mut locales = BTreeSet::new();
    for lexicalization in &value.lexicalizations {
        validate_semantic_lexicalization(lexicalization)?;
        if !terms.contains(&identity(&lexicalization.term))
            || !locales.insert((
                identity(&lexicalization.term),
                lexicalization.locale.as_str(),
            ))
        {
            return Err("semantic lexicalization term or locale is invalid".into());
        }
    }
    if crate::semantic_catalog_digest(value)? != value.identity.digest_sha256 {
        return Err("semantic catalog digest differs".into());
    }
    Ok(())
}

fn validate_definition(
    value: &SemanticTermDefinition,
    catalog: &SemanticCatalogReference,
    catalogs: &BTreeSet<(&str, &str)>,
) -> Result<(), String> {
    validate_semantic_term_reference(&value.reference)?;
    if value.reference.catalog_id != catalog.catalog_id
        || value.reference.catalog_digest_sha256 != catalog.digest_sha256
        || crate::semantic_term_definition_digest(value)?
            != value.reference.definition_digest_sha256
    {
        return Err("semantic term definition identity differs".into());
    }
    if value
        .wire_schema
        .as_ref()
        .is_some_and(|schema| !crate::tokens::schema_uri(schema))
    {
        return Err("semantic term wire schema is invalid".into());
    }
    for term in value
        .dependencies
        .iter()
        .chain(&value.is_a)
        .chain(&value.domain)
        .chain(&value.range)
    {
        validate_semantic_term_reference(term)?;
        if !catalogs.contains(&(
            term.catalog_id.as_str(),
            term.catalog_digest_sha256.as_str(),
        )) {
            return Err("semantic term relation uses an undeclared catalog".into());
        }
    }
    Ok(())
}

fn identity(value: &SemanticTermReference) -> (&str, u64, &str) {
    (
        &value.term_id,
        value.version,
        &value.definition_digest_sha256,
    )
}

fn locale(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 35
        && value.split('-').all(|part| {
            !part.is_empty()
                && part.len() <= 8
                && part.bytes().all(|byte| byte.is_ascii_alphanumeric())
        })
}

fn text(value: &str, maximum: usize) -> bool {
    !value.is_empty()
        && value == value.trim()
        && value.len() <= maximum
        && !value.chars().any(char::is_control)
}
