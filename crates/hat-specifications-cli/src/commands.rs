//! Dispatch explicit read-only validation commands without external effects.
use super::{ResultEnvelope, cli_input, envelope, load_manifest};
use hat_specifications::{
    doctor, fit, parse_package, parse_profile, validate_manifest, validate_package,
};
use serde_json::json;

pub(super) fn run(arguments: &[String]) -> ResultEnvelope {
    match arguments.first().map(String::as_str) {
        Some("doctor") if arguments.len() == 1 => {
            let validation = doctor();
            envelope(
                "doctor",
                validation.valid,
                json!({
                    "manifest_schema": hat_specifications::MANIFEST_SCHEMA,
                    "profile_schema": hat_specifications::PROFILE_SCHEMA,
                    "network_enabled": false,
                    "credential_storage_enabled": false,
                    "execution_enabled": false,
                    "self_check": validation.valid
                }),
                validation.findings,
            )
        }
        Some("validate") if arguments.len() == 2 => {
            let loaded = load_manifest(&arguments[1]);
            match loaded {
                Ok(manifest) => {
                    let validation = validate_manifest(&manifest);
                    envelope(
                        "validate",
                        validation.valid,
                        json!({"hat_id": manifest.id, "valid": validation.valid}),
                        validation.findings,
                    )
                }
                Err(error) => envelope("validate", false, json!({}), vec![error]),
            }
        }
        Some("validate-package") if arguments.len() == 2 => {
            match cli_input::read_bounded_utf8(&arguments[1])
                .and_then(|source| parse_package(&source))
            {
                Ok(package) => {
                    let validation = validate_package(&package);
                    envelope(
                        "validate-package",
                        validation.valid,
                        json!({"package_id": package.package_id, "valid": validation.valid}),
                        validation.findings,
                    )
                }
                Err(error) => envelope("validate-package", false, json!({}), vec![error]),
            }
        }
        Some("fitting") if arguments.len() == 3 => {
            match (
                load_manifest(&arguments[1]),
                cli_input::read_bounded_utf8(&arguments[2])
                    .and_then(|source| parse_profile(&source)),
            ) {
                (Ok(manifest), Ok(profile)) => {
                    let fitting = fit(&manifest, &profile);
                    envelope(
                        "fitting",
                        fitting.fits,
                        json!({"hat_id": manifest.id, "profile_id": profile.id, "fits": fitting.fits}),
                        fitting.findings,
                    )
                }
                (Err(error), _) | (_, Err(error)) => {
                    envelope("fitting", false, json!({}), vec![error])
                }
            }
        }
        _ => envelope(
            "usage",
            false,
            json!({"usage": "hat-specifications <doctor|validate <hat>|validate-package <package>|fitting <hat> <profile>>"}),
            vec!["unsupported arguments".to_string()],
        ),
    }
}
