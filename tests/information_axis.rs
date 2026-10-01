// Added by the HAT Specifications project, 2026.
// Purpose: reject unknown axes and UI hierarchy coupling.

use hat_specifications::{
    ChangePolicy, INFORMATION_COORDINATE_SCHEMA, InformationCoordinate, InformationSensitivity,
    StructuralAxis, validate_information_coordinate,
};

const DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn coordinate() -> InformationCoordinate {
    InformationCoordinate {
        schema: INFORMATION_COORDINATE_SCHEMA.to_owned(),
        vocabulary_owner_id: "health-hat".to_owned(),
        catalog_digest_sha256: DIGEST.to_owned(),
        term_id: "hathq://health/observation/resting-heart-rate/v1".to_owned(),
        structural_axis: StructuralAxis::HealthObservation,
        change_policy: ChangePolicy::AppendOnlyObservation,
        data_domain_term_id: "hathq://vocabulary/data-domain/health/v1".to_owned(),
        sensitivity: InformationSensitivity::RestrictedSensitive,
    }
}

#[test]
fn independent_axes_validate_without_a_ui_path() {
    assert!(validate_information_coordinate(&coordinate()).valid);
}

#[test]
fn unknown_axis_and_change_policy_fail_closed() {
    let value = serde_json::to_value(coordinate()).expect("serialize");
    for (field, replacement) in [
        ("structural_axis", "dashboard-level-two"),
        ("change_policy", "model-decides"),
    ] {
        let mut invalid = value.clone();
        invalid.as_object_mut().expect("object").insert(
            field.to_owned(),
            serde_json::Value::String(replacement.to_owned()),
        );
        assert!(serde_json::from_value::<InformationCoordinate>(invalid).is_err());
    }
}

#[test]
fn console_hierarchy_cannot_enter_the_information_contract() {
    let mut value = serde_json::to_value(coordinate()).expect("serialize");
    value
        .as_object_mut()
        .expect("object")
        .insert("ui_level".to_owned(), serde_json::json!(2));
    assert!(serde_json::from_value::<InformationCoordinate>(value).is_err());
}

#[test]
fn data_domain_requires_a_canonical_term() {
    let mut coordinate = coordinate();
    coordinate.data_domain_term_id = "health".to_owned();
    assert!(!validate_information_coordinate(&coordinate).valid);
}
