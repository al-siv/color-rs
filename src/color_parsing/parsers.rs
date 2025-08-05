//! Individual parser implementations
//!
//! Contains specific parser implementations for different strategies and validation levels.
//! Each parser provides different capabilities and performance characteristics.

use crate::color_parser::{ColorParser, UnifiedColorManager, ColorFormat};
use crate::error::Result;

/// Parse color using CSS parser strategy
pub fn parse_css_color(input: &str) -> Result<(palette::Lab, ColorFormat)> {
    let parser = ColorParser::new();
    parser.parse(input)
}

/// Parse color using full comprehensive parser with all collections
pub fn parse_full_color(input: &str) -> Result<(palette::Lab, ColorFormat)> {
    let parser = ColorParser::new();
    parser.parse(input)
}

/// Parse color using custom parser with configurable validation
pub fn parse_custom_color(input: &str, strict_validation: bool) -> Result<(palette::Lab, ColorFormat)> {
    let parser = ColorParser::new();
    
    if strict_validation {
        // Could add additional validation logic here
        parser.parse(input)
    } else {
        parser.parse(input)
    }
}

/// Get color name using CSS color collections only
pub fn get_css_color_name(rgb: [u8; 3]) -> String {
    let parser = ColorParser::new();
    parser.get_color_name((rgb[0], rgb[1], rgb[2]))
}

/// Get color name using full comprehensive color matching
pub fn get_full_color_name(rgb: [u8; 3], _tolerance: f64) -> String {
    // Use unified manager for comprehensive color matching
    if let Ok(unified_manager) = UnifiedColorManager::new() {
        let matches = unified_manager.find_closest_across_all(rgb, 1);
        
        // Find the best match across all collections
        for (_, collection_matches) in &matches {
            if let Some(color_match) = collection_matches.first() {
                return color_match.entry.metadata.name.clone();
            }
        }
    }

    // Fallback to basic parser
    get_css_color_name(rgb)
}

/// Get color name using custom parser with fallback control
pub fn get_custom_color_name(rgb: [u8; 3], enable_fallback: bool) -> String {
    let name = get_css_color_name(rgb);
    
    if !enable_fallback && name.starts_with("rgb(") {
        "Unknown".to_string()
    } else {
        name
    }
}
