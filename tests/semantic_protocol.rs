#[path = "semantic_protocol/support.rs"]
mod support;

use hat_specifications::*;
use std::collections::BTreeSet;
use support::{clause, exchange, term};

#[test]
fn foundation_catalog_is_digest_bound_bilingual_and_domain_free() {
    let value = foundation_semantic_catalog();
    assert_eq!(validate_semantic_catalog(&value), Ok(()));
    assert!(value.foundation);
    assert!(value.terms.iter().all(|term| {
        term.reference.term_id.starts_with("core.") || term.reference.term_id.starts_with("world.")
    }));
    assert!(value.lexicalizations.iter().any(|item| {
        item.term.term_id == "world.person.identity"
            && item.locale == "ja"
            && item.preferred == "本人性"
    }));
    assert!(value.lexicalizations.iter().any(|item| {
        item.term.term_id == "world.person.identity"
            && item.locale == "en"
            && item.preferred == "Personal identity"
    }));
}

#[test]
fn missing_application_role_fails_closed() {
    let mut value = clause();
    value
        .bindings
        .retain(|binding| binding.role != SemanticClauseRole::Actor);
    assert!(validate_semantic_clause(&value).is_err());
}

#[test]
fn semantic_exchange_negotiates_exact_term_definitions_without_sync() {
    let value = exchange();
    assert_eq!(validate_semantic_exchange(&value), Ok(()));
    let exact = classify_semantic_exchange(&value, &value.terms, &BTreeSet::new()).expect("exact");
    assert!(exact.accepted);
    assert_eq!(exact.terms[0].state, SemanticTermComprehension::Understood);

    let absent = classify_semantic_exchange(&value, &[], &BTreeSet::new()).expect("absent");
    assert_eq!(
        absent.terms[0].state,
        SemanticTermComprehension::HatRequired
    );
    let changed = term(
        SemanticTermKind::Action,
        "hathq://vocabulary/action/schedule/v1",
        '0',
    );
    let conflict =
        classify_semantic_exchange(&value, &[changed], &BTreeSet::new()).expect("conflict");
    assert_eq!(
        conflict.terms[0].state,
        SemanticTermComprehension::VersionConflict
    );
}

#[test]
fn denied_term_and_exchange_mutation_do_not_enter_receiver_state() {
    let value = exchange();
    let denied = BTreeSet::from([value.terms[0].term_id.clone()]);
    let report = classify_semantic_exchange(&value, &value.terms, &denied).expect("denied");
    assert!(!report.accepted);
    assert_eq!(
        report.terms[0].state,
        SemanticTermComprehension::PolicyDenied
    );
    let mut changed = value;
    changed.purpose = "different-purpose".into();
    assert!(validate_semantic_exchange(&changed).is_err());
}
