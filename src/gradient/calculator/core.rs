//! Core gradient calculation functionality
//!
//! This module contains the main gradient calculator implementations,
//! unified gradient generation, and display value creation.

use super::algorithms::{IntelligentStopCalculator, EqualSpacingCalculator, cubic_bezier_ease};
use crate::color_distance_strategies::{DistanceAlgorithm, calculate_distance};
use crate::utils::Utils;
use crate::gradient::easing::EasingFunction;
use palette::{IntoColor, Lab, Mix, Srgb};
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
            
            let hex_color = format!("#{:02X}{:02X}{:02X}", r, g, b);
            let wcag_luminance = crate::color_ops::luminance::wcag_relative(srgb);

            // Calculate position
            let position = stop.mul_add(position_range, f64::from(start_position));

            gradient_values.push(GradientValue {
                position: format!("{}%", position.round() as u8),
                hex: hex_color,
                rgb: Utils::rgb_to_string(r, g, b),
                wcag_luminance: crate::precision_utils::PrecisionUtils::format_wcag_relative_luminance(wcag_luminance),
            });
        }

        Ok(gradient_values)
    }

    /// Unified gradient calculation function for both YAML and SVG generation
    /// This ensures consistent gradient calculation across all output formats
    pub fn calculate_unified_gradient(
        start_lab: Lab,
        end_lab: Lab,
        start_position: u8,
        end_position: u8,
        ease_in: f64,
        ease_out: f64,
        steps: usize,
        use_simple_mode: bool,
    ) -> Vec<UnifiedGradientStop> {
        Self::calculate_unified_gradient_with_algorithm(
            start_lab,
            end_lab,
            start_position,
            end_position,
            ease_in,
            ease_out,
            steps,
            use_simple_mode,
            DistanceAlgorithm::DeltaE2000,
        )
    }

    /// Unified gradient calculation function with custom distance strategy
    /// This allows testing different color distance algorithms
    pub fn calculate_unified_gradient_with_algorithm(
        start_lab: Lab,
        end_lab: Lab,
        start_position: u8,
        end_position: u8,
        ease_in: f64,
        ease_out: f64,
        steps: usize,
        use_simple_mode: bool,
        algorithm: DistanceAlgorithm,
    ) -> Vec<UnifiedGradientStop> {
        let mut gradient_stops = Vec::new();

        if use_simple_mode {
            // Simple mode: equal geometric intervals with RGB interpolation + bezier easing
            let start_srgb: Srgb = start_lab.into_color();
            let start_rgb = (
                (start_srgb.red * 255.0).round() as u8,
                (start_srgb.green * 255.0).round() as u8,
                (start_srgb.blue * 255.0).round() as u8,
            );
            let end_srgb: Srgb = end_lab.into_color();
            let end_rgb = (
                (end_srgb.red * 255.0).round() as u8,
                (end_srgb.green * 255.0).round() as u8,
                (end_srgb.blue * 255.0).round() as u8,
            );

            for i in 0..steps {
                let t = i as f64 / (steps - 1) as f64;

                // Apply bezier easing to geometric progression
                let bezier_t = cubic_bezier_ease(t, ease_in, ease_out);

                // RGB interpolation with bezier timing
                let r = (start_rgb.0 as f64 + (end_rgb.0 as f64 - start_rgb.0 as f64) * bezier_t)
                    .round() as u8;
                let g = (start_rgb.1 as f64 + (end_rgb.1 as f64 - start_rgb.1 as f64) * bezier_t)
                    .round() as u8;
                let b = (start_rgb.2 as f64 + (end_rgb.2 as f64 - start_rgb.2 as f64) * bezier_t)
                    .round() as u8;

                // Convert back to LAB for consistent output format using functional conversion
                let rgb_srgb = Srgb::new(
                    f32::from(r) / 255.0,
                    f32::from(g) / 255.0,
                    f32::from(b) / 255.0,
                );
                let rgb_lab: Lab = rgb_srgb.into_color();

                let position = (start_position as f64 + t * (end_position - start_position) as f64)
                    .round() as u8;

                gradient_stops.push(UnifiedGradientStop {
                    position,
                    geometric_t: t,
                    bezier_t,
                    lab_color: rgb_lab,
                    rgb_color: (r, g, b),
                });
            }
        } else {
            // Smart mode: Equal distance with geometric position finding using custom algorithm
            // Calculate total distance between start and end colors using provided algorithm
            let total_distance = calculate_distance(algorithm, start_lab, end_lab);
            let step_distance = total_distance / (steps - 1) as f64;

            for i in 0..steps {
                if i == 0 {
                    // First stop: use start color
                    let start_srgb: Srgb = start_lab.into_color();
                    let start_rgb = (
                        (start_srgb.red * 255.0).round() as u8,
                        (start_srgb.green * 255.0).round() as u8,
                        (start_srgb.blue * 255.0).round() as u8,
                    );
                    gradient_stops.push(UnifiedGradientStop {
                        position: start_position,
                        geometric_t: 0.0,
                        bezier_t: 0.0,
                        lab_color: start_lab,
                        rgb_color: start_rgb,
                    });
                } else if i == steps - 1 {
                    // Last stop: use end color
                    let end_srgb: Srgb = end_lab.into_color();
                    let end_rgb = (
                        (end_srgb.red * 255.0).round() as u8,
                        (end_srgb.green * 255.0).round() as u8,
                        (end_srgb.blue * 255.0).round() as u8,
                    );
                    gradient_stops.push(UnifiedGradientStop {
                        position: end_position,
                        geometric_t: 1.0,
                        bezier_t: 1.0,
                        lab_color: end_lab,
                        rgb_color: end_rgb,
                    });
                } else {
                    // Middle stops: find geometric position that produces target Delta E distance
                    let target_distance = step_distance * i as f64;

                    // Binary search to find geometric_t that produces target distance
                    let mut low = 0.0;
                    let mut high = 1.0;
                    let mut best_t = 0.5;

                    for _ in 0..50 {
                        // Binary search with 50 iterations for precision
                        let mid_t = (low + high) / 2.0;
                        let bezier_t = cubic_bezier_ease(mid_t, ease_in, ease_out);
                        let test_color = start_lab.mix(end_lab, bezier_t as f32);
                        let actual_distance = calculate_distance(algorithm, start_lab, test_color);

                        if (actual_distance - target_distance).abs() < 0.01 {
                            best_t = mid_t;
                            break;
                        }

                        if actual_distance < target_distance {
                            low = mid_t;
                        } else {
                            high = mid_t;
                        }

                        best_t = mid_t;
                    }

                    // Calculate final bezier_t and actual color using found geometric position
                    let final_bezier_t = cubic_bezier_ease(best_t, ease_in, ease_out);
                    let actual_lab = start_lab.mix(end_lab, final_bezier_t as f32);
                    let actual_srgb: Srgb = actual_lab.into_color();
                    let rgb_color = (
                        (actual_srgb.red * 255.0).round() as u8,
                        (actual_srgb.green * 255.0).round() as u8,
                        (actual_srgb.blue * 255.0).round() as u8,
                    );

                    // Calculate position using the found geometric t
                    let position = (start_position as f64
                        + best_t * (end_position - start_position) as f64)
                        .round() as u8;

                    gradient_stops.push(UnifiedGradientStop {
                        position,
                        geometric_t: best_t,
                        bezier_t: final_bezier_t,
                        lab_color: actual_lab,
                        rgb_color,
                    });
                }
            }
        }

        gradient_stops
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

        let stops = GradientCalculator::calculate_unified_gradient(
            start_lab,
            end_lab,
            0,
            100,
            0.42,
            0.58,
            3,
            true, // simple mode
        );

        assert_eq!(stops.len(), 3);
        assert_eq!(stops[0].position, 0);
        assert_eq!(stops[2].position, 100);
    }
}
