use serde::{Deserialize, Serialize};

/// Role package containing declarative capabilities and permission bounds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatManifest {
    pub schema: String,
    pub id: String,
    pub name: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub decision_boundaries: Vec<String>,
    pub input_classifications: Vec<String>,
    pub output_classification: String,
    pub permissions: Vec<HatPermission>,
}

/// One resource permission limited to a mode-specific closed operation set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatPermission {
    pub resource: String,
    pub operations: Vec<String>,
    pub mode: String,
}

/// Runtime-independent limits granted to a HAT by its fitting environment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FittingProfile {
    pub schema: String,
    pub id: String,
    pub granted_capabilities: Vec<String>,
    pub allowed_resources: Vec<String>,
    pub decision_authorities: Vec<String>,
    pub maximum_input_classification: String,
}

/// Deterministic validation result with sorted, deduplicated findings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Validation {
    pub valid: bool,
    pub findings: Vec<String>,
}

/// Compatibility result between a manifest and fitting profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Fitting {
    pub fits: bool,
    pub findings: Vec<String>,
}

pub(crate) fn validation(mut findings: Vec<String>) -> Validation {
    findings.sort();
    findings.dedup();
    Validation {
        valid: findings.is_empty(),
        findings,
    }
}
