//! Gradient-related output serialization structures extracted from `output_formats.rs`
//!
//! This module isolates gradient-specific data models to reduce the size of the
//! monolithic `output_formats.rs` while preserving backward compatibility via
//! re-exports in that original module. All structs remain pure data containers
//! with `serde` derives only.

use serde::Serialize;
use crate::output_formats::{
    ColorCollectionMatches, ColorNameInfo, NestedColorInfo, ProgramMetadata, ColorInfo,
};

/// Gradient configuration section
#[derive(Debug, Clone, Serialize)]
pub struct GradientConfiguration {
    pub start_color: String,
    pub end_color: String,
    pub start_position: u8,
    pub end_position: u8,
    pub ease_in: f64,
    pub ease_out: f64,
    pub gradient_steps: usize,
}

/// Start and end color information
#[derive(Debug, Clone, Serialize)]
pub struct GradientColors {
    pub start: ColorInfo,
    pub end: ColorInfo,
}

/// Individual gradient stop (legacy format)
#[derive(Debug, Clone, Serialize)]
pub struct GradientStop {
    pub position: u32, // Changed to integer for cleaner display
    #[serde(skip_serializing_if = "String::is_empty")] pub hex: String,
    #[serde(skip_serializing_if = "String::is_empty")] pub rgb: String,
    #[serde(skip_serializing_if = "String::is_empty")] pub lab: String,
    #[serde(skip_serializing_if = "String::is_empty")] pub lch: String,
    #[serde(serialize_with = "crate::precision_utils::PrecisionUtils::serialize_wcag_luminance")]
    pub wcag21_relative_luminance: f64,
    pub distance: f32, // Color distance from start_color using Delta E 2000
    pub color_name: Option<ColorNameInfo>,
}

/// Enhanced gradient stop with nested color structure
#[derive(Debug, Clone, Serialize)]
pub struct EnhancedGradientStop {
    pub position: u32,               // Integer position without decimals
    pub color: NestedColorInfo, // Simplified color info for nesting
    pub collections: ColorCollectionMatches,
}

/// Complete gradient analysis result that can be serialized to TOML/YAML
#[derive(Debug, Clone, Serialize)]
pub struct GradientAnalysisOutput {
    /// Program metadata
    pub metadata: ProgramMetadata,
    /// Gradient configuration
    pub configuration: GradientConfiguration,
    /// Start and end color information
    pub colors: GradientColors,
    /// Gradient steps/stops
    pub gradient_stops: Vec<GradientStop>,
}

/// Enhanced gradient analysis output with nested color structure
#[derive(Debug, Clone, Serialize)]
pub struct EnhancedGradientAnalysisOutput {
    /// Program metadata
    pub metadata: ProgramMetadata,
    /// Gradient configuration
    pub configuration: GradientConfiguration,
    /// Start and end color information
    pub colors: GradientColors,
    /// Enhanced gradient steps/stops with nested structure
    pub gradient_stops: Vec<EnhancedGradientStop>,
}

impl GradientAnalysisOutput {
    /// Serialize to TOML format
    ///
    /// # Errors
    /// Returns `toml::ser::Error` if TOML serialization fails
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> { crate::serialization::to_toml(self) }

    /// Serialize to YAML format
    ///
    /// # Errors
    /// Returns `serde_yml::Error` if YAML serialization fails
    pub fn to_yaml(&self) -> Result<String, serde_yml::Error> { crate::serialization::to_yaml(self) }
}

impl EnhancedGradientAnalysisOutput {
    /// Serialize to TOML format
    ///
    /// # Errors
    /// Returns `toml::ser::Error` if TOML serialization fails
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> { crate::serialization::to_toml(self) }

    /// Serialize to YAML format
    ///
    /// # Errors
    /// Returns `serde_yml::Error` if YAML serialization fails
    pub fn to_yaml(&self) -> Result<String, serde_yml::Error> { crate::serialization::to_yaml(self) }
}
