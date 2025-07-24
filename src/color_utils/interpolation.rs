//! Color interpolation algorithms using Template Method pattern
//!
//! This module provides various color interpolation strategies with consistent interfaces
//! using the Template Method pattern for algorithm structure.

use crate::error::Result;
use palette::{Hsl, IntoColor, Lab, Mix, Srgb};

/// Result of color interpolation
#[derive(Debug, Clone)]
pub struct InterpolationResult {
    pub interpolated_color: Lab,
    pub intermediate_steps: Vec<Lab>,
    pub algorithm_used: &'static str,
    pub t_value: f64,
}

/// Template method interface for color interpolation algorithms
pub trait ColorInterpolationTemplate {
    /// Main template method for color interpolation
    fn interpolate(&self, start: Lab, end: Lab, t: f64) -> Result<InterpolationResult> {
        self.validate_parameters(t)?;
        let normalized_t = self.normalize_t_value(t);
        let interpolated = self.perform_interpolation(start, end, normalized_t);
        let steps = self.generate_intermediate_steps(start, end, normalized_t);
        let result = self.create_result(interpolated, steps, normalized_t);
        self.post_process_result(result)
    }

    /// Generate interpolation steps
    fn interpolate_steps(&self, start: Lab, end: Lab, num_steps: usize) -> Result<Vec<Lab>> {
        if num_steps == 0 {
            return Ok(Vec::new());
        }
        if num_steps == 1 {
            return Ok(vec![self.perform_interpolation(start, end, 0.5)]);
        }

        let mut steps = Vec::new();
        for i in 0..num_steps {
            let t = i as f64 / (num_steps - 1) as f64;
            steps.push(self.perform_interpolation(start, end, t));
        }
        Ok(steps)
    }

    /// Validate interpolation parameters (hook method)
    fn validate_parameters(&self, t: f64) -> Result<()> {
        if !(0.0..=1.0).contains(&t) {
            Err(crate::error::ColorError::InvalidArguments(format!(
                "Interpolation parameter t must be between 0.0 and 1.0, got {t}"
            )))
        } else {
            Ok(())
        }
    }

    /// Normalize t value (hook method)
    fn normalize_t_value(&self, t: f64) -> f64 {
        t.clamp(0.0, 1.0)
    }

    /// Perform the actual interpolation (abstract method)
    fn perform_interpolation(&self, start: Lab, end: Lab, t: f64) -> Lab;

    /// Generate intermediate steps (hook method)
    fn generate_intermediate_steps(&self, start: Lab, end: Lab, t: f64) -> Vec<Lab> {
        if t == 0.0 {
            vec![start]
        } else if t == 1.0 {
            vec![end]
        } else {
            vec![start, self.perform_interpolation(start, end, t), end]
        }
    }

    /// Create interpolation result (concrete method)
    fn create_result(&self, color: Lab, steps: Vec<Lab>, t: f64) -> InterpolationResult {
        InterpolationResult {
            interpolated_color: color,
            intermediate_steps: steps,
            algorithm_used: self.algorithm_name(),
            t_value: t,
        }
    }

    /// Post-process result (hook method)
    fn post_process_result(&self, result: InterpolationResult) -> Result<InterpolationResult> {
        Ok(result)
    }

    /// Get algorithm name (abstract method)
    fn algorithm_name(&self) -> &'static str;
}

/// Linear LAB interpolation
pub struct LinearLabInterpolator;

impl ColorInterpolationTemplate for LinearLabInterpolator {
    fn perform_interpolation(&self, start: Lab, end: Lab, t: f64) -> Lab {
        // Simple linear interpolation in LAB space
        Lab {
            l: start.l + t as f32 * (end.l - start.l),
            a: start.a + t as f32 * (end.a - start.a),
            b: start.b + t as f32 * (end.b - start.b),
            white_point: start.white_point,
        }
    }

    fn algorithm_name(&self) -> &'static str {
        "Linear LAB"
    }
}

/// Perceptually uniform interpolation using palette's Mix trait
pub struct PerceptualInterpolator;

impl ColorInterpolationTemplate for PerceptualInterpolator {
    fn perform_interpolation(&self, start: Lab, end: Lab, t: f64) -> Lab {
        start.mix(end, t as f32)
    }

    fn algorithm_name(&self) -> &'static str {
        "Perceptual Mix"
    }
}

/// RGB-based interpolation (converted through LAB)
pub struct RgbInterpolator;

impl ColorInterpolationTemplate for RgbInterpolator {
    fn perform_interpolation(&self, start: Lab, end: Lab, t: f64) -> Lab {
        let start_rgb: Srgb = start.into_color();
        let end_rgb: Srgb = end.into_color();

        let interpolated_rgb = Srgb::new(
            start_rgb.red + t as f32 * (end_rgb.red - start_rgb.red),
            start_rgb.green + t as f32 * (end_rgb.green - start_rgb.green),
            start_rgb.blue + t as f32 * (end_rgb.blue - start_rgb.blue),
        );

        interpolated_rgb.into_color()
    }

    fn algorithm_name(&self) -> &'static str {
        "RGB Linear"
    }
}

/// HSL-based interpolation with hue wrapping
pub struct HslInterpolator;

impl ColorInterpolationTemplate for HslInterpolator {
    fn perform_interpolation(&self, start: Lab, end: Lab, t: f64) -> Lab {
        let start_hsl: Hsl = start.into_color();
        let end_hsl: Hsl = end.into_color();

        // Handle hue interpolation with proper wrapping
        let start_hue = start_hsl.hue.into_inner();
        let end_hue = end_hsl.hue.into_inner();

        let hue_diff = if (end_hue - start_hue).abs() > 180.0 {
            if end_hue > start_hue {
                end_hue - start_hue - 360.0
            } else {
                end_hue - start_hue + 360.0
            }
        } else {
            end_hue - start_hue
        };

        let interpolated_hue = (start_hue + t as f32 * hue_diff + 360.0) % 360.0;

        let interpolated_hsl: Hsl<palette::encoding::Srgb> = Hsl::new(
            interpolated_hue,
            start_hsl.saturation + t as f32 * (end_hsl.saturation - start_hsl.saturation),
            start_hsl.lightness + t as f32 * (end_hsl.lightness - start_hsl.lightness),
        );

        let rgb: Srgb = interpolated_hsl.into_color();
        rgb.into_color()
    }

    fn algorithm_name(&self) -> &'static str {
        "HSL Circular"
    }
}

/// Smooth interpolation with easing functions
pub struct SmoothInterpolator {
    base_interpolator: Box<dyn ColorInterpolationTemplate>,
    easing_function: EasingFunction,
}

impl SmoothInterpolator {
    pub fn new(base: Box<dyn ColorInterpolationTemplate>, easing: EasingFunction) -> Self {
        Self {
            base_interpolator: base,
            easing_function: easing,
        }
    }
}

impl ColorInterpolationTemplate for SmoothInterpolator {
    fn normalize_t_value(&self, t: f64) -> f64 {
        let normalized = t.clamp(0.0, 1.0);
        self.easing_function.apply(normalized)
    }

    fn perform_interpolation(&self, start: Lab, end: Lab, t: f64) -> Lab {
        self.base_interpolator.perform_interpolation(start, end, t)
    }

    fn algorithm_name(&self) -> &'static str {
        "Smooth Eased"
    }
}

/// Easing functions for smooth interpolation
#[derive(Debug, Clone, Copy)]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Smooth,
}

impl EasingFunction {
    #[must_use]
    pub fn apply(&self, t: f64) -> f64 {
        match self {
            EasingFunction::Linear => t,
            EasingFunction::EaseIn => t * t,
            EasingFunction::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            EasingFunction::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - 2.0 * (1.0 - t) * (1.0 - t)
                }
            }
            EasingFunction::Smooth => {
                // Smoothstep function: 3t² - 2t³
                t * t * (3.0 - 2.0 * t)
            }
        }
    }
}

/// Factory for creating interpolation strategies
pub struct InterpolationFactory;

impl InterpolationFactory {
    pub fn create_linear_lab() -> Box<dyn ColorInterpolationTemplate> {
        Box::new(LinearLabInterpolator)
    }

    pub fn create_perceptual() -> Box<dyn ColorInterpolationTemplate> {
        Box::new(PerceptualInterpolator)
    }

    pub fn create_rgb() -> Box<dyn ColorInterpolationTemplate> {
        Box::new(RgbInterpolator)
    }

    pub fn create_hsl() -> Box<dyn ColorInterpolationTemplate> {
        Box::new(HslInterpolator)
    }

    pub fn create_smooth(
        base_type: &str,
        easing: EasingFunction,
    ) -> Box<dyn ColorInterpolationTemplate> {
        let base = match base_type.to_lowercase().as_str() {
            "lab" => Self::create_linear_lab(),
            "perceptual" => Self::create_perceptual(),
            "rgb" => Self::create_rgb(),
            "hsl" => Self::create_hsl(),
            _ => Self::create_perceptual(), // Default
        };

        Box::new(SmoothInterpolator::new(base, easing))
    }

    pub fn create_by_name(name: &str) -> Box<dyn ColorInterpolationTemplate> {
        match name.to_lowercase().as_str() {
            "linear" | "lab" => Self::create_linear_lab(),
            "perceptual" => Self::create_perceptual(),
            "rgb" => Self::create_rgb(),
            "hsl" => Self::create_hsl(),
            _ => Self::create_perceptual(), // Default
        }
    }

    #[must_use]
    pub fn available_algorithms() -> Vec<&'static str> {
        vec!["Linear LAB", "Perceptual", "RGB", "HSL", "Smooth"]
    }
}

/// Advanced interpolation service with multiple algorithms
pub struct InterpolationService {
    primary_algorithm: Box<dyn ColorInterpolationTemplate>,
    fallback_algorithm: Option<Box<dyn ColorInterpolationTemplate>>,
}

impl InterpolationService {
    pub fn new(primary: Box<dyn ColorInterpolationTemplate>) -> Self {
        Self {
            primary_algorithm: primary,
            fallback_algorithm: None,
        }
    }

    pub fn with_fallback(mut self, fallback: Box<dyn ColorInterpolationTemplate>) -> Self {
        self.fallback_algorithm = Some(fallback);
        self
    }

    pub fn interpolate_with_fallback(
        &self,
        start: Lab,
        end: Lab,
        t: f64,
    ) -> Result<InterpolationResult> {
        match self.primary_algorithm.interpolate(start, end, t) {
            Ok(result) => Ok(result),
            Err(_) => {
                if let Some(ref fallback) = self.fallback_algorithm {
                    fallback.interpolate(start, end, t)
                } else {
                    Err(crate::error::ColorError::InvalidOperation(
                        "Interpolation failed and no fallback available".to_string(),
                    ))
                }
            }
        }
    }

    /// Create gradient with specified number of steps
    pub fn create_gradient(&self, start: Lab, end: Lab, steps: usize) -> Result<Vec<Lab>> {
        self.primary_algorithm.interpolate_steps(start, end, steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Lab;

    #[test]
    fn test_linear_lab_interpolation() {
        let interpolator = LinearLabInterpolator;
        let start = Lab::new(0.0, 0.0, 0.0);
        let end = Lab::new(100.0, 0.0, 0.0);

        let result = interpolator.interpolate(start, end, 0.5).unwrap();
        assert!((result.interpolated_color.l - 50.0).abs() < 0.1);
        assert_eq!(result.algorithm_used, "Linear LAB");
    }

    #[test]
    fn test_perceptual_interpolation() {
        let interpolator = PerceptualInterpolator;
        let start = Lab::new(0.0, 0.0, 0.0);
        let end = Lab::new(100.0, 0.0, 0.0);

        let result = interpolator.interpolate(start, end, 0.5).unwrap();
        assert_eq!(result.algorithm_used, "Perceptual Mix");
    }

    #[test]
    fn test_interpolation_steps() {
        let interpolator = LinearLabInterpolator;
        let start = Lab::new(0.0, 0.0, 0.0);
        let end = Lab::new(100.0, 0.0, 0.0);

        let steps = interpolator.interpolate_steps(start, end, 5).unwrap();
        assert_eq!(steps.len(), 5);
        assert!((steps[0].l - 0.0).abs() < 0.1);
        assert!((steps[4].l - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_easing_functions() {
        assert_eq!(EasingFunction::Linear.apply(0.5), 0.5);
        assert!(EasingFunction::EaseIn.apply(0.5) < 0.5);
        assert!(EasingFunction::EaseOut.apply(0.5) > 0.5);
    }

    #[test]
    fn test_interpolation_factory() {
        let algorithms = InterpolationFactory::available_algorithms();
        assert!(algorithms.contains(&"Linear LAB"));
        assert!(algorithms.contains(&"Perceptual"));

        let interpolator = InterpolationFactory::create_by_name("lab");
        assert_eq!(interpolator.algorithm_name(), "Linear LAB");
    }

    #[test]
    fn test_interpolation_service() {
        let primary = InterpolationFactory::create_perceptual();
        let fallback = InterpolationFactory::create_linear_lab();
        let service = InterpolationService::new(primary).with_fallback(fallback);

        let start = Lab::new(0.0, 0.0, 0.0);
        let end = Lab::new(100.0, 0.0, 0.0);

        let result = service.interpolate_with_fallback(start, end, 0.5).unwrap();
        assert!(!result.algorithm_used.is_empty());

        let gradient = service.create_gradient(start, end, 5).unwrap();
        assert_eq!(gradient.len(), 5);
    }
}
