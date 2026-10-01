use hat_specifications::{
    HatSemanticContribution, SEMANTIC_CONTRIBUTION_SCHEMA, SemanticBinding, SemanticEntity,
    SemanticStableRef, foundation_semantic_catalog,
    validate_semantic_contribution_against_catalogs,
};
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn accepts_only_explicitly_accepted_catalog_references() {
    let catalog = foundation_semantic_catalog();
    let contribution = fixture(catalog.identity.clone());
    assert_eq!(
        validate_semantic_contribution_against_catalogs(&contribution, &[catalog]),
        Ok(())
    );

    let rejected = foundation_semantic_catalog();
    let mut contribution = fixture(rejected.identity.clone());
    contribution.catalogs[0].digest_sha256 = hex('9');
    assert!(validate_semantic_contribution_against_catalogs(&contribution, &[rejected]).is_err());
}

#[test]
fn rejects_path_binding_and_unknown_or_wrong_kind_terms() {
    let catalog = foundation_semantic_catalog();
    let mut contribution = fixture(catalog.identity.clone());
    contribution.bindings[0].scope_ref.id = "/home/user/repository".into();
    assert!(
        validate_semantic_contribution_against_catalogs(
            &contribution,
            std::slice::from_ref(&catalog)
        )
        .is_err()
    );

    let mut contribution = fixture(catalog.identity.clone());
    contribution.entities[0].types = BTreeSet::from(["world.unknown".into()]);
    assert!(
        validate_semantic_contribution_against_catalogs(
            &contribution,
            std::slice::from_ref(&catalog)
        )
        .is_err()
    );

    contribution.entities[0].types = BTreeSet::from(["core.event.observation".into()]);
    assert!(validate_semantic_contribution_against_catalogs(&contribution, &[catalog]).is_err());
}

fn fixture(catalog: hat_specifications::SemanticCatalogReference) -> HatSemanticContribution {
    HatSemanticContribution {
        schema: SEMANTIC_CONTRIBUTION_SCHEMA.into(),
        producer_hat: reference("hathq://hat/package/v2", "hat:fixture", 'a'),
        canonical_source: reference("zixcel://source/v1", "source:fixture", 'b'),
        source_revision: 1,
        content_digest_sha256: hex('c'),
        catalogs: vec![catalog],
        sources: vec![],
        entities: vec![SemanticEntity {
            identity: reference("hathq://semantic/entity/v1", "person:self", '9'),
            types: BTreeSet::from(["world.person".into()]),
            properties: BTreeMap::new(),
        }],
        relations: vec![],
        events: vec![],
        assertions: vec![],
        evidence: vec![],
        bindings: vec![SemanticBinding {
            binding_id: "binding:fixture".into(),
            subject_ref: reference("hathq://subject/v1", "subject:self", 'd'),
            scope_ref: reference("zixcel://scope/v1", "repository:hatter", 'e'),
            hat_package_ref: reference("hathq://hat/package/v2", "hat:fixture", 'a'),
            fitting_ref: reference("hathq://hat/fitting/v1", "fitting:default", 'f'),
            policy_ref: reference("hathq://policy/v1", "policy:default", '1'),
            provider_ref: reference("zixcel://provider/v1", "provider:source", '2'),
            revision: 1,
            active: true,
        }],
    }
}

fn reference(schema: &str, id: &str, byte: char) -> SemanticStableRef {
    SemanticStableRef {
        schema: schema.into(),
        id: id.into(),
        revision: 1,
        digest_sha256: hex(byte),
    }
}

fn hex(byte: char) -> String {
    byte.to_string().repeat(64)
}
