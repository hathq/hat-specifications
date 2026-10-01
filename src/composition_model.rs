use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatCompositionMember {
    pub repository_id: String,
    pub package_id: String,
    pub package_digest_sha256: String,
    pub catalog_digest_sha256: String,
    pub fitting_digest_sha256: String,
    pub policy_digest_sha256: String,
    pub expected_binding_revision: Option<u64>,
    pub dependency_repository_ids: Vec<String>,
    pub incompatible_repository_ids: Vec<String>,
    pub operation_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatCompositionProposal {
    pub schema: String,
    pub proposal_id: String,
    pub subject_ref: String,
    pub scope_ref: String,
    pub expected_composition_revision: Option<u64>,
    pub members: Vec<HatCompositionMember>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatCompositionApproval {
    pub schema: String,
    pub approval_id: String,
    pub proposal_digest_sha256: String,
    pub approved_by_subject_ref: String,
    pub expected_composition_revision: Option<u64>,
}
