use crate::security::inspect;
use crate::{FittingProfile, HatManifest};

/// Parses a bounded, closed HAT manifest after credential screening.
///
/// # Errors
///
/// Returns an error for invalid TOML, unknown fields, excessive input, deep
/// nesting, credential-shaped keys, or high-confidence raw secret values.
pub fn parse_manifest(source: &str) -> Result<HatManifest, String> {
    inspect(source)?
        .try_into()
        .map_err(|error: toml::de::Error| error.to_string())
}

/// Parses a bounded, closed Fitting profile after credential screening.
///
/// # Errors
///
/// Returns an error for invalid TOML, unknown fields, excessive input, deep
/// nesting, credential-shaped keys, or high-confidence raw secret values.
pub fn parse_profile(source: &str) -> Result<FittingProfile, String> {
    inspect(source)?
        .try_into()
        .map_err(|error: toml::de::Error| error.to_string())
}
