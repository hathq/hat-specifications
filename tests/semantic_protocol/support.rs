use hat_specifications::*;
use std::collections::BTreeSet;

pub fn term(kind: SemanticTermKind, id: &str, byte: char) -> SemanticTermReference {
    SemanticTermReference {
        catalog_id: "hathq.test.semantic".into(),
        catalog_digest_sha256: catalog().identity.digest_sha256,
        term_id: id.into(),
        version: 1,
        definition_digest_sha256: hex(byte),
        kind,
    }
}

pub fn clause() -> SemanticClause {
    let catalog = catalog();
    let action = catalog.terms[0].reference.clone();
    let mut frame = SemanticClauseFrame {
        frame_id: "hathq://frame/schedule/v1".into(),
        definition_digest_sha256: String::new(),
        predicate: action,
        roles: vec![
            SemanticRoleSpec {
                role: SemanticClauseRole::Actor,
                required: true,
                accepted_kinds: BTreeSet::from([SemanticTermKind::EntityType]),
            },
            SemanticRoleSpec {
                role: SemanticClauseRole::Object,
                required: true,
                accepted_kinds: BTreeSet::from([SemanticTermKind::EntityType]),
            },
        ],
    };
    frame.definition_digest_sha256 = semantic_clause_frame_digest(&frame).expect("frame digest");
    SemanticClause {
        schema: SEMANTIC_CLAUSE_SCHEMA.into(),
        clause_id: "schedule-checkup".into(),
        kind: SemanticClauseKind::Action,
        frame,
        bindings: vec![
            SemanticRoleBinding {
                role: SemanticClauseRole::Actor,
                term: None,
                reference: Some(reference("subject:alice", 'd')),
                value: None,
            },
            SemanticRoleBinding {
                role: SemanticClauseRole::Object,
                term: None,
                reference: Some(reference("checkup:1", 'e')),
                value: None,
            },
        ],
        modality: SemanticModality::Proposed,
        positive: true,
    }
}

pub fn catalog() -> SemanticCatalog {
    let mut definition = SemanticTermDefinition {
        reference: SemanticTermReference {
            catalog_id: "hathq.test.semantic".into(),
            catalog_digest_sha256: String::new(),
            term_id: "hathq://vocabulary/action/schedule/v1".into(),
            version: 1,
            definition_digest_sha256: String::new(),
            kind: SemanticTermKind::Action,
        },
        wire_schema: Some("hathq://hat-specifications/semantic-fixture/v1".into()),
        semantic_icon: VocabularyIcon::Action,
        dependencies: BTreeSet::new(),
        is_a: BTreeSet::new(),
        domain: BTreeSet::new(),
        range: BTreeSet::new(),
    };
    definition.reference.definition_digest_sha256 =
        semantic_term_definition_digest(&definition).expect("definition digest");
    let mut value = SemanticCatalog {
        schema: SEMANTIC_CATALOG_SCHEMA.into(),
        identity: SemanticCatalogReference {
            catalog_id: "hathq.test.semantic".into(),
            version: 1,
            digest_sha256: String::new(),
        },
        owner_repository_id: "hat-semantic-protocol-test".into(),
        foundation: false,
        dependencies: Vec::new(),
        incompatibilities: Vec::new(),
        terms: vec![definition],
        lexicalizations: Vec::new(),
        frames: Vec::new(),
    };
    value.identity.digest_sha256 = semantic_catalog_digest(&value).expect("catalog digest");
    value.terms[0]
        .reference
        .catalog_digest_sha256
        .clone_from(&value.identity.digest_sha256);
    value
}

pub fn exchange() -> SemanticExchange {
    let clause = clause();
    let mut value = SemanticExchange {
        schema: SEMANTIC_EXCHANGE_SCHEMA.into(),
        exchange_id: "exchange-1".into(),
        sender: reference("subject:alice", 'd'),
        receiver: reference("subject:bob", '6'),
        terms: vec![clause.frame.predicate.clone()],
        frame_digest_sha256: clause.frame.definition_digest_sha256.clone(),
        output_schema_digest_sha256: hex('7'),
        clause,
        source_revision: 1,
        evidence: vec![reference("evidence:calendar", '8')],
        information_classification: "internal".into(),
        purpose: "share-schedule".into(),
        retention: reference("retention:owner-choice", '9'),
        issued_at_epoch_s: 100,
        expires_at_epoch_s: 200,
        nonce: "nonce-1".into(),
        content_digest_sha256: String::new(),
    };
    value.content_digest_sha256 = semantic_exchange_digest(&value).expect("exchange digest");
    value
}

pub fn reference(id: &str, byte: char) -> SemanticStableRef {
    SemanticStableRef {
        schema: "hathq://semantic/reference/v1".into(),
        id: id.into(),
        revision: 1,
        digest_sha256: hex(byte),
    }
}

pub fn hex(value: char) -> String {
    value.to_string().repeat(64)
}
