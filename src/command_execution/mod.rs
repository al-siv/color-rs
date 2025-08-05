//! Command execution module - Functional command processing
//!
//! This module provides a functional replacement for the Command Pattern in CLI operations.
//! Instead of trait objects and command invokers, it uses enum-based dispatch and pure functions
//! for zero-cost abstraction and better performance.

pub mod types;
pub mod execution;
pub mod commands;
pub mod convenience;

// Re-export main types and functions for public API
pub use types::{
    CommandType, ExecutionContext, ExecutionResult, 
    PreHookStep, PostHookStep, AVAILABLE_COMMAND_TYPES
};

pub use execution::{
    execute_command, get_command_name, get_command_description, supports_undo,
    execute_command_simple, execute_command_with_validation, execute_command_enhanced
};

pub use commands::{
    execute_generate_gradient, execute_find_closest_color,
    execute_analyze_color, execute_convert_color
};

pub use convenience::{
    create_gradient_command, create_analyze_command, 
    create_find_closest_command, create_convert_command,
    execute_simple, execute_with_validation, execute_enhanced
};

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
        let result = execute_command_with_validation(cmd);
        assert!(result.is_ok()); // The execution returns a failure result, but no error
        let result = result.unwrap();
        assert!(!result.success); // Command should fail validation
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

    #[test]
    fn test_convenience_execution_aliases() {
        let cmd = create_analyze_command("#ff0000".to_string(), false);
        
        // Test convenience aliases
        let result1 = execute_simple(cmd.clone());
        let result2 = execute_with_validation(cmd.clone());
        let result3 = execute_enhanced(cmd);
        
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());
    }
}
