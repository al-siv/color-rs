//! Utility functions for color-rs

use crate::config::{BEZIER_MAX, BEZIER_MIN, MAX_PERCENTAGE, display_constants, math_constants};
use crate::error::{ColorError, Result};

/// Utility functions for various operations
pub struct Utils;

impl Utils {
    /// Clamp a value between min and max
    #[must_use]
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
    #[must_use]
    pub fn round_to_decimals(value: f64, decimals: u32) -> f64 {
        let multiplier = 10f64.powi(decimals as i32);
        (value * multiplier).round() / multiplier
    }

    /// Validate a percentage value (0-100)
    /// Validate percentage value is in range 0-100
    ///
    /// # Errors
    /// Returns `ColorError::InvalidValue` if the percentage is outside the valid range.
    pub fn validate_percentage(value: u8) -> Result<()> {
        if value > MAX_PERCENTAGE {
            return Err(ColorError::InvalidArguments(format!(
                "Percentage value {value} exceeds maximum of {MAX_PERCENTAGE}"
            )));
        }
        Ok(())
    }

    /// Validate a cubic-bezier control point (0.0-1.0)
    /// Validate bezier control point is in range 0.0-1.0
    ///
    /// # Errors
    /// Returns `ColorError::InvalidValue` if the bezier point is outside the valid range.
    pub fn validate_bezier_point(value: f64) -> Result<()> {
        if !(BEZIER_MIN..=BEZIER_MAX).contains(&value) {
            return Err(ColorError::InvalidArguments(format!(
                "Cubic-bezier control point {value} must be between {BEZIER_MIN} and {BEZIER_MAX}"
            )));
        }
        Ok(())
    }

    /// Convert RGB values to HSL representation string
    #[must_use]
    pub fn rgb_to_string(r: u8, g: u8, b: u8) -> String {
        format!("rgb({r}, {g}, {b})")
    }

    /// Convert HSL values to string representation
    #[must_use]
    pub fn hsl_to_string(h: f64, s: f64, l: f64) -> String {
        format!(
            "HSL({:.1}°, {:.1}%, {:.1}%)",
            h,
            s * math_constants::PERCENTAGE_MULTIPLIER,
            l * math_constants::PERCENTAGE_MULTIPLIER
        )
    }

    /// DUPLICATION ELIMINATED: Use `PrecisionUtils::format_percentage()` instead
    /// Format a percentage value as string - Simple u8 version for legacy compatibility
    #[must_use]
    pub fn format_percentage(value: u8) -> String {
        format!("{value}%")
    }

    /// Check if a string is a valid filename
    /// Validate filename contains only safe characters
    ///
    /// # Errors
    /// Returns `ColorError::InvalidFilename` if the filename contains illegal characters
    /// or patterns that could cause file system issues.
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
                    "Filename contains invalid character: {ch}"
                )));
            }
        }

        Ok(())
    }

    /// Convert easing function name to control points
    #[must_use]
    pub fn easing_preset_to_points(preset: &str) -> Option<(f64, f64)> {
        use crate::config::bezier_presets;

        match preset.to_lowercase().as_str() {
            "linear" => Some(bezier_presets::LINEAR),
            "ease" => Some(bezier_presets::EASE),
            "ease-in" => Some(bezier_presets::EASE_IN),
            "ease-out" => Some(bezier_presets::EASE_OUT),
            "ease-in-out" => Some(bezier_presets::EASE_IN_OUT),
            _ => None,
        }
    }

    /// Get available easing presets
    #[must_use]
    pub fn get_easing_presets() -> Vec<&'static str> {
        vec!["linear", "ease", "ease-in", "ease-out", "ease-in-out"]
    }

    /// Calculate aspect ratio for image dimensions
    #[must_use]
    pub fn calculate_image_height(width: u32) -> u32 {
        (f64::from(width) * display_constants::HEIGHT_RATIO) as u32
    }

    /// Calculate legend height based on gradient height
    #[must_use]
    pub fn calculate_legend_height(gradient_height: u32) -> u32 {
        (f64::from(gradient_height) * display_constants::DEFAULT_LEGEND_HEIGHT_RATIO)
            .max(display_constants::MIN_LEGEND_HEIGHT) as u32
    }

    /// Calculate font size based on legend height
    #[must_use]
    pub fn calculate_font_size(legend_height: u32) -> u32 {
        (f64::from(legend_height) * display_constants::DEFAULT_FONT_SIZE_RATIO)
            .max(display_constants::MIN_FONT_SIZE) as u32
    }

    /// Sanitize a color hex string
    #[must_use]
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
        let pi_rounded = Utils::round_to_decimals(std::f64::consts::PI, 2);
        // Use approximately equal comparison instead of exact
        assert!(
            (pi_rounded - (std::f64::consts::PI * 100.0).round() / 100.0).abs()
                < f64::EPSILON * 10.0
        );
        assert!((Utils::round_to_decimals(2.5, 0) - 3.0).abs() < f64::EPSILON);
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
        use crate::config::bezier_presets;

        assert_eq!(
            Utils::easing_preset_to_points("linear"),
            Some(bezier_presets::LINEAR)
        );
        assert_eq!(
            Utils::easing_preset_to_points("ease-in-out"),
            Some(bezier_presets::EASE_IN_OUT)
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
