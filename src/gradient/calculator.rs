//! Gradient calculation algorithms and stop generation
//!
//! This module implements the Template Method pattern for gradient calculation
//! and provides various algorithms for calculating gradient stops.

use super::easing::EasingStrategy;
use crate::color_utils::LegacyColorUtils as ColorUtils;
use crate::config::INTELLIGENT_STOP_SAMPLE_POINTS;
use crate::utils::Utils;
use kurbo::{CubicBez, ParamCurve, Point};
use palette::Lab;
use tabled::Tabled;

/// Gradient value for display in tables
#[derive(Tabled, Clone)]
pub struct GradientValue {
    #[tabled(rename = "Position")]
    pub position: String,
    #[tabled(rename = "Hex")]
    pub hex: String,
    #[tabled(rename = "RGB")]
    pub rgb: String,
    #[tabled(rename = "WCAG Luminance")]
    pub wcag_luminance: String,
}

/// Abstract template for gradient calculation algorithms
pub trait GradientCalculationTemplate {
    /// Calculate stop positions (Template Method pattern)
    fn calculate_stops(&self, num_stops: usize) -> Vec<f64> {
        self.validate_input(num_stops);
        let stops = self.generate_stops(num_stops);
        self.post_process_stops(stops)
    }

    /// Validate input parameters (hook method)
    fn validate_input(&self, num_stops: usize) {
        assert!(num_stops > 0, "Number of stops must be positive");
    }

    /// Generate the actual stop positions (abstract method)
    fn generate_stops(&self, num_stops: usize) -> Vec<f64>;

    /// Post-process stops if needed (hook method)
    fn post_process_stops(&self, stops: Vec<f64>) -> Vec<f64> {
        stops
    }
}

/// Intelligent stop calculation using easing functions
pub struct IntelligentStopCalculator {
    ease_in: f64,
    ease_out: f64,
}

impl IntelligentStopCalculator {
    pub fn new(ease_in: f64, ease_out: f64) -> Self {
        Self { ease_in, ease_out }
    }
}

impl GradientCalculationTemplate for IntelligentStopCalculator {
    fn generate_stops(&self, num_stops: usize) -> Vec<f64> {
        if num_stops <= 1 {
            return vec![0.0];
        }
        if num_stops == 2 {
            return vec![0.0, 1.0];
        }

        // Create cubic bezier curve for easing analysis
        let curve = CubicBez::new(
            Point::new(0.0, 0.0),
            Point::new(self.ease_in, 0.0),
            Point::new(self.ease_out, 1.0),
            Point::new(1.0, 1.0),
        );

        let mut cumulative_importance = vec![0.0; INTELLIGENT_STOP_SAMPLE_POINTS + 1];

        for i in 0..INTELLIGENT_STOP_SAMPLE_POINTS {
            let t = i as f64 / INTELLIGENT_STOP_SAMPLE_POINTS as f64;
            let dt = 1.0 / INTELLIGENT_STOP_SAMPLE_POINTS as f64;

            // Calculate derivative magnitude using numerical differentiation
            let current_point = curve.eval(t);
            let next_point = curve.eval((t + dt).min(1.0));

            let dy = next_point.y - current_point.y;
            let derivative_magnitude = dy.abs();

            cumulative_importance[i + 1] = cumulative_importance[i] + derivative_magnitude;
        }

        let total_importance = cumulative_importance[INTELLIGENT_STOP_SAMPLE_POINTS];
        if total_importance == 0.0 {
            // Fallback to equal spacing
            return (0..num_stops)
                .map(|i| i as f64 / (num_stops - 1).max(1) as f64)
                .collect();
        }

        // Distribute stops based on cumulative importance
        let mut stops = Vec::new();
        for i in 0..num_stops {
            let target_importance = (i as f64 / (num_stops - 1).max(1) as f64) * total_importance;

            // Binary search to find the t value
            let mut low = 0;
            let mut high = INTELLIGENT_STOP_SAMPLE_POINTS;

            while high - low > 1 {
                let mid = (low + high) / 2;
                if cumulative_importance[mid] < target_importance {
                    low = mid;
                } else {
                    high = mid;
                }
            }

            // Linear interpolation between the two closest points
            let t = if cumulative_importance[high] - cumulative_importance[low] > f64::EPSILON {
                let ratio = (target_importance - cumulative_importance[low])
                    / (cumulative_importance[high] - cumulative_importance[low]);
                (low as f64 + ratio) / INTELLIGENT_STOP_SAMPLE_POINTS as f64
            } else {
                low as f64 / INTELLIGENT_STOP_SAMPLE_POINTS as f64
            };

            stops.push(t.clamp(0.0, 1.0));
        }

        stops
    }
}

/// Equal spacing stop calculation
pub struct EqualSpacingCalculator;

impl GradientCalculationTemplate for EqualSpacingCalculator {
    fn generate_stops(&self, num_stops: usize) -> Vec<f64> {
        (0..num_stops)
            .map(|i| i as f64 / (num_stops - 1).max(1) as f64)
            .collect()
    }
}

/// Main gradient calculator using Strategy pattern
pub struct GradientCalculator {
    calculator: Box<dyn GradientCalculationTemplate>,
}

impl GradientCalculator {
    /// Create calculator with intelligent stop algorithm
    #[must_use]
    pub fn with_intelligent_stops(ease_in: f64, ease_out: f64) -> Self {
        Self {
            calculator: Box::new(IntelligentStopCalculator::new(ease_in, ease_out)),
        }
    }

    /// Create calculator with equal spacing
    #[must_use]
    pub fn with_equal_spacing() -> Self {
        Self {
            calculator: Box::new(EqualSpacingCalculator),
        }
    }

    /// Cubic bezier easing function for backwards compatibility
    #[must_use]
    pub fn cubic_bezier_ease(t: f64, ease_in: f64, ease_out: f64) -> f64 {
        // Simple cubic bezier approximation
        let t2 = t * t;
        let t3 = t2 * t;
        let inv_t = 1.0 - t;
        let inv_t2 = inv_t * inv_t;
        let inv_t3 = inv_t2 * inv_t;

        inv_t3 + 3.0 * inv_t2 * t * ease_in + 3.0 * inv_t * t2 * ease_out + t3
    }

    /// Calculate stop positions
    #[must_use]
    pub fn calculate_stops(&self, num_stops: usize) -> Vec<f64> {
        self.calculator.calculate_stops(num_stops)
    }

    /// Calculate integer stop positions (0-100 range)
    #[must_use]
    pub fn calculate_stops_integer(&self, num_stops: usize, start_pos: u8, end_pos: u8) -> Vec<u8> {
        let stops = self.calculate_stops(num_stops);
        let range = end_pos as f64 - start_pos as f64;

        stops
            .into_iter()
            .map(|stop| {
                let position = start_pos as f64 + stop * range;
                position.round().clamp(0.0, 255.0) as u8
            })
            .collect()
    }

    /// Generate gradient values using Template Method pattern
    pub fn generate_gradient_values(
        &self,
        start_lab: Lab,
        end_lab: Lab,
        num_stops: usize,
        start_position: u8,
        end_position: u8,
        easing_strategy: &dyn EasingStrategy,
    ) -> crate::error::Result<Vec<GradientValue>> {
        if num_stops == 0 {
            return Ok(Vec::new());
        }

        let stops = if num_stops == 1 {
            vec![0.5] // Single stop at midpoint
        } else {
            self.calculate_stops(num_stops)
        };

        let mut gradient_values = Vec::new();
        let position_range = end_position as f64 - start_position as f64;

        for &stop in &stops {
            // Apply easing function
            let eased_t = easing_strategy.ease(stop);

            // Interpolate color in LAB space
            let interpolated_lab = Lab {
                l: start_lab.l + (eased_t as f32) * (end_lab.l - start_lab.l),
                a: start_lab.a + (eased_t as f32) * (end_lab.a - start_lab.a),
                b: start_lab.b + (eased_t as f32) * (end_lab.b - start_lab.b),
                white_point: start_lab.white_point,
            };

            // Convert to display formats
            let rgb_values = ColorUtils::lab_to_rgb(interpolated_lab);
            let hex_color = format!(
                "#{:02X}{:02X}{:02X}",
                rgb_values.0, rgb_values.1, rgb_values.2,
            );
            let wcag_luminance =
                ColorUtils::wcag_relative_luminance_rgb((rgb_values.0, rgb_values.1, rgb_values.2));

            // Calculate position
            let position = start_position as f64 + stop * position_range;

            gradient_values.push(GradientValue {
                position: format!("{}%", position.round() as u8),
                hex: hex_color,
                rgb: Utils::rgb_to_string(rgb_values.0, rgb_values.1, rgb_values.2),
                wcag_luminance: format!("{wcag_luminance:.3}"),
            });
        }

        Ok(gradient_values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gradient::easing::LinearEasing;

    #[test]
    fn test_intelligent_stop_calculator() {
        let calculator = IntelligentStopCalculator::new(0.42, 0.58);
        let stops = calculator.calculate_stops(5);

        assert_eq!(stops.len(), 5);
        assert_eq!(stops[0], 0.0);
        assert_eq!(stops[stops.len() - 1], 1.0);
    }

    #[test]
    fn test_equal_spacing_calculator() {
        let calculator = EqualSpacingCalculator;
        let stops = calculator.calculate_stops(4);

        assert_eq!(stops, vec![0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0]);
    }

    #[test]
    fn test_gradient_calculator() {
        let calculator = GradientCalculator::with_equal_spacing();
        let stops = calculator.calculate_stops(3);

        assert_eq!(stops, vec![0.0, 0.5, 1.0]);
    }

    #[test]
    fn test_gradient_values_generation() {
        let calculator = GradientCalculator::with_equal_spacing();
        let easing = LinearEasing;

        let start_lab = Lab::new(50.0, 0.0, 0.0);
        let end_lab = Lab::new(70.0, 0.0, 0.0);

        let values = calculator
            .generate_gradient_values(start_lab, end_lab, 3, 0, 100, &easing)
            .unwrap();

        assert_eq!(values.len(), 3);
        assert_eq!(values[0].position, "0%");
        assert_eq!(values[2].position, "100%");
    }
}
