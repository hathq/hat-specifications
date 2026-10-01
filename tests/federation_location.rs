use hat_specifications::{
    EXECUTION_LOCATION_SCHEMA, EXECUTION_LOCATION_SET_SCHEMA, HatExecutionKind,
    HatExecutionLocation, HatExecutionLocationSet, execution_location_digest,
    validate_execution_location, validate_execution_location_set,
};

const DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn location() -> HatExecutionLocation {
    HatExecutionLocation {
        schema: EXECUTION_LOCATION_SCHEMA.into(),
        location_id: "tokyo-owner-node".into(),
        package_id: "hat/source-curator".into(),
        package_digest_sha256: DIGEST.into(),
        publisher_id: "hathq".into(),
        execution_kind: HatExecutionKind::FederatedManaged,
        worker_service_id: "source-curator-tokyo".into(),
        identity_authority_ref: "ihat/service/source-curator-tokyo".into(),
        evidence_recovery: hat_specifications::HatEvidenceRecovery::Unrecoverable {},
        transport_profile_ref: "crowsi/mtls-authority-v1".into(),
        route_ref: "crowsi/route/source-curator-tokyo".into(),
        region: Some("jp-east".into()),
        jurisdictions: vec!["jp".into()],
        data_residencies: vec!["jp".into()],
        operation_ids: vec!["hathq://hat/action/review-source/v1".into()],
        accepted_classifications: vec!["internal".into()],
        capability_ids: vec!["text-analysis".into()],
        assurance: "verified".into(),
        issued_at_epoch_s: 1_000,
        expires_at_epoch_s: 1_300,
        revocation_epoch: 7,
    }
}

#[test]
fn exact_location_is_valid_only_inside_its_signed_lifetime() {
    let value = location();
    assert!(validate_execution_location(&value, 1_100).valid);
    assert!(!validate_execution_location(&value, 999).valid);
    assert!(!validate_execution_location(&value, 1_300).valid);
}

#[test]
fn location_rejects_transport_shaped_and_ambiguous_values() {
    for mutation in [
        |value: &mut HatExecutionLocation| value.route_ref = "https://host/path".into(),
        |value: &mut HatExecutionLocation| value.location_id = "Tokyo".into(),
        |value: &mut HatExecutionLocation| value.operation_ids.clear(),
        |value: &mut HatExecutionLocation| value.jurisdictions.push("jp".into()),
    ] {
        let mut value = location();
        mutation(&mut value);
        assert!(!validate_execution_location(&value, 1_100).valid);
    }
}

#[test]
fn location_wire_is_closed() {
    let mut value = serde_json::to_value(location()).expect("location JSON");
    value["endpoint"] = serde_json::json!("203.0.113.4:443");
    assert!(serde_json::from_value::<HatExecutionLocation>(value).is_err());
}

#[test]
fn location_digest_is_stable_and_mutation_sensitive() {
    let value = location();
    let digest = execution_location_digest(&value).expect("location digest");
    assert_eq!(digest.len(), 64);
    assert_eq!(
        digest,
        execution_location_digest(&value).expect("same digest")
    );
    let mut changed = value;
    changed.revocation_epoch += 1;
    assert_ne!(
        digest,
        execution_location_digest(&changed).expect("changed")
    );
}

#[test]
fn location_set_is_closed_bounded_and_package_exact() {
    let value = HatExecutionLocationSet {
        schema: EXECUTION_LOCATION_SET_SCHEMA.into(),
        package_id: "hat/source-curator".into(),
        package_digest_sha256: DIGEST.into(),
        revision: 1,
        issued_at_epoch_s: 1_000,
        expires_at_epoch_s: 1_300,
        locations: vec![location()],
    };
    assert!(validate_execution_location_set(&value, 1_100).valid);
    let mut duplicate = value.clone();
    duplicate.locations.push(location());
    assert!(!validate_execution_location_set(&duplicate, 1_100).valid);
    let mut substituted = value;
    substituted.locations[0].package_id = "hat/accountant".into();
    assert!(!validate_execution_location_set(&substituted, 1_100).valid);
}
