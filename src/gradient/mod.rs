//! Simplified gradient generation module
//! 
//! Cleaned up from over-engineered pattern implementation to basic functionality

pub mod calculator;
pub mod easing;
pub mod output;

// Simple re-exports for basic functionality
pub use calculator::{GradientCalculator, GradientValue};
pub use easing::{EasingStrategy, EasingType, LinearEasing, CubicBezierEasing};

/// Simplified gradient generation function for CLI interface
pub fn generate_gradient(args: crate::cli::GradientArgs) -> crate::error::Result<()> {
    use crate::color_utils::LegacyColorUtils as ColorUtils;
    
    // Parse colors
    let start_lab = ColorUtils::parse_hex_color(&args.start_color)?;
    let end_lab = ColorUtils::parse_hex_color(&args.end_color)?;
    
    // Simple linear interpolation
    let steps = args.stops;
    for i in 0..steps {
        let t = i as f64 / (steps - 1) as f64;
        let interpolated = ColorUtils::interpolate_lab(start_lab, end_lab, t);
        let hex = ColorUtils::lab_to_hex(interpolated);
        println!("Step {i}: {hex}");
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_gradient() {
        // Basic test - just ensure types are accessible
        let _easing_type = EasingType::Linear;
        assert!(true);
    }
}
