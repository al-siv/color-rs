//! Comprehensive unit tests for hue analysis collection integration
//!
//! This module tests the integration with different color collections,
//! data loading, and metadata extraction functionality.

use color_rs::color::parse_color_input;
use color_rs::color_ops::analysis::hue::{
    ColorCollectionType, HueAnalysisOptions, SortCriteria, analyze_collection_hues,
    load_collection_colors,
};
use palette::{IntoColor, Lch};
use std::str::FromStr;

/// Test ColorCollectionType enum parsing
#[test]
fn test_color_collection_type_parsing() {
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

    // Case sensitivity
    assert_eq!(
        ColorCollectionType::from_str("CSS").unwrap(),
        ColorCollectionType::Css
    );
    assert_eq!(
        ColorCollectionType::from_str("Css").unwrap(),
        ColorCollectionType::Css
    );

    // Invalid collection types
    assert!(ColorCollectionType::from_str("invalid").is_err());
    assert!(ColorCollectionType::from_str("").is_err());
    assert!(ColorCollectionType::from_str("ral").is_err()); // Ambiguous
}

/// Test CSS collection loading
#[test]
fn test_load_css_collection() -> color_rs::Result<()> {
    let colors = load_collection_colors(&ColorCollectionType::Css)?;

    // CSS collection should have a reasonable number of colors
    assert!(!colors.is_empty(), "CSS collection should not be empty");
    assert!(
        colors.len() > 100,
        "CSS collection should have > 100 colors, got {}",
        colors.len()
    );
    assert!(
        colors.len() < 200,
        "CSS collection should have < 200 colors, got {}",
        colors.len()
    );

    // All colors should have proper metadata
    for color in &colors {
        assert!(color.name.is_some(), "CSS colors should have names");
        assert_eq!(color.collection, "css");
        assert!(color.hue_distance >= 0.0);
        assert!(color.saturation >= 0.0);
        assert!(color.lightness >= 0.0);
        assert!(color.lightness <= 100.0);
    }

    // Should contain some well-known CSS colors
    let color_names: Vec<String> = colors
        .iter()
        .filter_map(|c| c.name.as_ref())
        .map(|n| n.to_lowercase())
        .collect();

    let expected_colors = ["red", "green", "blue", "white", "black"];
    for expected in &expected_colors {
        assert!(
            color_names.iter().any(|name| name.contains(expected)),
            "CSS collection should contain color related to '{}'",
            expected
        );
    }

    Ok(())
}

/// Test RAL Classic collection loading
#[test]
fn test_load_ral_classic_collection() -> color_rs::Result<()> {
    let colors = load_collection_colors(&ColorCollectionType::RalClassic)?;

    // RAL Classic should have around 200+ colors
    assert!(
        !colors.is_empty(),
        "RAL Classic collection should not be empty"
    );
    assert!(
        colors.len() > 150,
        "RAL Classic should have > 150 colors, got {}",
        colors.len()
    );
    assert!(
        colors.len() < 300,
        "RAL Classic should have < 300 colors, got {}",
        colors.len()
    );

    // All colors should have proper metadata
    for color in &colors {
        assert!(color.name.is_some(), "RAL Classic colors should have names");
        assert_eq!(color.collection, "ral-classic");
        assert!(color.hue_distance >= 0.0);
        assert!(color.saturation >= 0.0);
        assert!(color.lightness >= 0.0);
        assert!(color.lightness <= 100.0);
    }

    Ok(())
}

/// Test RAL Design collection loading
#[test]
fn test_load_ral_design_collection() -> color_rs::Result<()> {
    let colors = load_collection_colors(&ColorCollectionType::RalDesign)?;

    // RAL Design should have the most colors (1600+)
    assert!(
        !colors.is_empty(),
        "RAL Design collection should not be empty"
    );
    assert!(
        colors.len() > 1000,
        "RAL Design should have > 1000 colors, got {}",
        colors.len()
    );
    assert!(
        colors.len() < 2000,
        "RAL Design should have < 2000 colors, got {}",
        colors.len()
    );

    // All colors should have proper metadata
    for color in &colors {
        assert!(color.name.is_some(), "RAL Design colors should have names");
        assert_eq!(color.collection, "ral-design");
        assert!(color.hue_distance >= 0.0);
        assert!(color.saturation >= 0.0);
        assert!(color.lightness >= 0.0);
        assert!(color.lightness <= 100.0);
    }

    Ok(())
}

/// Test All collections loading
#[test]
fn test_load_all_collections() -> color_rs::Result<()> {
    let all_colors = load_collection_colors(&ColorCollectionType::All)?;
    let css_colors = load_collection_colors(&ColorCollectionType::Css)?;
    let ral_classic_colors = load_collection_colors(&ColorCollectionType::RalClassic)?;
    let ral_design_colors = load_collection_colors(&ColorCollectionType::RalDesign)?;

    // All collection should contain more colors than any individual collection
    assert!(all_colors.len() > css_colors.len());
    assert!(all_colors.len() > ral_classic_colors.len());
    assert!(all_colors.len() > ral_design_colors.len());

    // Should be approximately the sum of all collections
    let expected_total = css_colors.len() + ral_classic_colors.len() + ral_design_colors.len();
    let tolerance = expected_total / 10; // 10% tolerance
    assert!(
        all_colors.len() >= expected_total - tolerance
            && all_colors.len() <= expected_total + tolerance,
        "All collection size {} should be close to sum of individual collections {}",
        all_colors.len(),
        expected_total
    );

    // Should contain colors from all collections
    let collections: std::collections::HashSet<String> =
        all_colors.iter().map(|c| c.collection.clone()).collect();

    assert!(collections.contains("css"));
    assert!(collections.contains("ral-classic"));
    assert!(collections.contains("ral-design"));

    Ok(())
}

/// Test collection integration with different sort criteria
#[test]
fn test_collection_integration_sorting() -> color_rs::Result<()> {
    let input_color: Lch = parse_color_input("#ff0000")?.into_color();
    let options = HueAnalysisOptions {
        target_hue: None,
        tolerance: 15.0,
        min_saturation: None,
        min_lightness: None,
    };

    // Test different sort criteria
    let sort_criteria = [
        SortCriteria::HueDistance,
        SortCriteria::Saturation,
        SortCriteria::Lightness,
        SortCriteria::Name,
    ];

    for criteria in &sort_criteria {
        let results = analyze_collection_hues(
            &ColorCollectionType::Css,
            &input_color,
            &options,
            criteria.clone(),
            10,
        )?;

        assert!(
            !results.is_empty(),
            "Should return results for criteria {:?}",
            criteria
        );
        assert!(results.len() <= 10, "Should respect limit");

        // Verify sorting is correct
        match criteria {
            SortCriteria::HueDistance => {
                for i in 1..results.len() {
                    assert!(
                        results[i - 1].hue_distance <= results[i].hue_distance,
                        "Results should be sorted by hue distance"
                    );
                }
            }
            SortCriteria::Saturation => {
                for i in 1..results.len() {
                    assert!(
                        results[i - 1].saturation >= results[i].saturation,
                        "Results should be sorted by saturation (descending)"
                    );
                }
            }
            SortCriteria::Lightness => {
                for i in 1..results.len() {
                    assert!(
                        results[i - 1].lightness >= results[i].lightness,
                        "Results should be sorted by lightness (descending)"
                    );
                }
            }
            SortCriteria::Name => {
                for i in 1..results.len() {
                    if let (Some(name1), Some(name2)) = (&results[i - 1].name, &results[i].name) {
                        assert!(name1 <= name2, "Results should be sorted by name");
                    }
                }
            }
        }
    }

    Ok(())
}

/// Test collection integration with filtering
#[test]
fn test_collection_integration_filtering() -> color_rs::Result<()> {
    let input_color: Lch = parse_color_input("#00ff00")?.into_color();

    // Test with restrictive filters
    let restrictive_options = HueAnalysisOptions {
        target_hue: Some(120.0), // Green area
        tolerance: 20.0,
        min_saturation: Some(50.0),
        min_lightness: Some(40.0),
    };

    let filtered_results = analyze_collection_hues(
        &ColorCollectionType::Css,
        &input_color,
        &restrictive_options,
        SortCriteria::HueDistance,
        50,
    )?;

    // Test with permissive filters
    let permissive_options = HueAnalysisOptions {
        target_hue: None,
        tolerance: 180.0,
        min_saturation: None,
        min_lightness: None,
    };

    let unfiltered_results = analyze_collection_hues(
        &ColorCollectionType::Css,
        &input_color,
        &permissive_options,
        SortCriteria::HueDistance,
        50,
    )?;

    // Filtered results should be subset of unfiltered
    assert!(filtered_results.len() <= unfiltered_results.len());

    // All filtered results should meet criteria
    for result in &filtered_results {
        assert!(result.saturation >= 50.0, "Saturation should be >= 50.0");
        assert!(result.lightness >= 40.0, "Lightness should be >= 40.0");

        // Check hue distance from target
        let hue_distance = (result.color.hue - 120.0)
            .abs()
            .min(360.0 - (result.color.hue - 120.0).abs());
        assert!(hue_distance <= 20.0, "Hue distance should be <= 20.0");
    }

    Ok(())
}

/// Test cross-collection consistency
#[test]
fn test_cross_collection_consistency() -> color_rs::Result<()> {
    let input_color: Lch = parse_color_input("#0000ff")?.into_color();
    let options = HueAnalysisOptions {
        target_hue: None,
        tolerance: 15.0,
        min_saturation: None,
        min_lightness: None,
    };

    let collections = [
        ColorCollectionType::Css,
        ColorCollectionType::RalClassic,
        ColorCollectionType::RalDesign,
    ];

    for collection in &collections {
        let results = analyze_collection_hues(
            collection,
            &input_color,
            &options,
            SortCriteria::HueDistance,
            5,
        )?;

        // All collections should return some results
        assert!(
            !results.is_empty(),
            "Collection {:?} should return results",
            collection
        );

        // All results should have consistent structure
        for result in &results {
            assert!(result.name.is_some(), "All results should have names");
            assert!(
                !result.collection.is_empty(),
                "All results should have collection info"
            );
            assert!(
                result.hue_distance >= 0.0,
                "Hue distance should be non-negative"
            );
            assert!(
                result.saturation >= 0.0,
                "Saturation should be non-negative"
            );
            assert!(result.lightness >= 0.0, "Lightness should be non-negative");
            assert!(result.lightness <= 100.0, "Lightness should be <= 100");
        }
    }

    Ok(())
}

/// Test collection loading performance characteristics
#[test]
fn test_collection_loading_performance() -> color_rs::Result<()> {
    use std::time::Instant;

    let collections = [
        ColorCollectionType::Css,
        ColorCollectionType::RalClassic,
        ColorCollectionType::RalDesign,
        ColorCollectionType::All,
    ];

    for collection in &collections {
        let start = Instant::now();
        let colors = load_collection_colors(collection)?;
        let duration = start.elapsed();

        // Collection loading should be reasonably fast
        assert!(
            duration.as_millis() < 1000,
            "Collection {:?} loading took too long: {:?}",
            collection,
            duration
        );

        // Should return reasonable number of colors
        match collection {
            ColorCollectionType::Css => assert!(colors.len() > 100 && colors.len() < 200),
            ColorCollectionType::RalClassic => assert!(colors.len() > 150 && colors.len() < 300),
            ColorCollectionType::RalDesign => assert!(colors.len() > 1000 && colors.len() < 2000),
            ColorCollectionType::All => assert!(colors.len() > 1200),
        }
    }

    Ok(())
}

/// Test metadata extraction accuracy
#[test]
fn test_metadata_extraction() -> color_rs::Result<()> {
    let colors = load_collection_colors(&ColorCollectionType::Css)?;

    // Find specific known colors to test metadata
    let red_colors: Vec<_> = colors
        .iter()
        .filter(|c| {
            c.name
                .as_ref()
                .map_or(false, |n| n.to_lowercase().contains("red"))
        })
        .collect();

    assert!(
        !red_colors.is_empty(),
        "Should find red colors in CSS collection"
    );

    for red_color in &red_colors {
        // Red colors should have hue around 0° (or near 360°)
        let hue = red_color.color.hue;
        let normalized_hue = if hue < 0.0 { hue + 360.0 } else { hue };
        assert!(
            normalized_hue <= 30.0 || normalized_hue >= 330.0,
            "Red color '{}' has unexpected hue: {}°",
            red_color.name.as_ref().unwrap(),
            hue
        );
    }

    Ok(())
}

/// Test error handling in collection loading
#[test]
fn test_collection_loading_error_handling() {
    // This test ensures graceful handling of potential collection loading issues
    // Note: In the current implementation, all collections should load successfully
    // This test serves as a placeholder for future error scenarios

    let collections = [
        ColorCollectionType::Css,
        ColorCollectionType::RalClassic,
        ColorCollectionType::RalDesign,
        ColorCollectionType::All,
    ];

    for collection in &collections {
        let result = load_collection_colors(collection);

        match result {
            Ok(colors) => {
                assert!(
                    !colors.is_empty(),
                    "Collection {:?} should not be empty",
                    collection
                );
            }
            Err(e) => {
                panic!(
                    "Collection {:?} loading failed unexpectedly: {:?}",
                    collection, e
                );
            }
        }
    }
}
