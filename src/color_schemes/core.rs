//! Core color scheme calculation components
//!
//! This module provides the main calculator, builder pattern, and result structures
//! for comprehensive color scheme calculations using multiple strategies and
//! luminance preservation options.

use crate::error::Result;
use palette::Lab;
use super::algorithms::*;
use super::strategies::*;

/// Builder for configuring color scheme calculations
#[derive(Debug, Clone)]
pub struct ColorSchemeBuilder {
    preserve_relative_luminance: bool,
    preserve_lab_luminance: bool,
    target_relative_luminance: Option<f64>,
    target_lab_luminance: Option<f64>,
}

impl ColorSchemeBuilder {
    /// Create a new builder with default settings
    pub fn new() -> Self {
        Self {
            preserve_relative_luminance: false,
            preserve_lab_luminance: false,
            target_relative_luminance: None,
            target_lab_luminance: None,
        }
    }

    /// Enable relative luminance preservation for calculated schemes
    pub fn preserve_relative_luminance(mut self) -> Self {
        self.preserve_relative_luminance = true;
        self.preserve_lab_luminance = false; // Mutually exclusive
        self
    }

    /// Enable Lab luminance preservation for calculated schemes
    pub fn preserve_lab_luminance(mut self) -> Self {
        self.preserve_lab_luminance = true;
        self.preserve_relative_luminance = false; // Mutually exclusive
        self
    }

    /// Set target relative luminance for the base color
    pub fn target_relative_luminance(mut self, luminance: f64) -> Self {
        self.target_relative_luminance = Some(luminance);
        self.target_lab_luminance = None; // Mutually exclusive
        self
    }

    /// Set target Lab luminance for the base color
    pub fn target_lab_luminance(mut self, luminance: f64) -> Self {
        self.target_lab_luminance = Some(luminance);
        self.target_relative_luminance = None; // Mutually exclusive
        self
    }

    /// Build the color scheme calculator
    pub fn build(self) -> ColorSchemeCalculator {
        ColorSchemeCalculator {
            preserve_relative_luminance: self.preserve_relative_luminance,
            preserve_lab_luminance: self.preserve_lab_luminance,
            target_relative_luminance: self.target_relative_luminance,
            target_lab_luminance: self.target_lab_luminance,
        }
    }
}

impl Default for ColorSchemeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculator for color schemes with various options
pub struct ColorSchemeCalculator {
    preserve_relative_luminance: bool,
    preserve_lab_luminance: bool,
    target_relative_luminance: Option<f64>,
    target_lab_luminance: Option<f64>,
}

impl ColorSchemeCalculator {
    /// Create a new calculator with default settings
    pub fn new() -> Self {
        ColorSchemeBuilder::new().build()
    }

    /// Calculate color schemes for the given color using both HSL and Lab strategies
    pub fn calculate(&self, mut base_color: Lab) -> Result<ColorSchemeResult> {
        // Apply color replacement if target luminance is specified
        base_color = self.apply_target_luminance(base_color)?;

        // Create both strategies
        let hsl_strategy = HslColorSchemeStrategy;
        let lab_strategy = LabColorSchemeStrategy;

        // Calculate basic schemes using both strategies
        let basic_schemes = self.calculate_basic_schemes(base_color, &hsl_strategy, &lab_strategy);
        
        // Calculate luminance-matched variations if requested
        let luminance_matched = self.calculate_luminance_matched_schemes(base_color, &basic_schemes)?;

        Ok(ColorSchemeResult {
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
        })
    }

    /// Apply target luminance adjustment to base color if specified
    fn apply_target_luminance(&self, base_color: Lab) -> Result<Lab> {
        if let Some(target_rel_lum) = self.target_relative_luminance {
            adjust_color_relative_luminance(base_color, target_rel_lum)
        } else if let Some(target_lab_lum) = self.target_lab_luminance {
            adjust_color_lab_luminance(base_color, target_lab_lum)
        } else {
            Ok(base_color)
        }
    }

    /// Calculate basic color schemes using both strategies
    fn calculate_basic_schemes(
        &self,
        base_color: Lab,
        hsl_strategy: &HslColorSchemeStrategy,
        lab_strategy: &LabColorSchemeStrategy,
    ) -> BasicSchemes {
        BasicSchemes {
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

    /// Calculate luminance-matched schemes if requested
    fn calculate_luminance_matched_schemes(
        &self,
        base_color: Lab,
        basic_schemes: &BasicSchemes,
    ) -> Result<LuminanceMatchedSchemes> {
        if self.preserve_relative_luminance {
            Ok(LuminanceMatchedSchemes {
                hsl_complementary: Some(preserve_wcag_relative_luminance(
                    basic_schemes.hsl_complementary,
                    base_color,
                )?),
                hsl_split_complementary: Some((
                    preserve_wcag_relative_luminance(basic_schemes.hsl_split_complementary.0, base_color)?,
                    preserve_wcag_relative_luminance(basic_schemes.hsl_split_complementary.1, base_color)?,
                )),
                hsl_triadic: Some((
                    preserve_wcag_relative_luminance(basic_schemes.hsl_triadic.0, base_color)?,
                    preserve_wcag_relative_luminance(basic_schemes.hsl_triadic.1, base_color)?,
                )),
                hsl_tetradic: Some((
                    preserve_wcag_relative_luminance(basic_schemes.hsl_tetradic.0, base_color)?,
                    preserve_wcag_relative_luminance(basic_schemes.hsl_tetradic.1, base_color)?,
                    preserve_wcag_relative_luminance(basic_schemes.hsl_tetradic.2, base_color)?,
                )),
                lab_complementary: Some(preserve_wcag_relative_luminance(
                    basic_schemes.lab_complementary,
                    base_color,
                )?),
                lab_split_complementary: Some((
                    preserve_wcag_relative_luminance(basic_schemes.lab_split_complementary.0, base_color)?,
                    preserve_wcag_relative_luminance(basic_schemes.lab_split_complementary.1, base_color)?,
                )),
                lab_triadic: Some((
                    preserve_wcag_relative_luminance(basic_schemes.lab_triadic.0, base_color)?,
                    preserve_wcag_relative_luminance(basic_schemes.lab_triadic.1, base_color)?,
                )),
                lab_tetradic: Some((
                    preserve_wcag_relative_luminance(basic_schemes.lab_tetradic.0, base_color)?,
                    preserve_wcag_relative_luminance(basic_schemes.lab_tetradic.1, base_color)?,
                    preserve_wcag_relative_luminance(basic_schemes.lab_tetradic.2, base_color)?,
                )),
            })
        } else if self.preserve_lab_luminance {
            Ok(LuminanceMatchedSchemes {
                hsl_complementary: Some(preserve_lab_luminance(
                    basic_schemes.hsl_complementary,
                    base_color,
                )?),
                hsl_split_complementary: Some((
                    preserve_lab_luminance(basic_schemes.hsl_split_complementary.0, base_color)?,
                    preserve_lab_luminance(basic_schemes.hsl_split_complementary.1, base_color)?,
                )),
                hsl_triadic: Some((
                    preserve_lab_luminance(basic_schemes.hsl_triadic.0, base_color)?,
                    preserve_lab_luminance(basic_schemes.hsl_triadic.1, base_color)?,
                )),
                hsl_tetradic: Some((
                    preserve_lab_luminance(basic_schemes.hsl_tetradic.0, base_color)?,
                    preserve_lab_luminance(basic_schemes.hsl_tetradic.1, base_color)?,
                    preserve_lab_luminance(basic_schemes.hsl_tetradic.2, base_color)?,
                )),
                lab_complementary: Some(preserve_lab_luminance(
                    basic_schemes.lab_complementary,
                    base_color,
                )?),
                lab_split_complementary: Some((
                    preserve_lab_luminance(basic_schemes.lab_split_complementary.0, base_color)?,
                    preserve_lab_luminance(basic_schemes.lab_split_complementary.1, base_color)?,
                )),
                lab_triadic: Some((
                    preserve_lab_luminance(basic_schemes.lab_triadic.0, base_color)?,
                    preserve_lab_luminance(basic_schemes.lab_triadic.1, base_color)?,
                )),
                lab_tetradic: Some((
                    preserve_lab_luminance(basic_schemes.lab_tetradic.0, base_color)?,
                    preserve_lab_luminance(basic_schemes.lab_tetradic.1, base_color)?,
                    preserve_lab_luminance(basic_schemes.lab_tetradic.2, base_color)?,
                )),
            })
        } else {
            Ok(LuminanceMatchedSchemes::none())
        }
    }
}

impl Default for ColorSchemeCalculator {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of color scheme calculations with both HSL and Lab strategies
#[derive(Debug, Clone)]
pub struct ColorSchemeResult {
    pub base_color: Lab,

    // HSL strategy results
    pub hsl_complementary: Lab,
    pub hsl_split_complementary: (Lab, Lab),
    pub hsl_triadic: (Lab, Lab),
    pub hsl_tetradic: (Lab, Lab, Lab),

    // Lab strategy results
    pub lab_complementary: Lab,
    pub lab_split_complementary: (Lab, Lab),
    pub lab_triadic: (Lab, Lab),
    pub lab_tetradic: (Lab, Lab, Lab),

    // Luminance-matched variations (if requested)
    pub luminance_matched_hsl_complementary: Option<Lab>,
    pub luminance_matched_hsl_split_complementary: Option<(Lab, Lab)>,
    pub luminance_matched_hsl_triadic: Option<(Lab, Lab)>,
    pub luminance_matched_hsl_tetradic: Option<(Lab, Lab, Lab)>,
    pub luminance_matched_lab_complementary: Option<Lab>,
    pub luminance_matched_lab_split_complementary: Option<(Lab, Lab)>,
    pub luminance_matched_lab_triadic: Option<(Lab, Lab)>,
    pub luminance_matched_lab_tetradic: Option<(Lab, Lab, Lab)>,
}

/// Helper struct for basic scheme calculations
struct BasicSchemes {
    hsl_complementary: Lab,
    hsl_split_complementary: (Lab, Lab),
    hsl_triadic: (Lab, Lab),
    hsl_tetradic: (Lab, Lab, Lab),
    lab_complementary: Lab,
    lab_split_complementary: (Lab, Lab),
    lab_triadic: (Lab, Lab),
    lab_tetradic: (Lab, Lab, Lab),
}

/// Helper struct for luminance-matched schemes
struct LuminanceMatchedSchemes {
    hsl_complementary: Option<Lab>,
    hsl_split_complementary: Option<(Lab, Lab)>,
    hsl_triadic: Option<(Lab, Lab)>,
    hsl_tetradic: Option<(Lab, Lab, Lab)>,
    lab_complementary: Option<Lab>,
    lab_split_complementary: Option<(Lab, Lab)>,
    lab_triadic: Option<(Lab, Lab)>,
    lab_tetradic: Option<(Lab, Lab, Lab)>,
}

impl LuminanceMatchedSchemes {
    fn none() -> Self {
        Self {
            hsl_complementary: None,
            hsl_split_complementary: None,
            hsl_triadic: None,
            hsl_tetradic: None,
            lab_complementary: None,
            lab_split_complementary: None,
            lab_triadic: None,
            lab_tetradic: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::{IntoColor, Srgb};

    #[test]
    fn test_color_scheme_builder() {
        let calculator = ColorSchemeBuilder::new()
            .preserve_relative_luminance()
            .build();

        assert!(calculator.preserve_relative_luminance);
        assert!(!calculator.preserve_lab_luminance);
    }

    #[test]
    fn test_color_scheme_builder_lab_luminance() {
        let calculator = ColorSchemeBuilder::new()
            .preserve_lab_luminance()
            .build();

        assert!(!calculator.preserve_relative_luminance);
        assert!(calculator.preserve_lab_luminance);
    }

    #[test]
    fn test_color_scheme_builder_target_luminance() {
        let calculator = ColorSchemeBuilder::new()
            .target_relative_luminance(0.5)
            .build();

        assert_eq!(calculator.target_relative_luminance, Some(0.5));
        assert_eq!(calculator.target_lab_luminance, None);
    }

    #[test]
    fn test_color_scheme_calculation() {
        let calculator = ColorSchemeBuilder::new().build();
        let red_srgb = Srgb::new(1.0, 0.0, 0.0);
        let red_lab: Lab = red_srgb.into_color();

        let result = calculator.calculate(red_lab).unwrap();

        // Basic validation that scheme calculation works
        // Test that we get different colors (using distance comparison instead of direct equality)
        let tolerance = 0.1; // Small tolerance for floating point comparison

        assert!(
            (result.hsl_complementary.l - red_lab.l).abs() > tolerance
                || (result.hsl_complementary.a - red_lab.a).abs() > tolerance
                || (result.hsl_complementary.b - red_lab.b).abs() > tolerance
        );

        assert!(result.lab_complementary != red_lab);

        // For triadic, just test that the function executed successfully
        assert!(result.lab_triadic.0.l >= 0.0); // Basic validity check
    }

    #[test]
    fn test_calculator_default() {
        let calculator = ColorSchemeCalculator::default();
        let red_lab: Lab = Srgb::new(1.0, 0.0, 0.0).into_color();

        let result = calculator.calculate(red_lab).unwrap();
        
        // Should have basic schemes but no luminance matching
        assert!(result.luminance_matched_hsl_complementary.is_none());
        assert!(result.luminance_matched_lab_complementary.is_none());
    }

    #[test]
    fn test_luminance_preservation() {
        let calculator = ColorSchemeBuilder::new()
            .preserve_relative_luminance()
            .build();

        let red_lab: Lab = Srgb::new(1.0, 0.0, 0.0).into_color();
        let result = calculator.calculate(red_lab).unwrap();

        // Should have luminance-matched variations
        assert!(result.luminance_matched_hsl_complementary.is_some());
        assert!(result.luminance_matched_lab_complementary.is_some());
    }
}
