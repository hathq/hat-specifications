use hat_specifications::{
    ActionReference, BINDING_SCHEMA, CHECKPOINT_SCHEMA, COMMUNICATION_CAPABILITY_SCHEMA,
    CommunicationDirection, CommunicationProtocolReference, CommunicationRole, ContextPartition,
    EventSemantics, HatBinding, HatCheckpoint, HatCommunicationCapability, HatInvocation,
    HatPackage, HatProjection, HatProjectionEvent, INVOCATION_SCHEMA, PROJECTION_EVENT_SCHEMA,
    PROJECTION_SCHEMA, SemanticTimeInterval, communication_semantic_catalog, parse_package,
    semantic_catalog_digest, validate_binding, validate_checkpoint, validate_invocation,
    validate_package, validate_projection, validate_projection_event,
};

const DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn package() -> HatPackage {
    let raw = include_str!("../examples/source-curator.package.json");
    parse_package(raw).expect("package")
}

fn binding() -> HatBinding {
    HatBinding {
        schema: BINDING_SCHEMA.to_owned(),
        package_id: "hat/source-curator".to_owned(),
        package_digest_sha256: DIGEST.to_owned(),
        catalog_digest_sha256: DIGEST.to_owned(),
        fitting_digest_sha256: DIGEST.to_owned(),
        subject_ref: "subject:test".to_owned(),
        scope_ref: "scope:personal".to_owned(),
    }
}

#[test]
fn package_carries_structuring_logic_without_runtime_authority() {
    let package = package();
    assert!(validate_package(&package).valid);
    assert_eq!(package.repository_id, "hat-source-curator");
    assert_eq!(
        package.operations[0].context_plan.unresolved_policy,
        "record-unresolved"
    );
    assert_eq!(package.operations[0].reducer.conflict_policy, "reject");
    assert_eq!(package.operations[0].handler.kind, "declarative-hat");
    assert_eq!(
        serde_json::to_value(package.catalog.terms[0].semantic_icon).expect("semantic icon"),
        serde_json::json!("source")
    );
}

#[test]
fn protocol_speaking_is_generic_exact_and_operation_bound() {
    let mut package = package();
    let operation_id = package.operations[0].id.clone();
    package.communication_capabilities = vec![HatCommunicationCapability {
        schema: COMMUNICATION_CAPABILITY_SCHEMA.into(),
        id: "http-source-reader".into(),
        protocol: CommunicationProtocolReference {
            owner_id: "ietf".into(),
            protocol_id: "urn:ietf:rfc:9112".into(),
            version: 1,
            specification_ref: "https://www.rfc-editor.org/rfc/rfc9112".into(),
        },
        role: CommunicationRole::Client,
        direction: CommunicationDirection::Bidirectional,
        operation_ids: vec![operation_id],
    }];
    package
        .catalog
        .dependencies
        .push(communication_semantic_catalog().identity);
    refresh_catalog_digest(&mut package);
    assert!(validate_package(&package).valid);

    let communication = communication_semantic_catalog();
    let ids = communication
        .terms
        .iter()
        .map(|term| term.reference.term_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert!(ids.contains("core.communication-protocol"));
    assert!(ids.contains("core.communication-capability"));
    assert!(ids.contains("core.relation.speaks"));
    assert!(
        !ids.iter()
            .any(|id| id.contains("http") || id.contains("zixcel"))
    );

    package.communication_capabilities[0].operation_ids =
        vec!["hathq://vocabulary/action/not-owned/v1".into()];
    assert!(!validate_package(&package).valid);
}

fn refresh_catalog_digest(package: &mut HatPackage) {
    let digest = semantic_catalog_digest(&package.catalog).expect("catalog digest");
    package.catalog.identity.digest_sha256.clone_from(&digest);
    let catalog_id = package.catalog.identity.catalog_id.clone();
    for definition in &mut package.catalog.terms {
        if definition.reference.catalog_id == catalog_id {
            definition
                .reference
                .catalog_digest_sha256
                .clone_from(&digest);
        }
        for references in [
            &mut definition.dependencies,
            &mut definition.is_a,
            &mut definition.domain,
            &mut definition.range,
        ] {
            *references = std::mem::take(references)
                .into_iter()
                .map(|mut reference| {
                    if reference.catalog_id == catalog_id {
                        reference.catalog_digest_sha256.clone_from(&digest);
                    }
                    reference
                })
                .collect();
        }
    }
    for lexicalization in &mut package.catalog.lexicalizations {
        if lexicalization.term.catalog_id == catalog_id {
            lexicalization
                .term
                .catalog_digest_sha256
                .clone_from(&digest);
        }
    }
    for frame in &mut package.catalog.frames {
        if frame.predicate.catalog_id == catalog_id {
            frame.predicate.catalog_digest_sha256.clone_from(&digest);
        }
    }
}

#[test]
fn unknown_fields_credentials_and_inferred_policy_fail_closed() {
    let mut raw = serde_json::to_value(package()).expect("json");
    raw.as_object_mut()
        .expect("object")
        .insert("token".to_owned(), serde_json::json!("x"));
    assert!(parse_package(&serde_json::to_string(&raw).expect("json")).is_err());
    let mut package = package();
    package.operations[0].context_plan.unresolved_policy = "guess".to_owned();
    package.operations[0].reducer.conflict_policy = "last-write-wins".to_owned();
    assert!(!validate_package(&package).valid);
}

#[test]
fn catalog_accepts_only_closed_semantic_icon_tokens() {
    let source = serde_json::to_string_pretty(&package()).expect("json");
    let invalid = source.replace(
        "\"semantic_icon\": \"source\"",
        "\"semantic_icon\": \"i-lucide-database\"",
    );
    assert!(parse_package(&invalid).is_err());

    let without_icon = source.replace("\"semantic_icon\": \"source\",", "");
    assert!(
        parse_package(&without_icon).is_err(),
        "catalog icons are required"
    );
}

#[test]
fn context_partition_runtime_contracts_are_revisioned_and_digest_bound() {
    let binding = binding();
    assert!(validate_binding(&binding).valid);
    let mut unscoped = binding.clone();
    unscoped.scope_ref.clear();
    assert!(!validate_binding(&unscoped).valid);
    let invocation = HatInvocation {
        schema: INVOCATION_SCHEMA.to_owned(),
        invocation_id: "invoke-1".to_owned(),
        binding,
        context_partition: ContextPartition {
            context_partition_id: "context-partition-1".to_owned(),
            revision: 7,
            policy_digest_sha256: DIGEST.to_owned(),
        },
        operation_id: "hathq://vocabulary/action/review-source/v1".to_owned(),
        expected_projection_revision: 4,
        idempotency_key: "request-1".to_owned(),
        input: action_reference("hathq://hat/source-review-input/v1"),
        effective_grant: action_reference("hathq://hat/effective-grant/v1"),
        placement: action_reference("hathq://hat/placement/v1"),
    };
    assert!(validate_invocation(&invocation).valid);

    let event = HatProjectionEvent {
        schema: PROJECTION_EVENT_SCHEMA.to_owned(),
        event_id: "event-1".to_owned(),
        semantics: event_semantics(&invocation.invocation_id),
        invocation_id: invocation.invocation_id,
        context_partition_id: invocation.context_partition.context_partition_id.clone(),
        operation_id: invocation.operation_id,
        previous_revision: 4,
        next_revision: 5,
        evidence_refs: Vec::new(),
    };
    assert!(validate_projection_event(&event).valid);

    let projection = HatProjection {
        schema: PROJECTION_SCHEMA.to_owned(),
        context_partition_id: invocation.context_partition.context_partition_id.clone(),
        package_id: "hat/source-curator".to_owned(),
        revision: 5,
        timeline_watermark: 5,
        projection_digest_sha256: DIGEST.to_owned(),
        evidence_refs: Vec::new(),
        unresolved_terms: Vec::new(),
    };
    assert!(validate_projection(&projection).valid);

    let checkpoint = HatCheckpoint {
        schema: CHECKPOINT_SCHEMA.to_owned(),
        context_partition_id: invocation.context_partition.context_partition_id,
        package_id: projection.package_id,
        from_watermark: 0,
        to_watermark: 5,
        previous_projection_digest_sha256: DIGEST.to_owned(),
        new_projection_digest_sha256: projection.projection_digest_sha256,
        reducer_version: "1.0.0".to_owned(),
    };
    assert!(validate_checkpoint(&checkpoint).valid);
}

fn action_reference(schema_id: &str) -> ActionReference {
    ActionReference {
        owner_id: "zixcel".to_owned(),
        reference: "artifact-1".to_owned(),
        schema_id: schema_id.to_owned(),
        digest_sha256: DIGEST.to_owned(),
    }
}

#[test]
fn revision_skips_and_invalid_digests_are_rejected() {
    let mut binding = binding();
    binding.package_digest_sha256 = "short".to_owned();
    assert!(!validate_binding(&binding).valid);
    let event = HatProjectionEvent {
        schema: PROJECTION_EVENT_SCHEMA.to_owned(),
        event_id: "event-1".to_owned(),
        semantics: event_semantics("invoke-1"),
        invocation_id: "invoke-1".to_owned(),
        context_partition_id: "context-partition-1".to_owned(),
        operation_id: "hathq://vocabulary/action/review-source/v1".to_owned(),
        previous_revision: 4,
        next_revision: 6,
        evidence_refs: Vec::new(),
    };
    assert!(!validate_projection_event(&event).valid);
}

fn event_semantics(invocation_id: &str) -> EventSemantics {
    EventSemantics {
        event_type: "core.event.action.completed".to_owned(),
        occurred_time: SemanticTimeInterval {
            start_epoch_ms: 1_700_000_000_000,
            end_epoch_ms: None,
        },
        observed_at_epoch_ms: 1_700_000_000_000,
        correlation_id: Some(invocation_id.to_owned()),
        causation_id: None,
    }
}
