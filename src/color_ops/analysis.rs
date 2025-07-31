//! Color analysis functions
//!
//! Pure functions for analyzing color properties and characteristics.
//! Provides unified analysis structure replacing facade pattern analysis.

use crate::color_ops::{contrast, distance, luminance};
use crate::color_utils::LegacyColorUtils as ColorUtils;
use palette::{Hsl, Hsv, Lab, Lch, Srgb};
use serde::{Deserialize, Serialize};

/// Comprehensive color analysis result
///
/// Unified structure containing all color analysis data.
/// Replaces the facade pattern with a functional approach.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorAnalysis {
    /// Original color in sRGB space
    pub color: Srgb,
    
    /// Basic color properties
    pub properties: ColorProperties,
    
    /// Color space representations
    pub color_spaces: ColorSpaces,
    
    /// Perceptual characteristics
    pub perception: PerceptualData,
    
    /// Accessibility information
    pub accessibility: AccessibilityData,
}

/// Basic color properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorProperties {
    /// WCAG relative luminance (0.0-1.0)
    pub luminance: f64,
    
    /// Perceived brightness using LAB L* (0.0-100.0)
    pub brightness: f64,
    
    /// Whether color is considered "light" (luminance > 0.5)
    pub is_light: bool,
    
    /// Whether color is considered "dark" (luminance < 0.5)
    pub is_dark: bool,
    
    /// RGB values as (u8, u8, u8) tuple
    pub rgb_tuple: (u8, u8, u8),
    
    /// Hex representation (#RRGGBB)
    pub hex: String,
}

/// Color space representations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorSpaces {
    /// HSL (Hue, Saturation, Lightness)
    pub hsl: Hsl,
    
    /// HSV (Hue, Saturation, Value)
    pub hsv: Hsv,
    
    /// CIELAB (L*, a*, b*)
    pub lab: Lab,
    
    /// LCH (Lightness, Chroma, Hue)
    pub lch: Lch,
}

/// Perceptual characteristics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerceptualData {
    /// Dominant hue category
    pub hue_category: String,
    
    /// Color temperature classification
    pub temperature: String,
    
    /// Saturation level classification
    pub saturation_level: String,
    
    /// Overall color mood/feeling
    pub mood: String,
}

/// Accessibility information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessibilityData {
    /// Suitable text colors for this background
    pub text_recommendations: TextRecommendations,
    
    /// WCAG compliance information
    pub wcag_info: WcagInfo,
}

/// Text color recommendations for accessibility
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextRecommendations {
    /// Best high-contrast text color (black or white)
    pub high_contrast: Srgb,
    
    /// Contrast ratio with high-contrast text
    pub high_contrast_ratio: f64,
    
    /// Alternative text colors that meet AA standards
    pub aa_compliant: Vec<Srgb>,
    
    /// Alternative text colors that meet AAA standards
    pub aaa_compliant: Vec<Srgb>,
}

/// WCAG compliance information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WcagInfo {
    /// Whether color alone can convey information (should be false)
    pub relies_on_color_alone: bool,
    
    /// Minimum luminance difference needed for AA compliance
    pub min_luminance_diff_aa: f64,
    
    /// Minimum luminance difference needed for AAA compliance
    pub min_luminance_diff_aaa: f64,
}

/// Analyze a color comprehensively
///
/// Performs complete color analysis including properties, color spaces,
/// perception, and accessibility information.
///
/// # Arguments
/// * `color` - Color to analyze in sRGB space
///
/// # Returns
/// * Comprehensive `ColorAnalysis` structure
///
/// # Example
/// ```rust
/// use color_rs::color_ops::analysis;
/// use palette::Srgb;
///
/// let red = Srgb::new(1.0, 0.0, 0.0);
/// let analysis = analysis::analyze_color(red);
/// 
/// assert!(analysis.properties.is_dark);
/// assert_eq!(analysis.perception.hue_category, "Red");
/// ```
pub fn analyze_color(color: Srgb) -> ColorAnalysis {
    let properties = analyze_properties(color);
    let color_spaces = get_color_spaces(color);
    let perception = analyze_perception(color, &color_spaces);
    let accessibility = analyze_accessibility(color);
    
    ColorAnalysis {
        color,
        properties,
        color_spaces,
        perception,
        accessibility,
    }
}

/// Analyze basic color properties
fn analyze_properties(color: Srgb) -> ColorProperties {
    let luminance_val = luminance::wcag_relative(color);
    let brightness = luminance::perceived_brightness(color);
    let is_light = luminance_val > 0.5;
    let is_dark = !is_light;
    
    let rgb_tuple = crate::color_ops::conversion::srgb_to_rgb_tuple(color);
    let hex = crate::color_ops::conversion::srgb_to_hex(color);
    
    ColorProperties {
        luminance: luminance_val,
        brightness,
        is_light,
        is_dark,
        rgb_tuple,
        hex,
    }
}

/// Get color space representations
fn get_color_spaces(color: Srgb) -> ColorSpaces {
    use crate::color_ops::conversion;
    
    ColorSpaces {
        hsl: conversion::srgb_to_hsl(color),
        hsv: conversion::srgb_to_hsv(color),
        lab: conversion::srgb_to_lab(color),
        lch: conversion::srgb_to_lch(color),
    }
}

/// Analyze perceptual characteristics
fn analyze_perception(color: Srgb, color_spaces: &ColorSpaces) -> PerceptualData {
    let hue_category = classify_hue(color_spaces.hsv.hue.into_inner());
    let temperature = classify_temperature(color_spaces.hsv.hue.into_inner());
    let saturation_level = classify_saturation(color_spaces.hsv.saturation);
    let mood = classify_mood(&hue_category, &temperature, color_spaces.hsv.value);
    
    PerceptualData {
        hue_category,
        temperature,
        saturation_level,
        mood,
    }
}

/// Analyze accessibility characteristics
fn analyze_accessibility(color: Srgb) -> AccessibilityData {
    let text_recommendations = get_text_recommendations(color);
    let wcag_info = get_wcag_info(color);
    
    AccessibilityData {
        text_recommendations,
        wcag_info,
    }
}

/// Get text color recommendations for a background color
fn get_text_recommendations(background: Srgb) -> TextRecommendations {
    let white = Srgb::new(1.0, 1.0, 1.0);
    let black = Srgb::new(0.0, 0.0, 0.0);
    
    let white_ratio = contrast::wcag_ratio(white, background);
    let black_ratio = contrast::wcag_ratio(black, background);
    
    let (high_contrast, high_contrast_ratio) = if white_ratio > black_ratio {
        (white, white_ratio)
    } else {
        (black, black_ratio)
    };
    
    // Generate some alternative colors that meet standards
    let aa_compliant = generate_compliant_colors(background, 4.5);
    let aaa_compliant = generate_compliant_colors(background, 7.0);
    
    TextRecommendations {
        high_contrast,
        high_contrast_ratio,
        aa_compliant,
        aaa_compliant,
    }
}

/// Generate colors that meet contrast requirements
fn generate_compliant_colors(background: Srgb, min_ratio: f64) -> Vec<Srgb> {
    let mut compliant = Vec::new();
    
    // Test grayscale colors
    for lightness in [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0] {
        let test_color = Srgb::new(lightness, lightness, lightness);
        if contrast::wcag_ratio(test_color, background) >= min_ratio {
            compliant.push(test_color);
        }
    }
    
    compliant
}

/// Get WCAG compliance information
fn get_wcag_info(color: Srgb) -> WcagInfo {
    WcagInfo {
        relies_on_color_alone: false, // Should be determined by usage context
        min_luminance_diff_aa: 4.5,
        min_luminance_diff_aaa: 7.0,
    }
}

/// Classify hue into named categories
fn classify_hue(hue_degrees: f32) -> String {
    match hue_degrees {
        h if h >= 345.0 || h < 15.0 => "Red".to_string(),
        h if h >= 15.0 && h < 45.0 => "Red-Orange".to_string(),
        h if h >= 45.0 && h < 75.0 => "Orange".to_string(),
        h if h >= 75.0 && h < 105.0 => "Yellow".to_string(),
        h if h >= 105.0 && h < 135.0 => "Yellow-Green".to_string(),
        h if h >= 135.0 && h < 165.0 => "Green".to_string(),
        h if h >= 165.0 && h < 195.0 => "Blue-Green".to_string(),
        h if h >= 195.0 && h < 225.0 => "Cyan".to_string(),
        h if h >= 225.0 && h < 255.0 => "Blue".to_string(),
        h if h >= 255.0 && h < 285.0 => "Blue-Violet".to_string(),
        h if h >= 285.0 && h < 315.0 => "Violet".to_string(),
        h if h >= 315.0 && h < 345.0 => "Red-Violet".to_string(),
        _ => "Unknown".to_string(),
    }
}

/// Classify color temperature
fn classify_temperature(hue_degrees: f32) -> String {
    match hue_degrees {
        h if h >= 315.0 || h < 135.0 => "Warm".to_string(),
        h if h >= 135.0 && h < 225.0 => "Cool".to_string(),
        h if h >= 225.0 && h < 315.0 => "Cool".to_string(),
        _ => "Neutral".to_string(),
    }
}

/// Classify saturation level
fn classify_saturation(saturation: f32) -> String {
    match saturation {
        s if s < 0.2 => "Very Low".to_string(),
        s if s < 0.4 => "Low".to_string(),
        s if s < 0.6 => "Medium".to_string(),
        s if s < 0.8 => "High".to_string(),
        _ => "Very High".to_string(),
    }
}

/// Classify color mood
fn classify_mood(hue_category: &str, temperature: &str, value: f32) -> String {
    let base_mood = match hue_category {
        "Red" | "Red-Orange" => "Energetic",
        "Orange" | "Yellow" => "Cheerful",
        "Green" | "Yellow-Green" => "Calming",
        "Blue" | "Cyan" => "Serene",
        "Blue-Violet" | "Violet" => "Mysterious",
        "Red-Violet" => "Passionate",
        _ => "Neutral",
    };
    
    let intensity = if value > 0.7 { "Bright" } else if value < 0.3 { "Dark" } else { "Medium" };
    
    format!("{} {}", intensity, base_mood)
}

/// Compare two colors and return detailed comparison
///
/// # Arguments
/// * `color1` - First color to compare
/// * `color2` - Second color to compare
///
/// # Returns
/// * Detailed color comparison structure
pub fn compare_colors(color1: Srgb, color2: Srgb) -> ColorComparison {
    let analysis1 = analyze_color(color1);
    let analysis2 = analyze_color(color2);
    
    ColorComparison {
        color1: analysis1,
        color2: analysis2,
        distance_metrics: DistanceMetrics {
            delta_e_2000: distance::delta_e_2000(color1, color2),
            delta_e_cie94: distance::delta_e_cie94(color1, color2),
            delta_e_cie76: distance::delta_e_cie76(color1, color2),
            rgb_euclidean: distance::rgb_euclidean(color1, color2),
            lab_euclidean: distance::lab_euclidean(color1, color2),
        },
        contrast_ratio: contrast::wcag_ratio(color1, color2),
        perceptual_similarity: classify_similarity(color1, color2),
    }
}

/// Color comparison result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorComparison {
    pub color1: ColorAnalysis,
    pub color2: ColorAnalysis,
    pub distance_metrics: DistanceMetrics,
    pub contrast_ratio: f64,
    pub perceptual_similarity: String,
}

/// Distance metrics between colors
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistanceMetrics {
    pub delta_e_2000: f64,
    pub delta_e_cie94: f64,
    pub delta_e_cie76: f64,
    pub rgb_euclidean: f64,
    pub lab_euclidean: f64,
}

/// Classify perceptual similarity between colors
fn classify_similarity(color1: Srgb, color2: Srgb) -> String {
    let delta_e = distance::delta_e_2000(color1, color2);
    
    match delta_e {
        d if d < 1.0 => "Identical".to_string(),
        d if d < 2.3 => "Just Noticeable".to_string(),
        d if d < 5.0 => "Perceptible".to_string(),
        d if d < 10.0 => "Noticeable".to_string(),
        d if d < 20.0 => "Different".to_string(),
        d if d < 40.0 => "Very Different".to_string(),
        _ => "Extremely Different".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Srgb;

    #[test]
    fn test_analyze_color_red() {
        let red = Srgb::new(1.0, 0.0, 0.0);
        let analysis = analyze_color(red);
        
        assert_eq!(analysis.color, red);
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
        assert_eq!(classify_hue(180.0), "Cyan");
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
        
        assert!(comparison.distance_metrics.delta_e_2000 > 50.0);
        assert_eq!(comparison.perceptual_similarity, "Extremely Different");
        assert!(comparison.contrast_ratio > 1.0);
    }

    #[test]
    fn test_text_recommendations() {
        let dark_bg = Srgb::new(0.1, 0.1, 0.1);
        let analysis = analyze_color(dark_bg);
        
        // Dark background should recommend white text
        let white = Srgb::new(1.0, 1.0, 1.0);
        assert_eq!(analysis.accessibility.text_recommendations.high_contrast, white);
        assert!(analysis.accessibility.text_recommendations.high_contrast_ratio > 10.0);
    }
}
