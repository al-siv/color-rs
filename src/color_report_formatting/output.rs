//! Output generation and file operations
//!
//! Handles file writing, format-specific output generation, and file management utilities.

use crate::cli::OutputFormat;
use crate::error::{ColorError, Result};
use crate::output_formats::ColorAnalysisOutput;
use colored::Colorize;
use std::fs::File;
use std::io::Write;

/// Write analysis data to file in the specified format
/// 
/// # Errors
/// Returns an error if file writing fails or serialization errors occur
pub fn write_output_file(
    analysis_data: &ColorAnalysisOutput,
    filename: &str,
    format: &OutputFormat,
) -> Result<()> {
    match format {
        OutputFormat::Toml => write_toml_file(analysis_data, filename),
        OutputFormat::Yaml => write_yaml_file(analysis_data, filename),
    }
}

/// Write analysis data to TOML file
fn write_toml_file(analysis_data: &ColorAnalysisOutput, filename: &str) -> Result<()> {
    let toml_filename = ensure_file_extension(filename, "toml");
    let toml_content = analysis_data
        .to_toml()
        .map_err(|e| ColorError::InvalidArguments(format!("Failed to serialize to TOML: {e}")))?;

    write_file_content(&toml_filename, &toml_content)?;
    println!(
        "Color analysis saved to TOML file: {}",
        toml_filename.green()
    );
    Ok(())
}

/// Write analysis data to YAML file
fn write_yaml_file(analysis_data: &ColorAnalysisOutput, filename: &str) -> Result<()> {
    let yaml_filename = ensure_yaml_extension(filename);
    let yaml_content = analysis_data
        .to_yaml()
        .map_err(|e| ColorError::InvalidArguments(format!("Failed to serialize to YAML: {e}")))?;

    write_file_content(&yaml_filename, &yaml_content)?;
    println!(
        "Color analysis saved to YAML file: {}",
        yaml_filename.green()
    );
    Ok(())
}

/// Ensure filename has the correct extension
fn ensure_file_extension(filename: &str, extension: &str) -> String {
    if std::path::Path::new(filename)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case(extension))
    {
        filename.to_string()
    } else {
        format!("{filename}.{extension}")
    }
}

/// Ensure filename has YAML extension (supports both .yaml and .yml)
fn ensure_yaml_extension(filename: &str) -> String {
    if std::path::Path::new(filename)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("yaml"))
        || std::path::Path::new(filename)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("yml"))
    {
        filename.to_string()
    } else {
        format!("{filename}.yaml")
    }
}

/// Write content to file with error handling
fn write_file_content(filename: &str, content: &str) -> Result<()> {
    let mut file = File::create(filename).map_err(|e| {
        ColorError::InvalidArguments(format!("Failed to create file {filename}: {e}"))
    })?;

    file.write_all(content.as_bytes()).map_err(|e| {
        ColorError::InvalidArguments(format!("Failed to write to file {filename}: {e}"))
    })?;

    Ok(())
}
