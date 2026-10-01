// Added by the HAT Specifications project, 2026.
// Purpose: define owner-bound representative roles, decision matters, and explicit automation.

use crate::{DelegationMode, OwnerDecisionReference, SemanticStableRef, VocabularyIcon};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const REPRESENTATIVE_ROLE_ARCHETYPE_SCHEMA: &str =
    "hathq://hat/representative-role-archetype/v1";
pub const REPRESENTATIVE_ROLE_INSTANCE_SCHEMA: &str = "hathq://hat/representative-role-instance/v1";
pub const MATTER_SCHEMA: &str = "hathq://hat/matter/v1";
pub const MATTER_CONTRIBUTION_SCHEMA: &str = "hathq://hat/matter-contribution/v1";
pub const AUTOMATION_POLICY_SCHEMA: &str = "hathq://hat/automation-policy/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MatterContributionKind {
    Observation,
    ValueConcern,
    Option,
    Proposal,
    Decision,
    Instruction,
    ExternalExchange,
    Verification,
    UnresolvedQuestion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RepresentativeRoleState {
    Active,
    Suspended,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MatterState {
    Open,
    WaitingOwner,
    Decided,
    Executing,
    AwaitingVerification,
    Closed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepresentativeRoleArchetype {
    pub schema: String,
    /// Stable code name and identity. Display-name changes never alter it.
    pub archetype_id: String,
    pub initial_display_name: String,
    pub description: String,
    pub initial_icon: VocabularyIcon,
    pub contribution_kinds: BTreeSet<MatterContributionKind>,
    pub authority_ceiling: DelegationMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepresentativeRoleInstance {
    pub schema: String,
    pub instance_id: String,
    pub archetype_id: String,
    pub parent_instance_id: Option<String>,
    pub owner_subject_id: String,
    pub display_name: String,
    pub description: String,
    pub scope_refs: Vec<SemanticStableRef>,
    pub context_refs: Vec<SemanticStableRef>,
    pub hat_binding_refs: Vec<SemanticStableRef>,
    pub automation_policy_ref: Option<SemanticStableRef>,
    pub revision: u64,
    pub state: RepresentativeRoleState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Matter {
    pub schema: String,
    pub matter_id: String,
    pub revision: u64,
    pub title: String,
    pub owner_subject_id: String,
    pub subject_ref: SemanticStableRef,
    pub scope_refs: Vec<SemanticStableRef>,
    pub goal_ref: SemanticStableRef,
    pub participant_role_instance_ids: BTreeSet<String>,
    pub contribution_refs: Vec<SemanticStableRef>,
    pub pending_question_refs: Vec<SemanticStableRef>,
    pub decision_ref: Option<SemanticStableRef>,
    pub state: MatterState,
    pub created_at_epoch_s: u64,
    pub updated_at_epoch_s: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatterContribution {
    pub schema: String,
    pub contribution_id: String,
    pub revision: u64,
    pub matter_id: String,
    pub role_instance_id: String,
    pub kind: MatterContributionKind,
    pub content_ref: SemanticStableRef,
    pub evidence_refs: Vec<SemanticStableRef>,
    pub supersedes_contribution_id: Option<String>,
    pub recorded_at_epoch_s: u64,
    pub content_digest_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AutomationPolicy {
    pub schema: String,
    pub policy_id: String,
    pub revision: u64,
    pub role_instance_id: String,
    pub operation_id: String,
    pub scope_refs: Vec<SemanticStableRef>,
    pub condition_refs: Vec<SemanticStableRef>,
    pub mode: DelegationMode,
    pub valid_from_epoch_s: u64,
    pub valid_until_epoch_s: u64,
    pub decision: OwnerDecisionReference,
}

/// Returns the five immutable base archetypes. They are perspectives, not independent subjects.
#[must_use]
pub fn base_representative_role_archetypes() -> Vec<RepresentativeRoleArchetype> {
    use MatterContributionKind as Kind;
    [
        (
            "white-queen",
            "White Queen",
            "Decides within exact owner-approved boundaries.",
            VocabularyIcon::Decision,
            [Kind::Option, Kind::Proposal, Kind::Decision].as_slice(),
            DelegationMode::Delegated,
        ),
        (
            "white-rabbit",
            "White Rabbit",
            "Turns accepted decisions into bounded instructions and actions.",
            VocabularyIcon::Action,
            [Kind::Instruction].as_slice(),
            DelegationMode::Delegated,
        ),
        (
            "white-knight",
            "White Knight",
            "Prepares and performs explicitly authorized external exchanges.",
            VocabularyIcon::Communication,
            [Kind::ExternalExchange].as_slice(),
            DelegationMode::Delegated,
        ),
        (
            "cheshire-cat",
            "Cheshire Cat",
            "Observes facts, conflicts, risk, and independently verifies results.",
            VocabularyIcon::Evidence,
            [
                Kind::Observation,
                Kind::Verification,
                Kind::UnresolvedQuestion,
            ]
            .as_slice(),
            DelegationMode::Observe,
        ),
        (
            "alice",
            "Alice",
            "Raises human values, emotion, burden, and relationship concerns.",
            VocabularyIcon::Person,
            [Kind::ValueConcern, Kind::UnresolvedQuestion].as_slice(),
            DelegationMode::Propose,
        ),
    ]
    .into_iter()
    .map(
        |(id, name, description, icon, kinds, ceiling)| RepresentativeRoleArchetype {
            schema: REPRESENTATIVE_ROLE_ARCHETYPE_SCHEMA.into(),
            archetype_id: id.into(),
            initial_display_name: name.into(),
            description: description.into(),
            initial_icon: icon,
            contribution_kinds: kinds.iter().copied().collect(),
            authority_ceiling: ceiling,
        },
    )
    .collect()
}

/// Validates one immutable representative-role archetype declaration.
///
/// # Errors
///
/// Returns a stable error when identity, display metadata, or contribution kinds are invalid.
pub fn validate_representative_role_archetype(
    value: &RepresentativeRoleArchetype,
) -> Result<(), String> {
    if value.schema != REPRESENTATIVE_ROLE_ARCHETYPE_SCHEMA
        || !token(&value.archetype_id)
        || !text(&value.initial_display_name, 160)
        || !text(&value.description, 512)
        || value.contribution_kinds.is_empty()
    {
        return Err("representative role archetype is invalid".into());
    }
    Ok(())
}

/// Validates one owner-bound representative-role instance.
///
/// # Errors
///
/// Returns a stable error when identity, revision, scope, or references are invalid.
pub fn validate_representative_role_instance(
    value: &RepresentativeRoleInstance,
) -> Result<(), String> {
    if value.schema != REPRESENTATIVE_ROLE_INSTANCE_SCHEMA
        || !token(&value.instance_id)
        || !token(&value.archetype_id)
        || !token(&value.owner_subject_id)
        || value
            .parent_instance_id
            .as_ref()
            .is_some_and(|id| !token(id) || id == &value.instance_id)
        || !text(&value.display_name, 160)
        || !text(&value.description, 512)
        || value.scope_refs.is_empty()
        || value.revision == 0
        || !unique_refs(&value.scope_refs)
        || !unique_refs(&value.context_refs)
        || !unique_refs(&value.hat_binding_refs)
        || value
            .automation_policy_ref
            .as_ref()
            .is_some_and(|value| !stable_ref(value))
    {
        return Err("representative role instance is invalid".into());
    }
    Ok(())
}

/// Validates ownership, subset boundaries, and acyclicity for a role hierarchy.
///
/// # Errors
///
/// Returns a stable error for an absent parent, expanded child boundary, duplicate, or cycle.
pub fn validate_representative_role_hierarchy(
    values: &[RepresentativeRoleInstance],
) -> Result<(), String> {
    if values.is_empty() || values.len() > 256 {
        return Err("representative role hierarchy is invalid".into());
    }
    let by_id = values
        .iter()
        .map(|value| (value.instance_id.as_str(), value))
        .collect::<BTreeMap<_, _>>();
    if by_id.len() != values.len() {
        return Err("representative role identity is duplicated".into());
    }
    for value in values {
        validate_representative_role_instance(value)?;
        if let Some(parent_id) = &value.parent_instance_id {
            let parent = by_id
                .get(parent_id.as_str())
                .ok_or_else(|| "representative role parent is absent".to_owned())?;
            if value.owner_subject_id != parent.owner_subject_id
                || !refs_subset(&value.scope_refs, &parent.scope_refs)
                || !refs_subset(&value.context_refs, &parent.context_refs)
                || !refs_subset(&value.hat_binding_refs, &parent.hat_binding_refs)
            {
                return Err("child representative role expands parent boundary".into());
            }
        }
        let mut seen = BTreeSet::new();
        let mut current = Some(value.instance_id.as_str());
        while let Some(id) = current {
            if !seen.insert(id) {
                return Err("representative role hierarchy contains a cycle".into());
            }
            current = by_id
                .get(id)
                .and_then(|item| item.parent_instance_id.as_deref());
        }
    }
    Ok(())
}

/// Validates one bounded matter and its state-dependent references.
///
/// # Errors
///
/// Returns a stable error when identity, state, revision, participant, or reference data conflict.
pub fn validate_matter(value: &Matter) -> Result<(), String> {
    if value.schema != MATTER_SCHEMA
        || !token(&value.matter_id)
        || value.revision == 0
        || !text(&value.title, 240)
        || !token(&value.owner_subject_id)
        || !stable_ref(&value.subject_ref)
        || value.scope_refs.is_empty()
        || !unique_refs(&value.scope_refs)
        || !stable_ref(&value.goal_ref)
        || value.participant_role_instance_ids.is_empty()
        || value
            .participant_role_instance_ids
            .iter()
            .any(|id| !token(id))
        || value.contribution_refs.len() > 4096
        || !refs(value.contribution_refs.iter())
        || !unique_refs(&value.contribution_refs)
        || !unique_refs(&value.pending_question_refs)
        || value
            .decision_ref
            .as_ref()
            .is_some_and(|value| !stable_ref(value))
        || value.created_at_epoch_s == 0
        || value.updated_at_epoch_s < value.created_at_epoch_s
    {
        return Err("matter is invalid".into());
    }
    if matches!(value.state, MatterState::WaitingOwner) == value.pending_question_refs.is_empty()
        || matches!(
            value.state,
            MatterState::Decided
                | MatterState::Executing
                | MatterState::AwaitingVerification
                | MatterState::Closed
        ) && value.decision_ref.is_none()
    {
        return Err("matter state is inconsistent".into());
    }
    Ok(())
}

/// Validates a role contribution against the exact matter and archetype contract.
///
/// # Errors
///
/// Returns a stable error when the contribution escapes the matter or role boundary.
pub fn validate_matter_contribution(
    value: &MatterContribution,
    matter: &Matter,
    role: &RepresentativeRoleInstance,
    archetype: &RepresentativeRoleArchetype,
) -> Result<(), String> {
    if value.schema != MATTER_CONTRIBUTION_SCHEMA
        || !token(&value.contribution_id)
        || value.revision == 0
        || value.matter_id != matter.matter_id
        || value.role_instance_id != role.instance_id
        || role.archetype_id != archetype.archetype_id
        || !matter
            .participant_role_instance_ids
            .contains(&role.instance_id)
        || !archetype.contribution_kinds.contains(&value.kind)
        || !stable_ref(&value.content_ref)
        || !unique_refs(&value.evidence_refs)
        || value
            .supersedes_contribution_id
            .as_ref()
            .is_some_and(|id| !token(id) || id == &value.contribution_id)
        || value.recorded_at_epoch_s == 0
        || !digest(&value.content_digest_sha256)
    {
        return Err("matter contribution is invalid".into());
    }
    Ok(())
}

/// Validates one owner-approved automation policy.
///
/// # Errors
///
/// Returns a stable error when its identity, scope, validity, or decision proof is invalid.
pub fn validate_automation_policy(value: &AutomationPolicy) -> Result<(), String> {
    if value.schema != AUTOMATION_POLICY_SCHEMA
        || !token(&value.policy_id)
        || value.revision == 0
        || !token(&value.role_instance_id)
        || !uri(&value.operation_id)
        || value.scope_refs.is_empty()
        || !unique_refs(&value.scope_refs)
        || !unique_refs(&value.condition_refs)
        || value.valid_until_epoch_s <= value.valid_from_epoch_s
        || !token(&value.decision.decision_id)
        || !token(&value.decision.decided_by_subject_id)
        || value.decision.decision_revision == 0
        || !digest(&value.decision.decision_digest_sha256)
    {
        return Err("automation policy is invalid".into());
    }
    Ok(())
}

/// Validates a single-step promotion or any immediate downgrade of automation authority.
///
/// # Errors
///
/// Returns a stable error when identity changes, revisions do not advance, or a stage is skipped.
pub fn validate_automation_policy_transition(
    previous: &AutomationPolicy,
    next: &AutomationPolicy,
) -> Result<(), String> {
    validate_automation_policy(previous)?;
    validate_automation_policy(next)?;
    if previous.policy_id != next.policy_id
        || previous.role_instance_id != next.role_instance_id
        || previous.operation_id != next.operation_id
        || previous.scope_refs != next.scope_refs
        || next.revision != previous.revision + 1
        || next.decision.decision_revision <= previous.decision.decision_revision
    {
        return Err("automation policy transition changes identity or authority".into());
    }
    let rank = |mode| match mode {
        DelegationMode::Observe => 0,
        DelegationMode::Propose => 1,
        DelegationMode::Conditional => 2,
        DelegationMode::Delegated => 3,
    };
    if rank(next.mode) > rank(previous.mode) + 1 {
        return Err("automation policy skips an approval stage".into());
    }
    Ok(())
}

fn token(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 96
        && value
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
}
fn text(value: &str, max: usize) -> bool {
    !value.trim().is_empty() && value.len() <= max && !value.chars().any(char::is_control)
}
fn digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
}
fn uri(value: &str) -> bool {
    value.starts_with("hathq://")
        && value.len() <= 512
        && !value.chars().any(|value| matches!(value, '?' | '#' | '@'))
}
fn stable_ref(value: &SemanticStableRef) -> bool {
    uri(&value.schema)
        && !value.id.is_empty()
        && value.id.len() <= 256
        && value.revision > 0
        && digest(&value.digest_sha256)
}
fn refs<'a>(values: impl Iterator<Item = &'a SemanticStableRef>) -> bool {
    values.into_iter().all(stable_ref)
}
fn identity(value: &SemanticStableRef) -> (&str, &str, u64, &str) {
    (
        &value.schema,
        &value.id,
        value.revision,
        &value.digest_sha256,
    )
}
fn unique_refs(values: &[SemanticStableRef]) -> bool {
    refs(values.iter())
        && values.iter().enumerate().all(|(index, value)| {
            values[index + 1..]
                .iter()
                .all(|candidate| identity(value) != identity(candidate))
        })
}
fn refs_subset(values: &[SemanticStableRef], boundary: &[SemanticStableRef]) -> bool {
    values.iter().all(|value| {
        boundary
            .iter()
            .any(|candidate| identity(value) == identity(candidate))
    })
}
