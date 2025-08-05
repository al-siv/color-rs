//! Color interpolation methods
//!
//! Provides various color space interpolation algorithms for smooth color transitions.
//! Each method operates in a different color space for different visual characteristics.

use palette::{IntoColor, Lab, Lch, Mix, Srgb};

/// Mix two colors using linear RGB interpolation
///
/// Simple linear interpolation in RGB color space. Fast but may produce
/// muddy results for complementary colors.
///
/// # Arguments
/// * `color1` - First color
/// * `color2` - Second color
/// * `factor` - Mixing factor (0.0 = color1, 1.0 = color2)
///
/// # Returns
/// * Mixed color in sRGB space
///
/// # Example
/// ```rust
/// use color_rs::color_ops::mixing::interpolation;
/// use palette::Srgb;
///
/// let red = Srgb::new(1.0, 0.0, 0.0);
/// let blue = Srgb::new(0.0, 0.0, 1.0);
/// let purple = interpolation::linear_rgb(red, blue, 0.5);
/// ```
#[must_use]
pub fn linear_rgb(color1: Srgb, color2: Srgb, factor: f32) -> Srgb {
    color1.mix(color2, factor)
}

/// Mix two colors using LAB color space interpolation
///
/// Interpolates in CIELAB color space for more perceptually uniform results.
/// Better than RGB for avoiding muddy intermediate colors.
///
/// # Arguments
/// * `color1` - First color
/// * `color2` - Second color
/// * `factor` - Mixing factor (0.0 = color1, 1.0 = color2)
///
/// # Returns
/// * Mixed color in sRGB space
///
/// # Example
/// ```rust
/// use color_rs::color_ops::mixing::interpolation;
/// use palette::Srgb;
///
/// let yellow = Srgb::new(1.0, 1.0, 0.0);
/// let blue = Srgb::new(0.0, 0.0, 1.0);
/// let mixed = interpolation::lab_interpolation(yellow, blue, 0.3);
/// ```
#[must_use]
pub fn lab_interpolation(color1: Srgb, color2: Srgb, factor: f32) -> Srgb {
    let lab1: Lab = color1.into_color();
    let lab2: Lab = color2.into_color();
    let mixed_lab = lab1.mix(lab2, factor);
    mixed_lab.into_color()
}

/// Mix two colors using LCH color space interpolation
///
/// Interpolates in LCH (cylindrical LAB) color space, providing smooth
/// hue transitions. Best for creating natural color progressions.
///
/// # Arguments
/// * `color1` - First color
/// * `color2` - Second color
/// * `factor` - Mixing factor (0.0 = color1, 1.0 = color2)
///
/// # Returns
/// * Mixed color in sRGB space
///
/// # Example
/// ```rust
/// use color_rs::color_ops::mixing::interpolation;
/// use palette::Srgb;
///
/// let red = Srgb::new(1.0, 0.0, 0.0);
/// let green = Srgb::new(0.0, 1.0, 0.0);
/// let mixed = interpolation::lch_interpolation(red, green, 0.5);
/// ```
#[must_use]
pub fn lch_interpolation(color1: Srgb, color2: Srgb, factor: f32) -> Srgb {
    let lch1: Lch = color1.into_color();
    let lch2: Lch = color2.into_color();
    let mixed_lch = lch1.mix(lch2, factor);
    mixed_lch.into_color()
}

/// Mix two colors using HSL color space interpolation
///
/// Interpolates in HSL space, useful for maintaining saturation and
/// lightness relationships while transitioning hue.
///
/// # Arguments
/// * `color1` - First color
/// * `color2` - Second color
/// * `factor` - Mixing factor (0.0 = color1, 1.0 = color2)
///
/// # Returns
/// * Mixed color in sRGB space
///
/// # Example
/// ```rust
/// use color_rs::color_ops::mixing::interpolation;
/// use palette::Srgb;
///
/// let orange = Srgb::new(1.0, 0.5, 0.0);
/// let purple = Srgb::new(0.5, 0.0, 1.0);
/// let mixed = interpolation::hsl_interpolation(orange, purple, 0.4);
/// ```
pub fn hsl_interpolation(color1: Srgb, color2: Srgb, factor: f32) -> Srgb {
    use crate::color_ops::conversion;

    let hsl1 = conversion::srgb_to_hsl(color1);
    let hsl2 = conversion::srgb_to_hsl(color2);
    let mixed_hsl = hsl1.mix(hsl2, factor);
    conversion::hsl_to_srgb(mixed_hsl)
}

/// Mix two colors using HSV color space interpolation
///
/// Interpolates in HSV space, which can be intuitive for adjusting
/// brightness while maintaining hue relationships.
///
/// # Arguments
/// * `color1` - First color
/// * `color2` - Second color
/// * `factor` - Mixing factor (0.0 = color1, 1.0 = color2)
///
/// # Returns
/// * Mixed color in sRGB space
///
/// # Example
/// ```rust
/// use color_rs::color_ops::mixing::interpolation;
/// use palette::Srgb;
///
/// let bright_red = Srgb::new(1.0, 0.0, 0.0);
/// let dark_red = Srgb::new(0.3, 0.0, 0.0);
/// let mixed = interpolation::hsv_interpolation(bright_red, dark_red, 0.6);
/// ```
pub fn hsv_interpolation(color1: Srgb, color2: Srgb, factor: f32) -> Srgb {
    use crate::color_ops::conversion;

    let hsv1 = conversion::srgb_to_hsv(color1);
    let hsv2 = conversion::srgb_to_hsv(color2);
    let mixed_hsv = hsv1.mix(hsv2, factor);
    conversion::hsv_to_srgb(mixed_hsv)
}
