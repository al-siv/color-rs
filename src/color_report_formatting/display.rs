//! Display formatting functions
//!
//! Handles terminal display colorization, formatting, and visual output enhancement.

use crate::cli::OutputFormat;
use colored::Colorize;

/// Display formatted output to terminal with colorization
pub fn display_terminal_output(formatted_output: &str, format: &OutputFormat) {
    for line in formatted_output.lines() {
        let colored_line = colorize_structured_line(line, format);
        println!("{colored_line}");
    }
}

/// Colorize a single line of TOML/YAML output
#[must_use]
pub fn colorize_structured_line(line: &str, format: &OutputFormat) -> String {
    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];

    match format {
        OutputFormat::Toml => colorize_toml_line(indent, trimmed),
        OutputFormat::Yaml => colorize_yaml_line(indent, trimmed),
    }
}

/// Colorize TOML format lines
fn colorize_toml_line(indent: &str, trimmed: &str) -> String {
    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        // Section headers like [metadata], [conversion]
        format!("{}{}", indent, trimmed.bold().cyan())
    } else if trimmed.starts_with("[[") && trimmed.ends_with("]]") {
        // Array section headers like [[color_collections.css_colors]]
        format!("{}{}", indent, trimmed.bold().magenta())
    } else if let Some(eq_pos) = trimmed.find(" = ") {
        // Key = value pairs
        let key = &trimmed[..eq_pos];
        let value = &trimmed[eq_pos + 3..];
        format!("{}{} = {}", indent, key.green(), value)
    } else {
        format!("{indent}{trimmed}")
    }
}

/// Colorize YAML format lines
fn colorize_yaml_line(indent: &str, trimmed: &str) -> String {
    if trimmed.ends_with(':') && !trimmed.contains(' ') {
        // Top-level keys like "metadata:", "conversion:"
        format!("{}{}", indent, trimmed.bold().cyan())
    } else if let Some(colon_pos) = trimmed.find(": ") {
        // Key: value pairs
        let key = &trimmed[..=colon_pos];
        let value = &trimmed[colon_pos + 2..];
        format!("{}{} {}", indent, key.green(), value)
    } else if let Some(stripped) = trimmed.strip_prefix("- ") {
        // Array items
        format!("{indent}- {stripped}")
    } else {
        format!("{indent}{trimmed}")
    }
}
