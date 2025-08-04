//! Backward Compatibility Layer
//!
//! This module provides compatibility shims and deprecated re-exports to maintain
//! backward compatibility for users who may have been using the legacy GoF patterns
//! that were removed in v0.16.0.
//!
//! All items in this module are deprecated and users should migrate to the modern
//! equivalents as documented in the migration guide.

use crate::error::Result;
use crate::color_parsing::{ParserType, ParsingConfig, parse_color, get_color_name};
use crate::command_execution::{CommandType, execute_command, ExecutionContext};
use std::collections::HashMap;

/// Backward compatibility type alias for the removed ColorParserType
/// 
/// **MIGRATION NOTE**: Use `ParserType` from `color_parsing` instead.
pub type ColorParserType = ParserType;

/// Backward compatibility function for the removed ColorParserFactory::create_parser
/// 
/// **MIGRATION NOTE**: Use `parse_color` directly instead.
pub fn create_parser(parser_type: ParserType) -> Result<Box<dyn ColorParserCompatTrait>> {
    Ok(Box::new(CompatParser { parser_type }))
}

/// Compatibility trait to mimic the old ColorParserTrait interface
/// 
/// **MIGRATION NOTE**: Use modern parsing functions instead.
pub trait ColorParserCompatTrait {
    fn parse(&self, input: &str) -> Result<(palette::Lab, crate::color_parser::ColorFormat)>;
    fn get_color_name(&self, rgb: (u8, u8, u8)) -> String;
}

/// Compatibility implementation
struct CompatParser {
    parser_type: ParserType,
}

#[allow(deprecated)]
impl ColorParserCompatTrait for CompatParser {
    fn parse(&self, input: &str) -> Result<(palette::Lab, crate::color_parser::ColorFormat)> {
        let config = ParsingConfig {
            parser_type: self.parser_type.clone(),
            enable_fallback_naming: true,
            color_tolerance: 10.0,
            preprocessing: vec![],
            postprocessing: vec![],
        };
        parse_color(input, &config)
    }

    fn get_color_name(&self, rgb: (u8, u8, u8)) -> String {
        let config = ParsingConfig {
            parser_type: self.parser_type.clone(),
            enable_fallback_naming: true,
            color_tolerance: 10.0,
            preprocessing: vec![],
            postprocessing: vec![],
        };
        get_color_name([rgb.0, rgb.1, rgb.2], &config)
    }
}

/// Backward compatibility type alias for the removed CommandType
/// 
/// **MIGRATION NOTE**: Use `CommandType` from `command_execution` instead.
pub type LegacyCommandType = CommandType;

/// Backward compatibility function for command execution
/// 
/// **MIGRATION NOTE**: Use `execute_command` directly instead.
pub fn execute_legacy_command(cmd_type: CommandType) -> Result<String> {
    let context = ExecutionContext {
        command_type: cmd_type,
        pre_hooks: vec![],
        post_hooks: vec![],
        metadata: HashMap::new(),
    };
    
    let result = execute_command(&context)?;
    Ok(result.output)
}

/// Compatibility module for re-exports of removed types
pub mod legacy {
    /// Re-export for backward compatibility
    pub use super::ColorParserType;
    
    /// Re-export for backward compatibility  
    pub use super::LegacyCommandType;
    
    /// Re-export for backward compatibility
    pub use super::ColorParserCompatTrait;
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use crate::cli::GradientArgs;
    use super::{create_parser, execute_legacy_command, CommandType};
    use crate::color_parsing::ParserType;

    #[test]
    #[allow(deprecated)]
    fn test_compat_parser() {
        let parser = create_parser(ParserType::Css).unwrap();
        
        // Test parsing
        let result = parser.parse("#FF0000");
        assert!(result.is_ok());
        
        // Test color name retrieval
        let name = parser.get_color_name((255, 0, 0));
        assert!(!name.is_empty());
    }

    #[test]
    #[allow(deprecated)]
    fn test_compat_command() {
        let args = GradientArgs {
            start_color: "#FF0000".to_string(),
            end_color: "#0000FF".to_string(),
            start_position: 0,
            end_position: 100,
            ease_in: 0.65,
            ease_out: 0.35,
            svg: None,
            png: None,
            no_legend: false,
            width: 1000,
            step: Some(10),
            stops: 5,
            stops_simple: false,
            output_format: Some(crate::cli::OutputFormat::Yaml),
            output_file: None,
            func_filter: None,
        };
        
        let cmd = CommandType::GenerateGradient {
            args,
            output_path: None,
        };
        
        let result = execute_legacy_command(cmd);
        assert!(result.is_ok());
    }
}
