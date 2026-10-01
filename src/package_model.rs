use serde::{Deserialize, Serialize};

use crate::{
    CommunicationProtocolReference, HatCommunicationCapability, HatManifest, SemanticCatalog,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatPackage {
    pub schema: String,
    pub package_id: String,
    pub repository_id: String,
    pub version: String,
    pub specification_version: String,
    pub manifest: HatManifest,
    /// The package-owned, digest-bound vocabulary and clause catalog.
    ///
    /// This is the sole semantic authority shipped by a HAT package. Package
    /// consumers must resolve terms through this catalog and its exact
    /// dependency closure; no parallel package vocabulary exists.
    pub catalog: SemanticCatalog,
    #[serde(default)]
    pub information_surfaces: Vec<HatInformationSurface>,
    #[serde(default)]
    pub service_requirements: Vec<HatServiceRequirement>,
    /// Exact protocols this package can speak for the listed operations.
    ///
    /// Protocol definitions stay with their standards or ecosystem owner;
    /// this package carries references only.
    #[serde(default)]
    pub communication_capabilities: Vec<HatCommunicationCapability>,
    #[serde(default)]
    pub setup_templates: Vec<HatSetupTemplate>,
    pub operations: Vec<HatOperation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatSetupTemplate {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub action_label: String,
    pub context_namespace: String,
    pub projection_schema: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatServiceRequirement {
    pub id: String,
    pub operation_ids: Vec<String>,
    /// Optional exact acceptable communication protocols. Empty means that
    /// operation compatibility is sufficient and no transport is implied.
    #[serde(default)]
    pub accepted_protocols: Vec<CommunicationProtocolReference>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatInformationSurface {
    pub id: String,
    pub canonical_type: String,
    pub projection_schema: String,
    pub data_domain_term_id: String,
    pub subject_relation: HatSubjectRelation,
    #[serde(default)]
    pub semantic_icon: Option<VocabularyIcon>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HatSubjectRelation {
    Owned,
    Managed,
    Used,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VocabularyIcon {
    Generic,
    Entity,
    Identity,
    Person,
    People,
    Organization,
    Place,
    Jurisdiction,
    Language,
    Biology,
    Health,
    Time,
    Event,
    State,
    Action,
    Reason,
    Relation,
    Evidence,
    Source,
    Resource,
    Asset,
    Money,
    Accounting,
    Work,
    Repository,
    Software,
    Communication,
    Service,
    Credential,
    Model,
    Hat,
    Layer,
    Namespace,
    Directory,
    File,
    Decision,
    Contract,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatOperation {
    pub id: String,
    pub input_schema: String,
    pub output_schema: String,
    pub context_plan: ContextPlan,
    pub reducer: ReducerSpec,
    pub procedure: HatProcedure,
    pub handler: HatHandler,
    #[serde(default)]
    pub effects: Vec<HatOperationEffect>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatOperationEffect {
    pub kind: HatOperationEffectKind,
    pub surface_id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HatOperationEffectKind {
    Create,
    Link,
    Update,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextPlan {
    pub id: String,
    pub selectors: Vec<ContextSelector>,
    pub unresolved_policy: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextSelector {
    pub namespace: String,
    pub projection_schema: String,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReducerSpec {
    pub id: String,
    pub strategy: String,
    pub event_schema: String,
    pub projection_schema: String,
    pub conflict_policy: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatProcedure {
    pub id: String,
    pub steps: Vec<ProcedureStep>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProcedureStep {
    pub action_id: String,
    pub input_schema: String,
    pub output_schema: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatHandler {
    pub kind: String,
    pub reference: String,
}
