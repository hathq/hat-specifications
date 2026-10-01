// Added by the HAT Specifications project, 2026.
// Purpose: prove the provider-, model-, language-, and domain-neutral operational loop.

use hat_specifications::*;

const DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn reference(id: &str) -> SemanticStableRef {
    SemanticStableRef {
        schema: "hathq://semantic/reference/v1".into(),
        id: id.into(),
        revision: 1,
        digest_sha256: DIGEST.into(),
    }
}

fn term(id: &str, kind: SemanticTermKind) -> SemanticTermReference {
    SemanticTermReference {
        catalog_id: "hathq.test.operational".into(),
        catalog_digest_sha256: DIGEST.into(),
        term_id: id.into(),
        version: 1,
        definition_digest_sha256: DIGEST.into(),
        kind,
    }
}

fn evidence() -> Vec<EvidenceReference> {
    vec![EvidenceReference {
        owner_id: "owner".into(),
        reference: "evidence/1".into(),
        digest_sha256: DIGEST.into(),
    }]
}

fn decision() -> OwnerDecisionReference {
    OwnerDecisionReference {
        decision_id: "decision-1".into(),
        decided_by_subject_id: "owner".into(),
        decision_revision: 1,
        decision_digest_sha256: DIGEST.into(),
    }
}

fn desired_state() -> DesiredState {
    DesiredState {
        schema: DESIRED_STATE_SCHEMA.into(),
        desired_state_id: "desired-state-1".into(),
        revision: 1,
        subject: reference("subject:owner"),
        requirements: vec![reference("clause:required-state")],
        valid_from_epoch_s: 100,
        valid_until_epoch_s: Some(1_000),
        evidence_refs: evidence(),
    }
}

fn goal(origin: GoalOrigin) -> Goal {
    let derived = origin == GoalOrigin::HatDerived;
    Goal {
        schema: GOAL_SCHEMA.into(),
        goal_id: if derived { "goal-child" } else { "goal-owner" }.into(),
        revision: 1,
        owner_subject: reference("subject:owner"),
        origin,
        parent_goal: derived.then(|| reference("goal:owner")),
        proposed_by_hat: derived.then(|| reference("hat:planner")),
        desired_state: reference("desired-state:1"),
        constraint_refs: vec![reference("constraint:1")],
        state: GoalState::Active,
        evidence_refs: evidence(),
    }
}

fn commitment() -> Commitment {
    Commitment {
        schema: COMMITMENT_SCHEMA.into(),
        commitment_id: "commitment-1".into(),
        revision: 1,
        goal: reference("goal:owner"),
        desired_state: reference("desired-state:1"),
        committed_by: reference("subject:owner"),
        decision: decision(),
        valid_from_epoch_s: 100,
        valid_until_epoch_s: 1_000,
        state: CommitmentState::Active,
    }
}

fn task(state: TaskState) -> Task {
    Task {
        schema: TASK_SCHEMA.into(),
        task_id: "task-1".into(),
        revision: 1,
        goal: reference("goal:owner"),
        commitment: reference("commitment:1"),
        derived_by_hat: reference("hat:planner"),
        derived_from_requirement: reference("clause:required-state"),
        derivation_rule: reference("rule:task-derivation"),
        observation_refs: vec![reference("observation:current-state")],
        operation: term(
            "hathq://vocabulary/action/test/v1",
            SemanticTermKind::Action,
        ),
        capability: term(
            "hathq://vocabulary/capability/test/v1",
            SemanticTermKind::Concept,
        ),
        scope_refs: vec![reference("scope:one")],
        dependency_refs: vec![],
        constraint_refs: vec![reference("constraint:1")],
        state,
        delegation: matches!(
            state,
            TaskState::Ready
                | TaskState::Executing
                | TaskState::AwaitingVerification
                | TaskState::Completed
        )
        .then(|| reference("delegation:delegation-1")),
        verified_by: (state == TaskState::Completed)
            .then(|| reference("verification:verification-1")),
    }
}

fn delegation() -> Delegation {
    Delegation {
        schema: DELEGATION_SCHEMA.into(),
        delegation_id: "delegation-1".into(),
        revision: 1,
        task: reference("task:task-1"),
        delegator: reference("subject:owner"),
        delegate: reference("hat:operator"),
        capability: task(TaskState::Candidate).capability,
        scope_refs: vec![reference("scope:one")],
        condition_refs: vec![reference("constraint:1")],
        valid_from_epoch_s: 100,
        valid_until_epoch_s: 1_000,
        maximum_executions: 1,
        state: DelegationState::Active,
        decision: decision(),
    }
}

#[test]
fn task_derivation_is_explicit_and_cannot_be_replaced_by_an_llm_guess() {
    let value = task(TaskState::Candidate);
    assert!(validate_task(&value).valid);
    let mut missing_observation = value.clone();
    missing_observation.observation_refs.clear();
    assert!(!validate_task(&missing_observation).valid);
    let mut missing_rule = value;
    missing_rule.derivation_rule.id.clear();
    assert!(!validate_task(&missing_rule).valid);
}

fn execution() -> Execution {
    Execution {
        schema: EXECUTION_SCHEMA.into(),
        execution_id: "execution-1".into(),
        revision: 1,
        task: reference("task:task-1"),
        delegation: reference("delegation:delegation-1"),
        provider: reference("provider:external"),
        capability: task(TaskState::Candidate).capability,
        operation: task(TaskState::Candidate).operation,
        request: reference("request:1"),
        result: Some(reference("result:1")),
        started_at_epoch_s: 200,
        finished_at_epoch_s: Some(210),
        outcome: ExecutionOutcome::Succeeded,
        evidence_refs: evidence(),
    }
}

fn verification(outcome: VerificationOutcome) -> Verification {
    Verification {
        schema: VERIFICATION_SCHEMA.into(),
        verification_id: "verification-1".into(),
        revision: 1,
        task: reference("task:task-1"),
        execution: reference("execution:1"),
        desired_state: reference("desired-state:1"),
        verifier: reference("hat:verifier"),
        observation_refs: vec![reference("observation:1")],
        outcome,
        verified_at_epoch_s: 220,
        evidence_refs: evidence(),
    }
}

#[test]
fn owner_goal_and_hat_derived_subgoal_are_distinct() {
    assert!(validate_goal(&goal(GoalOrigin::UserDeclared)).valid);
    assert!(validate_goal(&goal(GoalOrigin::HatDerived)).valid);

    let mut fabricated = goal(GoalOrigin::UserDeclared);
    fabricated.proposed_by_hat = Some(reference("hat:planner"));
    assert!(!validate_goal(&fabricated).valid);

    let mut orphan = goal(GoalOrigin::HatDerived);
    orphan.parent_goal = None;
    assert!(!validate_goal(&orphan).valid);
}

#[test]
fn desired_state_and_commitment_are_bounded_canonical_records() {
    assert!(validate_desired_state(&desired_state()).valid);
    assert!(validate_commitment(&commitment()).valid);

    let mut empty = desired_state();
    empty.requirements.clear();
    assert!(!validate_desired_state(&empty).valid);

    let mut wrong_owner = commitment();
    wrong_owner.decision.decided_by_subject_id = "subject-other".into();
    assert!(!validate_commitment(&wrong_owner).valid);
}

#[test]
fn local_intelligence_can_only_emit_non_authoritative_artifacts() {
    for kind in [
        IntelligenceArtifactKind::Observation,
        IntelligenceArtifactKind::Candidate,
        IntelligenceArtifactKind::Projection,
        IntelligenceArtifactKind::Proposal,
    ] {
        let artifact = IntelligenceArtifact {
            schema: INTELLIGENCE_ARTIFACT_SCHEMA.into(),
            artifact_id: "artifact-1".into(),
            revision: 1,
            kind,
            producer: reference("provider:local-inference"),
            subject: reference("subject:owner"),
            payload: reference("semantic-clause:1"),
            evidence_refs: evidence(),
        };
        assert!(validate_intelligence_artifact(&artifact).valid);
        assert!(!artifact.authorizes_effect());
    }

    let mut value = serde_json::to_value(IntelligenceArtifact {
        schema: INTELLIGENCE_ARTIFACT_SCHEMA.into(),
        artifact_id: "artifact-1".into(),
        revision: 1,
        kind: IntelligenceArtifactKind::Proposal,
        producer: reference("provider:local-inference"),
        subject: reference("subject:owner"),
        payload: reference("semantic-clause:1"),
        evidence_refs: evidence(),
    })
    .expect("serialize");
    value
        .as_object_mut()
        .expect("object")
        .insert("execute".into(), serde_json::json!(true));
    assert!(serde_json::from_value::<IntelligenceArtifact>(value).is_err());
}

#[test]
fn delegation_authorizes_only_the_exact_task_capability_scope_and_time() {
    let value = delegation();
    assert!(validate_delegation(&value).valid);
    assert_eq!(authorize_task(&task(TaskState::Ready), &value, 500), Ok(()));

    let mut widened = task(TaskState::Ready);
    widened.scope_refs.push(reference("scope:two"));
    assert!(authorize_task(&widened, &value, 500).is_err());

    let mut different = task(TaskState::Ready);
    different.capability = term(
        "hathq://vocabulary/capability/other/v1",
        SemanticTermKind::Concept,
    );
    assert!(authorize_task(&different, &value, 500).is_err());
    assert!(authorize_task(&task(TaskState::Ready), &value, 1_001).is_err());
}

#[test]
fn a_child_delegation_cannot_expand_parent_authority() {
    let parent = delegation();
    let mut child = parent.clone();
    child.delegation_id = "delegation-child".into();
    child.revision = 2;
    child.delegate = reference("hat:child-operator");
    child.valid_from_epoch_s = 200;
    child.valid_until_epoch_s = 900;
    assert_eq!(validate_delegation_subset(&parent, &child), Ok(()));

    child.scope_refs.push(reference("scope:two"));
    assert!(validate_delegation_subset(&parent, &child).is_err());
}

#[test]
fn successful_execution_waits_for_independent_verification() {
    assert!(validate_execution(&execution()).valid);
    let waiting = task(TaskState::AwaitingVerification);
    assert_ne!(waiting.state, TaskState::Completed);
    assert!(complete_task(&waiting, &verification(VerificationOutcome::Unsatisfied)).is_err());

    let completed = complete_task(&waiting, &verification(VerificationOutcome::Satisfied))
        .expect("verified completion");
    assert_eq!(completed.state, TaskState::Completed);
    assert_eq!(completed.revision, waiting.revision + 1);
    assert_eq!(
        completed.verified_by,
        Some(SemanticStableRef {
            schema: VERIFICATION_SCHEMA.into(),
            id: "verification:verification-1".into(),
            revision: 1,
            digest_sha256: DIGEST.into(),
        })
    );
}

#[test]
fn provider_names_models_and_human_language_are_not_operational_authority() {
    let mut value = serde_json::to_value(execution()).expect("serialize");
    let object = value.as_object_mut().expect("object");
    object.insert("model".into(), serde_json::json!("gpt-example"));
    object.insert("prompt".into(), serde_json::json!("please do it"));
    object.insert("locale".into(), serde_json::json!("ja"));
    assert!(serde_json::from_value::<Execution>(value).is_err());
}
