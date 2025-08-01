//! Luminance calculation functions
//!
//! Pure functions for calculating color luminance using various methods.
//! All functions operate directly on color values without object instantiation.

use palette::{Srgb, Lab, IntoColor};

/// Calculate WCAG relative luminance for a color
///
/// Implements the WCAG 2.1 relative luminance formula for accessibility compliance.
/// 
/// # Arguments
/// * `srgb` - Source color in sRGB color space
///
/// # Returns
/// * Relative luminance value (0.0 = black, 1.0 = white)
///
/// # Example
/// ```rust
/// use color_rs::color_ops::luminance;
/// use palette::Srgb;
///
/// let red = Srgb::new(1.0, 0.0, 0.0);
/// let luminance = luminance::wcag_relative(red);
/// assert!(luminance > 0.2 && luminance < 0.3);
/// ```
pub fn wcag_relative(srgb: Srgb) -> f64 {
    // WCAG 2.1 relative luminance formula
    let r = srgb_gamma_correct(srgb.red as f64);
    let g = srgb_gamma_correct(srgb.green as f64); 
    let b = srgb_gamma_correct(srgb.blue as f64);
    
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// Helper function for gamma correction in WCAG luminance calculation
fn srgb_gamma_correct(value: f64) -> f64 {
    if value <= 0.03928 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

/// Calculate relative luminance from RGB tuple
///
/// Convenience function for RGB tuple input.
///
/// # Arguments
/// * `rgb` - RGB values as (u8, u8, u8) tuple
///
/// # Returns
/// * Relative luminance value (0.0-1.0)
///
/// # Example
/// ```rust
/// use color_rs::color_ops::luminance;
///
/// let luminance = luminance::from_rgb((255, 0, 0)); // Red
/// assert!(luminance > 0.2 && luminance < 0.3);
/// ```
pub fn from_rgb(rgb: (u8, u8, u8)) -> f64 {
    let srgb = Srgb::new(
        rgb.0 as f32 / 255.0,
        rgb.1 as f32 / 255.0,
        rgb.2 as f32 / 255.0
    );
    wcag_relative(srgb)
}

/// Alias for `wcag_relative` - more concise name
pub fn relative_luminance(srgb: Srgb) -> f64 {
    wcag_relative(srgb)
}

/// Calculate perceived brightness using LAB L* component
///
/// Uses the LAB color space L* component which better represents
/// human perception of brightness compared to RGB luminance.
///
/// # Arguments
/// * `srgb` - Source color in sRGB color space
///
/// # Returns
/// * Perceived brightness (0.0-100.0, where 100 is pure white)
///
/// # Example
/// ```rust
/// use color_rs::color_ops::luminance;
/// use palette::Srgb;
///
/// let yellow = Srgb::new(1.0, 1.0, 0.0);
/// let brightness = luminance::perceived_brightness(yellow);
/// assert!(brightness > 90.0); // Yellow appears very bright
/// ```
pub fn perceived_brightness(srgb: Srgb) -> f64 {
    let lab: Lab = srgb.into_color();
    lab.l as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Srgb;

    #[test]
    fn test_wcag_relative_luminance() {
        // Test pure colors
        let white = Srgb::new(1.0, 1.0, 1.0);
        assert!((wcag_relative(white) - 1.0).abs() < 1e-6);

        let black = Srgb::new(0.0, 0.0, 0.0);
        assert!(wcag_relative(black) < 1e-6);

        let red = Srgb::new(1.0, 0.0, 0.0);
        let red_luminance = wcag_relative(red);
        assert!(red_luminance > 0.2 && red_luminance < 0.3);
    }

    #[test]
    fn test_from_rgb_tuple() {
        let red_luminance = from_rgb((255, 0, 0));
        let srgb_red = Srgb::new(1.0, 0.0, 0.0);
        let srgb_luminance = wcag_relative(srgb_red);
        
        // Should be approximately equal
        assert!((red_luminance - srgb_luminance).abs() < 1e-6);
    }

    #[test]
    fn test_perceived_brightness() {
        let white = Srgb::new(1.0, 1.0, 1.0);
        let brightness = perceived_brightness(white);
        assert!(brightness > 95.0); // Close to 100

        let black = Srgb::new(0.0, 0.0, 0.0);
        let dark_brightness = perceived_brightness(black);
        assert!(dark_brightness < 5.0); // Close to 0
    }

    #[test]
    fn test_relative_luminance_alias() {
        let color = Srgb::new(0.5, 0.5, 0.5);
        assert_eq!(wcag_relative(color), relative_luminance(color));
    }
}
