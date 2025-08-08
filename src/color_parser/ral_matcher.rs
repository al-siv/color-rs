//! RAL Color Matching with Unified Collections Backend
//!
//! This module provides RAL color matching functionality using the new unified
//! color collection system. It maintains backward compatibility with the existing API
//! while leveraging the improved architecture underneath.

use super::collections::ColorMatch as UnifiedColorMatch;
use super::unified_manager::UnifiedColorManager;
#[cfg(test)]
use crate::color_distance_strategies::{DistanceAlgorithm, calculate_distance};
use palette::{IntoColor, Lab, Srgb};

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
            f32::from(self.r) / 255.0,
            f32::from(self.g) / 255.0,
            f32::from(self.b) / 255.0,
        );
        srgb.into_color()
    }
}

/// Get a unified color manager with safe fallback (non-panicking)
fn get_manager() -> UnifiedColorManager {
    UnifiedColorManager::new().unwrap_or_default()
}

/// Convert unified ColorMatch to RalMatch with classification
fn to_ral_match(color_match: &UnifiedColorMatch, classification: RalClassification) -> RalMatch {
    let hex = color_match
        .entry
        .metadata
        .extra_data
        .get("hex")
        .cloned()
        .unwrap_or_else(|| {
            format!(
                "#{:02X}{:02X}{:02X}",
                color_match.entry.color.rgb[0],
                color_match.entry.color.rgb[1],
                color_match.entry.color.rgb[2]
            )
        });

    RalMatch {
        code: color_match.entry.metadata.code.clone().unwrap_or_default(),
        name: color_match.entry.metadata.name.clone(),
        hex,
        distance: color_match.distance,
        classification,
    }
}

/// Find closest RAL Classic colors
#[must_use]
pub fn find_closest_ral_classic(rgb: &RgbColor, max_results: usize) -> Vec<RalMatch> {
    let manager = get_manager();
    let matches = manager.find_closest_ral_classic([rgb.r, rgb.g, rgb.b], max_results);
    matches
        .iter()
        .map(|m| to_ral_match(m, RalClassification::Classic))
        .collect()
}

/// Find the two closest RAL Design System+ colors to the given RGB color
#[must_use]
pub fn find_closest_ral_design(rgb: &RgbColor, max_results: usize) -> Vec<RalMatch> {
    let manager = get_manager();
    let matches = manager.find_closest_ral_design([rgb.r, rgb.g, rgb.b], max_results);
    matches
        .iter()
        .map(|m| to_ral_match(m, RalClassification::DesignSystem))
        .collect()
}

/// Find closest colors from both RAL classifications
#[must_use]
pub fn find_closest_ral_colors(rgb: &RgbColor, max_results: usize) -> Vec<RalMatch> {
    let manager = get_manager();
    let mut all: Vec<RalMatch> = Vec::new();
    let classic = manager.find_closest_ral_classic([rgb.r, rgb.g, rgb.b], max_results);
    let design = manager.find_closest_ral_design([rgb.r, rgb.g, rgb.b], max_results);
    all.extend(
        classic
            .iter()
            .map(|m| to_ral_match(m, RalClassification::Classic)),
    );
    all.extend(
        design
            .iter()
            .map(|m| to_ral_match(m, RalClassification::DesignSystem)),
    );
    all.sort_by(|a, b| a.distance.total_cmp(&b.distance));
    all.truncate(max_results);
    all
}

/// Parse RAL Classic code (e.g., "RAL2013", "RAL 2013") without regex
#[must_use]
pub fn parse_ral_classic_code(input: &str) -> Option<RalMatch> {
    let s = input.trim();
    // Case-insensitive check for prefix "RAL"
    let (prefix, _rest) = s.split_at(s.len().min(3));
    if !prefix.eq_ignore_ascii_case("RAL") {
        return None;
    }
    // Remaining may start with spaces then exactly 4 digits, and nothing else
    let mut rest = &s[3..];
    rest = rest.trim_start();
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.len() != 4 {
        return None;
    }
    // Ensure no trailing non-whitespace after digits
    let consumed = rest.chars().take_while(|c| c.is_ascii_digit()).count();
    let tail = &rest[consumed..].trim();
    if !tail.is_empty() {
        return None;
    }
    let code = format!("RAL {digits}");
    find_ral_by_code(&code)
}

/// Parse RAL Design System+ code (e.g., "RAL 010 20 10") without regex
#[must_use]
pub fn parse_ral_design_code(input: &str) -> Option<RalMatch> {
    let s = input.trim();
    let (prefix, _rest) = s.split_at(s.len().min(3));
    if !prefix.eq_ignore_ascii_case("RAL") {
        return None;
    }
    let rest = s[3..].trim();
    let parts: Vec<&str> = rest.split_whitespace().collect();
    if parts.len() != 3 {
        return None;
    }
    let ok = parts[0].len() == 3
        && parts[1].len() == 2
        && parts[2].len() == 2
        && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()));
    if !ok {
        return None;
    }
    let search_code = format!("RAL {} {} {}", parts[0], parts[1], parts[2]);
    find_ral_by_code(&search_code)
}

/// Find RAL color by name (case-insensitive partial match)
#[must_use]
pub fn find_ral_by_name(name: &str) -> Vec<RalMatch> {
    let manager = get_manager();
    let mut matches = Vec::new();
    for (collection_name, entry) in manager.find_by_name(name) {
        let classification = if collection_name == "RAL Classic" {
            RalClassification::Classic
        } else {
            RalClassification::DesignSystem
        };
        let hex = entry
            .metadata
            .extra_data
            .get("hex")
            .cloned()
            .unwrap_or_else(|| {
                format!(
                    "#{:02X}{:02X}{:02X}",
                    entry.color.rgb[0], entry.color.rgb[1], entry.color.rgb[2]
                )
            });
        matches.push(RalMatch {
            code: entry.metadata.code.unwrap_or_default(),
            name: entry.metadata.name,
            hex,
            distance: 0.0,
            classification,
        });
    }
    matches
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

/// Find RAL color by exact code
#[must_use]
pub fn find_ral_by_code(code: &str) -> Option<RalMatch> {
    let manager = get_manager();
    manager.find_by_code(code).map(|(collection_name, entry)| {
        let classification = if collection_name == "RAL Classic" {
            RalClassification::Classic
        } else {
            RalClassification::DesignSystem
        };
        let hex = entry
            .metadata
            .extra_data
            .get("hex")
            .cloned()
            .unwrap_or_else(|| {
                format!(
                    "#{:02X}{:02X}{:02X}",
                    entry.color.rgb[0], entry.color.rgb[1], entry.color.rgb[2]
                )
            });
        RalMatch {
            code: entry.metadata.code.unwrap_or_default(),
            name: entry.metadata.name,
            hex,
            distance: 0.0,
            classification,
        }
    })
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
        let green_6038_lab_color =
            Lab::new(green_6038_lab[0], green_6038_lab[1], green_6038_lab[2]);
        let green_6018_lab_color =
            Lab::new(green_6018_lab[0], green_6018_lab[1], green_6018_lab[2]);
        let blue_5000_lab_color = Lab::new(blue_5000_lab[0], blue_5000_lab[1], blue_5000_lab[2]);
        let blue_5007_lab_color = Lab::new(blue_5007_lab[0], blue_5007_lab[1], blue_5007_lab[2]);

        let green_6038_dist = calculate_distance(
            DistanceAlgorithm::DeltaE2000,
            input_lab_color,
            green_6038_lab_color,
        );
        let green_6018_dist = calculate_distance(
            DistanceAlgorithm::DeltaE2000,
            input_lab_color,
            green_6018_lab_color,
        );
        let blue_5000_dist = calculate_distance(
            DistanceAlgorithm::DeltaE2000,
            input_lab_color,
            blue_5000_lab_color,
        );
        let blue_5007_dist = calculate_distance(
            DistanceAlgorithm::DeltaE2000,
            input_lab_color,
            blue_5007_lab_color,
        );

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
