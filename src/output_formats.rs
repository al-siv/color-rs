//! Output format serialization for TOML and YAML files
//!
//! This module implements serializable data structures for exporting color analysis
//! results to TOML and YAML formats using the builder pattern for different output types.

use serde::{Deserialize, Serialize};

/// Enhanced color name information with multi-collection support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorNameInfo {
    pub exact: Option<String>,
    pub nearest: Option<NearestColorMatch>,
    pub all_collections: Option<ColorNameAllCollections>,
}

/// Color name matches from all collections
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ColorNameAllCollections {
    pub css: Option<CollectionColorMatch>,
    pub ral_classic: Option<CollectionColorMatch>,
    pub ral_design: Option<CollectionColorMatch>,
}

/// Color match from a specific collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionColorMatch {
    pub exact: Option<String>,
    pub nearest: Option<NearestColorMatch>,
}

/// Nearest color match information with distance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearestColorMatch {
    pub name: String,
    pub collection: String,
    pub distance: f64,
}

/// Complete color analysis result that can be serialized to TOML/YAML
#[derive(Debug, Clone, Serialize)]
pub struct ColorAnalysisOutput {
    /// Program metadata
    pub metadata: ProgramMetadata,
    /// Input information
    pub input: InputInfo,
    /// Color format conversions
    pub conversion: ColorFormats,
    /// Contrast and luminance information
    pub contrast: ContrastData,
    /// Grayscale variations
    pub grayscale: GrayscaleData,
    /// Color collection matches
    pub color_collections: ColorCollections,
    /// Color schemes
    pub color_schemes: ColorSchemes,
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

/// Individual color information
#[derive(Debug, Clone, Serialize)]
pub struct ColorInfo {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub hex: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub rgb: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub lab: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub lch: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contrast: Option<ContrastAnalysis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<ColorCollectionMatches>,
}

/// Contrast analysis between two colors
#[derive(Debug, Clone, Serialize)]
pub struct ContrastAnalysis {
    pub distance: f64,
    pub wcag21_relative_luminance: f64,
    pub relative_contrast: f32,
}

/// Color collection matches for gradient stops
#[derive(Debug, Clone, Serialize)]
pub struct ColorCollectionMatches {
    pub css: String,
    pub css_distance: f64,
    pub ralc: String,
    pub ralc_distance: f64,
    pub raldsp: String,
    pub raldsp_distance: f64,
}

/// Enhanced gradient stop with nested color structure
#[derive(Debug, Clone, Serialize)]
pub struct EnhancedGradientStop {
    pub position: u32,          // Integer position without decimals
    pub color: NestedColorInfo, // Simplified color info for nesting
    pub collections: ColorCollectionMatches,
}

/// Simplified color information for nested structures (no contrast/collections)
#[derive(Debug, Clone, Serialize)]
pub struct NestedColorInfo {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub hex: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub rgb: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub lab: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub lch: String,
    pub wcag21_relative_luminance: f64,
}

/// Individual gradient stop (legacy format)
#[derive(Debug, Clone, Serialize)]
pub struct GradientStop {
    pub position: u32, // Changed to integer for cleaner display
    #[serde(skip_serializing_if = "String::is_empty")]
    pub hex: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub rgb: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub lab: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub lch: String,
    pub wcag21_relative_luminance: f64,
    pub color_name: Option<ColorNameInfo>,
}

/// Program metadata section
#[derive(Debug, Clone, Serialize)]
pub struct ProgramMetadata {
    pub program_name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub generated_at: String,
    pub distance_strategy: String,
}

/// Input color information
#[derive(Debug, Clone, Serialize, Default)]
pub struct InputInfo {
    pub input_color: String,
    pub base_color: String,
}

/// All color format conversions
#[derive(Debug, Clone, Serialize, Default)]
pub struct ColorFormats {
    pub hex: String,
    pub rgb: String,
    pub hsl: String,
    pub hsb: String,
    pub lab: String,
    pub lch: String,
    pub cmyk: String,
    pub xyz: String,
    pub oklch: String,
}

/// Contrast and luminance information
#[derive(Debug, Clone, Serialize)]
pub struct ContrastData {
    pub wcag21_relative_luminance: f64,
    pub contrast_vs_white: ContrastInfo,
    pub contrast_vs_black: ContrastInfo,
    pub brightness: BrightnessInfo,
}

/// Grayscale variations
#[derive(Debug, Clone, Serialize, Default)]
pub struct GrayscaleData {
    pub lch0_hex: String,
    pub lch0: String,
    pub lch2_hex: String,
    pub lch2: String,
    pub lch4_hex: String,
    pub lch4: String,
    pub lch6_hex: String,
    pub lch6: String,
}

/// Contrast information
#[derive(Debug, Clone, Serialize)]
pub struct ContrastInfo {
    pub ratio: f64,
    pub assessment: String,
}

/// Brightness assessment
#[derive(Debug, Clone, Serialize, Default)]
pub struct BrightnessInfo {
    pub lab_assessment: String,
    pub wcag_assessment: String,
}

/// Color collection matches
#[derive(Debug, Clone, Serialize, Default)]
pub struct ColorCollections {
    pub css_colors: Vec<ColorMatch>,
    pub ral_classic: Vec<ColorMatch>,
    pub ral_design: Vec<ColorMatch>,
}

/// Individual color match
#[derive(Debug, Clone, Serialize, Default)]
pub struct ColorMatch {
    pub name: String,
    pub hex: String,
    pub lch: String,
    pub code: Option<String>,
    pub distance: f64,
    pub wcag21_relative_luminance: f64,
}

/// Color schemes with configurable strategy
#[derive(Debug, Clone, Serialize, Default)]
pub struct ColorSchemes {
    pub complementary: EnhancedColorSchemeItem,
    pub split_complementary: Vec<EnhancedColorSchemeItem>,
    pub triadic: Vec<EnhancedColorSchemeItem>,
    pub tetradic: Vec<EnhancedColorSchemeItem>,
}

/// Enhanced color scheme item with direct collection matches
#[derive(Debug, Clone, Serialize, Default)]
pub struct EnhancedColorSchemeItem {
    pub hex: String,
    pub hsl: String,
    pub lch: String,
    pub css: Option<CollectionMatch>,
    pub ral_classic: Option<CollectionMatch>,
    pub ral_design: Option<CollectionMatch>,
}

/// Simplified collection match with essential data
#[derive(Debug, Clone, Serialize, Default)]
pub struct CollectionMatch {
    pub name: String,
    pub hex: String,
    pub distance: f64,
    pub wcag_relative_luminance: f64,
}

/// Individual color scheme item with format conversions and color name matching (deprecated)
#[derive(Debug, Clone, Serialize, Default)]
pub struct ColorSchemeItem {
    pub hex: String,
    pub hsl: String,
    pub lch: String,
    pub color_name: Option<ColorNameInfo>,
}

/// Set of color schemes for a strategy (deprecated)
#[derive(Debug, Clone, Serialize, Default)]
pub struct ColorSchemeSet {
    pub complementary: ColorSchemeItem,
    pub split_complementary: Vec<ColorSchemeItem>,
    pub triadic: Vec<ColorSchemeItem>,
    pub tetradic: Vec<ColorSchemeItem>,
}

impl Default for ColorAnalysisOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorAnalysisOutput {
    /// Create a new empty color analysis output
    #[must_use] pub fn new() -> Self {
        Self {
            metadata: ProgramMetadata::new(None),
            input: InputInfo::default(),
            conversion: ColorFormats::default(),
            contrast: ContrastData::default(),
            grayscale: GrayscaleData::default(),
            color_collections: ColorCollections::default(),
            color_schemes: ColorSchemes::default(),
        }
    }

    /// Set input information
    #[must_use] pub fn with_input(mut self, input_color: String, base_color: String) -> Self {
        self.input = InputInfo {
            input_color,
            base_color,
        };
        self
    }

    /// Set color formats
    #[must_use] pub fn with_conversion(mut self, conversion: ColorFormats) -> Self {
        self.conversion = conversion;
        self
    }

    /// Set contrast information
    #[must_use] pub fn with_contrast(mut self, contrast: ContrastData) -> Self {
        self.contrast = contrast;
        self
    }

    /// Set grayscale information
    #[must_use] pub fn with_grayscale(mut self, grayscale: GrayscaleData) -> Self {
        self.grayscale = grayscale;
        self
    }

    /// Set color collections
    #[must_use] pub fn with_color_collections(mut self, color_collections: ColorCollections) -> Self {
        self.color_collections = color_collections;
        self
    }

    /// Set color schemes
    #[must_use] pub fn with_color_schemes(mut self, color_schemes: ColorSchemes) -> Self {
        self.color_schemes = color_schemes;
        self
    }

    /// Serialize to TOML format
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    /// Serialize to YAML format
    pub fn to_yaml(&self) -> Result<String, serde_yml::Error> {
        serde_yml::to_string(self)
    }
}

impl GradientAnalysisOutput {
    /// Serialize to TOML format
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    /// Serialize to YAML format
    pub fn to_yaml(&self) -> Result<String, serde_yml::Error> {
        serde_yml::to_string(self)
    }
}

impl EnhancedGradientAnalysisOutput {
    /// Serialize to TOML format
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    /// Serialize to YAML format
    pub fn to_yaml(&self) -> Result<String, serde_yml::Error> {
        serde_yml::to_string(self)
    }
}

impl ProgramMetadata {
    #[must_use] pub fn new(distance_strategy: Option<&str>) -> Self {
        use chrono::Utc;

        Self {
            program_name: crate::config::APP_NAME.to_string(),
            version: crate::config::APP_VERSION.to_string(),
            author: crate::config::APP_AUTHOR.to_string(),
            description: crate::config::APP_DESCRIPTION.to_string(),
            generated_at: Utc::now().to_rfc3339(),
            distance_strategy: distance_strategy.unwrap_or("LAB Delta E").to_string(),
        }
    }
}

impl Default for ContrastData {
    fn default() -> Self {
        Self {
            wcag21_relative_luminance: 0.0,
            contrast_vs_white: ContrastInfo::default(),
            contrast_vs_black: ContrastInfo::default(),
            brightness: BrightnessInfo::default(),
        }
    }
}

impl Default for ContrastInfo {
    fn default() -> Self {
        Self {
            ratio: 0.0,
            assessment: String::new(),
        }
    }
}
