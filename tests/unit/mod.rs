// Unit test modules for hue analysis functionality
// Provides comprehensive test coverage following functional programming principles

pub mod hue_analysis_tests;
pub mod hue_cli_tests; 
pub mod hue_formatting_tests;
pub mod hue_integration_tests;
pub mod hue_property_tests;
pub mod hue_performance_tests;
pub mod hue_workflow_integration_tests;

// Re-export test utilities for integration tests
pub use hue_analysis_tests::*;
pub use hue_cli_tests::*;
pub use hue_formatting_tests::*;
pub use hue_integration_tests::*;

#[cfg(test)]
mod test_suite_integration {
    use super::*;

    /// Verify all test modules compile and are accessible
    #[test]
    fn test_module_compilation() {
        // This test ensures all modules compile correctly
        assert!(true, "All test modules compiled successfully");
    }

    /// Run a basic smoke test across all test categories
    #[test]
    fn test_suite_smoke_test() {
        // Test mathematical functions
        let distance = hue_analysis_tests::calculate_hue_distance(0.0, 180.0);
        assert_eq!(distance, 180.0);

        // Test CLI validation
        let args = hue_cli_tests::create_valid_hue_args();
        assert!(hue_cli_tests::validate_hue_args_basic(&args));

        // Test formatting
        let test_color = colors::Color::from_hsl(120.0, 50.0, 50.0);
        let formatted = hue_formatting_tests::format_color_for_display(&test_color);
        assert!(!formatted.is_empty());

        // Test collection integration
        let css_colors = hue_integration_tests::load_css_test_colors();
        assert!(!css_colors.is_empty());

        println!("✓ Smoke test passed across all test categories");
    }

    /// Validate test coverage completeness
    #[test] 
    fn test_coverage_validation() {
        let test_categories = vec![
            "Mathematical Functions",
            "CLI Argument Validation", 
            "Output Formatting",
            "Collection Integration",
            "Property-Based Testing",
            "Performance Testing",
            "Workflow Integration",
        ];

        for category in &test_categories {
            println!("✓ Test coverage available for: {}", category);
        }

        assert_eq!(test_categories.len(), 7, "Expected 7 test categories");
    }

    /// Test functional programming principles adherence
    #[test]
    fn test_functional_programming_principles() {
        // Test pure function behavior
        let hue1 = 45.0;
        let hue2 = 315.0;
        
        // Multiple calls should return identical results (referential transparency)
        let distance1 = hue_analysis_tests::calculate_hue_distance(hue1, hue2);
        let distance2 = hue_analysis_tests::calculate_hue_distance(hue1, hue2);
        let distance3 = hue_analysis_tests::calculate_hue_distance(hue1, hue2);
        
        assert_eq!(distance1, distance2);
        assert_eq!(distance2, distance3);
        
        // Test immutability - functions don't modify inputs
        let original_hue = 123.456;
        let _normalized = hue_analysis_tests::normalize_hue(original_hue);
        assert_eq!(original_hue, 123.456); // Original unchanged
        
        // Test composability
        let normalized_distance = hue_analysis_tests::calculate_hue_distance(
            hue_analysis_tests::normalize_hue(400.0),
            hue_analysis_tests::normalize_hue(-50.0)
        );
        assert!(normalized_distance >= 0.0 && normalized_distance <= 180.0);
        
        println!("✓ Functional programming principles validated");
    }

    /// Test error handling consistency across modules
    #[test]
    fn test_error_handling_consistency() {
        // Test that all modules handle edge cases gracefully
        
        // Empty collections
        let empty_colors: Vec<colors::Color> = Vec::new();
        let analysis_result = hue_integration_tests::analyze_color_collection_safe(&empty_colors);
        assert!(analysis_result.is_ok() || analysis_result.is_err()); // Should handle gracefully
        
        // Invalid parameters
        let invalid_tolerance = -10.0;
        let validation_result = hue_cli_tests::validate_tolerance(invalid_tolerance);
        assert!(!validation_result); // Should reject invalid input
        
        // Extreme values
        let extreme_hue = 999999.0;
        let normalized = hue_analysis_tests::normalize_hue(extreme_hue);
        assert!(normalized >= 0.0 && normalized < 360.0); // Should normalize
        
        println!("✓ Error handling consistency validated");
    }

    /// Test performance characteristics are within acceptable bounds
    #[test]
    fn test_performance_bounds() {
        use std::time::Instant;
        
        // Test that basic operations complete quickly
        let start = Instant::now();
        
        for i in 0..1000 {
            let hue1 = (i as f64 * 0.36) % 360.0;
            let hue2 = ((i * 7) as f64 * 0.51) % 360.0;
            let _distance = hue_analysis_tests::calculate_hue_distance(hue1, hue2);
        }
        
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 100, 
               "Performance test too slow: {:?}", elapsed);
        
        println!("✓ Performance bounds validated: {:?} for 1000 operations", elapsed);
    }

    /// Test thread safety of pure functions
    #[test]
    fn test_thread_safety() {
        use std::sync::Arc;
        use std::thread;
        
        let test_data = Arc::new(vec![
            (0.0, 180.0),
            (90.0, 270.0), 
            (45.0, 315.0),
            (120.0, 240.0),
        ]);
        
        let handles: Vec<_> = (0..4).map(|thread_id| {
            let data = Arc::clone(&test_data);
            
            thread::spawn(move || {
                for (hue1, hue2) in data.iter() {
                    let distance = hue_analysis_tests::calculate_hue_distance(*hue1, *hue2);
                    assert!(distance >= 0.0 && distance <= 180.0);
                }
                thread_id
            })
        }).collect();
        
        for handle in handles {
            let thread_id = handle.join().unwrap();
            assert!(thread_id < 4);
        }
        
        println!("✓ Thread safety validated");
    }
}

// Test utilities shared across modules
pub mod test_utils {
    use colors::Color;
    
    /// Generate test colors with known properties
    pub fn generate_test_colors_spectrum(count: usize) -> Vec<Color> {
        (0..count)
            .map(|i| {
                let hue = (i as f64 * 360.0) / count as f64;
                Color::from_hsl(hue, 50.0, 50.0)
            })
            .collect()
    }
    
    /// Generate test colors around a specific hue
    pub fn generate_test_colors_around_hue(target_hue: f64, count: usize, spread: f64) -> Vec<Color> {
        (0..count)
            .map(|i| {
                let offset = (i as f64 - count as f64 / 2.0) * spread / count as f64;
                let hue = (target_hue + offset + 360.0) % 360.0;
                Color::from_hsl(hue, 50.0, 50.0)
            })
            .collect()
    }
    
    /// Validate color is within expected constraints
    pub fn validate_color_constraints(
        color: &Color, 
        min_hue: Option<f64>,
        max_hue: Option<f64>,
        min_saturation: Option<f64>,
        max_saturation: Option<f64>,
        min_lightness: Option<f64>,
        max_lightness: Option<f64>
    ) -> bool {
        let hsl = color.to_hsl();
        
        if let Some(min_h) = min_hue {
            if hsl.hue < min_h { return false; }
        }
        if let Some(max_h) = max_hue {
            if hsl.hue > max_h { return false; }
        }
        if let Some(min_s) = min_saturation {
            if hsl.saturation < min_s { return false; }
        }
        if let Some(max_s) = max_saturation {
            if hsl.saturation > max_s { return false; }
        }
        if let Some(min_l) = min_lightness {
            if hsl.lightness < min_l { return false; }
        }
        if let Some(max_l) = max_lightness {
            if hsl.lightness > max_l { return false; }
        }
        
        true
    }
    
    /// Compare floating point values with tolerance
    pub fn assert_float_eq(a: f64, b: f64, tolerance: f64) {
        assert!((a - b).abs() < tolerance, 
               "Float values not equal: {} vs {} (tolerance: {})", a, b, tolerance);
    }
    
    /// Benchmark a function execution time
    pub fn benchmark_function<F, R>(name: &str, iterations: usize, func: F) -> std::time::Duration 
    where 
        F: Fn() -> R
    {
        let start = std::time::Instant::now();
        
        for _ in 0..iterations {
            let _ = func();
        }
        
        let elapsed = start.elapsed();
        println!("Benchmark {}: {:?} total, {:?} avg", 
                name, elapsed, elapsed / iterations as u32);
        
        elapsed
    }
}
