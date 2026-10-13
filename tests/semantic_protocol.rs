#[path = "semantic_protocol/support.rs"]
mod support;

use hat_specifications::*;
use support::clause;

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
