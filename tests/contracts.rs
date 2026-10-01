use hat_specifications::{
    doctor, fit, parse_manifest, parse_profile, validate_manifest, validate_profile,
};

const HAT: &str = include_str!("../examples/source-curator.hat.toml");
const PROFILE: &str = include_str!("../examples/editor.profile.toml");

#[test]
fn bundled_manifest_and_profile_fit_without_external_authority() {
    let manifest = parse_manifest(HAT).expect("manifest");
    let profile = parse_profile(PROFILE).expect("profile");
    assert!(validate_manifest(&manifest).valid);
    assert!(validate_profile(&profile).valid);
    assert!(fit(&manifest, &profile).fits);
    assert!(doctor().valid);
}

#[test]
fn permission_modes_are_closed_and_mode_specific() {
    let mut manifest = parse_manifest(HAT).expect("manifest");
    for operation in [
        "execute", "send", "write", "update", "create", "deploy", "approve",
    ] {
        manifest.permissions[0].operations = vec![operation.to_owned()];
        assert!(!validate_manifest(&manifest).valid, "{operation}");
    }
    manifest = parse_manifest(HAT).expect("manifest");
    manifest.permissions[0].mode = "observe".to_owned();
    manifest.permissions[0].operations = vec!["propose-classification".to_owned()];
    assert!(!validate_manifest(&manifest).valid);

    manifest = parse_manifest(HAT).expect("manifest");
    manifest.permissions[0].mode = "execute".to_owned();
    manifest.permissions[0].operations = vec!["publish-artifact".to_owned()];
    assert!(validate_manifest(&manifest).valid);
    manifest.permissions[0].operations = vec!["inspect-metadata".to_owned()];
    assert!(!validate_manifest(&manifest).valid);
}

#[test]
fn input_is_closed_bounded_and_credential_free() {
    assert!(parse_manifest(&format!("{HAT}\ntoken = \"no\"\n")).is_err());
    assert!(parse_manifest(&format!("{HAT}\nfuture = true\n")).is_err());
    assert!(parse_manifest(&" ".repeat(1_048_577)).is_err());
    let raw_key = HAT.replace(
        "Source Curator",
        "-----BEGIN PRIVATE KEY----- no -----END PRIVATE KEY-----",
    );
    assert!(parse_manifest(&raw_key).is_err());
}

#[test]
fn invalid_identity_classification_and_duplicate_grants_do_not_fit() {
    let mut manifest = parse_manifest(HAT).expect("manifest");
    manifest.id = "hat/source/curator".to_owned();
    manifest.input_classifications = vec!["unknown".to_owned()];
    let mut profile = parse_profile(PROFILE).expect("profile");
    profile
        .granted_capabilities
        .push(profile.granted_capabilities[0].clone());
    assert!(!validate_manifest(&manifest).valid);
    assert!(!validate_profile(&profile).valid);
    assert!(!fit(&manifest, &profile).fits);
}

#[test]
fn deletion_is_an_explicit_execute_operation_never_observation_or_proposal() {
    let mut manifest = parse_manifest(HAT).expect("manifest");
    manifest.permissions[0].resource = "github-repository-administration".into();
    manifest.permissions[0].operations = vec!["delete-resource".into()];
    for mode in ["observe", "propose"] {
        manifest.permissions[0].mode = mode.into();
        assert!(!validate_manifest(&manifest).valid);
    }
    manifest.permissions[0].mode = "execute".into();
    assert!(validate_manifest(&manifest).valid);
}
