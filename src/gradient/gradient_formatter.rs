//! Gradient Formatting System
//!
//! This module provides gradient output formatting in various formats
//! using functional composition and callback systems.

use super::calculator::GradientValue;
use crate::error::Result;
use tabled::{settings::Style, Table};

/// Gradient output format using enum dispatch for zero-cost abstractions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GradientFormat {
    /// Table format with ASCII art
    Table,
    /// CSS gradient format
    Css,
    /// JSON format
    Json,
    /// Custom format with provided formatter function
    Custom { name: String },
}

impl GradientFormat {
    /// Format gradient values using functional approach
    pub fn format_gradient(&self, values: &[GradientValue]) -> Result<String> {
        match self {
            Self::Table => format_as_table(values),
            Self::Css => format_as_css(values),
            Self::Json => format_as_json(values),
            Self::Custom { name } => format_as_custom(values, name),
        }
    }

    /// Get format name
    pub fn name(&self) -> &str {
        match self {
            Self::Table => "Table",
            Self::Css => "CSS",
            Self::Json => "JSON",
            Self::Custom { name } => name,
        }
    }
}

/// Pure function for table formatting
fn format_as_table(values: &[GradientValue]) -> Result<String> {
    if values.is_empty() {
        return Ok("No gradient values to display".to_string());
    }

    let table = Table::new(values).with(Style::rounded()).to_string();

    Ok(table)
}

/// Pure function for CSS formatting
fn format_as_css(values: &[GradientValue]) -> Result<String> {
    if values.is_empty() {
        return Ok("".to_string());
    }

    let css_stops: Vec<String> = values
        .iter()
        .map(|value| format!("{} {}", value.hex, value.position))
        .collect();

    Ok(format!("linear-gradient({})", css_stops.join(", ")))
}

/// Pure function for JSON formatting
fn format_as_json(values: &[GradientValue]) -> Result<String> {
    if values.is_empty() {
        return Ok("[]".to_string());
    }

    let json_values: Vec<String> = values
        .iter()
        .map(|value| {
            format!(
                r#"{{"position": "{}", "hex": "{}", "rgb": "{}", "wcag_luminance": "{}"}}"#,
                value.position, value.hex, value.rgb, value.wcag_luminance
            )
        })
        .collect();

    Ok(format!("[{}]", json_values.join(", ")))
}

/// Pure function for custom formatting
fn format_as_custom(values: &[GradientValue], format_name: &str) -> Result<String> {
    // For now, fall back to table format for unknown custom formats
    // In a real implementation, this could dispatch to registered formatters
    format_as_table(values).map(|output| format!("Custom format '{format_name}' output:\n{output}"))
}

/// Functional event callback system to replace Observer pattern
pub type GradientGeneratedCallback = Box<dyn Fn(&[GradientValue]) + Send + Sync>;
pub type GradientFormattedCallback = Box<dyn Fn(&str) + Send + Sync>;
pub type ErrorCallback = Box<dyn Fn(&str) + Send + Sync>;

/// Functional event callbacks container
#[derive(Default)]
pub struct EventCallbacks {
    gradient_generated: Vec<GradientGeneratedCallback>,
    output_formatted: Vec<GradientFormattedCallback>,
    errors: Vec<ErrorCallback>,
}

impl EventCallbacks {
    /// Create new empty callbacks container
    pub fn new() -> Self {
        Self::default()
    }

    /// Add gradient generated callback
    pub fn on_gradient_generated<F>(mut self, callback: F) -> Self
    where
        F: Fn(&[GradientValue]) + Send + Sync + 'static,
    {
        self.gradient_generated.push(Box::new(callback));
        self
    }

    /// Add output formatted callback
    pub fn on_output_formatted<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.output_formatted.push(Box::new(callback));
        self
    }

    /// Add error callback
    pub fn on_error<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.errors.push(Box::new(callback));
        self
    }

    /// Notify gradient generated
    pub fn notify_gradient_generated(&self, values: &[GradientValue]) {
        for callback in &self.gradient_generated {
            callback(values);
        }
    }

    /// Notify output formatted
    pub fn notify_output_formatted(&self, output: &str) {
        for callback in &self.output_formatted {
            callback(output);
        }
    }

    /// Notify error
    pub fn notify_error(&self, error: &str) {
        for callback in &self.errors {
            callback(error);
        }
    }
}

/// Pure functional formatting entrypoint (replaces removed GradientFormatter struct)
pub fn format_gradient_with_callbacks(
    format: &GradientFormat,
    values: &[GradientValue],
    callbacks: &EventCallbacks,
) -> Result<String> {
    callbacks.notify_gradient_generated(values);
    match format.format_gradient(values) {
        Ok(output) => {
            callbacks.notify_output_formatted(&output);
            Ok(output)
        }
        Err(e) => {
            let msg = format!("Formatting error: {e}");
            callbacks.notify_error(&msg);
            Err(e)
        }
    }
}

/// Convenience functions for common callback patterns
pub mod callbacks {
    use super::*;

    /// Console logging callback for gradient generation
    pub fn console_gradient_logger() -> impl Fn(&[GradientValue]) + Send + Sync {
        |values: &[GradientValue]| {
            eprintln!("Generated {} gradient values", values.len());
        }
    }

    /// Console logging callback for output formatting
    pub fn console_output_logger() -> impl Fn(&str) + Send + Sync {
        |output: &str| {
            eprintln!("Formatted output ({} characters)", output.len());
        }
    }

    /// Console logging callback for errors
    pub fn console_error_logger() -> impl Fn(&str) + Send + Sync {
        |error: &str| {
            eprintln!("Output error: {error}");
        }
    }

    /// File writing callback for output
    pub fn file_writer(path: String) -> impl Fn(&str) + Send + Sync {
        move |output: &str| {
            if let Err(e) = std::fs::write(&path, output) {
                eprintln!("Failed to write to {path}: {e}");
            }
        }
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
    fn test_table_formatting() {
        let values = create_test_values();
        let result = format_as_table(&values).unwrap();

        assert!(result.contains("FF0000"));
        assert!(result.contains("0000FF"));
        assert!(result.contains("0%"));
        assert!(result.contains("100%"));
    }

    #[test]
    fn test_css_formatting() {
        let values = create_test_values();
        let result = format_as_css(&values).unwrap();

        assert!(result.starts_with("linear-gradient("));
        assert!(result.contains("#FF0000 0%"));
        assert!(result.contains("#0000FF 100%"));
    }

    #[test]
    fn test_json_formatting() {
        let values = create_test_values();
        let result = format_as_json(&values).unwrap();

        assert!(result.starts_with('['));
        assert!(result.ends_with(']'));
        assert!(result.contains("FF0000"));
        assert!(result.contains("0000FF"));
    }

    #[test]
    fn test_empty_values() {
        let values = vec![];

        assert_eq!(
            format_as_table(&values).unwrap(),
            "No gradient values to display"
        );
        assert_eq!(format_as_css(&values).unwrap(), "");
        assert_eq!(format_as_json(&values).unwrap(), "[]");
    }

    #[test]
    fn test_gradient_formatter() {
        let values = create_test_values();
        let callbacks = EventCallbacks::new();
        let result = format_gradient_with_callbacks(&GradientFormat::Table, &values, &callbacks).unwrap();
        assert!(result.contains("FF0000"));
    }

    #[test]
    fn test_event_callbacks() {
        use std::sync::{Arc, Mutex};

        let gradient_called = Arc::new(Mutex::new(false));
        let output_called = Arc::new(Mutex::new(false));

        let gradient_flag = gradient_called.clone();
        let output_flag = output_called.clone();

        let callbacks = EventCallbacks::new()
            .on_gradient_generated(move |_| {
                *gradient_flag.lock().unwrap() = true;
            })
            .on_output_formatted(move |_| {
                *output_flag.lock().unwrap() = true;
            });

    let values = create_test_values();
    let _result = format_gradient_with_callbacks(&GradientFormat::Table, &values, &callbacks).unwrap();

        assert!(*gradient_called.lock().unwrap());
        assert!(*output_called.lock().unwrap());
    }

    #[test]
    fn test_format_names() {
        assert_eq!(GradientFormat::Table.name(), "Table");
        assert_eq!(GradientFormat::Css.name(), "CSS");
        assert_eq!(GradientFormat::Json.name(), "JSON");
        assert_eq!(GradientFormat::Custom { name: "test".to_string() }.name(), "test");
    }

    #[test]
    fn test_functional_formatting_api() {
        let values = create_test_values();
        let callbacks = EventCallbacks::new();
        let out = format_gradient_with_callbacks(&GradientFormat::Json, &values, &callbacks).unwrap();
        assert!(out.contains("FF0000"));
    }
}
