//! Legacy template patterns for gradient calculation
//!
//! This module contains the template method pattern interfaces
//! that are scheduled for replacement with functional patterns.
//! These are maintained for backward compatibility during migration.

use super::algorithms::{IntelligentStopCalculator, EqualSpacingCalculator};

/// Abstract template for gradient calculation algorithms
/// 
/// WARNING: This trait implements the Template Method pattern which is deprecated
/// in favor of functional composition. Use the functions in `algorithms.rs` and
/// `core.rs` for new code. This interface is maintained only for backward
/// compatibility during the migration phase.
#[deprecated(since = "0.17.0", note = "Use functional algorithms in algorithms.rs instead")]
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

/// Legacy wrapper for IntelligentStopCalculator using Template Method pattern
#[deprecated(since = "0.17.0", note = "Use IntelligentStopCalculator directly from algorithms.rs")]
pub struct LegacyIntelligentStopCalculator {
    calculator: IntelligentStopCalculator,
}

impl LegacyIntelligentStopCalculator {
    #[must_use]
    pub const fn new(ease_in: f64, ease_out: f64) -> Self {
        Self {
            calculator: IntelligentStopCalculator::new(ease_in, ease_out),
        }
    }
}

#[allow(deprecated)]
impl GradientCalculationTemplate for LegacyIntelligentStopCalculator {
    fn generate_stops(&self, num_stops: usize) -> Vec<f64> {
        self.calculator.calculate_stops(num_stops)
    }
}

/// Legacy wrapper for EqualSpacingCalculator using Template Method pattern
#[deprecated(since = "0.17.0", note = "Use EqualSpacingCalculator directly from algorithms.rs")]
pub struct LegacyEqualSpacingCalculator {
    calculator: EqualSpacingCalculator,
}

impl LegacyEqualSpacingCalculator {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            calculator: EqualSpacingCalculator,
        }
    }
}

impl Default for LegacyEqualSpacingCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(deprecated)]
impl GradientCalculationTemplate for LegacyEqualSpacingCalculator {
    fn generate_stops(&self, num_stops: usize) -> Vec<f64> {
        self.calculator.calculate_stops(num_stops)
    }
}

/// Legacy gradient calculator using Strategy pattern with Box<dyn>
/// 
/// WARNING: This struct uses the Strategy pattern with heap allocation
/// which is deprecated in favor of enum dispatch. Use `GradientCalculator`
/// from `core.rs` for new code.
#[deprecated(since = "0.17.0", note = "Use GradientCalculator from core.rs instead")]
pub struct LegacyGradientCalculator {
    calculator: Box<dyn GradientCalculationTemplate>,
}

#[allow(deprecated)]
impl LegacyGradientCalculator {
    /// Create calculator with intelligent stop algorithm
    #[must_use]
    pub fn with_intelligent_stops(ease_in: f64, ease_out: f64) -> Self {
        Self {
            calculator: Box::new(LegacyIntelligentStopCalculator::new(ease_in, ease_out)),
        }
    }

    /// Create calculator with equal spacing
    #[must_use]
    pub fn with_equal_spacing() -> Self {
        Self {
            calculator: Box::new(LegacyEqualSpacingCalculator::new()),
        }
    }

    /// Calculate stop positions
    #[must_use]
    pub fn calculate_stops(&self, num_stops: usize) -> Vec<f64> {
        self.calculator.calculate_stops(num_stops)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn test_legacy_intelligent_calculator() {
        let calculator = LegacyIntelligentStopCalculator::new(0.42, 0.58);
        let stops = calculator.calculate_stops(5);

        assert_eq!(stops.len(), 5);
        assert_eq!(stops[0], 0.0);
        assert_eq!(stops[stops.len() - 1], 1.0);
    }

    #[test]
    #[allow(deprecated)]
    fn test_legacy_equal_spacing_calculator() {
        let calculator = LegacyEqualSpacingCalculator::new();
        let stops = calculator.calculate_stops(4);

        assert_eq!(stops, vec![0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0]);
    }

    #[test]
    #[allow(deprecated)]
    fn test_legacy_gradient_calculator() {
        let calculator = LegacyGradientCalculator::with_equal_spacing();
        let stops = calculator.calculate_stops(3);

        assert_eq!(stops, vec![0.0, 0.5, 1.0]);
    }
}
