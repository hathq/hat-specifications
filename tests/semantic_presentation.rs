use hat_specifications::{
    SEMANTIC_PRESENTATION_CATALOG_SCHEMA, SemanticClauseRole, SemanticModality,
    SemanticPresentationCatalog, SemanticPresentationTemplate, present_semantic_activity,
};
use std::collections::BTreeMap;

#[test]
fn presents_required_and_executed_without_inferring_grammar() {
    let catalog = SemanticPresentationCatalog {
        schema: SEMANTIC_PRESENTATION_CATALOG_SCHEMA.into(),
        owner_repository_id: "hat-profile".into(),
        templates: vec![
            template(SemanticModality::Required, "{object}を登録する"),
            template(SemanticModality::Executed, "{object}を登録した"),
        ],
    };
    let values = BTreeMap::from([(SemanticClauseRole::Object, "表示名".into())]);
    assert_eq!(
        present_semantic_activity(
            &catalog,
            "profile.display-name.register",
            SemanticModality::Required,
            "ja",
            &values,
        )
        .expect("required"),
        "表示名を登録する"
    );
    assert_eq!(
        present_semantic_activity(
            &catalog,
            "profile.display-name.register",
            SemanticModality::Executed,
            "ja",
            &values,
        )
        .expect("executed"),
        "表示名を登録した"
    );
}

#[test]
fn rejects_missing_locale_role_and_unsupported_placeholder() {
    let catalog = SemanticPresentationCatalog {
        schema: SEMANTIC_PRESENTATION_CATALOG_SCHEMA.into(),
        owner_repository_id: "hat-profile".into(),
        templates: vec![template(SemanticModality::Required, "{object}を登録する")],
    };
    assert!(
        present_semantic_activity(
            &catalog,
            "profile.display-name.register",
            SemanticModality::Required,
            "en",
            &BTreeMap::new()
        )
        .is_err()
    );
    assert!(
        present_semantic_activity(
            &catalog,
            "profile.display-name.register",
            SemanticModality::Required,
            "ja",
            &BTreeMap::new()
        )
        .is_err()
    );
    let invalid = SemanticPresentationCatalog {
        templates: vec![template(SemanticModality::Required, "{unknown}を登録する")],
        ..catalog
    };
    assert!(hat_specifications::validate_semantic_presentation_catalog(&invalid).is_err());
}

fn template(modality: SemanticModality, value: &str) -> SemanticPresentationTemplate {
    SemanticPresentationTemplate {
        frame_id: "profile.display-name.register".into(),
        modality,
        locale: "ja".into(),
        template: value.into(),
    }
}
