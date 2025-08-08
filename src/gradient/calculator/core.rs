//! Core gradient calculation functionality
//!
//! This module contains the main gradient calculator implementations,
//! unified gradient generation, and display value creation.

use super::algorithms::{EqualSpacingCalculator, IntelligentStopCalculator};
use crate::gradient::easing::EasingFunction;
use crate::utils::Utils;
use palette::{IntoColor, Lab, Srgb};
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

/// Main gradient calculator using functional approach with algorithm selection
#[derive(Debug, Clone)]
pub struct GradientCalculator {
    algorithm: CalculationAlgorithm,
}

/// Algorithm selection for gradient calculation
#[derive(Debug, Clone)]
pub enum CalculationAlgorithm {
    Intelligent { ease_in: f64, ease_out: f64 },
    EqualSpacing,
}

impl GradientCalculator {
    /// Create calculator with intelligent stop algorithm
    #[must_use]
    pub fn with_intelligent_stops(ease_in: f64, ease_out: f64) -> Self {
        Self {
            algorithm: CalculationAlgorithm::Intelligent { ease_in, ease_out },
        }
    }

    /// Create calculator with equal spacing
    #[must_use]
    pub fn with_equal_spacing() -> Self {
        Self {
            algorithm: CalculationAlgorithm::EqualSpacing,
        }
    }

    /// Calculate stop positions using selected algorithm
    #[must_use]
    pub fn calculate_stops(&self, num_stops: usize) -> Vec<f64> {
        match &self.algorithm {
            CalculationAlgorithm::Intelligent { ease_in, ease_out } => {
                let calculator = IntelligentStopCalculator::new(*ease_in, *ease_out);
                calculator.calculate_stops(num_stops)
            }
            CalculationAlgorithm::EqualSpacing => {
                let calculator = EqualSpacingCalculator;
                calculator.calculate_stops(num_stops)
            }
        }
    }

    /// Calculate integer stop positions (0-100 range)
    #[must_use]
    pub fn calculate_stops_integer(&self, num_stops: usize, start_pos: u8, end_pos: u8) -> Vec<u8> {
        let stops = self.calculate_stops(num_stops);
        let range = f64::from(end_pos) - f64::from(start_pos);

        stops
            .into_iter()
            .map(|stop| {
                let position = stop.mul_add(range, f64::from(start_pos));
                position.round().clamp(0.0, 255.0) as u8
            })
            .collect()
    }

    /// Generate gradient values for display
    pub fn generate_gradient_values(
        &self,
        start_lab: Lab,
        end_lab: Lab,
        num_stops: usize,
        start_position: u8,
        end_position: u8,
        easing_function: &EasingFunction,
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
        let position_range = f64::from(end_position) - f64::from(start_position);

        for &stop in &stops {
            // Apply easing function
            let eased_t = easing_function.ease(stop);

            // Interpolate color in LAB space
            let interpolated_lab = Lab {
                l: (eased_t as f32).mul_add(end_lab.l - start_lab.l, start_lab.l),
                a: (eased_t as f32).mul_add(end_lab.a - start_lab.a, start_lab.a),
                b: (eased_t as f32).mul_add(end_lab.b - start_lab.b, start_lab.b),
                white_point: start_lab.white_point,
            };

            // Convert to display formats using functional conversion
            let srgb: Srgb = interpolated_lab.into_color();
            let r = (srgb.red * 255.0).round() as u8;
            let g = (srgb.green * 255.0).round() as u8;
            let b = (srgb.blue * 255.0).round() as u8;

            let hex_color = format!("#{r:02X}{g:02X}{b:02X}");
            let wcag_luminance = crate::color_ops::luminance::wcag_relative(srgb);

            // Calculate position
            let position = stop.mul_add(position_range, f64::from(start_position));

            gradient_values.push(GradientValue {
                position: format!("{}%", position.round() as u8),
                hex: hex_color,
                rgb: Utils::rgb_to_string(r, g, b),
                wcag_luminance:
                    crate::precision_utils::PrecisionUtils::format_wcag_relative_luminance(
                        wcag_luminance,
                    ),
            });
        }

        Ok(gradient_values)
    }

    /// Unified gradient calculation function for both YAML and SVG generation
    /// This ensures consistent gradient calculation across all output formats
    /// Config-struct-based API to avoid too many arguments
    pub fn calculate_unified_gradient_cfg(cfg: crate::gradient::unified_calculator::GradientCalculationConfig) -> Vec<UnifiedGradientStop> {
    // Delegate to the functional implementation in unified_calculator
    crate::gradient::unified_calculator::calculate_unified_gradient_cfg(cfg)
    }
}

/// Unified gradient stop data structure
/// Contains all necessary information for both YAML and SVG generation
#[derive(Debug, Clone)]
pub struct UnifiedGradientStop {
    pub position: u8,
    pub geometric_t: f64, // Position in 0-1 range for geometric calculations
    pub bezier_t: f64,    // Position after bezier easing applied
    pub lab_color: Lab,   // Color in LAB space
    pub rgb_color: (u8, u8, u8), // Color in RGB space
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gradient::easing::EasingFunction;

    #[test]
    fn test_gradient_calculator_equal_spacing() {
        let calculator = GradientCalculator::with_equal_spacing();
        let stops = calculator.calculate_stops(3);

        assert_eq!(stops, vec![0.0, 0.5, 1.0]);
    }

    #[test]
    fn test_gradient_calculator_intelligent() {
        let calculator = GradientCalculator::with_intelligent_stops(0.42, 0.58);
        let stops = calculator.calculate_stops(5);

        assert_eq!(stops.len(), 5);
        assert_eq!(stops[0], 0.0);
        assert_eq!(stops[stops.len() - 1], 1.0);
    }

    #[test]
    fn test_gradient_values_generation() {
        let calculator = GradientCalculator::with_equal_spacing();
        let easing = EasingFunction::Linear;

        let start_lab = Lab::new(50.0, 0.0, 0.0);
        let end_lab = Lab::new(70.0, 0.0, 0.0);

        let values = calculator
            .generate_gradient_values(start_lab, end_lab, 3, 0, 100, &easing)
            .unwrap();

        assert_eq!(values.len(), 3);
        assert_eq!(values[0].position, "0%");
        assert_eq!(values[2].position, "100%");
    }

    #[test]
    fn test_unified_gradient_calculation() {
        let start_lab = Lab::new(50.0, 0.0, 0.0);
        let end_lab = Lab::new(70.0, 0.0, 0.0);

        use crate::color_distance_strategies::DistanceAlgorithm;
        let cfg = crate::gradient::unified_calculator::GradientCalculationConfig {
            start_lab,
            end_lab,
            start_position: 0,
            end_position: 100,
            ease_in: 0.42,
            ease_out: 0.58,
            steps: 3,
            use_simple_mode: true, // simple mode
            algorithm: DistanceAlgorithm::DeltaE2000,
        };
        let stops = GradientCalculator::calculate_unified_gradient_cfg(cfg);

        assert_eq!(stops.len(), 3);
        assert_eq!(stops[0].position, 0);
        assert_eq!(stops[2].position, 100);
    }
}
