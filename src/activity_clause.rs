// Added by the HAT Specifications project, 2026.
// Purpose: describe application activity presentation; language outcomes belong to sem-lang.

use crate::SemanticTermReference;
use crate::{SemanticClauseFrame, SemanticClauseRole, SemanticScalar, SemanticStableRef};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SemanticClauseKind {
    Fact,
    Action,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SemanticModality {
    Asserted,
    Observed,
    Proposed,
    Required,
    Executed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticRoleBinding {
    pub role: SemanticClauseRole,
    pub term: Option<SemanticTermReference>,
    pub reference: Option<SemanticStableRef>,
    pub value: Option<SemanticScalar>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticClause {
    pub schema: String,
    pub clause_id: String,
    pub kind: SemanticClauseKind,
    pub frame: SemanticClauseFrame,
    pub bindings: Vec<SemanticRoleBinding>,
    pub modality: SemanticModality,
    pub positive: bool,
}
