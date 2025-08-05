//! Core command execution logic and orchestration
//!
//! This module provides the main execution engine for command processing,
//! using functional composition and pattern matching instead of virtual dispatch.

use crate::error::{ColorError, Result};
use super::types::{CommandType, ExecutionContext, ExecutionResult, PreHookStep, PostHookStep};
use super::commands::{
    execute_generate_gradient, execute_find_closest_color,
    execute_analyze_color, execute_convert_color
};

/// Main functional command execution - replaces Command trait methods
pub fn execute_command(context: &ExecutionContext) -> Result<ExecutionResult> {
    let start_time = std::time::Instant::now();

    // Apply pre-execution hooks
    for hook in &context.pre_hooks {
        if let Err(e) = apply_pre_hook(hook, &context.command_type) {
            return Ok(ExecutionResult::failure(format!("Pre-hook failed: {}", e)));
        }
    }

    // Execute command based on type using pattern matching (not virtual dispatch)
    let mut result = match &context.command_type {
        CommandType::GenerateGradient { args, output_path } => {
            execute_generate_gradient(args, output_path.as_deref())
        }
        CommandType::FindClosestColor { color_input, collection, algorithm, count } => {
            execute_find_closest_color(color_input, collection.as_deref(), algorithm, *count)
        }
        CommandType::AnalyzeColor { color_input, include_schemes, output_format } => {
            execute_analyze_color(color_input, *include_schemes, output_format)
        }
        CommandType::ConvertColor { color_input, target_format, precision } => {
            execute_convert_color(color_input, target_format, *precision)
        }
    }?;

    // Add context metadata
    result.metadata.extend(context.metadata.clone());

    // Apply post-execution hooks
    for hook in &context.post_hooks {
        result = apply_post_hook(hook, result);
    }

    // Update execution time
    let execution_time = start_time.elapsed().as_millis();
    result = result.with_execution_time(execution_time);

    Ok(result)
}

/// Get command name - pure function instead of trait method
pub fn get_command_name(command_type: &CommandType) -> &'static str {
    match command_type {
        CommandType::GenerateGradient { .. } => "generate_gradient",
        CommandType::FindClosestColor { .. } => "find_closest_color",
        CommandType::AnalyzeColor { .. } => "analyze_color",
        CommandType::ConvertColor { .. } => "convert_color",
    }
}

/// Get command description - pure function instead of trait method
pub fn get_command_description(command_type: &CommandType) -> &'static str {
    match command_type {
        CommandType::GenerateGradient { .. } => "Generate a color gradient between two colors",
        CommandType::FindClosestColor { .. } => "Find the closest matching colors in collections",
        CommandType::AnalyzeColor { .. } => "Analyze color properties and conversion options",
        CommandType::ConvertColor { .. } => "Convert color between different formats",
    }
}

/// Check if command supports undo - pure function with pattern matching
pub const fn supports_undo(command_type: &CommandType) -> bool {
    match command_type {
        // All commands are either file generation or read-only operations
        CommandType::GenerateGradient { .. } |   // File generation can't be undone easily
        CommandType::FindClosestColor { .. } |   // Read-only operation
        CommandType::AnalyzeColor { .. } |       // Read-only operation
        CommandType::ConvertColor { .. } => false, // Pure transformation
    }
}

/// Execute command with default context (no hooks)
/// # Errors
/// Returns error if command execution fails
pub fn execute_command_simple(command_type: CommandType) -> Result<ExecutionResult> {
    let context = ExecutionContext::new(command_type);
    execute_command(&context)
}

/// Execute command with validation hooks
/// # Errors
/// Returns error if command execution or validation fails
pub fn execute_command_with_validation(command_type: CommandType) -> Result<ExecutionResult> {
    let context = ExecutionContext::new(command_type)
        .with_pre_hook(PreHookStep::ValidateParameters)
        .with_pre_hook(PreHookStep::LogStart)
        .with_post_hook(PostHookStep::LogCompletion);
    
    execute_command(&context)
}

/// Execute command with full hooks and formatting
/// # Errors
/// Returns error if command execution fails
pub fn execute_command_enhanced(command_type: CommandType) -> Result<ExecutionResult> {
    let context = ExecutionContext::new(command_type)
        .with_pre_hook(PreHookStep::ValidateParameters)
        .with_pre_hook(PreHookStep::LogStart)
        .with_pre_hook(PreHookStep::CheckPrerequisites)
        .with_post_hook(PostHookStep::FormatOutput)
        .with_post_hook(PostHookStep::LogCompletion)
        .with_post_hook(PostHookStep::SaveOutput);
    
    execute_command(&context)
}

// Hook execution functions using function composition

fn apply_pre_hook(hook: &PreHookStep, command_type: &CommandType) -> Result<()> {
    match hook {
        PreHookStep::ValidateParameters => validate_command_parameters(command_type),
        PreHookStep::LogStart => {
            eprintln!("Starting command: {}", get_command_name(command_type));
            Ok(())
        }
        PreHookStep::CheckPrerequisites => check_command_prerequisites(command_type),
        PreHookStep::Custom(func) => func(command_type),
    }
}

fn apply_post_hook(hook: &PostHookStep, result: ExecutionResult) -> ExecutionResult {
    match hook {
        PostHookStep::FormatOutput => format_command_output(result),
        PostHookStep::LogCompletion => {
            eprintln!("Command completed in {}ms", result.execution_time_ms);
            result
        }
        PostHookStep::SaveOutput => save_command_output(result),
        PostHookStep::Custom(func) => func(&result),
    }
}

fn validate_command_parameters(command_type: &CommandType) -> Result<()> {
    match command_type {
        CommandType::GenerateGradient { args, .. } => {
            if args.start_color.is_empty() || args.end_color.is_empty() {
                return Err(ColorError::InvalidArguments("Start and end colors required".to_string()));
            }
            if args.stops < 2 {
                return Err(ColorError::InvalidArguments("At least 2 gradient steps required".to_string()));
            }
        }
        CommandType::FindClosestColor { color_input, count, .. } => {
            if color_input.is_empty() {
                return Err(ColorError::InvalidArguments("Color input required".to_string()));
            }
            if *count == 0 {
                return Err(ColorError::InvalidArguments("Count must be greater than 0".to_string()));
            }
        }
        CommandType::AnalyzeColor { color_input, .. } => {
            if color_input.is_empty() {
                return Err(ColorError::InvalidArguments("Color input required".to_string()));
            }
        }
        CommandType::ConvertColor { color_input, target_format, .. } => {
            if color_input.is_empty() || target_format.is_empty() {
                return Err(ColorError::InvalidArguments("Color input and target format required".to_string()));
            }
        }
    }
    Ok(())
}

const fn check_command_prerequisites(_command_type: &CommandType) -> Result<()> {
    // Could check for required files, permissions, etc.
    Ok(())
}

fn format_command_output(mut result: ExecutionResult) -> ExecutionResult {
    // Add formatting markers or structure output
    if result.success && !result.output.is_empty() {
        result.output = format!("=== Command Output ===\n{}\n=== End Output ===", result.output);
    }
    result
}

const fn save_command_output(result: ExecutionResult) -> ExecutionResult {
    // Could save output to file if specified in metadata
    // For now, just return the result unchanged
    result
}
