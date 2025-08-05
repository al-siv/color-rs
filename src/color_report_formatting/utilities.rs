//! Utility functions for color collection matching and enhanced data generation
//!
//! Provides functions for matching colors against various collections and creating
//! enhanced color scheme data with metadata.

use crate::color_schemes::ColorSchemeResult;
use crate::color_distance_strategies::DistanceAlgorithm;
use crate::output_formats::{CollectionMatch, ColorSchemes, EnhancedColorSchemeItem};
use palette::Lab;

use super::core::{lab_to_hex, lab_to_hsl_tuple, lab_to_rgb, rgb_to_lab, rgb_to_srgb};

/// Collect enhanced color schemes data for new flattened file output
#[must_use]
pub fn collect_enhanced_color_schemes_data(
    schemes: &ColorSchemeResult,
    strategy: &str,
    distance_algorithm: DistanceAlgorithm,
) -> ColorSchemes {
    use crate::color_parser::unified_manager::UnifiedColorManager;

    // Create manager for color matching with strategy support
    let manager = UnifiedColorManager::new().unwrap_or_default();

    // Select the appropriate strategy schemes
    let selected_schemes = match strategy {
        "hsl" => (
            schemes.hsl_complementary,
            schemes.hsl_split_complementary,
            schemes.hsl_triadic,
            schemes.hsl_tetradic,
        ),
        _ => (
            schemes.lab_complementary,
            schemes.lab_split_complementary,
            schemes.lab_triadic,
            schemes.lab_tetradic,
        ),
    };

    ColorSchemes {
        complementary: lab_to_enhanced_item(selected_schemes.0, &manager, distance_algorithm),
        split_complementary: vec![
            lab_to_enhanced_item(selected_schemes.1.0, &manager, distance_algorithm),
            lab_to_enhanced_item(selected_schemes.1.1, &manager, distance_algorithm),
        ],
        triadic: vec![
            lab_to_enhanced_item(selected_schemes.2.0, &manager, distance_algorithm),
            lab_to_enhanced_item(selected_schemes.2.1, &manager, distance_algorithm),
        ],
        tetradic: vec![
            lab_to_enhanced_item(selected_schemes.3.0, &manager, distance_algorithm),
            lab_to_enhanced_item(selected_schemes.3.1, &manager, distance_algorithm),
            lab_to_enhanced_item(selected_schemes.3.2, &manager, distance_algorithm),
        ],
    }
}

/// Convert a Lab color to an `EnhancedColorSchemeItem` with full color information
fn lab_to_enhanced_item(
    color: Lab,
    manager: &crate::color_parser::unified_manager::UnifiedColorManager,
    distance_algorithm: DistanceAlgorithm,
) -> EnhancedColorSchemeItem {
    use crate::color_parser::UniversalColor;

    let hex = lab_to_hex(color);
    let hsl_tuple = lab_to_hsl_tuple(color);
    let hsl = format!(
        "hsl({:.1}, {:.2}%, {:.2}%)",
        hsl_tuple.0,
        hsl_tuple.1 * 100.0,
        hsl_tuple.2 * 100.0
    );
    let lch = crate::format_utils::FormatUtils::lab_to_lch(color);

    // Get color name information with enhanced collection matches
    let (r, g, b) = lab_to_rgb(color);
    let target = UniversalColor::from_rgb([r, g, b]);

    // Get collection matches
    let css_match = get_closest_css_match(&target, manager, distance_algorithm);
    let ral_classic_match = get_closest_ral_classic_match(&target, manager, distance_algorithm);
    let ral_design_match = get_closest_ral_design_match(&target, manager, distance_algorithm);

    EnhancedColorSchemeItem {
        hex,
        hsl,
        lch,
        css: css_match,
        ral_classic: ral_classic_match,
        ral_design: ral_design_match,
    }
}

/// Get closest CSS collection match using distance strategy
fn get_closest_css_match(
    target: &crate::color_parser::UniversalColor,
    manager: &crate::color_parser::unified_manager::UnifiedColorManager,
    distance_algorithm: DistanceAlgorithm,
) -> Option<CollectionMatch> {
    let rgb = [target.rgb[0], target.rgb[1], target.rgb[2]];
    let matches = manager.find_closest_css_colors_with_algorithm(rgb, 1, distance_algorithm);
    
    matches.first().map(|closest| {
        let target_lab = rgb_to_lab((target.rgb[0], target.rgb[1], target.rgb[2]));
        let closest_lab = rgb_to_lab((
            closest.entry.color.rgb[0],
            closest.entry.color.rgb[1],
            closest.entry.color.rgb[2],
        ));
        let distance = crate::color_distance_strategies::calculate_distance(distance_algorithm, target_lab, closest_lab);
        let srgb = rgb_to_srgb((
            closest.entry.color.rgb[0],
            closest.entry.color.rgb[1],
            closest.entry.color.rgb[2],
        ));
        let wcag_relative_luminance = crate::color_ops::luminance::wcag_relative(srgb);
        
        CollectionMatch {
            name: closest.entry.metadata.name.clone(),
            hex: format!(
                "#{:02X}{:02X}{:02X}",
                closest.entry.color.rgb[0],
                closest.entry.color.rgb[1],
                closest.entry.color.rgb[2]
            ),
            distance,
            wcag_relative_luminance,
        }
    })
}

/// Get closest RAL Classic collection match using distance strategy
fn get_closest_ral_classic_match(
    target: &crate::color_parser::UniversalColor,
    manager: &crate::color_parser::unified_manager::UnifiedColorManager,
    distance_algorithm: DistanceAlgorithm,
) -> Option<CollectionMatch> {
    let rgb = [target.rgb[0], target.rgb[1], target.rgb[2]];
    let matches = manager.find_closest_ral_classic_with_algorithm(rgb, 1, distance_algorithm);
    
    matches.first().map(|closest| {
        let target_lab = rgb_to_lab((target.rgb[0], target.rgb[1], target.rgb[2]));
        let closest_lab = rgb_to_lab((
            closest.entry.color.rgb[0],
            closest.entry.color.rgb[1],
            closest.entry.color.rgb[2],
        ));
        let distance = crate::color_distance_strategies::calculate_distance(distance_algorithm, target_lab, closest_lab);
        let srgb = rgb_to_srgb((
            closest.entry.color.rgb[0],
            closest.entry.color.rgb[1],
            closest.entry.color.rgb[2],
        ));
        let wcag_relative_luminance = crate::color_ops::luminance::wcag_relative(srgb);
        
        CollectionMatch {
            name: closest.entry.metadata.name.clone(),
            hex: format!(
                "#{:02X}{:02X}{:02X}",
                closest.entry.color.rgb[0],
                closest.entry.color.rgb[1],
                closest.entry.color.rgb[2]
            ),
            distance,
            wcag_relative_luminance,
        }
    })
}

/// Get closest RAL Design collection match using distance strategy
fn get_closest_ral_design_match(
    target: &crate::color_parser::UniversalColor,
    manager: &crate::color_parser::unified_manager::UnifiedColorManager,
    distance_algorithm: DistanceAlgorithm,
) -> Option<CollectionMatch> {
    let rgb = [target.rgb[0], target.rgb[1], target.rgb[2]];
    let matches = manager.find_closest_ral_design_with_algorithm(rgb, 1, distance_algorithm);
    
    matches.first().map(|closest| {
        let target_lab = rgb_to_lab((target.rgb[0], target.rgb[1], target.rgb[2]));
        let closest_lab = rgb_to_lab((
            closest.entry.color.rgb[0],
            closest.entry.color.rgb[1],
            closest.entry.color.rgb[2],
        ));
        let distance = crate::color_distance_strategies::calculate_distance(distance_algorithm, target_lab, closest_lab);
        let srgb = rgb_to_srgb((
            closest.entry.color.rgb[0],
            closest.entry.color.rgb[1],
            closest.entry.color.rgb[2],
        ));
        let wcag_relative_luminance = crate::color_ops::luminance::wcag_relative(srgb);
        
        CollectionMatch {
            name: closest.entry.metadata.name.clone(),
            hex: format!(
                "#{:02X}{:02X}{:02X}",
                closest.entry.color.rgb[0],
                closest.entry.color.rgb[1],
                closest.entry.color.rgb[2]
            ),
            distance,
            wcag_relative_luminance,
        }
    })
}
