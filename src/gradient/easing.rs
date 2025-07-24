//! Easing functions and curve calculations for gradient generation
//!
//! This module implements various easing strategies using the Strategy design pattern
//! to provide different timing functions for gradient interpolation.

use crate::config::{BEZIER_MAX, BEZIER_MIN};
use kurbo::{CubicBez, ParamCurve, Point};

/// Enum representing different types of easing functions
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EasingType {
    #[default]
    Linear,
    CubicBezier,
    Smooth,
}

/// Strategy trait for different easing functions
pub trait EasingStrategy {
    /// Calculate the eased value for a given time parameter t (0.0 to 1.0)
    fn ease(&self, t: f64) -> f64;

    /// Get the name of this easing strategy
    fn name(&self) -> &'static str;
}

/// Cubic Bezier easing strategy
/// Implements cubic-bezier(x1, 0, x2, 1) easing functions
/// This matches CSS timing functions like ease-in-out: cubic-bezier(0.42, 0, 0.58, 1)
pub struct CubicBezierEasing {
    x1: f64,
    x2: f64,
}

impl CubicBezierEasing {
    /// Create a new cubic bezier easing with control points
    #[must_use]
    pub const fn new(x1: f64, x2: f64) -> Self {
        Self {
            x1: x1.clamp(BEZIER_MIN, BEZIER_MAX),
            x2: x2.clamp(BEZIER_MIN, BEZIER_MAX),
        }
    }

    /// Create ease-in-out timing function
    #[must_use]
    pub const fn ease_in_out() -> Self {
        Self::new(0.42, 0.58)
    }

    /// Create ease-in timing function
    #[must_use]
    pub const fn ease_in() -> Self {
        Self::new(0.42, 1.0)
    }

    /// Create ease-out timing function
    #[must_use]
    pub const fn ease_out() -> Self {
        Self::new(0.0, 0.58)
    }
}

impl EasingStrategy for CubicBezierEasing {
    fn ease(&self, t: f64) -> f64 {
        if t <= 0.0 {
            return 0.0;
        }
        if t >= 1.0 {
            return 1.0;
        }

        // Create cubic bezier curve with control points (0,0), (x1,0), (x2,1), (1,1)
        // This matches cubic-bezier specification
        let curve = CubicBez::new(
            Point::new(0.0, 0.0),     // Start point
            Point::new(self.x1, 0.0), // First control point (x1, 0)
            Point::new(self.x2, 1.0), // Second control point (x2, 1)
            Point::new(1.0, 1.0),     // End point
        );

        // Find parameter value that corresponds to input t using binary search
        let mut low = 0.0;
        let mut high = 1.0;
        let epsilon = 1e-7;

        while high - low > epsilon {
            let mid = f64::midpoint(low, high);
            let point = curve.eval(mid);

            if point.x < t {
                low = mid;
            } else {
                high = mid;
            }
        }

        let final_param = f64::midpoint(low, high);
        curve.eval(final_param).y
    }

    fn name(&self) -> &'static str {
        "Cubic Bezier"
    }
}

/// Linear easing strategy (no easing)
pub struct LinearEasing;

impl EasingStrategy for LinearEasing {
    fn ease(&self, t: f64) -> f64 {
        t.clamp(0.0, 1.0)
    }

    fn name(&self) -> &'static str {
        "Linear"
    }
}

/// Factory for creating easing strategies
pub struct EasingFactory;

impl EasingFactory {
    /// Create an easing strategy from type and parameters
    #[must_use]
    pub fn create_easing(
        easing_type: EasingType,
        ease_in: f64,
        ease_out: f64,
    ) -> Box<dyn EasingStrategy> {
        match easing_type {
            EasingType::Linear => Self::create_linear(),
            EasingType::CubicBezier => Self::create_cubic_bezier(ease_in, ease_out),
            EasingType::Smooth => Self::create_ease_in_out(),
        }
    }

    /// Create an easing strategy from parameters
    #[must_use]
    pub fn create_cubic_bezier(x1: f64, x2: f64) -> Box<dyn EasingStrategy> {
        Box::new(CubicBezierEasing::new(x1, x2))
    }

    /// Create a linear easing strategy
    #[must_use]
    pub fn create_linear() -> Box<dyn EasingStrategy> {
        Box::new(LinearEasing)
    }

    /// Create ease-in-out strategy
    #[must_use]
    pub fn create_ease_in_out() -> Box<dyn EasingStrategy> {
        Box::new(CubicBezierEasing::ease_in_out())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubic_bezier_ease() {
        let easing = CubicBezierEasing::new(0.42, 0.58);

        // Test edge cases
        assert_eq!(easing.ease(0.0), 0.0);
        assert_eq!(easing.ease(1.0), 1.0);

        // Test middle value
        let mid = easing.ease(0.5);
        assert!(mid > 0.0 && mid < 1.0);
    }

    #[test]
    fn test_linear_easing() {
        let easing = LinearEasing;

        assert_eq!(easing.ease(0.0), 0.0);
        assert_eq!(easing.ease(0.5), 0.5);
        assert_eq!(easing.ease(1.0), 1.0);
    }

    #[test]
    fn test_easing_factory() {
        let cubic = EasingFactory::create_cubic_bezier(0.25, 0.75);
        let linear = EasingFactory::create_linear();

        assert_eq!(cubic.name(), "Cubic Bezier");
        assert_eq!(linear.name(), "Linear");
    }
}
