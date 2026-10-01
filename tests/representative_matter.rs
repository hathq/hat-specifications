// Added by the HAT Specifications project, 2026.
// Purpose: prove base representatives, bounded children, matters, and gradual automation.

use hat_specifications::*;
use std::collections::BTreeSet;

const DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn reference(id: &str) -> SemanticStableRef {
    SemanticStableRef {
        schema: "hathq://test/reference/v1".into(),
        id: id.into(),
        revision: 1,
        digest_sha256: DIGEST.into(),
    }
}
fn decision(revision: u64) -> OwnerDecisionReference {
    OwnerDecisionReference {
        decision_id: format!("decision-{revision}"),
        decided_by_subject_id: "owner".into(),
        decision_revision: revision,
        decision_digest_sha256: DIGEST.into(),
    }
}
fn role(id: &str, archetype: &str, parent: Option<&str>) -> RepresentativeRoleInstance {
    RepresentativeRoleInstance {
        schema: REPRESENTATIVE_ROLE_INSTANCE_SCHEMA.into(),
        instance_id: id.into(),
        archetype_id: archetype.into(),
        parent_instance_id: parent.map(str::to_owned),
        owner_subject_id: "owner".into(),
        display_name: id.into(),
        description: "Bounded representative role".into(),
        scope_refs: vec![reference("scope-personal")],
        context_refs: vec![reference("context-personal")],
        hat_binding_refs: vec![reference("binding-base")],
        automation_policy_ref: None,
        revision: 1,
        state: RepresentativeRoleState::Active,
    }
}

#[test]
fn seeds_exactly_five_base_archetypes_with_stable_code_names() {
    let values = base_representative_role_archetypes();
    assert_eq!(values.len(), 5);
    assert_eq!(
        values
            .iter()
            .map(|v| v.archetype_id.as_str())
            .collect::<Vec<_>>(),
        [
            "white-queen",
            "white-rabbit",
            "white-knight",
            "cheshire-cat",
            "alice"
        ]
    );
    assert!(
        values
            .iter()
            .all(|value| validate_representative_role_archetype(value).is_ok())
    );
    let mut renamed = role("queen-personal", "white-queen", None);
    renamed.display_name = "My decision steward".into();
    assert_eq!(renamed.archetype_id, "white-queen");
    assert!(validate_representative_role_instance(&renamed).is_ok());
}

#[test]
fn child_roles_cannot_expand_or_cycle_parent_boundaries() {
    let parent = role("rabbit-personal", "white-rabbit", None);
    let child = role(
        "rabbit-development",
        "white-rabbit",
        Some("rabbit-personal"),
    );
    assert_eq!(
        validate_representative_role_hierarchy(&[parent.clone(), child.clone()]),
        Ok(())
    );
    let mut wider = child.clone();
    wider.hat_binding_refs.push(reference("binding-extra"));
    assert!(validate_representative_role_hierarchy(&[parent.clone(), wider]).is_err());
    let mut cyclic = parent;
    cyclic.parent_instance_id = Some(child.instance_id.clone());
    assert!(validate_representative_role_hierarchy(&[cyclic, child]).is_err());
}

#[test]
fn one_matter_accepts_only_declared_role_contribution_kinds() {
    let queen = role("queen-personal", "white-queen", None);
    let archetype = base_representative_role_archetypes().remove(0);
    let matter = Matter {
        schema: MATTER_SCHEMA.into(),
        matter_id: "matter-japan-setup".into(),
        revision: 1,
        title: "Enable Japan public information".into(),
        owner_subject_id: "owner".into(),
        subject_ref: reference("subject-owner"),
        scope_refs: vec![reference("scope-personal")],
        goal_ref: reference("goal-japan-setup"),
        participant_role_instance_ids: BTreeSet::from([queen.instance_id.clone()]),
        contribution_refs: Vec::new(),
        pending_question_refs: vec![reference("question-install")],
        decision_ref: None,
        state: MatterState::WaitingOwner,
        created_at_epoch_s: 1,
        updated_at_epoch_s: 1,
    };
    assert_eq!(validate_matter(&matter), Ok(()));
    let mut contribution = MatterContribution {
        schema: MATTER_CONTRIBUTION_SCHEMA.into(),
        contribution_id: "proposal-japan-setup".into(),
        revision: 1,
        matter_id: matter.matter_id.clone(),
        role_instance_id: queen.instance_id.clone(),
        kind: MatterContributionKind::Proposal,
        content_ref: reference("proposal-content"),
        evidence_refs: vec![reference("official-catalog")],
        supersedes_contribution_id: None,
        recorded_at_epoch_s: 2,
        content_digest_sha256: DIGEST.into(),
    };
    assert_eq!(
        validate_matter_contribution(&contribution, &matter, &queen, &archetype),
        Ok(())
    );
    contribution.kind = MatterContributionKind::ExternalExchange;
    assert!(validate_matter_contribution(&contribution, &matter, &queen, &archetype).is_err());
}

#[test]
fn automation_promotes_one_owner_approved_stage_and_can_downgrade_immediately() {
    let policy = |revision, mode| AutomationPolicy {
        schema: AUTOMATION_POLICY_SCHEMA.into(),
        policy_id: "policy-japan-refresh".into(),
        revision,
        role_instance_id: "rabbit-personal".into(),
        operation_id: "hathq://operation/japan-government-refresh/v1".into(),
        scope_refs: vec![reference("scope-personal")],
        condition_refs: vec![reference("condition-official-signed")],
        mode,
        valid_from_epoch_s: 1,
        valid_until_epoch_s: 1000,
        decision: decision(revision),
    };
    let propose = policy(1, DelegationMode::Propose);
    let conditional = policy(2, DelegationMode::Conditional);
    assert_eq!(
        validate_automation_policy_transition(&propose, &conditional),
        Ok(())
    );
    let skipped = policy(2, DelegationMode::Delegated);
    assert!(validate_automation_policy_transition(&propose, &skipped).is_err());
    let observe = policy(3, DelegationMode::Observe);
    assert_eq!(
        validate_automation_policy_transition(&conditional, &observe),
        Ok(())
    );
}
