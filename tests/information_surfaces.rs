use hat_specifications::{parse_package, validate_package};
use serde_json::{Value, json};

const PACKAGE: &str = include_str!("../examples/source-curator.package.json");
const SURFACE: &str = "knowledge-sources";

fn declared_package() -> Value {
    let mut value: Value = serde_json::from_str(PACKAGE).expect("package json");
    value["information_surfaces"] = json!([{
        "id": "knowledge-sources",
        "canonical_type": "domain.knowledge.source",
        "projection_schema": "hathq://hat-source-curator/knowledge-source/v1",
        "data_domain_term_id": "hathq://vocabulary/data-domain/knowledge/v1",
        "subject_relation": "managed",
        "semantic_icon": "source"
    }]);
    value["operations"][0]["effects"] = json!([{
        "kind": "update",
        "surface_id": SURFACE
    }]);
    value
}

#[test]
fn package_can_declare_information_without_shipping_ui_code() {
    let wire = serde_json::to_string(&declared_package()).expect("wire");
    let package = parse_package(&wire).expect("declared package");
    assert!(validate_package(&package).valid);
    assert_eq!(
        package.information_surfaces[0].canonical_type,
        "domain.knowledge.source"
    );
    assert_eq!(package.operations[0].effects[0].surface_id, SURFACE);
}

#[test]
fn effects_cannot_reference_undeclared_information_surfaces() {
    let mut value = declared_package();
    value["operations"][0]["effects"][0]["surface_id"] = json!("other-surface");
    let package = parse_package(&serde_json::to_string(&value).expect("wire")).expect("package");
    assert!(!validate_package(&package).valid);
}

#[test]
fn legacy_term_binding_is_rejected_instead_of_preserved() {
    let mut value = declared_package();
    value["information_surfaces"][0]["term_id"] =
        json!("hathq://vocabulary/entity/knowledge-source/v1");
    assert!(parse_package(&serde_json::to_string(&value).expect("wire")).is_err());
    value = declared_package();
    value["operations"][0]["effects"][0]["term_id"] =
        json!("hathq://vocabulary/entity/knowledge-source/v1");
    assert!(parse_package(&serde_json::to_string(&value).expect("wire")).is_err());
}

#[test]
fn host_presentation_fields_are_rejected() {
    let mut value = declared_package();
    value["information_surfaces"][0]["component"] = json!("RemoteRepositoryCard.vue");
    let wire = serde_json::to_string(&value).expect("wire");
    assert!(parse_package(&wire).is_err());
}

#[test]
fn service_requirement_declares_operations_without_fixing_a_provider() {
    let mut value = declared_package();
    value["service_requirements"] = json!([{
        "id": "work-item-publication",
        "operation_ids": ["hathq://vocabulary/action/create-github-issue/v1"]
    }]);
    let package = parse_package(&serde_json::to_string(&value).expect("wire")).expect("package");
    assert!(validate_package(&package).valid);
    assert_eq!(package.service_requirements[0].id, "work-item-publication");

    value["service_requirements"][0]["provider_repository_id"] = json!("hat-github-operator");
    assert!(parse_package(&serde_json::to_string(&value).expect("wire")).is_err());
}

#[test]
fn setup_template_reuses_a_declared_required_context() {
    let mut value = declared_package();
    let selector = value["operations"][0]["context_plan"]["selectors"][0].clone();
    value["setup_templates"] = json!([{
        "id": "review-source-context",
        "title": "情報源を設定する",
        "summary": "このHATが確認する情報源を選択します。",
        "action_label": "情報源を設定",
        "context_namespace": selector["namespace"],
        "projection_schema": selector["projection_schema"]
    }]);
    let package = parse_package(&serde_json::to_string(&value).expect("wire")).expect("package");
    assert!(validate_package(&package).valid);

    value["setup_templates"][0]["context_namespace"] = json!("undeclared-context");
    let package = parse_package(&serde_json::to_string(&value).expect("wire")).expect("package");
    assert!(!validate_package(&package).valid);
}
