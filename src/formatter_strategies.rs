//! Formatting Strategy Pattern Implementation
//!
//! This module implements the Strategy pattern for different color formatting approaches.
//! It allows for pluggable formatting strategies while maintaining a consistent interface.

use crate::error::Result;
use palette::Lab;
use std::collections::HashMap;

/// Strategy trait for formatting color information
pub trait ColorFormattingStrategy: Send + Sync {
    /// Format color information according to this strategy
    fn format_color(&self, lab_color: Lab, input: &str, name: &str) -> Result<String>;

    /// Get the name of this formatting strategy
    fn strategy_name(&self) -> &str;

    /// Get a description of what this strategy provides
    fn description(&self) -> &str;
}

/// Comprehensive formatting strategy (current default behavior)
pub struct ComprehensiveFormattingStrategy;

impl ColorFormattingStrategy for ComprehensiveFormattingStrategy {
    fn format_color(&self, lab_color: Lab, input: &str, name: &str) -> Result<String> {
        // This delegates to the existing comprehensive formatter
        crate::color_formatter::ColorFormatter::format_comprehensive_report(lab_color, input, name)
    }

    fn strategy_name(&self) -> &str {
        "comprehensive"
    }

    fn description(&self) -> &str {
        "Complete color analysis with all format conversions and collection matches"
    }
}

/// Minimal formatting strategy (just basic info)
pub struct MinimalFormattingStrategy;

impl ColorFormattingStrategy for MinimalFormattingStrategy {
    fn format_color(&self, lab_color: Lab, input: &str, _name: &str) -> Result<String> {
        use palette::{IntoColor, Srgb};
        use std::fmt::Write;

        let mut output = String::new();

        // Convert to RGB for display
        let srgb: Srgb = lab_color.into_color();
        let r = (srgb.red * 255.0).round() as u8;
        let g = (srgb.green * 255.0).round() as u8;
        let b = (srgb.blue * 255.0).round() as u8;

        writeln!(output, "Input: {}", input)?;
        writeln!(output, "RGB: ({}, {}, {})", r, g, b)?;
        writeln!(output, "HEX: #{:02X}{:02X}{:02X}", r, g, b)?;

        Ok(output)
    }

    fn strategy_name(&self) -> &str {
        "minimal"
    }

    fn description(&self) -> &str {
        "Basic color information with RGB and HEX values only"
    }
}

/// JSON formatting strategy for machine-readable output
pub struct JsonFormattingStrategy;

impl ColorFormattingStrategy for JsonFormattingStrategy {
    fn format_color(&self, lab_color: Lab, input: &str, name: &str) -> Result<String> {
        use palette::{Hsl, IntoColor, Srgb};
        use std::fmt::Write;

        let mut output = String::new();

        // Convert to different color spaces
        let srgb: Srgb = lab_color.into_color();
        let hsl: Hsl = lab_color.into_color();

        let r = (srgb.red * 255.0).round() as u8;
        let g = (srgb.green * 255.0).round() as u8;
        let b = (srgb.blue * 255.0).round() as u8;

        // Simple JSON output (could be enhanced with serde)
        writeln!(output, "{{")?;
        writeln!(output, "  \"input\": \"{}\",", input)?;
        writeln!(output, "  \"name\": \"{}\",", name)?;
        writeln!(output, "  \"rgb\": [{}, {}, {}],", r, g, b)?;
        writeln!(output, "  \"hex\": \"#{:02X}{:02X}{:02X}\",", r, g, b)?;
        writeln!(
            output,
            "  \"hsl\": [{:.1}, {:.1}, {:.1}],",
            hsl.hue.into_positive_degrees(),
            hsl.saturation * 100.0,
            hsl.lightness * 100.0
        )?;
        writeln!(
            output,
            "  \"lab\": [{:.2}, {:.2}, {:.2}]",
            lab_color.l, lab_color.a, lab_color.b
        )?;
        writeln!(output, "}}")?;

        Ok(output)
    }

    fn strategy_name(&self) -> &str {
        "json"
    }

    fn description(&self) -> &str {
        "Machine-readable JSON format output"
    }
}

/// Factory for creating formatting strategies
pub struct FormattingStrategyFactory;

impl FormattingStrategyFactory {
    /// Create a formatting strategy by name
    pub fn create_strategy(name: &str) -> Box<dyn ColorFormattingStrategy> {
        match name.to_lowercase().as_str() {
            "minimal" | "simple" => Box::new(MinimalFormattingStrategy),
            "json" => Box::new(JsonFormattingStrategy),
            "comprehensive" | "full" | _ => Box::new(ComprehensiveFormattingStrategy),
        }
    }

    /// Get all available strategy names
    pub fn available_strategies() -> Vec<&'static str> {
        vec!["comprehensive", "minimal", "json"]
    }

    /// Get strategy descriptions
    pub fn strategy_descriptions() -> HashMap<&'static str, &'static str> {
        let mut map = HashMap::new();
        map.insert(
            "comprehensive",
            "Complete color analysis with all format conversions and collection matches",
        );
        map.insert(
            "minimal",
            "Basic color information with RGB and HEX values only",
        );
        map.insert("json", "Machine-readable JSON format output");
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Lab;

    #[test]
    fn test_comprehensive_strategy() {
        let strategy = ComprehensiveFormattingStrategy;
        let lab = Lab::new(50.0, 0.0, 0.0);
        let result = strategy.format_color(lab, "#808080", "gray");
        assert!(result.is_ok());
        assert_eq!(strategy.strategy_name(), "comprehensive");
    }

    #[test]
    fn test_minimal_strategy() {
        let strategy = MinimalFormattingStrategy;
        let lab = Lab::new(50.0, 0.0, 0.0);
        let result = strategy.format_color(lab, "#808080", "gray");
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("RGB:"));
        assert!(output.contains("HEX:"));
        assert_eq!(strategy.strategy_name(), "minimal");
    }

    #[test]
    fn test_json_strategy() {
        let strategy = JsonFormattingStrategy;
        let lab = Lab::new(50.0, 0.0, 0.0);
        let result = strategy.format_color(lab, "#808080", "gray");
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("\"rgb\":"));
        assert!(output.contains("\"hex\":"));
        assert!(output.contains("\"lab\":"));
        assert_eq!(strategy.strategy_name(), "json");
    }

    #[test]
    fn test_factory() {
        let strategy = FormattingStrategyFactory::create_strategy("minimal");
        assert_eq!(strategy.strategy_name(), "minimal");

        let strategy = FormattingStrategyFactory::create_strategy("json");
        assert_eq!(strategy.strategy_name(), "json");

        let strategy = FormattingStrategyFactory::create_strategy("unknown");
        assert_eq!(strategy.strategy_name(), "comprehensive");
    }

    #[test]
    fn test_available_strategies() {
        let strategies = FormattingStrategyFactory::available_strategies();
        assert!(strategies.contains(&"comprehensive"));
        assert!(strategies.contains(&"minimal"));
        assert!(strategies.contains(&"json"));
    }
}
