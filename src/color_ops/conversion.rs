//! Color space conversion functions
//!
//! Pure functions for converting between different color spaces.
//! All conversions use the palette crate's color space implementations.

use palette::{Hsl, Hsv, Lab, Lch, Srgb, Xyz, IntoColor};

// ============================================================================
// RGB Conversions
// ============================================================================

/// Convert sRGB to HSL color space
///
/// # Arguments
/// * `srgb` - Source color in sRGB space
///
/// # Returns
/// * Color in HSL space (Hue: 0-360°, Saturation: 0-1, Lightness: 0-1)
///
/// # Example
/// ```rust
/// use color_rs::color_ops::conversion;
/// use palette::Srgb;
///
/// let red = Srgb::new(1.0, 0.0, 0.0);
/// let hsl = conversion::srgb_to_hsl(red);
/// assert!((hsl.hue.into_inner() - 0.0).abs() < 1e-6); // Red hue
/// ```
pub fn srgb_to_hsl(srgb: Srgb) -> Hsl {
    srgb.into_color()
}

/// Convert HSL to sRGB color space
///
/// # Arguments
/// * `hsl` - Source color in HSL space
///
/// # Returns
/// * Color in sRGB space
///
/// # Example
/// ```rust
/// use color_rs::color_ops::conversion;
/// use palette::{Hsl, Srgb};
///
/// let hsl = Hsl::new(120.0, 1.0, 0.5); // Pure green
/// let srgb = conversion::hsl_to_srgb(hsl);
/// ```
pub fn hsl_to_srgb(hsl: Hsl) -> Srgb {
    hsl.into_color()
}

/// Convert sRGB to HSV color space
///
/// # Arguments
/// * `srgb` - Source color in sRGB space
///
/// # Returns
/// * Color in HSV space (Hue: 0-360°, Saturation: 0-1, Value: 0-1)
///
/// # Example
/// ```rust
/// use color_rs::color_ops::conversion;
/// use palette::Srgb;
///
/// let blue = Srgb::new(0.0, 0.0, 1.0);
/// let hsv = conversion::srgb_to_hsv(blue);
/// assert!((hsv.hue.into_inner() - 240.0).abs() < 1e-6); // Blue hue
/// ```
pub fn srgb_to_hsv(srgb: Srgb) -> Hsv {
    srgb.into_color()
}

/// Convert HSV to sRGB color space
///
/// # Arguments
/// * `hsv` - Source color in HSV space
///
/// # Returns
/// * Color in sRGB space
///
/// # Example
/// ```rust
/// use color_rs::color_ops::conversion;
/// use palette::{Hsv, Srgb};
///
/// let hsv = Hsv::new(240.0, 1.0, 1.0); // Pure blue
/// let srgb = conversion::hsv_to_srgb(hsv);
/// ```
pub fn hsv_to_srgb(hsv: Hsv) -> Srgb {
    hsv.into_color()
}

// ============================================================================
// LAB Conversions
// ============================================================================

/// Convert sRGB to CIELAB color space
///
/// CIELAB is perceptually uniform, making it ideal for color difference calculations.
///
/// # Arguments
/// * `srgb` - Source color in sRGB space
///
/// # Returns
/// * Color in CIELAB space (L: 0-100, a: -128 to +127, b: -128 to +127)
///
/// # Example
/// ```rust
/// use color_rs::color_ops::conversion;
/// use palette::Srgb;
///
/// let white = Srgb::new(1.0, 1.0, 1.0);
/// let lab = conversion::srgb_to_lab(white);
/// assert!(lab.l > 95.0); // White has high lightness
/// ```
pub fn srgb_to_lab(srgb: Srgb) -> Lab {
    srgb.into_color()
}

/// Convert CIELAB to sRGB color space
///
/// # Arguments
/// * `lab` - Source color in CIELAB space
///
/// # Returns
/// * Color in sRGB space (may be clamped to valid RGB range)
///
/// # Example
/// ```rust
/// use color_rs::color_ops::conversion;
/// use palette::Lab;
///
/// let lab = Lab::new(50.0, 0.0, 0.0); // Neutral gray
/// let srgb = conversion::lab_to_srgb(lab);
/// ```
pub fn lab_to_srgb(lab: Lab) -> Srgb {
    lab.into_color()
}

// ============================================================================
// LCH Conversions
// ============================================================================

/// Convert sRGB to LCH color space
///
/// LCH (Lightness, Chroma, Hue) is CIELAB in cylindrical coordinates,
/// making it intuitive for color manipulation.
///
/// # Arguments
/// * `srgb` - Source color in sRGB space
///
/// # Returns
/// * Color in LCH space (L: 0-100, C: 0+, H: 0-360°)
///
/// # Example
/// ```rust
/// use color_rs::color_ops::conversion;
/// use palette::Srgb;
///
/// let red = Srgb::new(1.0, 0.0, 0.0);
/// let lch = conversion::srgb_to_lch(red);
/// ```
pub fn srgb_to_lch(srgb: Srgb) -> Lch {
    srgb.into_color()
}

/// Convert LCH to sRGB color space
///
/// # Arguments
/// * `lch` - Source color in LCH space
///
/// # Returns
/// * Color in sRGB space (may be clamped to valid RGB range)
///
/// # Example
/// ```rust
/// use color_rs::color_ops::conversion;
/// use palette::Lch;
///
/// let lch = Lch::new(70.0, 50.0, 180.0); // Cyan-ish color
/// let srgb = conversion::lch_to_srgb(lch);
/// ```
pub fn lch_to_srgb(lch: Lch) -> Srgb {
    lch.into_color()
}

// ============================================================================
// XYZ Conversions
// ============================================================================

/// Convert sRGB to CIE XYZ color space
///
/// XYZ is the foundation color space for most other color spaces.
///
/// # Arguments
/// * `srgb` - Source color in sRGB space
///
/// # Returns
/// * Color in CIE XYZ space
///
/// # Example
/// ```rust
/// use color_rs::color_ops::conversion;
/// use palette::Srgb;
///
/// let green = Srgb::new(0.0, 1.0, 0.0);
/// let xyz = conversion::srgb_to_xyz(green);
/// ```
pub fn srgb_to_xyz(srgb: Srgb) -> Xyz {
    srgb.into_color()
}

/// Convert CIE XYZ to sRGB color space
///
/// # Arguments
/// * `xyz` - Source color in CIE XYZ space
///
/// # Returns
/// * Color in sRGB space (may be clamped to valid RGB range)
///
/// # Example
/// ```rust
/// use color_rs::color_ops::conversion;
/// use palette::Xyz;
///
/// let xyz = Xyz::new(0.5, 0.5, 0.5);
/// let srgb = conversion::xyz_to_srgb(xyz);
/// ```
pub fn xyz_to_srgb(xyz: Xyz) -> Srgb {
    xyz.into_color()
}

// ============================================================================
// RGB Tuple Conversions
// ============================================================================

/// Convert RGB tuple to sRGB
///
/// Convenience function for converting (u8, u8, u8) tuples to sRGB.
///
/// # Arguments
/// * `rgb` - RGB values as (u8, u8, u8) tuple
///
/// # Returns
/// * Color in sRGB space with values 0.0-1.0
///
/// # Example
/// ```rust
/// use color_rs::color_ops::conversion;
///
/// let srgb = conversion::rgb_tuple_to_srgb((255, 128, 0));
/// assert!((srgb.red - 1.0).abs() < 1e-6);
/// assert!((srgb.green - 0.5019607843137255).abs() < 1e-6);
/// ```
pub fn rgb_tuple_to_srgb(rgb: (u8, u8, u8)) -> Srgb {
    Srgb::new(
        rgb.0 as f32 / 255.0,
        rgb.1 as f32 / 255.0,
        rgb.2 as f32 / 255.0,
    )
}

/// Convert sRGB to RGB tuple
///
/// Convenience function for converting sRGB to (u8, u8, u8) tuples.
///
/// # Arguments
/// * `srgb` - Source color in sRGB space
///
/// # Returns
/// * RGB values as (u8, u8, u8) tuple
///
/// # Example
/// ```rust
/// use color_rs::color_ops::conversion;
/// use palette::Srgb;
///
/// let srgb = Srgb::new(1.0, 0.5, 0.0);
/// let rgb = conversion::srgb_to_rgb_tuple(srgb);
/// assert_eq!(rgb, (255, 127, 0));
/// ```
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // Safe: values clamped to [0.0, 255.0] range
pub fn srgb_to_rgb_tuple(srgb: Srgb) -> (u8, u8, u8) {
    (
        (srgb.red * 255.0).round().clamp(0.0, 255.0) as u8,
        (srgb.green * 255.0).round().clamp(0.0, 255.0) as u8,
        (srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8,
    )
}

// ============================================================================
// Hex String Conversions
// ============================================================================

/// Convert hex string to sRGB
///
/// Supports both #RGB and #RRGGBB formats.
///
/// # Arguments
/// * `hex` - Hex color string (with or without # prefix)
///
/// # Returns
/// * `Ok(Srgb)` if valid hex, `Err(String)` if invalid
///
/// # Example
/// ```rust
/// use color_rs::color_ops::conversion;
///
/// let srgb = conversion::hex_to_srgb("#FF8000").unwrap();
/// assert!((srgb.red - 1.0).abs() < 1e-6);
/// ```
pub fn hex_to_srgb(hex: &str) -> Result<Srgb, String> {
    let hex = hex.trim_start_matches('#');
    
    match hex.len() {
        3 => {
            // #RGB format
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)
                .map_err(|_| format!("Invalid hex color: #{hex}"))?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)
                .map_err(|_| format!("Invalid hex color: #{hex}"))?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)
                .map_err(|_| format!("Invalid hex color: #{hex}"))?;
            Ok(rgb_tuple_to_srgb((r, g, b)))
        }
        6 => {
            // #RRGGBB format
            let r = u8::from_str_radix(&hex[0..2], 16)
                .map_err(|_| format!("Invalid hex color: #{hex}"))?;
            let g = u8::from_str_radix(&hex[2..4], 16)
                .map_err(|_| format!("Invalid hex color: #{hex}"))?;
            let b = u8::from_str_radix(&hex[4..6], 16)
                .map_err(|_| format!("Invalid hex color: #{hex}"))?;
            Ok(rgb_tuple_to_srgb((r, g, b)))
        }
        _ => Err(format!("Invalid hex color length: #{hex}")),
    }
}

/// Convert sRGB to hex string
///
/// Returns uppercase hex string with # prefix.
///
/// # Arguments
/// * `srgb` - Source color in sRGB space
///
/// # Returns
/// * Hex color string in #RRGGBB format
///
/// # Example
/// ```rust
/// use color_rs::color_ops::conversion;
/// use palette::Srgb;
///
/// let srgb = Srgb::new(1.0, 0.5, 0.0);
/// let hex = conversion::srgb_to_hex(srgb);
/// assert_eq!(hex, "#FF7F00");
/// ```
pub fn srgb_to_hex(srgb: Srgb) -> String {
    let (r, g, b) = srgb_to_rgb_tuple(srgb);
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Srgb;

    #[test]
    fn test_srgb_hsl_roundtrip() {
        let original = Srgb::new(0.8, 0.3, 0.6);
        let hsl = srgb_to_hsl(original);
        let converted = hsl_to_srgb(hsl);
        
        assert!((original.red - converted.red).abs() < 1e-6);
        assert!((original.green - converted.green).abs() < 1e-6);
        assert!((original.blue - converted.blue).abs() < 1e-6);
    }

    #[test]
    fn test_srgb_hsv_roundtrip() {
        let original = Srgb::new(0.7, 0.2, 0.9);
        let hsv = srgb_to_hsv(original);
        let converted = hsv_to_srgb(hsv);
        
        assert!((original.red - converted.red).abs() < 1e-6);
        assert!((original.green - converted.green).abs() < 1e-6);
        assert!((original.blue - converted.blue).abs() < 1e-6);
    }

    #[test]
    fn test_srgb_lab_roundtrip() {
        let original = Srgb::new(0.5, 0.7, 0.2);
        let lab = srgb_to_lab(original);
        let converted = lab_to_srgb(lab);
        
        // LAB conversion may have some precision loss
        assert!((original.red - converted.red).abs() < 1e-3);
        assert!((original.green - converted.green).abs() < 1e-3);
        assert!((original.blue - converted.blue).abs() < 1e-3);
    }

    #[test]
    fn test_rgb_tuple_conversions() {
        let rgb = (255, 128, 64);
        let srgb = rgb_tuple_to_srgb(rgb);
        let converted = srgb_to_rgb_tuple(srgb);
        
        assert_eq!(rgb, converted);
    }

    #[test]
    fn test_hex_conversions() {
        // Test #RRGGBB format
        let srgb = hex_to_srgb("#FF8040").unwrap();
        let hex = srgb_to_hex(srgb);
        assert_eq!(hex, "#FF8040");
        
        // Test #RGB format
        let srgb_short = hex_to_srgb("#F84").unwrap();
        assert!((srgb_short.red - (255.0 / 255.0)).abs() < 1e-6);
        assert!((srgb_short.green - (136.0 / 255.0)).abs() < 1e-6);
        assert!((srgb_short.blue - (68.0 / 255.0)).abs() < 1e-6);
        
        // Test without # prefix
        let srgb_no_hash = hex_to_srgb("FF8040").unwrap();
        assert_eq!(srgb_to_hex(srgb_no_hash), "#FF8040");
    }

    #[test]
    fn test_hex_error_cases() {
        assert!(hex_to_srgb("#GG0000").is_err()); // Invalid hex
        assert!(hex_to_srgb("#FF00").is_err());   // Wrong length
        assert!(hex_to_srgb("#FF00000").is_err()); // Wrong length
    }

    #[test]
    fn test_primary_colors() {
        // Test red in HSL
        let red = Srgb::new(1.0, 0.0, 0.0);
        let red_hsl = srgb_to_hsl(red);
        assert!((red_hsl.hue.into_inner() - 0.0).abs() < 1e-6);
        assert!((red_hsl.saturation - 1.0).abs() < 1e-6);
        
        // Test green in HSV
        let green = Srgb::new(0.0, 1.0, 0.0);
        let green_hsv = srgb_to_hsv(green);
        assert!((green_hsv.hue.into_inner() - 120.0).abs() < 1e-6);
        assert!((green_hsv.saturation - 1.0).abs() < 1e-6);
    }
}
