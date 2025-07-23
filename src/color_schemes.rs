//! Color harmony and color scheme calculations using traditional color theory
//!
//! This module provides functionality for calculating color harmonies such as
//! complementary, split-complementary, and triadic colors using both HSV and
//! Lab color spaces, with support for luminance matching.

use crate::color_utils::LegacyColorUtils as ColorUtils;
use crate::error::{ColorError, Result};
use palette::Lab;

/// Strategy trait for different color scheme calculation methods
pub trait ColorSchemeStrategy {
    /// Calculate complementary color
    fn complementary(&self, color: Lab) -> Lab;

    /// Calculate split-complementary colors
    fn split_complementary(&self, color: Lab) -> (Lab, Lab);

    /// Calculate triadic colors  
    fn triadic(&self, color: Lab) -> (Lab, Lab);

    /// Calculate tetradic colors (four colors, evenly spaced)
    fn tetradic(&self, color: Lab) -> (Lab, Lab, Lab);

    /// Get the name of this strategy
    fn name(&self) -> &'static str;
}

/// HSL-based color scheme strategy (default method)
pub struct HslColorSchemeStrategy;

impl ColorSchemeStrategy for HslColorSchemeStrategy {
    fn complementary(&self, color: Lab) -> Lab {
        let colors = ColorUtils::complementary_hsl(color);
        if colors.len() >= 2 {
            colors[1] // Return the actual complementary color, not the original
        } else {
            color
        }
    }

    fn split_complementary(&self, color: Lab) -> (Lab, Lab) {
        let colors = ColorUtils::split_complementary_hsl(color);
        if colors.len() >= 2 {
            (colors[0], colors[1])
        } else {
            (color, color)
        }
    }

    fn triadic(&self, color: Lab) -> (Lab, Lab) {
        let colors = ColorUtils::triadic_hsl(color);
        if colors.len() >= 2 {
            (colors[0], colors[1])
        } else {
            (color, color)
        }
    }

    fn tetradic(&self, color: Lab) -> (Lab, Lab, Lab) {
        let colors = ColorUtils::tetradic_hsl(color);
        if colors.len() >= 3 {
            (colors[0], colors[1], colors[2])
        } else {
            (color, color, color)
        }
    }

    fn name(&self) -> &'static str {
        "HSL"
    }
}

/// Lab-based color scheme strategy (perceptually uniform method)
pub struct LabColorSchemeStrategy;

impl ColorSchemeStrategy for LabColorSchemeStrategy {
    fn complementary(&self, color: Lab) -> Lab {
        let colors = ColorUtils::complementary_lab(color);
        if colors.len() >= 2 {
            colors[1] // Return the actual complementary color, not the original
        } else {
            color
        }
    }

    fn split_complementary(&self, color: Lab) -> (Lab, Lab) {
        let colors = ColorUtils::split_complementary_lab(color);
        if colors.len() >= 2 {
            (colors[0], colors[1])
        } else {
            (color, color)
        }
    }

    fn triadic(&self, color: Lab) -> (Lab, Lab) {
        let colors = ColorUtils::triadic_lab(color);
        if colors.len() >= 2 {
            (colors[0], colors[1])
        } else {
            (color, color)
        }
    }

    fn tetradic(&self, color: Lab) -> (Lab, Lab, Lab) {
        let colors = ColorUtils::tetradic_lab(color);
        if colors.len() >= 3 {
            (colors[0], colors[1], colors[2])
        } else {
            (color, color, color)
        }
    }

    fn name(&self) -> &'static str {
        "Lab"
    }
}

/// Builder for configuring color scheme calculations
pub struct ColorSchemeBuilder {
    preserve_relative_luminance: bool,
    preserve_lab_luminance: bool,
    target_relative_luminance: Option<f64>,
    target_lab_luminance: Option<f64>,
}

impl ColorSchemeBuilder {
    /// Create a new color scheme builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            preserve_relative_luminance: false,
            preserve_lab_luminance: false,
            target_relative_luminance: None,
            target_lab_luminance: None,
        }
    }

    /// Preserve relative luminance for color scheme calculations
    pub fn preserve_relative_luminance(mut self) -> Self {
        self.preserve_relative_luminance = true;
        self
    }

    /// Preserve Lab luminance for color scheme calculations
    pub fn preserve_lab_luminance(mut self) -> Self {
        self.preserve_lab_luminance = true;
        self
    }

    /// Set target relative luminance for replacement color
    #[must_use]
    pub fn with_target_relative_luminance(mut self, luminance: f64) -> Self {
        self.target_relative_luminance = Some(luminance);
        self
    }

    /// Set target Lab luminance for replacement color
    #[must_use]
    pub fn with_target_lab_luminance(mut self, luminance: f64) -> Self {
        self.target_lab_luminance = Some(luminance);
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

impl ColorSchemeCalculator {
    /// Calculate color schemes for the given color using both HSL and Lab strategies
    pub fn calculate(&self, mut base_color: Lab) -> Result<ColorSchemeResult> {
        // Apply color replacement if target luminance is specified
        if let Some(target_rel_lum) = self.target_relative_luminance {
            base_color = ColorUtils::adjust_color_relative_luminance(base_color, target_rel_lum)?;
        } else if let Some(target_lab_lum) = self.target_lab_luminance {
            base_color = adjust_color_lab_luminance(base_color, target_lab_lum)?;
        }

        // Create both strategies
        let hsl_strategy = HslColorSchemeStrategy;
        let lab_strategy = LabColorSchemeStrategy;

        // Calculate HSL-based color schemes
        let hsl_complementary = hsl_strategy.complementary(base_color);
        let hsl_split_complementary = hsl_strategy.split_complementary(base_color);
        let hsl_triadic = hsl_strategy.triadic(base_color);
        let hsl_tetradic = hsl_strategy.tetradic(base_color);

        // Calculate Lab-based color schemes
        let lab_complementary = lab_strategy.complementary(base_color);
        let lab_split_complementary = lab_strategy.split_complementary(base_color);
        let lab_triadic = lab_strategy.triadic(base_color);
        let lab_tetradic = lab_strategy.tetradic(base_color);

        // Calculate luminance-matched variations for HSL results if requested
        let luminance_matched_hsl_complementary = if self.preserve_relative_luminance {
            Some(preserve_wcag_relative_luminance(
                hsl_complementary,
                base_color,
            )?)
        } else if self.preserve_lab_luminance {
            Some(preserve_lab_luminance(hsl_complementary, base_color)?)
        } else {
            None
        };

        let luminance_matched_hsl_split_complementary = if self.preserve_relative_luminance {
            let comp1 = preserve_wcag_relative_luminance(hsl_split_complementary.0, base_color)?;
            let comp2 = preserve_wcag_relative_luminance(hsl_split_complementary.1, base_color)?;
            Some((comp1, comp2))
        } else if self.preserve_lab_luminance {
            let comp1 = preserve_lab_luminance(hsl_split_complementary.0, base_color)?;
            let comp2 = preserve_lab_luminance(hsl_split_complementary.1, base_color)?;
            Some((comp1, comp2))
        } else {
            None
        };

        let luminance_matched_hsl_triadic = if self.preserve_relative_luminance {
            let tri1 = preserve_wcag_relative_luminance(hsl_triadic.0, base_color)?;
            let tri2 = preserve_wcag_relative_luminance(hsl_triadic.1, base_color)?;
            Some((tri1, tri2))
        } else if self.preserve_lab_luminance {
            let tri1 = preserve_lab_luminance(hsl_triadic.0, base_color)?;
            let tri2 = preserve_lab_luminance(hsl_triadic.1, base_color)?;
            Some((tri1, tri2))
        } else {
            None
        };

        // Calculate luminance-matched variations for Lab results if requested
        let luminance_matched_lab_complementary = if self.preserve_relative_luminance {
            Some(preserve_wcag_relative_luminance(
                lab_complementary,
                base_color,
            )?)
        } else if self.preserve_lab_luminance {
            Some(preserve_lab_luminance(lab_complementary, base_color)?)
        } else {
            None
        };

        let luminance_matched_lab_split_complementary = if self.preserve_relative_luminance {
            let comp1 = preserve_wcag_relative_luminance(lab_split_complementary.0, base_color)?;
            let comp2 = preserve_wcag_relative_luminance(lab_split_complementary.1, base_color)?;
            Some((comp1, comp2))
        } else if self.preserve_lab_luminance {
            let comp1 = preserve_lab_luminance(lab_split_complementary.0, base_color)?;
            let comp2 = preserve_lab_luminance(lab_split_complementary.1, base_color)?;
            Some((comp1, comp2))
        } else {
            None
        };

        let luminance_matched_lab_triadic = if self.preserve_relative_luminance {
            let tri1 = preserve_wcag_relative_luminance(lab_triadic.0, base_color)?;
            let tri2 = preserve_wcag_relative_luminance(lab_triadic.1, base_color)?;
            Some((tri1, tri2))
        } else if self.preserve_lab_luminance {
            let tri1 = preserve_lab_luminance(lab_triadic.0, base_color)?;
            let tri2 = preserve_lab_luminance(lab_triadic.1, base_color)?;
            Some((tri1, tri2))
        } else {
            None
        };

        let luminance_matched_hsl_tetradic = if self.preserve_relative_luminance {
            let tet1 = preserve_wcag_relative_luminance(hsl_tetradic.0, base_color)?;
            let tet2 = preserve_wcag_relative_luminance(hsl_tetradic.1, base_color)?;
            let tet3 = preserve_wcag_relative_luminance(hsl_tetradic.2, base_color)?;
            Some((tet1, tet2, tet3))
        } else if self.preserve_lab_luminance {
            let tet1 = preserve_lab_luminance(hsl_tetradic.0, base_color)?;
            let tet2 = preserve_lab_luminance(hsl_tetradic.1, base_color)?;
            let tet3 = preserve_lab_luminance(hsl_tetradic.2, base_color)?;
            Some((tet1, tet2, tet3))
        } else {
            None
        };

        let luminance_matched_lab_tetradic = if self.preserve_relative_luminance {
            let tet1 = preserve_wcag_relative_luminance(lab_tetradic.0, base_color)?;
            let tet2 = preserve_wcag_relative_luminance(lab_tetradic.1, base_color)?;
            let tet3 = preserve_wcag_relative_luminance(lab_tetradic.2, base_color)?;
            Some((tet1, tet2, tet3))
        } else if self.preserve_lab_luminance {
            let tet1 = preserve_lab_luminance(lab_tetradic.0, base_color)?;
            let tet2 = preserve_lab_luminance(lab_tetradic.1, base_color)?;
            let tet3 = preserve_lab_luminance(lab_tetradic.2, base_color)?;
            Some((tet1, tet2, tet3))
        } else {
            None
        };

        Ok(ColorSchemeResult {
            base_color,
            hsl_complementary,
            hsl_split_complementary,
            hsl_triadic,
            hsl_tetradic,
            lab_complementary,
            lab_split_complementary,
            lab_triadic,
            lab_tetradic,
            luminance_matched_hsl_complementary,
            luminance_matched_hsl_split_complementary,
            luminance_matched_hsl_triadic,
            luminance_matched_hsl_tetradic,
            luminance_matched_lab_complementary,
            luminance_matched_lab_split_complementary,
            luminance_matched_lab_triadic,
            luminance_matched_lab_tetradic,
        })
    }
}

/// Adjust a color to have the specified Lab luminance while preserving a and b components.
/// Clamps the luminance to [0.0, 100.0] and returns an error if out of range.
pub fn adjust_color_lab_luminance(color: Lab, target_luminance: f64) -> Result<Lab> {
    if !(0.0..=100.0).contains(&target_luminance) {
        return Err(ColorError::InvalidArguments(format!(
            "Lab luminance must be in [0.0, 100.0], got {target_luminance}"
        )));
    }
    Ok(Lab::new(target_luminance as f32, color.a, color.b))
}

/// Preserve WCAG relative luminance from reference color in target color
pub fn preserve_wcag_relative_luminance(color: Lab, reference: Lab) -> Result<Lab> {
    let reference_srgb = ColorUtils::lab_to_srgb(reference);
    let target_luminance = ColorUtils::wcag_relative_luminance(reference_srgb);
    ColorUtils::adjust_color_relative_luminance(color, target_luminance)
}

/// Preserve Lab luminance from reference color in target color
pub fn preserve_lab_luminance(color: Lab, reference: Lab) -> Result<Lab> {
    adjust_color_lab_luminance(color, reference.l as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Srgb;

    #[test]
    fn test_hsl_complementary() {
        let strategy = HslColorSchemeStrategy;
        let red_lab = ColorUtils::srgb_to_lab(Srgb::new(1.0, 0.0, 0.0));
        let comp = strategy.complementary(red_lab);

        // Complementary of red should be cyan-ish
        let comp_srgb = ColorUtils::lab_to_srgb(comp);
        assert!(comp_srgb.blue > 0.5); // Should have significant blue component
        assert!(comp_srgb.red < 0.5); // Should have minimal red component
    }

    #[test]
    fn test_relative_luminance_adjustment() {
        let red_lab = ColorUtils::srgb_to_lab(Srgb::new(1.0, 0.0, 0.0));
        let adjusted = ColorUtils::adjust_color_relative_luminance(red_lab, 0.5).unwrap();

        let adjusted_srgb = ColorUtils::lab_to_srgb(adjusted);
        let actual_luminance = ColorUtils::wcag_relative_luminance(adjusted_srgb);

        // Should be close to target luminance
        assert!((actual_luminance - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_lab_luminance_adjustment() {
        let red_lab = ColorUtils::srgb_to_lab(Srgb::new(1.0, 0.0, 0.0));
        let adjusted = adjust_color_lab_luminance(red_lab, 75.0).unwrap();

        // Lab L component should be exactly 75.0
        assert!((adjusted.l - 75.0).abs() < 0.001);
        // a and b should be preserved
        assert!((adjusted.a - red_lab.a).abs() < 0.001);
        assert!((adjusted.b - red_lab.b).abs() < 0.001);
    }

    #[test]
    fn test_color_scheme_builder() {
        let calculator = ColorSchemeBuilder::new()
            .preserve_relative_luminance()
            .build();

        assert!(calculator.preserve_relative_luminance);
    }

    #[test]
    fn test_color_scheme_calculation() {
        let calculator = ColorSchemeBuilder::new().build();
        let red_lab = ColorUtils::srgb_to_lab(Srgb::new(1.0, 0.0, 0.0));

        let result = calculator.calculate(red_lab).unwrap();

        // Result should have both HSL and Lab color schemes calculated
        assert!(result.hsl_complementary != red_lab);
        assert!(result.hsl_split_complementary.0 != red_lab);
        assert!(result.hsl_triadic.0 != red_lab);
        assert!(result.lab_complementary != red_lab);
        assert!(result.lab_split_complementary.0 != red_lab);
        assert!(result.lab_triadic.0 != red_lab);
    }
}
