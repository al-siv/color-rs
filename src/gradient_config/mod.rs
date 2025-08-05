//! Gradient Configuration Module
//!
//! This module provides immutable alternatives to the traditional Builder pattern.
//! It uses smart constructors, compile-time validation, and value types to ensure
//! type safety and composable configuration.
//!
//! # Example
//! ```rust
//! use color_rs::gradient_config::{GradientConfig, ColorPair, EasingConfig};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let config = GradientConfig::new(
//!     ColorPair::new("#FF0000", "#0000FF")?,
//!     EasingConfig::ease_in_out()
//! )?
//! .with_svg_output("gradient.svg")?
//! .with_steps(10)?;
//!
//! let args = config.to_gradient_args();
//! # Ok(())
//! # }
//! ```

pub mod types;
pub mod validation;
pub mod config;
pub mod convenience;

// Re-export all public types and functions
pub use types::*;
pub use convenience::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_pair_creation() {
        let pair = ColorPair::new("#FF0000", "#0000FF").unwrap();
        assert_eq!(pair.start(), "#FF0000");
        assert_eq!(pair.end(), "#0000FF");
    }

    #[test]
    fn test_color_pair_validation() {
        assert!(ColorPair::new("", "#0000FF").is_err());
        assert!(ColorPair::new("#FF0000", "").is_err());
        assert!(ColorPair::new("  ", "#0000FF").is_err());
    }

    #[test]
    fn test_easing_presets() {
        let linear = EasingConfig::linear();
        assert_eq!(linear.ease_in_value(), 0.0);
        assert_eq!(linear.ease_out_value(), 1.0);

        let ease_in_out = EasingConfig::ease_in_out();
        assert_eq!(ease_in_out.ease_in_value(), 0.42);
        assert_eq!(ease_in_out.ease_out_value(), 0.58);
    }

    #[test]
    fn test_easing_validation() {
        assert!(EasingConfig::new(-0.1, 0.5).is_err());
        assert!(EasingConfig::new(0.5, 1.1).is_err());
        assert!(EasingConfig::new(0.0, 1.0).is_ok());
    }

    #[test]
    fn test_position_range() {
        let range = PositionRange::new(20, 80).unwrap();
        assert_eq!(range.start(), 20);
        assert_eq!(range.end(), 80);

        // Invalid ranges
        assert!(PositionRange::new(80, 20).is_err()); // start >= end
        assert!(PositionRange::new(50, 101).is_err()); // end > 100
    }

    #[test]
    fn test_image_output() {
        let svg_output = ImageOutput::svg("test.svg", 800, true).unwrap();
        assert!(svg_output.has_svg());
        assert!(!svg_output.has_png());
        assert_eq!(svg_output.svg_filename(), Some("test.svg"));
        assert_eq!(svg_output.width(), 800);
        assert!(svg_output.show_legend());

        // Invalid width
        assert!(ImageOutput::svg("test.svg", 0, true).is_err());
        // Empty filename
        assert!(ImageOutput::svg("", 800, true).is_err());
    }

    #[test]
    fn test_stop_config() {
        let steps = StopConfig::steps(10).unwrap();
        assert!(matches!(steps, StopConfig::Steps(10)));

        let intelligent = StopConfig::intelligent_stops(7);
        assert!(matches!(intelligent, StopConfig::IntelligentStops(7)));

        // Invalid step
        assert!(StopConfig::steps(0).is_err());
    }

    #[test]
    fn test_gradient_config_basic() {
        let colors = ColorPair::new("#FF0000", "#0000FF").unwrap();
        let easing = EasingConfig::ease_in_out();
        let config = GradientConfig::new(colors, easing).unwrap();

        assert_eq!(config.colors().start(), "#FF0000");
        assert_eq!(config.colors().end(), "#0000FF");
        assert_eq!(config.easing().ease_in_value(), 0.42);
        assert_eq!(config.easing().ease_out_value(), 0.58);
    }

    #[test]
    fn test_gradient_config_immutable_updates() {
        let original = linear_gradient("#FF0000", "#0000FF").unwrap();
        
        let with_svg = original.clone().with_svg_output("test.svg").unwrap();
        assert!(with_svg.image_output().has_svg());
        assert!(!original.image_output().has_svg()); // Original unchanged

        let with_steps = original.clone().with_steps(5).unwrap();
        assert!(matches!(with_steps.stop_config(), StopConfig::Steps(5)));
        assert!(matches!(original.stop_config(), StopConfig::IntelligentStops(5))); // Original unchanged
    }

    #[test]
    fn test_gradient_config_to_args() {
        let config = linear_gradient("#FF0000", "#0000FF")
            .unwrap()
            .with_svg_output("test.svg")
            .unwrap()
            .with_steps(10)
            .unwrap()
            .with_width(800)
            .unwrap();

        let args = config.to_gradient_args();
        assert_eq!(args.start_color, "#FF0000");
        assert_eq!(args.end_color, "#0000FF");
        assert_eq!(args.svg, Some("test.svg".to_string()));
        assert_eq!(args.step, Some(10));
        assert_eq!(args.width, 800);
    }

    #[test]
    fn test_convenience_functions() {
        let linear = linear_gradient("#FF0000", "#0000FF").unwrap();
        assert_eq!(linear.easing().ease_in_value(), 0.0);
        assert_eq!(linear.easing().ease_out_value(), 1.0);

        let smooth = smooth_gradient("red", "blue").unwrap();
        assert_eq!(smooth.easing().ease_in_value(), 0.42);
        assert_eq!(smooth.easing().ease_out_value(), 0.58);

        let positioned = positioned_gradient("red", "blue", 20, 80).unwrap();
        assert_eq!(positioned.position_range().start(), 20);
        assert_eq!(positioned.position_range().end(), 80);
    }

    #[test]
    fn test_gradient_composition() {
        // Test that we can chain operations
        let config = linear_gradient("#FF0000", "#0000FF")
            .and_then(|c| c.with_svg_output("gradient.svg"))
            .and_then(|c| c.with_steps(5))
            .and_then(|c| c.with_width(1200))
            .map(|c| c.with_legend(false))
            .unwrap();

        assert!(config.image_output().has_svg());
        assert_eq!(config.image_output().width(), 1200);
        assert!(!config.image_output().show_legend());
        assert!(matches!(config.stop_config(), StopConfig::Steps(5)));
    }

    #[test]
    fn test_error_propagation() {
        // Test that validation errors are properly propagated
        let result = linear_gradient("", "#0000FF");
        assert!(result.is_err());

        let result = linear_gradient("#FF0000", "#0000FF")
            .unwrap()
            .with_steps(0);
        assert!(result.is_err());

        let result = linear_gradient("#FF0000", "#0000FF")
            .unwrap()
            .with_width(0);
        assert!(result.is_err());
    }
}
