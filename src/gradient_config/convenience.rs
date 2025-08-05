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
    
    GradientConfig::new(colors, easing)?
        .with_position_range(position_range)
}

/// Generate gradient using modern approach (Milestone 2.1b integration)
pub fn generate_gradient(args: GradientArgs) -> Result<()> {
    // Create gradient configuration from CLI arguments
    let config = GradientConfig::from_gradient_args(args)?;
    
    // TODO: Implement actual gradient generation using the gradient config
    // For now, delegate to the old system to maintain functionality
    // This will be replaced with modern implementation later
    
    // Convert back to GradientArgs and use existing generation
    let gradient_args = config.to_gradient_args();
    crate::gradient::generate_gradient(gradient_args)
}
