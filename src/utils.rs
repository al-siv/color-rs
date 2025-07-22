//! Utility functions for color-rs

use crate::config::*;
use crate::error::{ColorError, Result};

/// Utility functions for various operations
pub struct Utils;

impl Utils {
    /// Clamp a value between min and max
    pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }

    /// Round a float to a specified number of decimal places
    pub fn round_to_decimals(value: f64, decimals: u32) -> f64 {
        let multiplier = 10f64.powi(decimals as i32);
        (value * multiplier).round() / multiplier
    }

    /// Validate a percentage value (0-100)
    pub fn validate_percentage(value: u8) -> Result<()> {
        if value > MAX_PERCENTAGE {
            return Err(ColorError::InvalidArguments(format!(
                "Percentage value {} exceeds maximum of {}",
                value, MAX_PERCENTAGE
            )));
        }
        Ok(())
    }

    /// Validate a cubic-bezier control point (0.0-1.0)
    pub fn validate_bezier_point(value: f64) -> Result<()> {
        if !(BEZIER_MIN..=BEZIER_MAX).contains(&value) {
            return Err(ColorError::InvalidArguments(format!(
                "Cubic-bezier control point {} must be between {} and {}",
                value, BEZIER_MIN, BEZIER_MAX
            )));
        }
        Ok(())
    }

    /// Convert RGB values to HSL representation string
    pub fn rgb_to_string(r: u8, g: u8, b: u8) -> String {
        format!("rgb({}, {}, {})", r, g, b)
    }

    /// Convert HSL values to string representation
    pub fn hsl_to_string(h: f64, s: f64, l: f64) -> String {
        format!("HSL({:.1}°, {:.1}%, {:.1}%)", h, s * 100.0, l * 100.0)
    }

    /// Convert Lab values to string representation - DEPRECATED: Use PrecisionUtils::format_lab instead
    pub fn lab_to_string(l: f64, a: f64, b: f64) -> String {
        format!("Lab({:.1}, {:.1}, {:.1})", l, a, b)
    }

    /// DUPLICATION ELIMINATED: Use PrecisionUtils::format_percentage() instead
    /// Format a percentage value as string - Simple u8 version for legacy compatibility
    pub fn format_percentage(value: u8) -> String {
        format!("{}%", value)
    }

    /// Check if a string is a valid filename
    pub fn validate_filename(filename: &str) -> Result<()> {
        if filename.is_empty() {
            return Err(ColorError::InvalidArguments(
                "Filename cannot be empty".to_string(),
            ));
        }

        // Check for invalid characters (basic validation)
        let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
        for ch in invalid_chars {
            if filename.contains(ch) {
                return Err(ColorError::InvalidArguments(format!(
                    "Filename contains invalid character: {}",
                    ch
                )));
            }
        }

        Ok(())
    }

    /// Convert easing function name to control points
    pub fn easing_preset_to_points(preset: &str) -> Option<(f64, f64)> {
        match preset.to_lowercase().as_str() {
            "linear" => Some((0.0, 1.0)),
            "ease" => Some((0.25, 1.0)),
            "ease-in" => Some((0.42, 1.0)),
            "ease-out" => Some((0.0, 0.58)),
            "ease-in-out" => Some((0.42, 0.58)),
            _ => None,
        }
    }

    /// Get available easing presets
    pub fn get_easing_presets() -> Vec<&'static str> {
        vec!["linear", "ease", "ease-in", "ease-out", "ease-in-out"]
    }

    /// Calculate aspect ratio for image dimensions
    pub fn calculate_image_height(width: u32) -> u32 {
        (width as f64 * HEIGHT_RATIO) as u32
    }

    /// Calculate legend height based on gradient height
    pub fn calculate_legend_height(gradient_height: u32) -> u32 {
        (gradient_height as f64 * DEFAULT_LEGEND_HEIGHT_RATIO).max(20.0) as u32
    }

    /// Calculate font size based on legend height
    pub fn calculate_font_size(legend_height: u32) -> u32 {
        (legend_height as f64 * DEFAULT_FONT_SIZE_RATIO).max(10.0) as u32
    }

    /// Sanitize a color hex string
    pub fn sanitize_hex_color(hex: &str) -> String {
        let mut cleaned = hex.trim().to_uppercase();

        // Remove # if present
        if cleaned.starts_with('#') {
            cleaned = cleaned[1..].to_string();
        }

        // Ensure it's 6 characters
        if cleaned.len() == 3 {
            // Convert 3-digit hex to 6-digit
            let chars: Vec<char> = cleaned.chars().collect();
            cleaned = format!(
                "{}{}{}{}{}{}",
                chars[0], chars[0], chars[1], chars[1], chars[2], chars[2]
            );
        }

        cleaned
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp() {
        assert_eq!(Utils::clamp(5, 0, 10), 5);
        assert_eq!(Utils::clamp(-1, 0, 10), 0);
        assert_eq!(Utils::clamp(15, 0, 10), 10);
    }

    #[test]
    fn test_round_to_decimals() {
        assert_eq!(Utils::round_to_decimals(std::f64::consts::PI, 2), 3.14);
        assert_eq!(Utils::round_to_decimals(2.5, 0), 3.0);
    }

    #[test]
    fn test_validate_percentage() {
        assert!(Utils::validate_percentage(50).is_ok());
        assert!(Utils::validate_percentage(0).is_ok());
        assert!(Utils::validate_percentage(100).is_ok());
        assert!(Utils::validate_percentage(101).is_err());
    }

    #[test]
    fn test_validate_bezier_point() {
        assert!(Utils::validate_bezier_point(0.5).is_ok());
        assert!(Utils::validate_bezier_point(0.0).is_ok());
        assert!(Utils::validate_bezier_point(1.0).is_ok());
        assert!(Utils::validate_bezier_point(-0.1).is_err());
        assert!(Utils::validate_bezier_point(1.1).is_err());
    }

    #[test]
    fn test_easing_presets() {
        assert_eq!(Utils::easing_preset_to_points("linear"), Some((0.0, 1.0)));
        assert_eq!(
            Utils::easing_preset_to_points("ease-in-out"),
            Some((0.42, 0.58))
        );
        assert_eq!(Utils::easing_preset_to_points("invalid"), None);
    }

    #[test]
    fn test_sanitize_hex_color() {
        assert_eq!(Utils::sanitize_hex_color("#ff0000"), "FF0000");
        assert_eq!(Utils::sanitize_hex_color("ff0000"), "FF0000");
        assert_eq!(Utils::sanitize_hex_color("#f00"), "FF0000");
        assert_eq!(Utils::sanitize_hex_color("f00"), "FF0000");
    }

    #[test]
    fn test_filename_validation() {
        assert!(Utils::validate_filename("gradient.svg").is_ok());
        assert!(Utils::validate_filename("test|file.png").is_err());
        assert!(Utils::validate_filename("").is_err());
    }
}
