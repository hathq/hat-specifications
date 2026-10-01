use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SituationKind {
    Event,
    Task,
    Observation,
    Question,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SituationStatus {
    Planned,
    Active,
    Waiting,
    Completed,
    Cancelled,
    Unresolved,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SituationTime {
    pub start_at_unix_ms: u64,
    pub end_at_unix_ms: Option<u64>,
    pub time_zone: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SituationCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SituationPlace {
    pub label: String,
    pub coordinates: Option<SituationCoordinates>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SituationSource {
    pub repository_id: String,
    pub digest_sha256: String,
    pub revision: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SituationContribution {
    pub schema: String,
    pub contribution_id: String,
    pub category_term_id: String,
    pub kind: SituationKind,
    pub title: String,
    pub status: SituationStatus,
    pub time: Option<SituationTime>,
    pub place: Option<SituationPlace>,
    pub actors: Vec<String>,
    pub object_label: Option<String>,
    pub detail_handle: String,
    pub source: SituationSource,
}
