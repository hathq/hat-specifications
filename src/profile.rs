use crate::model::validation;
use crate::tokens::{classification_rank, namespaced_id, unique_tokens};
use crate::{FittingProfile, PROFILE_SCHEMA, Validation};

/// Validates the bounded grants available to a fitted HAT.
#[must_use]
pub fn validate_profile(profile: &FittingProfile) -> Validation {
    let mut findings = Vec::new();
    if profile.schema != PROFILE_SCHEMA {
        findings.push(format!("profile schema must be {PROFILE_SCHEMA}"));
    }
    namespaced_id(&profile.id, "profile/", "profile id", &mut findings);
    unique_tokens(
        &profile.granted_capabilities,
        "granted_capabilities",
        256,
        &mut findings,
    );
    unique_tokens(
        &profile.allowed_resources,
        "allowed_resources",
        256,
        &mut findings,
    );
    unique_tokens(
        &profile.decision_authorities,
        "decision_authorities",
        128,
        &mut findings,
    );
    if classification_rank(&profile.maximum_input_classification).is_none() {
        findings.push("profile maximum_input_classification is unsupported".to_owned());
    }
    validation(findings)
}
