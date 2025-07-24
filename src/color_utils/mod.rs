//! Modular color utilities with design patterns
//!
//! This module refactors the original color_utils.rs into a modular structure using
//! several Gang of Four design patterns:
//! - Strategy Pattern: For color conversion and interpolation algorithms
//! - Template Method Pattern: For consistent algorithm structure
//! - Factory Pattern: For creating algorithm instances
//! - Facade Pattern: For simplified access to complex functionality

pub mod conversion_strategies;
pub mod contrast_calculator;
pub mod interpolation;

// Re-export main types for convenient access
pub use conversion_strategies::{
    ColorConversionStrategy, ConversionResult, ConversionStrategyFactory,
    ColorValue, LabColor, RgbColor, HslColor, ConversionUseCase
};
pub use contrast_calculator::{
    ContrastCalculationTemplate, ContrastAnalysis, ContrastLevel, ContrastAssessment,
    WcagContrastCalculator, LabContrastCalculator, DeltaEContrastCalculator,
    EnhancedContrastCalculator, ContrastCalculatorFactory
};
pub use interpolation::{
    ColorInterpolationTemplate, InterpolationResult, InterpolationFactory,
    InterpolationService, EasingFunction, LinearLabInterpolator, PerceptualInterpolator
};

use crate::error::{Result, ColorError};
use palette::{Lab, Srgb, Hsl, IntoColor};

/// Facade for simplified access to color utilities
pub struct ColorUtilsFacade;

impl ColorUtilsFacade {
    /// Convert colors using the optimal strategy for the use case
    pub fn convert_color_optimized(
        lab: Lab,
        use_case: ConversionUseCase,
    ) -> ConversionResult {
        let strategy = ConversionStrategyFactory::create_for_use_case(use_case);
        strategy.convert_from_lab(lab)
    }
    
    /// Get comprehensive contrast analysis between two colors
    pub fn analyze_contrast_comprehensive(
        color1: Srgb,
        color2: Srgb,
    ) -> Result<ContrastAssessment> {
        let calculator = ContrastCalculatorFactory::create_enhanced_calculator();
        calculator.get_comprehensive_assessment(color1, color2)
    }
    
    /// Interpolate between colors with the best perceptual algorithm
    pub fn interpolate_perceptual(
        start: Lab,
        end: Lab,
        t: f64,
    ) -> Result<InterpolationResult> {
        let interpolator = InterpolationFactory::create_perceptual();
        interpolator.interpolate(start, end, t)
    }
    
    /// Create a smooth gradient with easing
    pub fn create_smooth_gradient(
        start: Lab,
        end: Lab,
        steps: usize,
        easing: EasingFunction,
    ) -> Result<Vec<Lab>> {
        let interpolator = InterpolationFactory::create_smooth("perceptual", easing);
        interpolator.interpolate_steps(start, end, steps)
    }
    
    /// Quick color conversion for common use cases
    pub fn quick_convert(color: Lab) -> QuickConversionResult {
        let result = ConversionStrategyFactory::create_strategy("fast").convert_from_lab(color);
        QuickConversionResult {
            hex: result.hex,
            rgb: result.rgb_tuple,
            hsl: result.hsl_tuple,
            luminance: Self::calculate_wcag_luminance(result.srgb),
        }
    }
    
    /// Calculate WCAG luminance efficiently
    #[must_use]
    pub fn calculate_wcag_luminance(color: Srgb) -> f64 {
        let calculator = ContrastCalculatorFactory::create_wcag_calculator();
        calculator.calculate_luminance(color)
    }
    
    /// Check if two colors meet WCAG contrast requirements
    pub fn meets_wcag_contrast(
        color1: Srgb,
        color2: Srgb,
        level: WcagLevel,
    ) -> Result<bool> {
        let calculator = ContrastCalculatorFactory::create_wcag_calculator();
        let analysis = calculator.calculate_contrast(color1, color2)?;
        
        Ok(match level {
            WcagLevel::AA => analysis.meets_wcag_aa,
            WcagLevel::AAA => analysis.meets_wcag_aaa,
        })
    }
    
    /// Get color distance using perceptual algorithm
    #[must_use]
    pub fn calculate_perceptual_distance(color1: Lab, color2: Lab) -> f64 {
        // Simple Euclidean distance in LAB space as fallback
        let dl = color1.l - color2.l;
        let da = color1.a - color2.a;
        let db = color1.b - color2.b;
        (dl * dl + da * da + db * db).sqrt() as f64
    }
}

/// Simplified conversion result for quick operations
#[derive(Debug, Clone)]
pub struct QuickConversionResult {
    pub hex: String,
    pub rgb: (u8, u8, u8),
    pub hsl: (f32, f32, f32),
    pub luminance: f64,
}

/// WCAG conformance levels
#[derive(Debug, Clone, Copy)]
pub enum WcagLevel {
    AA,
    AAA,
}

/// Builder for creating complex color operations
pub struct ColorOperationBuilder {
    conversion_strategy: Option<String>,
    contrast_algorithm: Option<String>,
    interpolation_method: Option<String>,
    easing_function: Option<EasingFunction>,
}

impl ColorOperationBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            conversion_strategy: None,
            contrast_algorithm: None,
            interpolation_method: None,
            easing_function: None,
        }
    }
    
    pub fn with_conversion_strategy(mut self, strategy: &str) -> Self {
        self.conversion_strategy = Some(strategy.to_string());
        self
    }
    
    pub fn with_contrast_algorithm(mut self, algorithm: &str) -> Self {
        self.contrast_algorithm = Some(algorithm.to_string());
        self
    }
    
    pub fn with_interpolation_method(mut self, method: &str) -> Self {
        self.interpolation_method = Some(method.to_string());
        self
    }
    
    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing_function = Some(easing);
        self
    }
    
    pub fn build(self) -> ColorOperationService {
        ColorOperationService {
            conversion_strategy: ConversionStrategyFactory::create_strategy(
                &self.conversion_strategy.unwrap_or_else(|| "rgb".to_string())
            ),
            contrast_calculator: ContrastCalculatorFactory::create_by_name(
                &self.contrast_algorithm.unwrap_or_else(|| "wcag".to_string())
            ),
            interpolator: if let Some(easing) = self.easing_function {
                InterpolationFactory::create_smooth(
                    &self.interpolation_method.unwrap_or_else(|| "perceptual".to_string()),
                    easing
                )
            } else {
                InterpolationFactory::create_by_name(
                    &self.interpolation_method.unwrap_or_else(|| "perceptual".to_string())
                )
            },
        }
    }
}

impl Default for ColorOperationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Service providing configured color operations
pub struct ColorOperationService {
    conversion_strategy: Box<dyn ColorConversionStrategy>,
    contrast_calculator: Box<dyn ContrastCalculationTemplate>,
    interpolator: Box<dyn ColorInterpolationTemplate>,
}

impl ColorOperationService {
    /// Convert color using configured strategy
    pub fn convert(&self, lab: Lab) -> ConversionResult {
        self.conversion_strategy.convert_from_lab(lab)
    }
    
    /// Calculate contrast using configured algorithm
    pub fn calculate_contrast(&self, color1: Srgb, color2: Srgb) -> Result<ContrastAnalysis> {
        self.contrast_calculator.calculate_contrast(color1, color2)
    }
    
    /// Interpolate colors using configured method
    pub fn interpolate(&self, start: Lab, end: Lab, t: f64) -> Result<InterpolationResult> {
        self.interpolator.interpolate(start, end, t)
    }
    
    /// Create gradient using all configured algorithms
    pub fn create_gradient(&self, start: Lab, end: Lab, steps: usize) -> Result<Vec<Lab>> {
        self.interpolator.interpolate_steps(start, end, steps)
    }
}

/// Compatibility layer for legacy color_utils.rs interface
pub struct LegacyColorUtils;

impl LegacyColorUtils {
    /// Legacy method: Get contrast assessment
    #[must_use]
    pub fn get_contrast_assessment(rgb1: (u8, u8, u8), rgb2: (u8, u8, u8)) -> (f32, String) {
        let srgb1 = Srgb::new(rgb1.0 as f32 / 255.0, rgb1.1 as f32 / 255.0, rgb1.2 as f32 / 255.0);
        let srgb2 = Srgb::new(rgb2.0 as f32 / 255.0, rgb2.1 as f32 / 255.0, rgb2.2 as f32 / 255.0);
        
        if let Ok(analysis) = ColorUtilsFacade::analyze_contrast_comprehensive(srgb1, srgb2) {
            (analysis.primary_analysis.contrast_ratio as f32, analysis.overall_assessment.to_string().to_string())
        } else {
            (1.0, "Low".to_string())
        }
    }
    
    /// Legacy method: LAB distance
    #[must_use]
    pub fn lab_distance(lab1: Lab, lab2: Lab) -> f64 {
        ColorUtilsFacade::calculate_perceptual_distance(lab1, lab2)
    }
    
    /// Legacy method: Interpolate LAB
    pub fn interpolate_lab(start: Lab, end: Lab, t: f64) -> Lab {
        ColorUtilsFacade::interpolate_perceptual(start, end, t)
            .map(|result| result.interpolated_color)
            .unwrap_or(start)
    }
    
    /// Legacy method: WCAG relative luminance
    #[must_use]
    pub fn wcag_relative_luminance(srgb: Srgb) -> f64 {
        ColorUtilsFacade::calculate_wcag_luminance(srgb)
    }
    
    /// Legacy method: WCAG relative luminance from RGB
    #[must_use]
    pub fn wcag_relative_luminance_rgb(rgb: (u8, u8, u8)) -> f64 {
        let srgb = Srgb::new(rgb.0 as f32 / 255.0, rgb.1 as f32 / 255.0, rgb.2 as f32 / 255.0);
        Self::wcag_relative_luminance(srgb)
    }
    
    /// Legacy method: LAB to hex
    pub fn lab_to_hex(lab: Lab) -> String {
        ColorUtilsFacade::quick_convert(lab).hex
    }
    
    /// Legacy method: LAB to RGB
    pub fn lab_to_rgb(lab: Lab) -> (u8, u8, u8) {
        ColorUtilsFacade::quick_convert(lab).rgb
    }
    
    /// Legacy method: LAB to sRGB
    pub fn lab_to_srgb(lab: Lab) -> Srgb {
        lab.into_color()
    }
    
    /// Legacy method: sRGB to LAB
    pub fn srgb_to_lab(srgb: Srgb) -> Lab {
        srgb.into_color()
    }
    
    /// Legacy method: RGB to LAB
    pub fn rgb_to_lab(rgb: (u8, u8, u8)) -> Lab {
        let srgb = Srgb::new(rgb.0 as f32 / 255.0, rgb.1 as f32 / 255.0, rgb.2 as f32 / 255.0);
        srgb.into_color()
    }
    
    /// Legacy method: RGB to sRGB
    pub fn rgb_to_srgb(rgb: (u8, u8, u8)) -> Srgb {
        Srgb::new(rgb.0 as f32 / 255.0, rgb.1 as f32 / 255.0, rgb.2 as f32 / 255.0)
    }
    
    /// Legacy method: sRGB to RGB
    pub fn srgb_to_rgb(srgb: Srgb) -> (u8, u8, u8) {
        ((srgb.red * 255.0) as u8, (srgb.green * 255.0) as u8, (srgb.blue * 255.0) as u8)
    }
    
    /// Array-based functions for collections compatibility
    pub fn rgb_array_to_lab(rgb: [u8; 3]) -> [f32; 3] {
        let lab = Self::rgb_to_lab((rgb[0], rgb[1], rgb[2]));
        [lab.l, lab.a, lab.b]
    }

    pub fn lab_array_to_rgb(lab: [f32; 3]) -> [u8; 3] {
        let lab_color = Lab::new(lab[0], lab[1], lab[2]);
        let (r, g, b) = Self::lab_to_rgb(lab_color);
        [r, g, b]
    }

    pub fn lab_array_distance(lab1: [f32; 3], lab2: [f32; 3]) -> f64 {
        let lab1_color = Lab::new(lab1[0], lab1[1], lab1[2]);
        let lab2_color = Lab::new(lab2[0], lab2[1], lab2[2]);
        Self::lab_distance(lab1_color, lab2_color)
    }
    
    /// Additional legacy functions for backward compatibility
    pub fn hex_to_lab(hex: &str) -> Result<Lab> {
        Self::parse_hex_color(hex)
    }
    
    /// Legacy method: LAB to HSL tuple
    pub fn lab_to_hsl_tuple(lab: Lab) -> (f64, f64, f64) {
        let srgb: Srgb = lab.into_color();
        let hsl: Hsl = srgb.into_color();
        (hsl.hue.into_degrees() as f64, hsl.saturation as f64, hsl.lightness as f64)
    }
    
    /// Legacy method: sRGB to HSL tuple
    pub fn srgb_to_hsl_tuple(srgb: Srgb) -> (f64, f64, f64) {
        let hsl: Hsl = srgb.into_color();
        (hsl.hue.into_degrees() as f64, hsl.saturation as f64, hsl.lightness as f64)
    }
    
    /// Legacy method: LAB to LCH
    pub fn lab_to_lch(lab: Lab) -> palette::Lch {
        lab.into_color()
    }
    
    /// Legacy method: LCH to LAB
    pub fn lch_to_lab(lch: palette::Lch) -> Lab {
        lch.into_color()
    }
    
    /// Legacy method: LCH tulip format to LAB
    pub fn lch_tulip_to_lab(lch: (f64, f64, f64)) -> Lab {
        let lch_color = palette::Lch::new(lch.0 as f32, lch.1 as f32, lch.2 as f32);
        lch_color.into_color()
    }
    
    /// Legacy method: WCAG contrast ratio
    #[must_use]
    pub fn wcag_contrast_ratio(color1: Srgb, color2: Srgb) -> f64 {
        let l1 = Self::wcag_relative_luminance(color1);
        let l2 = Self::wcag_relative_luminance(color2);
        let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
        (lighter + 0.05) / (darker + 0.05)
    }
    
    /// Legacy method: Parse hex color to LAB
    pub fn parse_hex_color(hex: &str) -> Result<Lab> {
        let hex_clean = hex.trim_start_matches('#');
        if hex_clean.len() != 6 {
            return Err(ColorError::InvalidArguments("Invalid hex color format".to_string()));
        }
        
        let r = u8::from_str_radix(&hex_clean[0..2], 16)
            .map_err(|_| ColorError::InvalidArguments("Invalid hex color".to_string()))?;
        let g = u8::from_str_radix(&hex_clean[2..4], 16)
            .map_err(|_| ColorError::InvalidArguments("Invalid hex color".to_string()))?;
        let b = u8::from_str_radix(&hex_clean[4..6], 16)
            .map_err(|_| ColorError::InvalidArguments("Invalid hex color".to_string()))?;
        
        Ok(Self::rgb_to_lab((r, g, b)))
    }
    
    // Additional missing functions for format_utils
    pub fn lab_to_hsv_tuple(lab: Lab) -> (f64, f64, f64) {
        let srgb: Srgb = lab.into_color();
        let hsv: palette::Hsv = srgb.into_color();
        (hsv.hue.into_degrees() as f64, hsv.saturation as f64, hsv.value as f64)
    }
    
    pub fn lab_to_cmyk_tuple(lab: Lab) -> (f64, f64, f64, f64) {
        let (r, g, b) = Self::lab_to_rgb(lab);
        let (rf, gf, bf) = (r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0);
        let k = 1.0 - rf.max(gf).max(bf);
        if k == 1.0 {
            (0.0, 0.0, 0.0, 1.0)
        } else {
            let c = (1.0 - rf - k) / (1.0 - k);
            let m = (1.0 - gf - k) / (1.0 - k);
            let y = (1.0 - bf - k) / (1.0 - k);
            (c, m, y, k)
        }
    }
    
    pub fn lab_to_xyz_tuple(lab: Lab) -> (f64, f64, f64) {
        let xyz: palette::Xyz = lab.into_color();
        (xyz.x as f64, xyz.y as f64, xyz.z as f64)
    }
    
    pub fn lab_to_lch_tuple(lab: Lab) -> (f64, f64, f64) {
        let lch = Self::lab_to_lch(lab);
        (lch.l as f64, lch.chroma as f64, lch.hue.into_degrees() as f64)
    }
    
    pub fn lab_to_oklch_tuple(lab: Lab) -> (f64, f64, f64) {
        // Convert LAB -> sRGB -> Oklab -> Oklch
        let srgb: Srgb = lab.into_color();
        let oklab: palette::Oklab = srgb.into_color();
        let oklch: palette::Oklch = oklab.into_color();
        (oklch.l as f64, oklch.chroma as f64, oklch.hue.into_degrees() as f64)
    }
    
    // Color scheme functions for backward compatibility
    #[must_use]
    pub fn complementary_hsl(color: Lab) -> Vec<Lab> {
        let srgb: Srgb = color.into_color();
        let hsl: palette::Hsl = srgb.into_color();
        let comp_hue = (hsl.hue.into_degrees() + 180.0) % 360.0;
        let comp_hsl = palette::Hsl::new(comp_hue, hsl.saturation, hsl.lightness);
        let comp_srgb: Srgb = comp_hsl.into_color();
        vec![color, comp_srgb.into_color()]
    }
    
    #[must_use]
    pub fn split_complementary_hsl(color: Lab) -> Vec<Lab> {
        let srgb: Srgb = color.into_color();
        let hsl: palette::Hsl = srgb.into_color();
        let hue = hsl.hue.into_degrees();
        let hsl1 = palette::Hsl::new((hue + 150.0) % 360.0, hsl.saturation, hsl.lightness);
        let hsl2 = palette::Hsl::new((hue + 210.0) % 360.0, hsl.saturation, hsl.lightness);
        vec![
            color,
            palette::IntoColor::<Srgb>::into_color(hsl1).into_color(),
            palette::IntoColor::<Srgb>::into_color(hsl2).into_color(),
        ]
    }
    
    #[must_use]
    pub fn triadic_hsl(color: Lab) -> Vec<Lab> {
        let srgb: Srgb = color.into_color();
        let hsl: palette::Hsl = srgb.into_color();
        let hue = hsl.hue.into_degrees();
        let hsl1 = palette::Hsl::new((hue + 120.0) % 360.0, hsl.saturation, hsl.lightness);
        let hsl2 = palette::Hsl::new((hue + 240.0) % 360.0, hsl.saturation, hsl.lightness);
        vec![
            color,
            palette::IntoColor::<Srgb>::into_color(hsl1).into_color(),
            palette::IntoColor::<Srgb>::into_color(hsl2).into_color(),
        ]
    }
    
    #[must_use]
    pub fn tetradic_hsl(color: Lab) -> Vec<Lab> {
        let srgb: Srgb = color.into_color();
        let hsl: palette::Hsl = srgb.into_color();
        let hue = hsl.hue.into_degrees();
        let hsl1 = palette::Hsl::new((hue + 90.0) % 360.0, hsl.saturation, hsl.lightness);
        let hsl2 = palette::Hsl::new((hue + 180.0) % 360.0, hsl.saturation, hsl.lightness);
        let hsl3 = palette::Hsl::new((hue + 270.0) % 360.0, hsl.saturation, hsl.lightness);
        vec![
            color,
            palette::IntoColor::<Srgb>::into_color(hsl1).into_color(),
            palette::IntoColor::<Srgb>::into_color(hsl2).into_color(),
            palette::IntoColor::<Srgb>::into_color(hsl3).into_color(),
        ]
    }
    
    #[must_use]
    pub fn complementary_lab(color: Lab) -> Vec<Lab> {
        Self::complementary_hsl(color) // Same logic, different entry point
    }
    
    #[must_use]
    pub fn split_complementary_lab(color: Lab) -> Vec<Lab> {
        Self::split_complementary_hsl(color)
    }
    
    #[must_use]
    pub fn triadic_lab(color: Lab) -> Vec<Lab> {
        Self::triadic_hsl(color)
    }
    
    #[must_use]
    pub fn tetradic_lab(color: Lab) -> Vec<Lab> {
        Self::tetradic_hsl(color)
    }
    
    pub fn adjust_color_relative_luminance(color: Lab, target_luminance: f64) -> Result<Lab> {
        // Simple luminance adjustment by scaling lightness
        let current_srgb: Srgb = color.into_color();
        let current_luminance = Self::wcag_relative_luminance(current_srgb);
        if current_luminance == 0.0 {
            return Err(ColorError::InvalidArguments("Cannot adjust luminance of black color".to_string()));
        }
        
        let scale_factor = (target_luminance / current_luminance).sqrt();
        let adjusted_l = (color.l * scale_factor as f32).clamp(0.0, 100.0);
        Ok(Lab::new(adjusted_l, color.a, color.b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::{Lab, Srgb};

    #[test]
    fn test_color_utils_facade() {
        let lab = Lab::new(50.0, 20.0, -30.0);
        
        let result = ColorUtilsFacade::convert_color_optimized(lab, ConversionUseCase::WebDisplay);
        assert!(!result.hex.is_empty());
        
        let quick_result = ColorUtilsFacade::quick_convert(lab);
        assert!(!quick_result.hex.is_empty());
        assert!(quick_result.luminance >= 0.0);
    }
    
    #[test]
    fn test_contrast_analysis() {
        let white = Srgb::new(1.0, 1.0, 1.0);
        let black = Srgb::new(0.0, 0.0, 0.0);
        
        let analysis = ColorUtilsFacade::analyze_contrast_comprehensive(white, black).unwrap();
        assert_eq!(analysis.overall_assessment, ContrastLevel::High);
        assert!(analysis.primary_analysis.meets_wcag_aaa);
        
        let meets_aa = ColorUtilsFacade::meets_wcag_contrast(white, black, WcagLevel::AA).unwrap();
        assert!(meets_aa);
    }
    
    #[test]
    fn test_interpolation() {
        let start = Lab::new(0.0, 0.0, 0.0);
        let end = Lab::new(100.0, 0.0, 0.0);
        
        let result = ColorUtilsFacade::interpolate_perceptual(start, end, 0.5).unwrap();
        assert!((result.interpolated_color.l - 50.0).abs() < 10.0);
        
        let gradient = ColorUtilsFacade::create_smooth_gradient(
            start, end, 5, EasingFunction::Smooth
        ).unwrap();
        assert_eq!(gradient.len(), 5);
    }
    
    #[test]
    fn test_color_operation_builder() {
        let service = ColorOperationBuilder::new()
            .with_conversion_strategy("lab")
            .with_contrast_algorithm("wcag")
            .with_interpolation_method("perceptual")
            .with_easing(EasingFunction::Smooth)
            .build();
        
        let lab = Lab::new(50.0, 0.0, 0.0);
        let result = service.convert(lab);
        assert!(!result.hex.is_empty());
    }
    
    #[test]
    fn test_legacy_compatibility() {
        let lab1 = Lab::new(0.0, 0.0, 0.0);
        let lab2 = Lab::new(100.0, 0.0, 0.0);
        
        let distance = LegacyColorUtils::lab_distance(lab1, lab2);
        assert!(distance > 0.0);
        
        let interpolated = LegacyColorUtils::interpolate_lab(lab1, lab2, 0.5);
        assert!((interpolated.l - 50.0).abs() < 10.0);
        
        let hex = LegacyColorUtils::lab_to_hex(lab1);
        assert!(hex.starts_with('#'));
        
        let _rgb = LegacyColorUtils::lab_to_rgb(lab1);
        // RGB values are u8, so they're automatically in 0-255 range
    }
}
