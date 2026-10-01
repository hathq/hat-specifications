// Added by the HAT Specifications project, 2026.
// Purpose: define the shared local registry ledger without choosing Cargo, npm, OCI or HAT catalog as the root model.

use crate::{EvidenceReference, SemanticTermReference};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalRegistryPurpose {
    Development,
    HatAcquisition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalRegistryArtifactKind {
    CargoCrate,
    HatPackage,
    HatWorker,
    LocalModelPackage,
    UiModule,
    SourceSnapshot,
    ProviderArtifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalRegistrySourceKind {
    LocalDirectory,
    LocalRegistry,
    Https,
    GithubRepository,
    GithubActionsArtifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalRegistryInterfaceKind {
    HatterLedger,
    HatCatalog,
    CargoSparseRegistry,
    NpmRegistry,
    OciRegistry,
    FilesystemCas,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalRegistryVerificationState {
    Verified,
    Unverified,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalRegistryLedger {
    pub schema: String,
    pub ledger_id: String,
    pub revision: u64,
    pub purpose: LocalRegistryPurpose,
    pub owner_ref: String,
    pub entries: Vec<LocalRegistryEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalRegistryEntry {
    pub entry_id: String,
    pub source: LocalRegistrySource,
    pub artifact: LocalRegistryArtifact,
    pub verification: LocalRegistryVerification,
    pub published_as: Vec<LocalRegistryPublication>,
    pub recorded_at_epoch_s: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalRegistrySource {
    pub kind: LocalRegistrySourceKind,
    pub reference: String,
    pub source_schema: Option<String>,
    pub source_digest_sha256: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalRegistryArtifact {
    pub kind: LocalRegistryArtifactKind,
    pub artifact_id: String,
    pub digest_sha256: String,
    pub size_bytes: u64,
    pub media_type: Option<String>,
    pub local_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalRegistryVerification {
    pub state: LocalRegistryVerificationState,
    pub verifier_ref: String,
    pub verified_at_epoch_s: Option<u64>,
    pub reason_id: Option<String>,
    pub evidence_refs: Vec<EvidenceReference>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalRegistryPublication {
    pub interface: LocalRegistryInterfaceKind,
    pub reference: String,
    pub digest_sha256: String,
    pub published_at_epoch_s: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticMappingContract {
    pub schema: String,
    pub contract_id: String,
    pub revision: u64,
    pub owner_repository_id: String,
    pub external_schema: String,
    pub target_catalogs: Vec<crate::SemanticCatalogReference>,
    pub field_mappings: Vec<SemanticFieldMapping>,
    pub unresolved_policy: SemanticMappingUnresolvedPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticFieldMapping {
    pub mapping_id: String,
    pub source_pointer: String,
    pub target_term: SemanticTermReference,
    pub target_field: String,
    pub required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SemanticMappingUnresolvedPolicy {
    RecordUnresolved,
    Reject,
}
