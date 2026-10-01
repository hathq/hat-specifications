use hat_specifications::{SemanticTermKind, foundation_semantic_catalog};
use std::collections::BTreeSet;

#[test]
fn foundation_expresses_person_world_and_action_without_a_hat_owner() {
    let catalog = foundation_semantic_catalog();
    let terms = catalog
        .terms
        .iter()
        .map(|term| term.reference.term_id.as_str())
        .collect::<BTreeSet<_>>();
    for id in [
        "core.role",
        "core.preference",
        "core.goal",
        "core.desired-state",
        "core.commitment",
        "core.constraint",
        "core.plan",
        "core.task",
        "core.capability",
        "core.permission",
        "core.delegation",
        "core.action",
        "core.result",
        "core.provider",
        "core.execution",
        "core.verification",
        "world.person.role",
        "world.person.preference",
        "world.person.goal",
        "world.body",
        "world.body.part",
        "world.organization.role",
        "world.institution",
        "world.jurisdiction",
        "world.language",
        "world.physical-object",
        "world.digital-resource",
        "world.service",
    ] {
        assert!(terms.contains(id), "missing foundation term {id}");
    }
    assert!(
        catalog
            .terms
            .iter()
            .find(|term| term.reference.term_id == "core.action")
            .is_some_and(|term| term.reference.kind == SemanticTermKind::Action)
    );
    assert!(
        catalog
            .lexicalizations
            .iter()
            .any(|value| value.term.term_id == "world.person.role" && value.locale == "ja")
    );
}

#[test]
fn foundation_remains_owner_stable_and_contains_no_vendor_service_term() {
    let catalog = foundation_semantic_catalog();
    assert_eq!(catalog.owner_repository_id, "hat-specifications");
    assert!(catalog.terms.iter().all(|term| {
        let id = term.reference.term_id.to_ascii_lowercase();
        !id.contains("github") && !id.contains("openai") && !id.contains("chatgpt")
    }));
}
