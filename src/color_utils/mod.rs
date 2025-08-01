//! Minimal color utilities module
//!
//! This module provides a simplified interface for basic color utility functions
//! using pure functional programming patterns with the palette crate.

use palette::{IntoColor, Lab, Srgb, color_difference::DeltaE};

/// Calculate perceptual distance between two LAB colors using DeltaE
pub fn calculate_perceptual_distance(color1: Lab, color2: Lab) -> f64 {
    color1.delta_e(color2) as f64
}

/// Calculate WCAG relative luminance for RGB color
pub fn calculate_wcag_luminance(srgb: Srgb) -> f64 {
    let linear = srgb.into_linear();
    0.2126 * linear.red as f64 + 0.7152 * linear.green as f64 + 0.0722 * linear.blue as f64
}

/// Quick conversion from LAB to RGB
pub fn quick_convert(lab: Lab) -> Srgb {
    lab.into_color()
}

/// Simple perceptual interpolation between two LAB colors
pub fn interpolate_perceptual(start: Lab, end: Lab, factor: f64) -> Lab {
    use palette::Mix;
    start.mix(end, factor as f32)
}
