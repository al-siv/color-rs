//! Gradient calculation modules
//!
//! This module provides gradient calculation functionality split into focused submodules:
//! - `algorithms`: Core calculation algorithms (intelligent stops, equal spacing, bezier)
//! - `core`: Main calculator structures and unified gradient generation

pub mod algorithms;
pub mod core;

// Re-export main functionality for clean API
pub use algorithms::{EqualSpacingCalculator, IntelligentStopCalculator, cubic_bezier_ease};

pub use core::{CalculationAlgorithm, GradientCalculator, GradientValue, UnifiedGradientStop};

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::gradient::easing::EasingFunction;
    use palette::Lab;

    #[test]
    fn test_algorithm_integration() {
        // Test that algorithms work correctly through the re-exports
        let intelligent = IntelligentStopCalculator::new(0.42, 0.58);
        let equal_spacing = EqualSpacingCalculator;

        let intelligent_stops = intelligent.calculate_stops(5);
        let equal_stops = equal_spacing.calculate_stops(5);

        assert_eq!(intelligent_stops.len(), 5);
        assert_eq!(equal_stops.len(), 5);
        assert_eq!(equal_stops, vec![0.0, 0.25, 0.5, 0.75, 1.0]);
    }

    #[test]
    fn test_core_calculator_integration() {
        // Test that the main calculator works with both algorithms
        let calculator1 = GradientCalculator::with_equal_spacing();
        let calculator2 = GradientCalculator::with_intelligent_stops(0.42, 0.58);

        let stops1 = calculator1.calculate_stops(3);
        let stops2 = calculator2.calculate_stops(3);

        assert_eq!(stops1, vec![0.0, 0.5, 1.0]);
        assert_eq!(stops2.len(), 3);
        assert_eq!(stops2[0], 0.0);
        assert_eq!(stops2[2], 1.0);
    }

    #[test]
    fn test_unified_gradient_calculation() {
        // Test the unified gradient calculation function
        let start_lab = Lab::new(30.0, 10.0, -20.0);
        let end_lab = Lab::new(80.0, -30.0, 40.0);

        let simple_stops = GradientCalculator::calculate_unified_gradient(
            start_lab, end_lab, 0, 100, 0.25, 0.75, 4, true,
        );

        let smart_stops = GradientCalculator::calculate_unified_gradient(
            start_lab, end_lab, 0, 100, 0.25, 0.75, 4, false,
        );

        assert_eq!(simple_stops.len(), 4);
        assert_eq!(smart_stops.len(), 4);

        // Both should have same start and end positions
        assert_eq!(simple_stops[0].position, 0);
        assert_eq!(simple_stops[3].position, 100);
        assert_eq!(smart_stops[0].position, 0);
        assert_eq!(smart_stops[3].position, 100);
    }

    #[test]
    fn test_gradient_values_generation() {
        // Test the gradient values generation for display
        let calculator = GradientCalculator::with_equal_spacing();
        let easing = EasingFunction::Linear;
        let start_lab = Lab::new(50.0, 0.0, 0.0);
        let end_lab = Lab::new(70.0, 0.0, 0.0);

        let values = calculator
            .generate_gradient_values(start_lab, end_lab, 3, 10, 90, &easing)
            .unwrap();

        assert_eq!(values.len(), 3);
        assert_eq!(values[0].position, "10%");
        assert_eq!(values[1].position, "50%");
        assert_eq!(values[2].position, "90%");

        // Check that all values have required fields
        for value in &values {
            assert!(!value.hex.is_empty());
            assert!(!value.rgb.is_empty());
            assert!(!value.wcag_luminance.is_empty());
        }
    }

    #[test]
    fn test_cubic_bezier_ease_function() {
        // Test the cubic bezier easing function
        assert_eq!(cubic_bezier_ease(0.0, 0.42, 0.58), 0.0);
        assert_eq!(cubic_bezier_ease(1.0, 0.42, 0.58), 1.0);

        let mid_result = cubic_bezier_ease(0.5, 0.42, 0.58);
        assert!(mid_result > 0.0 && mid_result < 1.0);

        // Linear easing should return input
        let linear_result = cubic_bezier_ease(0.3, 0.0, 1.0);
        assert!((linear_result - 0.3).abs() < 0.001);
    }
}
