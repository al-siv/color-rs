//! Core analysis functions and data structures
//!
//! Provides comprehensive color analysis including properties, perception,
//! accessibility characteristics, and classification functions.

use crate::color_ops::{contrast, luminance};
use palette::Srgb;
use serde::{Deserialize, Serialize};

use super::conversions::{ColorSpaces, SerializableRgb, get_color_spaces};

/// Comprehensive color analysis result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorAnalysis {
    /// Original color being analyzed
    pub color: SerializableRgb,

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
    /// WCAG relative luminance (0.0 to 1.0)
    pub luminance: f64,

    /// Perceived brightness (0.0 to 100.0)
    pub brightness: f64,

    /// Whether color is considered light
    pub is_light: bool,

    /// Whether color is considered dark
    pub is_dark: bool,

    /// RGB tuple representation (0-255)
    pub rgb_tuple: (u8, u8, u8),

    /// Hexadecimal representation
    pub hex: String,
}

/// Perceptual color characteristics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerceptualData {
    /// Hue category (e.g., "Red", "Blue-Green")
    pub hue_category: String,

    /// Color temperature ("Warm", "Cool", "Neutral")
    pub temperature: String,

    /// Saturation level description
    pub saturation_level: String,

    /// Mood/emotion association
    pub mood: String,
}

/// Accessibility-related information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessibilityData {
    /// Text color recommendations for this background
    pub text_recommendations: TextRecommendations,

    /// WCAG compliance information
    pub wcag_info: WcagInfo,
}

/// Text color recommendations for accessibility
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextRecommendations {
    /// Best high contrast color (black or white)
    pub high_contrast: SerializableRgb,

    /// Contrast ratio with high contrast color
    pub high_contrast_ratio: f64,

    /// Alternative text colors that meet AA standards
    pub aa_compliant: Vec<SerializableRgb>,

    /// Alternative text colors that meet AAA standards
    pub aaa_compliant: Vec<SerializableRgb>,
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
        color: color.into(),
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

/// Analyze perceptual characteristics
fn analyze_perception(_color: Srgb, color_spaces: &ColorSpaces) -> PerceptualData {
    let hue_category = classify_hue(color_spaces.hsv.hue);
    let temperature = classify_temperature(color_spaces.hsv.hue);
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
#[allow(clippy::similar_names)] // AA and AAA are standard WCAG levels
fn get_text_recommendations(background: Srgb) -> TextRecommendations {
    let (high_contrast, high_contrast_ratio) = find_best_contrast_color(background);
    let aa_compliant_colors = generate_compliant_colors(background, 4.5);
    let aaa_compliant_colors = generate_compliant_colors(background, 7.0);

    TextRecommendations {
        high_contrast: high_contrast.into(),
        high_contrast_ratio,
        aa_compliant: aa_compliant_colors.into_iter().map(|c| c.into()).collect(),
        aaa_compliant: aaa_compliant_colors.into_iter().map(|c| c.into()).collect(),
    }
}

/// Find the best contrast color (black or white) against a background
fn find_best_contrast_color(background: Srgb) -> (Srgb, f64) {
    let white = Srgb::new(1.0, 1.0, 1.0);
    let black = Srgb::new(0.0, 0.0, 0.0);

    let white_ratio = contrast::wcag_ratio(white, background);
    let black_ratio = contrast::wcag_ratio(black, background);

    if white_ratio > black_ratio {
        (white, white_ratio)
    } else {
        (black, black_ratio)
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
fn get_wcag_info(_color: Srgb) -> WcagInfo {
    WcagInfo {
        relies_on_color_alone: false, // Should be determined by usage context
        min_luminance_diff_aa: 4.5,
        min_luminance_diff_aaa: 7.0,
    }
}

/// Classify hue into named categories
pub fn classify_hue(hue_degrees: f32) -> String {
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
pub fn classify_temperature(hue_degrees: f32) -> String {
    match hue_degrees {
        h if h >= 315.0 || h < 135.0 => "Warm".to_string(),
        h if h >= 135.0 && h < 225.0 => "Cool".to_string(),
        h if h >= 225.0 && h < 315.0 => "Cool".to_string(),
        _ => "Neutral".to_string(),
    }
}

/// Classify saturation level
pub fn classify_saturation(saturation: f32) -> String {
    match saturation {
        s if s < 0.2 => "Very Low".to_string(),
        s if s < 0.4 => "Low".to_string(),
        s if s < 0.6 => "Medium".to_string(),
        s if s < 0.8 => "High".to_string(),
        _ => "Very High".to_string(),
    }
}

/// Classify color mood
pub fn classify_mood(hue_category: &str, _temperature: &str, value: f32) -> String {
    let base_mood = match hue_category {
        "Red" | "Red-Orange" => "Energetic",
        "Orange" | "Yellow" => "Cheerful",
        "Green" | "Yellow-Green" => "Calming",
        "Blue" | "Cyan" => "Serene",
        "Blue-Violet" | "Violet" => "Mysterious",
        "Red-Violet" => "Passionate",
        _ => "Neutral",
    };

    let intensity = if value > 0.7 {
        "Bright"
    } else if value < 0.3 {
        "Dark"
    } else {
        "Medium"
    };

    format!("{} {}", intensity, base_mood)
}
