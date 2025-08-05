//! Convenience functions for command creation and simplified execution
//!
//! This module provides functional equivalents of command creation patterns,
//! using smart constructors and preset configurations.

use super::types::{CommandType, ExecutionResult};
use super::execution::{execute_command_simple, execute_command_with_validation, execute_command_enhanced};
use crate::error::Result;

// Convenience functions - functional equivalents of command creation

/// Create gradient generation command
pub fn create_gradient_command(start_color: String, end_color: String, stops: usize) -> CommandType {
    use crate::cli::GradientArgs;
    
    // Create GradientArgs manually with actual fields
    let args = GradientArgs {
        start_color,
        end_color,
        start_position: 0,
        end_position: 100,
        ease_in: 0.65,
        ease_out: 0.35,
        svg: None,
        png: None,
        no_legend: false,
        width: 1000,
        step: None,
        stops,
        stops_simple: false,
        output_format: None,
        output_file: None,
        func_filter: None,
    };
    
    CommandType::GenerateGradient {
        args,
        output_path: None,
    }
}

/// Create color analysis command
pub fn create_analyze_command(color_input: String, include_schemes: bool) -> CommandType {
    CommandType::AnalyzeColor {
        color_input,
        include_schemes,
        output_format: "text".to_string(),
    }
}

/// Create color matching command
pub fn create_find_closest_command(color_input: String, count: usize) -> CommandType {
    CommandType::FindClosestColor {
        color_input,
        collection: None,
        algorithm: "delta-e-2000".to_string(),
        count,
    }
}

/// Create color conversion command
pub fn create_convert_command(color_input: String, target_format: String) -> CommandType {
    CommandType::ConvertColor {
        color_input,
        target_format,
        precision: 2,
    }
}

// Simplified execution functions

/// Execute command with default context (no hooks)
pub fn execute_simple(command_type: CommandType) -> Result<ExecutionResult> {
    execute_command_simple(command_type)
}

/// Execute command with validation hooks
pub fn execute_with_validation(command_type: CommandType) -> Result<ExecutionResult> {
    execute_command_with_validation(command_type)
}

/// Execute command with full hooks and formatting
pub fn execute_enhanced(command_type: CommandType) -> Result<ExecutionResult> {
    execute_command_enhanced(command_type)
}
