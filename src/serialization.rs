//! Centralized serialization helpers and trait to decouple data models from format-specific logic.

use serde::Serialize;

pub trait StructuredSerialize: Serialize {
    fn as_toml_pretty(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }
    fn as_yaml(&self) -> Result<String, serde_yml::Error> {
        serde_yml::to_string(self)
    }
}

// Blanket impl for all Serialize types; keeps possibility to add specialization later if needed.
impl<T: Serialize> StructuredSerialize for T {}
