use std::collections::BTreeSet;

use crate::model::validation;
use crate::tokens::{namespaced_id, schema_uri, semantic_version, stable_token};
use crate::{HatOperation, HatPackage, PACKAGE_SCHEMA, SemanticTermKind, Validation};

#[must_use]
pub fn validate_package(package: &HatPackage) -> Validation {
    let mut findings = Vec::new();
    if package.schema != PACKAGE_SCHEMA {
        findings.push(format!("schema must be {PACKAGE_SCHEMA}"));
    }
    namespaced_id(&package.package_id, "hat/", "package id", &mut findings);
    if package.package_id != package.manifest.id || package.version != package.manifest.version {
        findings.push("package identity must equal manifest identity".to_owned());
    }
    if !stable_token(&package.repository_id) {
        findings.push("repository_id must be a stable token".to_owned());
    }
    if !semantic_version(&package.specification_version) {
        findings.push("specification_version must be semantic".to_owned());
    }
    findings.extend(crate::validate_manifest(&package.manifest).findings);
    validate_catalog(package, &mut findings);
    let surface_ids = validate_information_surfaces(package, &mut findings);
    validate_service_requirements(package, &mut findings);
    validate_communication_capabilities(package, &mut findings);
    validate_setup_templates(package, &mut findings);
    validate_operations(&package.operations, &surface_ids, &mut findings);
    validation(findings)
}

fn validate_setup_templates(package: &HatPackage, findings: &mut Vec<String>) {
    if package.setup_templates.len() > 32 {
        findings.push("package must contain at most 32 setup templates".to_owned());
    }
    let mut ids = BTreeSet::new();
    for template in &package.setup_templates {
        if !ids.insert(template.id.as_str()) || !stable_token(&template.id) {
            findings.push("setup template IDs must be unique stable tokens".to_owned());
        }
        if !bounded_text(&template.title, 160)
            || !bounded_text(&template.summary, 512)
            || !bounded_text(&template.action_label, 120)
        {
            findings.push("setup template presentation text is invalid".to_owned());
        }
        if !stable_token(&template.context_namespace) || !schema_uri(&template.projection_schema) {
            findings.push("setup template context binding is invalid".to_owned());
        }
        let declared = package.operations.iter().any(|operation| {
            operation.context_plan.selectors.iter().any(|selector| {
                selector.required
                    && selector.namespace == template.context_namespace
                    && selector.projection_schema == template.projection_schema
            })
        });
        if !declared {
            findings.push("setup templates must reference a required operation context".to_owned());
        }
    }
}

fn bounded_text(value: &str, max: usize) -> bool {
    !value.trim().is_empty()
        && value == value.trim()
        && value.len() <= max
        && !value.chars().any(char::is_control)
}

fn validate_service_requirements(package: &HatPackage, findings: &mut Vec<String>) {
    if package.service_requirements.len() > 32 {
        findings.push("package must contain at most 32 service requirements".to_owned());
    }
    let mut ids = BTreeSet::new();
    for requirement in &package.service_requirements {
        if !stable_token(&requirement.id) || !ids.insert(requirement.id.as_str()) {
            findings.push("service requirement IDs must be unique stable tokens".to_owned());
        }
        if requirement.operation_ids.is_empty() || requirement.operation_ids.len() > 16 {
            findings.push("service requirement must contain 1..=16 operations".to_owned());
        }
        let mut operations = BTreeSet::new();
        for operation_id in &requirement.operation_ids {
            if !schema_uri(operation_id) || !operations.insert(operation_id.as_str()) {
                findings.push(
                    "service requirement operations must be unique canonical URIs".to_owned(),
                );
            }
        }
        let protocols = requirement
            .accepted_protocols
            .iter()
            .collect::<BTreeSet<_>>();
        if requirement.accepted_protocols.len() > 16
            || protocols.len() != requirement.accepted_protocols.len()
        {
            findings.push("service requirement protocols must be unique and bounded".to_owned());
        }
        for protocol in &requirement.accepted_protocols {
            if let Err(finding) = crate::validate_communication_protocol_reference(protocol) {
                findings.push(finding);
            }
        }
    }
}

fn validate_communication_capabilities(package: &HatPackage, findings: &mut Vec<String>) {
    if package.communication_capabilities.len() > 32 {
        findings.push("package must contain at most 32 communication capabilities".to_owned());
    }
    let operation_ids = package
        .operations
        .iter()
        .map(|operation| operation.id.as_str())
        .collect::<BTreeSet<_>>();
    let mut ids = BTreeSet::new();
    for capability in &package.communication_capabilities {
        findings.extend(crate::validate_communication_capability(capability).findings);
        if !ids.insert(capability.id.as_str()) {
            findings.push("communication capability IDs must be unique".to_owned());
        }
        if capability
            .operation_ids
            .iter()
            .any(|operation| !operation_ids.contains(operation.as_str()))
        {
            findings.push(
                "communication capability must reference package-owned operations".to_owned(),
            );
        }
    }
}

fn validate_information_surfaces<'a>(
    package: &'a HatPackage,
    findings: &mut Vec<String>,
) -> BTreeSet<&'a str> {
    if package.information_surfaces.len() > 64 {
        findings.push("package must contain at most 64 information surfaces".to_owned());
    }
    let mut ids = BTreeSet::new();
    for surface in &package.information_surfaces {
        if !ids.insert(surface.id.as_str()) || !stable_token(&surface.id) {
            findings.push("information surface IDs must be unique stable tokens".to_owned());
        }
        if !canonical_type(&surface.canonical_type) {
            findings.push("information surface canonical_type is invalid".to_owned());
        }
        if !schema_uri(&surface.projection_schema) {
            findings.push("information surface projection_schema is invalid".to_owned());
        }
        if !schema_uri(&surface.data_domain_term_id)
            || !surface
                .data_domain_term_id
                .starts_with("hathq://vocabulary/data-domain/")
        {
            findings.push("information surface data-domain term is invalid".to_owned());
        }
    }
    ids
}

fn canonical_type(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.split('.').count() >= 2
        && value.split('.').all(stable_token)
}

fn validate_catalog(package: &HatPackage, findings: &mut Vec<String>) {
    if let Err(finding) = crate::validate_semantic_catalog(&package.catalog) {
        findings.push(finding);
        return;
    }
    if package.catalog.foundation
        || package.catalog.owner_repository_id != package.repository_id
        || package.catalog.identity.catalog_id != package.repository_id
    {
        findings.push("package catalog identity must equal its repository identity".to_owned());
    }
    let foundation = crate::foundation_semantic_catalog();
    if package
        .catalog
        .dependencies
        .iter()
        .filter(|dependency| *dependency == &foundation.identity)
        .count()
        != 1
    {
        findings.push("package catalog must depend on the exact foundation catalog".to_owned());
    }
    let uses_communication = !package.communication_capabilities.is_empty()
        || package
            .service_requirements
            .iter()
            .any(|requirement| !requirement.accepted_protocols.is_empty());
    let communication = crate::communication_semantic_catalog();
    if uses_communication
        && package
            .catalog
            .dependencies
            .iter()
            .filter(|dependency| *dependency == &communication.identity)
            .count()
            != 1
    {
        findings.push(
            "protocol-speaking packages must depend on the exact communication catalog".to_owned(),
        );
    }
    if package
        .catalog
        .terms
        .iter()
        .any(|definition| definition.wire_schema.is_none())
    {
        findings.push("package catalog terms must declare an exact wire schema".to_owned());
    }
    for operation in &package.operations {
        if package
            .catalog
            .terms
            .iter()
            .filter(|definition| {
                definition.reference.term_id == operation.id
                    && definition.reference.kind == SemanticTermKind::Action
                    && definition.wire_schema.as_deref() == Some(operation.input_schema.as_str())
            })
            .count()
            != 1
        {
            findings.push(
                "operation must resolve to one wire-bound action in the package catalog".to_owned(),
            );
        }
        if package
            .catalog
            .frames
            .iter()
            .filter(|frame| frame.predicate.term_id == operation.id)
            .count()
            != 1
        {
            findings.push(
                "operation must resolve to one clause frame in the package catalog".to_owned(),
            );
        }
    }
}

fn validate_operations(
    operations: &[HatOperation],
    surface_terms: &BTreeSet<&str>,
    findings: &mut Vec<String>,
) {
    if operations.is_empty() || operations.len() > 128 {
        findings.push("package must contain 1..=128 operations".to_owned());
    }
    let mut ids = BTreeSet::new();
    for operation in operations {
        if !ids.insert(operation.id.as_str()) || !schema_uri(&operation.id) {
            findings.push("operation IDs must be unique canonical URIs".to_owned());
        }
        validate_operation(operation, surface_terms, findings);
    }
}

fn validate_operation(
    operation: &HatOperation,
    surface_terms: &BTreeSet<&str>,
    findings: &mut Vec<String>,
) {
    for schema in [&operation.input_schema, &operation.output_schema] {
        if !schema_uri(schema) {
            findings.push("operation schema is invalid".to_owned());
        }
    }
    let plan = &operation.context_plan;
    if !stable_token(&plan.id) || plan.selectors.is_empty() || plan.selectors.len() > 64 {
        findings.push("context plan is invalid".to_owned());
    }
    if plan.unresolved_policy != "record-unresolved" {
        findings.push("unknown terms must remain unresolved".to_owned());
    }
    for selector in &plan.selectors {
        if !stable_token(&selector.namespace) || !schema_uri(&selector.projection_schema) {
            findings.push("context selector is invalid".to_owned());
        }
    }
    if operation.effects.len() > 16 {
        findings.push("operation must contain at most 16 information effects".to_owned());
    }
    let mut effects = BTreeSet::new();
    for effect in &operation.effects {
        if !surface_terms.contains(effect.surface_id.as_str())
            || !effects.insert((effect.kind, effect.surface_id.as_str()))
        {
            findings.push(
                "operation effects must be unique and reference an information surface".to_owned(),
            );
        }
    }
    validate_reducer_and_procedure(operation, findings);
}

fn validate_reducer_and_procedure(operation: &HatOperation, findings: &mut Vec<String>) {
    let reducer = &operation.reducer;
    if !stable_token(&reducer.id)
        || reducer.strategy != "typed-domain-event"
        || !schema_uri(&reducer.event_schema)
        || !schema_uri(&reducer.projection_schema)
        || reducer.conflict_policy != "reject"
    {
        findings.push("reducer must be deterministic and fail closed".to_owned());
    }
    let procedure = &operation.procedure;
    if !stable_token(&procedure.id) || procedure.steps.is_empty() || procedure.steps.len() > 64 {
        findings.push("procedure is invalid".to_owned());
    }
    for step in &procedure.steps {
        if !schema_uri(&step.action_id)
            || !schema_uri(&step.input_schema)
            || !schema_uri(&step.output_schema)
        {
            findings.push("procedure step is invalid".to_owned());
        }
    }
    if !matches!(
        operation.handler.kind.as_str(),
        "declarative-hat" | "hat-service" | "ecosystem-adapter" | "delegated-hat"
    ) || operation.handler.reference.is_empty()
        || operation.handler.reference.len() > 256
    {
        findings.push("handler reference is invalid".to_owned());
    }
}
