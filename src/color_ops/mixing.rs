//! Color mixing and blending functions
//!
//! Pure functions for mixing colors using various blending modes and interpolation methods.
//! All functions operate directly on color values without object instantiation.

use palette::{Lab, Lch, Mix, Srgb, IntoColor};

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
/// use color_rs::color_ops::mixing;
/// use palette::Srgb;
///
/// let red = Srgb::new(1.0, 0.0, 0.0);
/// let blue = Srgb::new(0.0, 0.0, 1.0);
/// let purple = mixing::linear_rgb(red, blue, 0.5);
/// ```
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
/// use color_rs::color_ops::mixing;
/// use palette::Srgb;
///
/// let yellow = Srgb::new(1.0, 1.0, 0.0);
/// let blue = Srgb::new(0.0, 0.0, 1.0);
/// let mixed = mixing::lab_interpolation(yellow, blue, 0.3);
/// ```
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
/// use color_rs::color_ops::mixing;
/// use palette::Srgb;
///
/// let red = Srgb::new(1.0, 0.0, 0.0);
/// let green = Srgb::new(0.0, 1.0, 0.0);
/// let mixed = mixing::lch_interpolation(red, green, 0.5);
/// ```
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
/// use color_rs::color_ops::mixing;
/// use palette::Srgb;
///
/// let orange = Srgb::new(1.0, 0.5, 0.0);
/// let purple = Srgb::new(0.5, 0.0, 1.0);
/// let mixed = mixing::hsl_interpolation(orange, purple, 0.4);
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
/// use color_rs::color_ops::mixing;
/// use palette::Srgb;
///
/// let bright_red = Srgb::new(1.0, 0.0, 0.0);
/// let dark_red = Srgb::new(0.3, 0.0, 0.0);
/// let mixed = mixing::hsv_interpolation(bright_red, dark_red, 0.6);
/// ```
pub fn hsv_interpolation(color1: Srgb, color2: Srgb, factor: f32) -> Srgb {
    use crate::color_ops::conversion;
    
    let hsv1 = conversion::srgb_to_hsv(color1);
    let hsv2 = conversion::srgb_to_hsv(color2);
    let mixed_hsv = hsv1.mix(hsv2, factor);
    conversion::hsv_to_srgb(mixed_hsv)
}

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
/// use color_rs::color_ops::mixing;
/// use palette::Srgb;
///
/// let yellow = Srgb::new(1.0, 1.0, 0.0);
/// let cyan = Srgb::new(0.0, 1.0, 1.0);
/// let blended = mixing::multiply_blend(yellow, cyan, 0.8);
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
/// use color_rs::color_ops::mixing;
/// use palette::Srgb;
///
/// let red = Srgb::new(0.8, 0.0, 0.0);
/// let blue = Srgb::new(0.0, 0.0, 0.8);
/// let blended = mixing::screen_blend(red, blue, 0.7);
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
/// use color_rs::color_ops::mixing;
/// use palette::Srgb;
///
/// let gray = Srgb::new(0.5, 0.5, 0.5);
/// let orange = Srgb::new(1.0, 0.5, 0.0);
/// let blended = mixing::overlay_blend(gray, orange, 0.6);
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

/// Create a color palette by mixing between multiple colors
///
/// Generates a palette by interpolating between an array of key colors.
/// Useful for creating gradients and color schemes.
///
/// # Arguments
/// * `key_colors` - Array of key colors to interpolate between
/// * `steps` - Number of colors to generate in the palette
/// * `color_space` - Color space to use for interpolation
///
/// # Returns
/// * Vector of interpolated colors
///
/// # Example
/// ```rust
/// use color_rs::color_ops::mixing;
/// use palette::Srgb;
///
/// let keys = vec![
///     Srgb::new(1.0, 0.0, 0.0), // Red
///     Srgb::new(0.0, 1.0, 0.0), // Green
///     Srgb::new(0.0, 0.0, 1.0), // Blue
/// ];
/// let palette = mixing::create_palette(&keys, 10, ColorSpace::Lab);
/// ```
pub fn create_palette(
    key_colors: &[Srgb],
    steps: usize,
    color_space: ColorSpace,
) -> Vec<Srgb> {
    if key_colors.len() < 2 || steps < 2 {
        return key_colors.to_vec();
    }
    
    let mut palette = Vec::with_capacity(steps);
    let segments = key_colors.len() - 1;
    let _steps_per_segment = (steps - 1) as f32 / segments as f32;
    
    for i in 0..steps {
        let position = i as f32 / (steps - 1) as f32;
        let segment_index = (position * segments as f32).floor() as usize;
        let segment_index = segment_index.min(segments - 1);
        
        let local_position = (position * segments as f32) - segment_index as f32;
        
        let color1 = key_colors[segment_index];
        let color2 = key_colors[segment_index + 1];
        
        let mixed = match color_space {
            ColorSpace::Rgb => linear_rgb(color1, color2, local_position),
            ColorSpace::Lab => lab_interpolation(color1, color2, local_position),
            ColorSpace::Lch => lch_interpolation(color1, color2, local_position),
            ColorSpace::Hsl => hsl_interpolation(color1, color2, local_position),
            ColorSpace::Hsv => hsv_interpolation(color1, color2, local_position),
        };
        
        palette.push(mixed);
    }
    
    palette
}

/// Color space options for interpolation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    /// Linear RGB interpolation
    Rgb,
    /// CIELAB color space (perceptually uniform)
    Lab,
    /// LCH color space (cylindrical LAB)
    Lch,
    /// HSL color space (Hue, Saturation, Lightness)
    Hsl,
    /// HSV color space (Hue, Saturation, Value)
    Hsv,
}

/// Mix multiple colors with specified weights
///
/// Weighted average of multiple colors. Weights don't need to sum to 1.0.
///
/// # Arguments
/// * `colors_and_weights` - Slice of (color, weight) tuples
/// * `color_space` - Color space to use for mixing
///
/// # Returns
/// * Mixed color, or None if no colors provided
///
/// # Example
/// ```rust
/// use color_rs::color_ops::mixing;
/// use palette::Srgb;
///
/// let colors = &[
///     (Srgb::new(1.0, 0.0, 0.0), 2.0), // Red with weight 2
///     (Srgb::new(0.0, 1.0, 0.0), 1.0), // Green with weight 1
///     (Srgb::new(0.0, 0.0, 1.0), 1.0), // Blue with weight 1
/// ];
/// let mixed = mixing::weighted_mix(colors, ColorSpace::Lab).unwrap();
/// ```
pub fn weighted_mix(
    colors_and_weights: &[(Srgb, f32)],
    color_space: ColorSpace,
) -> Option<Srgb> {
    if let Some(total_weight) = validate_weights(colors_and_weights) {
        match color_space {
            ColorSpace::Rgb => Some(mix_in_rgb_space(colors_and_weights, total_weight)),
            ColorSpace::Lab => Some(mix_in_lab_space(colors_and_weights, total_weight)),
            _ => weighted_mix(colors_and_weights, ColorSpace::Rgb), // Fallback
        }
    } else {
        None
    }
}

/// Validate color weights and return total weight if valid
fn validate_weights(colors_and_weights: &[(Srgb, f32)]) -> Option<f32> {
    if colors_and_weights.is_empty() {
        return None;
    }
    
    let total_weight: f32 = colors_and_weights.iter().map(|(_, w)| w).sum();
    if total_weight <= 0.0 {
        None
    } else {
        Some(total_weight)
    }
}

/// Mix colors in RGB color space
fn mix_in_rgb_space(colors_and_weights: &[(Srgb, f32)], total_weight: f32) -> Srgb {
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    
    for &(color, weight) in colors_and_weights {
        let normalized_weight = weight / total_weight;
        r += color.red * normalized_weight;
        g += color.green * normalized_weight;
        b += color.blue * normalized_weight;
    }
    
    Srgb::new(r, g, b)
}

/// Mix colors in LAB color space
fn mix_in_lab_space(colors_and_weights: &[(Srgb, f32)], total_weight: f32) -> Srgb {
    use crate::color_ops::conversion;
    
    let mut l = 0.0f32;
    let mut a = 0.0f32;
    let mut b = 0.0f32;
    
    for &(color, weight) in colors_and_weights {
        let lab = conversion::srgb_to_lab(color);
        let normalized_weight = weight / total_weight;
        l += lab.l * normalized_weight;
        a += lab.a * normalized_weight;
        b += lab.b * normalized_weight;
    }
    
    let mixed_lab = Lab::new(l, a, b);
    conversion::lab_to_srgb(mixed_lab)
}

/// Alias for `lab_interpolation` - recommended mixing method
pub fn mix(color1: Srgb, color2: Srgb, factor: f32) -> Srgb {
    lab_interpolation(color1, color2, factor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Srgb;

    #[test]
    fn test_linear_rgb_mixing() {
        let red = Srgb::new(1.0, 0.0, 0.0);
        let blue = Srgb::new(0.0, 0.0, 1.0);
        
        let mixed = linear_rgb(red, blue, 0.5);
        assert!((mixed.red - 0.5).abs() < 1e-6);
        assert!(mixed.green < 1e-6);
        assert!((mixed.blue - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_mixing_extremes() {
        let color1 = Srgb::new(0.8, 0.3, 0.6);
        let color2 = Srgb::new(0.2, 0.7, 0.1);
        
        // Factor 0.0 should return color1
        let mixed_0 = linear_rgb(color1, color2, 0.0);
        assert!((mixed_0.red - color1.red).abs() < 1e-6);
        assert!((mixed_0.green - color1.green).abs() < 1e-6);
        assert!((mixed_0.blue - color1.blue).abs() < 1e-6);
        
        // Factor 1.0 should return color2
        let mixed_1 = linear_rgb(color1, color2, 1.0);
        assert!((mixed_1.red - color2.red).abs() < 1e-6);
        assert!((mixed_1.green - color2.green).abs() < 1e-6);
        assert!((mixed_1.blue - color2.blue).abs() < 1e-6);
    }

    #[test]
    fn test_multiply_blend() {
        let white = Srgb::new(1.0, 1.0, 1.0);
        let gray = Srgb::new(0.5, 0.5, 0.5);
        
        let blended = multiply_blend(white, gray, 1.0);
        // Multiply should give (1.0 * 0.5, 1.0 * 0.5, 1.0 * 0.5) = (0.5, 0.5, 0.5)
        assert!((blended.red - 0.5).abs() < 1e-6);
        assert!((blended.green - 0.5).abs() < 1e-6);
        assert!((blended.blue - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_screen_blend() {
        let black = Srgb::new(0.0, 0.0, 0.0);
        let gray = Srgb::new(0.5, 0.5, 0.5);
        
        let blended = screen_blend(black, gray, 1.0);
        // Screen should give 1 - (1-0) * (1-0.5) = 1 - 1 * 0.5 = 0.5
        assert!((blended.red - 0.5).abs() < 1e-6);
        assert!((blended.green - 0.5).abs() < 1e-6);
        assert!((blended.blue - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_create_palette() {
        let keys = vec![
            Srgb::new(1.0, 0.0, 0.0), // Red
            Srgb::new(0.0, 1.0, 0.0), // Green
        ];
        
        let palette = create_palette(&keys, 3, ColorSpace::Rgb);
        assert_eq!(palette.len(), 3);
        
        // First should be red
        assert!((palette[0].red - 1.0).abs() < 1e-6);
        assert!(palette[0].green < 1e-6);
        
        // Last should be green
        assert!(palette[2].red < 1e-6);
        assert!((palette[2].green - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_weighted_mix() {
        let colors = &[
            (Srgb::new(1.0, 0.0, 0.0), 3.0), // Red with weight 3
            (Srgb::new(0.0, 1.0, 0.0), 1.0), // Green with weight 1
        ];
        
        let mixed = weighted_mix(colors, ColorSpace::Rgb).unwrap();
        // Should be 3/4 red + 1/4 green = (0.75, 0.25, 0.0)
        assert!((mixed.red - 0.75).abs() < 1e-6);
        assert!((mixed.green - 0.25).abs() < 1e-6);
        assert!(mixed.blue < 1e-6);
    }

    #[test]
    fn test_weighted_mix_empty() {
        let colors: &[(Srgb, f32)] = &[];
        assert!(weighted_mix(colors, ColorSpace::Rgb).is_none());
    }

    #[test]
    fn test_mix_alias() {
        let color1 = Srgb::new(0.8, 0.2, 0.5);
        let color2 = Srgb::new(0.3, 0.9, 0.1);
        
        assert_eq!(mix(color1, color2, 0.6), lab_interpolation(color1, color2, 0.6));
    }
}
