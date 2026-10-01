use std::collections::BTreeSet;

use crate::tokens::{classification_rank, schema_uri, stable_token};

pub(crate) fn hat_id(value: &str) -> bool {
    value.strip_prefix("hat/").is_some_and(stable_token)
}

pub(crate) fn reference(value: &str, prefix: &str) -> bool {
    value.len() <= 256
        && value.strip_prefix(prefix).is_some_and(|suffix| {
            !suffix.is_empty()
                && suffix.bytes().all(|byte| {
                    byte.is_ascii_lowercase()
                        || byte.is_ascii_digit()
                        || matches!(byte, b'-' | b'/')
                })
        })
}

pub(crate) fn unique_tokens(values: &[String], max: usize, findings: &mut Vec<String>) {
    let unique = values.iter().collect::<BTreeSet<_>>();
    if values.is_empty()
        || values.len() > max
        || unique.len() != values.len()
        || values.iter().any(|item| !stable_token(item))
    {
        findings.push("execution location token set is invalid".into());
    }
}

pub(crate) fn unique_schemas(values: &[String], max: usize, findings: &mut Vec<String>) {
    let unique = values.iter().collect::<BTreeSet<_>>();
    if values.is_empty()
        || values.len() > max
        || unique.len() != values.len()
        || values.iter().any(|item| !schema_uri(item))
    {
        findings.push("execution location operation set is invalid".into());
    }
}

pub(crate) fn classifications(values: &[String], findings: &mut Vec<String>) {
    let unique = values.iter().collect::<BTreeSet<_>>();
    if values.is_empty()
        || values.len() > 4
        || unique.len() != values.len()
        || values
            .iter()
            .any(|item| classification_rank(item).is_none())
    {
        findings.push("execution classification set is invalid".into());
    }
}
