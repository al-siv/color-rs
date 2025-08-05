//! Color analysis module providing comprehensive color analysis and comparison
//!
//! This module has been decomposed into focused submodules following functional
//! programming principles and single responsibility design.
//!
//! ## Submodule Organization
//! - `conversions` - Type conversion logic and serializable color representations
//! - `core` - Core analysis functions and main logic 
//! - `formatting` - Result formatting and comparison functions
//!
//! ## Main Functions
//! - `analyze_color()` - Comprehensive color analysis
//! - `compare_colors()` - Detailed color comparison
//!
//! ## Example Usage
//! ```rust
//! use color_rs::color_ops::analysis;
//! use palette::Srgb;
//!
//! let red = Srgb::new(1.0, 0.0, 0.0);
//! let analysis = analysis::analyze_color(red);
//! 
//! let blue = Srgb::new(0.0, 0.0, 1.0);
//! let comparison = analysis::compare_colors(red, blue);
//! ```

pub mod conversions;
pub mod core;
pub mod formatting;

// Re-export main functions for backward compatibility
pub use core::{
    analyze_color,
    classify_hue,
    classify_temperature, 
    classify_saturation,
    classify_mood,
};

pub use formatting::{
    compare_colors,
};

// Re-export all types for public API
pub use conversions::{
    SerializableRgb,
    SerializableHsl, 
    SerializableHsv,
    SerializableLab,
    SerializableLch,
    ColorSpaces,
    get_color_spaces,
};

pub use core::{
    ColorAnalysis,
    ColorProperties,
    PerceptualData,
    AccessibilityData,
    TextRecommendations,
    WcagInfo,
};

pub use formatting::{
    ColorComparison,
    DistanceMetrics,
};

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Srgb;

    #[test]
    fn test_analyze_color_red() {
        let red = Srgb::new(1.0, 0.0, 0.0);
        let analysis = analyze_color(red);
        
        assert_eq!(analysis.color, red.into());
        assert!(analysis.properties.is_dark);
        assert_eq!(analysis.perception.hue_category, "Red");
        assert_eq!(analysis.perception.temperature, "Warm");
        assert_eq!(analysis.properties.hex, "#FF0000");
    }

    #[test]
    fn test_analyze_color_white() {
        let white = Srgb::new(1.0, 1.0, 1.0);
        let analysis = analyze_color(white);
        
        assert!(analysis.properties.is_light);
        assert!((analysis.properties.luminance - 1.0).abs() < 1e-6);
        assert!(analysis.properties.brightness > 95.0);
        assert_eq!(analysis.properties.rgb_tuple, (255, 255, 255));
    }

    #[test]
    fn test_hue_classification() {
        assert_eq!(classify_hue(0.0), "Red");
        assert_eq!(classify_hue(60.0), "Orange");
        assert_eq!(classify_hue(120.0), "Yellow-Green");
        assert_eq!(classify_hue(180.0), "Blue-Green");  // Corrected: 180.0 is in Blue-Green range (165-195)
        assert_eq!(classify_hue(210.0), "Cyan");       // Use 210.0 for Cyan range (195-225)
        assert_eq!(classify_hue(240.0), "Blue");
        assert_eq!(classify_hue(300.0), "Violet");
    }

    #[test]
    fn test_temperature_classification() {
        assert_eq!(classify_temperature(0.0), "Warm");    // Red
        assert_eq!(classify_temperature(60.0), "Warm");   // Orange
        assert_eq!(classify_temperature(180.0), "Cool");  // Cyan
        assert_eq!(classify_temperature(240.0), "Cool");  // Blue
    }

    #[test]
    fn test_saturation_classification() {
        assert_eq!(classify_saturation(0.1), "Very Low");
        assert_eq!(classify_saturation(0.3), "Low");
        assert_eq!(classify_saturation(0.5), "Medium");
        assert_eq!(classify_saturation(0.7), "High");
        assert_eq!(classify_saturation(0.9), "Very High");
    }

    #[test]
    fn test_compare_colors() {
        let red = Srgb::new(1.0, 0.0, 0.0);
        let blue = Srgb::new(0.0, 0.0, 1.0);
        
        let comparison = compare_colors(red, blue);
        
        // Red to Blue has Delta E 2000 of approximately 23
        assert!(comparison.distance_metrics.delta_e_2000 > 20.0);
        assert_eq!(comparison.perceptual_similarity, "Extremely Different");  // Updated to match improved algorithm
        assert!(comparison.contrast_ratio > 1.0);
    }

    #[test]
    fn test_text_recommendations() {
        let dark_bg = Srgb::new(0.1, 0.1, 0.1);
        let analysis = analyze_color(dark_bg);
        
        // Dark background should recommend white text
        let white = Srgb::new(1.0, 1.0, 1.0);
        assert_eq!(analysis.accessibility.text_recommendations.high_contrast, white.into());
        assert!(analysis.accessibility.text_recommendations.high_contrast_ratio > 10.0);
    }

    #[test]
    fn test_submodule_integration() {
        let color = Srgb::new(0.5, 0.3, 0.8);
        
        // Test conversions submodule
        let spaces = conversions::get_color_spaces(color);
        assert!(spaces.hsv.hue >= 0.0);
        assert!(spaces.hsv.saturation >= 0.0);
        
        // Test core submodule
        let analysis = core::analyze_color(color);
        assert_eq!(analysis.color, color.into());
        
        // Test formatting submodule
        let comparison = formatting::compare_colors(color, Srgb::new(1.0, 1.0, 1.0));
        assert!(comparison.distance_metrics.delta_e_2000 > 0.0);
    }

    #[test]
    fn test_api_backward_compatibility() {
        let red = Srgb::new(1.0, 0.0, 0.0);
        let blue = Srgb::new(0.0, 0.0, 1.0);
        
        // These calls should work exactly as before decomposition
        let _analysis = analyze_color(red);
        let _comparison = compare_colors(red, blue);
        let _hue = classify_hue(240.0);
        let _temp = classify_temperature(180.0);
        let _sat = classify_saturation(0.5);
    }
}
