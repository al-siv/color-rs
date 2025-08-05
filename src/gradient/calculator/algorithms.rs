//! Gradient calculation algorithms
//!
//! This module contains the core algorithms for calculating gradient stops,
//! including intelligent positioning and binary search implementations.

use crate::config::INTELLIGENT_STOP_SAMPLE_POINTS;
use kurbo::{CubicBez, ParamCurve, Point};

/// Intelligent stop calculation using easing functions and importance analysis
#[derive(Debug, Clone)]
pub struct IntelligentStopCalculator {
    ease_in: f64,
    ease_out: f64,
}

impl IntelligentStopCalculator {
    /// Create new intelligent stop calculator with easing parameters
    #[must_use]
    pub const fn new(ease_in: f64, ease_out: f64) -> Self {
        Self { ease_in, ease_out }
    }

    /// Calculate gradient stops using intelligent positioning
    #[must_use]
    pub fn calculate_stops(&self, num_stops: usize) -> Vec<f64> {
        if let Some(simple_stops) = Self::handle_simple_cases(num_stops) {
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

    /// Handle simple cases (0-2 stops) with direct return values
    fn handle_simple_cases(num_stops: usize) -> Option<Vec<f64>> {
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
            let derivative_magnitude = Self::calculate_derivative_magnitude(curve, i);
            cumulative_importance[i + 1] = cumulative_importance[i] + derivative_magnitude;
        }

        cumulative_importance
    }

    /// Calculate derivative magnitude at a specific sample point
    fn calculate_derivative_magnitude(curve: &CubicBez, sample_index: usize) -> f64 {
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

/// Equal spacing stop calculation algorithm
#[derive(Debug, Clone)]
pub struct EqualSpacingCalculator;

impl EqualSpacingCalculator {
    /// Calculate evenly spaced gradient stops
    #[must_use]
    pub fn calculate_stops(&self, num_stops: usize) -> Vec<f64> {
        (0..num_stops)
            .map(|i| i as f64 / (num_stops - 1).max(1) as f64)
            .collect()
    }
}

/// Cubic bezier easing function - proper CSS cubic-bezier implementation
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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_cubic_bezier_ease() {
        // Test linear easing
        assert!((cubic_bezier_ease(0.5, 0.0, 1.0) - 0.5).abs() < 0.001);
        
        // Test ease-in
        let ease_in_result = cubic_bezier_ease(0.5, 0.42, 1.0);
        assert!(ease_in_result < 0.5); // Should be slower at start
        
        // Test edge cases
        assert_eq!(cubic_bezier_ease(0.0, 0.42, 0.58), 0.0);
        assert_eq!(cubic_bezier_ease(1.0, 0.42, 0.58), 1.0);
    }
}
