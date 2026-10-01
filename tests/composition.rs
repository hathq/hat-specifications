use hat_specifications::{
    COMPOSITION_APPROVAL_SCHEMA, COMPOSITION_PROPOSAL_SCHEMA, HatCompositionApproval,
    HatCompositionMember, HatCompositionProposal, composition_proposal_digest,
    validate_composition_approval, validate_composition_proposal,
};

const DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn member(repository_id: &str, dependency_repository_ids: Vec<String>) -> HatCompositionMember {
    HatCompositionMember {
        repository_id: repository_id.into(),
        package_id: format!("hat/{}", repository_id.trim_start_matches("hat-")),
        package_digest_sha256: DIGEST.into(),
        catalog_digest_sha256: DIGEST.into(),
        fitting_digest_sha256: DIGEST.into(),
        policy_digest_sha256: DIGEST.into(),
        expected_binding_revision: None,
        dependency_repository_ids,
        incompatible_repository_ids: Vec::new(),
        operation_ids: vec![format!("hathq://{repository_id}/operate/v1")],
    }
}

fn proposal() -> HatCompositionProposal {
    HatCompositionProposal {
        schema: COMPOSITION_PROPOSAL_SCHEMA.into(),
        proposal_id: "composition-proposal-1".into(),
        subject_ref: "subject:self".into(),
        scope_ref: "scope:personal".into(),
        expected_composition_revision: None,
        members: vec![
            member("hat-accountant", Vec::new()),
            member("hat-budget-planner", vec!["hat-accountant".into()]),
        ],
    }
}

#[test]
fn explicit_dependency_graph_and_whole_approval_are_exact() {
    let value = proposal();
    assert!(validate_composition_proposal(&value).valid);
    let digest = composition_proposal_digest(&value).expect("digest");
    let approval = HatCompositionApproval {
        schema: COMPOSITION_APPROVAL_SCHEMA.into(),
        approval_id: "composition-approval-1".into(),
        proposal_digest_sha256: digest,
        approved_by_subject_ref: value.subject_ref,
        expected_composition_revision: None,
    };
    assert!(validate_composition_approval(&approval).valid);
}

#[test]
fn missing_cyclic_incompatible_and_duplicate_ownership_fail_closed() {
    let mut missing = proposal();
    missing.members[1].dependency_repository_ids = vec!["hat-missing".into()];
    assert!(!validate_composition_proposal(&missing).valid);

    let mut cyclic = proposal();
    cyclic.members[0].dependency_repository_ids = vec!["hat-budget-planner".into()];
    assert!(!validate_composition_proposal(&cyclic).valid);

    let mut incompatible = proposal();
    incompatible.members[0].incompatible_repository_ids = vec!["hat-budget-planner".into()];
    assert!(!validate_composition_proposal(&incompatible).valid);

    let mut duplicate = proposal();
    duplicate.members[1].operation_ids = duplicate.members[0].operation_ids.clone();
    assert!(!validate_composition_proposal(&duplicate).valid);
}

#[test]
fn unsorted_relationships_and_digest_mutation_are_rejected_or_detected() {
    let value = proposal();
    let digest = composition_proposal_digest(&value).expect("digest");
    let mut changed = value;
    changed.scope_ref = "scope:organization".into();
    assert_ne!(
        composition_proposal_digest(&changed).expect("changed"),
        digest
    );

    changed.members[1].dependency_repository_ids =
        vec!["hat-source-curator".into(), "hat-accountant".into()];
    assert!(!validate_composition_proposal(&changed).valid);
}
