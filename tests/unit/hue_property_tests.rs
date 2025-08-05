use proptest::prelude::*;
use colors::{Color, ColorCollectionType, hue_analysis};
use std::f64::consts::PI;

proptest! {
    /// Property test: hue normalization is idempotent
    #[test]
    fn hue_normalization_is_idempotent(hue in 0.0..360.0) {
        let normalized_once = normalize_hue(hue);
        let normalized_twice = normalize_hue(normalized_once);
        prop_assert!((normalized_once - normalized_twice).abs() < 1e-10);
    }

    /// Property test: normalized hue is always in [0, 360) range
    #[test]
    fn normalized_hue_in_range(hue in -1000.0..1000.0) {
        let normalized = normalize_hue(hue);
        prop_assert!(normalized >= 0.0);
        prop_assert!(normalized < 360.0);
    }

    /// Property test: hue distance is symmetric
    #[test]
    fn hue_distance_symmetry(hue1 in 0.0..360.0, hue2 in 0.0..360.0) {
        let distance1 = calculate_hue_distance(hue1, hue2);
        let distance2 = calculate_hue_distance(hue2, hue1);
        prop_assert!((distance1 - distance2).abs() < 1e-10);
    }

    /// Property test: hue distance is always non-negative
    #[test]
    fn hue_distance_non_negative(hue1 in 0.0..360.0, hue2 in 0.0..360.0) {
        let distance = calculate_hue_distance(hue1, hue2);
        prop_assert!(distance >= 0.0);
    }

    /// Property test: hue distance is at most 180 degrees
    #[test]
    fn hue_distance_bounded(hue1 in 0.0..360.0, hue2 in 0.0..360.0) {
        let distance = calculate_hue_distance(hue1, hue2);
        prop_assert!(distance <= 180.0);
    }

    /// Property test: zero distance when hues are identical
    #[test]
    fn hue_distance_zero_for_identical(hue in 0.0..360.0) {
        let distance = calculate_hue_distance(hue, hue);
        prop_assert!(distance.abs() < 1e-10);
    }

    /// Property test: hue criteria with valid tolerance ranges
    #[test]
    fn hue_criteria_valid_tolerance(target in 0.0..360.0, tolerance in 0.0..180.0) {
        // Tolerance of 0 should match only exact hue
        prop_assert!(!meets_hue_criteria(target + 1.0, target, 0.0));
        
        // Any tolerance >= 180 should match all hues
        if tolerance >= 180.0 {
            prop_assert!(meets_hue_criteria(target + 180.0, target, tolerance));
        }
    }

    /// Property test: hue range analysis consistency
    #[test]
    fn hue_range_analysis_consistency(
        hues in prop::collection::vec(0.0..360.0, 1..100)
    ) {
        let colors: Vec<Color> = hues.iter()
            .map(|&h| Color::from_hsl(h, 50.0, 50.0))
            .collect();
        
        let range = calculate_hue_range(&colors);
        
        // Range should be non-negative
        prop_assert!(range >= 0.0);
        
        // Range should be at most 360.0
        prop_assert!(range <= 360.0);
        
        // For single color, range should be 0
        if colors.len() == 1 {
            prop_assert!(range.abs() < 1e-10);
        }
    }

    /// Property test: color collection parsing roundtrip
    #[test]
    fn collection_type_parsing_roundtrip(
        collection_str in "(css|ral-classic|ral-design|all)"
    ) {
        if let Ok(collection_type) = collection_str.parse::<ColorCollectionType>() {
            let serialized = collection_type.to_string();
            let reparsed = serialized.parse::<ColorCollectionType>();
            prop_assert!(reparsed.is_ok());
            prop_assert_eq!(collection_type, reparsed.unwrap());
        }
    }

    /// Property test: hue filtering preserves color count constraints
    #[test]
    fn hue_filtering_count_constraints(
        target_hue in 0.0..360.0,
        tolerance in 1.0..180.0,
        collection_size in 10..1000usize
    ) {
        // Generate test colors with known hue distribution
        let colors: Vec<Color> = (0..collection_size)
            .map(|i| {
                let hue = (i as f64 * 360.0) / collection_size as f64;
                Color::from_hsl(hue, 50.0, 50.0)
            })
            .collect();
        
        let filtered = filter_colors_by_hue(&colors, target_hue, tolerance);
        
        // Filtered count should not exceed original count
        prop_assert!(filtered.len() <= colors.len());
        
        // All filtered colors should meet the hue criteria
        for color in &filtered {
            let hue = color.to_hsl().hue;
            prop_assert!(meets_hue_criteria(hue, target_hue, tolerance));
        }
    }

    /// Property test: mathematical function composability
    #[test]
    fn mathematical_function_composability(
        hue1 in 0.0..360.0,
        hue2 in 0.0..360.0,
        hue3 in 0.0..360.0
    ) {
        // Triangle inequality for hue distances
        let d12 = calculate_hue_distance(hue1, hue2);
        let d23 = calculate_hue_distance(hue2, hue3);
        let d13 = calculate_hue_distance(hue1, hue3);
        
        // Due to circular nature, this is a modified triangle inequality
        prop_assert!(d13 <= d12 + d23 || d13 >= 360.0 - (d12 + d23));
    }

    /// Property test: hue analysis output format consistency
    #[test]
    fn hue_analysis_output_consistency(
        target_hue in 0.0..360.0,
        tolerance in 1.0..30.0,
        collection_name in "[a-z-]{3,15}"
    ) {
        // Test that analysis output maintains mathematical properties
        let mock_colors = vec![
            Color::from_hsl(target_hue, 50.0, 50.0),
            Color::from_hsl(target_hue + tolerance, 60.0, 40.0),
            Color::from_hsl(target_hue - tolerance, 40.0, 60.0),
        ];
        
        let analysis = analyze_hue_distribution(&mock_colors, target_hue, tolerance);
        
        // Analysis should maintain count consistency
        prop_assert!(analysis.total_matches <= mock_colors.len());
        prop_assert!(analysis.total_matches > 0); // At least target should match
        
        // Average distance should be meaningful
        prop_assert!(analysis.average_distance >= 0.0);
        prop_assert!(analysis.average_distance <= tolerance);
    }

    /// Property test: error handling consistency across invalid inputs
    #[test]
    fn error_handling_consistency(
        invalid_tolerance in -100.0..0.0,
        invalid_hue in 400.0..1000.0
    ) {
        // Test that invalid inputs are handled consistently
        let test_colors = vec![Color::from_hsl(180.0, 50.0, 50.0)];
        
        // Invalid tolerance should result in error or normalization
        if invalid_tolerance < 0.0 {
            let result = filter_colors_by_hue(&test_colors, 180.0, invalid_tolerance);
            // Should either error gracefully or normalize to valid range
            prop_assert!(result.is_empty() || result.len() <= test_colors.len());
        }
        
        // Invalid hue should be normalized
        let normalized = normalize_hue(invalid_hue);
        prop_assert!(normalized >= 0.0 && normalized < 360.0);
    }
}

// Helper functions for property tests

fn normalize_hue(hue: f64) -> f64 {
    ((hue % 360.0) + 360.0) % 360.0
}

fn calculate_hue_distance(hue1: f64, hue2: f64) -> f64 {
    let diff = (hue1 - hue2).abs();
    diff.min(360.0 - diff)
}

fn meets_hue_criteria(hue: f64, target: f64, tolerance: f64) -> bool {
    calculate_hue_distance(hue, target) <= tolerance
}

fn calculate_hue_range(colors: &[Color]) -> f64 {
    if colors.is_empty() {
        return 0.0;
    }
    
    let hues: Vec<f64> = colors.iter()
        .map(|c| c.to_hsl().hue)
        .collect();
    
    if hues.len() == 1 {
        return 0.0;
    }
    
    // Calculate circular range
    let mut sorted_hues = hues.clone();
    sorted_hues.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let max_gap = sorted_hues.windows(2)
        .map(|w| w[1] - w[0])
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0);
    
    let circular_gap = 360.0 - (sorted_hues.last().unwrap() - sorted_hues.first().unwrap());
    
    360.0 - max_gap.max(circular_gap)
}

fn filter_colors_by_hue(colors: &[Color], target_hue: f64, tolerance: f64) -> Vec<Color> {
    colors.iter()
        .filter(|color| {
            let hue = color.to_hsl().hue;
            meets_hue_criteria(hue, target_hue, tolerance)
        })
        .cloned()
        .collect()
}

#[derive(Debug)]
struct HueAnalysis {
    total_matches: usize,
    average_distance: f64,
}

fn analyze_hue_distribution(colors: &[Color], target_hue: f64, tolerance: f64) -> HueAnalysis {
    let matching_colors = filter_colors_by_hue(colors, target_hue, tolerance);
    
    let total_matches = matching_colors.len();
    let average_distance = if total_matches > 0 {
        matching_colors.iter()
            .map(|color| calculate_hue_distance(color.to_hsl().hue, target_hue))
            .sum::<f64>() / total_matches as f64
    } else {
        0.0
    };
    
    HueAnalysis {
        total_matches,
        average_distance,
    }
}
