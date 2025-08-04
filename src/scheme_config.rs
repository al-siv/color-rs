//! Color scheme configuration using immutable patterns and smart constructors
//!
//! This module provides an alternative to the builder pattern for color scheme configuration.
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

impl Default for ColorSchemeConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Color scheme calculator using immutable configuration
/// 
/// This struct holds an immutable configuration and provides pure functions
/// for color scheme calculation without any mutable state.
#[derive(Debug, Clone, Copy)]
pub struct ColorSchemeCalculator {
    config: ColorSchemeConfig,
}

impl ColorSchemeCalculator {
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
    pub fn standard() -> ColorSchemeCalculator {
        ColorSchemeCalculator::default()
    }

    /// Color scheme calculation with relative luminance preservation
    pub fn with_relative_luminance_preservation() -> ColorSchemeCalculator {
        ColorSchemeCalculator::new(ColorSchemeConfig::with_relative_luminance_preservation())
    }

    /// Color scheme calculation with lab luminance preservation
    pub fn with_lab_luminance_preservation() -> ColorSchemeCalculator {
        ColorSchemeCalculator::new(ColorSchemeConfig::with_lab_luminance_preservation())
    }

    /// Color scheme calculation with target relative luminance
    pub fn with_target_relative_luminance(luminance: f64) -> std::result::Result<ColorSchemeCalculator, ConfigError> {
        Ok(ColorSchemeCalculator::new(
            ColorSchemeConfig::with_target_relative_luminance(luminance)?
        ))
    }

    /// Color scheme calculation with target lab luminance
    pub fn with_target_lab_luminance(luminance: f64) -> std::result::Result<ColorSchemeCalculator, ConfigError> {
        Ok(ColorSchemeCalculator::new(
            ColorSchemeConfig::with_target_lab_luminance(luminance)?
        ))
    }

    /// Builder-like configuration composition for complex configurations
    /// 
    /// This function demonstrates how configuration composition can replace
    /// traditional builder patterns with compile-time safety.
    pub fn complex_config() -> std::result::Result<ColorSchemeCalculator, ConfigError> {
        let config = ColorSchemeConfig::default()
            .preserve_relative_luminance()?
            .set_target_relative_luminance(0.5)?;
        
        Ok(ColorSchemeCalculator::new(config))
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
    fn test_scheme_combinators() {
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
    fn test_scheme_calculator() {
        let config = ColorSchemeConfig::with_relative_luminance_preservation();
        let calculator = ColorSchemeCalculator::new(config);
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
    fn test_complex_configuration_composition() {
        // This test demonstrates configuration composition replacing builder pattern
        let result = presets::complex_config();
        assert!(result.is_ok());
        
        let calculator = result.unwrap();
        let config = calculator.config();
        assert!(config.preserve_relative_luminance);
        assert_eq!(config.target_relative_luminance, Some(0.5));
    }

    #[test]
    fn test_scheme_calculation_equivalence() {
        use palette::{IntoColor, Srgb};
        
        // Create traditional builder
        let traditional = crate::color_schemes::ColorSchemeBuilder::new()
            .preserve_relative_luminance()
            .build();

        // Create configuration equivalent
        let functional = ColorSchemeCalculator::new(
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

/// Functional refactoring of the long ColorSchemeCalculator::calculate method
/// 
/// This module breaks down the 140+ line calculate method into focused, composable functions.
pub mod scheme_calculation {
    use super::*;
    use crate::color_schemes::{
        ColorSchemeResult, HslColorSchemeStrategy, LabColorSchemeStrategy, ColorSchemeStrategy,
        preserve_wcag_relative_luminance, preserve_lab_luminance,
        adjust_color_lab_luminance,
    };
    use crate::color_ops::luminance::wcag_relative;

    /// Local implementation of relative luminance adjustment since the original is private
    fn adjust_color_relative_luminance(color: Lab, target_luminance: f64) -> Result<Lab> {
        use palette::{IntoColor, Srgb};
        
        // Convert to RGB, check current luminance
        let srgb: Srgb = color.into_color();
        let current_luminance = wcag_relative(srgb);
        
        if (current_luminance - target_luminance).abs() < 0.001 {
            return Ok(color);
        }
        
        // Binary search for target luminance by scaling lightness
        let mut low = 0.0_f32;
        let mut high = 100.0_f32;
        let mut best_color = color;
        
        for _ in 0..50 {
            let mid = (low + high) / 2.0;
            let test_color = Lab::new(mid, color.a, color.b);
            let test_srgb: Srgb = test_color.into_color();
            let test_luminance = wcag_relative(test_srgb);
            
            if (test_luminance - target_luminance).abs() < 0.001 {
                return Ok(test_color);
            }
            
            if test_luminance < target_luminance {
                low = mid;
            } else {
                high = mid;
            }
            
            best_color = test_color;
        }
        
        Ok(best_color)
    }

    /// Configuration for luminance preservation
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

    /// Apply target luminance adjustment to base color
    pub fn apply_target_luminance(config: ColorSchemeConfig, mut base_color: Lab) -> Result<Lab> {
        if let Some(target_rel_lum) = config.target_relative_luminance {
            base_color = adjust_color_relative_luminance(base_color, target_rel_lum)?;
        } else if let Some(target_lab_lum) = config.target_lab_luminance {
            base_color = adjust_color_lab_luminance(base_color, target_lab_lum)?;
        }
        Ok(base_color)
    }

    /// Calculate basic color schemes using both HSL and Lab strategies
    #[derive(Debug, Clone)]
    pub struct BasicColorSchemes {
        pub hsl_complementary: Lab,
        pub hsl_split_complementary: (Lab, Lab),
        pub hsl_triadic: (Lab, Lab),
        pub hsl_tetradic: (Lab, Lab, Lab),
        pub lab_complementary: Lab,
        pub lab_split_complementary: (Lab, Lab),
        pub lab_triadic: (Lab, Lab),
        pub lab_tetradic: (Lab, Lab, Lab),
    }

    pub fn calculate_basic_schemes(base_color: Lab) -> BasicColorSchemes {
        let hsl_strategy = HslColorSchemeStrategy;
        let lab_strategy = LabColorSchemeStrategy;

        BasicColorSchemes {
            hsl_complementary: hsl_strategy.complementary(base_color),
            hsl_split_complementary: hsl_strategy.split_complementary(base_color),
            hsl_triadic: hsl_strategy.triadic(base_color),
            hsl_tetradic: hsl_strategy.tetradic(base_color),
            lab_complementary: lab_strategy.complementary(base_color),
            lab_split_complementary: lab_strategy.split_complementary(base_color),
            lab_triadic: lab_strategy.triadic(base_color),
            lab_tetradic: lab_strategy.tetradic(base_color),
        }
    }

    /// Apply luminance matching to a single color
    pub fn apply_luminance_matching(
        color: Lab,
        base_color: Lab,
        config: LuminanceConfig,
    ) -> Result<Option<Lab>> {
        if config.preserve_relative_luminance {
            Ok(Some(preserve_wcag_relative_luminance(color, base_color)?))
        } else if config.preserve_lab_luminance {
            Ok(Some(preserve_lab_luminance(color, base_color)?))
        } else {
            Ok(None)
        }
    }

    /// Apply luminance matching to a pair of colors
    pub fn apply_luminance_matching_pair(
        colors: (Lab, Lab),
        base_color: Lab,
        config: LuminanceConfig,
    ) -> Result<Option<(Lab, Lab)>> {
        if config.preserve_relative_luminance {
            let color1 = preserve_wcag_relative_luminance(colors.0, base_color)?;
            let color2 = preserve_wcag_relative_luminance(colors.1, base_color)?;
            Ok(Some((color1, color2)))
        } else if config.preserve_lab_luminance {
            let color1 = preserve_lab_luminance(colors.0, base_color)?;
            let color2 = preserve_lab_luminance(colors.1, base_color)?;
            Ok(Some((color1, color2)))
        } else {
            Ok(None)
        }
    }

    /// Apply luminance matching to a triple of colors
    pub fn apply_luminance_matching_triple(
        colors: (Lab, Lab, Lab),
        base_color: Lab,
        config: LuminanceConfig,
    ) -> Result<Option<(Lab, Lab, Lab)>> {
        if config.preserve_relative_luminance {
            let color1 = preserve_wcag_relative_luminance(colors.0, base_color)?;
            let color2 = preserve_wcag_relative_luminance(colors.1, base_color)?;
            let color3 = preserve_wcag_relative_luminance(colors.2, base_color)?;
            Ok(Some((color1, color2, color3)))
        } else if config.preserve_lab_luminance {
            let color1 = preserve_lab_luminance(colors.0, base_color)?;
            let color2 = preserve_lab_luminance(colors.1, base_color)?;
            let color3 = preserve_lab_luminance(colors.2, base_color)?;
            Ok(Some((color1, color2, color3)))
        } else {
            Ok(None)
        }
    }

    /// Calculate all luminance-matched variations
    #[derive(Debug, Clone)]
    pub struct LuminanceMatchedSchemes {
        pub hsl_complementary: Option<Lab>,
        pub hsl_split_complementary: Option<(Lab, Lab)>,
        pub hsl_triadic: Option<(Lab, Lab)>,
        pub hsl_tetradic: Option<(Lab, Lab, Lab)>,
        pub lab_complementary: Option<Lab>,
        pub lab_split_complementary: Option<(Lab, Lab)>,
        pub lab_triadic: Option<(Lab, Lab)>,
        pub lab_tetradic: Option<(Lab, Lab, Lab)>,
    }

    pub fn calculate_luminance_matched_schemes(
        basic_schemes: &BasicColorSchemes,
        base_color: Lab,
        luminance_config: LuminanceConfig,
    ) -> Result<LuminanceMatchedSchemes> {
        Ok(LuminanceMatchedSchemes {
            hsl_complementary: apply_luminance_matching(
                basic_schemes.hsl_complementary,
                base_color,
                luminance_config,
            )?,
            hsl_split_complementary: apply_luminance_matching_pair(
                basic_schemes.hsl_split_complementary,
                base_color,
                luminance_config,
            )?,
            hsl_triadic: apply_luminance_matching_pair(
                basic_schemes.hsl_triadic,
                base_color,
                luminance_config,
            )?,
            hsl_tetradic: apply_luminance_matching_triple(
                basic_schemes.hsl_tetradic,
                base_color,
                luminance_config,
            )?,
            lab_complementary: apply_luminance_matching(
                basic_schemes.lab_complementary,
                base_color,
                luminance_config,
            )?,
            lab_split_complementary: apply_luminance_matching_pair(
                basic_schemes.lab_split_complementary,
                base_color,
                luminance_config,
            )?,
            lab_triadic: apply_luminance_matching_pair(
                basic_schemes.lab_triadic,
                base_color,
                luminance_config,
            )?,
            lab_tetradic: apply_luminance_matching_triple(
                basic_schemes.lab_tetradic,
                base_color,
                luminance_config,
            )?,
        })
    }

    /// Combine basic and luminance-matched schemes into final result
    pub fn create_color_scheme_result(
        base_color: Lab,
        basic_schemes: BasicColorSchemes,
        luminance_matched: LuminanceMatchedSchemes,
    ) -> ColorSchemeResult {
        ColorSchemeResult {
            base_color,
            hsl_complementary: basic_schemes.hsl_complementary,
            hsl_split_complementary: basic_schemes.hsl_split_complementary,
            hsl_triadic: basic_schemes.hsl_triadic,
            hsl_tetradic: basic_schemes.hsl_tetradic,
            lab_complementary: basic_schemes.lab_complementary,
            lab_split_complementary: basic_schemes.lab_split_complementary,
            lab_triadic: basic_schemes.lab_triadic,
            lab_tetradic: basic_schemes.lab_tetradic,
            luminance_matched_hsl_complementary: luminance_matched.hsl_complementary,
            luminance_matched_hsl_split_complementary: luminance_matched.hsl_split_complementary,
            luminance_matched_hsl_triadic: luminance_matched.hsl_triadic,
            luminance_matched_hsl_tetradic: luminance_matched.hsl_tetradic,
            luminance_matched_lab_complementary: luminance_matched.lab_complementary,
            luminance_matched_lab_split_complementary: luminance_matched.lab_split_complementary,
            luminance_matched_lab_triadic: luminance_matched.lab_triadic,
            luminance_matched_lab_tetradic: luminance_matched.lab_tetradic,
        }
    }

    /// Functional refactored version of the calculate method
    pub fn calculate_color_schemes(
        config: ColorSchemeConfig,
        base_color: Lab,
    ) -> Result<ColorSchemeResult> {
        // Apply target luminance adjustments
        let adjusted_base_color = apply_target_luminance(config, base_color)?;

        // Calculate basic color schemes
        let basic_schemes = calculate_basic_schemes(adjusted_base_color);

        // Calculate luminance-matched variations
        let luminance_config = LuminanceConfig::from(config);
        let luminance_matched = calculate_luminance_matched_schemes(
            &basic_schemes,
            adjusted_base_color,
            luminance_config,
        )?;

        // Combine into final result
        Ok(create_color_scheme_result(
            adjusted_base_color,
            basic_schemes,
            luminance_matched,
        ))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use palette::{IntoColor, Srgb};

        #[test]
        fn test_apply_target_luminance() {
            let config = ColorSchemeConfig::with_target_relative_luminance(0.5).unwrap();
            let base_color = Lab::new(20.0, 10.0, -5.0);
            
            let result = apply_target_luminance(config, base_color).unwrap();
            // Should have adjusted the color for target luminance
            assert_ne!(result, base_color);
        }

        #[test]
        fn test_calculate_basic_schemes() {
            let red_srgb = Srgb::new(1.0, 0.0, 0.0);
            let red_lab: Lab = red_srgb.into_color();
            
            let schemes = calculate_basic_schemes(red_lab);
            
            // HSL and Lab complementary should be different
            assert_ne!(schemes.hsl_complementary, schemes.lab_complementary);
            
            // Split complementary should return pairs
            assert_ne!(schemes.hsl_split_complementary.0, schemes.hsl_split_complementary.1);
            assert_ne!(schemes.lab_split_complementary.0, schemes.lab_split_complementary.1);
        }

        #[test]
        fn test_luminance_matching_single() {
            let red_lab = Lab::new(53.24, 80.09, 67.20);
            let blue_lab = Lab::new(32.30, 79.19, -107.86);
            
            let config = LuminanceConfig {
                preserve_relative_luminance: true,
                preserve_lab_luminance: false,
            };
            
            let result = apply_luminance_matching(blue_lab, red_lab, config).unwrap();
            assert!(result.is_some());
            
            let matched_color = result.unwrap();
            assert_ne!(matched_color, blue_lab); // Should be adjusted
        }

        #[test]
        fn test_luminance_matching_pair() {
            let red_lab = Lab::new(53.24, 80.09, 67.20);
            let colors = (Lab::new(20.0, 10.0, -5.0), Lab::new(80.0, -10.0, 15.0));
            
            let config = LuminanceConfig {
                preserve_relative_luminance: false,
                preserve_lab_luminance: true,
            };
            
            let result = apply_luminance_matching_pair(colors, red_lab, config).unwrap();
            assert!(result.is_some());
            
            let matched_colors = result.unwrap();
            assert_ne!(matched_colors.0, colors.0); // Should be adjusted
            assert_ne!(matched_colors.1, colors.1); // Should be adjusted
        }

        #[test]
        fn test_scheme_equivalence() {
            let red_srgb = Srgb::new(1.0, 0.0, 0.0);
            let red_lab: Lab = red_srgb.into_color();
            
            let config = ColorSchemeConfig::with_relative_luminance_preservation();
            
            // Test functional version
            let functional_result = calculate_color_schemes(config, red_lab).unwrap();
            
            // Test original version via ColorSchemeCalculator
            let calculator = ColorSchemeCalculator::new(config);
            let original_result = calculator.calculate(red_lab).unwrap();
            
            // Compare key fields (allowing for floating point precision)
            assert_eq!(functional_result.base_color.l, original_result.base_color.l);
            assert_eq!(functional_result.hsl_complementary.l, original_result.hsl_complementary.l);
            assert_eq!(functional_result.lab_complementary.l, original_result.lab_complementary.l);
        }

        #[test]
        fn test_no_luminance_preservation() {
            let config = LuminanceConfig {
                preserve_relative_luminance: false,
                preserve_lab_luminance: false,
            };
            
            let color = Lab::new(50.0, 10.0, -5.0);
            let base_color = Lab::new(30.0, 5.0, 10.0);
            
            let result = apply_luminance_matching(color, base_color, config).unwrap();
            assert!(result.is_none());
        }
    }
}
