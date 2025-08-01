//! RAL Color Matching with Unified Collections Backend
//!
//! This module provides RAL color matching functionality using the new unified
//! color collection system. It maintains backward compatibility with the existing API
//! while leveraging the improved architecture underneath.

use super::compat::{
    find_closest_ral_classic_compat, find_closest_ral_colors_compat,
    find_closest_ral_design_compat, find_ral_by_code_compat, find_ral_by_name_pattern_compat,
};
#[cfg(test)]
use crate::color_distance_strategies::{DistanceAlgorithm, calculate_distance};
use palette::{Lab, Srgb, IntoColor};
use regex::Regex;
use std::sync::OnceLock;

/// RAL color match result with distance information
#[derive(Debug, Clone)]
pub struct RalMatch {
    pub code: String,
    pub name: String,
    pub hex: String,
    pub distance: f64,
    pub classification: RalClassification,
}

/// RAL color classification type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RalClassification {
    Classic,
    DesignSystem,
}

/// RGB color for matching
#[derive(Debug, Clone)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Convert RGB to LAB color space for accurate distance calculation
    #[must_use]
    pub fn to_lab(&self) -> Lab {
        let srgb = Srgb::new(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        );
        srgb.into_color()
    }
}

/// Find the two closest RAL Classic colors to the given RGB color
#[must_use]
pub fn find_closest_ral_classic(rgb: &RgbColor, max_results: usize) -> Vec<RalMatch> {
    find_closest_ral_classic_compat(rgb, max_results)
}

/// Find the two closest RAL Design System+ colors to the given RGB color
#[must_use]
pub fn find_closest_ral_design(rgb: &RgbColor, max_results: usize) -> Vec<RalMatch> {
    find_closest_ral_design_compat(rgb, max_results)
}

/// Find closest colors from both RAL classifications
#[must_use]
pub fn find_closest_ral_colors(rgb: &RgbColor, max_results: usize) -> Vec<RalMatch> {
    find_closest_ral_colors_compat(rgb, max_results)
}

/// Parse RAL Classic code (e.g., "RAL2013", "RAL 2013")
#[must_use]
pub fn parse_ral_classic_code(input: &str) -> Option<RalMatch> {
    static RAL_CLASSIC_REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = RAL_CLASSIC_REGEX.get_or_init(|| Regex::new(r"(?i)^RAL\s*(\d{4})$").unwrap());

    if let Some(caps) = regex.captures(input.trim()) {
        let number = caps.get(1).unwrap().as_str();
        let code = format!("RAL {number}");
        find_ral_by_code_compat(&code)
    } else {
        None
    }
}

/// Parse RAL Design System+ code (e.g., "RAL 010 20 10")
#[must_use]
pub fn parse_ral_design_code(input: &str) -> Option<RalMatch> {
    static RAL_DESIGN_REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = RAL_DESIGN_REGEX
        .get_or_init(|| Regex::new(r"(?i)^RAL\s*(\d{3})\s*(\d{2})\s*(\d{2})$").unwrap());

    if let Some(caps) = regex.captures(input.trim()) {
        let hue = caps.get(1).unwrap().as_str();
        let lightness = caps.get(2).unwrap().as_str();
        let chroma = caps.get(3).unwrap().as_str();
        let search_code = format!("RAL {hue} {lightness} {chroma}");
        find_ral_by_code_compat(&search_code)
    } else {
        None
    }
}

/// Find RAL color by name (case-insensitive partial match)
#[must_use]
pub fn find_ral_by_name(name: &str) -> Vec<RalMatch> {
    find_ral_by_name_pattern_compat(name)
}

/// Main RAL color parsing function - tries all formats
#[must_use]
pub fn parse_ral_color(input: &str) -> Option<RalMatch> {
    // Try RAL Classic code first (RAL XXXX format)
    if let Some(color) = parse_ral_classic_code(input) {
        return Some(color);
    }

    // Try RAL Design System+ code (HXXXLXXCXX format)
    if let Some(color) = parse_ral_design_code(input) {
        return Some(color);
    }

    // Try name search - let the system handle conflicts naturally
    let name_matches = find_ral_by_name(input);
    if !name_matches.is_empty() {
        return Some(name_matches[0].clone());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ral_classic_code() {
        // Test various RAL Classic code formats
        assert!(parse_ral_classic_code("RAL 1000").is_some());
        assert!(parse_ral_classic_code("RAL1000").is_some());
        assert!(parse_ral_classic_code("ral 1000").is_some());
        assert!(parse_ral_classic_code("invalid").is_none());
    }

    #[test]
    fn test_parse_ral_design_code() {
        // Test RAL Design System+ code format (with spaces as in CSV)
        assert!(parse_ral_design_code("RAL 000 15 00").is_some());
        assert!(parse_ral_design_code("ral 000 15 00").is_some());
        assert!(parse_ral_design_code("invalid").is_none());
    }

    #[test]
    fn test_find_ral_by_name() {
        let matches = find_ral_by_name("red");
        assert!(!matches.is_empty());

        let matches = find_ral_by_name("beige");
        assert!(!matches.is_empty());
        assert!(
            matches
                .iter()
                .any(|m| m.name.to_lowercase().contains("beige"))
        );
    }

    #[test]
    fn test_closest_color_matching() {
        // Test with a known color (approximate red)
        let red = RgbColor::new(255, 0, 0);
        let matches = find_closest_ral_colors(&red, 2);
        assert_eq!(matches.len(), 2);
        assert!(matches[0].distance <= matches[1].distance);
    }

    #[test]
    fn test_manual_distance_verification() {
        // Manual verification of distance calculations
        let input_lab = [60.52568f32, 5.942_374_f32, -61.562_084_f32];

        // Green colors that are showing up first
        let green_6038_lab = [35.69f32, 62.308f32, -84.293f32]; // Luminous green
        let green_6018_lab = [41.37f32, 57.587f32, -35.153f32]; // Yellow green

        // Blue colors that should be closer
        let blue_5000_lab = [32.75f32, 32.585f32, -1.282f32]; // Violet blue  
        let blue_5007_lab = [40.39f32, 42.93f32, -6.80f32]; // Brilliant blue

        // Calculate distances using functional approach
        let input_lab_color = Lab::new(input_lab[0], input_lab[1], input_lab[2]);
        let green_6038_lab_color = Lab::new(green_6038_lab[0], green_6038_lab[1], green_6038_lab[2]);
        let green_6018_lab_color = Lab::new(green_6018_lab[0], green_6018_lab[1], green_6018_lab[2]);
        let blue_5000_lab_color = Lab::new(blue_5000_lab[0], blue_5000_lab[1], blue_5000_lab[2]);
        let blue_5007_lab_color = Lab::new(blue_5007_lab[0], blue_5007_lab[1], blue_5007_lab[2]);
        
        let green_6038_dist = calculate_distance(DistanceAlgorithm::DeltaE2000, input_lab_color, green_6038_lab_color);
        let green_6018_dist = calculate_distance(DistanceAlgorithm::DeltaE2000, input_lab_color, green_6018_lab_color);
        let blue_5000_dist = calculate_distance(DistanceAlgorithm::DeltaE2000, input_lab_color, blue_5000_lab_color);
        let blue_5007_dist = calculate_distance(DistanceAlgorithm::DeltaE2000, input_lab_color, blue_5007_lab_color);

        println!(
            "Input: LAB({:.2}, {:.2}, {:.2})",
            input_lab[0], input_lab[1], input_lab[2]
        );
        println!(
            "Green 6038: LAB({:.2}, {:.2}, {:.2}) -> ΔE {:.2}",
            green_6038_lab[0], green_6038_lab[1], green_6038_lab[2], green_6038_dist
        );
        println!(
            "Green 6018: LAB({:.2}, {:.2}, {:.2}) -> ΔE {:.2}",
            green_6018_lab[0], green_6018_lab[1], green_6018_lab[2], green_6018_dist
        );
        println!(
            "Blue  5000: LAB({:.2}, {:.2}, {:.2}) -> ΔE {:.2}",
            blue_5000_lab[0], blue_5000_lab[1], blue_5000_lab[2], blue_5000_dist
        );
        println!(
            "Blue  5007: LAB({:.2}, {:.2}, {:.2}) -> ΔE {:.2}",
            blue_5007_lab[0], blue_5007_lab[1], blue_5007_lab[2], blue_5007_dist
        );

        // The bug: green colors have lower ΔE than blue colors, which seems wrong
        println!("\nAre green colors really closer than blue colors?");
        println!(
            "Green vs Blue: {} vs {}",
            green_6038_dist < blue_5000_dist,
            green_6038_dist < blue_5007_dist
        );
    }
}
