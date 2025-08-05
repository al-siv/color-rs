//! ColorSchemeCalculator implementation

use super::types::{ColorSchemeCalculator, ColorSchemeConfig};
use crate::error::Result;
use palette::Lab;

impl ColorSchemeCalculator {
    /// Create a new calculator with the given configuration
    pub const fn new(config: ColorSchemeConfig) -> Self {
        Self { config }
    }

    /// Create a calculator with default configuration
    pub const fn default() -> Self {
        Self::new(ColorSchemeConfig::DEFAULT)
    }

    /// Get the configuration
    pub const fn config(&self) -> ColorSchemeConfig {
        self.config
    }

    /// Calculate color schemes using the configured options
    ///
    /// This is equivalent to the original calculate method but uses
    /// immutable configuration instead of mutable builder state.
    pub fn calculate(&self, base_color: Lab) -> Result<crate::color_schemes::ColorSchemeResult> {
        // Create a traditional calculator using the builder pattern
        let mut builder = crate::color_schemes::ColorSchemeBuilder::new();

        let config = self.config();
        if config.preserve_relative_luminance {
            builder = builder.preserve_relative_luminance();
        }

        if config.preserve_lab_luminance {
            builder = builder.preserve_lab_luminance();
        }

        if let Some(target_rel_lum) = config.target_relative_luminance {
            builder = builder.target_relative_luminance(target_rel_lum);
        }

        if let Some(target_lab_lum) = config.target_lab_luminance {
            builder = builder.target_lab_luminance(target_lab_lum);
        }

        let traditional_calculator = builder.build();
        traditional_calculator.calculate(base_color)
    }
}
