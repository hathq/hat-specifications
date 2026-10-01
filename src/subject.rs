// Added by the HAT Specifications project, 2026.
// Purpose: define closed natural-person and legal-person subject references.

use serde::{Deserialize, Serialize};

/// The independently identified kind of a subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SubjectKind {
    NaturalPerson,
    LegalPerson,
}

/// A reference to an identity held by an external identity authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubjectReference {
    pub schema: String,
    pub subject_id: String,
    pub kind: SubjectKind,
    pub identity_authority_id: String,
    pub identity_reference: String,
    pub identity_revision: u64,
}
