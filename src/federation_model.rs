use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HatExecutionKind {
    OwnerLocal,
    OwnerDevice,
    FederatedManaged,
    ExternalProvider,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatExecutionLocation {
    pub schema: String,
    pub location_id: String,
    pub package_id: String,
    pub package_digest_sha256: String,
    pub publisher_id: String,
    pub execution_kind: HatExecutionKind,
    pub worker_service_id: String,
    pub identity_authority_ref: String,
    /// Signed placement declaration. Never inferred from the current worker.
    pub evidence_recovery: crate::HatEvidenceRecovery,
    pub transport_profile_ref: String,
    pub route_ref: String,
    pub region: Option<String>,
    pub jurisdictions: Vec<String>,
    pub data_residencies: Vec<String>,
    pub operation_ids: Vec<String>,
    pub accepted_classifications: Vec<String>,
    pub capability_ids: Vec<String>,
    pub assurance: String,
    pub issued_at_epoch_s: u64,
    pub expires_at_epoch_s: u64,
    pub revocation_epoch: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatExecutionLocationSet {
    pub schema: String,
    pub package_id: String,
    pub package_digest_sha256: String,
    pub revision: u64,
    pub issued_at_epoch_s: u64,
    pub expires_at_epoch_s: u64,
    pub locations: Vec<HatExecutionLocation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatPlacementSelection {
    pub schema: String,
    pub selection_id: String,
    pub context_partition_id: String,
    pub package_id: String,
    pub package_digest_sha256: String,
    pub location_id: String,
    pub location_digest_sha256: String,
    pub route_policy_digest_sha256: String,
    pub permitted_classifications: Vec<String>,
    pub approved_failover_location_digests: Vec<String>,
    pub revision: u64,
    pub selected_at_epoch_s: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatFederationExecutionReceipt {
    pub schema: String,
    pub receipt_id: String,
    pub invocation_id: String,
    pub invocation_digest_sha256: String,
    pub placement_selection_digest_sha256: String,
    pub location_digest_sha256: String,
    pub worker_service_id: String,
    pub worker_identity_ref: String,
    pub transport_receipt_digest_sha256: String,
    pub result_digest_sha256: String,
    pub completed_at_epoch_s: u64,
}
