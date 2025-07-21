//! Output format serialization for TOML and YAML files
//!
//! This module implements serializable data structures for exporting color analysis
//! results to TOML and YAML formats using the Strategy pattern for different output types.

use serde::Serialize;
use std::collections::HashMap;

/// Complete color analysis result that can be serialized to TOML/YAML
#[derive(Debug, Serialize)]
pub struct ColorAnalysisOutput {
    /// Program metadata
    pub metadata: ProgramMetadata,
    /// Input information
    pub input: InputInfo,
    /// Color format conversions
    pub formats: ColorFormats,
    /// Additional color information
    pub additional_info: AdditionalInfo,
    /// Color collection matches
    pub color_collections: ColorCollections,
    /// Color schemes
    pub color_schemes: ColorSchemes,
}

/// Program metadata section
#[derive(Debug, Serialize)]
pub struct ProgramMetadata {
    pub program_name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub generated_at: String,
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

/// Additional color information
#[derive(Debug, Serialize)]
pub struct AdditionalInfo {
    pub grayscale_lab: String,
    pub grayscale_lch_0: String,
    pub grayscale_lch_2: String,
    pub grayscale_lch_4: String,
    pub grayscale_lch_6: String,
    pub relative_luminance: f64,
    pub contrast_vs_white: ContrastInfo,
    pub contrast_vs_black: ContrastInfo,
    pub brightness: BrightnessInfo,
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
#[derive(Debug, Serialize)]
pub struct ColorMatch {
    pub name: String,
    pub hex: String,
    pub code: Option<String>,
    pub delta_e_distance: f64,
}

/// Color schemes
#[derive(Debug, Serialize)]
pub struct ColorSchemes {
    pub hsl_strategy: ColorSchemeSet,
    pub lab_strategy: ColorSchemeSet,
}

/// Set of color schemes for a strategy
#[derive(Debug, Serialize)]
pub struct ColorSchemeSet {
    pub complementary: String,
    pub split_complementary: Vec<String>,
    pub triadic: Vec<String>,
}

impl ColorAnalysisOutput {
    /// Create a new empty color analysis output
    pub fn new() -> Self {
        Self {
            metadata: ProgramMetadata::new(),
            input: InputInfo::default(),
            formats: ColorFormats::default(),
            additional_info: AdditionalInfo::default(),
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
    pub fn with_formats(mut self, formats: ColorFormats) -> Self {
        self.formats = formats;
        self
    }

    /// Set additional information
    pub fn with_additional_info(mut self, additional_info: AdditionalInfo) -> Self {
        self.additional_info = additional_info;
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
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
}

impl ProgramMetadata {
    fn new() -> Self {
        use chrono::Utc;
        
        Self {
            program_name: crate::config::APP_NAME.to_string(),
            version: crate::config::APP_VERSION.to_string(),
            author: crate::config::APP_AUTHOR.to_string(),
            description: crate::config::APP_ABOUT.to_string(),
            generated_at: Utc::now().to_rfc3339(),
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

impl Default for AdditionalInfo {
    fn default() -> Self {
        Self {
            grayscale_lab: String::new(),
            grayscale_lch_0: String::new(),
            grayscale_lch_2: String::new(),
            grayscale_lch_4: String::new(),
            grayscale_lch_6: String::new(),
            relative_luminance: 0.0,
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

impl Default for ColorSchemeSet {
    fn default() -> Self {
        Self {
            complementary: String::new(),
            split_complementary: Vec::new(),
            triadic: Vec::new(),
        }
    }
}
