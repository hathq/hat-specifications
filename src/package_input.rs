use crate::HatPackage;
use crate::security::inspect_json;

/// Parses a bounded, closed and credential-free HAT package.
///
/// # Errors
///
/// Returns an error for malformed JSON, unknown fields, excessive nesting or
/// credential-shaped content.
pub fn parse_package(source: &str) -> Result<HatPackage, String> {
    let value = inspect_json(source)?;
    serde_json::from_value(value).map_err(|error| error.to_string())
}
