//! RAL Color Matching
//!
//! Functions for finding closest RAL colors and parsing RAL codes

use super::ral_data::{RAL_CLASSIC_DATA, RAL_DESIGN_DATA};
use crate::color_utils::ColorUtils;
use palette::{Lab, Srgb, IntoColor};
use regex::Regex;
use std::sync::OnceLock;

/// RAL color match result with distance information
#[derive(Debug, Clone)]
pub struct RalMatch {
    pub code: String,
    pub name: String,
    pub hex: String,
    pub distance: f32,
    pub classification: RalClassification,
}

/// RAL color classification type
#[derive(Debug, Clone, PartialEq)]
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
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    
    /// Convert RGB to LAB color space for accurate distance calculation
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
pub fn find_closest_ral_classic(rgb: &RgbColor, max_results: usize) -> Vec<RalMatch> {
    let target_lab = rgb.to_lab();
    let mut matches = Vec::new();
    
    for &(code, name, hex, lab_l, lab_a, lab_b, _cmyk_c, _cmyk_m, _cmyk_y, _cmyk_k, _lrv) in RAL_CLASSIC_DATA {
        let ral_lab = Lab::new(lab_l, lab_a, lab_b);
        let distance = ColorUtils::lab_distance(target_lab, ral_lab);
        
        matches.push(RalMatch {
            code: code.to_string(),
            name: name.to_string(),
            hex: hex.to_string(),
            distance,
            classification: RalClassification::Classic,
        });
    }
    
    // Sort by distance and return top results
    matches.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    matches.truncate(max_results);
    matches
}

/// Find the two closest RAL Design System+ colors to the given RGB color
pub fn find_closest_ral_design(rgb: &RgbColor, max_results: usize) -> Vec<RalMatch> {
    let target_lab = rgb.to_lab();
    let mut matches = Vec::new();
    
    for &(name, code, [r, g, b], _hue, _lightness, _chromaticity) in RAL_DESIGN_DATA {
        // Convert RAL Design RGB to LAB for comparison
        let ral_rgb = RgbColor::new(r, g, b);
        let ral_lab = ral_rgb.to_lab();
        let distance = ColorUtils::lab_distance(target_lab, ral_lab);
        
        // Generate hex color from RGB
        let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
        
        matches.push(RalMatch {
            code: code.to_string(),
            name: name.to_string(),
            hex,
            distance,
            classification: RalClassification::DesignSystem,
        });
    }
    
    // Sort by distance and return top results
    matches.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    matches.truncate(max_results);
    matches
}

/// Find closest colors from both RAL classifications
pub fn find_closest_ral_colors(rgb: &RgbColor, max_results: usize) -> Vec<RalMatch> {
    let mut all_matches = Vec::new();
    
    // Get matches from both classifications
    all_matches.extend(find_closest_ral_classic(rgb, RAL_CLASSIC_DATA.len()));
    all_matches.extend(find_closest_ral_design(rgb, RAL_DESIGN_DATA.len()));
    
    // Sort by distance and return top results
    all_matches.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    all_matches.truncate(max_results);
    all_matches
}

/// Parse RAL Classic code (e.g., "RAL2013", "RAL 2013")
pub fn parse_ral_classic_code(input: &str) -> Option<RalMatch> {
    static RAL_CLASSIC_REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = RAL_CLASSIC_REGEX.get_or_init(|| {
        Regex::new(r"(?i)^RAL\s*(\d{4})$").unwrap()
    });
    
    if let Some(caps) = regex.captures(input.trim()) {
        let number = caps.get(1).unwrap().as_str();
        let code = format!("RAL {}", number);
        
        // Search for the exact code in RAL Classic data
        for &(ral_code, name, hex, _lab_l, _lab_a, _lab_b, _cmyk_c, _cmyk_m, _cmyk_y, _cmyk_k, _lrv) in RAL_CLASSIC_DATA {
            if ral_code == code {
                return Some(RalMatch {
                    code: ral_code.to_string(),
                    name: name.to_string(),
                    hex: hex.to_string(),
                    distance: 0.0, // Exact match
                    classification: RalClassification::Classic,
                });
            }
        }
    }
    
    None
}

/// Parse RAL Design System+ code (e.g., "H010L20C10")
pub fn parse_ral_design_code(input: &str) -> Option<RalMatch> {
    static RAL_DESIGN_REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = RAL_DESIGN_REGEX.get_or_init(|| {
        Regex::new(r"(?i)^H(\d{3})L(\d{2})C(\d{2})$").unwrap()
    });
    
    if let Some(_caps) = regex.captures(input.trim()) {
        let search_code = input.trim().to_uppercase();
        
        // Search for the exact code in RAL Design data
        for &(name, code, [r, g, b], _hue, _lightness, _chromaticity) in RAL_DESIGN_DATA {
            if code == search_code {
                let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
                return Some(RalMatch {
                    code: code.to_string(),
                    name: name.to_string(),
                    hex,
                    distance: 0.0, // Exact match
                    classification: RalClassification::DesignSystem,
                });
            }
        }
    }
    
    None
}

/// Find RAL color by name (case-insensitive partial match)
pub fn find_ral_by_name(name: &str) -> Vec<RalMatch> {
    let search_name = name.to_lowercase();
    let mut matches = Vec::new();
    
    // Search RAL Classic colors
    for &(code, ral_name, hex, _lab_l, _lab_a, _lab_b, _cmyk_c, _cmyk_m, _cmyk_y, _cmyk_k, _lrv) in RAL_CLASSIC_DATA {
        if ral_name.to_lowercase().contains(&search_name) {
            matches.push(RalMatch {
                code: code.to_string(),
                name: ral_name.to_string(),
                hex: hex.to_string(),
                distance: 0.0, // Name match, no distance
                classification: RalClassification::Classic,
            });
        }
    }
    
    // Search RAL Design System+ colors
    for &(ral_name, code, [r, g, b], _hue, _lightness, _chromaticity) in RAL_DESIGN_DATA {
        if ral_name.to_lowercase().contains(&search_name) {
            let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
            matches.push(RalMatch {
                code: code.to_string(),
                name: ral_name.to_string(),
                hex,
                distance: 0.0, // Name match, no distance
                classification: RalClassification::DesignSystem,
            });
        }
    }
    
    matches
}

/// Main RAL color parsing function - tries all formats
pub fn parse_ral_color(input: &str) -> Option<RalMatch> {
    // Try RAL Classic code first (RAL XXXX format)
    if let Some(color) = parse_ral_classic_code(input) {
        return Some(color);
    }
    
    // Try RAL Design System+ code (HXXXLXXCXX format)
    if let Some(color) = parse_ral_design_code(input) {
        return Some(color);
    }
    
    // For name search, avoid common CSS color names to prevent conflicts
    let input_lower = input.to_lowercase();
    let common_css_colors = [
        "red", "green", "blue", "black", "white", "yellow", "cyan", "magenta",
        "orange", "purple", "pink", "brown", "gray", "grey", "navy", "lime",
        "olive", "maroon", "teal", "silver", "aqua", "fuchsia"
    ];
    
    if common_css_colors.contains(&input_lower.as_str()) {
        return None;
    }
    
    // Try name search for non-CSS color names
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
        // Test RAL Design System+ code format
        assert!(parse_ral_design_code("H000L15C00").is_some());
        assert!(parse_ral_design_code("h000l15c00").is_some());
        assert!(parse_ral_design_code("invalid").is_none());
    }
    
    #[test]
    fn test_find_ral_by_name() {
        let matches = find_ral_by_name("red");
        assert!(!matches.is_empty());
        
        let matches = find_ral_by_name("beige");
        assert!(!matches.is_empty());
        assert!(matches.iter().any(|m| m.name.to_lowercase().contains("beige")));
    }
    
    #[test]
    fn test_closest_color_matching() {
        // Test with a known color (approximate red)
        let red = RgbColor::new(255, 0, 0);
        let matches = find_closest_ral_colors(&red, 2);
        assert_eq!(matches.len(), 2);
        assert!(matches[0].distance <= matches[1].distance);
    }
}
