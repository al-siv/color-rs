//! Color conversion strategies using Strategy pattern
//!
//! This module implements the Strategy pattern for different color space conversions
//! allowing dynamic selection of conversion algorithms.

use crate::error::Result;
use palette::{Hsl, Hsv, IntoColor, Lab, Lch, Srgb, Xyz};

/// Safely convert a clamped f32 color component to u8
/// Safely convert a clamped f32 color component to u8
#[inline]
fn f32_to_u8_clamped(value: f32) -> u8 {
    (value * 255.0).round().clamp(0.0, 255.0) as u8
}

/// Strategy interface for color space conversions
pub trait ColorConversionStrategy {
    /// Convert from LAB to target color space
    fn convert_from_lab(&self, lab: Lab) -> ConversionResult;

    /// Convert to LAB from source color space  
    fn to_lab(&self, color: &dyn ColorValue) -> Result<Lab>;

    /// Get the name of this conversion strategy
    fn strategy_name(&self) -> &'static str;
}

/// Unified color value that can represent different color spaces
pub trait ColorValue {
    fn as_lab(&self) -> Option<Lab>;
    fn as_srgb(&self) -> Option<Srgb>;
    fn as_hsl(&self) -> Option<Hsl>;
    fn as_hsv(&self) -> Option<Hsv>;
    fn as_hex(&self) -> Option<String>;
}

/// Result of color conversion containing multiple representations
#[derive(Debug, Clone)]
pub struct ConversionResult {
    pub lab: Lab,
    pub srgb: Srgb,
    pub hsl: Hsl,
    pub hsv: Hsv,
    pub lch: Lch,
    pub xyz: Xyz,
    pub hex: String,
    pub rgb_tuple: (u8, u8, u8),
    pub hsl_tuple: (f32, f32, f32),
    pub lch_tuple: (f32, f32, f32),
}

impl ConversionResult {
    /// Create conversion result from LAB color
    #[must_use] pub fn from_lab(lab: Lab) -> Self {
        let srgb: Srgb = lab.into_color();
        #[allow(clippy::similar_names)]
        let hsl: Hsl = srgb.into_color();
        #[allow(clippy::similar_names)]
        let hsv: Hsv = srgb.into_color();
        let lch: Lch = lab.into_color();
        let xyz: Xyz = lab.into_color();

        let hex = format!(
            "#{:02X}{:02X}{:02X}",
            f32_to_u8_clamped(srgb.red),
            f32_to_u8_clamped(srgb.green),
            f32_to_u8_clamped(srgb.blue)
        );

        let rgb_tuple = (
            f32_to_u8_clamped(srgb.red),
            f32_to_u8_clamped(srgb.green),
            f32_to_u8_clamped(srgb.blue),
        );

        let hsl_tuple = (hsl.hue.into_inner(), hsl.saturation, hsl.lightness);
        let lch_tuple = (lch.l, lch.chroma, lch.hue.into_inner());

        Self {
            lab,
            srgb,
            hsl,
            hsv,
            lch,
            xyz,
            hex,
            rgb_tuple,
            hsl_tuple,
            lch_tuple,
        }
    }
}

/// Standard RGB conversion strategy
pub struct RgbConversionStrategy;

impl ColorConversionStrategy for RgbConversionStrategy {
    fn convert_from_lab(&self, lab: Lab) -> ConversionResult {
        ConversionResult::from_lab(lab)
    }

    fn to_lab(&self, color: &dyn ColorValue) -> Result<Lab> {
        if let Some(lab) = color.as_lab() {
            Ok(lab)
        } else if let Some(srgb) = color.as_srgb() {
            Ok(srgb.into_color())
        } else if let Some(hsl) = color.as_hsl() {
            let srgb: Srgb = hsl.into_color();
            Ok(srgb.into_color())
        } else {
            Err(crate::error::ColorError::ParseError(
                "Unsupported color format for conversion".to_string(),
            ))
        }
    }

    fn strategy_name(&self) -> &'static str {
        "RGB"
    }
}

/// High precision LAB conversion strategy
pub struct LabConversionStrategy;

impl ColorConversionStrategy for LabConversionStrategy {
    fn convert_from_lab(&self, lab: Lab) -> ConversionResult {
        ConversionResult::from_lab(lab)
    }

    fn to_lab(&self, color: &dyn ColorValue) -> Result<Lab> {
        if let Some(lab) = color.as_lab() {
            Ok(lab)
        } else if let Some(srgb) = color.as_srgb() {
            // Use high-precision conversion through XYZ
            let xyz: Xyz = srgb.into_color();
            Ok(xyz.into_color())
        } else {
            Err(crate::error::ColorError::ParseError(
                "Unsupported color format for LAB conversion".to_string(),
            ))
        }
    }

    fn strategy_name(&self) -> &'static str {
        "LAB"
    }
}

/// Fast conversion strategy optimized for performance
pub struct FastConversionStrategy;

impl ColorConversionStrategy for FastConversionStrategy {
    fn convert_from_lab(&self, lab: Lab) -> ConversionResult {
        ConversionResult::from_lab(lab)
    }

    fn to_lab(&self, color: &dyn ColorValue) -> Result<Lab> {
        if let Some(lab) = color.as_lab() {
            Ok(lab)
        } else if let Some(srgb) = color.as_srgb() {
            // Direct conversion for speed
            Ok(srgb.into_color())
        } else {
            Err(crate::error::ColorError::ParseError(
                "Unsupported color format for fast conversion".to_string(),
            ))
        }
    }

    fn strategy_name(&self) -> &'static str {
        "Fast"
    }
}

/// Factory for creating conversion strategies
pub struct ConversionStrategyFactory;

impl ConversionStrategyFactory {
    /// Create strategy by name
    #[must_use] pub fn create_strategy(strategy_name: &str) -> Box<dyn ColorConversionStrategy> {
        match strategy_name.to_lowercase().as_str() {
            "rgb" => Box::new(RgbConversionStrategy),
            "lab" => Box::new(LabConversionStrategy),
            "fast" => Box::new(FastConversionStrategy),
            _ => Box::new(RgbConversionStrategy), // Default
        }
    }

    /// Get list of available strategies
    #[must_use]
    pub fn available_strategies() -> Vec<&'static str> {
        vec!["RGB", "LAB", "Fast"]
    }

    /// Create strategy optimized for specific use case
    #[must_use] pub fn create_for_use_case(use_case: ConversionUseCase) -> Box<dyn ColorConversionStrategy> {
        match use_case {
            ConversionUseCase::HighPrecision => Box::new(LabConversionStrategy),
            ConversionUseCase::WebDisplay => Box::new(RgbConversionStrategy),
            ConversionUseCase::Performance => Box::new(FastConversionStrategy),
        }
    }
}

/// Use cases for color conversion optimization
#[derive(Debug, Clone, Copy)]
pub enum ConversionUseCase {
    HighPrecision,
    WebDisplay,
    Performance,
}

/// Concrete color value implementations
#[derive(Debug, Clone)]
pub struct LabColor(pub Lab);

impl ColorValue for LabColor {
    fn as_lab(&self) -> Option<Lab> {
        Some(self.0)
    }
    fn as_srgb(&self) -> Option<Srgb> {
        Some(self.0.into_color())
    }
    fn as_hsl(&self) -> Option<Hsl> {
        let srgb: Srgb = self.0.into_color();
        Some(srgb.into_color())
    }
    fn as_hsv(&self) -> Option<Hsv> {
        let srgb: Srgb = self.0.into_color();
        Some(srgb.into_color())
    }
    fn as_hex(&self) -> Option<String> {
        let srgb: Srgb = self.0.into_color();
        Some(format!(
            "#{:02X}{:02X}{:02X}",
            (srgb.red * 255.0) as u8,
            (srgb.green * 255.0) as u8,
            (srgb.blue * 255.0) as u8
        ))
    }
}

#[derive(Debug, Clone)]
pub struct RgbColor(pub Srgb);

impl ColorValue for RgbColor {
    fn as_lab(&self) -> Option<Lab> {
        Some(self.0.into_color())
    }
    fn as_srgb(&self) -> Option<Srgb> {
        Some(self.0)
    }
    fn as_hsl(&self) -> Option<Hsl> {
        Some(self.0.into_color())
    }
    fn as_hsv(&self) -> Option<Hsv> {
        Some(self.0.into_color())
    }
    fn as_hex(&self) -> Option<String> {
        Some(format!(
            "#{:02X}{:02X}{:02X}",
            (self.0.red * 255.0) as u8,
            (self.0.green * 255.0) as u8,
            (self.0.blue * 255.0) as u8
        ))
    }
}

#[derive(Debug, Clone)]
pub struct HslColor(pub Hsl);

impl ColorValue for HslColor {
    fn as_lab(&self) -> Option<Lab> {
        let srgb: Srgb = self.0.into_color();
        Some(srgb.into_color())
    }
    fn as_srgb(&self) -> Option<Srgb> {
        Some(self.0.into_color())
    }
    fn as_hsl(&self) -> Option<Hsl> {
        Some(self.0)
    }
    fn as_hsv(&self) -> Option<Hsv> {
        let srgb: Srgb = self.0.into_color();
        Some(srgb.into_color())
    }
    fn as_hex(&self) -> Option<String> {
        let srgb: Srgb = self.0.into_color();
        Some(format!(
            "#{:02X}{:02X}{:02X}",
            (srgb.red * 255.0) as u8,
            (srgb.green * 255.0) as u8,
            (srgb.blue * 255.0) as u8
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_strategies() {
        let lab = Lab::new(50.0, 20.0, -30.0);
        let rgb_strategy = RgbConversionStrategy;
        let lab_strategy = LabConversionStrategy;

        let rgb_result = rgb_strategy.convert_from_lab(lab);
        let lab_result = lab_strategy.convert_from_lab(lab);

        assert_eq!(rgb_strategy.strategy_name(), "RGB");
        assert_eq!(lab_strategy.strategy_name(), "LAB");
        assert!(!rgb_result.hex.is_empty());
        assert!(!lab_result.hex.is_empty());
    }

    #[test]
    fn test_conversion_factory() {
        let strategies = ConversionStrategyFactory::available_strategies();
        assert!(strategies.contains(&"RGB"));
        assert!(strategies.contains(&"LAB"));

        let strategy = ConversionStrategyFactory::create_strategy("RGB");
        assert_eq!(strategy.strategy_name(), "RGB");
    }

    #[test]
    fn test_color_values() {
        let lab = Lab::new(50.0, 0.0, 0.0);
        let lab_color = LabColor(lab);

        assert!(lab_color.as_lab().is_some());
        assert!(lab_color.as_srgb().is_some());
        assert!(lab_color.as_hex().is_some());
    }
}
