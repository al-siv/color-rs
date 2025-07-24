//! Color distance calculation strategies using the Strategy Pattern
//!
//! This module provides different algorithms for calculating color distances,
//! allowing users to choose the most appropriate method for their needs.

use palette::{
    Lab,
    color_difference::{EuclideanDistance, ImprovedCiede2000, ImprovedDeltaE},
};

/// Trait defining the strategy interface for color distance calculations
pub trait ColorDistanceStrategy {
    /// Calculate the distance between two LAB colors
    ///
    /// # Arguments
    /// * `lab1` - First LAB color
    /// * `lab2` - Second LAB color
    ///
    /// # Returns
    /// * Distance value (0.0 = identical, higher = more different)
    fn calculate_distance(&self, lab1: Lab, lab2: Lab) -> f64;

    /// Get a human-readable name for this strategy
    fn name(&self) -> &'static str;

    /// Get a description of this strategy's characteristics
    fn description(&self) -> &'static str;
}

/// Delta E 76 (CIE76) strategy - Simple Euclidean distance in LAB space
///
/// This is the original CIE Delta E formula from 1976. It's fast but less
/// perceptually accurate than newer methods.
/// TODO: check the correctness of the naming.
#[derive(Debug, Clone, Default)]
pub struct DeltaE76Strategy;

impl ColorDistanceStrategy for DeltaE76Strategy {
    fn calculate_distance(&self, lab1: Lab, lab2: Lab) -> f64 {
        lab1.improved_delta_e(lab2) as f64
    }

    fn name(&self) -> &'static str {
        "Delta E 76"
    }

    fn description(&self) -> &'static str {
        "Original CIE76 formula - Fast but less perceptually accurate"
    }
}

/// Delta E 2000 (CIEDE2000) strategy - Industry standard perceptual distance
///
/// This is the current industry standard for color difference measurement.
/// It provides the most accurate perceptual color differences but is computationally
/// more expensive than simpler methods.
/// TODO: check the correctness of the naming.
#[derive(Debug, Clone, Default)]
pub struct DeltaE2000Strategy;

impl ColorDistanceStrategy for DeltaE2000Strategy {
    fn calculate_distance(&self, lab1: Lab, lab2: Lab) -> f64 {
        // Use the improved CIEDE2000 implementation from palette
        lab1.improved_difference(lab2) as f64
    }

    fn name(&self) -> &'static str {
        "Delta E 2000"
    }

    fn description(&self) -> &'static str {
        "CIEDE2000 formula - Most perceptually accurate, industry standard"
    }
}

/// Euclidean strategy - Simple space distance
///
/// This calculates distance directly in LAB space using Euclidean distance.
#[derive(Debug, Clone, Default)]
pub struct EuclideanLabStrategy;

impl ColorDistanceStrategy for EuclideanLabStrategy {
    fn calculate_distance(&self, lab1: Lab, lab2: Lab) -> f64 {
        lab1.distance_squared(lab2).sqrt() as f64
    }

    fn name(&self) -> &'static str {
        "Euclidean distance"
    }

    fn description(&self) -> &'static str {
        "Simple Euclidean distance - Fast but not perceptually uniform"
    }
}

/// LCH Color Space strategy - Distance calculation in LCH cylindrical color space
///
/// This calculates distance in LCH (Lightness, Chroma, Hue) space, which can be
/// more intuitive for certain color operations as it separates lightness from
/// chromatic components.
#[derive(Debug, Clone, Default)]
pub struct LchStrategy;

impl ColorDistanceStrategy for LchStrategy {
    fn calculate_distance(&self, lab1: Lab, lab2: Lab) -> f64 {
        use crate::color_utils::LegacyColorUtils as ColorUtils;

        // Convert LAB to LCH using ColorUtils
        let lch1 = ColorUtils::lab_to_lch(lab1);
        let lch2 = ColorUtils::lab_to_lch(lab2);

        // Calculate differences in each component
        let delta_l = lch1.l - lch2.l;
        let delta_c = lch1.chroma - lch2.chroma;

        // Handle hue difference (circular space)
        let delta_h = {
            let h1 = lch1.hue.into_positive_degrees();
            let h2 = lch2.hue.into_positive_degrees();
            let diff = (h1 - h2).abs();
            if diff > 180.0 { 360.0 - diff } else { diff }
        };

        // Calculate Euclidean distance in LCH space
        // Note: Hue is weighted less since it's in degrees while L and C are in different scales
        let hue_weight = 0.1; // Adjust this weight as needed
        (delta_l.powi(2) + delta_c.powi(2) + (delta_h * hue_weight).powi(2)).sqrt() as f64
    }

    fn name(&self) -> &'static str {
        "LCH Color Space"
    }

    fn description(&self) -> &'static str {
        "Distance calculation in LCH cylindrical color space - Separates lightness from chroma"
    }
}

/// Convenience function to create a strategy by name
///
/// # Arguments
/// * `strategy_name` - Name of the strategy ("delta-e-76", "delta-e-2000", "euclidean-lab", "lch")
///
/// # Returns
/// * Boxed strategy instance, defaults to Delta E 2000 for unknown names
#[must_use]
pub fn create_strategy(strategy_name: &str) -> Box<dyn ColorDistanceStrategy> {
    match strategy_name.to_lowercase().as_str() {
        "delta-e-76" | "deltae76" | "cie76" => Box::new(DeltaE76Strategy),
        "delta-e-2000" | "deltae2000" | "ciede2000" | "default" => Box::new(DeltaE2000Strategy),
        "euclidean-lab" | "euclidean" | "lab" => Box::new(EuclideanLabStrategy),
        "lch" | "lch-space" | "lch-color-space" => Box::new(LchStrategy),
        _ => {
            eprintln!("Warning: Unknown strategy '{strategy_name}', using Delta E 2000");
            Box::new(DeltaE2000Strategy)
        }
    }
}

/// Get all available strategy names for CLI help
#[must_use]
pub fn available_strategies() -> Vec<&'static str> {
    vec!["delta-e-76", "delta-e-2000", "euclidean-lab", "lch"]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color_utils::LegacyColorUtils as ColorUtils;

    #[test]
    fn test_strategy_creation() {
        let strategy = create_strategy("delta-e-2000");
        assert_eq!(strategy.name(), "Delta E 2000");

        let strategy = create_strategy("delta-e-76");
        assert_eq!(strategy.name(), "Delta E 76");

        let strategy = create_strategy("euclidean-lab");
        assert_eq!(strategy.name(), "Euclidean distance");

        let strategy = create_strategy("lch");
        assert_eq!(strategy.name(), "LCH Color Space");
    }

    #[test]
    fn test_strategy_distance_calculations() {
        let red_lab = ColorUtils::rgb_to_lab((255, 0, 0));
        let blue_lab = ColorUtils::rgb_to_lab((0, 0, 255));

        let delta_e_76 = DeltaE76Strategy;
        let delta_e_2000 = DeltaE2000Strategy;
        let euclidean_lab = EuclideanLabStrategy;
        let lch_strategy = LchStrategy;

        let distance_76 = delta_e_76.calculate_distance(red_lab, blue_lab);
        let distance_2000 = delta_e_2000.calculate_distance(red_lab, blue_lab);
        let distance_lab = euclidean_lab.calculate_distance(red_lab, blue_lab);
        let distance_lch = lch_strategy.calculate_distance(red_lab, blue_lab);

        // All should return positive distances for different colors
        assert!(distance_76 > 0.0);
        assert!(distance_2000 > 0.0);
        assert!(distance_lab > 0.0);
        assert!(distance_lch > 0.0);

        // Test identity (same color should have distance 0 for all strategies)
        assert!((delta_e_76.calculate_distance(red_lab, red_lab) - 0.0).abs() < 0.001);
        assert!((delta_e_2000.calculate_distance(red_lab, red_lab) - 0.0).abs() < 0.001);
        assert!((euclidean_lab.calculate_distance(red_lab, red_lab) - 0.0).abs() < 0.001);
        assert!((lch_strategy.calculate_distance(red_lab, red_lab) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_strategy_symmetry() {
        let red_lab = ColorUtils::rgb_to_lab((255, 0, 0));
        let blue_lab = ColorUtils::rgb_to_lab((0, 0, 255));

        let strategies: Vec<Box<dyn ColorDistanceStrategy>> = vec![
            Box::new(DeltaE76Strategy),
            Box::new(DeltaE2000Strategy),
            Box::new(EuclideanLabStrategy),
            Box::new(LchStrategy),
        ];

        for strategy in strategies {
            let distance_ab = strategy.calculate_distance(red_lab, blue_lab);
            let distance_ba = strategy.calculate_distance(blue_lab, red_lab);

            // Distance should be symmetric
            assert!(
                (distance_ab - distance_ba).abs() < 0.001,
                "Strategy {} is not symmetric",
                strategy.name()
            );
        }
    }
}
