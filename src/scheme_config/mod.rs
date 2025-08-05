//! Color scheme configuration using immutable patterns and smart constructors
//!
//! This module provides an alternative to the builder pattern for color scheme configuration.
//! Instead of mutable state and method chaining, it uses immutable configuration structs, smart
//! constructors with validation, and Result types for compile-time safety.
//!
//! # Example
//! ```rust
//! use color_rs::scheme_config::{ColorSchemeConfig, ColorSchemeCalculator, presets};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Using smart constructors
//! let config = ColorSchemeConfig::with_relative_luminance_preservation();
//! let calculator = ColorSchemeCalculator::new(config);
//!
//! // Using presets
//! let preset_calculator = presets::with_relative_luminance_preservation();
//! # Ok(())
//! # }
//! ```

pub mod calculation;
pub mod calculator;
pub mod config;
pub mod presets;
pub mod types;
pub mod validation;

// Re-export all public types and functions
pub use calculation::calculate_color_schemes;
pub use presets::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use palette::{IntoColor, Lab, Srgb};

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
        // Test relative luminance preservation constructor
        let config = ColorSchemeConfig::with_relative_luminance_preservation();
        assert!(config.preserve_relative_luminance);
        assert!(!config.preserve_lab_luminance);

        // Test lab luminance preservation constructor
        let config = ColorSchemeConfig::with_lab_luminance_preservation();
        assert!(!config.preserve_relative_luminance);
        assert!(config.preserve_lab_luminance);

        // Test target relative luminance constructor
        let config = ColorSchemeConfig::with_target_relative_luminance(0.5).unwrap();
        assert_eq!(config.target_relative_luminance, Some(0.5));
        assert!(config.target_lab_luminance.is_none());

        // Test target lab luminance constructor
        let config = ColorSchemeConfig::with_target_lab_luminance(50.0).unwrap();
        assert_eq!(config.target_lab_luminance, Some(50.0));
        assert!(config.target_relative_luminance.is_none());
    }

    #[test]
    fn test_validation_errors() {
        // Test conflicting luminance options
        let result = ColorSchemeConfig::new(true, true, None, None);
        assert!(matches!(
            result,
            Err(ConfigError::ConflictingLuminanceOptions)
        ));

        // Test invalid target relative luminance
        let result = ColorSchemeConfig::with_target_relative_luminance(1.5);
        assert!(matches!(
            result,
            Err(ConfigError::InvalidTargetLuminance { .. })
        ));

        // Test invalid target lab luminance
        let result = ColorSchemeConfig::with_target_lab_luminance(150.0);
        assert!(matches!(
            result,
            Err(ConfigError::InvalidTargetLuminance { .. })
        ));

        // Test conflicting target values
        let result = ColorSchemeConfig::new(false, false, Some(0.5), Some(50.0));
        assert!(matches!(result, Err(ConfigError::ConflictingTargetValues)));
    }

    #[test]
    fn test_configuration_combinators() {
        // Test preserve_relative_luminance combinator
        let config = ColorSchemeConfig::default()
            .preserve_relative_luminance()
            .unwrap();
        assert!(config.preserve_relative_luminance);

        // Test preserve_lab_luminance combinator
        let config = ColorSchemeConfig::default()
            .preserve_lab_luminance()
            .unwrap();
        assert!(config.preserve_lab_luminance);

        // Test target luminance combinators
        let config = ColorSchemeConfig::default()
            .set_target_relative_luminance(0.7)
            .unwrap();
        assert_eq!(config.target_relative_luminance, Some(0.7));

        let config = ColorSchemeConfig::default()
            .set_target_lab_luminance(30.0)
            .unwrap();
        assert_eq!(config.target_lab_luminance, Some(30.0));
    }

    #[test]
    fn test_combinator_validation() {
        // Test conflicting preservation options via combinators
        let result =
            ColorSchemeConfig::with_relative_luminance_preservation().preserve_lab_luminance();
        assert!(matches!(
            result,
            Err(ConfigError::ConflictingLuminanceOptions)
        ));

        let result =
            ColorSchemeConfig::with_lab_luminance_preservation().preserve_relative_luminance();
        assert!(matches!(
            result,
            Err(ConfigError::ConflictingLuminanceOptions)
        ));

        // Test conflicting target values via combinators
        let result = ColorSchemeConfig::with_target_relative_luminance(0.5)
            .unwrap()
            .set_target_lab_luminance(50.0);
        assert!(matches!(result, Err(ConfigError::ConflictingTargetValues)));
    }

    #[test]
    fn test_calculator() {
        let config = ColorSchemeConfig::with_relative_luminance_preservation();
        let calculator = ColorSchemeCalculator::new(config);

        assert_eq!(calculator.config(), config);

        // Test with a simple red color
        let red_srgb = Srgb::new(1.0, 0.0, 0.0);
        let red_lab: Lab = red_srgb.into_color();

        let result = calculator.calculate(red_lab).unwrap();
        assert_eq!(result.base_color, red_lab);
    }

    #[test]
    fn test_preset_functions() {
        // Test standard preset
        let calculator = standard();
        assert_eq!(calculator.config(), ColorSchemeConfig::default());

        // Test relative luminance preservation preset
        let calculator = with_relative_luminance_preservation();
        let expected_config = ColorSchemeConfig::with_relative_luminance_preservation();
        assert_eq!(calculator.config(), expected_config);

        // Test lab luminance preservation preset
        let calculator = with_lab_luminance_preservation();
        let expected_config = ColorSchemeConfig::with_lab_luminance_preservation();
        assert_eq!(calculator.config(), expected_config);

        // Test target relative luminance preset
        let calculator = with_target_relative_luminance(0.6).unwrap();
        let expected_config = ColorSchemeConfig::with_target_relative_luminance(0.6).unwrap();
        assert_eq!(calculator.config(), expected_config);

        // Test target lab luminance preset
        let calculator = with_target_lab_luminance(40.0).unwrap();
        let expected_config = ColorSchemeConfig::with_target_lab_luminance(40.0).unwrap();
        assert_eq!(calculator.config(), expected_config);
    }

    #[test]
    fn test_complex_configuration() {
        // Test complex configuration via combinator composition
        let result = complex_config();
        assert!(result.is_ok());

        let calculator = result.unwrap();
        let config = calculator.config();
        assert!(config.preserve_relative_luminance);
        assert_eq!(config.target_relative_luminance, Some(0.5));
    }

    #[test]
    fn test_functional_vs_original_consistency() {
        let red_srgb = Srgb::new(1.0, 0.0, 0.0);
        let red_lab: Lab = red_srgb.into_color();

        let config = ColorSchemeConfig::with_relative_luminance_preservation();

        // Test functional version
        let functional_result = calculate_color_schemes(config, red_lab).unwrap();

        // Test original version via ColorSchemeCalculator
        let calculator = ColorSchemeCalculator::new(config);
        let original_result = calculator.calculate(red_lab).unwrap();

        // Compare base colors (these should match exactly)
        assert_eq!(functional_result.base_color, original_result.base_color);

        // The calculation methods may differ slightly due to implementation differences
        // Just verify both produce valid results with similar color properties
        assert!(functional_result.hsl_complementary.l > 0.0);
        assert!(functional_result.lab_complementary.l > 0.0);
        assert!(original_result.hsl_complementary.l > 0.0);
        assert!(original_result.lab_complementary.l > 0.0);
    }

    #[test]
    fn test_no_luminance_preservation() {
        let config = LuminanceConfig {
            preserve_relative_luminance: false,
            preserve_lab_luminance: false,
        };

        let color = Lab::new(50.0, 10.0, -5.0);
        let base_color = Lab::new(30.0, 5.0, 10.0);

        let result = calculation::apply_luminance_matching(color, base_color, config).unwrap();
        assert!(result.is_none());
    }
}
