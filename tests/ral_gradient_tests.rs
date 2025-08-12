// Sprint 0.19.4 - RAL Color Resolution Test Cases
// This file contains test cases to validate RAL color support in gradient command

use color_rs::{ColorRs, cli::GradientArgs};

#[cfg(test)]
mod ral_gradient_tests {
    use super::*;

    /// Test RAL Classic codes in gradient command
    #[test]
    fn test_ral_classic_gradient() {
        let color_rs = ColorRs::new();

        // Test the original failing case
        let args = GradientArgs {
            start_color: "RAL 5019".to_string(), // Capri blue
            end_color: "RAL 1004".to_string(),   // Golden yellow
            start_position: 0,
            end_position: 100,
            ease_in: 0.65,
            ease_out: 0.35,
            svg: None,
            png: None,
            no_legend: false,
            width: 1000,
            step: None,
            stops: 5,
            stops_simple: false,
            output_format: None,
            output_file: None,
            func_filter: None,
            vectorized_text: false,
        };

        // This should NOT panic or return an error
        let result = color_rs.generate_gradient(args);
        assert!(
            result.is_ok(),
            "RAL Classic gradient should work: {result:?}"
        );
    }

    /// Test RAL Classic with standard colors
    #[test]
    fn test_mixed_ral_standard_gradient() {
        let color_rs = ColorRs::new();

        let test_cases = vec![
            ("RAL 3020", "blue"),         // Traffic red to blue
            ("red", "RAL 5005"),          // Red to signal blue
            ("RAL 1003", "#00FF00"),      // Signal yellow to green hex
            ("rgb(255,0,0)", "RAL 6029"), // RGB red to mint green
        ];

        for (start, end) in test_cases {
            let args = GradientArgs {
                start_color: start.to_string(),
                end_color: end.to_string(),
                start_position: 0,
                end_position: 100,
                ease_in: 0.65,
                ease_out: 0.35,
                svg: None,
                png: None,
                no_legend: false,
                width: 1000,
                step: None,
                stops: 5,
                stops_simple: false,
                output_format: None,
                output_file: None,
                func_filter: None,
                vectorized_text: false,
            };

            let result = color_rs.generate_gradient(args);
            assert!(
                result.is_ok(),
                "Mixed gradient {start}->{end} should work: {result:?}"
            );
        }
    }

    /// Test RAL Design System+ codes
    #[test]
    fn test_ral_design_gradient() {
        let color_rs = ColorRs::new();

        let args = GradientArgs {
            start_color: "RAL 010 40 30".to_string(), // Deep red
            end_color: "RAL 270 30 40".to_string(),   // Deep purple
            start_position: 0,
            end_position: 100,
            ease_in: 0.65,
            ease_out: 0.35,
            svg: None,
            png: None,
            no_legend: false,
            width: 1000,
            step: None,
            stops: 5,
            stops_simple: false,
            output_format: None,
            output_file: None,
            func_filter: None,
            vectorized_text: false,
        };

        let result = color_rs.generate_gradient(args);
        assert!(
            result.is_ok(),
            "RAL Design gradient should work: {result:?}"
        );
    }

    /// Test that error messages are consistent between color and gradient commands
    #[test]
    fn test_consistent_error_handling() {
        let color_rs = ColorRs::new();

        // Test invalid RAL code
        let invalid_args = GradientArgs {
            start_color: "RAL 9999".to_string(), // Invalid RAL code
            end_color: "blue".to_string(),
            start_position: 0,
            end_position: 100,
            ease_in: 0.65,
            ease_out: 0.35,
            svg: None,
            png: None,
            no_legend: false,
            width: 1000,
            step: None,
            stops: 5,
            stops_simple: false,
            output_format: None,
            output_file: None,
            func_filter: None,
            vectorized_text: false,
        };

        let result = color_rs.generate_gradient(invalid_args);
        assert!(result.is_err(), "Invalid RAL code should return error");

        // Error message should be helpful and consistent
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("RAL 9999") || error_msg.contains("color"),
            "Error message should mention the problematic input: {error_msg}"
        );
    }

    /// Performance test - ensure no significant degradation
    #[test]
    fn test_performance_impact() {
        let color_rs = ColorRs::new();
        // Run multiple batches and take median to reduce variance-based flakes
        const BATCHES: usize = 3;
        const ITER_PER_BATCH: usize = 10;
        let mut times: Vec<u128> = Vec::with_capacity(BATCHES);
        for _ in 0..BATCHES {
            let start_time = std::time::Instant::now();
            for _ in 0..ITER_PER_BATCH {
                let args = GradientArgs {
                    start_color: "RAL 3020".to_string(),
                    end_color: "RAL 5005".to_string(),
                    start_position: 0,
                    end_position: 100,
                    ease_in: 0.65,
                    ease_out: 0.35,
                    svg: None,
                    png: None,
                    no_legend: false,
                    width: 1000,
                    step: None,
                    stops: 10,
                    stops_simple: false,
                    output_format: None,
                    output_file: None,
                    func_filter: None,
                    vectorized_text: false,
                };
                let result = color_rs.generate_gradient(args);
                assert!(result.is_ok());
            }
            times.push(start_time.elapsed().as_millis());
        }
        times.sort_unstable();
        let median = times[times.len()/2];
        println!("Median time over {BATCHES} batches of {ITER_PER_BATCH} gradients: {median}ms (all: {:?})", times);
        // Threshold rationale (updated): previously single-run upper bound 8000ms; using median allows slightly stricter guard.
        // Keep same ceiling to avoid false positives while reducing flakes due to outlier batch.
        let threshold_ms = 8000u128;
        assert!(
            median < threshold_ms,
            "Performance regression detected (median {median}ms > {threshold_ms}ms, batch times {:?})",
            times
        );
    }

    /// Test comprehensive RAL color support
    #[test]
    fn test_comprehensive_ral_support() {
        let color_rs = ColorRs::new();

        // Sample of RAL Classic colors from different groups
        let ral_colors = vec![
            "RAL 1000", // Yellow group
            "RAL 2004", // Orange group
            "RAL 3020", // Red group
            "RAL 4007", // Violet group
            "RAL 5019", // Blue group
            "RAL 6029", // Green group
            "RAL 7035", // Gray group
            "RAL 8017", // Brown group
            "RAL 9003", // White group
        ];

        // Test each color in a gradient
        for ral_color in ral_colors {
            let args = GradientArgs {
                start_color: ral_color.to_string(),
                end_color: "white".to_string(),
                start_position: 0,
                end_position: 100,
                ease_in: 0.65,
                ease_out: 0.35,
                svg: None,
                png: None,
                no_legend: false,
                width: 1000,
                step: None,
                stops: 5,
                stops_simple: false,
                output_format: None,
                output_file: None,
                func_filter: None,
                vectorized_text: false,
            };

            let result = color_rs.generate_gradient(args);
            assert!(
                result.is_ok(),
                "RAL color {ral_color} should work in gradient: {result:?}"
            );
        }
    }
}

/// Manual test function for CLI validation
/// Run with: cargo test --test ral_gradient_manual -- --nocapture
#[cfg(test)]
mod manual_tests {
    #[test]
    #[ignore] // Use --ignored to run
    fn manual_cli_test_instructions() {
        println!("\n=== Manual CLI Test Cases ===");
        println!("Run these commands to validate RAL gradient support:");
        println!();
        println!("# Original failing case:");
        println!("cargo run --release -- gradient \"RAL 5019\" \"RAL 1004\"");
        println!();
        println!("# Mixed RAL and standard colors:");
        println!("cargo run --release -- gradient \"RAL 3020\" \"blue\"");
        println!("cargo run --release -- gradient \"red\" \"RAL 5005\"");
        println!();
        println!("# RAL Design System+ colors:");
        println!("cargo run --release -- gradient \"RAL 010 40 30\" \"RAL 270 30 40\"");
        println!();
        println!("# With SVG output:");
        println!(
            "cargo run --release -- gradient \"RAL 5019\" \"RAL 1004\" --svg test-ral-gradient.svg"
        );
        println!();
        println!("# Extended formats (should also work):");
        println!("cargo run --release -- gradient \"lab(35,0,-29)\" \"lch(90,10,120)\"");
        println!();
        println!("All commands should execute successfully without errors.");
    }
}
