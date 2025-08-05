//! Core types and errors for color scheme configuration

use palette::Lab;

/// Immutable configuration for color scheme calculations
/// 
/// This struct replaces the mutable ColorSchemeBuilder with an immutable configuration
/// that is validated at construction time using smart constructors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorSchemeConfig {
    pub preserve_relative_luminance: bool,
    pub preserve_lab_luminance: bool,
    pub target_relative_luminance: Option<f64>,
    pub target_lab_luminance: Option<f64>,
}

/// Validation errors for color scheme configuration
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigError {
    /// Both relative and lab luminance preservation cannot be enabled simultaneously
    ConflictingLuminanceOptions,
    /// Target luminance values must be within valid ranges
    InvalidTargetLuminance { value: f64, min: f64, max: f64 },
    /// Multiple target luminance values cannot be specified
    ConflictingTargetValues,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConflictingLuminanceOptions => {
                write!(f, "Cannot preserve both relative and lab luminance simultaneously")
            }
            Self::InvalidTargetLuminance { value, min, max } => {
                write!(f, "Target luminance {value} is outside valid range [{min}, {max}]")
            }
            Self::ConflictingTargetValues => {
                write!(f, "Cannot specify both relative and lab target luminance values")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

/// Calculator struct that holds an immutable configuration and provides pure functions
#[derive(Debug, Clone)]
pub struct ColorSchemeCalculator {
    pub(super) config: ColorSchemeConfig,
}

/// Configuration for luminance matching in scheme calculation
#[derive(Debug, Clone, Copy)]
pub struct LuminanceConfig {
    pub preserve_relative_luminance: bool,
    pub preserve_lab_luminance: bool,
}

impl From<ColorSchemeConfig> for LuminanceConfig {
    fn from(config: ColorSchemeConfig) -> Self {
        Self {
            preserve_relative_luminance: config.preserve_relative_luminance,
            preserve_lab_luminance: config.preserve_lab_luminance,
        }
    }
}

/// Basic color schemes calculated from a base color
#[derive(Debug, Clone)]
pub struct BasicColorSchemes {
    pub hsl_complementary: Lab,
    pub lab_complementary: Lab,
    pub analogous_warm: Lab,
    pub analogous_cool: Lab,
    pub triadic_1: Lab,
    pub triadic_2: Lab,
    pub split_complementary_1: Lab,
    pub split_complementary_2: Lab,
    pub tetradic_1: Lab,
    pub tetradic_2: Lab,
    pub tetradic_3: Lab,
}
