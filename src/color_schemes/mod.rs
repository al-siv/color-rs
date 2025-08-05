//! Color scheme calculation modules
//!
//! This module provides comprehensive color scheme and harmony calculations using
//! multiple color space strategies (HSL and Lab) with support for luminance preservation.
//! 
//! The module is organized into focused submodules:
//! - `algorithms`: Core calculation algorithms for color harmonies
//! - `strategies`: Strategy pattern implementations for different color spaces  
//! - `core`: Main calculator, builder pattern, and result structures

pub mod algorithms;
pub mod core;
pub mod strategies;

// Re-export main functionality for clean API
pub use algorithms::{
    complementary_hsl,
    split_complementary_hsl, 
    triadic_hsl,
    tetradic_hsl,
    complementary_lab,
    split_complementary_lab,
    triadic_lab,
    tetradic_lab,
    adjust_color_relative_luminance,
    adjust_color_lab_luminance,
    preserve_wcag_relative_luminance,
    preserve_lab_luminance,
};

pub use strategies::{
    ColorSchemeStrategy,
    HslColorSchemeStrategy,
    LabColorSchemeStrategy,
};

pub use core::{
    ColorSchemeBuilder,
    ColorSchemeCalculator,
    ColorSchemeResult,
};

#[cfg(test)]
mod integration_tests {
    use super::*;
    use palette::{IntoColor, Srgb, Lab};

    #[test]
    fn test_algorithm_integration() {
        // Test that algorithms work correctly through the re-exports
        let red_lab: Lab = Srgb::new(1.0, 0.0, 0.0).into_color();
        
        let comp_hsl = complementary_hsl(red_lab);
        let comp_lab = complementary_lab(red_lab);
        
        // Should get different results from different algorithms
        assert!(comp_hsl != comp_lab);
        assert!(comp_hsl != red_lab);
        assert!(comp_lab != red_lab);
    }

    #[test]
    fn test_strategy_integration() {
        // Test strategy pattern works through re-exports
        let hsl_strategy = HslColorSchemeStrategy;
        let lab_strategy = LabColorSchemeStrategy;
        
        let red_lab: Lab = Srgb::new(1.0, 0.0, 0.0).into_color();
        
        let hsl_comp = hsl_strategy.complementary(red_lab);
        let lab_comp = lab_strategy.complementary(red_lab);
        
        assert!(hsl_comp != lab_comp);
        assert_eq!(hsl_strategy.name(), "HSL");
        assert_eq!(lab_strategy.name(), "Lab");
    }

    #[test]
    fn test_core_integration() {
        // Test complete workflow through re-exports
        let calculator = ColorSchemeBuilder::new()
            .preserve_relative_luminance()
            .build();
            
        let red_lab: Lab = Srgb::new(1.0, 0.0, 0.0).into_color();
        let result = calculator.calculate(red_lab).unwrap();
        
        // Should have all basic schemes
        assert!(result.hsl_complementary != red_lab);
        assert!(result.lab_complementary != red_lab);
        assert!(result.hsl_triadic.0 != red_lab);
        assert!(result.lab_triadic.0 != red_lab);
        
        // Should have luminance-matched schemes
        assert!(result.luminance_matched_hsl_complementary.is_some());
        assert!(result.luminance_matched_lab_complementary.is_some());
    }

    #[test]
    fn test_luminance_preservation_integration() {
        // Test luminance preservation functions work correctly
        let red_lab: Lab = Srgb::new(1.0, 0.0, 0.0).into_color();
        let blue_lab: Lab = Srgb::new(0.0, 0.0, 1.0).into_color();
        
        // Test relative luminance preservation
        let preserved_relative = preserve_wcag_relative_luminance(blue_lab, red_lab).unwrap();
        let preserved_lab = preserve_lab_luminance(blue_lab, red_lab).unwrap();
        
        // Should get different results
        assert!(preserved_relative != blue_lab);
        assert!(preserved_lab != blue_lab);
        assert!(preserved_relative != preserved_lab);
        
        // Lab preservation should preserve luminance exactly
        assert!((preserved_lab.l - red_lab.l).abs() < 0.01);
    }

    #[test]
    fn test_full_workflow_integration() {
        // Test complete end-to-end workflow
        let red_lab: Lab = Srgb::new(1.0, 0.0, 0.0).into_color();
        
        // Test direct algorithm usage
        let direct_comp = complementary_hsl(red_lab);
        
        // Test strategy usage
        let strategy = HslColorSchemeStrategy;
        let strategy_comp = strategy.complementary(red_lab);
        
        // Should get same result
        assert_eq!(direct_comp, strategy_comp);
        
        // Test full calculator
        let calculator = ColorSchemeCalculator::new();
        let result = calculator.calculate(red_lab).unwrap();
        
        // Should match strategy result
        assert_eq!(result.hsl_complementary, strategy_comp);
    }
}
