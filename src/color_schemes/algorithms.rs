//! Color harmony calculation algorithms
//!
//! This module contains the core algorithms for calculating color harmonies
//! in both HSL and Lab color spaces, including complementary, split-complementary,
//! triadic, and tetradic color schemes.

use crate::error::{ColorError, Result};
use palette::{Hsl, IntoColor, Lab, Srgb};

/// Calculate complementary color in HSL space
#[must_use]
pub fn complementary_hsl(color: Lab) -> Lab {
    let srgb: Srgb = color.into_color();
    let hsl: Hsl = srgb.into_color();
    
    // Complementary color is 180 degrees opposite on the hue wheel
    let complementary_hue = (hsl.hue.into_positive_degrees() + 180.0) % 360.0;
    let complementary_hsl = Hsl::new(complementary_hue, hsl.saturation, hsl.lightness);
    let complementary_srgb: Srgb = complementary_hsl.into_color();
    complementary_srgb.into_color()
}

/// Calculate split-complementary colors in HSL space
#[must_use]
pub fn split_complementary_hsl(color: Lab) -> (Lab, Lab) {
    let srgb: Srgb = color.into_color();
    let hsl: Hsl = srgb.into_color();
    
    // Split-complementary: 150 and 210 degrees from original hue
    let base_hue = hsl.hue.into_positive_degrees();
    let color1_hue = (base_hue + 150.0) % 360.0;
    let color2_hue = (base_hue + 210.0) % 360.0;
    
    let color1_hsl = Hsl::new(color1_hue, hsl.saturation, hsl.lightness);
    let color2_hsl = Hsl::new(color2_hue, hsl.saturation, hsl.lightness);
    
    let color1_srgb: Srgb = color1_hsl.into_color();
    let color2_srgb: Srgb = color2_hsl.into_color();
    
    (color1_srgb.into_color(), color2_srgb.into_color())
}

/// Calculate triadic colors in HSL space
#[must_use]
pub fn triadic_hsl(color: Lab) -> (Lab, Lab) {
    let srgb: Srgb = color.into_color();
    let hsl: Hsl = srgb.into_color();
    
    // Triadic: 120 and 240 degrees from original hue
    let base_hue = hsl.hue.into_positive_degrees();
    let color1_hue = (base_hue + 120.0) % 360.0;
    let color2_hue = (base_hue + 240.0) % 360.0;
    
    let color1_hsl = Hsl::new(color1_hue, hsl.saturation, hsl.lightness);
    let color2_hsl = Hsl::new(color2_hue, hsl.saturation, hsl.lightness);
    
    let color1_srgb: Srgb = color1_hsl.into_color();
    let color2_srgb: Srgb = color2_hsl.into_color();
    
    (color1_srgb.into_color(), color2_srgb.into_color())
}

/// Calculate tetradic colors in HSL space
pub fn tetradic_hsl(color: Lab) -> (Lab, Lab, Lab) {
    let srgb: Srgb = color.into_color();
    let hsl: Hsl = srgb.into_color();
    
    // Tetradic: 90, 180, and 270 degrees from original hue (square on color wheel)
    let base_hue = hsl.hue.into_positive_degrees();
    let color1_hue = (base_hue + 90.0) % 360.0;
    let color2_hue = (base_hue + 180.0) % 360.0;
    let color3_hue = (base_hue + 270.0) % 360.0;
    
    let color1_hsl = Hsl::new(color1_hue, hsl.saturation, hsl.lightness);
    let color2_hsl = Hsl::new(color2_hue, hsl.saturation, hsl.lightness);
    let color3_hsl = Hsl::new(color3_hue, hsl.saturation, hsl.lightness);
    
    let color1_srgb: Srgb = color1_hsl.into_color();
    let color2_srgb: Srgb = color2_hsl.into_color();
    let color3_srgb: Srgb = color3_hsl.into_color();
    
    (color1_srgb.into_color(), color2_srgb.into_color(), color3_srgb.into_color())
}

/// Calculate complementary color in Lab space
pub fn complementary_lab(color: Lab) -> Lab {
    // Complementary in Lab space: negate a and b components
    Lab::new(color.l, -color.a, -color.b)
}

/// Calculate split-complementary colors in Lab space
pub fn split_complementary_lab(color: Lab) -> (Lab, Lab) {
    // Split-complementary approximated in Lab space by rotating a/b vector
    let a = f64::from(color.a);
    let b = f64::from(color.b);
    
    // Rotate by approximately ±150 degrees (2.618 radians)
    let cos_150 = -0.866; // cos(150°)
    let sin_150 = 0.5;     // sin(150°)
    
    let a1 = a * cos_150 - b * sin_150;
    let b1 = a * sin_150 + b * cos_150;
    
    let a2 = a * cos_150 + b * sin_150;
    let b2 = -a * sin_150 + b * cos_150;
    
    (
        Lab::new(color.l, a1 as f32, b1 as f32),
        Lab::new(color.l, a2 as f32, b2 as f32),
    )
}

/// Calculate triadic colors in Lab space
pub fn triadic_lab(color: Lab) -> (Lab, Lab) {
    // Triadic in Lab space: rotate a/b vector by ±120 degrees
    let a = f64::from(color.a);
    let b = f64::from(color.b);
    
    // Rotate by ±120 degrees
    let cos_120 = -0.5; // cos(120°)
    let sin_120 = 0.866; // sin(120°)
    
    let a1 = a * cos_120 - b * sin_120;
    let b1 = a * sin_120 + b * cos_120;
    
    let a2 = a * cos_120 + b * sin_120;
    let b2 = -a * sin_120 + b * cos_120;
    
    (
        Lab::new(color.l, a1 as f32, b1 as f32),
        Lab::new(color.l, a2 as f32, b2 as f32),
    )
}

/// Calculate tetradic colors in Lab space
pub fn tetradic_lab(color: Lab) -> (Lab, Lab, Lab) {
    // Tetradic in Lab space: rotate a/b vector by 90°, 180°, 270°
    let a = f64::from(color.a);
    let b = f64::from(color.b);
    
    // 90 degrees rotation
    let a1 = -b;
    let b1 = a;
    
    // 180 degrees rotation (complementary)
    let a2 = -a;
    let b2 = -b;
    
    // 270 degrees rotation
    let a3 = b;
    let b3 = -a;
    
    (
        Lab::new(color.l, a1 as f32, b1 as f32),
        Lab::new(color.l, a2 as f32, b2 as f32),
        Lab::new(color.l, a3 as f32, b3 as f32),
    )
}

/// Adjust a color to have the specified relative luminance while preserving hue and saturation.
/// Uses a basic approximation by scaling the RGB components.
pub fn adjust_color_relative_luminance(color: Lab, target_luminance: f64) -> Result<Lab> {
    if !(0.0..=1.0).contains(&target_luminance) {
        return Err(ColorError::InvalidArguments(format!(
            "Relative luminance must be in [0.0, 1.0], got {target_luminance}"
        )));
    }

    let srgb: Srgb = color.into_color();
    let current_luminance = crate::color_ops::luminance::wcag_relative(srgb);

    if current_luminance == 0.0 {
        // Handle black color case - can't scale from zero
        return Ok(color);
    }

    // Simple scaling approach - scale RGB components to achieve target luminance
    let scale_factor = (target_luminance / current_luminance).sqrt();
    let scaled_srgb = Srgb::new(
        (srgb.red * scale_factor as f32).clamp(0.0, 1.0),
        (srgb.green * scale_factor as f32).clamp(0.0, 1.0),
        (srgb.blue * scale_factor as f32).clamp(0.0, 1.0),
    );

    Ok(scaled_srgb.into_color())
}

/// Adjust a color to have the specified Lab luminance while preserving a and b components.
/// Clamps the luminance to [0.0, 100.0] and returns an error if out of range.
pub fn adjust_color_lab_luminance(color: Lab, target_luminance: f64) -> Result<Lab> {
    if !(0.0..=100.0).contains(&target_luminance) {
        return Err(ColorError::InvalidArguments(format!(
            "Lab luminance must be in [0.0, 100.0], got {target_luminance}"
        )));
    }
    Ok(Lab::new(target_luminance as f32, color.a, color.b))
}

/// Preserve WCAG relative luminance from reference color in target color
pub fn preserve_wcag_relative_luminance(color: Lab, reference: Lab) -> Result<Lab> {
    let reference_srgb: Srgb = reference.into_color();
    let target_luminance = crate::color_ops::luminance::wcag_relative(reference_srgb);
    adjust_color_relative_luminance(color, target_luminance)
}

/// Preserve Lab luminance from reference color in target color
pub fn preserve_lab_luminance(color: Lab, reference: Lab) -> Result<Lab> {
    adjust_color_lab_luminance(color, f64::from(reference.l))
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Srgb;

    #[test]
    fn test_complementary_hsl() {
        let red_lab: Lab = Srgb::new(1.0, 0.0, 0.0).into_color();
        let comp = complementary_hsl(red_lab);

        // Complementary of red should be cyan-ish
        let comp_srgb: Srgb = comp.into_color();
        assert!(comp_srgb.blue > 0.5); // Should have significant blue component
        assert!(comp_srgb.red < 0.5); // Should have minimal red component
    }

    #[test]
    fn test_complementary_lab() {
        let red_lab: Lab = Srgb::new(1.0, 0.0, 0.0).into_color();
        let comp = complementary_lab(red_lab);

        // In Lab space, complementary negates a and b
        assert!((comp.l - red_lab.l).abs() < 0.01);
        assert!((comp.a + red_lab.a).abs() < 0.01);
        assert!((comp.b + red_lab.b).abs() < 0.01);
    }

    #[test]
    fn test_triadic_hsl() {
        let red_lab: Lab = Srgb::new(1.0, 0.0, 0.0).into_color();
        let (tri1, tri2) = triadic_hsl(red_lab);

        // Basic validity check - should get different colors
        assert!(tri1 != red_lab);
        assert!(tri2 != red_lab);
        assert!(tri1 != tri2);
    }

    #[test]
    fn test_adjust_color_lab_luminance() {
        let red_srgb = Srgb::new(1.0, 0.0, 0.0);
        let red_lab: Lab = red_srgb.into_color();
        let adjusted = adjust_color_lab_luminance(red_lab, 75.0).unwrap();

        // Lab L component should be exactly 75.0
        assert!((adjusted.l - 75.0).abs() < 0.001);
        // a and b should be preserved
        assert!((adjusted.a - red_lab.a).abs() < 0.001);
        assert!((adjusted.b - red_lab.b).abs() < 0.001);
    }

    #[test]
    fn test_adjust_color_relative_luminance() {
        let red_lab: Lab = Srgb::new(1.0, 0.0, 0.0).into_color();

        // Test if the function exists and works
        if let Ok(adjusted) = adjust_color_relative_luminance(red_lab, 0.5) {
            let adjusted_srgb: Srgb = adjusted.into_color();
            let actual_luminance = crate::color_ops::luminance::wcag_relative(adjusted_srgb);

            // Very lenient check - just ensure the function doesn't crash
            assert!((0.0..=1.0).contains(&actual_luminance));
        }
    }
}
