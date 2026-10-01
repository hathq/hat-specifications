#![forbid(unsafe_code)]
#![doc = "Transport-neutral HAT and Fitting contracts with fail-closed validation."]

mod activity_clause;
mod assertion;
mod assertion_validation;
mod communication;
mod composition_digest;
mod composition_graph;
mod composition_model;
mod composition_validation;
mod control_grant;
mod control_grant_validation;
mod doctor;
mod execution_evidence;
mod failure;
mod failure_validation;
mod federation_digest;
mod federation_model;
mod federation_validation;
mod federation_validation_support;
mod fitting;
mod foundation;
mod inference_action;
mod inference_action_validation;
mod information_axis;
mod information_axis_validation;
mod input;
mod local_registry;
mod local_registry_validation;
mod manifest;
mod model;
mod operational;
mod operational_validation;
mod operations;
mod package_input;
mod package_model;
mod package_validation;
mod perception;
mod position;
mod position_validation;
mod profile;
mod representative;
mod retention;
mod retention_validation;
mod runtime_model;
mod runtime_validation;
mod security;
mod semantic_catalog;
mod semantic_catalog_digest;
mod semantic_catalog_validation;
mod semantic_clause_validation;
mod semantic_exchange_model;
mod semantic_exchange_validation;
mod semantic_model;
mod semantic_presentation;
mod semantic_validation;
mod situation;
mod situation_validation;
mod subject;
mod subject_validation;
mod tokens;

pub use activity_clause::{
    SemanticClause, SemanticClauseKind, SemanticModality, SemanticRoleBinding,
};
pub use assertion::{KnowledgeState, PersonAssertion};
pub use assertion_validation::validate_person_assertion;
pub use communication::{
    CommunicationDirection, CommunicationProtocolReference, CommunicationRole,
    HatCommunicationCapability, communication_semantic_catalog, validate_communication_capability,
    validate_communication_protocol_reference,
};
pub use composition_digest::composition_proposal_digest;
pub use composition_model::{HatCompositionApproval, HatCompositionMember, HatCompositionProposal};
pub use composition_validation::{validate_composition_approval, validate_composition_proposal};
pub use control_grant::{ControlGrant, ControlGrantState, OwnerDecisionReference};
pub use control_grant_validation::{validate_control_grant, validate_control_grant_revocation};
pub use doctor::doctor;
pub use execution_evidence::{
    HatEvidenceRecovery, HatExecutionEvidence, HatExecutionEvidenceBinding,
    HatExecutionEvidenceStatement, validate_execution_evidence_binding,
    validate_execution_evidence_statement,
};
pub use failure::{
    HatFailureClass, HatFailureEnvelope, HatFailureRecovery, HatFailureResponsibility,
};
pub use failure_validation::{
    validate_action_failure, validate_action_failure_with_catalog, validate_failure_envelope,
    validate_failure_envelope_with_catalog,
};
pub use federation_digest::{
    action_result_digest, execution_location_digest, federation_execution_receipt_digest,
    invocation_digest, placement_selection_digest,
};
pub use federation_model::{
    HatExecutionKind, HatExecutionLocation, HatExecutionLocationSet, HatFederationExecutionReceipt,
    HatPlacementSelection,
};
pub use federation_validation::{
    validate_execution_location, validate_execution_location_set,
    validate_federation_execution_receipt, validate_federation_execution_receipt_against,
    validate_placement_selection,
};
pub use fitting::fit;
pub use foundation::foundation_semantic_catalog;
pub use inference_action::{
    DelegationMode, InferenceAction, InferenceActionState, InferenceDecision,
    InferenceDecisionState, InferenceDelegation, InferenceEffectKind, InferenceInputSnapshot,
    InferenceProcedureKind, InferenceProcedureState, InferenceProcedureStep,
    InferenceProposedEffect, InferenceTrigger, InferenceTriggerKind, InspectableInferenceInput,
    InspectableValueKind,
};
pub use inference_action_validation::validate_inference_action;
pub use information_axis::{
    ChangePolicy, InformationCoordinate, InformationSensitivity, StructuralAxis,
};
pub use information_axis_validation::validate_information_coordinate;
pub use input::{parse_manifest, parse_profile};
pub use local_registry::{
    LocalRegistryArtifact, LocalRegistryArtifactKind, LocalRegistryEntry,
    LocalRegistryInterfaceKind, LocalRegistryLedger, LocalRegistryPublication,
    LocalRegistryPurpose, LocalRegistrySource, LocalRegistrySourceKind, LocalRegistryVerification,
    LocalRegistryVerificationState, SemanticFieldMapping, SemanticMappingContract,
    SemanticMappingUnresolvedPolicy,
};
pub use local_registry_validation::{
    validate_local_registry_ledger, validate_semantic_mapping_contract,
};
pub use manifest::validate_manifest;
pub use model::{Fitting, FittingProfile, HatManifest, HatPermission, Validation};
pub use operational::{
    Commitment, CommitmentState, Delegation, DelegationState, DesiredState, Execution,
    ExecutionOutcome, Goal, GoalOrigin, GoalState, IntelligenceArtifact, IntelligenceArtifactKind,
    Task, TaskState, Verification, VerificationOutcome,
};
pub use operational_validation::{
    authorize_task, complete_task, validate_commitment, validate_delegation,
    validate_delegation_subset, validate_desired_state, validate_execution, validate_goal,
    validate_intelligence_artifact, validate_task, validate_verification,
};
pub use package_input::parse_package;
pub use package_model::{
    ContextPlan, ContextSelector, HatHandler, HatInformationSurface, HatOperation,
    HatOperationEffect, HatOperationEffectKind, HatPackage, HatProcedure, HatServiceRequirement,
    HatSetupTemplate, HatSubjectRelation, ProcedureStep, ReducerSpec, VocabularyIcon,
};
pub use package_validation::validate_package;
pub use perception::{
    InterpretationRoute, PerceptionBinding, PerceptionError, PerceptionObservation,
    PerceptionPlan, PerceptionState, plan_perception,
};
pub use position::{PositionChangeKind, PositionRecord};
pub use position_validation::{
    validate_non_conflicting_position_records, validate_position_record,
    validate_position_transition,
};
pub use profile::validate_profile;
pub use representative::{
    AUTOMATION_POLICY_SCHEMA, AutomationPolicy, MATTER_CONTRIBUTION_SCHEMA, MATTER_SCHEMA, Matter,
    MatterContribution, MatterContributionKind, MatterState, REPRESENTATIVE_ROLE_ARCHETYPE_SCHEMA,
    REPRESENTATIVE_ROLE_INSTANCE_SCHEMA, RepresentativeRoleArchetype, RepresentativeRoleInstance,
    RepresentativeRoleState, base_representative_role_archetypes, validate_automation_policy,
    validate_automation_policy_transition, validate_matter, validate_matter_contribution,
    validate_representative_role_archetype, validate_representative_role_hierarchy,
    validate_representative_role_instance,
};
pub use retention::{
    CompleteDeletionPreview, CompleteDeletionReceipt, ExternalOwnerDisclosure, HatRetentionMinimum,
    LocalDeletionClass, LostLocalCapability, RetentionPolicy, StorePurgeReceipt,
};
pub use retention_validation::{
    validate_complete_deletion_preview, validate_complete_deletion_receipt,
    validate_retention_policy,
};
pub use runtime_model::{
    ActionReference, ContextPartition, EvidenceReference, HatActionResult, HatActionStatus,
    HatBinding, HatCheckpoint, HatInvocation, HatInvocationControl, HatInvocationControlKind,
    HatInvocationOutcome, HatInvocationPhase, HatProjection, HatProjectionEvent,
    HatProjectionJournal, UnresolvedTerm,
};
pub use runtime_validation::{
    validate_action_result, validate_action_status, validate_binding, validate_checkpoint,
    validate_invocation, validate_invocation_against_package, validate_invocation_control,
    validate_projection, validate_projection_event, validate_projection_journal,
};
pub use semantic_catalog::{
    SemanticCatalog, SemanticCatalogReference, SemanticClauseFrame, SemanticClauseRole,
    SemanticLexicalization, SemanticRoleSpec, SemanticTermDefinition, SemanticTermKind,
    SemanticTermReference,
};
pub use semantic_catalog_digest::{
    semantic_catalog_digest, semantic_clause_frame_digest, semantic_exchange_digest,
    semantic_term_definition_digest,
};
pub use semantic_catalog_validation::{
    validate_semantic_catalog, validate_semantic_catalog_reference, validate_semantic_clause_frame,
    validate_semantic_lexicalization, validate_semantic_term_reference,
};
pub use semantic_clause_validation::validate_semantic_clause;
pub use semantic_exchange_model::{
    SemanticComprehensionItem, SemanticComprehensionReport, SemanticExchange,
    SemanticTermComprehension,
};
pub use semantic_exchange_validation::{classify_semantic_exchange, validate_semantic_exchange};
pub use semantic_model::{
    EventSemantics, HatSemanticContribution, SemanticAssertion, SemanticBinding, SemanticEntity,
    SemanticEvent, SemanticEvidence, SemanticInterpretation, SemanticRelation, SemanticScalar,
    SemanticSource, SemanticStableRef, SemanticTimeInterval,
};
pub use semantic_presentation::{
    SEMANTIC_PRESENTATION_CATALOG_SCHEMA, SemanticPresentationCatalog,
    SemanticPresentationTemplate, present_semantic_activity,
    validate_semantic_presentation_catalog,
};
pub use semantic_validation::validate_semantic_contribution_against_catalogs;
pub use situation::{
    SituationContribution, SituationCoordinates, SituationKind, SituationPlace, SituationSource,
    SituationStatus, SituationTime,
};
pub use situation_validation::validate_situation_contribution;
pub use subject::{SubjectKind, SubjectReference};
pub use subject_validation::{validate_distinct_subjects, validate_subject_reference};

/// Machine-readable CLI result schema.
pub const RESULT_SCHEMA: &str = "hathq://hat-specifications/result/v1";
/// HAT manifest wire schema.
pub const MANIFEST_SCHEMA: &str = "hathq://hat/manifest/v1";
/// Fitting profile wire schema.
pub const PROFILE_SCHEMA: &str = "hathq://hat/profile/v1";
/// Installed HAT package wire schema.
pub const PACKAGE_SCHEMA: &str = "hathq://hat/package/v2";
/// Exact declaration that a HAT can speak an externally owned communication protocol.
pub const COMMUNICATION_CAPABILITY_SCHEMA: &str = "hathq://hat/communication-capability/v1";
/// Immutable HAT binding wire schema.
pub const BINDING_SCHEMA: &str = "hathq://hat/binding/v2";
/// HAT invocation wire schema.
pub const INVOCATION_SCHEMA: &str = "hathq://hat/invocation/v4";
/// HAT invocation status wire schema.
pub const ACTION_STATUS_SCHEMA: &str = "hathq://hat/action-status/v2";
/// Terminal HAT invocation result wire schema.
pub const ACTION_RESULT_SCHEMA: &str = "hathq://hat/action-result/v1";
/// Bounded semantic failure reported by a HAT or Hatter component.
pub const FAILURE_ENVELOPE_SCHEMA: &str = "hathq://hat/failure-envelope/v1";
/// Status, cancellation and recovery request wire schema.
pub const INVOCATION_CONTROL_SCHEMA: &str = "hathq://hat/invocation-control/v2";
/// Vocabulary-bound HAT projection event wire schema.
pub const PROJECTION_EVENT_SCHEMA: &str = "hathq://hat/projection-event/v1";
/// Bounded context-partition projection journal page wire schema.
pub const PROJECTION_JOURNAL_SCHEMA: &str = "hathq://hat/projection-journal/v1";
/// Source-preserving event or action contribution used by aggregate views.
pub const SITUATION_CONTRIBUTION_SCHEMA: &str = "hathq://hat/situation-contribution/v1";
/// HAT projection wire schema.
pub const PROJECTION_SCHEMA: &str = "hathq://hat/projection/v2";
/// HAT checkpoint wire schema.
pub const CHECKPOINT_SCHEMA: &str = "hathq://hat/checkpoint/v2";
/// Natural-person or legal-person subject reference wire schema.
pub const SUBJECT_REFERENCE_SCHEMA: &str = "hathq://hat/subject-reference/v1";
/// Revisioned position and relationship record wire schema.
pub const POSITION_RECORD_SCHEMA: &str = "hathq://hat/position-record/v1";
/// One-user represented-subject control grant wire schema.
pub const CONTROL_GRANT_SCHEMA: &str = "hathq://hat/control-grant/v1";
/// Independent structural/change/domain/sensitivity coordinate wire schema.
pub const INFORMATION_COORDINATE_SCHEMA: &str = "hathq://hat/information-coordinate/v1";
/// Evidence-linked person assertion wire schema.
pub const PERSON_ASSERTION_SCHEMA: &str = "hathq://hat/person-assertion/v2";
/// Versioned retention selection wire schema.
pub const RETENTION_POLICY_SCHEMA: &str = "hathq://hat/retention-policy/v2";
/// Complete local deletion preview wire schema.
pub const COMPLETE_DELETION_PREVIEW_SCHEMA: &str = "hathq://hat/complete-deletion-preview/v2";
/// Subject-neutral complete local deletion receipt wire schema.
pub const COMPLETE_DELETION_RECEIPT_SCHEMA: &str = "hathq://hat/complete-deletion-receipt/v1";
/// Explicit bounded multi-HAT composition proposal wire schema.
pub const COMPOSITION_PROPOSAL_SCHEMA: &str = "hathq://hat/composition-proposal/v1";
/// Whole-proposal user approval wire schema.
pub const COMPOSITION_APPROVAL_SCHEMA: &str = "hathq://hat/composition-approval/v1";
/// Selectable logical execution location wire schema.
pub const EXECUTION_LOCATION_SCHEMA: &str = "hathq://hat/execution-location/v1";
/// Bounded set of locations bound by one signed federation-directory digest.
pub const EXECUTION_LOCATION_SET_SCHEMA: &str = "hathq://hat/execution-location-set/v1";
/// Owner-approved context-partition placement selection wire schema.
pub const PLACEMENT_SELECTION_SCHEMA: &str = "hathq://hat/placement-selection/v2";
/// Correlated receipt for one federated HAT execution.
pub const FEDERATION_RECEIPT_SCHEMA: &str = "hathq://hat/federation-execution-receipt/v1";
/// Declarative semantic contribution emitted by an exact installed HAT package.
pub const SEMANTIC_CONTRIBUTION_SCHEMA: &str = "hathq://hat/semantic-contribution/v1";
/// Provider-neutral, complete fact or action clause wire schema.
pub const SEMANTIC_CLAUSE_SCHEMA: &str = "hathq://hat/semantic-clause/v1";
/// Provider-neutral inference request wire schema.
/// Cross-owner semantic result proposal wire schema.
pub const SEMANTIC_EXCHANGE_SCHEMA: &str = "hathq://hat/semantic-exchange/v1";
/// Receiver comprehension report wire schema.
pub const SEMANTIC_COMPREHENSION_SCHEMA: &str = "hathq://hat/semantic-comprehension-report/v1";
/// Local registry provenance ledger shared by development and HAT acquisition surfaces.
pub const LOCAL_REGISTRY_LEDGER_SCHEMA: &str = "hathq://hat/local-registry-ledger/v1";
/// External-service schema to Hatter vocabulary mapping contract.
pub const SEMANTIC_MAPPING_CONTRACT_SCHEMA: &str = "hathq://hat/semantic-mapping-contract/v1";
pub const DESIRED_STATE_SCHEMA: &str = "hathq://hat/desired-state/v1";
pub const GOAL_SCHEMA: &str = "hathq://hat/goal/v1";
pub const COMMITMENT_SCHEMA: &str = "hathq://hat/commitment/v1";
pub const TASK_SCHEMA: &str = "hathq://hat/task/v2";
pub const DELEGATION_SCHEMA: &str = "hathq://hat/delegation/v1";
pub const EXECUTION_SCHEMA: &str = "hathq://hat/execution/v1";
pub const VERIFICATION_SCHEMA: &str = "hathq://hat/verification/v1";
pub const INTELLIGENCE_ARTIFACT_SCHEMA: &str = "hathq://hat/intelligence-artifact/v1";
/// User-inspectable, action-grained local or remote inference execution wire schema.
pub const INFERENCE_ACTION_SCHEMA: &str = "hathq://hat/inference-action/v1";
/// Immutable semantic vocabulary catalog wire schema.
pub const SEMANTIC_CATALOG_SCHEMA: &str = "hathq://hat/semantic-catalog/v1";
