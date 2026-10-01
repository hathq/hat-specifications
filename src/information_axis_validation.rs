// Added by the HAT Specifications project, 2026.
// Purpose: validate independent information coordinates without display inference.

use crate::model::validation;
use crate::tokens::{lower_hex_32, schema_uri, stable_token};
use crate::{INFORMATION_COORDINATE_SCHEMA, InformationCoordinate, Validation};

#[must_use]
pub fn validate_information_coordinate(coordinate: &InformationCoordinate) -> Validation {
    let mut findings = Vec::new();
    if coordinate.schema != INFORMATION_COORDINATE_SCHEMA
        || !stable_token(&coordinate.vocabulary_owner_id)
        || !lower_hex_32(&coordinate.catalog_digest_sha256)
        || !schema_uri(&coordinate.term_id)
        || !schema_uri(&coordinate.data_domain_term_id)
        || !coordinate
            .data_domain_term_id
            .starts_with("hathq://vocabulary/data-domain/")
    {
        findings.push("information coordinate is invalid".to_owned());
    }
    validation(findings)
}
