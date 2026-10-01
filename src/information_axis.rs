// Added by the HAT Specifications project, 2026.
// Purpose: keep person/social information axes independent from Console navigation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StructuralAxis {
    HumanStructure,
    IdentityLifeEvent,
    BiologicalProfile,
    HealthObservation,
    LocationResidence,
    JurisdictionInstitution,
    SocialRelationship,
    PreferenceObjective,
    WorkTask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChangePolicy {
    SpecificationVersioned,
    CorrectionOnly,
    Interval,
    AppendOnlyObservation,
    RefreshableExternalRule,
    UserEditable,
    RebuildableProjection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InformationSensitivity {
    Public,
    Internal,
    InternalConfidential,
    RestrictedSensitive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InformationCoordinate {
    pub schema: String,
    pub vocabulary_owner_id: String,
    pub catalog_digest_sha256: String,
    pub term_id: String,
    pub structural_axis: StructuralAxis,
    pub change_policy: ChangePolicy,
    pub data_domain_term_id: String,
    pub sensitivity: InformationSensitivity,
}
