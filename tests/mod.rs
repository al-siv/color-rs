// Integration point for comprehensive test suite
// This module organizes and provides access to all test categories

pub mod unit;

// Re-export unit test modules for easy access
pub use unit::*;

#[cfg(test)]
mod test_suite_runner {
    use super::*;

    /// Master test to verify complete test suite functionality
    #[test]
    fn run_comprehensive_test_suite() {
        println!("🚀 Starting comprehensive hue analysis test suite...");

        // Test mathematical foundations
        println!("📐 Testing mathematical functions...");
        test_mathematical_foundations();

        // Test CLI interface
        println!("⌨️  Testing CLI interface...");
        test_cli_interface();

        // Test output formatting
        println!("📄 Testing output formatting...");
        test_output_formatting();

        // Test collection integration
        println!("🎨 Testing collection integration...");
        test_collection_integration();

        // Test performance characteristics
        println!("⚡ Testing performance...");
        test_performance_suite();

        // Test workflow integration
        println!("🔄 Testing workflow integration...");
        test_workflow_integration();

        println!("✅ Comprehensive test suite completed successfully!");
    }

    fn test_mathematical_foundations() {
        // Core mathematical function validation
        assert_eq!(
            unit::hue_analysis_tests::calculate_hue_distance(0.0, 180.0),
            180.0
        );
        assert_eq!(
            unit::hue_analysis_tests::calculate_hue_distance(350.0, 10.0),
            20.0
        );

        // Hue normalization
        assert_eq!(unit::hue_analysis_tests::normalize_hue(360.0), 0.0);
        assert_eq!(unit::hue_analysis_tests::normalize_hue(-90.0), 270.0);

        // Symmetry properties
        let hue1 = 45.0;
        let hue2 = 315.0;
        assert_eq!(
            unit::hue_analysis_tests::calculate_hue_distance(hue1, hue2),
            unit::hue_analysis_tests::calculate_hue_distance(hue2, hue1)
        );

        println!("  ✓ Mathematical foundations validated");
    }

    fn test_cli_interface() {
        // Valid argument creation
        let args = unit::hue_cli_tests::create_valid_hue_args();
        assert!(unit::hue_cli_tests::validate_hue_args_basic(&args));

        // Invalid argument handling
        assert!(!unit::hue_cli_tests::validate_tolerance(-10.0));
        assert!(!unit::hue_cli_tests::validate_hue_range(400.0));

        // Edge case handling
        let edge_args = unit::hue_cli_tests::create_edge_case_args();
        let normalized_args = unit::hue_cli_tests::normalize_hue_args(edge_args);
        assert!(unit::hue_cli_tests::validate_hue_args_basic(
            &normalized_args
        ));

        println!("  ✓ CLI interface validated");
    }

    fn test_output_formatting() {
        use colors::Color;

        // Color formatting
        let test_color = Color::from_hsl(120.0, 50.0, 50.0);
        let formatted = unit::hue_formatting_tests::format_color_for_display(&test_color);
        assert!(!formatted.is_empty());

        // Collection formatting
        let colors = vec![test_color.clone()];
        let collection_formatted = unit::hue_formatting_tests::format_color_collection(&colors);
        assert!(collection_formatted.contains("120"));

        // Export formatting
        let yaml_output = unit::hue_formatting_tests::format_as_yaml(&colors);
        assert!(yaml_output.contains("hue:"));

        println!("  ✓ Output formatting validated");
    }

    fn test_collection_integration() {
        // Collection loading
        let css_colors = unit::hue_integration_tests::load_css_test_colors();
        assert!(!css_colors.is_empty());

        // Collection filtering
        let filtered =
            unit::hue_integration_tests::filter_collection_by_hue(&css_colors, 180.0, 30.0);
        assert!(filtered.len() <= css_colors.len());

        // Cross-collection consistency
        let consistency_check =
            unit::hue_integration_tests::validate_cross_collection_consistency();
        assert!(consistency_check);

        println!("  ✓ Collection integration validated");
    }

    fn test_performance_suite() {
        use std::time::Instant;

        // Basic operation performance
        let start = Instant::now();
        for i in 0..1000 {
            let hue1 = (i as f64 * 0.36) % 360.0;
            let hue2 = ((i * 7) as f64 * 0.51) % 360.0;
            let _distance = unit::hue_analysis_tests::calculate_hue_distance(hue1, hue2);
        }
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 100, "Basic operations too slow");

        // Collection filtering performance
        let large_collection = unit::test_utils::generate_test_colors_spectrum(10000);
        let filter_start = Instant::now();
        let _filtered =
            unit::hue_integration_tests::filter_collection_by_hue(&large_collection, 120.0, 30.0);
        let filter_elapsed = filter_start.elapsed();
        assert!(
            filter_elapsed.as_millis() < 1000,
            "Collection filtering too slow"
        );

        println!("  ✓ Performance characteristics validated");
    }

    fn test_workflow_integration() {
        // End-to-end workflow simulation
        let test_colors = unit::test_utils::generate_test_colors_spectrum(100);
        let target_hue = 240.0;
        let tolerance = 45.0;

        // Filter colors
        let filtered = unit::hue_integration_tests::filter_collection_by_hue(
            &test_colors,
            target_hue,
            tolerance,
        );

        // Validate results
        for color in &filtered {
            let hue = color.to_hsl().hue;
            let distance = unit::hue_analysis_tests::calculate_hue_distance(hue, target_hue);
            assert!(distance <= tolerance, "Filtered color outside tolerance");
        }

        // Format output
        let formatted_output = unit::hue_formatting_tests::format_color_collection(&filtered);
        assert!(!formatted_output.is_empty());

        println!("  ✓ Workflow integration validated");
    }

    /// Test runner for specific test categories
    #[test]
    fn run_category_tests() {
        println!("🔬 Running category-specific tests...");

        // Mathematical tests
        run_mathematical_category_tests();

        // CLI tests
        run_cli_category_tests();

        // Formatting tests
        run_formatting_category_tests();

        // Integration tests
        run_integration_category_tests();

        println!("✅ All category tests completed!");
    }

    fn run_mathematical_category_tests() {
        // Distance calculation tests
        assert_eq!(
            unit::hue_analysis_tests::calculate_hue_distance(0.0, 0.0),
            0.0
        );
        assert_eq!(
            unit::hue_analysis_tests::calculate_hue_distance(0.0, 180.0),
            180.0
        );
        assert_eq!(
            unit::hue_analysis_tests::calculate_hue_distance(350.0, 10.0),
            20.0
        );

        // Normalization tests
        assert_eq!(unit::hue_analysis_tests::normalize_hue(720.0), 0.0);
        assert_eq!(unit::hue_analysis_tests::normalize_hue(-180.0), 180.0);

        // Criteria tests
        assert!(unit::hue_analysis_tests::meets_hue_criteria(
            120.0, 120.0, 0.0
        ));
        assert!(unit::hue_analysis_tests::meets_hue_criteria(
            110.0, 120.0, 15.0
        ));
        assert!(!unit::hue_analysis_tests::meets_hue_criteria(
            90.0, 120.0, 15.0
        ));

        println!("  ✓ Mathematical category tests passed");
    }

    fn run_cli_category_tests() {
        // Argument validation
        let valid_args = unit::hue_cli_tests::create_valid_hue_args();
        assert!(unit::hue_cli_tests::validate_hue_args_basic(&valid_args));

        // Parameter validation
        assert!(unit::hue_cli_tests::validate_tolerance(30.0));
        assert!(!unit::hue_cli_tests::validate_tolerance(-5.0));
        assert!(unit::hue_cli_tests::validate_hue_range(359.9));
        assert!(!unit::hue_cli_tests::validate_hue_range(360.0));

        // Edge case handling
        let edge_args = unit::hue_cli_tests::create_edge_case_args();
        let normalized = unit::hue_cli_tests::normalize_hue_args(edge_args);
        assert!(unit::hue_cli_tests::validate_hue_args_basic(&normalized));

        println!("  ✓ CLI category tests passed");
    }

    fn run_formatting_category_tests() {
        use colors::Color;

        // Single color formatting
        let color = Color::from_hsl(240.0, 75.0, 50.0);
        let formatted = unit::hue_formatting_tests::format_color_for_display(&color);
        assert!(formatted.contains("240"));
        assert!(formatted.contains("75"));
        assert!(formatted.contains("50"));

        // Collection formatting
        let colors = vec![color.clone()];
        let collection_format = unit::hue_formatting_tests::format_color_collection(&colors);
        assert!(!collection_format.is_empty());

        // Export formats
        let yaml = unit::hue_formatting_tests::format_as_yaml(&colors);
        let json = unit::hue_formatting_tests::format_as_json(&colors);
        assert!(yaml.contains("hue:"));
        assert!(json.contains("\"hue\""));

        println!("  ✓ Formatting category tests passed");
    }

    fn run_integration_category_tests() {
        // Collection loading
        let css_colors = unit::hue_integration_tests::load_css_test_colors();
        let ral_colors = unit::hue_integration_tests::load_ral_test_colors();
        assert!(!css_colors.is_empty());
        assert!(!ral_colors.is_empty());

        // Filtering
        let filtered_css =
            unit::hue_integration_tests::filter_collection_by_hue(&css_colors, 120.0, 30.0);
        let filtered_ral =
            unit::hue_integration_tests::filter_collection_by_hue(&ral_colors, 120.0, 30.0);
        assert!(filtered_css.len() <= css_colors.len());
        assert!(filtered_ral.len() <= ral_colors.len());

        // Cross-collection validation
        assert!(unit::hue_integration_tests::validate_cross_collection_consistency());

        println!("  ✓ Integration category tests passed");
    }

    /// Benchmark runner for performance validation
    #[test]
    fn run_performance_benchmarks() {
        println!("📊 Running performance benchmarks...");

        // Mathematical function benchmarks
        unit::test_utils::benchmark_function("hue_distance_calculation", 10000, || {
            unit::hue_analysis_tests::calculate_hue_distance(123.45, 234.56)
        });

        unit::test_utils::benchmark_function("hue_normalization", 10000, || {
            unit::hue_analysis_tests::normalize_hue(456.78)
        });

        // Collection processing benchmarks
        let test_collection = unit::test_utils::generate_test_colors_spectrum(1000);
        unit::test_utils::benchmark_function("collection_filtering", 100, || {
            unit::hue_integration_tests::filter_collection_by_hue(&test_collection, 180.0, 45.0)
        });

        // Formatting benchmarks
        let small_collection = unit::test_utils::generate_test_colors_spectrum(10);
        unit::test_utils::benchmark_function("yaml_formatting", 1000, || {
            unit::hue_formatting_tests::format_as_yaml(&small_collection)
        });

        println!("✅ Performance benchmarks completed!");
    }

    /// Stress test runner for edge cases and limits
    #[test]
    #[ignore] // Only run manually for stress testing
    fn run_stress_tests() {
        println!("💪 Running stress tests...");

        // Large collection stress test
        let large_collection = unit::test_utils::generate_test_colors_spectrum(100000);
        let start = std::time::Instant::now();
        let _filtered =
            unit::hue_integration_tests::filter_collection_by_hue(&large_collection, 0.0, 1.0);
        let elapsed = start.elapsed();
        assert!(
            elapsed.as_secs() < 10,
            "Large collection stress test too slow"
        );

        // Extreme parameter stress test
        for extreme_hue in vec![-999999.0, 999999.0] {
            let normalized = unit::hue_analysis_tests::normalize_hue(extreme_hue);
            assert!(normalized >= 0.0 && normalized < 360.0);
        }

        // Memory stress test
        for _ in 0..1000 {
            let temp_collection = unit::test_utils::generate_test_colors_spectrum(1000);
            let _filtered = unit::hue_integration_tests::filter_collection_by_hue(
                &temp_collection,
                180.0,
                30.0,
            );
            // Collection should be dropped and memory freed
        }

        println!("✅ Stress tests completed!");
    }
}
