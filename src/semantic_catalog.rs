// Added by the HAT Specifications project, 2026.
// Purpose: define exact catalog, term, lexicalization and clause-frame identities.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SemanticTermKind {
    Concept,
    EntityType,
    Relation,
    Predicate,
    Property,
    EventType,
    StateType,
    Action,
    Reason,
    Unit,
    ProtocolField,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticCatalogReference {
    pub catalog_id: String,
    pub version: u64,
    pub digest_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTermReference {
    pub catalog_id: String,
    pub catalog_digest_sha256: String,
    pub term_id: String,
    pub version: u64,
    pub definition_digest_sha256: String,
    pub kind: SemanticTermKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticLexicalization {
    pub term: SemanticTermReference,
    pub locale: String,
    pub preferred: String,
    pub search_aliases: BTreeSet<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SemanticClauseRole {
    Actor,
    Subject,
    Object,
    Value,
    Source,
    Destination,
    Instrument,
    Time,
    Place,
    Evidence,
    Output,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticRoleSpec {
    pub role: SemanticClauseRole,
    pub required: bool,
    pub accepted_kinds: BTreeSet<SemanticTermKind>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticClauseFrame {
    pub frame_id: String,
    pub definition_digest_sha256: String,
    pub predicate: SemanticTermReference,
    pub roles: Vec<SemanticRoleSpec>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTermDefinition {
    pub reference: SemanticTermReference,
    /// Exact wire representation used when this term crosses a service boundary.
    /// Foundation concepts may omit it; package-owned operational terms must declare it.
    #[serde(default)]
    pub wire_schema: Option<String>,
    pub semantic_icon: crate::VocabularyIcon,
    pub dependencies: BTreeSet<SemanticTermReference>,
    pub is_a: BTreeSet<SemanticTermReference>,
    pub domain: BTreeSet<SemanticTermReference>,
    pub range: BTreeSet<SemanticTermReference>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticCatalog {
    pub schema: String,
    pub identity: SemanticCatalogReference,
    pub owner_repository_id: String,
    pub foundation: bool,
    pub dependencies: Vec<SemanticCatalogReference>,
    /// Exact catalogs that cannot participate in the same accepted composition.
    pub incompatibilities: Vec<SemanticCatalogReference>,
    pub terms: Vec<SemanticTermDefinition>,
    pub lexicalizations: Vec<SemanticLexicalization>,
    pub frames: Vec<SemanticClauseFrame>,
}
