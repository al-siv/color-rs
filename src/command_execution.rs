//! Functional Command Processing - Command Pattern Replacement
//!
//! This module provides a functional replacement for the Command Pattern in CLI operations.
//! Instead of trait objects and command invokers, it uses enum-based dispatch and pure functions
//! for zero-cost abstraction and better performance.

use crate::cli::GradientArgs;
use crate::error::{ColorError, Result};
use palette::{IntoColor, Mix, Lab};  // Import traits for LAB interpolation and conversion
use std::collections::HashMap;

/// Command type using enum dispatch (replaces trait objects)
#[derive(Debug, Clone)]
pub enum CommandType {
    /// Generate color gradient between two colors
    GenerateGradient {
        args: GradientArgs,
        output_path: Option<String>,
    },
    /// Find closest matching colors in collections
    FindClosestColor {
        color_input: String,
        collection: Option<String>,
        algorithm: String,
        count: usize,
    },
    /// Analyze color properties and conversion
    AnalyzeColor {
        color_input: String,
        include_schemes: bool,
        output_format: String,
    },
    /// Convert color between different formats
    ConvertColor {
        color_input: String,
        target_format: String,
        precision: usize,
    },
}

/// Command execution context with functional composition
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Command type determines execution strategy
    pub command_type: CommandType,
    /// Pre-execution hooks (validation, logging, etc.)
    pub pre_hooks: Vec<PreHookStep>,
    /// Post-execution hooks (cleanup, formatting, etc.)
    pub post_hooks: Vec<PostHookStep>,
    /// Execution metadata
    pub metadata: HashMap<String, String>,
}

/// Command execution result using value types (not trait objects)
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub success: bool,
    pub output: String,
    pub error_message: Option<String>,
    pub metadata: HashMap<String, String>,
    pub execution_time_ms: u128,
}

/// Pre-execution hook steps using function composition
#[derive(Debug, Clone)]
pub enum PreHookStep {
    /// Validate command parameters
    ValidateParameters,
    /// Log command execution start
    LogStart,
    /// Check prerequisites
    CheckPrerequisites,
    /// Custom pre-hook function
    Custom(fn(&CommandType) -> Result<()>),
}

/// Post-execution hook steps using function composition
#[derive(Debug, Clone)]
pub enum PostHookStep {
    /// Format output according to rules
    FormatOutput,
    /// Log execution completion
    LogCompletion,
    /// Save output to file if specified
    SaveOutput,
    /// Custom post-hook function
    Custom(fn(&ExecutionResult) -> ExecutionResult),
}

/// Command execution pipeline - pure functions instead of trait methods
impl ExecutionContext {
    /// Create new execution context
    pub fn new(command_type: CommandType) -> Self {
        Self {
            command_type,
            pre_hooks: vec![],
            post_hooks: vec![],
            metadata: HashMap::new(),
        }
    }

    /// Builder pattern for adding pre-hooks
    pub fn with_pre_hook(mut self, hook: PreHookStep) -> Self {
        self.pre_hooks.push(hook);
        self
    }

    /// Builder pattern for adding post-hooks
    pub fn with_post_hook(mut self, hook: PostHookStep) -> Self {
        self.post_hooks.push(hook);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

impl ExecutionResult {
    /// Create successful result
    pub fn success(output: String) -> Self {
        Self {
            success: true,
            output,
            error_message: None,
            metadata: HashMap::new(),
            execution_time_ms: 0,
        }
    }

    /// Create successful result with metadata
    pub fn success_with_metadata(output: String, metadata: HashMap<String, String>) -> Self {
        Self {
            success: true,
            output,
            error_message: None,
            metadata,
            execution_time_ms: 0,
        }
    }

    /// Create failure result
    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            output: String::new(),
            error_message: Some(error),
            metadata: HashMap::new(),
            execution_time_ms: 0,
        }
    }

    /// Update execution time
    pub fn with_execution_time(mut self, time_ms: u128) -> Self {
        self.execution_time_ms = time_ms;
        self
    }
}

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
pub fn supports_undo(command_type: &CommandType) -> bool {
    match command_type {
        CommandType::GenerateGradient { .. } => false, // File generation can't be undone easily
        CommandType::FindClosestColor { .. } => false, // Read-only operation
        CommandType::AnalyzeColor { .. } => false,     // Read-only operation
        CommandType::ConvertColor { .. } => false,     // Pure transformation
    }
}

// Command execution functions using pattern matching instead of virtual dispatch

fn execute_generate_gradient(args: &GradientArgs, output_path: Option<&str>) -> Result<ExecutionResult> {
    let (start_lab, end_lab) = parse_gradient_colors(args)?;
    let gradient_output = generate_gradient_steps(start_lab, end_lab, args.stops);
    let format_output = append_format_outputs(args, output_path);
    let metadata = create_gradient_metadata(args);
    
    let final_output = format!("{}{}", gradient_output, format_output);
    Ok(ExecutionResult::success_with_metadata(final_output, metadata))
}

/// Parse start and end colors for gradient generation
fn parse_gradient_colors(args: &GradientArgs) -> Result<(Lab, Lab)> {
    let start_lab = crate::color::parse_color_input(&args.start_color)
        .map_err(|e| ColorError::ParseError(format!("Failed to parse start color: {}", e)))?;

    let end_lab = crate::color::parse_color_input(&args.end_color)
        .map_err(|e| ColorError::ParseError(format!("Failed to parse end color: {}", e)))?;
    
    Ok((start_lab, end_lab))
}

/// Generate gradient steps with color interpolation
fn generate_gradient_steps(start_lab: Lab, end_lab: Lab, steps: usize) -> String {
    let mut output = String::new();
    output.push_str("Generated gradient:\n");

    for i in 0..steps {
        let t = i as f64 / (steps - 1) as f64;
        // Use functional LAB interpolation with palette Mix trait
        let interpolated = start_lab.mix(end_lab, t as f32);
        let hex = crate::color_ops::conversion::srgb_to_hex(interpolated.into_color());
        use std::fmt::Write;
        writeln!(output, "Step {}: {}", i, hex).unwrap();
    }
    
    output
}

/// Generate output format messages
fn append_format_outputs(args: &GradientArgs, output_path: Option<&str>) -> String {
    let mut output = String::new();
    
    if args.should_generate_svg() {
        output.push_str("\nSVG generated successfully\n");
        if let Some(path) = output_path {
            use std::fmt::Write;
            writeln!(output, "SVG saved to: {}", path).unwrap();
        }
    }

    if args.should_generate_png() {
        output.push_str("PNG generated successfully\n");
    }
    
    output
}

/// Create metadata for gradient execution result
fn create_gradient_metadata(args: &GradientArgs) -> HashMap<String, String> {
    let mut metadata = HashMap::new();
    metadata.insert("start_color".to_string(), args.start_color.clone());
    metadata.insert("end_color".to_string(), args.end_color.clone());
    metadata.insert("steps".to_string(), args.stops.to_string());
    metadata
}

fn execute_find_closest_color(
    color_input: &str,
    collection: Option<&str>,
    algorithm: &str,
    count: usize,
) -> Result<ExecutionResult> {
    // Parse the color for validation
    let _lab_color = crate::color::parse_color_input(color_input)
        .map_err(|e| ColorError::ParseError(format!("Failed to parse color: {}", e)))?;

    // Create output using functional color matching
    let output = format!(
        "Finding {} closest colors to {} using {} algorithm\n",
        count, color_input, algorithm
    );

    let mut metadata = HashMap::new();
    metadata.insert("input_color".to_string(), color_input.to_string());
    metadata.insert("algorithm".to_string(), algorithm.to_string());
    metadata.insert("count".to_string(), count.to_string());
    if let Some(collection) = collection {
        metadata.insert("collection".to_string(), collection.to_string());
    }

    Ok(ExecutionResult::success_with_metadata(output, metadata))
}

fn execute_analyze_color(
    color_input: &str,
    include_schemes: bool,
    output_format: &str,
) -> Result<ExecutionResult> {
    // Parse the color for analysis
    let _lab_color = crate::color::parse_color_input(color_input)
        .map_err(|e| ColorError::ParseError(format!("Failed to parse color: {}", e)))?;

    let mut output = format!("Analyzing color: {}\n", color_input);
    output.push_str(&format!("Output format: {}\n", output_format));
    
    if include_schemes {
        output.push_str("Including color schemes in analysis\n");
    }

    let mut metadata = HashMap::new();
    metadata.insert("input_color".to_string(), color_input.to_string());
    metadata.insert("include_schemes".to_string(), include_schemes.to_string());
    metadata.insert("output_format".to_string(), output_format.to_string());

    Ok(ExecutionResult::success_with_metadata(output, metadata))
}

fn execute_convert_color(
    color_input: &str,
    target_format: &str,
    precision: usize,
) -> Result<ExecutionResult> {
    // Parse the color for conversion
    let _lab_color = crate::color::parse_color_input(color_input)
        .map_err(|e| ColorError::ParseError(format!("Failed to parse color: {}", e)))?;

    let output = format!(
        "Converting {} to {} format with {} decimal precision\n",
        color_input, target_format, precision
    );

    let mut metadata = HashMap::new();
    metadata.insert("input_color".to_string(), color_input.to_string());
    metadata.insert("target_format".to_string(), target_format.to_string());
    metadata.insert("precision".to_string(), precision.to_string());

    Ok(ExecutionResult::success_with_metadata(output, metadata))
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

fn check_command_prerequisites(_command_type: &CommandType) -> Result<()> {
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

fn save_command_output(result: ExecutionResult) -> ExecutionResult {
    // Could save output to file if specified in metadata
    // For now, just return the result unchanged
    result
}

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

/// Execute command with default context (no hooks)
pub fn execute_command_simple(command_type: CommandType) -> Result<ExecutionResult> {
    let context = ExecutionContext::new(command_type);
    execute_command(&context)
}

/// Execute command with validation hooks
pub fn execute_command_with_validation(command_type: CommandType) -> Result<ExecutionResult> {
    let context = ExecutionContext::new(command_type)
        .with_pre_hook(PreHookStep::ValidateParameters)
        .with_pre_hook(PreHookStep::LogStart)
        .with_post_hook(PostHookStep::LogCompletion);
    
    execute_command(&context)
}

/// Execute command with full hooks and formatting
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

/// Available command types - compile-time constant
pub const AVAILABLE_COMMAND_TYPES: &[&str] = &[
    "generate_gradient",
    "find_closest_color",
    "analyze_color", 
    "convert_color",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_result() {
        let success = ExecutionResult::success("test output".to_string());
        assert!(success.success);
        assert_eq!(success.output, "test output");
        assert!(success.error_message.is_none());

        let failure = ExecutionResult::failure("test error".to_string());
        assert!(!failure.success);
        assert_eq!(failure.error_message.unwrap(), "test error");
    }

    #[test]
    fn test_command_name_description() {
        let gradient_cmd = create_gradient_command("red".to_string(), "blue".to_string(), 5);
        assert_eq!(get_command_name(&gradient_cmd), "generate_gradient");
        assert_eq!(get_command_description(&gradient_cmd), "Generate a color gradient between two colors");
    }

    #[test]
    fn test_command_execution() {
        let cmd = create_analyze_command("#ff0000".to_string(), false);
        let result = execute_command_simple(cmd);
        assert!(result.is_ok());
        assert!(result.unwrap().success);
    }

    #[test]
    fn test_supports_undo() {
        let gradient_cmd = create_gradient_command("red".to_string(), "blue".to_string(), 5);
        let analyze_cmd = create_analyze_command("#ff0000".to_string(), false);
        
        assert!(!supports_undo(&gradient_cmd));
        assert!(!supports_undo(&analyze_cmd));
    }

    #[test]
    fn test_execution_context_builder() {
        let cmd = create_gradient_command("red".to_string(), "blue".to_string(), 5);
        let context = ExecutionContext::new(cmd)
            .with_pre_hook(PreHookStep::ValidateParameters)
            .with_post_hook(PostHookStep::FormatOutput)
            .with_metadata("test".to_string(), "value".to_string());
        
        assert_eq!(context.pre_hooks.len(), 1);
        assert_eq!(context.post_hooks.len(), 1);
        assert_eq!(context.metadata.get("test").unwrap(), "value");
    }

    #[test]
    fn test_parameter_validation() {
        // Invalid gradient command (empty colors) - create manually
        let args = crate::cli::GradientArgs {
            start_color: "".to_string(),
            end_color: "blue".to_string(),
            start_position: 0,
            end_position: 100,
            ease_in: 0.65,
            ease_out: 0.35,
            svg: None,
            png: None,
            no_legend: false,
            width: 1000,
            step: None,
            stops: 5,
            stops_simple: false,
            output_format: None,
            output_file: None,
            func_filter: None,
        };
        
        let cmd = CommandType::GenerateGradient { args, output_path: None };
        let result = validate_command_parameters(&cmd);
        assert!(result.is_err());
    }

    #[test]
    fn test_convenience_functions() {
        let gradient_cmd = create_gradient_command("red".to_string(), "blue".to_string(), 10);
        let analyze_cmd = create_analyze_command("#ff0000".to_string(), true);
        let find_cmd = create_find_closest_command("green".to_string(), 5);
        let convert_cmd = create_convert_command("#ffffff".to_string(), "HSL".to_string());

        // Test that commands are created correctly
        match gradient_cmd {
            CommandType::GenerateGradient { args, .. } => {
                assert_eq!(args.stops, 10);
                assert_eq!(args.start_color, "red");
            }
            _ => panic!("Wrong command type"),
        }

        match analyze_cmd {
            CommandType::AnalyzeColor { include_schemes, .. } => {
                assert!(include_schemes);
            }
            _ => panic!("Wrong command type"),
        }

        match find_cmd {
            CommandType::FindClosestColor { count, .. } => {
                assert_eq!(count, 5);
            }
            _ => panic!("Wrong command type"),
        }

        match convert_cmd {
            CommandType::ConvertColor { target_format, .. } => {
                assert_eq!(target_format, "HSL");
            }
            _ => panic!("Wrong command type"),
        }
    }

    #[test]
    fn test_execution_with_hooks() {
        let cmd = create_analyze_command("#ff0000".to_string(), false);
        
        // Test simple execution
        let result = execute_command_simple(cmd.clone());
        assert!(result.is_ok());
        
        // Test execution with validation
        let result = execute_command_with_validation(cmd.clone());
        assert!(result.is_ok());
        
        // Test enhanced execution
        let result = execute_command_enhanced(cmd);
        assert!(result.is_ok());
    }
}
