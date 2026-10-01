use hat_specifications::{HatEvidenceRecovery, HatExecutionLocation};
use zixcel_revision::attestation::VerificationMaterial;

#[test]
fn historical_authority_is_explicit_bounded_and_separate_from_worker_identity() {
    let supported = HatEvidenceRecovery::DurableEvidence {
        authority: VerificationMaterial {
            authority: "provider/evidence".into(),
            epoch: 7,
            public_key: [1; 32],
        },
        query_route_ref: "crowsi/route/evidence".into(),
    };
    assert!(supported.valid_declaration());
    assert!(HatEvidenceRecovery::Unrecoverable {}.valid_declaration());
    for field in 0..5 {
        let mut bad = supported.clone();
        let HatEvidenceRecovery::DurableEvidence {
            authority,
            query_route_ref,
        } = &mut bad
        else {
            unreachable!()
        };
        match field {
            0 => authority.epoch = 0,
            1 => authority.public_key = [0; 32],
            2 => authority.authority = "x".repeat(257),
            3 => *query_route_ref = "https://host/execute".into(),
            _ => *query_route_ref = "crowsi/route/\nexecute".into(),
        }
        assert!(!bad.valid_declaration());
    }
    assert!(serde_json::from_str::<HatEvidenceRecovery>(r#"{"kind":"durable-evidence"}"#).is_err());
    assert!(
        serde_json::from_str::<HatEvidenceRecovery>(r#"{"kind":"unrecoverable","retry":true}"#)
            .is_err()
    );
    // Existing placement JSON must be deliberately regenerated, never defaulted
    // to support recovery or supplemented from whatever worker is current.
    assert!(serde_json::from_str::<HatExecutionLocation>("{}").is_err());
}
