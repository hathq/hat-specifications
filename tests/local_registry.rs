use hat_specifications::{
    LOCAL_REGISTRY_LEDGER_SCHEMA, LocalRegistryArtifact, LocalRegistryArtifactKind,
    LocalRegistryEntry, LocalRegistryInterfaceKind, LocalRegistryLedger, LocalRegistryPublication,
    LocalRegistryPurpose, LocalRegistrySource, LocalRegistrySourceKind, LocalRegistryVerification,
    LocalRegistryVerificationState, SEMANTIC_MAPPING_CONTRACT_SCHEMA, SemanticFieldMapping,
    SemanticMappingContract, SemanticMappingUnresolvedPolicy, SemanticTermKind,
    foundation_semantic_catalog, validate_local_registry_ledger,
    validate_semantic_mapping_contract,
};

const DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

#[test]
fn local_registry_ledger_distinguishes_development_from_hat_acquisition_without_changing_shape() {
    let mut ledger = ledger(LocalRegistryPurpose::Development);
    assert!(validate_local_registry_ledger(&ledger).valid);
    ledger.purpose = LocalRegistryPurpose::HatAcquisition;
    assert!(validate_local_registry_ledger(&ledger).valid);
    ledger.entries[0].verification.state = LocalRegistryVerificationState::Rejected;
    ledger.entries[0].verification.verified_at_epoch_s = None;
    assert!(!validate_local_registry_ledger(&ledger).valid);
    ledger.entries[0].verification.reason_id =
        Some("hathq://vocabulary/reason/artifact-untrusted/v1".to_owned());
    assert!(validate_local_registry_ledger(&ledger).valid);
}

#[test]
fn local_registry_ledger_rejects_secret_bearing_or_unproven_references() {
    let mut value = ledger(LocalRegistryPurpose::Development);
    value.entries[0].source.reference = "https://example.invalid?token=secret".to_owned();
    assert!(!validate_local_registry_ledger(&value).valid);
    value = ledger(LocalRegistryPurpose::Development);
    value.entries[0].artifact.digest_sha256 = "abc".to_owned();
    assert!(!validate_local_registry_ledger(&value).valid);
}

#[test]
fn semantic_mapping_contract_keeps_external_schema_mapping_out_of_provider_code() {
    let foundation = foundation_semantic_catalog();
    let target = foundation.terms[0].reference.clone();
    let contract = SemanticMappingContract {
        schema: SEMANTIC_MAPPING_CONTRACT_SCHEMA.to_owned(),
        contract_id: "github-repository-to-source".to_owned(),
        revision: 1,
        owner_repository_id: "hat-github-operator".to_owned(),
        external_schema: "hathq://zixcel/github/repository/v1".to_owned(),
        target_catalogs: vec![foundation.identity],
        field_mappings: vec![SemanticFieldMapping {
            mapping_id: "full-name".to_owned(),
            source_pointer: "$.full_name".to_owned(),
            target_term: target,
            target_field: "display-name".to_owned(),
            required: true,
        }],
        unresolved_policy: SemanticMappingUnresolvedPolicy::RecordUnresolved,
    };
    assert!(validate_semantic_mapping_contract(&contract).valid);
    let mut invalid = contract;
    invalid.field_mappings[0].target_term.kind = SemanticTermKind::ProtocolField;
    invalid.field_mappings[0].source_pointer = "$..private".to_owned();
    assert!(!validate_semantic_mapping_contract(&invalid).valid);
}

fn ledger(purpose: LocalRegistryPurpose) -> LocalRegistryLedger {
    LocalRegistryLedger {
        schema: LOCAL_REGISTRY_LEDGER_SCHEMA.to_owned(),
        ledger_id: "owner-local-registry".to_owned(),
        revision: 1,
        purpose,
        owner_ref: "hatter-owner://local".to_owned(),
        entries: vec![LocalRegistryEntry {
            entry_id: "artifact-one".to_owned(),
            source: LocalRegistrySource {
                kind: LocalRegistrySourceKind::GithubActionsArtifact,
                reference: "github://owner/repo/actions/runs/1/artifacts/2".to_owned(),
                source_schema: Some("hathq://zixcel/github/actions-artifact/v1".to_owned()),
                source_digest_sha256: Some(DIGEST.to_owned()),
            },
            artifact: LocalRegistryArtifact {
                kind: LocalRegistryArtifactKind::HatPackage,
                artifact_id: "hat-package-one".to_owned(),
                digest_sha256: DIGEST.to_owned(),
                size_bytes: 1024,
                media_type: Some("application/json".to_owned()),
                local_ref: "local-cas://sha256/aaaaaaaa".to_owned(),
            },
            verification: LocalRegistryVerification {
                state: LocalRegistryVerificationState::Verified,
                verifier_ref: "hatter://local-registry/verifier".to_owned(),
                verified_at_epoch_s: Some(1),
                reason_id: None,
                evidence_refs: Vec::new(),
            },
            published_as: vec![LocalRegistryPublication {
                interface: LocalRegistryInterfaceKind::HatCatalog,
                reference: "hatter-catalog://local/catalog/v2/index.json".to_owned(),
                digest_sha256: DIGEST.to_owned(),
                published_at_epoch_s: 2,
            }],
            recorded_at_epoch_s: 1,
        }],
    }
}
