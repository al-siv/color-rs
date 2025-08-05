//! Gradient Configuration Convenience Functions
//!
//! This module provides convenient factory functions for creating
//! common gradient configurations.

use super::types::*;
use crate::cli::GradientArgs;
use crate::error::Result;

/// Create a simple linear gradient
///
/// # Errors
/// Returns `ColorError` if color validation fails
pub fn linear_gradient(start_color: &str, end_color: &str) -> Result<GradientConfig> {
    let colors = ColorPair::new(start_color, end_color)?;
    let easing = EasingConfig::linear();
    GradientConfig::new(colors, easing)
}

/// Create an ease-in-out gradient
///
/// # Errors
/// Returns `ColorError` if color validation fails
pub fn smooth_gradient(start_color: &str, end_color: &str) -> Result<GradientConfig> {
    let colors = ColorPair::new(start_color, end_color)?;
    let easing = EasingConfig::ease_in_out();
    GradientConfig::new(colors, easing)
}

/// Create a gradient with custom position range
///
/// # Errors
/// Returns `ColorError` if validation fails
pub fn positioned_gradient(
    start_color: &str,
    end_color: &str,
    start_pos: u8,
    end_pos: u8,
) -> Result<GradientConfig> {
    let colors = ColorPair::new(start_color, end_color)?;
    let easing = EasingConfig::default_config();
    let position_range = PositionRange::new(start_pos, end_pos)?;

    GradientConfig::new(colors, easing)?.with_position_range(position_range)
}

/// Generate gradient using modern functional approach (Assignment 6 Milestone 6.1)
pub fn generate_gradient(args: GradientArgs) -> Result<()> {
    // Create gradient configuration from CLI arguments
    let config = GradientConfig::from_gradient_args(args)?;

    // Convert config to GradientArgs for the actual generation
    // This uses the validated and normalized configuration
    let gradient_args = config.to_gradient_args();

    // Delegate to the proven gradient generation implementation
    // This approach maintains backward compatibility while using
    // the new functional configuration system for validation and construction
    crate::gradient::generate_gradient(gradient_args)
}
