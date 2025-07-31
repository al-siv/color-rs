//! Color contrast calculation functions
//!
//! Pure functions for calculating contrast ratios and accessibility compliance.
//! Implements WCAG 2.1 guidelines for web accessibility.

use crate::color_ops::luminance;
use crate::color_utils::LegacyColorUtils as ColorUtils;
use palette::Srgb;

/// Calculate WCAG contrast ratio between two colors
///
/// Implements the WCAG 2.1 contrast ratio formula for accessibility compliance.
/// Used to determine if text and background color combinations meet accessibility standards.
///
/// # Arguments
/// * `color1` - First color (typically text)
/// * `color2` - Second color (typically background)
///
/// # Returns
/// * Contrast ratio (1.0 = no contrast, 21.0 = maximum contrast)
///
/// # Example
/// ```rust
/// use color_rs::color_ops::contrast;
/// use palette::Srgb;
///
/// let white = Srgb::new(1.0, 1.0, 1.0);
/// let black = Srgb::new(0.0, 0.0, 0.0);
/// let ratio = contrast::wcag_ratio(white, black);
/// assert!((ratio - 21.0).abs() < 0.1); // Maximum contrast
/// ```
pub fn wcag_ratio(color1: Srgb, color2: Srgb) -> f64 {
    ColorUtils::wcag_contrast_ratio(color1, color2)
}

/// Calculate contrast ratio using RGB tuples
///
/// Convenience function for RGB tuple input.
///
/// # Arguments
/// * `rgb1` - First color as (u8, u8, u8) tuple
/// * `rgb2` - Second color as (u8, u8, u8) tuple
///
/// # Returns
/// * WCAG contrast ratio
///
/// # Example
/// ```rust
/// use color_rs::color_ops::contrast;
///
/// let ratio = contrast::wcag_ratio_rgb((255, 255, 255), (0, 0, 0));
/// assert!((ratio - 21.0).abs() < 0.1);
/// ```
pub fn wcag_ratio_rgb(rgb1: (u8, u8, u8), rgb2: (u8, u8, u8)) -> f64 {
    ColorUtils::wcag_contrast_ratio_rgb(rgb1, rgb2)
}

/// Calculate contrast ratio from pre-computed luminance values
///
/// More efficient when you already have luminance values computed.
///
/// # Arguments
/// * `lum1` - Relative luminance of first color (0.0-1.0)
/// * `lum2` - Relative luminance of second color (0.0-1.0)
///
/// # Returns
/// * WCAG contrast ratio
///
/// # Example
/// ```rust
/// use color_rs::color_ops::{contrast, luminance};
/// use palette::Srgb;
///
/// let color1 = Srgb::new(0.8, 0.8, 0.8);
/// let color2 = Srgb::new(0.2, 0.2, 0.2);
/// 
/// let lum1 = luminance::wcag_relative(color1);
/// let lum2 = luminance::wcag_relative(color2);
/// let ratio = contrast::from_luminance(lum1, lum2);
/// ```
pub fn from_luminance(lum1: f64, lum2: f64) -> f64 {
    let lighter = lum1.max(lum2);
    let darker = lum1.min(lum2);
    (lighter + 0.05) / (darker + 0.05)
}

/// Check if contrast ratio meets WCAG AA standard
///
/// WCAG AA requires:
/// - 4.5:1 for normal text
/// - 3:1 for large text (18pt+ or 14pt+ bold)
///
/// # Arguments
/// * `ratio` - Contrast ratio to check
/// * `large_text` - Whether this is considered large text
///
/// # Returns
/// * `true` if meets AA standard, `false` otherwise
///
/// # Example
/// ```rust
/// use color_rs::color_ops::contrast;
///
/// assert!(contrast::meets_aa_standard(4.6, false)); // Normal text
/// assert!(contrast::meets_aa_standard(3.1, true));  // Large text
/// assert!(!contrast::meets_aa_standard(3.0, false)); // Fails normal text
/// ```
pub fn meets_aa_standard(ratio: f64, large_text: bool) -> bool {
    if large_text {
        ratio >= 3.0
    } else {
        ratio >= 4.5
    }
}

/// Check if contrast ratio meets WCAG AAA standard
///
/// WCAG AAA requires:
/// - 7:1 for normal text
/// - 4.5:1 for large text
///
/// # Arguments
/// * `ratio` - Contrast ratio to check
/// * `large_text` - Whether this is considered large text
///
/// # Returns
/// * `true` if meets AAA standard, `false` otherwise
///
/// # Example
/// ```rust
/// use color_rs::color_ops::contrast;
///
/// assert!(contrast::meets_aaa_standard(7.5, false)); // Normal text
/// assert!(contrast::meets_aaa_standard(4.6, true));  // Large text
/// assert!(!contrast::meets_aaa_standard(6.0, false)); // Fails normal text
/// ```
pub fn meets_aaa_standard(ratio: f64, large_text: bool) -> bool {
    if large_text {
        ratio >= 4.5
    } else {
        ratio >= 7.0
    }
}

/// Get accessibility compliance level for a contrast ratio
///
/// Returns the highest WCAG compliance level achieved.
///
/// # Arguments
/// * `ratio` - Contrast ratio to evaluate
/// * `large_text` - Whether this is considered large text
///
/// # Returns
/// * Compliance level: "AAA", "AA", or "Fail"
///
/// # Example
/// ```rust
/// use color_rs::color_ops::contrast;
///
/// assert_eq!(contrast::compliance_level(8.0, false), "AAA");
/// assert_eq!(contrast::compliance_level(5.0, false), "AA");
/// assert_eq!(contrast::compliance_level(3.0, false), "Fail");
/// ```
pub fn compliance_level(ratio: f64, large_text: bool) -> &'static str {
    if meets_aaa_standard(ratio, large_text) {
        "AAA"
    } else if meets_aa_standard(ratio, large_text) {
        "AA"
    } else {
        "Fail"
    }
}

/// Find minimum acceptable background luminance for text
///
/// Given a text color, find the darkest background that will meet
/// the specified contrast requirement.
///
/// # Arguments
/// * `text_luminance` - Relative luminance of text color
/// * `min_ratio` - Minimum required contrast ratio
///
/// # Returns
/// * Maximum background luminance that meets the requirement
///
/// # Example
/// ```rust
/// use color_rs::color_ops::{contrast, luminance};
/// use palette::Srgb;
///
/// let text_color = Srgb::new(0.2, 0.2, 0.2);
/// let text_lum = luminance::wcag_relative(text_color);
/// let max_bg_lum = contrast::max_background_luminance(text_lum, 4.5);
/// ```
pub fn max_background_luminance(text_luminance: f64, min_ratio: f64) -> f64 {
    // For darker backgrounds: (text_lum + 0.05) / (bg_lum + 0.05) = min_ratio
    // Solving for bg_lum: bg_lum = (text_lum + 0.05) / min_ratio - 0.05
    let max_dark_bg = ((text_luminance + 0.05) / min_ratio - 0.05).max(0.0);
    
    // For lighter backgrounds: (bg_lum + 0.05) / (text_lum + 0.05) = min_ratio
    // Solving for bg_lum: bg_lum = min_ratio * (text_lum + 0.05) - 0.05
    let min_light_bg = (min_ratio * (text_luminance + 0.05) - 0.05).min(1.0);
    
    // Return the darker option (maximum background luminance)
    max_dark_bg.max(0.0).min(1.0)
}

/// Alias for `wcag_ratio` - more concise name
pub fn ratio(color1: Srgb, color2: Srgb) -> f64 {
    wcag_ratio(color1, color2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Srgb;

    #[test]
    fn test_wcag_contrast_ratio() {
        // Test maximum contrast (white on black)
        let white = Srgb::new(1.0, 1.0, 1.0);
        let black = Srgb::new(0.0, 0.0, 0.0);
        let ratio = wcag_ratio(white, black);
        assert!((ratio - 21.0).abs() < 0.1);

        // Test minimum contrast (same colors)
        let gray = Srgb::new(0.5, 0.5, 0.5);
        let same_ratio = wcag_ratio(gray, gray);
        assert!((same_ratio - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_contrast_symmetry() {
        let color1 = Srgb::new(0.3, 0.6, 0.9);
        let color2 = Srgb::new(0.8, 0.2, 0.1);
        
        // Contrast should be symmetric
        assert!((wcag_ratio(color1, color2) - wcag_ratio(color2, color1)).abs() < 1e-10);
    }

    #[test]
    fn test_from_luminance() {
        let lum1 = 0.8;
        let lum2 = 0.2;
        let ratio = from_luminance(lum1, lum2);
        
        // Manual calculation: (0.8 + 0.05) / (0.2 + 0.05) = 0.85 / 0.25 = 3.4
        assert!((ratio - 3.4).abs() < 1e-6);
    }

    #[test]
    fn test_wcag_compliance_standards() {
        // Test AA standards
        assert!(meets_aa_standard(4.5, false)); // Normal text
        assert!(meets_aa_standard(3.0, true));  // Large text
        assert!(!meets_aa_standard(4.4, false)); // Fails normal
        assert!(!meets_aa_standard(2.9, true));  // Fails large

        // Test AAA standards
        assert!(meets_aaa_standard(7.0, false)); // Normal text
        assert!(meets_aaa_standard(4.5, true));  // Large text
        assert!(!meets_aaa_standard(6.9, false)); // Fails normal
        assert!(!meets_aaa_standard(4.4, true));  // Fails large
    }

    #[test]
    fn test_compliance_level() {
        assert_eq!(compliance_level(8.0, false), "AAA");
        assert_eq!(compliance_level(5.0, false), "AA");
        assert_eq!(compliance_level(3.0, false), "Fail");
        
        assert_eq!(compliance_level(5.0, true), "AAA");
        assert_eq!(compliance_level(3.5, true), "AA");
        assert_eq!(compliance_level(2.5, true), "Fail");
    }

    #[test]
    fn test_rgb_tuple_interface() {
        let ratio1 = wcag_ratio_rgb((255, 255, 255), (0, 0, 0));
        let ratio2 = wcag_ratio(
            Srgb::new(1.0, 1.0, 1.0),
            Srgb::new(0.0, 0.0, 0.0)
        );
        
        assert!((ratio1 - ratio2).abs() < 1e-6);
    }

    #[test]
    fn test_ratio_alias() {
        let color1 = Srgb::new(0.7, 0.3, 0.5);
        let color2 = Srgb::new(0.2, 0.8, 0.4);
        
        assert_eq!(ratio(color1, color2), wcag_ratio(color1, color2));
    }
}
