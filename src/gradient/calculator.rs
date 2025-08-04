//! Gradient calculation algorithms and stop generation
//!
//! This module implements functional gradient calculation
//! and provides various algorithms for calculating gradient stops.

use super::easing::EasingFunction;
use crate::color_distance_strategies::{DistanceAlgorithm, calculate_distance};
use crate::config::INTELLIGENT_STOP_SAMPLE_POINTS;
use crate::utils::Utils;
use kurbo::{CubicBez, ParamCurve, Point};
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
    #[must_use]
    pub const fn new(ease_in: f64, ease_out: f64) -> Self {
        Self { ease_in, ease_out }
    }
}

impl GradientCalculationTemplate for IntelligentStopCalculator {
    fn generate_stops(&self, num_stops: usize) -> Vec<f64> {
        if let Some(simple_stops) = self.handle_simple_cases(num_stops) {
            return simple_stops;
        }

        let curve = self.create_bezier_curve();
        let cumulative_importance = self.calculate_cumulative_importance(&curve);
        let total_importance = cumulative_importance[INTELLIGENT_STOP_SAMPLE_POINTS];
        
        if total_importance == 0.0 {
            return self.fallback_to_equal_spacing(num_stops);
        }

        self.distribute_stops_by_importance(num_stops, &cumulative_importance, total_importance)
    }
}

impl IntelligentStopCalculator {
    /// Handle simple cases (0-2 stops) with direct return values
    fn handle_simple_cases(&self, num_stops: usize) -> Option<Vec<f64>> {
        match num_stops {
            0 | 1 => Some(vec![0.0]),
            2 => Some(vec![0.0, 1.0]),
            _ => None,
        }
    }

    /// Create cubic bezier curve for easing analysis
    fn create_bezier_curve(&self) -> CubicBez {
        CubicBez::new(
            Point::new(0.0, 0.0),
            Point::new(self.ease_in, 0.0),
            Point::new(self.ease_out, 1.0),
            Point::new(1.0, 1.0),
        )
    }

    /// Calculate cumulative importance values along the curve
    fn calculate_cumulative_importance(&self, curve: &CubicBez) -> Vec<f64> {
        let mut cumulative_importance = vec![0.0; INTELLIGENT_STOP_SAMPLE_POINTS + 1];

        for i in 0..INTELLIGENT_STOP_SAMPLE_POINTS {
            let derivative_magnitude = self.calculate_derivative_magnitude(curve, i);
            cumulative_importance[i + 1] = cumulative_importance[i] + derivative_magnitude;
        }

        cumulative_importance
    }

    /// Calculate derivative magnitude at a specific sample point
    fn calculate_derivative_magnitude(&self, curve: &CubicBez, sample_index: usize) -> f64 {
        let t = sample_index as f64 / INTELLIGENT_STOP_SAMPLE_POINTS as f64;
        let dt = 1.0 / INTELLIGENT_STOP_SAMPLE_POINTS as f64;

        let current_point = curve.eval(t);
        let next_point = curve.eval((t + dt).min(1.0));

        let dy = next_point.y - current_point.y;
        dy.abs()
    }

    /// Fallback to equal spacing when no importance variation is detected
    fn fallback_to_equal_spacing(&self, num_stops: usize) -> Vec<f64> {
        (0..num_stops)
            .map(|i| i as f64 / (num_stops - 1).max(1) as f64)
            .collect()
    }

    /// Distribute stops based on cumulative importance using binary search
    fn distribute_stops_by_importance(
        &self,
        num_stops: usize,
        cumulative_importance: &[f64],
        total_importance: f64,
    ) -> Vec<f64> {
        let mut stops = Vec::new();
        
        for i in 0..num_stops {
            let target_importance = (i as f64 / (num_stops - 1).max(1) as f64) * total_importance;
            let t = self.find_position_for_importance(cumulative_importance, target_importance);
            stops.push(t.clamp(0.0, 1.0));
        }

        stops
    }

    /// Find the position (t) for a given target importance using binary search
    fn find_position_for_importance(&self, cumulative_importance: &[f64], target_importance: f64) -> f64 {
        let (low, high) = self.binary_search_importance(cumulative_importance, target_importance);
        self.interpolate_position(cumulative_importance, target_importance, low, high)
    }

    /// Binary search to find the importance range containing the target
    fn binary_search_importance(&self, cumulative_importance: &[f64], target_importance: f64) -> (usize, usize) {
        let mut low = 0;
        let mut high = INTELLIGENT_STOP_SAMPLE_POINTS;

        while high - low > 1 {
            let mid = usize::midpoint(low, high);
            if cumulative_importance[mid] < target_importance {
                low = mid;
            } else {
                high = mid;
            }
        }

        (low, high)
    }

    /// Linear interpolation between two closest importance points
    fn interpolate_position(
        &self,
        cumulative_importance: &[f64],
        target_importance: f64,
        low: usize,
        high: usize,
    ) -> f64 {
        if cumulative_importance[high] - cumulative_importance[low] > f64::EPSILON {
            let ratio = (target_importance - cumulative_importance[low])
                / (cumulative_importance[high] - cumulative_importance[low]);
            (low as f64 + ratio) / INTELLIGENT_STOP_SAMPLE_POINTS as f64
        } else {
            low as f64 / INTELLIGENT_STOP_SAMPLE_POINTS as f64
        }
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

    /// Cubic bezier easing function - proper CSS cubic-bezier implementation
    #[must_use]
    pub fn cubic_bezier_ease(t: f64, x1: f64, x2: f64) -> f64 {
        // CSS cubic-bezier(x1, 0, x2, 1) function
        // Control points: (0,0), (x1,0), (x2,1), (1,1)
        // We need to find Y given X=t using Newton-Raphson iteration

        if t <= 0.0 {
            return 0.0;
        }
        if t >= 1.0 {
            return 1.0;
        }

        // Newton-Raphson iteration to solve X(u) = t for parameter u
        let mut u = t; // Initial guess

        for _ in 0..8 {
            // 8 iterations should be enough for precision
            // Calculate X(u) using cubic bezier formula
            let u2 = u * u;
            let u3 = u2 * u;
            let inv_u = 1.0 - u;
            let inv_u2 = inv_u * inv_u;

            // X(u) = 3(1-u)²u*x1 + 3(1-u)u²*x2 + u³
            let x = 3.0 * inv_u2 * u * x1 + 3.0 * inv_u * u2 * x2 + u3;

            // X'(u) = 3(1-u)²*x1 + 6(1-u)u*(x2-x1) + 3u²*(1-x2)
            let dx = 3.0 * inv_u2 * x1 + 6.0 * inv_u * u * (x2 - x1) + 3.0 * u2 * (1.0 - x2);

            if dx.abs() < 1e-12 {
                break;
            } // Avoid division by zero

            u = u - (x - t) / dx;
            u = u.clamp(0.0, 1.0);

            if (x - t).abs() < 1e-12 {
                break;
            } // Converged
        }

        // Now calculate Y(u) using the found parameter u
        let u2 = u * u;
        let u3 = u2 * u;
        let inv_u = 1.0 - u;

        // Y(u) = 3(1-u)²u*0 + 3(1-u)u²*1 + u³*1 = 3(1-u)u² + u³
        3.0 * inv_u * u2 + u3
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
        let range = f64::from(end_pos) - f64::from(start_pos);

        stops
            .into_iter()
            .map(|stop| {
                let position = stop.mul_add(range, f64::from(start_pos));
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
                let bezier_t = Self::cubic_bezier_ease(t, ease_in, ease_out);

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
                        let bezier_t = Self::cubic_bezier_ease(mid_t, ease_in, ease_out);
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
                    let final_bezier_t = Self::cubic_bezier_ease(best_t, ease_in, ease_out);
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
}
