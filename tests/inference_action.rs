use hat_specifications::{
    ActionReference, DelegationMode, INFERENCE_ACTION_SCHEMA, InferenceAction,
    InferenceActionState, InferenceDecision, InferenceDecisionState, InferenceDelegation,
    InferenceEffectKind, InferenceInputSnapshot, InferenceProcedureKind, InferenceProcedureState,
    InferenceProcedureStep, InferenceProposedEffect, InferenceTrigger, InferenceTriggerKind,
    InspectableInferenceInput, InspectableValueKind, validate_inference_action,
};

fn reference(owner_id: &str, schema_id: &str, digest: char) -> ActionReference {
    ActionReference {
        owner_id: owner_id.into(),
        reference: format!("local://{owner_id}/record"),
        schema_id: schema_id.into(),
        digest_sha256: digest.to_string().repeat(64),
    }
}

fn action() -> InferenceAction {
    let source = reference("mail-source", "hathq://mail/message/v1", 'a');
    InferenceAction {
        schema: INFERENCE_ACTION_SCHEMA.into(),
        action_id: "mail-triage-one".into(),
        revision: 1,
        routine_id: "mail-triage".into(),
        title_key: "inference.mail-triage.title".into(),
        producer_repository_id: "hat-mail-organizer".into(),
        invocation_id: "invocation-mail-one".into(),
        route_id: "local-qwen-router".into(),
        operation_id: "hathq://operation/mail-triage/v1".into(),
        role_id: "hathq://inference-role/semantic-intent-route/v1".into(),
        state: InferenceActionState::Waiting,
        trigger: InferenceTrigger {
            kind: InferenceTriggerKind::Event,
            source: source.clone(),
        },
        input_snapshot: InferenceInputSnapshot {
            digest_sha256: "b".repeat(64),
            items: vec![InspectableInferenceInput {
                field_id: "mail-subject".into(),
                label_key: "mail.subject".into(),
                value_kind: InspectableValueKind::Text,
                canonical_value: "月末までの確認依頼".into(),
                source,
                purpose_term_id: "hathq://purpose/task-detection/v1".into(),
                information_classification: "internal-confidential".into(),
                included: true,
            }],
        },
        procedure: vec![
            InferenceProcedureStep {
                step_id: "observe".into(),
                kind: InferenceProcedureKind::Observe,
                title_key: "inference.step.observe".into(),
                state: InferenceProcedureState::Complete,
            },
            InferenceProcedureStep {
                step_id: "confirm".into(),
                kind: InferenceProcedureKind::Propose,
                title_key: "inference.step.confirm".into(),
                state: InferenceProcedureState::Current,
            },
        ],
        decision: Some(InferenceDecision {
            state: InferenceDecisionState::Confirmed,
            candidate_ids: vec!["hathq://operation/task-create/v1".into()],
            selected_candidate_id: Some("hathq://operation/task-create/v1".into()),
            evidence_field_ids: vec!["mail-subject".into()],
            reason_id: None,
        }),
        proposed_effects: vec![InferenceProposedEffect {
            effect_id: "create-task".into(),
            kind: InferenceEffectKind::Create,
            surface_id: "hathq://surface/tasks/v1".into(),
            payload: reference("mail-triage", "hathq://task/proposal/v1", 'c'),
            requires_approval: true,
        }],
        delegation: InferenceDelegation {
            mode: DelegationMode::Propose,
            policy: reference("owner-policy", "hathq://policy/delegation/v1", 'd'),
        },
        output: None,
        reason_id: None,
        created_at_epoch_s: 1_777_600_000,
        updated_at_epoch_s: 1_777_600_010,
    }
}

#[test]
fn validates_one_user_inspectable_inference_action() {
    assert!(validate_inference_action(&action()).valid);
}

#[test]
fn rejects_hidden_input_invented_selection_and_terminal_mismatch() {
    let mut value = action();
    value.input_snapshot.items[0].included = false;
    value
        .decision
        .as_mut()
        .expect("decision")
        .selected_candidate_id = Some("hathq://operation/mail-delete/v1".into());
    value.state = InferenceActionState::Completed;
    let result = validate_inference_action(&value);
    assert!(!result.valid);
    assert!(
        result
            .findings
            .iter()
            .any(|item| item.contains("no included item"))
    );
    assert!(
        result
            .findings
            .iter()
            .any(|item| item.contains("confirmed"))
    );
    assert!(result.findings.iter().any(|item| item.contains("terminal")));
}

#[test]
fn requires_unresolved_decisions_to_expose_a_reason_without_guessing() {
    let mut value = action();
    let decision = value.decision.as_mut().expect("decision");
    decision.state = InferenceDecisionState::Unresolved;
    decision.selected_candidate_id = None;
    decision.reason_id = Some("hathq://reason/missing-deadline/v1".into());
    assert!(validate_inference_action(&value).valid);
    value.decision.as_mut().expect("decision").reason_id = None;
    assert!(!validate_inference_action(&value).valid);
}
