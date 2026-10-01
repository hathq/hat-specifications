use crate::{
    EventSemantics, HatSemanticContribution, SEMANTIC_CONTRIBUTION_SCHEMA, SemanticAssertion,
    SemanticBinding, SemanticCatalog, SemanticEvent, SemanticScalar, SemanticStableRef,
    SemanticTermKind,
};
use std::collections::{BTreeMap, BTreeSet};

/// Validates one closed, bounded and reference-only semantic contribution.
///
/// # Errors
/// Returns a stable description when shape, bounds, references, vocabulary, or time are invalid.
pub fn validate_semantic_contribution_against_catalogs(
    value: &HatSemanticContribution,
    accepted_catalogs: &[SemanticCatalog],
) -> Result<(), String> {
    if value.schema != SEMANTIC_CONTRIBUTION_SCHEMA {
        return Err("semantic contribution schema is invalid".into());
    }
    reference(&value.producer_hat)?;
    reference(&value.canonical_source)?;
    digest(&value.content_digest_sha256)?;
    if value.source_revision != value.canonical_source.revision {
        return Err("semantic source revision mismatch".into());
    }
    let known_terms = catalogs(value, accepted_catalogs)?;
    identities(value)?;
    if value.entities.len() > 4096
        || value.relations.len() > 8192
        || value.events.len() > 8192
        || value.assertions.len() > 8192
        || value.evidence.len() > 8192
        || value.bindings.len() > 256
    {
        return Err("semantic contribution collection exceeds its bound".into());
    }
    for entity in &value.entities {
        reference(&entity.identity)?;
        if entity.types.is_empty() || entity.types.len() > 32 || entity.properties.len() > 128 {
            return Err("semantic entity shape is invalid".into());
        }
        for semantic_type in &entity.types {
            token(semantic_type, "semantic type")?;
            require_term(
                &known_terms,
                semantic_type,
                crate::SemanticTermKind::EntityType,
            )?;
        }
        for (key, value) in &entity.properties {
            token(key, "semantic property")?;
            require_term(&known_terms, key, crate::SemanticTermKind::Property)?;
            scalar(value)?;
        }
    }
    for relation in &value.relations {
        token(&relation.semantic_id, "relation id")?;
        token(&relation.relation_type, "relation type")?;
        require_term(
            &known_terms,
            &relation.relation_type,
            crate::SemanticTermKind::Relation,
        )?;
        reference(&relation.from)?;
        reference(&relation.to)?;
        digest(&relation.content_digest_sha256)?;
        for value in relation.properties.values() {
            scalar(value)?;
        }
    }
    for event in &value.events {
        validate_event(event, &known_terms)?;
    }
    for assertion in &value.assertions {
        validate_assertion(assertion, &known_terms)?;
    }
    for evidence in &value.evidence {
        reference(&evidence.identity)?;
        reference(&evidence.source)?;
    }
    for source in &value.sources {
        reference(&source.identity)?;
        token(&source.source_type, "source type")?;
        require_term(
            &known_terms,
            &source.source_type,
            crate::SemanticTermKind::EntityType,
        )?;
    }
    for binding in &value.bindings {
        validate_binding(binding)?;
    }
    Ok(())
}

fn identities(value: &HatSemanticContribution) -> Result<(), String> {
    let mut references = BTreeMap::<(String, String), (u64, String)>::new();
    let mut record_ids = BTreeSet::new();
    let mut add_reference = |value: &SemanticStableRef| -> Result<(), String> {
        let key = (value.schema.clone(), value.id.clone());
        let identity = (value.revision, value.digest_sha256.clone());
        if references
            .insert(key, identity.clone())
            .is_some_and(|old| old != identity)
        {
            return Err("semantic reference identity is inconsistent".into());
        }
        Ok(())
    };
    add_reference(&value.producer_hat)?;
    add_reference(&value.canonical_source)?;
    for entity in &value.entities {
        add_reference(&entity.identity)?;
    }
    for source in &value.sources {
        add_reference(&source.identity)?;
    }
    for evidence in &value.evidence {
        add_reference(&evidence.identity)?;
        add_reference(&evidence.source)?;
    }
    for relation in &value.relations {
        unique(&mut record_ids, "relation", &relation.semantic_id)?;
        add_reference(&relation.from)?;
        add_reference(&relation.to)?;
    }
    for event in &value.events {
        unique(&mut record_ids, "event", &event.semantic_id)?;
        add_reference(&event.producer)?;
        add_reference(&event.source)?;
        for reference in event
            .actors
            .iter()
            .chain(&event.objects)
            .chain(&event.evidence)
        {
            add_reference(reference)?;
        }
    }
    for assertion in &value.assertions {
        unique(&mut record_ids, "assertion", &assertion.semantic_id)?;
        add_reference(&assertion.subject)?;
        add_reference(&assertion.producer)?;
        add_reference(&assertion.source)?;
        for reference in &assertion.evidence {
            add_reference(reference)?;
        }
        if let SemanticScalar::Reference(reference) = &assertion.value {
            add_reference(reference)?;
        }
    }
    for binding in &value.bindings {
        unique(&mut record_ids, "binding", &binding.binding_id)?;
        for reference in [
            &binding.subject_ref,
            &binding.scope_ref,
            &binding.hat_package_ref,
            &binding.fitting_ref,
            &binding.policy_ref,
            &binding.provider_ref,
        ] {
            add_reference(reference)?;
        }
    }
    Ok(())
}

fn unique(values: &mut BTreeSet<String>, kind: &str, id: &str) -> Result<(), String> {
    if !values.insert(format!("{kind}\0{id}")) {
        return Err("semantic record identity is duplicated".into());
    }
    Ok(())
}

fn validate_event(
    value: &SemanticEvent,
    known: &BTreeMap<&str, SemanticTermKind>,
) -> Result<(), String> {
    token(&value.semantic_id, "event id")?;
    validate_event_semantics(&value.semantics)?;
    require_term(
        known,
        &value.semantics.event_type,
        crate::SemanticTermKind::EventType,
    )?;
    reference(&value.producer)?;
    reference(&value.source)?;
    if value.actors.len() > 64
        || value.objects.len() > 256
        || value.evidence.len() > 256
        || value.properties.len() > 128
    {
        return Err("semantic event shape exceeds its bound".into());
    }
    for reference_value in value
        .actors
        .iter()
        .chain(&value.objects)
        .chain(&value.evidence)
    {
        reference(reference_value)?;
    }
    for (key, property) in &value.properties {
        token(key, "event property")?;
        require_term(known, key, crate::SemanticTermKind::Property)?;
        scalar(property)?;
    }
    digest(&value.content_digest_sha256)
}

pub(crate) fn validate_event_semantics(value: &EventSemantics) -> Result<(), String> {
    token(&value.event_type, "event type")?;
    interval(&value.occurred_time)?;
    if value.observed_at_epoch_ms < value.occurred_time.start_epoch_ms {
        return Err("event observation precedes its occurrence".into());
    }
    if value
        .correlation_id
        .as_deref()
        .is_some_and(|id| token(id, "event correlation id").is_err())
        || value
            .causation_id
            .as_deref()
            .is_some_and(|id| token(id, "event causation id").is_err())
    {
        return Err("event correlation is invalid".into());
    }
    Ok(())
}

fn catalogs<'a>(
    value: &HatSemanticContribution,
    accepted_catalogs: &'a [SemanticCatalog],
) -> Result<BTreeMap<&'a str, SemanticTermKind>, String> {
    if value.catalogs.is_empty() || value.catalogs.len() > 64 {
        return Err("semantic catalog reference collection is invalid".into());
    }
    let accepted = accepted_catalogs
        .iter()
        .map(|catalog| {
            crate::validate_semantic_catalog(catalog)?;
            Ok((
                (
                    catalog.identity.catalog_id.as_str(),
                    catalog.identity.version,
                    catalog.identity.digest_sha256.as_str(),
                ),
                catalog,
            ))
        })
        .collect::<Result<BTreeMap<_, _>, String>>()?;
    let declared = value
        .catalogs
        .iter()
        .map(|reference| {
            crate::validate_semantic_catalog_reference(reference)?;
            let key = (
                reference.catalog_id.as_str(),
                reference.version,
                reference.digest_sha256.as_str(),
            );
            if !accepted.contains_key(&key) {
                return Err("semantic contribution catalog is not accepted".to_owned());
            }
            Ok(key)
        })
        .collect::<Result<BTreeSet<_>, String>>()?;
    if declared.len() != value.catalogs.len() {
        return Err("semantic contribution catalog is duplicated".into());
    }
    let mut known = BTreeMap::new();
    for key in &declared {
        let catalog = accepted
            .get(key)
            .ok_or_else(|| "semantic contribution catalog is not accepted".to_owned())?;
        for dependency in &catalog.dependencies {
            let dependency_key = (
                dependency.catalog_id.as_str(),
                dependency.version,
                dependency.digest_sha256.as_str(),
            );
            if !declared.contains(&dependency_key) {
                return Err("semantic contribution catalog dependency is absent".into());
            }
        }
        for definition in &catalog.terms {
            match known.insert(
                definition.reference.term_id.as_str(),
                definition.reference.kind,
            ) {
                Some(existing) if existing != definition.reference.kind => {
                    return Err("semantic catalog term identity conflicts".into());
                }
                Some(_) => return Err("semantic catalog term identity is duplicated".into()),
                None => {}
            }
        }
    }
    Ok(known)
}

fn validate_assertion(
    value: &SemanticAssertion,
    known: &BTreeMap<&str, SemanticTermKind>,
) -> Result<(), String> {
    token(&value.semantic_id, "assertion id")?;
    token(&value.predicate, "assertion predicate")?;
    require_term(known, &value.predicate, crate::SemanticTermKind::Predicate)?;
    reference(&value.subject)?;
    reference(&value.producer)?;
    reference(&value.source)?;
    scalar(&value.value)?;
    digest(&value.content_digest_sha256)?;
    token(&value.policy_classification, "policy classification")?;
    require_term(
        known,
        &value.policy_classification,
        crate::SemanticTermKind::StateType,
    )?;
    if value.resolved_at_epoch_ms < value.observed_at_epoch_ms {
        return Err("semantic assertion time is invalid".into());
    }
    if let Some(value) = &value.valid_time {
        interval(value)?;
    }
    for evidence in &value.evidence {
        reference(evidence)?;
    }
    if let Some(interpretation) = &value.interpretation {
        reference(&interpretation.policy_ref)?;
        if let Some(term) = &interpretation.jurisdiction_term {
            token(term, "jurisdiction term")?;
            if !term.starts_with("jurisdiction.") {
                return Err("jurisdiction interpretation term is invalid".into());
            }
            require_term(known, term, crate::SemanticTermKind::Concept)?;
        }
    }
    Ok(())
}

fn require_term(
    known: &BTreeMap<&str, SemanticTermKind>,
    term_id: &str,
    expected: crate::SemanticTermKind,
) -> Result<(), String> {
    match known.get(term_id) {
        Some(kind) if *kind == expected => Ok(()),
        Some(_) => Err("semantic vocabulary term kind differs from its use".into()),
        None => Err("semantic record uses an undeclared vocabulary term".into()),
    }
}

fn interval(value: &crate::SemanticTimeInterval) -> Result<(), String> {
    if value
        .end_epoch_ms
        .is_some_and(|end| end < value.start_epoch_ms)
    {
        return Err("semantic time interval is invalid".into());
    }
    Ok(())
}

fn validate_binding(value: &SemanticBinding) -> Result<(), String> {
    token(&value.binding_id, "binding id")?;
    for value in [
        &value.subject_ref,
        &value.scope_ref,
        &value.hat_package_ref,
        &value.fitting_ref,
        &value.policy_ref,
        &value.provider_ref,
    ] {
        reference(value)?;
        if value.id.starts_with('/') || value.id.contains('\\') {
            return Err("semantic binding contains a local path identity".into());
        }
    }
    Ok(())
}

pub(crate) fn reference(value: &SemanticStableRef) -> Result<(), String> {
    token(&value.schema, "reference schema")?;
    token(&value.id, "reference id")?;
    digest(&value.digest_sha256)
}

pub(crate) fn scalar(value: &SemanticScalar) -> Result<(), String> {
    match value {
        SemanticScalar::Float(value) if !value.is_finite() => {
            Err("semantic float must be finite".into())
        }
        SemanticScalar::String(value) if value.len() > 4096 => {
            Err("semantic string exceeds 4096 bytes".into())
        }
        SemanticScalar::Bytes(value) if value.len() > 65_536 => {
            Err("semantic bytes exceed 65536 bytes".into())
        }
        SemanticScalar::Reference(value) => reference(value),
        _ => Ok(()),
    }
}

pub(crate) fn token(value: &str, name: &str) -> Result<(), String> {
    if value.is_empty() || value.len() > 512 || value.trim() != value || value.contains('\0') {
        return Err(format!("{name} is invalid"));
    }
    Ok(())
}

pub(crate) fn digest(value: &str) -> Result<(), String> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err("digest must be 64 lowercase hexadecimal characters".into());
    }
    Ok(())
}
