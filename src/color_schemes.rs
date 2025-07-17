//! Color harmony and color scheme calculations using traditional color theory
//!
//! This module provides functionality for calculating color harmonies such as
//! complementary, split-complementary, and triadic colors using both HSV and
//! Lab color spaces, with support for luminance matching.

use crate::error::{ColorError, Result};
use crate::color_utils::ColorUtils;
use palette::{
    color_theory::{Complementary, SplitComplementary, Triadic},
    Hsl, Lab, Srgb, IntoColor, FromColor,
};

/// Strategy trait for different color scheme calculation methods
pub trait ColorSchemeStrategy {
    /// Calculate complementary color
    fn complementary(&self, color: Lab) -> Lab;
    
    /// Calculate split-complementary colors
    fn split_complementary(&self, color: Lab) -> (Lab, Lab);
    
    /// Calculate triadic colors  
    fn triadic(&self, color: Lab) -> (Lab, Lab);
    
    /// Get the name of this strategy
    fn name(&self) -> &'static str;
}

/// HSL-based color scheme strategy (default method)
pub struct HslColorSchemeStrategy;

impl ColorSchemeStrategy for HslColorSchemeStrategy {
    fn complementary(&self, color: Lab) -> Lab {
        let hsl: Hsl = color.into_color();
        let comp_hsl = hsl.complementary();
        comp_hsl.into_color()
    }
    
    fn split_complementary(&self, color: Lab) -> (Lab, Lab) {
        let hsl: Hsl = color.into_color();
        let (comp1, comp2) = hsl.split_complementary();
        (comp1.into_color(), comp2.into_color())
    }
    
    fn triadic(&self, color: Lab) -> (Lab, Lab) {
        let hsl: Hsl = color.into_color();
        let (tri1, tri2) = hsl.triadic();
        (tri1.into_color(), tri2.into_color())
    }
    
    fn name(&self) -> &'static str {
        "HSL"
    }
}

/// Lab-based color scheme strategy (perceptually uniform method)
pub struct LabColorSchemeStrategy;

impl ColorSchemeStrategy for LabColorSchemeStrategy {
    fn complementary(&self, color: Lab) -> Lab {
        // For Lab space, we work with LCh (polar Lab) for hue manipulation
        let lch = palette::Lch::from_color(color);
        let comp_lch = lch.complementary();
        Lab::from_color(comp_lch)
    }
    
    fn split_complementary(&self, color: Lab) -> (Lab, Lab) {
        let lch = palette::Lch::from_color(color);
        let (comp1, comp2) = lch.split_complementary();
        (Lab::from_color(comp1), Lab::from_color(comp2))
    }
    
    fn triadic(&self, color: Lab) -> (Lab, Lab) {
        let lch = palette::Lch::from_color(color);
        let (tri1, tri2) = lch.triadic();
        (Lab::from_color(tri1), Lab::from_color(tri2))
    }
    
    fn name(&self) -> &'static str {
        "Lab"
    }
}

/// Builder for configuring color scheme calculations
pub struct ColorSchemeBuilder {
    strategy: Box<dyn ColorSchemeStrategy>,
    preserve_relative_luminance: bool,
    preserve_lab_luminance: bool,
    target_relative_luminance: Option<f64>,
    target_lab_luminance: Option<f64>,
    use_lab_output: bool,
}

impl ColorSchemeBuilder {
    /// Create a new color scheme builder with HSL strategy (default)
    pub fn new() -> Self {
        Self {
            strategy: Box::new(HslColorSchemeStrategy),
            preserve_relative_luminance: false,
            preserve_lab_luminance: false,
            target_relative_luminance: None,
            target_lab_luminance: None,
            use_lab_output: false,
        }
    }
    
    /// Use Lab color space for calculations
    pub fn with_lab_strategy(mut self) -> Self {
        self.strategy = Box::new(LabColorSchemeStrategy);
        self
    }
    
    /// Use HSL color space for calculations (default)
    pub fn with_hsl_strategy(mut self) -> Self {
        self.strategy = Box::new(HslColorSchemeStrategy);
        self
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
    pub fn with_target_relative_luminance(mut self, luminance: f64) -> Self {
        self.target_relative_luminance = Some(luminance);
        self
    }
    
    /// Set target Lab luminance for replacement color
    pub fn with_target_lab_luminance(mut self, luminance: f64) -> Self {
        self.target_lab_luminance = Some(luminance);
        self
    }
    
    /// Enable Lab output format for color display
    pub fn with_lab_output(mut self) -> Self {
        self.use_lab_output = true;
        self
    }
    
    /// Build the color scheme calculator
    pub fn build(self) -> ColorSchemeCalculator {
        ColorSchemeCalculator {
            strategy: self.strategy,
            preserve_relative_luminance: self.preserve_relative_luminance,
            preserve_lab_luminance: self.preserve_lab_luminance,
            target_relative_luminance: self.target_relative_luminance,
            target_lab_luminance: self.target_lab_luminance,
            use_lab_output: self.use_lab_output,
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
    strategy: Box<dyn ColorSchemeStrategy>,
    preserve_relative_luminance: bool,
    preserve_lab_luminance: bool,
    target_relative_luminance: Option<f64>,
    target_lab_luminance: Option<f64>,
    use_lab_output: bool,
}

/// Result of color scheme calculations
#[derive(Debug, Clone)]
pub struct ColorSchemeResult {
    pub base_color: Lab,
    pub complementary: Lab,
    pub split_complementary: (Lab, Lab),
    pub triadic: (Lab, Lab),
    pub strategy_name: String,
    pub luminance_matched_complementary: Option<Lab>,
    pub luminance_matched_split_complementary: Option<(Lab, Lab)>,
    pub luminance_matched_triadic: Option<(Lab, Lab)>,
    /// Whether to use Lab color format in output (true) or HSL format (false)
    pub use_lab_output: bool,
}

impl ColorSchemeCalculator {
    /// Calculate color schemes for the given color
    pub fn calculate(&self, mut base_color: Lab) -> Result<ColorSchemeResult> {
        // Apply color replacement if target luminance is specified
        if let Some(target_rel_lum) = self.target_relative_luminance {
            base_color = ColorUtils::adjust_color_relative_luminance(base_color, target_rel_lum)?;
        } else if let Some(target_lab_lum) = self.target_lab_luminance {
            base_color = adjust_color_lab_luminance(base_color, target_lab_lum)?;
        }
        
        // Calculate base color schemes
        let complementary = self.strategy.complementary(base_color);
        let split_complementary = self.strategy.split_complementary(base_color);
        let triadic = self.strategy.triadic(base_color);
        
        // Calculate luminance-matched variations if requested
        let luminance_matched_complementary = if self.preserve_relative_luminance {
            Some(preserve_wcag_relative_luminance(complementary, base_color)?)
        } else if self.preserve_lab_luminance {
            Some(preserve_lab_luminance(complementary, base_color)?)
        } else {
            None
        };
        
        let luminance_matched_split_complementary = if self.preserve_relative_luminance {
            let comp1 = preserve_wcag_relative_luminance(split_complementary.0, base_color)?;
            let comp2 = preserve_wcag_relative_luminance(split_complementary.1, base_color)?;
            Some((comp1, comp2))
        } else if self.preserve_lab_luminance {
            let comp1 = preserve_lab_luminance(split_complementary.0, base_color)?;
            let comp2 = preserve_lab_luminance(split_complementary.1, base_color)?;
            Some((comp1, comp2))
        } else {
            None
        };
        
        let luminance_matched_triadic = if self.preserve_relative_luminance {
            let tri1 = preserve_wcag_relative_luminance(triadic.0, base_color)?;
            let tri2 = preserve_wcag_relative_luminance(triadic.1, base_color)?;
            Some((tri1, tri2))
        } else if self.preserve_lab_luminance {
            let tri1 = preserve_lab_luminance(triadic.0, base_color)?;
            let tri2 = preserve_lab_luminance(triadic.1, base_color)?;
            Some((tri1, tri2))
        } else {
            None
        };
        
        Ok(ColorSchemeResult {
            base_color,
            complementary,
            split_complementary,
            triadic,
            strategy_name: self.strategy.name().to_string(),
            luminance_matched_complementary,
            luminance_matched_split_complementary,
            luminance_matched_triadic,
            use_lab_output: self.use_lab_output,
        })
    }
}

/// Adjust a color to have the specified Lab luminance while preserving hue
pub fn adjust_color_lab_luminance(color: Lab, target_luminance: f64) -> Result<Lab> {
    if target_luminance < 0.0 || target_luminance > 100.0 {
        return Err(ColorError::InvalidArguments(
            "Lab luminance should typically be between 0.0 and 100.0".to_string(),
        ));
    }
    
    // Simply adjust the L component while preserving a and b
    Ok(Lab::new(target_luminance as f32, color.a, color.b))
}

/// Preserve WCAG relative luminance from reference color in target color
pub fn preserve_wcag_relative_luminance(color: Lab, reference: Lab) -> Result<Lab> {
    let reference_srgb: Srgb = reference.into_color();
    let target_luminance = ColorUtils::wcag_relative_luminance_from_srgb(reference_srgb);
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
        let red_lab = Lab::from_color(Srgb::new(1.0, 0.0, 0.0));
        let comp = strategy.complementary(red_lab);
        
        // Complementary of red should be cyan-ish
        let comp_srgb: Srgb = comp.into_color();
        assert!(comp_srgb.blue > 0.5); // Should have significant blue component
        assert!(comp_srgb.red < 0.5);  // Should have minimal red component
    }

    #[test]
    fn test_relative_luminance_adjustment() {
        let red_lab = Lab::from_color(Srgb::new(1.0, 0.0, 0.0));
        let adjusted = ColorUtils::adjust_color_relative_luminance(red_lab, 0.5).unwrap();
        
        let adjusted_srgb: Srgb = adjusted.into_color();
        let actual_luminance = ColorUtils::wcag_relative_luminance_from_srgb(adjusted_srgb);
        
        // Should be close to target luminance
        assert!((actual_luminance - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_lab_luminance_adjustment() {
        let red_lab = Lab::from_color(Srgb::new(1.0, 0.0, 0.0));
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
            .with_lab_strategy()
            .preserve_relative_luminance()
            .build();
        
        assert_eq!(calculator.strategy.name(), "Lab");
        assert!(calculator.preserve_relative_luminance);
    }

    #[test]
    fn test_color_scheme_calculation() {
        let calculator = ColorSchemeBuilder::new().build();
        let red_lab = Lab::from_color(Srgb::new(1.0, 0.0, 0.0));
        
        let result = calculator.calculate(red_lab).unwrap();
        
        assert_eq!(result.strategy_name, "HSL");
        // Result should have base color schemes calculated
        assert!(result.complementary != red_lab);
        assert!(result.split_complementary.0 != red_lab);
        assert!(result.triadic.0 != red_lab);
    }
}
