//! Color mixing and blending module
//!
//! This module has been decomposed into focused submodules following functional
//! programming principles and single responsibility design.
//!
//! ## Submodule Organization
//! - `interpolation` - Color space interpolation methods (RGB, LAB, LCH, HSL, HSV)
//! - `blending` - Color blending algorithms (multiply, screen, overlay)
//! - `utilities` - High-level mixing utilities (palette creation, weighted mixing)
//!
//! ## Main Functions
//! - `mix()` - Recommended general-purpose mixing (LAB interpolation)
//! - `linear_rgb()` - Fast RGB interpolation
//! - `lab_interpolation()` - Perceptually uniform mixing
//! - `create_palette()` - Generate color palettes
//! - `weighted_mix()` - Mix multiple colors with weights
//!
//! ## Example Usage
//! ```rust
//! use color_rs::color_ops::mixing;
//! use palette::Srgb;
//!
//! let red = Srgb::new(1.0, 0.0, 0.0);
//! let blue = Srgb::new(0.0, 0.0, 1.0);
//! 
//! // General-purpose mixing
//! let purple = mixing::mix(red, blue, 0.5);
//! 
//! // Specific interpolation method
//! let smooth = mixing::lab_interpolation(red, blue, 0.3);
//! 
//! // Create a gradient palette
//! let keys = vec![red, blue];
//! let palette = mixing::create_palette(&keys, 10, mixing::ColorSpace::Lab);
//! ```

pub mod interpolation;
pub mod blending;
pub mod utilities;

// Re-export main functions for backward compatibility
pub use interpolation::{
    linear_rgb,
    lab_interpolation,
    lch_interpolation,
    hsl_interpolation,
    hsv_interpolation,
};

pub use blending::{
    multiply_blend,
    screen_blend,
    overlay_blend,
};

pub use utilities::{
    create_palette,
    weighted_mix,
    mix,
    ColorSpace,
};

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Srgb;

    #[test]
    fn test_basic_mixing() {
        let red = Srgb::new(1.0, 0.0, 0.0);
        let blue = Srgb::new(0.0, 0.0, 1.0);
        
        let mixed = linear_rgb(red, blue, 0.5);
        assert!((mixed.red - 0.5).abs() < 1e-6);
        assert!(mixed.green < 1e-6);
        assert!((mixed.blue - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_blending_operations() {
        let white = Srgb::new(1.0, 1.0, 1.0);
        let gray = Srgb::new(0.5, 0.5, 0.5);
        
        let blended = multiply_blend(white, gray, 1.0);
        assert!((blended.red - 0.5).abs() < 1e-6);
        assert!((blended.green - 0.5).abs() < 1e-6);
        assert!((blended.blue - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_palette_creation() {
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
    fn test_weighted_mixing() {
        let colors = &[
            (Srgb::new(1.0, 0.0, 0.0), 3.0), // Red with weight 3
            (Srgb::new(0.0, 1.0, 0.0), 1.0), // Green with weight 1
        ];
        
        let mixed = weighted_mix(colors, ColorSpace::Rgb).unwrap();
        assert!((mixed.red - 0.75).abs() < 1e-6);
        assert!((mixed.green - 0.25).abs() < 1e-6);
        assert!(mixed.blue < 1e-6);
    }

    #[test] 
    fn test_module_re_exports() {
        // Verify all expected functions are accessible
        let red = Srgb::new(1.0, 0.0, 0.0);
        let blue = Srgb::new(0.0, 0.0, 1.0);
        
        // Interpolation methods
        let _rgb = linear_rgb(red, blue, 0.5);
        let _lab = lab_interpolation(red, blue, 0.5);
        let _lch = lch_interpolation(red, blue, 0.5);
        let _hsl = hsl_interpolation(red, blue, 0.5);
        let _hsv = hsv_interpolation(red, blue, 0.5);
        
        // Blending methods
        let _mult = multiply_blend(red, blue, 0.5);
        let _screen = screen_blend(red, blue, 0.5);
        let _overlay = overlay_blend(red, blue, 0.5);
        
        // Utility methods
        let _mixed = mix(red, blue, 0.5);
        let keys = vec![red, blue];
        let _palette = create_palette(&keys, 5, ColorSpace::Lab);
        let colors = &[(red, 1.0), (blue, 1.0)];
        let _weighted = weighted_mix(colors, ColorSpace::Rgb);
    }
}
