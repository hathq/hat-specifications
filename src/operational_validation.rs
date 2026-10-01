// Added by the HAT Specifications project, 2026.
// Purpose: enforce exact authority and mandatory verification for the operational loop.

use crate::semantic_validation::{reference, token};
use crate::{
    COMMITMENT_SCHEMA, Commitment, DELEGATION_SCHEMA, DESIRED_STATE_SCHEMA, Delegation,
    DelegationState, EXECUTION_SCHEMA, Execution, ExecutionOutcome, GOAL_SCHEMA, Goal, GoalOrigin,
    INTELLIGENCE_ARTIFACT_SCHEMA, IntelligenceArtifact, SemanticStableRef, SemanticTermKind,
    TASK_SCHEMA, Task, TaskState, VERIFICATION_SCHEMA, Validation, Verification,
    VerificationOutcome, validate_semantic_term_reference,
};
use std::collections::BTreeSet;

#[must_use]
pub fn validate_desired_state(value: &crate::DesiredState) -> Validation {
    let mut findings = Vec::new();
    identity(
        &value.schema,
        DESIRED_STATE_SCHEMA,
        &value.desired_state_id,
        value.revision,
        "desired state",
        &mut findings,
    );
    refs(&[&value.subject], "desired state subject", &mut findings);
    bounded_refs(
        &value.requirements,
        1,
        128,
        "desired state requirements",
        &mut findings,
    );
    if value.valid_from_epoch_s == 0
        || value
            .valid_until_epoch_s
            .is_some_and(|end| end <= value.valid_from_epoch_s)
    {
        findings.push("desired state interval is invalid".into());
    }
    evidence(&value.evidence_refs, &mut findings);
    crate::model::validation(findings)
}

#[must_use]
pub fn validate_goal(value: &Goal) -> Validation {
    let mut findings = Vec::new();
    identity(
        &value.schema,
        GOAL_SCHEMA,
        &value.goal_id,
        value.revision,
        "goal",
        &mut findings,
    );
    refs(
        &[&value.owner_subject, &value.desired_state],
        "goal reference",
        &mut findings,
    );
    bounded_refs(
        &value.constraint_refs,
        0,
        128,
        "goal constraints",
        &mut findings,
    );
    match (
        value.origin,
        value.parent_goal.as_ref(),
        value.proposed_by_hat.as_ref(),
    ) {
        (GoalOrigin::UserDeclared, None, None) => {}
        (GoalOrigin::HatDerived, Some(parent), Some(hat)) => {
            refs(&[parent, hat], "derived goal reference", &mut findings);
        }
        _ => findings.push("goal origin boundary is invalid".into()),
    }
    evidence(&value.evidence_refs, &mut findings);
    crate::model::validation(findings)
}

#[must_use]
pub fn validate_commitment(value: &Commitment) -> Validation {
    let mut findings = Vec::new();
    identity(
        &value.schema,
        COMMITMENT_SCHEMA,
        &value.commitment_id,
        value.revision,
        "commitment",
        &mut findings,
    );
    refs(
        &[&value.goal, &value.desired_state, &value.committed_by],
        "commitment reference",
        &mut findings,
    );
    if value.valid_from_epoch_s == 0 || value.valid_until_epoch_s <= value.valid_from_epoch_s {
        findings.push("commitment interval is invalid".into());
    }
    if value.decision.decided_by_subject_id != subject_id(&value.committed_by)
        || value.decision.decision_revision == 0
        || !digest(&value.decision.decision_digest_sha256)
        || token(&value.decision.decision_id, "commitment decision id").is_err()
    {
        findings.push("commitment owner decision is invalid".into());
    }
    crate::model::validation(findings)
}

#[must_use]
pub fn validate_task(value: &Task) -> Validation {
    let mut findings = Vec::new();
    identity(
        &value.schema,
        TASK_SCHEMA,
        &value.task_id,
        value.revision,
        "task",
        &mut findings,
    );
    refs(
        &[
            &value.goal,
            &value.commitment,
            &value.derived_by_hat,
            &value.derived_from_requirement,
            &value.derivation_rule,
        ],
        "task reference",
        &mut findings,
    );
    bounded_refs(
        &value.observation_refs,
        1,
        128,
        "task derivation observations",
        &mut findings,
    );
    exact_term(
        &value.operation,
        SemanticTermKind::Action,
        "task operation",
        &mut findings,
    );
    exact_term(
        &value.capability,
        SemanticTermKind::Concept,
        "task capability",
        &mut findings,
    );
    bounded_refs(&value.scope_refs, 1, 64, "task scopes", &mut findings);
    bounded_refs(
        &value.dependency_refs,
        0,
        128,
        "task dependencies",
        &mut findings,
    );
    bounded_refs(
        &value.constraint_refs,
        0,
        128,
        "task constraints",
        &mut findings,
    );
    match value.state {
        TaskState::Candidate | TaskState::AwaitingDecision | TaskState::Blocked => {
            if value.delegation.is_some() || value.verified_by.is_some() {
                findings.push("inactive task carries authority or verification".into());
            }
        }
        TaskState::Ready
        | TaskState::Executing
        | TaskState::AwaitingVerification
        | TaskState::Exception
        | TaskState::Cancelled => {
            if invalid_optional_ref(value.delegation.as_ref(), true) || value.verified_by.is_some()
            {
                findings.push("active task authority boundary is invalid".into());
            }
        }
        TaskState::Completed => {
            if invalid_optional_ref(value.delegation.as_ref(), true)
                || invalid_optional_ref(value.verified_by.as_ref(), true)
            {
                findings.push("completed task lacks authority or verification".into());
            }
        }
    }
    crate::model::validation(findings)
}

#[must_use]
pub fn validate_delegation(value: &Delegation) -> Validation {
    let mut findings = Vec::new();
    identity(
        &value.schema,
        DELEGATION_SCHEMA,
        &value.delegation_id,
        value.revision,
        "delegation",
        &mut findings,
    );
    refs(
        &[&value.task, &value.delegator, &value.delegate],
        "delegation reference",
        &mut findings,
    );
    exact_term(
        &value.capability,
        SemanticTermKind::Concept,
        "delegation capability",
        &mut findings,
    );
    bounded_refs(&value.scope_refs, 1, 64, "delegation scopes", &mut findings);
    bounded_refs(
        &value.condition_refs,
        0,
        128,
        "delegation conditions",
        &mut findings,
    );
    if value.valid_from_epoch_s == 0
        || value.valid_until_epoch_s <= value.valid_from_epoch_s
        || value.maximum_executions == 0
        || value.maximum_executions > 1_000
    {
        findings.push("delegation bounds are invalid".into());
    }
    if value.decision.decided_by_subject_id != subject_id(&value.delegator)
        || value.decision.decision_revision == 0
        || !digest(&value.decision.decision_digest_sha256)
        || token(&value.decision.decision_id, "delegation decision id").is_err()
    {
        findings.push("delegation owner decision is invalid".into());
    }
    crate::model::validation(findings)
}

/// Verifies the exact active delegation intersection for one task and instant.
///
/// # Errors
/// Returns an error when the task or delegation is invalid or any authority dimension differs.
pub fn authorize_task(
    task: &Task,
    delegation: &Delegation,
    now_epoch_s: u64,
) -> Result<(), String> {
    if !validate_task(task).valid
        || !validate_delegation(delegation).valid
        || delegation.state != DelegationState::Active
        || !(delegation.valid_from_epoch_s..=delegation.valid_until_epoch_s).contains(&now_epoch_s)
        || task.capability != delegation.capability
        || !task
            .delegation
            .as_ref()
            .is_some_and(|item| ref_id_matches(item, &delegation.delegation_id))
        || !subset(&task.scope_refs, &delegation.scope_refs)
        || !subset(&task.constraint_refs, &delegation.condition_refs)
    {
        return Err("delegation does not authorize the exact task".into());
    }
    Ok(())
}

/// Verifies that a child delegation does not expand its parent.
///
/// # Errors
/// Returns an error for an invalid record or wider capability, scope, condition, time or count.
pub fn validate_delegation_subset(parent: &Delegation, child: &Delegation) -> Result<(), String> {
    if !validate_delegation(parent).valid
        || !validate_delegation(child).valid
        || parent.state != DelegationState::Active
        || child.state != DelegationState::Active
        || parent.capability != child.capability
        || child.valid_from_epoch_s < parent.valid_from_epoch_s
        || child.valid_until_epoch_s > parent.valid_until_epoch_s
        || child.maximum_executions > parent.maximum_executions
        || !subset(&child.scope_refs, &parent.scope_refs)
        || !subset(&child.condition_refs, &parent.condition_refs)
    {
        return Err("child delegation expands parent authority".into());
    }
    Ok(())
}

#[must_use]
pub fn validate_execution(value: &Execution) -> Validation {
    let mut findings = Vec::new();
    identity(
        &value.schema,
        EXECUTION_SCHEMA,
        &value.execution_id,
        value.revision,
        "execution",
        &mut findings,
    );
    refs(
        &[
            &value.task,
            &value.delegation,
            &value.provider,
            &value.request,
        ],
        "execution reference",
        &mut findings,
    );
    if invalid_optional_ref(value.result.as_ref(), false) {
        findings.push("execution result is invalid".into());
    }
    exact_term(
        &value.capability,
        SemanticTermKind::Concept,
        "execution capability",
        &mut findings,
    );
    exact_term(
        &value.operation,
        SemanticTermKind::Action,
        "execution operation",
        &mut findings,
    );
    if value.started_at_epoch_s == 0
        || value
            .finished_at_epoch_s
            .is_some_and(|finished| finished < value.started_at_epoch_s)
        || matches!(value.outcome, ExecutionOutcome::Succeeded) && value.result.is_none()
    {
        findings.push("execution lifecycle is invalid".into());
    }
    evidence(&value.evidence_refs, &mut findings);
    crate::model::validation(findings)
}

#[must_use]
pub fn validate_verification(value: &Verification) -> Validation {
    let mut findings = Vec::new();
    identity(
        &value.schema,
        VERIFICATION_SCHEMA,
        &value.verification_id,
        value.revision,
        "verification",
        &mut findings,
    );
    refs(
        &[
            &value.task,
            &value.execution,
            &value.desired_state,
            &value.verifier,
        ],
        "verification reference",
        &mut findings,
    );
    bounded_refs(
        &value.observation_refs,
        1,
        128,
        "verification observations",
        &mut findings,
    );
    if value.verified_at_epoch_s == 0 {
        findings.push("verification time is invalid".into());
    }
    evidence(&value.evidence_refs, &mut findings);
    crate::model::validation(findings)
}

/// Applies the only successful task-completion transition.
///
/// # Errors
/// Returns an error unless an exact, valid, satisfied verification follows an awaiting task.
pub fn complete_task(task: &Task, verification: &Verification) -> Result<Task, String> {
    if !validate_task(task).valid
        || !validate_verification(verification).valid
        || task.state != TaskState::AwaitingVerification
        || verification.outcome != VerificationOutcome::Satisfied
        || !ref_id_matches(&verification.task, &task.task_id)
    {
        return Err("task completion requires satisfied exact verification".into());
    }
    let mut completed = task.clone();
    completed.revision = completed
        .revision
        .checked_add(1)
        .ok_or("task revision overflow")?;
    completed.state = TaskState::Completed;
    completed.verified_by = Some(SemanticStableRef {
        schema: VERIFICATION_SCHEMA.into(),
        id: format!("verification:{}", verification.verification_id),
        revision: verification.revision,
        digest_sha256: verification.evidence_refs[0].digest_sha256.clone(),
    });
    if !validate_task(&completed).valid {
        return Err("completed task is invalid".into());
    }
    Ok(completed)
}

#[must_use]
pub fn validate_intelligence_artifact(value: &IntelligenceArtifact) -> Validation {
    let mut findings = Vec::new();
    identity(
        &value.schema,
        INTELLIGENCE_ARTIFACT_SCHEMA,
        &value.artifact_id,
        value.revision,
        "intelligence artifact",
        &mut findings,
    );
    refs(
        &[&value.producer, &value.subject, &value.payload],
        "intelligence artifact reference",
        &mut findings,
    );
    evidence(&value.evidence_refs, &mut findings);
    crate::model::validation(findings)
}

fn identity(
    schema: &str,
    expected: &str,
    id: &str,
    revision: u64,
    label: &str,
    findings: &mut Vec<String>,
) {
    if schema != expected || token(id, label).is_err() || revision == 0 {
        findings.push(format!("{label} identity is invalid"));
    }
}

fn bounded_refs(
    values: &[SemanticStableRef],
    min: usize,
    max: usize,
    label: &str,
    findings: &mut Vec<String>,
) {
    let unique = values
        .iter()
        .map(|item| (&item.schema, &item.id, item.revision, &item.digest_sha256))
        .collect::<BTreeSet<_>>();
    if values.len() < min
        || values.len() > max
        || unique.len() != values.len()
        || values.iter().any(|item| reference(item).is_err())
    {
        findings.push(format!("{label} are invalid"));
    }
}

fn refs(values: &[&SemanticStableRef], label: &str, findings: &mut Vec<String>) {
    if values.iter().any(|item| reference(item).is_err()) {
        findings.push(format!("{label} is invalid"));
    }
}

fn evidence(values: &[crate::EvidenceReference], findings: &mut Vec<String>) {
    if values.is_empty()
        || values.len() > 128
        || values.iter().any(|item| {
            token(&item.owner_id, "evidence owner").is_err()
                || item.reference.is_empty()
                || item.reference.len() > 512
                || !digest(&item.digest_sha256)
        })
    {
        findings.push("operational evidence is invalid".into());
    }
}

fn exact_term(
    value: &crate::SemanticTermReference,
    kind: SemanticTermKind,
    label: &str,
    findings: &mut Vec<String>,
) {
    if validate_semantic_term_reference(value).is_err() || value.kind != kind {
        findings.push(format!("{label} is invalid"));
    }
}

fn invalid_optional_ref(value: Option<&SemanticStableRef>, required: bool) -> bool {
    value.is_some_and(|item| reference(item).is_err()) || required && value.is_none()
}

fn digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn subject_id(value: &SemanticStableRef) -> &str {
    value.id.strip_prefix("subject:").unwrap_or(&value.id)
}
fn subset(values: &[SemanticStableRef], bounds: &[SemanticStableRef]) -> bool {
    values.iter().all(|value| bounds.contains(value))
}
fn ref_id_matches(value: &SemanticStableRef, id: &str) -> bool {
    value.id == id
        || value
            .id
            .rsplit_once(':')
            .is_some_and(|(_, suffix)| suffix == id)
}
