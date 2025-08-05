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
pub mod analysis;
pub mod contrast;
pub mod conversion;
pub mod distance;
pub mod luminance;
pub mod mixing;

// Re-export commonly used functions for convenience
pub use analysis::{
    AccessibilityData, ColorAnalysis, ColorComparison, ColorProperties, ColorSpaces,
    PerceptualData, analyze_color, compare_colors,
};
pub use contrast::{compliance_level, meets_aa_standard, meets_aaa_standard, ratio, wcag_ratio};
pub use conversion::{
    hex_to_srgb, rgb_tuple_to_srgb, srgb_to_hex, srgb_to_hsl, srgb_to_hsv, srgb_to_lab,
    srgb_to_lch, srgb_to_rgb_tuple,
};
pub use distance::{delta_e_2000, delta_e_cie76, delta_e_cie94, find_closest, perceptual_distance};
pub use luminance::{perceived_brightness, relative_luminance, wcag_relative};
pub use mixing::{
    ColorSpace, create_palette, lab_interpolation, lch_interpolation, linear_rgb, mix,
    multiply_blend, overlay_blend, screen_blend, weighted_mix,
};
