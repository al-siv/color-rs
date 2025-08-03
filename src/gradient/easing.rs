//! Easing functions and curve calculations for gradient generation
//!
//! This module implements various easing functions using functional enum dispatch
//! to provide different timing functions for gradient interpolation with zero-cost abstractions.

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

/// Functional easing implementation using enum dispatch for zero-cost abstractions
#[derive(Debug, Clone, PartialEq)]
pub enum EasingFunction {
    /// Linear easing (no easing)
    Linear,
    /// Cubic Bezier easing with control points (x1, 0, x2, 1)
    CubicBezier { x1: f64, x2: f64 },
}

impl Default for EasingFunction {
    fn default() -> Self {
        Self::Linear
    }
}

impl EasingFunction {
    /// Create a new cubic bezier easing with control points
    #[must_use]
    pub const fn cubic_bezier(x1: f64, x2: f64) -> Self {
        Self::CubicBezier { 
            x1: if x1 < BEZIER_MIN { BEZIER_MIN } else if x1 > BEZIER_MAX { BEZIER_MAX } else { x1 },
            x2: if x2 < BEZIER_MIN { BEZIER_MIN } else if x2 > BEZIER_MAX { BEZIER_MAX } else { x2 },
        }
    }

    /// Create ease-in-out timing function
    #[must_use]
    pub const fn ease_in_out() -> Self {
        Self::cubic_bezier(0.42, 0.58)
    }

    /// Create ease-in timing function
    #[must_use]
    pub const fn ease_in() -> Self {
        Self::cubic_bezier(0.42, 1.0)
    }

    /// Create ease-out timing function
    #[must_use]
    pub const fn ease_out() -> Self {
        Self::cubic_bezier(0.0, 0.58)
    }

    /// Calculate the eased value for a given time parameter t (0.0 to 1.0)
    #[must_use]
    pub fn ease(&self, t: f64) -> f64 {
        match self {
            Self::Linear => t.clamp(0.0, 1.0),
            Self::CubicBezier { x1, x2 } => self.cubic_bezier_ease(t, *x1, *x2),
        }
    }

    /// Get the name of this easing function
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Linear => "Linear",
            Self::CubicBezier { .. } => "Cubic Bezier",
        }
    }

    /// Internal cubic bezier calculation
    fn cubic_bezier_ease(&self, t: f64, x1: f64, x2: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);

        // Handle edge cases
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
            Point::new(x1, 0.0),      // First control point (x1, 0)
            Point::new(x2, 1.0),      // Second control point (x2, 1)
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
}

/// Factory for creating easing functions using functional patterns
pub struct EasingFactory;

impl EasingFactory {
    /// Create an easing function from type and parameters
    #[must_use]
    pub fn create_easing(
        easing_type: EasingType,
        ease_in: f64,
        ease_out: f64,
    ) -> EasingFunction {
        match easing_type {
            EasingType::Linear => EasingFunction::Linear,
            EasingType::CubicBezier => EasingFunction::cubic_bezier(ease_in, ease_out),
            EasingType::Smooth => EasingFunction::ease_in_out(),
        }
    }

    /// Create a cubic bezier easing function
    #[must_use]
    pub fn create_cubic_bezier(x1: f64, x2: f64) -> EasingFunction {
        EasingFunction::cubic_bezier(x1, x2)
    }

    /// Create a linear easing function
    #[must_use]
    pub const fn create_linear() -> EasingFunction {
        EasingFunction::Linear
    }

    /// Create ease-in-out easing function
    #[must_use]
    pub const fn create_ease_in_out() -> EasingFunction {
        EasingFunction::ease_in_out()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubic_bezier_ease() {
        let easing = EasingFunction::cubic_bezier(0.42, 0.58);

        // Test edge cases
        assert_eq!(easing.ease(0.0), 0.0);
        assert_eq!(easing.ease(1.0), 1.0);

        // Test middle value
        let mid = easing.ease(0.5);
        assert!(mid > 0.0 && mid < 1.0);
    }

    #[test]
    fn test_linear_easing() {
        let easing = EasingFunction::Linear;

        assert_eq!(easing.ease(0.0), 0.0);
        assert_eq!(easing.ease(0.5), 0.5);
        assert_eq!(easing.ease(1.0), 1.0);
    }

    #[test]
    fn test_cubic_bezier_edge_cases() {
        let easing = EasingFunction::cubic_bezier(0.25, 0.75);

        // Test values outside valid range are clamped
        assert_eq!(easing.ease(-0.1), 0.0);
        assert_eq!(easing.ease(1.1), 1.0);
    }

    #[test]
    fn test_factory_create_easing() {
        let linear = EasingFactory::create_easing(EasingType::Linear, 0.0, 0.0);
        assert!(matches!(linear, EasingFunction::Linear));

        let bezier = EasingFactory::create_easing(EasingType::CubicBezier, 0.42, 0.58);
        assert!(matches!(bezier, EasingFunction::CubicBezier { x1: 0.42, x2: 0.58 }));

        let smooth = EasingFactory::create_easing(EasingType::Smooth, 0.0, 0.0);
        assert!(matches!(smooth, EasingFunction::CubicBezier { x1: 0.42, x2: 0.58 }));
    }

    #[test]
    fn test_easing_names() {
        assert_eq!(EasingFunction::Linear.name(), "Linear");
        assert_eq!(EasingFunction::cubic_bezier(0.42, 0.58).name(), "Cubic Bezier");
    }

    #[test]
    fn test_convenience_constructors() {
        let ease_in_out = EasingFunction::ease_in_out();
        assert!(matches!(ease_in_out, EasingFunction::CubicBezier { x1: 0.42, x2: 0.58 }));

        let ease_in = EasingFunction::ease_in();
        assert!(matches!(ease_in, EasingFunction::CubicBezier { x1: 0.42, x2: 1.0 }));

        let ease_out = EasingFunction::ease_out();
        assert!(matches!(ease_out, EasingFunction::CubicBezier { x1: 0.0, x2: 0.58 }));
    }
}
