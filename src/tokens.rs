use std::collections::BTreeSet;

pub(crate) fn namespaced_id(value: &str, prefix: &str, label: &str, findings: &mut Vec<String>) {
    if value
        .strip_prefix(prefix)
        .is_none_or(|suffix| !stable_token(suffix))
    {
        findings.push(format!("{label} is not a stable namespaced id"));
    }
}

pub(crate) fn unique_tokens(
    values: &[String],
    label: &str,
    max: usize,
    findings: &mut Vec<String>,
) {
    let unique: BTreeSet<_> = values.iter().filter(|value| stable_token(value)).collect();
    if values.is_empty() || values.len() > max || unique.len() != values.len() {
        findings.push(format!(
            "{label} must contain 1..={max} unique stable tokens"
        ));
    }
}

pub(crate) fn stable_token(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 96
        && value
            .bytes()
            .next()
            .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        && value
            .bytes()
            .last()
            .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

pub(crate) fn semantic_version(value: &str) -> bool {
    let parts = value.split('.').collect::<Vec<_>>();
    parts.len() == 3
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()))
}

pub(crate) fn lower_hex_32(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

pub(crate) fn schema_uri(value: &str) -> bool {
    let Some(path) = value.strip_prefix("hathq://") else {
        return false;
    };
    let Some((body, version)) = path.rsplit_once("/v") else {
        return false;
    };
    !body.is_empty()
        && body.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'/' | b'.')
        })
        && !version.is_empty()
        && version.bytes().all(|byte| byte.is_ascii_digit())
        && version != "0"
}

pub(crate) fn classification_rank(value: &str) -> Option<u8> {
    match value {
        "public" => Some(0),
        "internal" => Some(1),
        "internal-confidential" => Some(2),
        "restricted-sensitive" => Some(3),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{namespaced_id, stable_token};

    #[test]
    fn stable_tokens_are_bounded_and_namespaces_do_not_nest() {
        assert!(stable_token("source-curator"));
        assert!(!stable_token("source/curator"));
        assert!(!stable_token(&"x".repeat(97)));
        let mut findings = Vec::new();
        namespaced_id("hat/source/curator", "hat/", "hat id", &mut findings);
        assert!(!findings.is_empty());
    }
}
