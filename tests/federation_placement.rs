use hat_specifications::{
    ActionReference, FEDERATION_RECEIPT_SCHEMA, HatActionResult, HatFederationExecutionReceipt,
    HatInvocationOutcome, HatPlacementSelection, PLACEMENT_SELECTION_SCHEMA,
    federation_execution_receipt_digest, placement_selection_digest,
    validate_federation_execution_receipt, validate_placement_selection,
};

const A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

fn selection() -> HatPlacementSelection {
    HatPlacementSelection {
        schema: PLACEMENT_SELECTION_SCHEMA.into(),
        selection_id: "placement-1".into(),
        context_partition_id: "context-partition-1".into(),
        package_id: "hat/source-curator".into(),
        package_digest_sha256: A.into(),
        location_id: "tokyo-owner-node".into(),
        location_digest_sha256: B.into(),
        route_policy_digest_sha256: A.into(),
        permitted_classifications: vec!["internal".into()],
        approved_failover_location_digests: Vec::new(),
        revision: 1,
        selected_at_epoch_s: 1_100,
    }
}

fn result() -> HatActionResult {
    HatActionResult {
        schema: hat_specifications::ACTION_RESULT_SCHEMA.into(),
        invocation_id: "invoke-1".into(),
        operation_id: "hathq://hat/action/review-source/v1".into(),
        state_revision: 3,
        projection_revision: 2,
        outcome: HatInvocationOutcome::Completed,
        output: Some(ActionReference {
            owner_id: "source-curator-tokyo".into(),
            reference: "result/1".into(),
            schema_id: "hathq://hat/result/source-review/v1".into(),
            digest_sha256: A.into(),
        }),
        reason_id: None,
        evidence_refs: Vec::new(),
    }
}

#[test]
fn placement_has_no_implicit_failover() {
    let value = selection();
    assert!(validate_placement_selection(&value).valid);
    let mut duplicate = value;
    duplicate.approved_failover_location_digests = vec![B.into(), B.into()];
    assert!(!validate_placement_selection(&duplicate).valid);
}

#[test]
fn receipt_correlates_exact_invocation_placement_worker_and_result() {
    let result = result();
    let receipt = HatFederationExecutionReceipt {
        schema: FEDERATION_RECEIPT_SCHEMA.into(),
        receipt_id: "receipt-1".into(),
        invocation_id: result.invocation_id.clone(),
        invocation_digest_sha256: A.into(),
        placement_selection_digest_sha256: B.into(),
        location_digest_sha256: B.into(),
        worker_service_id: "source-curator-tokyo".into(),
        worker_identity_ref: "ihat/service/source-curator-tokyo".into(),
        transport_receipt_digest_sha256: A.into(),
        result_digest_sha256: B.into(),
        completed_at_epoch_s: 1_200,
    };
    assert!(validate_federation_execution_receipt(&receipt, &result).valid);
    let mut substituted = receipt;
    substituted.invocation_id = "invoke-2".into();
    assert!(!validate_federation_execution_receipt(&substituted, &result).valid);
}

#[test]
fn placement_wire_rejects_a_raw_endpoint() {
    let mut value = serde_json::to_value(selection()).expect("selection JSON");
    value["url"] = serde_json::json!("https://worker.example");
    assert!(serde_json::from_value::<HatPlacementSelection>(value).is_err());
}

#[test]
fn federation_digests_are_domain_separated_and_mutation_sensitive() {
    let value = selection();
    let digest = placement_selection_digest(&value).expect("placement digest");
    assert_eq!(digest.len(), 64);
    let mut changed = value;
    changed.revision += 1;
    assert_ne!(
        digest,
        placement_selection_digest(&changed).expect("changed")
    );

    let result = result();
    let receipt = HatFederationExecutionReceipt {
        schema: FEDERATION_RECEIPT_SCHEMA.into(),
        receipt_id: "receipt-1".into(),
        invocation_id: result.invocation_id.clone(),
        invocation_digest_sha256: A.into(),
        placement_selection_digest_sha256: B.into(),
        location_digest_sha256: B.into(),
        worker_service_id: "source-curator-tokyo".into(),
        worker_identity_ref: "ihat/service/source-curator-tokyo".into(),
        transport_receipt_digest_sha256: A.into(),
        result_digest_sha256: B.into(),
        completed_at_epoch_s: 1_200,
    };
    assert_ne!(
        federation_execution_receipt_digest(&receipt).expect("receipt digest"),
        digest
    );
}
