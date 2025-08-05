//! Result formatting and color comparison functions
//!
//! Provides formatted output structures and comparison functionality
//! for detailed color analysis and comparison reports.

use crate::color_ops::{contrast, distance};
use palette::Srgb;
use serde::{Deserialize, Serialize};

use super::core::{ColorAnalysis, analyze_color};

/// Color comparison result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorComparison {
    pub color1: ColorAnalysis,
    pub color2: ColorAnalysis,
    pub distance_metrics: DistanceMetrics,
    pub contrast_ratio: f64,
    pub perceptual_similarity: String,
}

/// Distance metrics between colors
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistanceMetrics {
    pub delta_e_2000: f64,
    pub delta_e_cie94: f64,
    pub delta_e_cie76: f64,
    pub rgb_euclidean: f64,
    pub lab_euclidean: f64,
}

/// Compare two colors and return detailed comparison
///
/// # Arguments
/// * `color1` - First color to compare
/// * `color2` - Second color to compare
///
/// # Returns
/// * Detailed color comparison structure
pub fn compare_colors(color1: Srgb, color2: Srgb) -> ColorComparison {
    let analysis1 = analyze_color(color1);
    let analysis2 = analyze_color(color2);
    let distance_metrics = calculate_distance_metrics(color1, color2);
    let contrast_ratio = contrast::wcag_ratio(color1, color2);
    let perceptual_similarity = classify_similarity(color1, color2);
    
    ColorComparison {
        color1: analysis1,
        color2: analysis2,
        distance_metrics,
        contrast_ratio,
        perceptual_similarity,
    }
}

/// Calculate all distance metrics between two colors
fn calculate_distance_metrics(color1: Srgb, color2: Srgb) -> DistanceMetrics {
    DistanceMetrics {
        delta_e_2000: distance::delta_e_2000(color1, color2),
        delta_e_cie94: distance::delta_e_cie94(color1, color2),
        delta_e_cie76: distance::delta_e_cie76(color1, color2),
        rgb_euclidean: distance::rgb_euclidean(color1, color2),
        lab_euclidean: distance::lab_euclidean(color1, color2),
    }
}

/// Classify perceptual similarity between colors
fn classify_similarity(color1: Srgb, color2: Srgb) -> String {
    let delta_e = distance::delta_e_2000(color1, color2);
    
    match delta_e {
        d if d < 1.0 => "Identical".to_string(),
        d if d < 2.3 => "Just Noticeable".to_string(),
        d if d < 5.0 => "Perceptible".to_string(),
        d if d < 10.0 => "Noticeable".to_string(),
        d if d < 20.0 => "Different".to_string(),
        d if d < 40.0 => "Very Different".to_string(),
        _ => "Extremely Different".to_string(),
    }
}
