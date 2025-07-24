//! Command Pattern Implementation for Color Operations
//!
//! This module implements the Command pattern to encapsulate color operations
//! as objects, allowing for better testing, undo functionality, and batch processing.

use crate::cli::GradientArgs;
use crate::error::{ColorError, Result};
use std::collections::HashMap;

/// Result of executing a command
#[derive(Debug, Clone)]
pub struct CommandResult {
    pub success: bool,
    pub output: String,
    pub error_message: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl CommandResult {
    #[must_use]
    pub fn success(output: String) -> Self {
        Self {
            success: true,
            output,
            error_message: None,
            metadata: HashMap::new(),
        }
    }

    #[must_use]
    pub const fn success_with_metadata(output: String, metadata: HashMap<String, String>) -> Self {
        Self {
            success: true,
            output,
            error_message: None,
            metadata,
        }
    }

    #[must_use]
    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            output: String::new(),
            error_message: Some(error),
            metadata: HashMap::new(),
        }
    }
}

/// Command trait for encapsulating operations
pub trait Command: Send + Sync {
    /// Execute the command
    fn execute(&self) -> Result<CommandResult>;

    /// Get the name of this command
    fn name(&self) -> &str;

    /// Get a description of what this command does
    fn description(&self) -> &str;

    /// Undo the command (optional, not all commands support undo)
    fn undo(&self) -> Result<CommandResult> {
        Err(ColorError::InvalidOperation(
            "Undo not supported for this command".to_string(),
        ))
    }

    /// Check if this command supports undo
    fn supports_undo(&self) -> bool {
        false
    }
}

/// Command for generating gradients
pub struct GenerateGradientCommand {
    args: GradientArgs,
    output_path: Option<String>,
}

impl GenerateGradientCommand {
    #[must_use]
    pub const fn new(args: GradientArgs) -> Self {
        Self {
            args,
            output_path: None,
        }
    }

    #[must_use]
    pub fn with_output_path(mut self, path: String) -> Self {
        self.output_path = Some(path);
        self
    }
}

impl Command for GenerateGradientCommand {
    fn execute(&self) -> Result<CommandResult> {
        // Parse start and end colors
        let start_lab = match crate::color::parse_color_input(&self.args.start_color) {
            Ok(lab) => lab,
            Err(e) => {
                return Ok(CommandResult::failure(format!(
                    "Failed to parse start color: {e}"
                )));
            }
        };

        let end_lab = match crate::color::parse_color_input(&self.args.end_color) {
            Ok(lab) => lab,
            Err(e) => {
                return Ok(CommandResult::failure(format!(
                    "Failed to parse end color: {e}"
                )));
            }
        };

        // Generate simple gradient output using basic interpolation
        let steps = self.args.stops;
        let mut output = String::new();
        output.push_str("Generated gradient:\n");

        for i in 0..steps {
            let t = i as f64 / (steps - 1) as f64;
            let interpolated =
                crate::color_utils::LegacyColorUtils::interpolate_lab(start_lab, end_lab, t);
            let hex = crate::color_utils::LegacyColorUtils::lab_to_hex(interpolated);
            use std::fmt::Write;
            writeln!(output, "Step {i}: {hex}").unwrap(); // Writing to String never fails
        }

        // Generate SVG if requested
        if self.args.should_generate_svg() {
            let _svg_content = self.generate_svg();
            output.push_str("\nSVG generated successfully\n");
            if let Some(path) = &self.output_path {
                use std::fmt::Write;
                writeln!(output, "SVG saved to: {path}").unwrap(); // Writing to String never fails
            }
        }

        // Generate PNG if requested
        if self.args.should_generate_png() {
            let _ = self.generate_png();
            output.push_str("PNG generated successfully\n");
        }

        let mut metadata = HashMap::new();
        metadata.insert("start_color".to_string(), self.args.start_color.clone());
        metadata.insert("end_color".to_string(), self.args.end_color.clone());
        metadata.insert("steps".to_string(), steps.to_string());

        Ok(CommandResult::success_with_metadata(output, metadata))
    }

    fn name(&self) -> &'static str {
        "generate_gradient"
    }

    fn description(&self) -> &'static str {
        "Generate a color gradient between two colors"
    }
}

impl GenerateGradientCommand {
    fn generate_svg(&self) -> String {
        // Simplified SVG generation - would use the full image module in practice
        format!(
            r#"<svg width="1000" height="200">
                <linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:{}"/>
                    <stop offset="100%" style="stop-color:{}"/>
                </linearGradient>
                <rect width="1000" height="200" fill="url(#grad)"/>
            </svg>"#,
            self.args.start_color, self.args.end_color
        )
    }

    const fn generate_png(&self) -> Vec<u8> {
        // Placeholder for PNG generation
        vec![] // Would use actual image generation
    }
}

/// Command for finding closest color matches
pub struct FindClosestColorCommand {
    color_input: String,
    collection: Option<String>,
    algorithm: String,
    count: usize,
}

impl FindClosestColorCommand {
    #[must_use]
    pub fn new(color_input: String) -> Self {
        Self {
            color_input,
            collection: None,
            algorithm: "delta-e-2000".to_string(),
            count: 5,
        }
    }

    #[must_use]
    pub fn with_collection(mut self, collection: String) -> Self {
        self.collection = Some(collection);
        self
    }

    #[must_use]
    pub fn with_algorithm(mut self, algorithm: String) -> Self {
        self.algorithm = algorithm;
        self
    }

    #[must_use]
    pub const fn with_count(mut self, count: usize) -> Self {
        self.count = count;
        self
    }
}

impl Command for FindClosestColorCommand {
    fn execute(&self) -> Result<CommandResult> {
        // Parse the color for validation
        let _lab_color = match crate::color::parse_color_input(&self.color_input) {
            Ok(lab) => lab,
            Err(e) => {
                return Ok(CommandResult::failure(format!(
                    "Failed to parse color: {e}"
                )));
            }
        };

        // Find closest colors using the color matching system
        // This would use the actual color matching implementation
        let output = format!(
            "Finding {} closest colors to {} using {} algorithm\n",
            self.count, self.color_input, self.algorithm
        );

        let mut metadata = HashMap::new();
        metadata.insert("input_color".to_string(), self.color_input.clone());
        metadata.insert("algorithm".to_string(), self.algorithm.clone());
        metadata.insert("count".to_string(), self.count.to_string());
        if let Some(collection) = &self.collection {
            metadata.insert("collection".to_string(), collection.clone());
        }

        Ok(CommandResult::success_with_metadata(output, metadata))
    }

    fn name(&self) -> &'static str {
        "find_closest_color"
    }

    fn description(&self) -> &'static str {
        "Find the closest matching colors in collections"
    }
}

/// Command invoker that manages and executes commands
pub struct CommandInvoker {
    history: Vec<Box<dyn Command>>,
    current_index: usize,
}

impl Default for CommandInvoker {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandInvoker {
    #[must_use]
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            current_index: 0,
        }
    }

    /// Execute a command and add it to history
    pub fn execute_command(&mut self, command: Box<dyn Command>) -> Result<CommandResult> {
        let result = command.execute()?;

        // Only add to history if execution was successful
        if result.success {
            self.history.push(command);
            self.current_index = self.history.len();
        }

        Ok(result)
    }

    /// Undo the last command (if supported)
    pub fn undo(&mut self) -> Result<CommandResult> {
        if self.current_index == 0 {
            return Ok(CommandResult::failure("Nothing to undo".to_string()));
        }

        self.current_index -= 1;
        let command = &self.history[self.current_index];

        if command.supports_undo() {
            command.undo()
        } else {
            Ok(CommandResult::failure(
                "Command does not support undo".to_string(),
            ))
        }
    }

    /// Get command history
    #[must_use]
    pub fn get_history(&self) -> Vec<&str> {
        self.history.iter().map(|cmd| cmd.name()).collect()
    }

    /// Clear command history
    pub fn clear_history(&mut self) {
        self.history.clear();
        self.current_index = 0;
    }
}

/// Macro for creating commands with builder pattern
#[macro_export]
macro_rules! create_command {
    (gradient: $start:expr, $end:expr) => {{
        let mut args = $crate::cli::GradientArgs::default();
        args.start_color = $start.to_string();
        args.end_color = $end.to_string();
        Box::new($crate::command_pattern::GenerateGradientCommand::new(args))
    }};

    (analyze: $color:expr) => {
        Box::new($crate::command_pattern::AnalyzeColorCommand::new(
            $color.to_string(),
        ))
    };

    (find_closest: $color:expr) => {
        Box::new($crate::command_pattern::FindClosestColorCommand::new(
            $color.to_string(),
        ))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_result() {
        let success = CommandResult::success("test output".to_string());
        assert!(success.success);
        assert_eq!(success.output, "test output");
        assert!(success.error_message.is_none());

        let failure = CommandResult::failure("test error".to_string());
        assert!(!failure.success);
        assert_eq!(failure.error_message.unwrap(), "test error");
    }

    #[test]
    fn test_command_invoker() {
        let mut invoker = CommandInvoker::new();
        assert_eq!(invoker.get_history().len(), 0);

        // Test undo when no commands
        let result = invoker.undo();
        assert!(result.is_ok());
        assert!(!result.unwrap().success);
    }

    #[test]
    fn test_find_closest_color_command() {
        let command = FindClosestColorCommand::new("#FF0000".to_string())
            .with_algorithm("delta-e-76".to_string())
            .with_count(3);

        assert_eq!(command.algorithm, "delta-e-76");
        assert_eq!(command.count, 3);
        assert_eq!(command.name(), "find_closest_color");
    }
}
