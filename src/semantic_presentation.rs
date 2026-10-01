//! Non-operational, locale-exact presentation of an already validated semantic clause.

use crate::{SemanticClauseRole, SemanticModality};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const SEMANTIC_PRESENTATION_CATALOG_SCHEMA: &str =
    "hathq://hat/semantic-presentation-catalog/v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticPresentationTemplate {
    pub frame_id: String,
    pub modality: SemanticModality,
    pub locale: String,
    pub template: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticPresentationCatalog {
    pub schema: String,
    pub owner_repository_id: String,
    pub templates: Vec<SemanticPresentationTemplate>,
}

/// Validates a display-only presentation catalog without granting operational authority.
///
/// # Errors
///
/// Returns an error when the schema or owner identity is invalid, the catalog is empty or
/// unbounded, or a template has an invalid identity, locale, body, placeholder, or duplicate
/// frame/modality/locale identity.
pub fn validate_semantic_presentation_catalog(
    value: &SemanticPresentationCatalog,
) -> Result<(), String> {
    if value.schema != SEMANTIC_PRESENTATION_CATALOG_SCHEMA
        || !token(&value.owner_repository_id)
        || value.templates.is_empty()
        || value.templates.len() > 4_096
    {
        return Err("semantic presentation catalog is invalid".into());
    }
    let mut identities = BTreeSet::new();
    for item in &value.templates {
        if !token(&item.frame_id)
            || !locale(&item.locale)
            || item.template.is_empty()
            || item.template.len() > 500
            || item.template.chars().any(char::is_control)
            || placeholders(&item.template).is_err()
            || !identities.insert((item.frame_id.as_str(), item.modality, item.locale.as_str()))
        {
            return Err("semantic presentation template is invalid".into());
        }
    }
    Ok(())
}

/// Presents one exact frame/modality/locale combination from explicit display values.
///
/// Missing templates, roles, or locales are errors. No locale fallback or grammatical inference
/// is performed.
///
/// # Errors
///
/// Returns an error when the catalog is invalid, the exact template is absent, a placeholder does
/// not name a supported semantic role, or an explicit bounded role value is unavailable.
pub fn present_semantic_activity(
    catalog: &SemanticPresentationCatalog,
    frame_id: &str,
    modality: SemanticModality,
    locale: &str,
    values: &BTreeMap<SemanticClauseRole, String>,
) -> Result<String, String> {
    validate_semantic_presentation_catalog(catalog)?;
    let template = catalog
        .templates
        .iter()
        .find(|item| {
            item.frame_id == frame_id && item.modality == modality && item.locale == locale
        })
        .ok_or_else(|| "semantic presentation template is unavailable".to_owned())?;
    let mut result = template.template.clone();
    for placeholder in placeholders(&template.template)? {
        let role = role(&placeholder)
            .ok_or_else(|| "semantic presentation placeholder is invalid".to_owned())?;
        let value = values
            .get(&role)
            .filter(|value| !value.is_empty() && value.len() <= 256)
            .ok_or_else(|| "semantic presentation value is unavailable".to_owned())?;
        result = result.replace(&format!("{{{placeholder}}}"), value);
    }
    Ok(result)
}

fn placeholders(value: &str) -> Result<BTreeSet<String>, String> {
    let mut result = BTreeSet::new();
    let mut rest = value;
    while let Some(start) = rest.find('{') {
        let after = &rest[start + 1..];
        let end = after
            .find('}')
            .ok_or_else(|| "semantic presentation placeholder is invalid".to_owned())?;
        let name = &after[..end];
        if role(name).is_none() || name.contains('{') {
            return Err("semantic presentation placeholder is invalid".into());
        }
        result.insert(name.to_owned());
        rest = &after[end + 1..];
    }
    if rest.contains('}') {
        return Err("semantic presentation placeholder is invalid".into());
    }
    Ok(result)
}

fn role(value: &str) -> Option<SemanticClauseRole> {
    match value {
        "actor" => Some(SemanticClauseRole::Actor),
        "subject" => Some(SemanticClauseRole::Subject),
        "object" => Some(SemanticClauseRole::Object),
        "value" => Some(SemanticClauseRole::Value),
        "source" => Some(SemanticClauseRole::Source),
        "destination" => Some(SemanticClauseRole::Destination),
        "instrument" => Some(SemanticClauseRole::Instrument),
        "time" => Some(SemanticClauseRole::Time),
        "place" => Some(SemanticClauseRole::Place),
        "evidence" => Some(SemanticClauseRole::Evidence),
        "output" => Some(SemanticClauseRole::Output),
        _ => None,
    }
}

fn token(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|value| value.is_ascii_alphanumeric() || matches!(value, b'-' | b'_' | b'.'))
}

fn locale(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 35
        && value.split('-').all(|part| {
            !part.is_empty()
                && part.len() <= 8
                && part.bytes().all(|value| value.is_ascii_alphanumeric())
        })
}
