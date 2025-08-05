//! Mixing utility functions
//!
//! High-level utility functions for creating color palettes, weighted mixing,
//! and other complex color manipulation operations.

use palette::Srgb;
use super::interpolation::{linear_rgb, lab_interpolation, lch_interpolation, hsl_interpolation, hsv_interpolation};

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
/// use color_rs::color_ops::mixing::utilities;
/// use palette::Srgb;
///
/// let keys = vec![
///     Srgb::new(1.0, 0.0, 0.0), // Red
///     Srgb::new(0.0, 1.0, 0.0), // Green
///     Srgb::new(0.0, 0.0, 1.0), // Blue
/// ];
/// let palette = utilities::create_palette(&keys, 10, utilities::ColorSpace::Lab);
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
/// use color_rs::color_ops::mixing::utilities;
/// use palette::Srgb;
///
/// let colors = &[
///     (Srgb::new(1.0, 0.0, 0.0), 2.0), // Red with weight 2
///     (Srgb::new(0.0, 1.0, 0.0), 1.0), // Green with weight 1
///     (Srgb::new(0.0, 0.0, 1.0), 1.0), // Blue with weight 1
/// ];
/// let mixed = utilities::weighted_mix(colors, utilities::ColorSpace::Lab).unwrap();
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

/// Alias for `lab_interpolation` - recommended mixing method
pub fn mix(color1: Srgb, color2: Srgb, factor: f32) -> Srgb {
    lab_interpolation(color1, color2, factor)
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
    use palette::{Lab, IntoColor};
    
    let mut l = 0.0f32;
    let mut a = 0.0f32;
    let mut b = 0.0f32;
    
    for &(color, weight) in colors_and_weights {
        let lab: Lab = color.into_color();
        let normalized_weight = weight / total_weight;
        l += lab.l * normalized_weight;
        a += lab.a * normalized_weight;
        b += lab.b * normalized_weight;
    }
    
    let mixed_lab = Lab::new(l, a, b);
    mixed_lab.into_color()
}
