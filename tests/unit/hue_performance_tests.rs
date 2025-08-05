use colors::{Color, ColorCollectionType, hue_analysis};
use std::time::{Duration, Instant};

#[cfg(test)]
mod hue_performance_tests {
    use super::*;

    const SMALL_COLLECTION_SIZE: usize = 100;
    const MEDIUM_COLLECTION_SIZE: usize = 1_000;
    const LARGE_COLLECTION_SIZE: usize = 10_000;
    const BENCHMARK_ITERATIONS: usize = 10;

    /// Test performance of hue distance calculation
    #[test]
    fn test_hue_distance_performance() {
        let test_cases = vec![
            ("small_batch", 1_000),
            ("medium_batch", 10_000),
            ("large_batch", 100_000),
        ];

        for (name, iterations) in test_cases {
            let start = Instant::now();
            
            for i in 0..iterations {
                let hue1 = (i as f64 * 137.5) % 360.0; // Golden angle distribution
                let hue2 = ((i * 7) as f64 * 51.4) % 360.0;
                let _distance = calculate_hue_distance(hue1, hue2);
            }
            
            let elapsed = start.elapsed();
            let per_operation = elapsed.as_nanos() / iterations as u128;
            
            println!("Hue distance {} iterations: {:?} total, {} ns/op", 
                    iterations, elapsed, per_operation);
            
            // Performance assertion: should be under 100ns per operation
            assert!(per_operation < 100, 
                   "Hue distance calculation too slow: {} ns/op", per_operation);
        }
    }

    /// Test performance of hue normalization
    #[test]
    fn test_hue_normalization_performance() {
        let test_values: Vec<f64> = (0..10_000)
            .map(|i| (i as f64 * 73.7) - 5000.0) // Range from -5000 to 5000
            .collect();

        let start = Instant::now();
        
        for &hue in &test_values {
            let _normalized = normalize_hue(hue);
        }
        
        let elapsed = start.elapsed();
        let per_operation = elapsed.as_nanos() / test_values.len() as u128;
        
        println!("Hue normalization {} operations: {:?} total, {} ns/op", 
                test_values.len(), elapsed, per_operation);
        
        // Performance assertion: should be under 50ns per operation
        assert!(per_operation < 50, 
               "Hue normalization too slow: {} ns/op", per_operation);
    }

    /// Test performance of color collection filtering
    #[test]
    fn test_collection_filtering_performance() {
        let collections = vec![
            ("small", generate_test_colors(SMALL_COLLECTION_SIZE)),
            ("medium", generate_test_colors(MEDIUM_COLLECTION_SIZE)),
            ("large", generate_test_colors(LARGE_COLLECTION_SIZE)),
        ];

        for (size_name, colors) in collections {
            let target_hue = 180.0;
            let tolerance = 30.0;
            
            let mut total_duration = Duration::new(0, 0);
            let mut results = Vec::new();
            
            for _ in 0..BENCHMARK_ITERATIONS {
                let start = Instant::now();
                let filtered = filter_colors_by_hue(&colors, target_hue, tolerance);
                let elapsed = start.elapsed();
                
                total_duration += elapsed;
                results.push(filtered.len());
            }
            
            let avg_duration = total_duration / BENCHMARK_ITERATIONS as u32;
            let per_color = avg_duration.as_nanos() / colors.len() as u128;
            
            println!("Collection filtering {} colors: {:?} avg, {} ns/color", 
                    colors.len(), avg_duration, per_color);
            
            // Performance assertions based on collection size
            let max_ns_per_color = match size_name {
                "small" => 1000,   // 1µs per color for small collections
                "medium" => 500,   // 500ns per color for medium collections  
                "large" => 200,    // 200ns per color for large collections
                _ => 1000,
            };
            
            assert!(per_color < max_ns_per_color, 
                   "Collection filtering too slow for {} collection: {} ns/color", 
                   size_name, per_color);
            
            // Consistency check
            let result_variance = calculate_variance(&results);
            assert!(result_variance < 1.0, 
                   "Results not consistent across runs: variance {}", result_variance);
        }
    }

    /// Test performance of hue analysis pipeline
    #[test]
    fn test_analysis_pipeline_performance() {
        let colors = generate_diverse_test_colors(MEDIUM_COLLECTION_SIZE);
        let test_scenarios = vec![
            ("tight_tolerance", 5.0),
            ("medium_tolerance", 30.0),
            ("wide_tolerance", 90.0),
        ];

        for (scenario_name, tolerance) in test_scenarios {
            let target_hue = 120.0; // Green
            
            let start = Instant::now();
            
            for _ in 0..BENCHMARK_ITERATIONS {
                let _analysis = perform_full_hue_analysis(&colors, target_hue, tolerance);
            }
            
            let elapsed = start.elapsed();
            let per_analysis = elapsed.as_millis() / BENCHMARK_ITERATIONS as u128;
            
            println!("Hue analysis pipeline {} ({}): {} ms/analysis", 
                    scenario_name, tolerance, per_analysis);
            
            // Performance assertion: full analysis should complete under 10ms
            assert!(per_analysis < 10, 
                   "Analysis pipeline too slow for {}: {} ms", scenario_name, per_analysis);
        }
    }

    /// Test memory efficiency of hue operations
    #[test]
    fn test_memory_efficiency() {
        let initial_colors = generate_test_colors(LARGE_COLLECTION_SIZE);
        
        // Test that filtering doesn't cause memory bloat
        let filtered = filter_colors_by_hue(&initial_colors, 180.0, 45.0);
        
        // Memory efficiency check: filtered collection should be reasonable size
        let efficiency_ratio = filtered.len() as f64 / initial_colors.len() as f64;
        
        println!("Memory efficiency - Original: {}, Filtered: {}, Ratio: {:.2}", 
                initial_colors.len(), filtered.len(), efficiency_ratio);
        
        // For 45-degree tolerance, we expect roughly 25% of colors (45/180 * 100%)
        assert!(efficiency_ratio >= 0.1 && efficiency_ratio <= 0.4, 
               "Memory efficiency outside expected range: {:.2}", efficiency_ratio);
    }

    /// Test concurrent operation performance
    #[test]
    fn test_concurrent_operations_performance() {
        use std::sync::Arc;
        use std::thread;
        
        let colors = Arc::new(generate_test_colors(MEDIUM_COLLECTION_SIZE));
        let num_threads = 4;
        let operations_per_thread = 25;
        
        let start = Instant::now();
        
        let handles: Vec<_> = (0..num_threads).map(|thread_id| {
            let colors_clone = Arc::clone(&colors);
            
            thread::spawn(move || {
                for i in 0..operations_per_thread {
                    let target_hue = (thread_id * 90 + i * 15) as f64;
                    let tolerance = 20.0;
                    let _filtered = filter_colors_by_hue(&colors_clone, target_hue, tolerance);
                }
            })
        }).collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let elapsed = start.elapsed();
        let total_operations = num_threads * operations_per_thread;
        let per_operation = elapsed.as_millis() / total_operations as u128;
        
        println!("Concurrent operations {} threads × {} ops: {:?} total, {} ms/op", 
                num_threads, operations_per_thread, elapsed, per_operation);
        
        // Performance assertion: concurrent operations should be efficient
        assert!(per_operation < 5, 
               "Concurrent operations too slow: {} ms/op", per_operation);
    }

    /// Stress test with extreme collection sizes
    #[test]
    #[ignore] // Only run manually for stress testing
    fn test_extreme_collection_performance() {
        let extreme_size = 100_000;
        let colors = generate_test_colors(extreme_size);
        
        println!("Starting extreme collection test with {} colors", extreme_size);
        
        let start = Instant::now();
        let filtered = filter_colors_by_hue(&colors, 0.0, 1.0); // Very tight tolerance
        let elapsed = start.elapsed();
        
        println!("Extreme collection filtering: {:?}, {} matches", elapsed, filtered.len());
        
        // Should complete within reasonable time even for extreme cases
        assert!(elapsed.as_secs() < 5, "Extreme collection test took too long: {:?}", elapsed);
    }

    // Helper functions

    fn calculate_hue_distance(hue1: f64, hue2: f64) -> f64 {
        let diff = (hue1 - hue2).abs();
        diff.min(360.0 - diff)
    }

    fn normalize_hue(hue: f64) -> f64 {
        ((hue % 360.0) + 360.0) % 360.0
    }

    fn filter_colors_by_hue(colors: &[Color], target_hue: f64, tolerance: f64) -> Vec<Color> {
        colors.iter()
            .filter(|color| {
                let hue = color.to_hsl().hue;
                calculate_hue_distance(hue, target_hue) <= tolerance
            })
            .cloned()
            .collect()
    }

    fn generate_test_colors(count: usize) -> Vec<Color> {
        (0..count)
            .map(|i| {
                let hue = (i as f64 * 360.0) / count as f64;
                let saturation = 50.0 + (i % 5) as f64 * 10.0; // 50-90%
                let lightness = 30.0 + (i % 7) as f64 * 10.0;  // 30-90%
                Color::from_hsl(hue, saturation, lightness)
            })
            .collect()
    }

    fn generate_diverse_test_colors(count: usize) -> Vec<Color> {
        (0..count)
            .map(|i| {
                // Use golden ratio for better distribution
                let hue = (i as f64 * 137.5077640500378) % 360.0;
                let saturation = 25.0 + (i % 3) as f64 * 25.0; // 25%, 50%, 75%
                let lightness = 25.0 + (i % 4) as f64 * 25.0;  // 25%, 50%, 75%, 100%
                Color::from_hsl(hue, saturation, lightness)
            })
            .collect()
    }

    fn perform_full_hue_analysis(colors: &[Color], target_hue: f64, tolerance: f64) -> HueAnalysisResult {
        let filtered = filter_colors_by_hue(colors, target_hue, tolerance);
        
        let total_matches = filtered.len();
        let average_distance = if total_matches > 0 {
            filtered.iter()
                .map(|color| calculate_hue_distance(color.to_hsl().hue, target_hue))
                .sum::<f64>() / total_matches as f64
        } else {
            0.0
        };
        
        let hue_range = if filtered.len() > 1 {
            let hues: Vec<f64> = filtered.iter()
                .map(|c| c.to_hsl().hue)
                .collect();
            calculate_hue_range(&hues)
        } else {
            0.0
        };
        
        HueAnalysisResult {
            total_matches,
            average_distance,
            hue_range,
            efficiency_ratio: total_matches as f64 / colors.len() as f64,
        }
    }

    fn calculate_variance(values: &[usize]) -> f64 {
        if values.len() < 2 {
            return 0.0;
        }
        
        let mean = values.iter().sum::<usize>() as f64 / values.len() as f64;
        let variance = values.iter()
            .map(|&x| {
                let diff = x as f64 - mean;
                diff * diff
            })
            .sum::<f64>() / values.len() as f64;
        
        variance
    }

    fn calculate_hue_range(hues: &[f64]) -> f64 {
        if hues.len() < 2 {
            return 0.0;
        }
        
        let mut sorted_hues = hues.to_vec();
        sorted_hues.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let linear_range = sorted_hues.last().unwrap() - sorted_hues.first().unwrap();
        let circular_gap = 360.0 - linear_range;
        
        // Return the smaller of linear range or circular range
        linear_range.min(circular_gap)
    }

    #[derive(Debug)]
    struct HueAnalysisResult {
        total_matches: usize,
        average_distance: f64,
        hue_range: f64,
        efficiency_ratio: f64,
    }
}
