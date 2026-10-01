// Added by the HAT Specifications project, 2026.
// Purpose: reject incomplete clauses instead of inferring missing role bindings.

use crate::semantic_validation::{reference, scalar, token};
use crate::{SEMANTIC_CLAUSE_SCHEMA, SemanticClause, SemanticClauseKind, SemanticTermKind};
use crate::{SemanticClauseRole, validate_semantic_clause_frame, validate_semantic_term_reference};
use std::collections::{BTreeMap, BTreeSet};

/// Validates one exact fact or action clause without completing missing roles.
///
/// # Errors
///
/// Returns an error when the schema, frame, role set or typed role bindings are
/// incomplete or inconsistent.
pub fn validate_semantic_clause(value: &SemanticClause) -> Result<(), String> {
    if value.schema != SEMANTIC_CLAUSE_SCHEMA {
        return Err("semantic clause schema is invalid".into());
    }
    token(&value.clause_id, "semantic clause id")?;
    validate_semantic_clause_frame(&value.frame)?;
    let predicate = value.frame.predicate.kind;
    if matches!(value.kind, SemanticClauseKind::Action)
        != matches!(predicate, SemanticTermKind::Action)
    {
        return Err("semantic clause kind differs from its predicate".into());
    }
    if value.bindings.is_empty() || value.bindings.len() > 32 {
        return Err("semantic clause binding count is invalid".into());
    }
    let specs = value
        .frame
        .roles
        .iter()
        .map(|spec| (spec.role, spec))
        .collect::<BTreeMap<_, _>>();
    let mut seen = BTreeSet::new();
    for binding in &value.bindings {
        if !seen.insert(binding.role) {
            return Err("semantic clause role is duplicated".into());
        }
        let spec = specs
            .get(&binding.role)
            .ok_or_else(|| "semantic clause role is undeclared".to_owned())?;
        validate_binding(binding)?;
        if binding
            .term
            .as_ref()
            .is_some_and(|term| !spec.accepted_kinds.contains(&term.kind))
        {
            return Err("semantic clause term kind is not accepted by its role".into());
        }
    }
    if value
        .frame
        .roles
        .iter()
        .any(|spec| spec.required && !seen.contains(&spec.role))
    {
        return Err("semantic clause has an unresolved required role".into());
    }
    if matches!(value.kind, SemanticClauseKind::Action)
        && !seen.contains(&SemanticClauseRole::Actor)
    {
        return Err("semantic action clause requires an actor".into());
    }
    Ok(())
}

fn validate_binding(value: &crate::SemanticRoleBinding) -> Result<(), String> {
    let count = usize::from(value.term.is_some())
        + usize::from(value.reference.is_some())
        + usize::from(value.value.is_some());
    if count != 1 {
        return Err("semantic clause binding must contain exactly one value".into());
    }
    if let Some(term) = &value.term {
        validate_semantic_term_reference(term)?;
    }
    if let Some(reference_value) = &value.reference {
        reference(reference_value)?;
    }
    if let Some(value) = &value.value {
        scalar(value)?;
    }
    Ok(())
}
