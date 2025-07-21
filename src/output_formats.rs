//! Output format serialization for TOML and YAML files
//!
//! This module implements serializable data structures for exporting color analysis
//! results to TOML and YAML formats using the Strategy pattern for different output types.

use serde::Serialize;

/// Complete color analysis result that can be serialized to TOML/YAML
#[derive(Debug, Serialize)]
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
#[derive(Debug, Serialize)]
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

/// Gradient configuration section
#[derive(Debug, Serialize)]
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
#[derive(Debug, Serialize)]
pub struct GradientColors {
    pub start: ColorInfo,
    pub end: ColorInfo,
}

/// Individual color information
#[derive(Debug, Serialize)]
pub struct ColorInfo {
    pub hex: String,
    pub rgb: String,
    pub lab: String,
    pub lch: String,
    pub wcag21_relative_luminance: f64,
}

/// Individual gradient stop
#[derive(Debug, Serialize)]
pub struct GradientStop {
    pub position: f64,
    pub hex: String,
    pub rgb: String,
    pub lab: String,
    pub lch: String,
    pub wcag21_relative_luminance: f64,
    pub color_name: Option<ColorNameInfo>,
}

/// Program metadata section
#[derive(Debug, Serialize)]
pub struct ProgramMetadata {
    pub program_name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub generated_at: String,
    pub distance_strategy: String,
}

/// Input color information
#[derive(Debug, Serialize)]
pub struct InputInfo {
    pub input_color: String,
    pub base_color: String,
}

/// All color format conversions
#[derive(Debug, Serialize)]
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
#[derive(Debug, Serialize)]
pub struct ContrastData {
    pub wcag21_relative_luminance: f64,
    pub contrast_vs_white: ContrastInfo,
    pub contrast_vs_black: ContrastInfo,
    pub brightness: BrightnessInfo,
}

/// Grayscale variations
#[derive(Debug, Serialize)]
pub struct GrayscaleData {
    pub grayscale_lab: String,
    pub grayscale_lch_0: String,
    pub grayscale_lch_2: String,
    pub grayscale_lch_4: String,
    pub grayscale_lch_6: String,
}

/// Contrast information
#[derive(Debug, Serialize)]
pub struct ContrastInfo {
    pub ratio: f64,
    pub assessment: String,
}

/// Brightness assessment
#[derive(Debug, Serialize)]
pub struct BrightnessInfo {
    pub lab_assessment: String,
    pub wcag_assessment: String,
}

/// Color collection matches
#[derive(Debug, Serialize)]
pub struct ColorCollections {
    pub css_colors: Vec<ColorMatch>,
    pub ral_classic: Vec<ColorMatch>,
    pub ral_design: Vec<ColorMatch>,
}

/// Individual color match
#[derive(Debug, Serialize, Default)]
pub struct ColorMatch {
    pub name: String,
    pub hex: String,
    pub lch: String,
    pub code: Option<String>,
    pub delta_e_distance: f64,
    pub wcag21_relative_luminance: f64,
}

/// Color schemes
#[derive(Debug, Serialize)]
pub struct ColorSchemes {
    pub hsl_strategy: ColorSchemeSet,
    pub lab_strategy: ColorSchemeSet,
}

/// Individual color scheme item with format conversions and color name matching
#[derive(Debug, Serialize)]
pub struct ColorSchemeItem {
    pub hex: String,
    pub hsl: String,
    pub lch: String,
    pub color_name: Option<ColorNameInfo>,
}

/// Color name information with exact and nearest matches
#[derive(Debug, Serialize)]
pub struct ColorNameInfo {
    pub exact: Option<String>,
    pub nearest: Option<NearestColorMatch>,
}

/// Nearest color match information with distance
#[derive(Debug, Serialize)]
pub struct NearestColorMatch {
    pub name: String,
    pub collection: String,
    pub distance: f64,
}

/// Set of color schemes for a strategy
#[derive(Debug, Serialize)]
pub struct ColorSchemeSet {
    pub complementary: ColorSchemeItem,
    pub split_complementary: Vec<ColorSchemeItem>,
    pub triadic: Vec<ColorSchemeItem>,
    pub tetradic: Vec<ColorSchemeItem>,
}

impl ColorAnalysisOutput {
    /// Create a new empty color analysis output
    pub fn new() -> Self {
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
    pub fn with_input(mut self, input_color: String, base_color: String) -> Self {
        self.input = InputInfo {
            input_color,
            base_color,
        };
        self
    }

    /// Set color formats
    pub fn with_conversion(mut self, conversion: ColorFormats) -> Self {
        self.conversion = conversion;
        self
    }

    /// Set contrast information
    pub fn with_contrast(mut self, contrast: ContrastData) -> Self {
        self.contrast = contrast;
        self
    }

    /// Set grayscale information
    pub fn with_grayscale(mut self, grayscale: GrayscaleData) -> Self {
        self.grayscale = grayscale;
        self
    }

    /// Set color collections
    pub fn with_color_collections(mut self, color_collections: ColorCollections) -> Self {
        self.color_collections = color_collections;
        self
    }

    /// Set color schemes
    pub fn with_color_schemes(mut self, color_schemes: ColorSchemes) -> Self {
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

impl ProgramMetadata {
    pub fn new(distance_strategy: Option<&str>) -> Self {
        use chrono::Utc;

        Self {
            program_name: crate::config::APP_NAME.to_string(),
            version: crate::config::APP_VERSION.to_string(),
            author: crate::config::APP_AUTHOR.to_string(),
            description: crate::config::APP_ABOUT.to_string(),
            generated_at: Utc::now().to_rfc3339(),
            distance_strategy: distance_strategy.unwrap_or("LAB Delta E").to_string(),
        }
    }
}

impl Default for InputInfo {
    fn default() -> Self {
        Self {
            input_color: String::new(),
            base_color: String::new(),
        }
    }
}

impl Default for ColorFormats {
    fn default() -> Self {
        Self {
            hex: String::new(),
            rgb: String::new(),
            hsl: String::new(),
            hsb: String::new(),
            lab: String::new(),
            lch: String::new(),
            cmyk: String::new(),
            xyz: String::new(),
            oklch: String::new(),
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

impl Default for GrayscaleData {
    fn default() -> Self {
        Self {
            grayscale_lab: String::new(),
            grayscale_lch_0: String::new(),
            grayscale_lch_2: String::new(),
            grayscale_lch_4: String::new(),
            grayscale_lch_6: String::new(),
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

impl Default for BrightnessInfo {
    fn default() -> Self {
        Self {
            lab_assessment: String::new(),
            wcag_assessment: String::new(),
        }
    }
}

impl Default for ColorCollections {
    fn default() -> Self {
        Self {
            css_colors: Vec::new(),
            ral_classic: Vec::new(),
            ral_design: Vec::new(),
        }
    }
}

impl Default for ColorSchemes {
    fn default() -> Self {
        Self {
            hsl_strategy: ColorSchemeSet::default(),
            lab_strategy: ColorSchemeSet::default(),
        }
    }
}

impl Default for ColorSchemeItem {
    fn default() -> Self {
        Self {
            hex: String::new(),
            hsl: String::new(),
            lch: String::new(),
            color_name: None,
        }
    }
}

impl Default for ColorSchemeSet {
    fn default() -> Self {
        Self {
            complementary: ColorSchemeItem::default(),
            split_complementary: Vec::new(),
            triadic: Vec::new(),
            tetradic: Vec::new(),
        }
    }
}
