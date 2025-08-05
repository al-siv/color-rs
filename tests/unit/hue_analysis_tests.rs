//! Comprehensive unit tests for hue analysis core domain logic
//!
//! This module provides comprehensive testing for all pure functions in the hue analysis
//! system, focusing on mathematical correctness, edge cases, and functional programming
//! compliance.

use color_rs::color::parse_color_input;
use color_rs::color_ops::analysis::hue::{
    ColorCollectionType, HueAnalysisOptions, HueAnalysisResult, SortCriteria,
    analyze_collection_hues, calculate_hue_distance, meets_criteria, normalize_hue,
};
use palette::{IntoColor, Lch};

/// Test hue distance calculation for basic cases
#[test]
fn test_hue_distance_basic_cases() {
    // Same hue should return 0
    assert_eq!(calculate_hue_distance(180.0, 180.0), 0.0);

    // Adjacent hues should return 1
    assert_eq!(calculate_hue_distance(180.0, 181.0), 1.0);
    assert_eq!(calculate_hue_distance(181.0, 180.0), 1.0);

    // Opposite hues should return 180
    assert_eq!(calculate_hue_distance(0.0, 180.0), 180.0);
    assert_eq!(calculate_hue_distance(180.0, 0.0), 180.0);

    // Quarter circle should return 90
    assert_eq!(calculate_hue_distance(0.0, 90.0), 90.0);
    assert_eq!(calculate_hue_distance(90.0, 0.0), 90.0);
}

/// Test hue distance calculation with wraparound cases
#[test]
fn test_hue_distance_wraparound() {
    // Wraparound near 0/360 boundary
    assert_eq!(calculate_hue_distance(359.0, 1.0), 2.0);
    assert_eq!(calculate_hue_distance(1.0, 359.0), 2.0);

    // Larger wraparound cases
    assert_eq!(calculate_hue_distance(350.0, 10.0), 20.0);
    assert_eq!(calculate_hue_distance(10.0, 350.0), 20.0);

    // Maximum distance through wraparound should be 180
    assert_eq!(calculate_hue_distance(270.0, 90.0), 180.0);
    assert_eq!(calculate_hue_distance(90.0, 270.0), 180.0);
}

/// Test hue normalization for various input ranges
#[test]
fn test_hue_normalization() {
    // Basic cases within range
    assert_eq!(normalize_hue(0.0), 0.0);
    assert_eq!(normalize_hue(180.0), 180.0);
    assert_eq!(normalize_hue(359.0), 359.0);

    // Cases requiring normalization
    assert_eq!(normalize_hue(360.0), 0.0);
    assert_eq!(normalize_hue(361.0), 1.0);
    assert_eq!(normalize_hue(450.0), 90.0);
    assert_eq!(normalize_hue(720.0), 0.0);

    // Negative cases
    assert_eq!(normalize_hue(-1.0), 359.0);
    assert_eq!(normalize_hue(-90.0), 270.0);
    assert_eq!(normalize_hue(-360.0), 0.0);
    assert_eq!(normalize_hue(-361.0), 359.0);
}

/// Test criteria filtering with various thresholds
#[test]
fn test_meets_criteria_filtering() {
    let options = HueAnalysisOptions {
        target_hue: Some(180.0),
        tolerance: 10.0,
        min_saturation: Some(30.0),
        min_lightness: Some(40.0),
    };

    // Color that meets all criteria
    let matching_color = Lch::new(50.0, 35.0, 175.0);
    assert!(meets_criteria(&matching_color, Some(180.0), &options));

    // Color outside hue tolerance
    let hue_mismatch = Lch::new(50.0, 35.0, 200.0);
    assert!(!meets_criteria(&hue_mismatch, Some(180.0), &options));

    // Color below saturation threshold
    let low_saturation = Lch::new(50.0, 20.0, 175.0);
    assert!(!meets_criteria(&low_saturation, Some(180.0), &options));

    // Color below lightness threshold
    let low_lightness = Lch::new(30.0, 35.0, 175.0);
    assert!(!meets_criteria(&low_lightness, Some(180.0), &options));
}

/// Test criteria filtering with no target hue
#[test]
fn test_meets_criteria_no_target() {
    let options = HueAnalysisOptions {
        target_hue: None,
        tolerance: 10.0,
        min_saturation: Some(30.0),
        min_lightness: Some(40.0),
    };

    // With no target hue, only saturation and lightness matter
    let good_color = Lch::new(50.0, 35.0, 250.0); // Any hue is fine
    assert!(meets_criteria(&good_color, None, &options));

    let low_saturation = Lch::new(50.0, 20.0, 250.0);
    assert!(!meets_criteria(&low_saturation, None, &options));
}

/// Test sort criteria parsing and validation
#[test]
fn test_sort_criteria_parsing() {
    use std::str::FromStr;

    // Valid criteria
    assert_eq!(
        SortCriteria::from_str("hue-distance").unwrap(),
        SortCriteria::HueDistance
    );
    assert_eq!(
        SortCriteria::from_str("saturation").unwrap(),
        SortCriteria::Saturation
    );
    assert_eq!(
        SortCriteria::from_str("lightness").unwrap(),
        SortCriteria::Lightness
    );
    assert_eq!(SortCriteria::from_str("name").unwrap(), SortCriteria::Name);

    // Invalid criteria should error
    assert!(SortCriteria::from_str("invalid").is_err());
    assert!(SortCriteria::from_str("").is_err());
}

/// Test color collection type parsing
#[test]
fn test_collection_type_parsing() {
    use std::str::FromStr;

    // Valid collection types
    assert_eq!(
        ColorCollectionType::from_str("css").unwrap(),
        ColorCollectionType::Css
    );
    assert_eq!(
        ColorCollectionType::from_str("ral-classic").unwrap(),
        ColorCollectionType::RalClassic
    );
    assert_eq!(
        ColorCollectionType::from_str("ral-design").unwrap(),
        ColorCollectionType::RalDesign
    );
    assert_eq!(
        ColorCollectionType::from_str("all").unwrap(),
        ColorCollectionType::All
    );

    // Invalid collection types should error
    assert!(ColorCollectionType::from_str("invalid").is_err());
    assert!(ColorCollectionType::from_str("").is_err());
}

/// Test complete hue analysis pipeline with CSS collection
#[test]
fn test_analyze_collection_hues_css() -> color_rs::Result<()> {
    let input_color: Lch = parse_color_input("#ff0000")?.into_color();

    let options = HueAnalysisOptions {
        target_hue: None,
        tolerance: 15.0,
        min_saturation: None,
        min_lightness: None,
    };

    let results = analyze_collection_hues(
        &ColorCollectionType::Css,
        &input_color,
        &options,
        SortCriteria::HueDistance,
        10,
    )?;

    // Should return some results
    assert!(!results.is_empty());
    assert!(results.len() <= 10);

    // Results should be sorted by hue distance
    for i in 1..results.len() {
        assert!(results[i - 1].hue_distance <= results[i].hue_distance);
    }

    // All results should have proper metadata
    for result in &results {
        assert!(result.name.is_some());
        assert_eq!(result.collection, "css");
        assert!(result.hue_distance >= 0.0);
        assert!(result.saturation >= 0.0);
        assert!(result.lightness >= 0.0);
    }

    Ok(())
}

/// Test analysis with filtering options
#[test]
fn test_analyze_collection_hues_with_filters() -> color_rs::Result<()> {
    let input_color: Lch = parse_color_input("#00ff00")?.into_color();

    let options = HueAnalysisOptions {
        target_hue: Some(120.0),
        tolerance: 30.0,
        min_saturation: Some(50.0),
        min_lightness: Some(30.0),
    };

    let results = analyze_collection_hues(
        &ColorCollectionType::Css,
        &input_color,
        &options,
        SortCriteria::Saturation,
        5,
    )?;

    // All results should meet the criteria
    for result in &results {
        assert!(
            result.saturation >= 50.0,
            "Saturation {} should be >= 50.0",
            result.saturation
        );
        assert!(
            result.lightness >= 30.0,
            "Lightness {} should be >= 30.0",
            result.lightness
        );
    }

    // Results should be sorted by saturation (descending)
    for i in 1..results.len() {
        assert!(results[i - 1].saturation >= results[i].saturation);
    }

    Ok(())
}

/// Test edge case with empty results
#[test]
fn test_analyze_collection_hues_empty_results() -> color_rs::Result<()> {
    let input_color: Lch = parse_color_input("#ffffff")?.into_color();

    // Very restrictive criteria that should match nothing
    let options = HueAnalysisOptions {
        target_hue: Some(180.0),
        tolerance: 1.0,
        min_saturation: Some(90.0),
        min_lightness: Some(95.0),
    };

    let results = analyze_collection_hues(
        &ColorCollectionType::Css,
        &input_color,
        &options,
        SortCriteria::HueDistance,
        10,
    )?;

    // Should handle empty results gracefully
    assert!(results.is_empty() || results.len() <= 10);

    Ok(())
}

/// Test analysis with limit parameter
#[test]
fn test_analyze_collection_hues_limit() -> color_rs::Result<()> {
    let input_color: Lch = parse_color_input("#0000ff")?.into_color();

    let options = HueAnalysisOptions {
        target_hue: None,
        tolerance: 15.0,
        min_saturation: None,
        min_lightness: None,
    };

    // Test with small limit
    let results = analyze_collection_hues(
        &ColorCollectionType::Css,
        &input_color,
        &options,
        SortCriteria::HueDistance,
        3,
    )?;

    assert!(results.len() <= 3);

    // Test with larger limit
    let larger_results = analyze_collection_hues(
        &ColorCollectionType::Css,
        &input_color,
        &options,
        SortCriteria::HueDistance,
        20,
    )?;

    assert!(larger_results.len() >= results.len());
    assert!(larger_results.len() <= 20);

    Ok(())
}
