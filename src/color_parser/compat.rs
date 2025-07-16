//! Backward Compatibility Layer
//!
//! This module provides backward compatibility with the existing API while using
//! the new unified color collection system under the hood.

use super::collections::ColorMatch as NewColorMatch;
use super::ral_matcher::{RalClassification, RalMatch};
use super::unified_manager::UnifiedColorManager;

/// Lazy static unified manager for backward compatibility
static UNIFIED_MANAGER: std::sync::LazyLock<UnifiedColorManager> =
    std::sync::LazyLock::new(|| UnifiedColorManager::new());

/// Convert new ColorMatch to old RalMatch for backward compatibility
fn color_match_to_ral_match(
    color_match: &NewColorMatch,
    classification: RalClassification,
) -> RalMatch {
    RalMatch {
        code: color_match.entry.metadata.code.clone().unwrap_or_default(),
        name: color_match.entry.metadata.name.clone(),
        hex: color_match
            .entry
            .metadata
            .extra_data
            .get("hex")
            .cloned()
            .unwrap_or_else(|| {
                format!(
                    "#{:02x}{:02x}{:02x}",
                    color_match.entry.color.rgb[0],
                    color_match.entry.color.rgb[1],
                    color_match.entry.color.rgb[2]
                )
            }),
        distance: color_match.distance,
        classification,
    }
}

/// Find closest RAL Classic colors (backward compatibility)
pub fn find_closest_ral_classic_compat(
    rgb: &crate::color_parser::RgbColor,
    max_results: usize,
) -> Vec<RalMatch> {
    let rgb_array = [rgb.r, rgb.g, rgb.b];
    let matches = UNIFIED_MANAGER.find_closest_ral_classic(rgb_array, max_results);

    matches
        .iter()
        .map(|m| color_match_to_ral_match(m, RalClassification::Classic))
        .collect()
}

/// Find closest RAL Design System+ colors (backward compatibility)
pub fn find_closest_ral_design_compat(
    rgb: &crate::color_parser::RgbColor,
    max_results: usize,
) -> Vec<RalMatch> {
    let rgb_array = [rgb.r, rgb.g, rgb.b];
    let matches = UNIFIED_MANAGER.find_closest_ral_design(rgb_array, max_results);

    matches
        .iter()
        .map(|m| color_match_to_ral_match(m, RalClassification::DesignSystem))
        .collect()
}

/// Find closest RAL colors from both collections (backward compatibility)
pub fn find_closest_ral_colors_compat(
    rgb: &crate::color_parser::RgbColor,
    max_results: usize,
) -> Vec<RalMatch> {
    let rgb_array = [rgb.r, rgb.g, rgb.b];

    // Get results from both collections
    let classic_matches = UNIFIED_MANAGER.find_closest_ral_classic(rgb_array, max_results);
    let design_matches = UNIFIED_MANAGER.find_closest_ral_design(rgb_array, max_results);

    // Convert and combine
    let mut all_matches: Vec<RalMatch> = Vec::new();

    all_matches.extend(
        classic_matches
            .iter()
            .map(|m| color_match_to_ral_match(m, RalClassification::Classic)),
    );

    all_matches.extend(
        design_matches
            .iter()
            .map(|m| color_match_to_ral_match(m, RalClassification::DesignSystem)),
    );

    // Sort by distance and limit results
    all_matches.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    all_matches.truncate(max_results);

    all_matches
}

/// Find RAL color by exact code (backward compatibility)
pub fn find_ral_by_code_compat(code: &str) -> Option<RalMatch> {
    if let Some((collection_name, entry)) = UNIFIED_MANAGER.find_by_code(code) {
        let classification = if collection_name == "RAL Classic" {
            RalClassification::Classic
        } else {
            RalClassification::DesignSystem
        };

        Some(RalMatch {
            code: entry.metadata.code.unwrap_or_default(),
            name: entry.metadata.name,
            hex: entry
                .metadata
                .extra_data
                .get("hex")
                .cloned()
                .unwrap_or_else(|| {
                    format!(
                        "#{:02x}{:02x}{:02x}",
                        entry.color.rgb[0], entry.color.rgb[1], entry.color.rgb[2]
                    )
                }),
            distance: 0.0, // Exact match
            classification,
        })
    } else {
        None
    }
}

/// Find RAL colors by name pattern (backward compatibility)  
pub fn find_ral_by_name_pattern_compat(name_pattern: &str) -> Vec<RalMatch> {
    let results = UNIFIED_MANAGER.find_ral_by_name_pattern(name_pattern);
    let mut matches = Vec::new();

    for (collection_name, entry) in results {
        let classification = if collection_name == "RAL Classic" {
            RalClassification::Classic
        } else {
            RalClassification::DesignSystem
        };

        matches.push(RalMatch {
            code: entry.metadata.code.unwrap_or_default(),
            name: entry.metadata.name,
            hex: entry
                .metadata
                .extra_data
                .get("hex")
                .cloned()
                .unwrap_or_else(|| {
                    format!(
                        "#{:02x}{:02x}{:02x}",
                        entry.color.rgb[0], entry.color.rgb[1], entry.color.rgb[2]
                    )
                }),
            distance: 0.0, // Name match
            classification,
        });
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color_parser::RgbColor;

    #[test]
    fn test_backward_compatibility() {
        let red = RgbColor { r: 255, g: 0, b: 0 };

        // Test classic compatibility
        let classic_matches = find_closest_ral_classic_compat(&red, 2);
        assert!(!classic_matches.is_empty());
        assert!(
            classic_matches
                .iter()
                .all(|m| m.classification == RalClassification::Classic)
        );

        // Test design compatibility
        let design_matches = find_closest_ral_design_compat(&red, 2);
        assert!(!design_matches.is_empty());
        assert!(
            design_matches
                .iter()
                .all(|m| m.classification == RalClassification::DesignSystem)
        );

        // Test combined compatibility
        let all_matches = find_closest_ral_colors_compat(&red, 4);
        assert!(!all_matches.is_empty());
        assert!(all_matches.len() <= 4);
    }

    #[test]
    fn test_find_by_code_compatibility() {
        let ral1000 = find_ral_by_code_compat("RAL 1000");
        assert!(ral1000.is_some());
        let match_result = ral1000.unwrap();
        assert_eq!(match_result.code, "RAL 1000");
        assert_eq!(match_result.classification, RalClassification::Classic);

        let hlc_color = find_ral_by_code_compat("H010L20C10");
        assert!(hlc_color.is_some());
        let match_result = hlc_color.unwrap();
        assert_eq!(match_result.code, "H010L20C10");
        assert_eq!(match_result.classification, RalClassification::DesignSystem);
    }
}
