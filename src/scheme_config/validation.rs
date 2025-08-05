//! Smart constructors and validation logic for color scheme configuration

use super::types::{ColorSchemeConfig, ConfigError};
use crate::config::display_constants;

impl ColorSchemeConfig {
    /// Default configuration with no special options
    pub const DEFAULT: Self = Self {
        preserve_relative_luminance: false,
        preserve_lab_luminance: false,
        target_relative_luminance: None,
        target_lab_luminance: None,
    };

    /// Create a validated configuration
    ///
    /// This smart constructor performs validation at compile time where possible
    /// and at runtime for dynamic values, returning a Result for error handling.
    pub fn new(
        preserve_relative_luminance: bool,
        preserve_lab_luminance: bool,
        target_relative_luminance: Option<f64>,
        target_lab_luminance: Option<f64>,
    ) -> std::result::Result<Self, ConfigError> {
        // Validate mutually exclusive options
        if preserve_relative_luminance && preserve_lab_luminance {
            return Err(ConfigError::ConflictingLuminanceOptions);
        }

        // Validate target luminance values
        if let Some(rel_lum) = target_relative_luminance {
            if !(0.0..=1.0).contains(&rel_lum) {
                return Err(ConfigError::InvalidTargetLuminance {
                    value: rel_lum,
                    min: 0.0,
                    max: 1.0,
                });
            }
        }

        if let Some(lab_lum) = target_lab_luminance {
            if !(display_constants::LAB_LUMINANCE_MIN..=display_constants::LAB_LUMINANCE_MAX)
                .contains(&lab_lum)
            {
                return Err(ConfigError::InvalidTargetLuminance {
                    value: lab_lum,
                    min: display_constants::LAB_LUMINANCE_MIN,
                    max: display_constants::LAB_LUMINANCE_MAX,
                });
            }
        }

        // Validate that only one target luminance is specified
        if target_relative_luminance.is_some() && target_lab_luminance.is_some() {
            return Err(ConfigError::ConflictingTargetValues);
        }

        Ok(Self {
            preserve_relative_luminance,
            preserve_lab_luminance,
            target_relative_luminance,
            target_lab_luminance,
        })
    }

    /// Smart constructor for relative luminance preservation
    pub fn with_relative_luminance_preservation() -> Self {
        Self {
            preserve_relative_luminance: true,
            preserve_lab_luminance: false,
            target_relative_luminance: None,
            target_lab_luminance: None,
        }
    }

    /// Smart constructor for lab luminance preservation
    pub fn with_lab_luminance_preservation() -> Self {
        Self {
            preserve_relative_luminance: false,
            preserve_lab_luminance: true,
            target_relative_luminance: None,
            target_lab_luminance: None,
        }
    }

    /// Smart constructor for target relative luminance
    pub fn with_target_relative_luminance(
        luminance: f64,
    ) -> std::result::Result<Self, ConfigError> {
        Self::new(false, false, Some(luminance), None)
    }

    /// Smart constructor for target lab luminance
    pub fn with_target_lab_luminance(luminance: f64) -> std::result::Result<Self, ConfigError> {
        Self::new(false, false, None, Some(luminance))
    }
}

impl Default for ColorSchemeConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}
