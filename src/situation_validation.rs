use crate::tokens::{lower_hex_32, schema_uri, stable_token};
use crate::{SITUATION_CONTRIBUTION_SCHEMA, SituationContribution, Validation};

#[must_use]
pub fn validate_situation_contribution(value: &SituationContribution) -> Validation {
    let mut findings = Vec::new();
    if value.schema != SITUATION_CONTRIBUTION_SCHEMA
        || !stable_token(&value.contribution_id)
        || !schema_uri(&value.category_term_id)
        || !stable_token(&value.detail_handle)
    {
        findings.push("situation identity is invalid".to_owned());
    }
    if !bounded_text(&value.title, 200)
        || value
            .object_label
            .as_ref()
            .is_some_and(|item| !bounded_text(item, 160))
        || value.actors.len() > 16
        || value.actors.iter().any(|item| !bounded_text(item, 120))
    {
        findings.push("situation content is invalid".to_owned());
    }
    if value.time.as_ref().is_some_and(|time| {
        !bounded_text(&time.time_zone, 80)
            || time
                .end_at_unix_ms
                .is_some_and(|end| end < time.start_at_unix_ms)
    }) {
        findings.push("situation time is invalid".to_owned());
    }
    if value.place.as_ref().is_some_and(|place| {
        !bounded_text(&place.label, 160)
            || place.coordinates.as_ref().is_some_and(|coordinates| {
                !coordinates.latitude.is_finite()
                    || !(-90.0..=90.0).contains(&coordinates.latitude)
                    || !coordinates.longitude.is_finite()
                    || !(-180.0..=180.0).contains(&coordinates.longitude)
            })
    }) {
        findings.push("situation place is invalid".to_owned());
    }
    if !stable_token(&value.source.repository_id)
        || !lower_hex_32(&value.source.digest_sha256)
        || value.source.revision == 0
    {
        findings.push("situation source is invalid".to_owned());
    }
    crate::model::validation(findings)
}

fn bounded_text(value: &str, maximum: usize) -> bool {
    !value.trim().is_empty() && value.len() <= maximum
}
