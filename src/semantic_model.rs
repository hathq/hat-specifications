use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticStableRef {
    pub schema: String,
    pub id: String,
    pub revision: u64,
    pub digest_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum SemanticScalar {
    Null,
    Bool(bool),
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    Reference(SemanticStableRef),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTimeInterval {
    pub start_epoch_ms: i64,
    pub end_epoch_ms: Option<i64>,
}

/// Vocabulary-bound meaning shared by durable runtime journals and digital-twin events.
///
/// The envelope intentionally contains no runtime state or domain payload. Producers can
/// therefore correlate an execution occurrence with a semantic occurrence without copying
/// either store into the other.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventSemantics {
    pub event_type: String,
    pub occurred_time: SemanticTimeInterval,
    pub observed_at_epoch_ms: i64,
    pub correlation_id: Option<String>,
    pub causation_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticEntity {
    pub identity: SemanticStableRef,
    pub types: BTreeSet<String>,
    pub properties: BTreeMap<String, SemanticScalar>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticRelation {
    pub semantic_id: String,
    pub relation_type: String,
    pub from: SemanticStableRef,
    pub to: SemanticStableRef,
    pub directed: bool,
    pub properties: BTreeMap<String, SemanticScalar>,
    pub revision: u64,
    pub content_digest_sha256: String,
}

/// A bounded occurrence in the digital twin. Events reference entities; they do not own them.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticEvent {
    pub semantic_id: String,
    #[serde(flatten)]
    pub semantics: EventSemantics,
    pub producer: SemanticStableRef,
    pub source: SemanticStableRef,
    pub actors: Vec<SemanticStableRef>,
    pub objects: Vec<SemanticStableRef>,
    pub evidence: Vec<SemanticStableRef>,
    pub properties: BTreeMap<String, SemanticScalar>,
    pub revision: u64,
    pub content_digest_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticAssertion {
    pub semantic_id: String,
    pub subject: SemanticStableRef,
    pub predicate: String,
    pub value: SemanticScalar,
    pub valid_time: Option<SemanticTimeInterval>,
    pub observed_at_epoch_ms: i64,
    pub resolved_at_epoch_ms: i64,
    pub producer: SemanticStableRef,
    pub source: SemanticStableRef,
    pub evidence: Vec<SemanticStableRef>,
    pub revision: u64,
    pub content_digest_sha256: String,
    pub policy_classification: String,
    pub interpretation: Option<SemanticInterpretation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticInterpretation {
    pub policy_ref: SemanticStableRef,
    pub jurisdiction_term: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticEvidence {
    pub identity: SemanticStableRef,
    pub source: SemanticStableRef,
    pub observed_at_epoch_ms: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticSource {
    pub identity: SemanticStableRef,
    pub source_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticBinding {
    pub binding_id: String,
    pub subject_ref: SemanticStableRef,
    pub scope_ref: SemanticStableRef,
    pub hat_package_ref: SemanticStableRef,
    pub fitting_ref: SemanticStableRef,
    pub policy_ref: SemanticStableRef,
    pub provider_ref: SemanticStableRef,
    pub revision: u64,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatSemanticContribution {
    pub schema: String,
    pub producer_hat: SemanticStableRef,
    pub canonical_source: SemanticStableRef,
    pub source_revision: u64,
    pub content_digest_sha256: String,
    pub catalogs: Vec<crate::SemanticCatalogReference>,
    pub sources: Vec<SemanticSource>,
    pub entities: Vec<SemanticEntity>,
    pub relations: Vec<SemanticRelation>,
    pub events: Vec<SemanticEvent>,
    pub assertions: Vec<SemanticAssertion>,
    pub evidence: Vec<SemanticEvidence>,
    pub bindings: Vec<SemanticBinding>,
}
