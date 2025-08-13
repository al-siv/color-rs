//! Centralized serialization helpers for TOML and YAML output.
//! Functions kept pure and small to reduce duplication across output model types.

/// Serialize any serde-serializable value to pretty TOML.
///
/// # Errors
/// Propagates `toml::ser::Error` from `toml::to_string_pretty`.
pub fn to_toml<T: serde::Serialize>(value: &T) -> Result<String, toml::ser::Error> {
    toml::to_string_pretty(value)
}

/// Serialize any serde-serializable value to YAML.
///
/// # Errors
/// Propagates `serde_yml::Error` from `serde_yml::to_string`.
pub fn to_yaml<T: serde::Serialize>(value: &T) -> Result<String, serde_yml::Error> {
    serde_yml::to_string(value)
}
