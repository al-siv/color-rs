//! Color blending algorithms
//!
//! Implements various blending modes similar to those found in image editing software.
//! All blending functions take base and overlay colors plus an opacity factor.

use palette::Srgb;
use super::interpolation::linear_rgb;

/// Blend two colors using multiply mode
///
/// Simulates mixing pigments - darker colors result from combining.
/// Multiplies color component values together.
///
/// # Arguments
/// * `base` - Base color
/// * `overlay` - Overlay color to blend
/// * `opacity` - Opacity of overlay (0.0 = transparent, 1.0 = opaque)
///
/// # Returns
/// * Blended color
///
/// # Example
/// ```rust
/// use color_rs::color_ops::mixing::blending;
/// use palette::Srgb;
///
/// let yellow = Srgb::new(1.0, 1.0, 0.0);
/// let cyan = Srgb::new(0.0, 1.0, 1.0);
/// let blended = blending::multiply_blend(yellow, cyan, 0.8);
/// ```
pub fn multiply_blend(base: Srgb, overlay: Srgb, opacity: f32) -> Srgb {
    let blended = Srgb::new(
        base.red * overlay.red,
        base.green * overlay.green,
        base.blue * overlay.blue,
    );
    
    linear_rgb(base, blended, opacity)
}

/// Blend two colors using screen mode
///
/// Simulates overlapping light sources - lighter colors result.
/// Inverts, multiplies, then inverts again.
///
/// # Arguments
/// * `base` - Base color
/// * `overlay` - Overlay color to blend
/// * `opacity` - Opacity of overlay (0.0 = transparent, 1.0 = opaque)
///
/// # Returns
/// * Blended color
///
/// # Example
/// ```rust
/// use color_rs::color_ops::mixing::blending;
/// use palette::Srgb;
///
/// let red = Srgb::new(0.8, 0.0, 0.0);
/// let blue = Srgb::new(0.0, 0.0, 0.8);
/// let blended = blending::screen_blend(red, blue, 0.7);
/// ```
pub fn screen_blend(base: Srgb, overlay: Srgb, opacity: f32) -> Srgb {
    let blended = Srgb::new(
        1.0 - (1.0 - base.red) * (1.0 - overlay.red),
        1.0 - (1.0 - base.green) * (1.0 - overlay.green),
        1.0 - (1.0 - base.blue) * (1.0 - overlay.blue),
    );
    
    linear_rgb(base, blended, opacity)
}

/// Blend two colors using overlay mode
///
/// Combines multiply and screen modes depending on base color brightness.
/// Preserves highlights and shadows while increasing contrast.
///
/// # Arguments
/// * `base` - Base color
/// * `overlay` - Overlay color to blend
/// * `opacity` - Opacity of overlay (0.0 = transparent, 1.0 = opaque)
///
/// # Returns
/// * Blended color
///
/// # Example
/// ```rust
/// use color_rs::color_ops::mixing::blending;
/// use palette::Srgb;
///
/// let gray = Srgb::new(0.5, 0.5, 0.5);
/// let orange = Srgb::new(1.0, 0.5, 0.0);
/// let blended = blending::overlay_blend(gray, orange, 0.6);
/// ```
pub fn overlay_blend(base: Srgb, overlay: Srgb, opacity: f32) -> Srgb {
    let blend_component = |base: f32, overlay: f32| -> f32 {
        if base < 0.5 {
            2.0 * base * overlay
        } else {
            1.0 - 2.0 * (1.0 - base) * (1.0 - overlay)
        }
    };
    
    let blended = Srgb::new(
        blend_component(base.red, overlay.red),
        blend_component(base.green, overlay.green),
        blend_component(base.blue, overlay.blue),
    );
    
    linear_rgb(base, blended, opacity)
}
