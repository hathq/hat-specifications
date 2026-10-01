use std::collections::BTreeSet;

use crate::model::validation;
use crate::operations::allowed;
use crate::tokens::{
    classification_rank, namespaced_id, semantic_version, stable_token, unique_tokens,
};
use crate::{HatManifest, MANIFEST_SCHEMA, Validation};

/// Validates identity, classification, and closed permission vocabulary.
#[must_use]
pub fn validate_manifest(manifest: &HatManifest) -> Validation {
    let mut findings = Vec::new();
    if manifest.schema != MANIFEST_SCHEMA {
        findings.push(format!("schema must be {MANIFEST_SCHEMA}"));
    }
    namespaced_id(&manifest.id, "hat/", "hat id", &mut findings);
    let name_length = manifest.name.chars().count();
    if name_length == 0 || name_length > 160 {
        findings.push("name must contain between 1 and 160 characters".to_owned());
    }
    if !semantic_version(&manifest.version) {
        findings.push("version must be a numeric major.minor.patch".to_owned());
    }
    unique_tokens(&manifest.capabilities, "capabilities", 256, &mut findings);
    unique_tokens(
        &manifest.decision_boundaries,
        "decision_boundaries",
        128,
        &mut findings,
    );
    validate_classifications(manifest, &mut findings);
    validate_permissions(manifest, &mut findings);
    validation(findings)
}

fn validate_classifications(manifest: &HatManifest, findings: &mut Vec<String>) {
    unique_tokens(
        &manifest.input_classifications,
        "input_classifications",
        8,
        findings,
    );
    for value in &manifest.input_classifications {
        if classification_rank(value).is_none() {
            findings.push(format!("unsupported input classification {value}"));
        }
    }
    if classification_rank(&manifest.output_classification).is_none() {
        findings.push("output_classification is unsupported".to_owned());
    }
}

fn validate_permissions(manifest: &HatManifest, findings: &mut Vec<String>) {
    if manifest.permissions.is_empty() || manifest.permissions.len() > 256 {
        findings.push("permissions must contain between 1 and 256 entries".to_owned());
    }
    let mut resources = BTreeSet::new();
    for permission in &manifest.permissions {
        if !resources.insert(permission.resource.as_str()) {
            findings.push(format!(
                "duplicate permission resource {}",
                permission.resource
            ));
        }
        if !stable_token(&permission.resource) {
            findings.push("permission resource must be a stable token".to_owned());
        }
        if !matches!(permission.mode.as_str(), "observe" | "propose" | "execute") {
            findings.push("permission mode must be observe, propose or execute".to_owned());
        }
        unique_tokens(
            &permission.operations,
            "permission operations",
            64,
            findings,
        );
        for operation in &permission.operations {
            if !allowed(&permission.mode).contains(&operation.as_str()) {
                findings.push(format!(
                    "operation {operation} is not allowed in {} mode",
                    permission.mode
                ));
            }
        }
    }
}
