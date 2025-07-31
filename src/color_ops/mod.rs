//! Color operations module
//!
//! Functional color operations organized by category.
//! Replaces the facade pattern with direct function access.
//!
//! # Design Philosophy
//! - Zero-cost abstractions
//! - Pure functions without side effects
//! - Modular organization by operation type
//! - Direct function access (no wrapper objects)
//! - Compile-time optimizations
//!
//! # Module Organization
//! 
//! - **luminance**: WCAG luminance and brightness calculations
//! - **distance**: Perceptual and mathematical color distance metrics
//! - **contrast**: WCAG contrast ratios and accessibility compliance
//! - **conversion**: Color space transformations and format conversions
//! - **analysis**: Comprehensive color analysis and comparison
//! - **mixing**: Color blending, interpolation, and palette generation
//!
//! # Usage Examples
//!
//! ```rust
//! use color_rs::color_ops;
//! use palette::Srgb;
//!
//! let color = Srgb::new(0.8, 0.3, 0.6);
//!
//! // Luminance calculation
//! let lum = color_ops::wcag_relative(color);
//!
//! // Color analysis
//! let analysis = color_ops::analyze_color(color);
//!
//! // Distance between colors
//! let other = Srgb::new(0.2, 0.7, 0.4);
//! let distance = color_ops::perceptual_distance(color, other);
//!
//! // Contrast ratio
//! let ratio = color_ops::wcag_ratio(color, other);
//!
//! // Color mixing
//! let mixed = color_ops::mix(color, other, 0.5);
//!
//! // Hex conversion
//! let hex = color_ops::srgb_to_hex(color);
//! let from_hex = color_ops::hex_to_srgb("#FF8040").unwrap();
//! ```

// Core operation modules
pub mod luminance;
pub mod distance;
pub mod contrast;
pub mod conversion;
pub mod analysis;
pub mod mixing;

// Re-export commonly used functions for convenience
pub use luminance::{wcag_relative, perceived_brightness, relative_luminance};
pub use distance::{delta_e_2000, perceptual_distance, find_closest, delta_e_cie76, delta_e_cie94};
pub use contrast::{wcag_ratio, meets_aa_standard, meets_aaa_standard, compliance_level, ratio};
pub use conversion::{
    srgb_to_hsl, srgb_to_lab, srgb_to_lch, srgb_to_hsv,
    hex_to_srgb, srgb_to_hex, rgb_tuple_to_srgb, srgb_to_rgb_tuple
};
pub use analysis::{
    analyze_color, compare_colors, ColorAnalysis, ColorComparison,
    ColorProperties, ColorSpaces, PerceptualData, AccessibilityData
};
pub use mixing::{
    mix, lab_interpolation, lch_interpolation, linear_rgb,
    create_palette, weighted_mix, ColorSpace,
    multiply_blend, screen_blend, overlay_blend
};
