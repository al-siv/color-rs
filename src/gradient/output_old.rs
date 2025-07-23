//! Gradient output formatting

use super::calculator::GradientValue;
use crate::format_utils::FormatUtils;
use crate::error::{ColorError, Result};
use palette::{Srgb, IntoColor};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Strategy interface for different output formats
pub trait OutputFormatter {
    fn format_gradient(&self, values: &[GradientValue]) -> crate::error::Result<String>;
}

/// Table output formatter
pub struct TableFormatter;

impl OutputFormatter for TableFormatter {
    fn format_gradient(&self, values: &[GradientValue]) -> crate::error::Result<String> {
        if values.is_empty() {
            return Ok("No gradient values to display".to_string());
        }

        let table = Table::new(values)
            .with(Style::rounded())
            .to_string();
        
        Ok(table)
    }
}

/// CSS gradient formatter
pub struct CssFormatter;

impl OutputFormatter for CssFormatter {
    fn format_gradient(&self, values: &[GradientValue]) -> crate::error::Result<String> {
        if values.is_empty() {
            return Ok("".to_string());
        }

        let css_stops: Vec<String> = values
            .iter()
            .map(|value| format!("{} {}", value.hex, value.position))
            .collect();

        Ok(format!("linear-gradient({})", css_stops.join(", ")))
    }
}

/// JSON output formatter
pub struct JsonFormatter;

impl OutputFormatter for JsonFormatter {
    fn format_gradient(&self, values: &[GradientValue]) -> crate::error::Result<String> {
        // Create JSON representation
        let json_values: Vec<serde_json::Value> = values
            .iter()
            .map(|value| {
                serde_json::json!({
                    "position": value.position,
                    "hex": value.hex,
                    "rgb": value.rgb,
                    "wcag_luminance": value.wcag_luminance
                })
            })
            .collect();

        Ok(serde_json::to_string_pretty(&json_values)?)
    }
}

/// Observer trait for gradient output events
pub trait GradientOutputObserver {
    fn on_gradient_generated(&self, values: &[GradientValue]);
    fn on_output_formatted(&self, output: &str);
    fn on_error(&self, error: &str);
}

/// Console observer for logging output events
pub struct ConsoleObserver;

impl GradientOutputObserver for ConsoleObserver {
    fn on_gradient_generated(&self, values: &[GradientValue]) {
        eprintln!("Generated {} gradient values", values.len());
    }

    fn on_output_formatted(&self, output: &str) {
        eprintln!("Formatted output ({} characters)", output.len());
    }

    fn on_error(&self, error: &str) {
        eprintln!("Output error: {}", error);
    }
}

/// Main gradient output manager using Strategy and Observer patterns
pub struct GradientOutputManager {
    formatter: Box<dyn OutputFormatter>,
    observers: Vec<Box<dyn GradientOutputObserver>>,
}

impl GradientOutputManager {
    /// Create output manager with table formatter
    pub fn with_table_format() -> Self {
        Self {
            formatter: Box::new(TableFormatter),
            observers: Vec::new(),
        }
    }

    /// Create output manager with CSS formatter
    pub fn with_css_format() -> Self {
        Self {
            formatter: Box::new(CssFormatter),
            observers: Vec::new(),
        }
    }

    /// Create output manager with JSON formatter
    pub fn with_json_format() -> Self {
        Self {
            formatter: Box::new(JsonFormatter),
            observers: Vec::new(),
        }
    }

    /// Add observer
    pub fn add_observer(&mut self, observer: Box<dyn GradientOutputObserver>) {
        self.observers.push(observer);
    }

    /// Notify observers of gradient generation
    fn notify_gradient_generated(&self, values: &[GradientValue]) {
        for observer in &self.observers {
            observer.on_gradient_generated(values);
        }
    }

    /// Notify observers of output formatting
    fn notify_output_formatted(&self, output: &str) {
        for observer in &self.observers {
            observer.on_output_formatted(output);
        }
    }

    /// Notify observers of errors
    fn notify_error(&self, error: &str) {
        for observer in &self.observers {
            observer.on_error(error);
        }
    }

    /// Format and output gradient values
    pub fn output_gradient(&self, values: &[GradientValue]) -> crate::error::Result<String> {
        // Notify observers
        self.notify_gradient_generated(values);

        // Format using strategy
        match self.formatter.format_gradient(values) {
            Ok(output) => {
                self.notify_output_formatted(&output);
                Ok(output)
            }
            Err(error) => {
                let error_msg = format!("Failed to format gradient: {}", error);
                self.notify_error(&error_msg);
                Err(error)
            }
        }
    }
}

/// Gradient output utility functions
pub struct GradientOutputUtils;

impl GradientOutputUtils {
    /// Display gradient as table with color highlighting
    pub fn display_gradient_table(values: &[GradientValue]) -> crate::error::Result<()> {
        if values.is_empty() {
            println!("No gradient values to display");
            return Ok(());
        }

        let manager = GradientOutputManager::with_table_format();
        let output = manager.output_gradient(values)?;
        println!("{}", output);
        Ok(())
    }

    /// Generate CSS linear gradient string
    pub fn generate_css_gradient(values: &[GradientValue]) -> crate::error::Result<String> {
        let manager = GradientOutputManager::with_css_format();
        manager.output_gradient(values)
    }

    /// Save gradient as JSON
    pub fn save_gradient_as_json(
        values: &[GradientValue],
        filename: &str,
    ) -> crate::error::Result<()> {
        let manager = GradientOutputManager::with_json_format();
        let json_output = manager.output_gradient(values)?;
        
        std::fs::write(filename, json_output)?;
        Ok(())
    }

    /// Print gradient with color previews in terminal
    pub fn print_gradient_with_colors(values: &[GradientValue]) -> crate::error::Result<()> {
        for value in values {
            // Parse hex color for RGB values
            if let Some(rgb) = FormatUtils::parse_hex_color(&value.hex) {
                // Use ANSI escape codes for background color
                let colored_output = OutputUtils::colorize_background(
                    &format!(" {} ", value.hex),
                    rgb.0,
                    rgb.1,
                    rgb.2,
                );
                println!(
                    "{} {} {} WCAG: {}",
                    colored_output, value.position, value.rgb, value.wcag_luminance
                );
            } else {
                println!(
                    "{} {} {} WCAG: {}",
                    value.hex, value.position, value.rgb, value.wcag_luminance
                );
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_values() -> Vec<GradientValue> {
        vec![
            GradientValue {
                position: "0%".to_string(),
                hex: "#FF0000".to_string(),
                rgb: "rgb(255, 0, 0)".to_string(),
                wcag_luminance: "0.213".to_string(),
            },
            GradientValue {
                position: "100%".to_string(),
                hex: "#0000FF".to_string(),
                rgb: "rgb(0, 0, 255)".to_string(),
                wcag_luminance: "0.072".to_string(),
            },
        ]
    }

    #[test]
    fn test_table_formatter() {
        let formatter = TableFormatter;
        let values = create_test_values();
        let result = formatter.format_gradient(&values);
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Position"));
        assert!(output.contains("Hex"));
        assert!(output.contains("#FF0000"));
    }

    #[test]
    fn test_css_formatter() {
        let formatter = CssFormatter;
        let values = create_test_values();
        let result = formatter.format_gradient(&values);
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("linear-gradient"));
        assert!(output.contains("#FF0000 0%"));
        assert!(output.contains("#0000FF 100%"));
    }

    #[test]
    fn test_json_formatter() {
        let formatter = JsonFormatter;
        let values = create_test_values();
        let result = formatter.format_gradient(&values);
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("position"));
        assert!(output.contains("hex"));
        assert!(output.contains("#FF0000"));
    }

    #[test]
    fn test_output_manager() {
        let manager = GradientOutputManager::with_table_format();
        let values = create_test_values();
        let result = manager.output_gradient(&values);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_css_gradient_generation() {
        let values = create_test_values();
        let result = GradientOutputUtils::generate_css_gradient(&values);
        
        assert!(result.is_ok());
        let css = result.unwrap();
        assert!(css.starts_with("linear-gradient"));
    }
}
