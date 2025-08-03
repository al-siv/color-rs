//! Functional color scheme configuration using immutable patterns and smart constructors
//!
//! This module provides a functional alternative to the builder pattern for color scheme configuration.
//! Instead of mutable state and method chaining, it uses immutable configuration structs, smart
//! constructors with validation, and Result types for compile-time safety.

use crate::error::Result;
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
            if !(0.0..=100.0).contains(&lab_lum) {
                return Err(ConfigError::InvalidTargetLuminance {
                    value: lab_lum,
                    min: 0.0,
                    max: 100.0,
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
    pub fn with_target_relative_luminance(luminance: f64) -> std::result::Result<Self, ConfigError> {
        Self::new(false, false, Some(luminance), None)
    }

    /// Smart constructor for target lab luminance
    pub fn with_target_lab_luminance(luminance: f64) -> std::result::Result<Self, ConfigError> {
        Self::new(false, false, None, Some(luminance))
    }

    /// Functional combinator to add relative luminance preservation
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

    /// Functional combinator to add lab luminance preservation
    pub fn preserve_lab_luminance(self) -> std::result::Result<Self, ConfigError> {
        if self.preserve_relative_luminance {
            return Err(ConfigError::ConflictingLuminanceOptions);
        }
        Ok(Self {
            preserve_lab_luminance: true,
            ..self
        })
    }

    /// Functional combinator to set target relative luminance
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

    /// Functional combinator to set target lab luminance
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

impl Default for ColorSchemeConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Functional color scheme calculator using immutable configuration
/// 
/// This struct holds an immutable configuration and provides pure functions
/// for color scheme calculation without any mutable state.
#[derive(Debug, Clone, Copy)]
pub struct FunctionalColorSchemeCalculator {
    config: ColorSchemeConfig,
}

impl FunctionalColorSchemeCalculator {
    /// Create a new calculator with the given configuration
    pub const fn new(config: ColorSchemeConfig) -> Self {
        Self { config }
    }

    /// Create a calculator with default configuration
    pub const fn default() -> Self {
        Self::new(ColorSchemeConfig::DEFAULT)
    }

    /// Get the configuration
    pub const fn config(&self) -> ColorSchemeConfig {
        self.config
    }

    /// Calculate color schemes using the configured options
    /// 
    /// This is equivalent to the original calculate method but uses
    /// immutable configuration instead of mutable builder state.
    pub fn calculate(&self, base_color: Lab) -> Result<crate::color_schemes::ColorSchemeResult> {
        // Create a traditional calculator using the builder pattern
        let mut builder = crate::color_schemes::ColorSchemeBuilder::new();
        
        if self.config.preserve_relative_luminance {
            builder = builder.preserve_relative_luminance();
        }
        
        if self.config.preserve_lab_luminance {
            builder = builder.preserve_lab_luminance();
        }
        
        if let Some(target_rel_lum) = self.config.target_relative_luminance {
            builder = builder.with_target_relative_luminance(target_rel_lum);
        }
        
        if let Some(target_lab_lum) = self.config.target_lab_luminance {
            builder = builder.with_target_lab_luminance(target_lab_lum);
        }
        
        let traditional_calculator = builder.build();
        traditional_calculator.calculate(base_color)
    }
}

/// Convenience functions for common configuration patterns
pub mod presets {
    use super::*;

    /// Standard color scheme calculation with no special options
    pub fn standard() -> FunctionalColorSchemeCalculator {
        FunctionalColorSchemeCalculator::default()
    }

    /// Color scheme calculation with relative luminance preservation
    pub fn with_relative_luminance_preservation() -> FunctionalColorSchemeCalculator {
        FunctionalColorSchemeCalculator::new(ColorSchemeConfig::with_relative_luminance_preservation())
    }

    /// Color scheme calculation with lab luminance preservation
    pub fn with_lab_luminance_preservation() -> FunctionalColorSchemeCalculator {
        FunctionalColorSchemeCalculator::new(ColorSchemeConfig::with_lab_luminance_preservation())
    }

    /// Color scheme calculation with target relative luminance
    pub fn with_target_relative_luminance(luminance: f64) -> std::result::Result<FunctionalColorSchemeCalculator, ConfigError> {
        Ok(FunctionalColorSchemeCalculator::new(
            ColorSchemeConfig::with_target_relative_luminance(luminance)?
        ))
    }

    /// Color scheme calculation with target lab luminance
    pub fn with_target_lab_luminance(luminance: f64) -> std::result::Result<FunctionalColorSchemeCalculator, ConfigError> {
        Ok(FunctionalColorSchemeCalculator::new(
            ColorSchemeConfig::with_target_lab_luminance(luminance)?
        ))
    }

    /// Builder-like functional composition for complex configurations
    /// 
    /// This function demonstrates how functional composition can replace
    /// traditional builder patterns with compile-time safety.
    pub fn complex_config() -> std::result::Result<FunctionalColorSchemeCalculator, ConfigError> {
        let config = ColorSchemeConfig::default()
            .preserve_relative_luminance()?
            .set_target_relative_luminance(0.5)?;
        
        Ok(FunctionalColorSchemeCalculator::new(config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ColorSchemeConfig::default();
        assert!(!config.preserve_relative_luminance);
        assert!(!config.preserve_lab_luminance);
        assert!(config.target_relative_luminance.is_none());
        assert!(config.target_lab_luminance.is_none());
    }

    #[test]
    fn test_smart_constructors() {
        let config = ColorSchemeConfig::with_relative_luminance_preservation();
        assert!(config.preserve_relative_luminance);
        assert!(!config.preserve_lab_luminance);

        let config = ColorSchemeConfig::with_lab_luminance_preservation();
        assert!(!config.preserve_relative_luminance);
        assert!(config.preserve_lab_luminance);
    }

    #[test]
    fn test_validation_conflicting_luminance() {
        let result = ColorSchemeConfig::new(true, true, None, None);
        assert!(matches!(result, Err(ConfigError::ConflictingLuminanceOptions)));
    }

    #[test]
    fn test_validation_invalid_relative_luminance() {
        let result = ColorSchemeConfig::with_target_relative_luminance(1.5);
        assert!(matches!(result, Err(ConfigError::InvalidTargetLuminance { .. })));

        let result = ColorSchemeConfig::with_target_relative_luminance(-0.1);
        assert!(matches!(result, Err(ConfigError::InvalidTargetLuminance { .. })));
    }

    #[test]
    fn test_validation_invalid_lab_luminance() {
        let result = ColorSchemeConfig::with_target_lab_luminance(150.0);
        assert!(matches!(result, Err(ConfigError::InvalidTargetLuminance { .. })));

        let result = ColorSchemeConfig::with_target_lab_luminance(-10.0);
        assert!(matches!(result, Err(ConfigError::InvalidTargetLuminance { .. })));
    }

    #[test]
    fn test_validation_conflicting_targets() {
        let result = ColorSchemeConfig::new(false, false, Some(0.5), Some(50.0));
        assert!(matches!(result, Err(ConfigError::ConflictingTargetValues)));
    }

    #[test]
    fn test_functional_combinators() {
        let config = ColorSchemeConfig::default()
            .preserve_relative_luminance()
            .unwrap();
        assert!(config.preserve_relative_luminance);

        let config = ColorSchemeConfig::default()
            .set_target_relative_luminance(0.5)
            .unwrap();
        assert_eq!(config.target_relative_luminance, Some(0.5));
    }

    #[test]
    fn test_functional_calculator() {
        let config = ColorSchemeConfig::with_relative_luminance_preservation();
        let calculator = FunctionalColorSchemeCalculator::new(config);
        assert!(calculator.config().preserve_relative_luminance);

        // Test that we can get the configuration back
        let retrieved_config = calculator.config();
        assert_eq!(retrieved_config, config);
    }

    #[test]
    fn test_presets() {
        let standard = presets::standard();
        assert!(!standard.config().preserve_relative_luminance);

        let rel_lum = presets::with_relative_luminance_preservation();
        assert!(rel_lum.config().preserve_relative_luminance);

        let lab_lum = presets::with_lab_luminance_preservation();
        assert!(lab_lum.config().preserve_lab_luminance);

        let target_rel = presets::with_target_relative_luminance(0.5).unwrap();
        assert_eq!(target_rel.config().target_relative_luminance, Some(0.5));

        let target_lab = presets::with_target_lab_luminance(50.0).unwrap();
        assert_eq!(target_lab.config().target_lab_luminance, Some(50.0));
    }

    #[test]
    fn test_preset_validation() {
        let result = presets::with_target_relative_luminance(1.5);
        assert!(result.is_err());

        let result = presets::with_target_lab_luminance(150.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_complex_functional_composition() {
        // This test demonstrates functional composition replacing builder pattern
        let result = presets::complex_config();
        assert!(result.is_ok());
        
        let calculator = result.unwrap();
        let config = calculator.config();
        assert!(config.preserve_relative_luminance);
        assert_eq!(config.target_relative_luminance, Some(0.5));
    }

    #[test]
    fn test_functional_calculation_equivalence() {
        use palette::{IntoColor, Srgb};
        
        // Create traditional builder
        let traditional = crate::color_schemes::ColorSchemeBuilder::new()
            .preserve_relative_luminance()
            .build();

        // Create functional equivalent
        let functional = FunctionalColorSchemeCalculator::new(
            ColorSchemeConfig::with_relative_luminance_preservation()
        );

        // Test color
        let red_srgb = Srgb::new(1.0, 0.0, 0.0);
        let red_lab: Lab = red_srgb.into_color();

        // Both should produce equivalent results
        let traditional_result = traditional.calculate(red_lab).unwrap();
        let functional_result = functional.calculate(red_lab).unwrap();

        // Compare key fields (floating point comparison with tolerance)
        let tolerance = 0.001;
        assert!((traditional_result.base_color.l - functional_result.base_color.l).abs() < tolerance);
        assert!((traditional_result.base_color.a - functional_result.base_color.a).abs() < tolerance);
        assert!((traditional_result.base_color.b - functional_result.base_color.b).abs() < tolerance);
    }
}
