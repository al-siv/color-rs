//! File output service implementing Strategy pattern for different file formats
//!
//! This module provides a unified interface for writing color analysis results
//! to different file formats (TOML, YAML) using the Strategy design pattern.

use crate::error::{ColorError, Result};
use crate::output_formats::ColorAnalysisOutput;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// Strategy trait for different file output formats
pub trait FileOutputStrategy {
    /// Serialize the color analysis data to the format-specific string
    fn serialize(&self, data: &ColorAnalysisOutput) -> Result<String>;

    /// Get the file extension for this format
    fn file_extension(&self) -> &'static str;

    /// Get the format name for logging/error messages
    fn format_name(&self) -> &'static str;
}

/// TOML output strategy
pub struct TomlOutputStrategy;

impl FileOutputStrategy for TomlOutputStrategy {
    fn serialize(&self, data: &ColorAnalysisOutput) -> Result<String> {
        data.to_toml().map_err(|e| {
            ColorError::InvalidArguments(format!("Failed to serialize to TOML: {}", e))
        })
    }

    fn file_extension(&self) -> &'static str {
        "toml"
    }

    fn format_name(&self) -> &'static str {
        "TOML"
    }
}

/// YAML output strategy
pub struct YamlOutputStrategy;

impl FileOutputStrategy for YamlOutputStrategy {
    fn serialize(&self, data: &ColorAnalysisOutput) -> Result<String> {
        data.to_yaml().map_err(|e| {
            ColorError::InvalidArguments(format!("Failed to serialize to YAML: {}", e))
        })
    }

    fn file_extension(&self) -> &'static str {
        "yaml"
    }

    fn format_name(&self) -> &'static str {
        "YAML"
    }
}

/// File output service that uses different strategies
pub struct FileOutputService;

impl FileOutputService {
    /// Write color analysis data to a file using the specified strategy
    pub fn write_to_file<S: FileOutputStrategy>(
        strategy: &S,
        data: &ColorAnalysisOutput,
        filename: &str,
    ) -> Result<()> {
        // Validate filename
        Self::validate_filename(filename)?;

        // Serialize data using the strategy
        let content = strategy.serialize(data)?;

        // Ensure the filename has the correct extension
        let filename = Self::ensure_extension(filename, strategy.file_extension());

        // Write to file
        Self::write_file_content(&filename, &content, strategy.format_name())?;

        println!(
            "Color analysis saved to {} file: {}",
            strategy.format_name(),
            filename
        );
        Ok(())
    }

    /// Write to TOML file
    pub fn write_toml(data: &ColorAnalysisOutput, filename: &str) -> Result<()> {
        Self::write_to_file(&TomlOutputStrategy, data, filename)
    }

    /// Write to YAML file
    pub fn write_yaml(data: &ColorAnalysisOutput, filename: &str) -> Result<()> {
        Self::write_to_file(&YamlOutputStrategy, data, filename)
    }

    /// Write gradient analysis to TOML file
    pub fn write_gradient_toml(
        data: &crate::output_formats::GradientAnalysisOutput,
        filename: &str,
    ) -> Result<()> {
        Self::validate_filename(filename)?;

        // Ensure .toml extension
        let filename = if filename.ends_with(".toml") {
            filename.to_string()
        } else {
            format!("{}.toml", filename)
        };

        let toml_string = data
            .to_toml()
            .map_err(|e| ColorError::General(format!("Failed to serialize to TOML: {}", e)))?;

        fs::write(&filename, toml_string).map_err(|e| ColorError::IoError(e))?;

        Ok(())
    }

    /// Write gradient analysis to YAML file
    pub fn write_gradient_yaml(
        data: &crate::output_formats::GradientAnalysisOutput,
        filename: &str,
    ) -> Result<()> {
        Self::validate_filename(filename)?;

        // Ensure .yaml extension
        let filename = if filename.ends_with(".yaml") || filename.ends_with(".yml") {
            filename.to_string()
        } else {
            format!("{}.yaml", filename)
        };

        let yaml_string = data
            .to_yaml()
            .map_err(|e| ColorError::General(format!("Failed to serialize to YAML: {}", e)))?;

        fs::write(&filename, yaml_string).map_err(|e| ColorError::IoError(e))?;

        Ok(())
    }

    /// Validate filename for security and correctness
    fn validate_filename(filename: &str) -> Result<()> {
        if filename.is_empty() {
            return Err(ColorError::InvalidArguments(
                "Filename cannot be empty".to_string(),
            ));
        }

        // Check for invalid characters (basic validation)
        let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
        for ch in invalid_chars {
            if filename.contains(ch) {
                return Err(ColorError::InvalidArguments(format!(
                    "Filename contains invalid character: '{}'",
                    ch
                )));
            }
        }

        // Prevent directory traversal
        if filename.contains("..") {
            return Err(ColorError::InvalidArguments(
                "Filename cannot contain '..' (directory traversal)".to_string(),
            ));
        }

        Ok(())
    }

    /// Ensure filename has the correct extension
    fn ensure_extension(filename: &str, extension: &str) -> String {
        let path = Path::new(filename);

        match path.extension() {
            Some(ext) if ext == extension => filename.to_string(),
            _ => format!(
                "{}.{}",
                filename.trim_end_matches(&format!(".{}", extension)),
                extension
            ),
        }
    }

    /// Write content to file with error handling
    fn write_file_content(filename: &str, content: &str, format_name: &str) -> Result<()> {
        let mut file = File::create(filename).map_err(|e| {
            ColorError::InvalidArguments(format!(
                "Failed to create {} file '{}': {}",
                format_name, filename, e
            ))
        })?;

        file.write_all(content.as_bytes()).map_err(|e| {
            ColorError::InvalidArguments(format!(
                "Failed to write {} content to '{}': {}",
                format_name, filename, e
            ))
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output_formats::*;

    fn create_test_data() -> ColorAnalysisOutput {
        ColorAnalysisOutput::new().with_input("red".to_string(), "#FF0000".to_string())
    }

    #[test]
    fn test_toml_strategy() {
        let strategy = TomlOutputStrategy;
        let data = create_test_data();

        let result = strategy.serialize(&data);
        assert!(result.is_ok());

        let content = result.unwrap();
        assert!(content.contains("[metadata]"));
        assert!(content.contains("program_name"));
    }

    #[test]
    fn test_yaml_strategy() {
        let strategy = YamlOutputStrategy;
        let data = create_test_data();

        let result = strategy.serialize(&data);
        assert!(result.is_ok());

        let content = result.unwrap();
        assert!(content.contains("metadata:"));
        assert!(content.contains("program_name:"));
    }

    #[test]
    fn test_filename_validation() {
        assert!(FileOutputService::validate_filename("valid_file").is_ok());
        assert!(FileOutputService::validate_filename("").is_err());
        assert!(FileOutputService::validate_filename("file<").is_err());
        assert!(FileOutputService::validate_filename("../file").is_err());
    }

    #[test]
    fn test_extension_handling() {
        assert_eq!(
            FileOutputService::ensure_extension("test", "toml"),
            "test.toml"
        );
        assert_eq!(
            FileOutputService::ensure_extension("test.toml", "toml"),
            "test.toml"
        );
        assert_eq!(
            FileOutputService::ensure_extension("test.txt", "toml"),
            "test.txt.toml"
        );
    }
}
