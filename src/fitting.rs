use std::collections::BTreeSet;

use crate::tokens::classification_rank;
use crate::{Fitting, FittingProfile, HatManifest, validate_manifest, validate_profile};

/// Checks whether a valid profile covers every manifest requirement.
#[must_use]
pub fn fit(manifest: &HatManifest, profile: &FittingProfile) -> Fitting {
    let mut findings = validate_manifest(manifest).findings;
    findings.extend(validate_profile(profile).findings);
    let capabilities: BTreeSet<_> = profile.granted_capabilities.iter().collect();
    for capability in &manifest.capabilities {
        if !capabilities.contains(capability) {
            findings.push(format!("profile lacks capability {capability}"));
        }
    }
    let resources: BTreeSet<_> = profile.allowed_resources.iter().collect();
    for permission in &manifest.permissions {
        if !resources.contains(&permission.resource) {
            findings.push(format!("profile does not allow {}", permission.resource));
        }
    }
    let authorities: BTreeSet<_> = profile.decision_authorities.iter().collect();
    for boundary in &manifest.decision_boundaries {
        if !authorities.contains(boundary) {
            findings.push(format!("profile lacks decision authority {boundary}"));
        }
    }
    if let Some(maximum) = classification_rank(&profile.maximum_input_classification) {
        for classification in &manifest.input_classifications {
            match classification_rank(classification) {
                Some(rank) if rank > maximum => {
                    findings
                        .push("HAT input classification exceeds the profile maximum".to_owned());
                }
                Some(_) => {}
                None => findings.push(format!("unsupported input classification {classification}")),
            }
        }
    }
    findings.sort();
    findings.dedup();
    Fitting {
        fits: findings.is_empty(),
        findings,
    }
}
