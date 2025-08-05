//! Color scheme strategy implementations
//!
//! This module provides strategy pattern implementations for different color space
//! approaches to color harmony calculations. Supports both HSL and Lab color spaces.

use palette::Lab;
use super::algorithms::*;

/// Trait for color scheme calculation strategies
pub trait ColorSchemeStrategy {
    /// Calculate complementary color
    fn complementary(&self, color: Lab) -> Lab;
    
    /// Calculate split-complementary colors
    fn split_complementary(&self, color: Lab) -> (Lab, Lab);
    
    /// Calculate triadic colors
    fn triadic(&self, color: Lab) -> (Lab, Lab);
    
    /// Calculate tetradic colors
    fn tetradic(&self, color: Lab) -> (Lab, Lab, Lab);
    
    /// Get the name of this strategy
    fn name(&self) -> &'static str;
}

/// HSL-based color scheme strategy
pub struct HslColorSchemeStrategy;

impl ColorSchemeStrategy for HslColorSchemeStrategy {
    fn complementary(&self, color: Lab) -> Lab {
        complementary_hsl(color)
    }

    fn split_complementary(&self, color: Lab) -> (Lab, Lab) {
        split_complementary_hsl(color)
    }

    fn triadic(&self, color: Lab) -> (Lab, Lab) {
        triadic_hsl(color)
    }

    fn tetradic(&self, color: Lab) -> (Lab, Lab, Lab) {
        tetradic_hsl(color)
    }

    fn name(&self) -> &'static str {
        "HSL"
    }
}

/// Lab-based color scheme strategy
pub struct LabColorSchemeStrategy;

impl ColorSchemeStrategy for LabColorSchemeStrategy {
    fn complementary(&self, color: Lab) -> Lab {
        complementary_lab(color)
    }

    fn split_complementary(&self, color: Lab) -> (Lab, Lab) {
        split_complementary_lab(color)
    }

    fn triadic(&self, color: Lab) -> (Lab, Lab) {
        triadic_lab(color)
    }

    fn tetradic(&self, color: Lab) -> (Lab, Lab, Lab) {
        tetradic_lab(color)
    }

    fn name(&self) -> &'static str {
        "Lab"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::{IntoColor, Srgb};

    #[test]
    fn test_hsl_strategy() {
        let strategy = HslColorSchemeStrategy;
        let red_lab: Lab = Srgb::new(1.0, 0.0, 0.0).into_color();
        let comp = strategy.complementary(red_lab);

        // Complementary of red should be cyan-ish
        let comp_srgb: Srgb = comp.into_color();
        assert!(comp_srgb.blue > 0.5); // Should have significant blue component
        assert!(comp_srgb.red < 0.5); // Should have minimal red component
        assert_eq!(strategy.name(), "HSL");
    }

    #[test]
    fn test_lab_strategy() {
        let strategy = LabColorSchemeStrategy;
        let red_lab: Lab = Srgb::new(1.0, 0.0, 0.0).into_color();
        let comp = strategy.complementary(red_lab);

        // In Lab space, complementary should be different
        assert!(comp != red_lab);
        assert_eq!(strategy.name(), "Lab");
    }

    #[test]
    fn test_strategy_consistency() {
        let hsl_strategy = HslColorSchemeStrategy;
        let lab_strategy = LabColorSchemeStrategy;
        
        let red_lab: Lab = Srgb::new(1.0, 0.0, 0.0).into_color();
        
        // Both strategies should produce results
        let hsl_triadic = hsl_strategy.triadic(red_lab);
        let lab_triadic = lab_strategy.triadic(red_lab);
        
        // Results should be different (different color spaces)
        assert!(hsl_triadic.0 != lab_triadic.0);
        assert!(hsl_triadic.1 != lab_triadic.1);
    }
}
