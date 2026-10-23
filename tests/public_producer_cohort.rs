//! Offline acceptance of exact public producer descriptors, not execution authority.
use hat_specifications::{
    ActionReference, BINDING_SCHEMA, ContextPartition, HatBinding, HatInvocation, HatPackage,
    INVOCATION_SCHEMA, foundation_semantic_catalog, parse_package, semantic_catalog_digest,
    validate_invocation_against_package, validate_package,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

fn sources() -> Value {
    serde_json::from_str(include_str!("fixtures/public-producers/sources.json")).expect("sources")
}

fn bytes(name: &str) -> &'static str {
    match name {
        "hat-source-curator.package.json" => {
            include_str!("fixtures/public-producers/hat-source-curator.package.json")
        }
        "hat-website-operator.package.json" => {
            include_str!("fixtures/public-producers/hat-website-operator.package.json")
        }
        _ => panic!("unselected producer"),
    }
}

fn packages() -> Vec<HatPackage> {
    sources()["sources"]
        .as_array()
        .expect("selected producers")
        .iter()
        .map(|source| {
            parse_package(bytes(source["fixture"].as_str().expect("fixture")))
                .expect("public producer package")
        })
        .collect()
}

#[test]
fn exact_public_producer_bytes_and_foundation_dependencies_are_accepted() {
    let sources = sources();
    let entries = sources["sources"].as_array().expect("sources");
    assert_eq!(entries.len(), 2);
    let foundation = foundation_semantic_catalog();
    for source in entries {
        let raw = bytes(source["fixture"].as_str().expect("fixture"));
        assert_eq!(
            json!(hex::encode(Sha256::digest(raw.as_bytes()))),
            source["sha256"]
        );
        let package = parse_package(raw).expect("public producer bytes");
        let validation = validate_package(&package);
        assert!(validation.valid, "{:?}", validation.findings);
        assert_eq!(json!(package.package_id), source["package_id"]);
        assert_eq!(json!(package.version), source["version"]);
        assert_eq!(
            json!(package.catalog.identity.digest_sha256),
            source["catalog_sha256"]
        );
        assert_eq!(
            semantic_catalog_digest(&package.catalog).expect("catalog digest"),
            package.catalog.identity.digest_sha256
        );
        assert!(package.catalog.dependencies.contains(&foundation.identity));
        assert!(!package.catalog.foundation);
    }
}

#[test]
fn modified_wire_contract_or_catalog_identity_cannot_reuse_selected_bytes() {
    for package in packages() {
        let original = serde_json::to_value(&package).expect("package");
        for mutation in ["wire", "catalog", "term"] {
            let mut changed = package.clone();
            match mutation {
                "wire" => {
                    changed.catalog.terms[0].wire_schema = Some("urn:test:substituted".into())
                }
                "catalog" => changed.catalog.identity.digest_sha256 = "f".repeat(64),
                _ => changed.catalog.terms[0].reference.definition_digest_sha256 = "f".repeat(64),
            }
            assert!(
                !validate_package(&changed).valid,
                "accepted changed {mutation}"
            );
        }
        let mut old_shape = original.clone();
        old_shape["catalog"]["frames"] = json!([]);
        assert!(parse_package(&old_shape.to_string()).is_err());
        let mut unsupported = original;
        unsupported["schema"] = json!("hathq://hat/package/v999");
        assert!(
            !validate_package(&parse_package(&unsupported.to_string()).expect("closed shape"))
                .valid
        );
    }
}

fn reference(schema: &str) -> ActionReference {
    ActionReference {
        owner_id: "fixture-owner".into(),
        reference: "local://fixture/selected".into(),
        schema_id: schema.into(),
        digest_sha256: "a".repeat(64),
    }
}

fn invocation(package: &HatPackage) -> HatInvocation {
    HatInvocation {
        schema: INVOCATION_SCHEMA.into(),
        invocation_id: "fixture-invocation".into(),
        binding: HatBinding {
            schema: BINDING_SCHEMA.into(),
            package_id: package.package_id.clone(),
            package_digest_sha256: "a".repeat(64),
            catalog_digest_sha256: package.catalog.identity.digest_sha256.clone(),
            fitting_digest_sha256: "b".repeat(64),
            subject_ref: "subject:fixture".into(),
            scope_ref: "scope:fixture".into(),
        },
        context_partition: ContextPartition {
            context_partition_id: "fixture-context".into(),
            revision: 1,
            policy_digest_sha256: "c".repeat(64),
        },
        operation_id: package.operations[0].id.clone(),
        expected_projection_revision: 1,
        idempotency_key: "fixture-request".into(),
        input: reference(&package.operations[0].input_schema),
        effective_grant: reference("hathq://hat/effective-grant/v1"),
        placement: reference("hathq://hat/placement/v1"),
    }
}

#[test]
fn actual_producer_operation_binding_rejects_cross_package_and_wire_substitution() {
    for package in packages() {
        let selected = invocation(&package);
        assert!(validate_invocation_against_package(&selected, &package).valid);
        for mutation in ["package", "operation", "input", "missing-grant-reference"] {
            let mut changed = selected.clone();
            match mutation {
                "package" => changed.binding.package_id = "hat/unselected".into(),
                "operation" => changed.operation_id = "urn:test:unselected".into(),
                "input" => changed.input.schema_id = "urn:test:wrong-input".into(),
                _ => changed.effective_grant.digest_sha256.clear(),
            }
            assert!(!validate_invocation_against_package(&changed, &package).valid);
        }
    }
    // A structurally valid reference is not a verified grant. No host, issuer,
    // credential or dispatch API is invoked by this contract validator.
}

#[test]
fn source_inventory_does_not_advertise_retired_files_as_current_interfaces() {
    let map: Value = serde_json::from_str(include_str!("../docs/oss-source-map.json"))
        .expect("source inventory");
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    for key in ["schema_definitions", "test_sources", "example_sources"] {
        for path in map[key].as_array().expect("source paths") {
            assert!(
                root.join(path.as_str().expect("path")).is_file(),
                "stale {key}: {path}"
            );
        }
    }
    for workflow in map["workflow_definitions"]
        .as_array()
        .expect("workflow paths")
    {
        assert!(
            root.join(workflow["path"].as_str().expect("workflow"))
                .is_file()
        );
        assert!(
            workflow.get("jobs").is_none(),
            "workflow source must not be duplicated in documentation"
        );
    }
}
