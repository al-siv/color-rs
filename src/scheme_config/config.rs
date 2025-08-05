//! Main implementation methods for ColorSchemeConfig

use super::types::{ColorSchemeConfig, ConfigError};

impl ColorSchemeConfig {
    /// Configuration combinator to add relative luminance preservation
    /// 
    /// Returns a new configuration with relative luminance preservation enabled.
    /// This is a pure function that doesn't mutate the original configuration.
    pub fn preserve_relative_luminance(self) -> std::result::Result<Self, ConfigError> {
        if self.preserve_lab_luminance {
            return Err(ConfigError::ConflictingLuminanceOptions);
        }
        Ok(Self {
            preserve_relative_luminance: true,
            ..self
        })
    }

    /// Configuration combinator to add lab luminance preservation
    pub fn preserve_lab_luminance(self) -> std::result::Result<Self, ConfigError> {
        if self.preserve_relative_luminance {
            return Err(ConfigError::ConflictingLuminanceOptions);
        }
        Ok(Self {
            preserve_lab_luminance: true,
            ..self
        })
    }

    /// Configuration combinator to set target relative luminance
    pub fn set_target_relative_luminance(self, luminance: f64) -> std::result::Result<Self, ConfigError> {
        if !(0.0..=1.0).contains(&luminance) {
            return Err(ConfigError::InvalidTargetLuminance {
                value: luminance,
                min: 0.0,
                max: 1.0,
            });
        }
        if self.target_lab_luminance.is_some() {
            return Err(ConfigError::ConflictingTargetValues);
        }
        Ok(Self {
            target_relative_luminance: Some(luminance),
            ..self
        })
    }

    /// Configuration combinator to set target lab luminance
    pub fn set_target_lab_luminance(self, luminance: f64) -> std::result::Result<Self, ConfigError> {
        if !(0.0..=100.0).contains(&luminance) {
            return Err(ConfigError::InvalidTargetLuminance {
                value: luminance,
                min: 0.0,
                max: 100.0,
            });
        }
        if self.target_relative_luminance.is_some() {
            return Err(ConfigError::ConflictingTargetValues);
        }
        Ok(Self {
            target_lab_luminance: Some(luminance),
            ..self
        })
    }
}
