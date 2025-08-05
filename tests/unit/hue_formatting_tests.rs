//! Comprehensive unit tests for hue analysis output formatting
//!
//! This module tests terminal formatting, file export, and serialization functionality
//! for hue analysis results.

use color_rs::cli::OutputFormat;
use color_rs::color_ops::analysis::hue::{
    ColorCollectionType, HueAnalysisOptions, HueAnalysisResult, SortCriteria, export_hue_analysis,
    format_hue_analysis_terminal,
};
use palette::Lch;
use std::fs;
use tempfile::tempdir;

/// Helper function to create sample hue analysis results
fn create_sample_results() -> Vec<HueAnalysisResult> {
    vec![
        HueAnalysisResult {
            color: Lch::new(50.0, 75.0, 0.0),
            name: Some("Red".to_string()),
            hue_distance: 0.0,
            saturation: 75.0,
            lightness: 50.0,
            collection: "css".to_string(),
        },
        HueAnalysisResult {
            color: Lch::new(60.0, 60.0, 120.0),
            name: Some("Green".to_string()),
            hue_distance: 120.0,
            saturation: 60.0,
            lightness: 60.0,
            collection: "css".to_string(),
        },
        HueAnalysisResult {
            color: Lch::new(40.0, 80.0, 240.0),
            name: Some("Blue".to_string()),
            hue_distance: 240.0,
            saturation: 80.0,
            lightness: 40.0,
            collection: "css".to_string(),
        },
    ]
}

/// Test terminal formatting with multiple results
#[test]
fn test_format_hue_analysis_terminal_multiple_results() {
    let results = create_sample_results();
    let output = format_hue_analysis_terminal(&results);

    // Should contain header
    assert!(output.contains("Hue Analysis Results"));
    assert!(output.contains("Hue"));
    assert!(output.contains("Code"));
    assert!(output.contains("HEX"));
    assert!(output.contains("LCH"));
    assert!(output.contains("Name"));
    assert!(output.contains("Hue Shift"));

    // Should contain all color names
    assert!(output.contains("Red"));
    assert!(output.contains("Green"));
    assert!(output.contains("Blue"));

    // Should contain total count
    assert!(output.contains("Total: 3 colors"));

    // Should be properly formatted as a table
    assert!(output.contains("┌")); // Top border
    assert!(output.contains("└")); // Bottom border
    assert!(output.contains("│")); // Vertical separators
}

/// Test terminal formatting with single result
#[test]
fn test_format_hue_analysis_terminal_single_result() {
    let results = vec![create_sample_results()[0].clone()];
    let output = format_hue_analysis_terminal(&results);

    assert!(output.contains("Hue Analysis Results"));
    assert!(output.contains("Red"));
    assert!(output.contains("Total: 1 colors"));

    // Should still be properly formatted
    assert!(output.contains("┌"));
    assert!(output.contains("└"));
    assert!(output.contains("│"));
}

/// Test terminal formatting with empty results
#[test]
fn test_format_hue_analysis_terminal_empty_results() {
    let results = vec![];
    let output = format_hue_analysis_terminal(&results);

    assert!(output.contains("No colors found"));
    // Should still be a valid table format
    assert!(!output.is_empty());
}

/// Test terminal formatting with colors without names
#[test]
fn test_format_hue_analysis_terminal_no_names() {
    let mut result = create_sample_results()[0].clone();
    result.name = None;
    let results = vec![result];

    let output = format_hue_analysis_terminal(&results);

    // Should handle missing names gracefully
    assert!(output.contains("Hue Analysis Results"));
    assert!(output.contains("Total: 1 colors"));
}

/// Test YAML export functionality
#[test]
fn test_export_hue_analysis_yaml() -> color_rs::Result<()> {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test_output.yaml");

    let results = create_sample_results();
    let input_color = Lch::new(50.0, 75.0, 0.0);
    let options = HueAnalysisOptions {
        target_hue: Some(180.0),
        tolerance: 15.0,
        min_saturation: Some(30.0),
        min_lightness: Some(20.0),
    };
    let collection_type = ColorCollectionType::Css;
    let sort_criteria = SortCriteria::HueDistance;

    // Export to YAML
    export_hue_analysis(
        &results,
        &input_color,
        &options,
        &collection_type,
        &sort_criteria,
        OutputFormat::Yaml,
        file_path.to_str().unwrap(),
    )?;

    // Verify file was created
    assert!(file_path.exists());

    // Read and verify content
    let content = fs::read_to_string(&file_path)?;

    // Should contain metadata
    assert!(content.contains("metadata:"));
    assert!(content.contains("program_name: color-rs"));
    assert!(content.contains("version:"));
    assert!(content.contains("analysis_mode: hue"));

    // Should contain input information
    assert!(content.contains("input:"));
    assert!(content.contains("tolerance: 15"));
    assert!(content.contains("collection: css"));

    // Should contain results
    assert!(content.contains("results:"));
    assert!(content.contains("- hue:"));
    assert!(content.contains("name: Red"));
    assert!(content.contains("name: Green"));
    assert!(content.contains("name: Blue"));

    // Should have proper YAML structure
    assert!(content.contains("lch:"));
    assert!(content.contains("l:"));
    assert!(content.contains("c:"));
    assert!(content.contains("h:"));

    Ok(())
}

/// Test TOML export functionality
#[test]
fn test_export_hue_analysis_toml() -> color_rs::Result<()> {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test_output.toml");

    let results = create_sample_results();
    let input_color = Lch::new(50.0, 75.0, 0.0);
    let options = HueAnalysisOptions {
        target_hue: Some(180.0),
        tolerance: 15.0,
        min_saturation: Some(30.0),
        min_lightness: Some(20.0),
    };
    let collection_type = ColorCollectionType::Css;
    let sort_criteria = SortCriteria::HueDistance;

    // Export to TOML
    export_hue_analysis(
        &results,
        &input_color,
        &options,
        &collection_type,
        &sort_criteria,
        OutputFormat::Toml,
        file_path.to_str().unwrap(),
    )?;

    // Verify file was created
    assert!(file_path.exists());

    // Read and verify content
    let content = fs::read_to_string(&file_path)?;

    // Should contain metadata section
    assert!(content.contains("[metadata]"));
    assert!(content.contains("program_name = \"color-rs\""));
    assert!(content.contains("analysis_mode = \"hue\""));

    // Should contain input section
    assert!(content.contains("[input]"));
    assert!(content.contains("tolerance = 15"));
    assert!(content.contains("collection = \"css\""));

    // Should contain results arrays
    assert!(content.contains("[[results]]"));
    assert!(content.contains("name = \"Red\""));
    assert!(content.contains("name = \"Green\""));
    assert!(content.contains("name = \"Blue\""));

    // Should have proper TOML structure for LCH values
    assert!(content.contains("[results.lch]"));

    Ok(())
}

/// Test export with empty results
#[test]
fn test_export_hue_analysis_empty_results() -> color_rs::Result<()> {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test_empty.yaml");

    let results = vec![];
    let input_color = Lch::new(50.0, 75.0, 0.0);
    let options = HueAnalysisOptions {
        target_hue: Some(180.0),
        tolerance: 15.0,
        min_saturation: None,
        min_lightness: None,
    };
    let collection_type = ColorCollectionType::Css;
    let sort_criteria = SortCriteria::HueDistance;

    // Export empty results
    export_hue_analysis(
        &results,
        &input_color,
        &options,
        &collection_type,
        &sort_criteria,
        OutputFormat::Yaml,
        file_path.to_str().unwrap(),
    )?;

    // Verify file was created
    assert!(file_path.exists());

    // Read and verify content
    let content = fs::read_to_string(&file_path)?;

    // Should contain metadata and input even with empty results
    assert!(content.contains("metadata:"));
    assert!(content.contains("input:"));
    assert!(content.contains("results: []"));

    Ok(())
}

/// Test file extension handling
#[test]
fn test_export_file_extension_handling() -> color_rs::Result<()> {
    let temp_dir = tempdir().expect("Failed to create temp directory");

    let results = create_sample_results();
    let input_color = Lch::new(50.0, 75.0, 0.0);
    let options = HueAnalysisOptions {
        target_hue: None,
        tolerance: 15.0,
        min_saturation: None,
        min_lightness: None,
    };
    let collection_type = ColorCollectionType::Css;
    let sort_criteria = SortCriteria::HueDistance;

    // Test YAML extension handling
    let yaml_base_path = temp_dir.path().join("test_yaml");
    export_hue_analysis(
        &results,
        &input_color,
        &options,
        &collection_type,
        &sort_criteria,
        OutputFormat::Yaml,
        yaml_base_path.to_str().unwrap(),
    )?;

    // Should create file with .yaml extension
    let yaml_file = temp_dir.path().join("test_yaml.yaml");
    assert!(yaml_file.exists());

    // Test TOML extension handling
    let toml_base_path = temp_dir.path().join("test_toml");
    export_hue_analysis(
        &results,
        &input_color,
        &options,
        &collection_type,
        &sort_criteria,
        OutputFormat::Toml,
        toml_base_path.to_str().unwrap(),
    )?;

    // Should create file with .toml extension
    let toml_file = temp_dir.path().join("test_toml.toml");
    assert!(toml_file.exists());

    Ok(())
}

/// Test formatting consistency across different result sets
#[test]
fn test_format_consistency() {
    // Test with various result configurations
    let single_result = vec![create_sample_results()[0].clone()];
    let multiple_results = create_sample_results();
    let empty_results = vec![];

    // All should produce valid, non-empty formatted output
    assert!(!format_hue_analysis_terminal(&single_result).is_empty());
    assert!(!format_hue_analysis_terminal(&multiple_results).is_empty());
    assert!(!format_hue_analysis_terminal(&empty_results).is_empty());

    // All should contain consistent header structure
    for results in [&single_result, &multiple_results, &empty_results] {
        let output = format_hue_analysis_terminal(results);
        if !results.is_empty() {
            assert!(output.contains("Hue Analysis Results"));
            assert!(output.contains("Total:"));
        } else {
            assert!(output.contains("No colors found"));
        }
    }
}

/// Test serialization structure and field completeness
#[test]
fn test_serialization_structure() -> color_rs::Result<()> {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("structure_test.yaml");

    let results = create_sample_results();
    let input_color = Lch::new(50.0, 75.0, 0.0);
    let options = HueAnalysisOptions {
        target_hue: Some(180.0),
        tolerance: 15.0,
        min_saturation: Some(30.0),
        min_lightness: Some(20.0),
    };
    let collection_type = ColorCollectionType::Css;
    let sort_criteria = SortCriteria::HueDistance;

    export_hue_analysis(
        &results,
        &input_color,
        &options,
        &collection_type,
        &sort_criteria,
        OutputFormat::Yaml,
        file_path.to_str().unwrap(),
    )?;

    let content = fs::read_to_string(&file_path)?;

    // Verify all required fields are present
    let required_metadata_fields = [
        "program_name",
        "version",
        "author",
        "description",
        "generated_at",
        "analysis_mode",
    ];

    for field in &required_metadata_fields {
        assert!(content.contains(field), "Missing metadata field: {}", field);
    }

    let required_input_fields = ["color", "tolerance", "collection", "sort_criteria"];

    for field in &required_input_fields {
        assert!(content.contains(field), "Missing input field: {}", field);
    }

    let required_result_fields = ["hue", "code", "hex", "name", "lch"];

    for field in &required_result_fields {
        assert!(content.contains(field), "Missing result field: {}", field);
    }

    // Verify LCH structure
    assert!(content.contains("l:"));
    assert!(content.contains("c:"));
    assert!(content.contains("h:"));

    Ok(())
}
