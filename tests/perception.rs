// Hatter downstream 2026: input connections are not semantic truth or execution grants.
use hat_specifications::{
    InterpretationRoute, PerceptionBinding, PerceptionObservation, PerceptionState,
    SemanticStableRef, plan_perception,
};

fn reference(id: &str) -> SemanticStableRef {
    SemanticStableRef {
        schema: "hat://hathq/reference/v1".into(),
        id: id.into(),
        revision: 1,
        digest_sha256: "a".repeat(64),
    }
}

fn fixture() -> (PerceptionBinding, PerceptionObservation) {
    let route = |id: &str| InterpretationRoute {
        hat_binding_ref: reference(&format!("hat/{id}")),
        runtime_role_ref: reference(&format!("role/{id}")),
        semantic_binding_ref: reference(&format!("meaning/{id}")),
    };
    let binding = PerceptionBinding {
        binding_id: "personal/mail".into(),
        revision: 1,
        role_ref: reference("person/one"),
        sense_ref: reference("sense/reading"),
        provider_ref: reference("hat/mail"),
        account_ref: reference("account/personal"),
        source_scope_ref: reference("mailbox/inbox"),
        input_contract_ref: reference("schema/mail"),
        state: PerceptionState::Enabled,
        interpretations: vec![route("accounting"), route("planning")],
    };
    let observation = PerceptionObservation {
        source_ref: reference("mail/uidvalidity-7/uid-10"),
        provider_ref: binding.provider_ref.clone(),
        account_ref: binding.account_ref.clone(),
        source_scope_ref: binding.source_scope_ref.clone(),
        input_contract_ref: binding.input_contract_ref.clone(),
        received_at_epoch_ms: 1000,
        occurrence_ref: None,
        location_ref: None,
    };
    (binding, observation)
}

#[test]
fn mail_fans_out_to_exact_installed_interpretations_without_authorizing_effects() {
    let (binding, observation) = fixture();
    let plan = plan_perception(&binding, &observation).unwrap();
    assert_eq!(plan.source_ref, observation.source_ref);
    assert_eq!(plan.role_ref, binding.role_ref);
    assert_eq!(plan.routes.len(), 2);
    assert_eq!(
        plan.routes[0].semantic_binding_ref,
        binding.interpretations[0].semantic_binding_ref
    );
    assert!(plan.location_ref.is_none());
    assert!(plan.occurrence_ref.is_none());
    let json = serde_json::to_value(&plan).unwrap();
    for absent in [
        "body",
        "classification",
        "task",
        "grant",
        "execute",
        "credentials",
    ] {
        assert!(json.get(absent).is_none());
    }
    assert_eq!(plan, plan_perception(&binding, &observation).unwrap());
    let mut reordered = binding.clone();
    reordered.interpretations.reverse();
    assert_eq!(plan, plan_perception(&reordered, &observation).unwrap());
    let mut new_meaning = binding.clone();
    new_meaning.interpretations[0].semantic_binding_ref.revision += 1;
    assert_ne!(
        plan.request_key,
        plan_perception(&new_meaning, &observation)
            .unwrap()
            .request_key
    );
    let mut other_message = observation.clone();
    other_message.source_ref.id.push_str("-next");
    assert_ne!(
        plan.request_key,
        plan_perception(&binding, &other_message)
            .unwrap()
            .request_key
    );
}

#[test]
fn rejects_cross_account_scope_revision_disabled_binding_and_unbounded_fanout() {
    let (binding, observation) = fixture();
    for field in [
        "provider_ref",
        "account_ref",
        "source_scope_ref",
        "input_contract_ref",
    ] {
        let mut json = serde_json::to_value(&observation).unwrap();
        json[field]["revision"] = 2.into();
        let stale: PerceptionObservation = serde_json::from_value(json).unwrap();
        assert!(plan_perception(&binding, &stale).is_err(), "{field}");
    }
    let mut disabled = binding.clone();
    disabled.state = PerceptionState::Disabled;
    assert!(plan_perception(&disabled, &observation).is_err());
    let mut duplicate = binding.clone();
    duplicate
        .interpretations
        .push(binding.interpretations[0].clone());
    assert!(plan_perception(&duplicate, &observation).is_err());
    let mut excessive = binding.clone();
    excessive.interpretations = vec![binding.interpretations[0].clone(); 33];
    assert!(plan_perception(&excessive, &observation).is_err());
    let mut empty = binding.clone();
    empty.interpretations.clear();
    assert!(plan_perception(&empty, &observation).is_err());
    let mut invalid = observation.clone();
    invalid.source_ref.revision = 0;
    assert!(plan_perception(&binding, &invalid).is_err());
    let mut payload = serde_json::to_value(observation).unwrap();
    payload["body"] = "untrusted content".into();
    assert!(serde_json::from_value::<PerceptionObservation>(payload).is_err());
}

#[test]
fn physical_location_and_occurrence_remain_separate_exact_source_references() {
    let (binding, mut observation) = fixture();
    observation.location_ref = Some(reference("place/observed-point"));
    observation.occurrence_ref = Some(reference("time/source-claimed-date"));
    let plan = plan_perception(&binding, &observation).unwrap();
    assert_eq!(plan.location_ref, observation.location_ref);
    assert_eq!(plan.occurrence_ref, observation.occurrence_ref);
    assert_eq!(plan.received_at_epoch_ms, 1000);
    let mut missing = observation;
    missing.location_ref = None;
    assert_ne!(
        plan.request_key,
        plan_perception(&binding, &missing).unwrap().request_key
    );
}
