use hat_specifications::{
    SemanticCatalog, foundation_semantic_catalog, parse_package, semantic_catalog_digest,
    validate_package,
};
use sha2::{Digest, Sha256};

#[test]
fn old_frames_are_unknown_fields_even_when_empty() {
    let mut value = serde_json::to_value(foundation_semantic_catalog()).expect("catalog");
    value["frames"] = serde_json::json!([]);
    let error = serde_json::from_value::<SemanticCatalog>(value).expect_err("old shape refused");
    assert!(error.to_string().contains("unknown field `frames`"));
}

#[test]
fn physical_schema_has_no_linguistic_frame_model_or_decoder() {
    let schema: serde_json::Value = serde_json::from_str(include_str!(
        "../schemas/hat-semantic-catalog-v1.schema.json"
    ))
    .expect("schema");
    assert_eq!(schema["additionalProperties"], false);
    assert!(
        !schema["required"]
            .as_array()
            .expect("required")
            .iter()
            .any(|v| v == "frames")
    );
    assert!(schema["properties"].get("frames").is_none());
    for old in ["role", "roleSpec", "frame"] {
        assert!(schema["$defs"].get(old).is_none());
    }
}

#[test]
fn empty_frame_removal_changes_canonical_digest_and_requires_new_package_bytes() {
    let catalog = foundation_semantic_catalog();
    let mut canonical = catalog.clone();
    canonical.identity.digest_sha256.clear();
    for term in &mut canonical.terms {
        term.reference.catalog_digest_sha256.clear();
        for references in [
            &mut term.dependencies,
            &mut term.is_a,
            &mut term.domain,
            &mut term.range,
        ] {
            *references = references
                .iter()
                .cloned()
                .map(|mut r| {
                    r.catalog_digest_sha256.clear();
                    r
                })
                .collect();
        }
    }
    for display in &mut canonical.lexicalizations {
        display.term.catalog_digest_sha256.clear();
    }
    let current = format!(
        "{:x}",
        Sha256::digest(
            serde_json::to_vec(&serde_json::to_value(&canonical).expect("canonical value"))
                .expect("canonical")
        )
    );
    assert_eq!(
        current,
        semantic_catalog_digest(&catalog).expect("current digest")
    );
    let mut old = serde_json::to_value(&canonical).expect("canonical object");
    old["frames"] = serde_json::json!([]);
    assert_ne!(
        current,
        format!(
            "{:x}",
            Sha256::digest(serde_json::to_vec(&old).expect("old shape"))
        )
    );
    let package = parse_package(include_str!("../examples/source-curator.package.json"))
        .expect("new example");
    assert!(validate_package(&package).valid);
    assert!(package.catalog.dependencies.contains(&catalog.identity));
}
