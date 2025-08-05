//! Unified gradient calculation algorithms
//!
//! This module contains unified gradient calculation algorithms,
//! breaking them down into focused, composable functions that follow single responsibility principle.

use super::calculator::{UnifiedGradientStop, cubic_bezier_ease, GradientCalculator};
use crate::color_distance_strategies::{DistanceAlgorithm, calculate_distance};
use palette::{IntoColor, Lab, Mix, Srgb};

/// Configuration for gradient calculation
#[derive(Debug, Clone, Copy)]
pub struct GradientCalculationConfig {
    pub start_lab: Lab,
    pub end_lab: Lab,
    pub start_position: u8,
    pub end_position: u8,
    pub ease_in: f64,
    pub ease_out: f64,
    pub steps: usize,
    pub use_simple_mode: bool,
    pub algorithm: DistanceAlgorithm,
}

/// RGB color representation as a tuple
type RgbTuple = (u8, u8, u8);

/// Functional conversion from Lab to RGB tuple
fn lab_to_rgb_tuple(lab: Lab) -> RgbTuple {
    let srgb: Srgb = lab.into_color();
    (
        (srgb.red * 255.0).round() as u8,
        (srgb.green * 255.0).round() as u8,
        (srgb.blue * 255.0).round() as u8,
    )
}

/// Calculate geometric position based on step index and total steps
fn calculate_geometric_position(step_index: usize, total_steps: usize) -> f64 {
    step_index as f64 / (total_steps - 1) as f64
}

/// Calculate actual position based on geometric t and position range
fn calculate_actual_position(geometric_t: f64, start_pos: u8, end_pos: u8) -> u8 {
    (start_pos as f64 + geometric_t * (end_pos - start_pos) as f64).round() as u8
}

/// Create a gradient stop with all necessary data
fn create_gradient_stop(
    position: u8,
    geometric_t: f64,
    bezier_t: f64,
    lab_color: Lab,
    rgb_color: RgbTuple,
) -> UnifiedGradientStop {
    UnifiedGradientStop {
        position,
        geometric_t,
        bezier_t,
        lab_color,
        rgb_color,
    }
}

/// Simple mode: RGB interpolation with bezier easing
fn calculate_simple_mode_stops(config: GradientCalculationConfig) -> Vec<UnifiedGradientStop> {
    let start_rgb = lab_to_rgb_tuple(config.start_lab);
    let end_rgb = lab_to_rgb_tuple(config.end_lab);
    
    (0..config.steps)
        .map(|i| calculate_simple_mode_stop(i, config, start_rgb, end_rgb))
        .collect()
}

/// Calculate a single stop for simple mode
fn calculate_simple_mode_stop(
    step_index: usize,
    config: GradientCalculationConfig,
    start_rgb: RgbTuple,
    end_rgb: RgbTuple,
) -> UnifiedGradientStop {
    let geometric_t = calculate_geometric_position(step_index, config.steps);
    let bezier_t = cubic_bezier_ease(geometric_t, config.ease_in, config.ease_out);
    
    // RGB interpolation with bezier timing
    let interpolated_rgb = interpolate_rgb(start_rgb, end_rgb, bezier_t);
    
    // Convert back to LAB for consistent output format
    let lab_color = rgb_tuple_to_lab(interpolated_rgb);
    let position = calculate_actual_position(geometric_t, config.start_position, config.end_position);
    
    create_gradient_stop(position, geometric_t, bezier_t, lab_color, interpolated_rgb)
}

/// Interpolate between two RGB colors using a factor
fn interpolate_rgb(start_rgb: RgbTuple, end_rgb: RgbTuple, factor: f64) -> RgbTuple {
    let r = (start_rgb.0 as f64 + (end_rgb.0 as f64 - start_rgb.0 as f64) * factor).round() as u8;
    let g = (start_rgb.1 as f64 + (end_rgb.1 as f64 - start_rgb.1 as f64) * factor).round() as u8;
    let b = (start_rgb.2 as f64 + (end_rgb.2 as f64 - start_rgb.2 as f64) * factor).round() as u8;
    (r, g, b)
}

/// Convert RGB tuple back to Lab color space
fn rgb_tuple_to_lab(rgb: RgbTuple) -> Lab {
    let srgb = Srgb::new(
        f32::from(rgb.0) / 255.0,
        f32::from(rgb.1) / 255.0,
        f32::from(rgb.2) / 255.0,
    );
    srgb.into_color()
}

/// Smart mode: Equal distance with geometric position finding
fn calculate_smart_mode_stops(config: GradientCalculationConfig) -> Vec<UnifiedGradientStop> {
    let total_distance = calculate_distance(config.algorithm, config.start_lab, config.end_lab);
    let step_distance = total_distance / (config.steps - 1) as f64;
    
    (0..config.steps)
        .map(|i| calculate_smart_mode_stop(i, config, step_distance))
        .collect()
}

/// Calculate a single stop for smart mode
fn calculate_smart_mode_stop(
    step_index: usize,
    config: GradientCalculationConfig,
    step_distance: f64,
) -> UnifiedGradientStop {
    match step_index {
        0 => create_start_stop(config),
        i if i == config.steps - 1 => create_end_stop(config),
        _ => create_middle_stop(step_index, config, step_distance),
    }
}

/// Create the starting gradient stop
fn create_start_stop(config: GradientCalculationConfig) -> UnifiedGradientStop {
    let start_rgb = lab_to_rgb_tuple(config.start_lab);
    create_gradient_stop(
        config.start_position,
        0.0,
        0.0,
        config.start_lab,
        start_rgb,
    )
}

/// Create the ending gradient stop
fn create_end_stop(config: GradientCalculationConfig) -> UnifiedGradientStop {
    let end_rgb = lab_to_rgb_tuple(config.end_lab);
    create_gradient_stop(
        config.end_position,
        1.0,
        1.0,
        config.end_lab,
        end_rgb,
    )
}

/// Create a middle gradient stop using binary search
fn create_middle_stop(
    step_index: usize,
    config: GradientCalculationConfig,
    step_distance: f64,
) -> UnifiedGradientStop {
    let target_distance = step_distance * step_index as f64;
    let geometric_t = find_geometric_position_for_distance(target_distance, config);
    let bezier_t = cubic_bezier_ease(geometric_t, config.ease_in, config.ease_out);
    let lab_color = config.start_lab.mix(config.end_lab, bezier_t as f32);
    let rgb_color = lab_to_rgb_tuple(lab_color);
    let position = calculate_actual_position(geometric_t, config.start_position, config.end_position);
    
    create_gradient_stop(position, geometric_t, bezier_t, lab_color, rgb_color)
}

/// Binary search configuration
#[derive(Debug, Clone, Copy)]
struct BinarySearchConfig {
    pub target_distance: f64,
    pub gradient_config: GradientCalculationConfig,
    pub max_iterations: usize,
    pub tolerance: f64,
}

impl Default for BinarySearchConfig {
    fn default() -> Self {
        Self {
            target_distance: 0.0,
            gradient_config: GradientCalculationConfig {
                start_lab: Lab::new(0.0, 0.0, 0.0),
                end_lab: Lab::new(100.0, 0.0, 0.0),
                start_position: 0,
                end_position: 100,
                ease_in: 0.0,
                ease_out: 0.0,
                steps: 10,
                use_simple_mode: false,
                algorithm: DistanceAlgorithm::DeltaE2000,
            },
            max_iterations: 50,
            tolerance: 0.01,
        }
    }
}

/// Find geometric position that produces target distance using binary search
fn find_geometric_position_for_distance(
    target_distance: f64,
    config: GradientCalculationConfig,
) -> f64 {
    let search_config = BinarySearchConfig {
        target_distance,
        gradient_config: config,
        ..Default::default()
    };
    
    binary_search_for_position(search_config)
}

/// Binary search algorithm to find geometric position
fn binary_search_for_position(search_config: BinarySearchConfig) -> f64 {
    let mut low = 0.0;
    let mut high = 1.0;
    let mut best_t = 0.5;
    
    for _ in 0..search_config.max_iterations {
        let mid_t = (low + high) / 2.0;
        let actual_distance = calculate_distance_at_position(mid_t, search_config.gradient_config);
        
        if (actual_distance - search_config.target_distance).abs() < search_config.tolerance {
            return mid_t;
        }
        
        if actual_distance < search_config.target_distance {
            low = mid_t;
        } else {
            high = mid_t;
        }
        
        best_t = mid_t;
    }
    
    best_t
}

/// Calculate distance from start color at a given geometric position
fn calculate_distance_at_position(
    geometric_t: f64,
    config: GradientCalculationConfig,
) -> f64 {
    let bezier_t = cubic_bezier_ease(geometric_t, config.ease_in, config.ease_out);
    let test_color = config.start_lab.mix(config.end_lab, bezier_t as f32);
    calculate_distance(config.algorithm, config.start_lab, test_color)
}

/// Main refactored function with functional composition
pub fn calculate_unified_gradient(
    start_lab: Lab,
    end_lab: Lab,
    start_position: u8,
    end_position: u8,
    ease_in: f64,
    ease_out: f64,
    steps: usize,
    use_simple_mode: bool,
    algorithm: DistanceAlgorithm,
) -> Vec<UnifiedGradientStop> {
    let config = GradientCalculationConfig {
        start_lab,
        end_lab,
        start_position,
        end_position,
        ease_in,
        ease_out,
        steps,
        use_simple_mode,
        algorithm,
    };
    
    if use_simple_mode {
        calculate_simple_mode_stops(config)
    } else {
        calculate_smart_mode_stops(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Lab;

    #[test]
    fn test_lab_to_rgb_tuple_conversion() {
        let red_lab = Lab::new(53.24, 80.09, 67.20);
        let rgb = lab_to_rgb_tuple(red_lab);
        // Should be approximately red
        assert!(rgb.0 > 200); // High red component
        assert!(rgb.1 < 100); // Low green component
        assert!(rgb.2 < 100); // Low blue component
    }

    #[test]
    fn test_geometric_position_calculation() {
        assert_eq!(calculate_geometric_position(0, 10), 0.0);
        assert_eq!(calculate_geometric_position(9, 10), 1.0);
        assert!((calculate_geometric_position(5, 11) - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_actual_position_calculation() {
        assert_eq!(calculate_actual_position(0.0, 0, 100), 0);
        assert_eq!(calculate_actual_position(1.0, 0, 100), 100);
        assert_eq!(calculate_actual_position(0.5, 0, 100), 50);
    }

    #[test]
    fn test_rgb_interpolation() {
        let start = (255, 0, 0); // Red
        let end = (0, 255, 0);   // Green
        
        let mid = interpolate_rgb(start, end, 0.5);
        assert_eq!(mid, (128, 128, 0)); // Middle should be yellow-ish
        
        let quarter = interpolate_rgb(start, end, 0.25);
        assert_eq!(quarter, (191, 64, 0));
    }

    #[test]
    fn test_rgb_tuple_to_lab_roundtrip() {
        let original_rgb = (255, 128, 64);
        let lab = rgb_tuple_to_lab(original_rgb);
        let converted_rgb = lab_to_rgb_tuple(lab);
        
        // Should be approximately the same (allowing for conversion precision)
        assert!((original_rgb.0 as i16 - converted_rgb.0 as i16).abs() <= 2);
        assert!((original_rgb.1 as i16 - converted_rgb.1 as i16).abs() <= 2);
        assert!((original_rgb.2 as i16 - converted_rgb.2 as i16).abs() <= 2);
    }

    #[test]
    fn test_simple_mode_calculation() {
        let config = GradientCalculationConfig {
            start_lab: Lab::new(0.0, 0.0, 0.0), // Black
            end_lab: Lab::new(100.0, 0.0, 0.0), // White
            start_position: 0,
            end_position: 100,
            ease_in: 0.0,
            ease_out: 0.0,
            steps: 3,
            use_simple_mode: true,
            algorithm: DistanceAlgorithm::DeltaE2000,
        };

        let stops = calculate_simple_mode_stops(config);
        assert_eq!(stops.len(), 3);
        
        // First stop should be at start
        assert_eq!(stops[0].position, 0);
        assert_eq!(stops[0].geometric_t, 0.0);
        
        // Last stop should be at end
        assert_eq!(stops[2].position, 100);
        assert_eq!(stops[2].geometric_t, 1.0);
        
        // Middle stop should be halfway
        assert_eq!(stops[1].position, 50);
        assert!((stops[1].geometric_t - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_unified_calculator_equivalence() {
        let start_lab = Lab::new(20.0, 10.0, -5.0);
        let end_lab = Lab::new(80.0, -10.0, 15.0);
        
        // Test that functional version produces same structure as original
        let functional_stops = calculate_unified_gradient(
            start_lab, end_lab, 0, 100, 0.42, 1.0, 5, true, DistanceAlgorithm::DeltaE2000
        );
        
        let original_stops = GradientCalculator::calculate_unified_gradient_with_algorithm(
            start_lab, end_lab, 0, 100, 0.42, 1.0, 5, true, DistanceAlgorithm::DeltaE2000
        );
        
        assert_eq!(functional_stops.len(), original_stops.len());
        
        // Check that all stops have equivalent positions (allowing for floating point precision)
        for (func_stop, orig_stop) in functional_stops.iter().zip(original_stops.iter()) {
            assert_eq!(func_stop.position, orig_stop.position);
            assert!((func_stop.geometric_t - orig_stop.geometric_t).abs() < 0.001);
        }
    }

    #[test]
    fn test_binary_search_convergence() {
        let config = GradientCalculationConfig {
            start_lab: Lab::new(20.0, 10.0, -5.0),
            end_lab: Lab::new(80.0, -10.0, 15.0),
            start_position: 0,
            end_position: 100,
            ease_in: 0.0,
            ease_out: 0.0,
            steps: 5,
            use_simple_mode: false,
            algorithm: DistanceAlgorithm::DeltaE2000,
        };
        
        // Test that binary search finds a reasonable position
        let target_distance = 20.0;
        let found_t = find_geometric_position_for_distance(target_distance, config);
        
        assert!(found_t >= 0.0);
        assert!(found_t <= 1.0);
        
        // Verify the found position produces distance close to target
        let actual_distance = calculate_distance_at_position(found_t, config);
        assert!((actual_distance - target_distance).abs() < 0.1);
    }
}
