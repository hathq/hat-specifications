// Added by the HAT Specifications project, 2026.
// Purpose: describe how a HAT can speak an externally owned protocol without copying it into Hatter vocabulary.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use crate::tokens::{schema_uri, stable_token};
use crate::{
    COMMUNICATION_CAPABILITY_SCHEMA, SEMANTIC_CATALOG_SCHEMA, SemanticCatalog,
    SemanticCatalogReference, SemanticLexicalization, SemanticTermDefinition, SemanticTermKind,
    SemanticTermReference, Validation, VocabularyIcon, foundation_semantic_catalog,
    semantic_catalog_digest, semantic_term_definition_digest, validate_semantic_catalog,
};

const COMMUNICATION_CATALOG_ID: &str = "hathq.communication";

/// Returns the immutable generic vocabulary used by protocol-speaking declarations.
///
/// This catalog is separate from foundation v1 so adding communication does not
/// mutate the digest used by already released HAT packages.
///
/// # Panics
///
/// Panics only when the compile-time foundation catalog is internally invalid.
#[must_use]
pub fn communication_semantic_catalog() -> SemanticCatalog {
    let foundation = foundation_semantic_catalog();
    let foundation_ref = |id: &str| {
        foundation
            .terms
            .iter()
            .find(|term| term.reference.term_id == id)
            .expect("communication foundation dependency")
            .reference
            .clone()
    };
    let protocol = communication_term(
        "core.communication-protocol",
        SemanticTermKind::Concept,
        VocabularyIcon::Communication,
        BTreeSet::from([foundation_ref("core.service")]),
    );
    let capability = communication_term(
        "core.communication-capability",
        SemanticTermKind::Concept,
        VocabularyIcon::Communication,
        BTreeSet::from([
            foundation_ref("core.capability"),
            protocol.reference.clone(),
        ]),
    );
    let terms = vec![
        protocol,
        capability.clone(),
        communication_term(
            "core.relation.speaks",
            SemanticTermKind::Relation,
            VocabularyIcon::Relation,
            BTreeSet::from([foundation_ref("core.service"), capability.reference]),
        ),
    ];
    let labels = [
        ("Communication protocol", "通信規約"),
        ("Protocol-speaking capability", "規約を話す能力"),
        ("Speaks protocol", "規約を話せる"),
    ];
    let mut lexicalizations = Vec::new();
    for (term, (en, ja)) in terms.iter().zip(labels) {
        for (locale, preferred) in [("en", en), ("ja", ja)] {
            lexicalizations.push(SemanticLexicalization {
                term: term.reference.clone(),
                locale: locale.into(),
                preferred: preferred.into(),
                search_aliases: BTreeSet::new(),
            });
        }
    }
    let mut catalog = SemanticCatalog {
        schema: SEMANTIC_CATALOG_SCHEMA.into(),
        identity: SemanticCatalogReference {
            catalog_id: COMMUNICATION_CATALOG_ID.into(),
            version: 1,
            digest_sha256: String::new(),
        },
        owner_repository_id: "hat-specifications".into(),
        foundation: false,
        dependencies: vec![foundation.identity],
        incompatibilities: Vec::new(),
        terms,
        lexicalizations,
        frames: Vec::new(),
    };
    let digest = semantic_catalog_digest(&catalog).expect("communication catalog digest");
    catalog.identity.digest_sha256.clone_from(&digest);
    for term in &mut catalog.terms {
        set_communication_digest(&mut term.reference, &digest);
        set_communication_digest_set(&mut term.dependencies, &digest);
    }
    for lexicalization in &mut catalog.lexicalizations {
        set_communication_digest(&mut lexicalization.term, &digest);
    }
    validate_semantic_catalog(&catalog).expect("valid communication semantic catalog");
    catalog
}

fn communication_term(
    id: &str,
    kind: SemanticTermKind,
    icon: VocabularyIcon,
    dependencies: BTreeSet<SemanticTermReference>,
) -> SemanticTermDefinition {
    let mut definition = SemanticTermDefinition {
        reference: SemanticTermReference {
            catalog_id: COMMUNICATION_CATALOG_ID.into(),
            catalog_digest_sha256: String::new(),
            term_id: id.into(),
            version: 1,
            definition_digest_sha256: String::new(),
            kind,
        },
        wire_schema: None,
        semantic_icon: icon,
        dependencies,
        is_a: BTreeSet::new(),
        domain: BTreeSet::new(),
        range: BTreeSet::new(),
    };
    definition.reference.definition_digest_sha256 =
        semantic_term_definition_digest(&definition).expect("communication term digest");
    definition
}

fn set_communication_digest(reference: &mut SemanticTermReference, digest: &str) {
    if reference.catalog_id == COMMUNICATION_CATALOG_ID {
        reference.catalog_digest_sha256 = digest.into();
    }
}

fn set_communication_digest_set(values: &mut BTreeSet<SemanticTermReference>, digest: &str) {
    *values = std::mem::take(values)
        .into_iter()
        .map(|mut reference| {
            set_communication_digest(&mut reference, digest);
            reference
        })
        .collect();
}

/// Exact identity of a communication protocol owned by a standards body or ecosystem repository.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommunicationProtocolReference {
    pub owner_id: String,
    pub protocol_id: String,
    pub version: u64,
    pub specification_ref: String,
}

/// Role implemented by the declaring speaker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CommunicationRole {
    Client,
    Server,
    Peer,
}

/// Message direction supported by the declaring speaker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CommunicationDirection {
    Send,
    Receive,
    Bidirectional,
}

/// One package declaration meaning “this HAT can speak this exact protocol”.
///
/// The declaration is capability metadata only. It contains no endpoint,
/// credential, route, transport policy, permission, or execution authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatCommunicationCapability {
    pub schema: String,
    pub id: String,
    pub protocol: CommunicationProtocolReference,
    pub role: CommunicationRole,
    pub direction: CommunicationDirection,
    pub operation_ids: Vec<String>,
}

/// Validates an exact externally owned protocol reference.
///
/// # Errors
///
/// Returns an error when identity is ambiguous, unversioned, credential-shaped,
/// or not an absolute opaque specification reference.
pub fn validate_communication_protocol_reference(
    value: &CommunicationProtocolReference,
) -> Result<(), String> {
    if !stable_token(&value.owner_id)
        || value.version == 0
        || !absolute_protocol_reference(&value.protocol_id)
        || !absolute_protocol_reference(&value.specification_ref)
    {
        return Err("communication protocol reference is invalid".into());
    }
    Ok(())
}

/// Validates one bounded protocol-speaking declaration.
#[must_use]
pub fn validate_communication_capability(value: &HatCommunicationCapability) -> Validation {
    let mut findings = Vec::new();
    if value.schema != COMMUNICATION_CAPABILITY_SCHEMA {
        findings.push(format!("schema must be {COMMUNICATION_CAPABILITY_SCHEMA}"));
    }
    if !stable_token(&value.id) {
        findings.push("communication capability id is invalid".to_owned());
    }
    if let Err(finding) = validate_communication_protocol_reference(&value.protocol) {
        findings.push(finding);
    }
    let operations = value.operation_ids.iter().collect::<BTreeSet<_>>();
    if value.operation_ids.is_empty()
        || value.operation_ids.len() > 64
        || operations.len() != value.operation_ids.len()
        || value.operation_ids.iter().any(|id| !schema_uri(id))
    {
        findings.push(
            "communication capability operations must be 1..=64 unique canonical actions"
                .to_owned(),
        );
    }
    crate::model::validation(findings)
}

fn absolute_protocol_reference(value: &str) -> bool {
    if value.is_empty()
        || value.len() > 384
        || value.contains('@')
        || value.contains('?')
        || value.contains('#')
        || value.bytes().any(|byte| !byte.is_ascii_graphic())
    {
        return false;
    }
    let Some((scheme, body)) = value.split_once(':') else {
        return false;
    };
    !body.is_empty()
        && scheme.len() >= 2
        && scheme.len() <= 32
        && scheme
            .bytes()
            .next()
            .is_some_and(|byte| byte.is_ascii_lowercase())
        && scheme.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'+' | b'.' | b'-')
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_and_ecosystem_protocols_share_one_reference_shape() {
        for value in [
            CommunicationProtocolReference {
                owner_id: "ietf".into(),
                protocol_id: "urn:ietf:rfc:9112".into(),
                version: 1,
                specification_ref: "https://www.rfc-editor.org/rfc/rfc9112".into(),
            },
            CommunicationProtocolReference {
                owner_id: "zixcel-aws".into(),
                protocol_id: "zixcel://aws/sign-request/v1".into(),
                version: 1,
                specification_ref: "zixcel://aws/sign-request/v1".into(),
            },
        ] {
            assert!(validate_communication_protocol_reference(&value).is_ok());
        }
    }

    #[test]
    fn endpoints_credentials_and_unversioned_references_are_rejected() {
        let mut value = CommunicationProtocolReference {
            owner_id: "ietf".into(),
            protocol_id: "https://user@example.test/protocol".into(),
            version: 1,
            specification_ref: "https://example.test/spec".into(),
        };
        assert!(validate_communication_protocol_reference(&value).is_err());
        value.protocol_id = "urn:ietf:rfc:9112".into();
        value.version = 0;
        assert!(validate_communication_protocol_reference(&value).is_err());
    }
}
