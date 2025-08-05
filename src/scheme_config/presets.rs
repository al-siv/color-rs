//! Preset factory functions for common color scheme configurations

use super::types::{ColorSchemeCalculator, ColorSchemeConfig, ConfigError};

/// Standard color scheme calculation with no special options
pub fn standard() -> ColorSchemeCalculator {
    ColorSchemeCalculator::default()
}

/// Color scheme calculation with relative luminance preservation
pub fn with_relative_luminance_preservation() -> ColorSchemeCalculator {
    ColorSchemeCalculator::new(ColorSchemeConfig::with_relative_luminance_preservation())
}

/// Color scheme calculation with lab luminance preservation
pub fn with_lab_luminance_preservation() -> ColorSchemeCalculator {
    ColorSchemeCalculator::new(ColorSchemeConfig::with_lab_luminance_preservation())
}

/// Color scheme calculation with target relative luminance
pub fn with_target_relative_luminance(luminance: f64) -> std::result::Result<ColorSchemeCalculator, ConfigError> {
    Ok(ColorSchemeCalculator::new(
        ColorSchemeConfig::with_target_relative_luminance(luminance)?
    ))
}

/// Color scheme calculation with target lab luminance
pub fn with_target_lab_luminance(luminance: f64) -> std::result::Result<ColorSchemeCalculator, ConfigError> {
    Ok(ColorSchemeCalculator::new(
        ColorSchemeConfig::with_target_lab_luminance(luminance)?
    ))
}

/// Builder-like configuration composition for complex configurations
/// 
/// This function demonstrates how configuration composition can replace
/// traditional builder patterns with compile-time safety.
pub fn complex_config() -> std::result::Result<ColorSchemeCalculator, ConfigError> {
    let config = ColorSchemeConfig::default()
        .preserve_relative_luminance()?
        .set_target_relative_luminance(0.5)?;
    
    Ok(ColorSchemeCalculator::new(config))
}
