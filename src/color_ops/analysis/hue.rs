//! Hue analysis module for color harmony and hue-based color relationships
//!
//! This module provides pure functional logic for analyzing colors based on their hue
//! relationships, implementing color harmony theory and hue-based filtering and sorting.
//!
//! ## Core Functions
//! - `analyze_hue_relationships()` - Analyze hue relationships between colors
//! - `filter_by_hue_criteria()` - Filter colors based on hue, saturation, and lightness criteria
//! - `sort_by_criteria()` - Sort colors by various criteria (hue distance, saturation, etc.)
//! - `calculate_hue_distance()` - Calculate perceptual hue distance between colors
//!
//! ## Design Principles
//! - All functions are pure with no side effects
//! - Immutable input parameters
//! - Explicit error handling with Result types
//! - Functional composition and pipeline patterns
//!
//! ## Example Usage
//! ```rust
//! use color_rs::color_ops::analysis::hue::{HueAnalysisOptions, analyze_hue_relationships};
//! use palette::{Lch, Srgb};
//!
//! let input_color = Lch::new(50.0, 40.0, 120.0);
//! let color_collection = vec![
//!     Lch::new(45.0, 35.0, 115.0),
//!     Lch::new(55.0, 45.0, 125.0),
//! ];
//!
//! let options = HueAnalysisOptions {
//!     target_hue: Some(120.0),
//!     tolerance: 15.0,
//!     min_saturation: Some(20.0),
//!     min_lightness: Some(30.0),
//! };
//!
//! let result = analyze_hue_relationships(&input_color, &color_collection, &options);
//! ```

use crate::color_parser::collections::ColorCollection;
use crate::color_parser::{CssColorCollection, RalClassicCollection, RalDesignCollection};
use crate::error::{ColorError, Result};
use palette::{IntoColor, Lch, Srgb};
use serde::{Deserialize, Serialize};

/// Color collection selection for hue analysis
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorCollectionType {
    /// CSS named colors collection
    Css,
    /// RAL Classic color collection
    RalClassic,
    /// RAL Design color collection
    RalDesign,
    /// All collections combined
    All,
}

impl std::str::FromStr for ColorCollectionType {
    type Err = ColorError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "css" => Ok(Self::Css),
            "ral-classic" | "ralc" => Ok(Self::RalClassic),
            "ral-design" | "rald" => Ok(Self::RalDesign),
            "all" => Ok(Self::All),
            _ => Err(ColorError::InvalidArguments(format!(
                "Invalid collection '{}'. Valid options: css, ral-classic, ral-design, all",
                s
            ))),
        }
    }
}

impl ColorCollectionType {
    /// Get all collection types for "all" selection
    #[must_use]
    pub fn all_collections() -> Vec<Self> {
        vec![Self::Css, Self::RalClassic, Self::RalDesign]
    }
}

/// Options for hue analysis operations
#[derive(Debug, Clone, PartialEq)]
pub struct HueAnalysisOptions {
    /// Target hue angle for analysis (0-360 degrees)
    pub target_hue: Option<f64>,
    /// Tolerance range for hue matching (±degrees from target)
    pub tolerance: f64,
    /// Minimum saturation threshold (0-100%)
    pub min_saturation: Option<f64>,
    /// Minimum lightness threshold (0-100%)
    pub min_lightness: Option<f64>,
}

/// Sort criteria for hue analysis results
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortCriteria {
    /// Sort by hue distance from target or input color
    HueDistance,
    /// Sort by saturation level (chroma)
    Saturation,
    /// Sort by lightness level
    Lightness,
    /// Sort alphabetically by color name
    Name,
}

impl std::str::FromStr for SortCriteria {
    type Err = ColorError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "hue-distance" => Ok(Self::HueDistance),
            "saturation" => Ok(Self::Saturation),
            "lightness" => Ok(Self::Lightness),
            "name" => Ok(Self::Name),
            _ => Err(ColorError::InvalidArguments(format!(
                "Invalid sort criteria: {}. Valid options: hue-distance, saturation, lightness, name",
                s
            ))),
        }
    }
}

/// Result of hue analysis containing color information and metrics
#[derive(Debug, Clone, PartialEq)]
pub struct HueAnalysisResult {
    /// Original color in LCH color space
    pub color: Lch,
    /// Color name if available
    pub name: Option<String>,
    /// Hue distance from target or reference color
    pub hue_distance: f64,
    /// Saturation level (chroma value)
    pub saturation: f64,
    /// Lightness level
    pub lightness: f64,
    /// Collection source (css, ral-classic, ral-design)
    pub collection: String,
}

/// Calculate the shortest angular distance between two hue values
///
/// Handles hue wraparound (360° = 0°) to find the shortest path around the color wheel.
///
/// # Arguments
/// * `hue1` - First hue value in degrees (0-360)
/// * `hue2` - Second hue value in degrees (0-360)
///
/// # Returns
/// The shortest angular distance between the hues (0-180 degrees)
///
/// # Examples
/// ```rust
/// use color_rs::color_ops::analysis::hue::calculate_hue_distance;
///
/// assert_eq!(calculate_hue_distance(10.0, 350.0), 20.0);
/// assert_eq!(calculate_hue_distance(180.0, 0.0), 180.0);
/// assert_eq!(calculate_hue_distance(100.0, 120.0), 20.0);
/// ```
#[must_use]
pub fn calculate_hue_distance(hue1: f64, hue2: f64) -> f64 {
    let diff = (hue1 - hue2).abs();
    diff.min(360.0 - diff)
}

/// Normalize hue value to 0-360 degree range
///
/// Handles negative values and values greater than 360 degrees.
///
/// # Arguments
/// * `hue` - Hue value in degrees (can be any value)
///
/// # Returns
/// Normalized hue value in 0-360 degree range
///
/// # Examples
/// ```rust
/// use color_rs::color_ops::analysis::hue::normalize_hue;
///
/// assert_eq!(normalize_hue(-30.0), 330.0);
/// assert_eq!(normalize_hue(400.0), 40.0);
/// assert_eq!(normalize_hue(180.0), 180.0);
/// ```
#[must_use]
pub fn normalize_hue(hue: f64) -> f64 {
    ((hue % 360.0) + 360.0) % 360.0
}

/// Check if a color meets the specified criteria
///
/// Applies filters for hue tolerance, minimum saturation, and minimum lightness.
///
/// # Arguments
/// * `color` - Color to check in LCH color space
/// * `reference_hue` - Reference hue for distance calculation (if applicable)
/// * `options` - Analysis options containing filter criteria
///
/// # Returns
/// `true` if the color meets all specified criteria, `false` otherwise
pub fn meets_criteria(
    color: &Lch,
    reference_hue: Option<f64>,
    options: &HueAnalysisOptions,
) -> bool {
    // Check hue tolerance if target hue is specified
    if let (Some(_target_hue), Some(ref_hue)) = (options.target_hue, reference_hue) {
        let hue_dist = calculate_hue_distance(f64::from(color.hue.into_degrees()), ref_hue);
        if hue_dist > options.tolerance {
            return false;
        }
    }

    // Check minimum saturation (chroma in LCH)
    if let Some(min_sat) = options.min_saturation {
        if f64::from(color.chroma) < min_sat {
            return false;
        }
    }

    // Check minimum lightness
    if let Some(min_light) = options.min_lightness {
        if f64::from(color.l) < min_light {
            return false;
        }
    }

    true
}

/// Filter a collection of colors based on hue analysis criteria
///
/// Applies all specified filters and returns only colors that meet the criteria.
///
/// # Arguments
/// * `colors` - Collection of colors with metadata to filter
/// * `reference_hue` - Reference hue for distance-based filtering
/// * `options` - Analysis options containing filter criteria
///
/// # Returns
/// Filtered vector of colors that meet all criteria
pub fn filter_by_hue_criteria(
    colors: &[HueAnalysisResult],
    reference_hue: Option<f64>,
    options: &HueAnalysisOptions,
) -> Vec<HueAnalysisResult> {
    colors
        .iter()
        .filter(|result| meets_criteria(&result.color, reference_hue, options))
        .cloned()
        .collect()
}

/// Sort colors by the specified criteria
///
/// Implements various sorting methods for hue analysis results.
///
/// # Arguments
/// * `colors` - Mutable reference to vector of hue analysis results
/// * `criteria` - Sort criteria to apply
/// * `reference_hue` - Reference hue for hue distance sorting (if applicable)
pub fn sort_by_criteria(
    colors: &mut [HueAnalysisResult],
    criteria: SortCriteria,
    reference_hue: Option<f64>,
) {
    match criteria {
        SortCriteria::HueDistance => {
            if let Some(ref_hue) = reference_hue {
                colors.sort_by(|a, b| {
                    let dist_a =
                        calculate_hue_distance(f64::from(a.color.hue.into_degrees()), ref_hue);
                    let dist_b =
                        calculate_hue_distance(f64::from(b.color.hue.into_degrees()), ref_hue);
                    dist_a
                        .partial_cmp(&dist_b)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                // Sort by hue value if no reference
                colors.sort_by(|a, b| {
                    a.color
                        .hue
                        .into_degrees()
                        .partial_cmp(&b.color.hue.into_degrees())
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        SortCriteria::Saturation => {
            colors.sort_by(|a, b| {
                b.color
                    .chroma
                    .partial_cmp(&a.color.chroma)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }
        SortCriteria::Lightness => {
            colors.sort_by(|a, b| {
                b.color
                    .l
                    .partial_cmp(&a.color.l)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }
        SortCriteria::Name => {
            colors.sort_by(|a, b| match (&a.name, &b.name) {
                (Some(name_a), Some(name_b)) => name_a.cmp(name_b),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            });
        }
    }
}

/// Convert RGB color to LCH and create analysis result
///
/// Utility function for creating hue analysis results from RGB colors.
///
/// # Arguments
/// * `rgb` - RGB color value
/// * `name` - Optional color name
/// * `collection` - Collection name (css, ral-classic, ral-design)
/// * `reference_hue` - Reference hue for distance calculation
///
/// # Returns
/// Hue analysis result with calculated metrics
pub fn create_analysis_result(
    rgb: Srgb,
    name: Option<String>,
    collection: String,
    reference_hue: Option<f64>,
) -> HueAnalysisResult {
    let lch: Lch = rgb.into_color();
    let hue_distance = reference_hue
        .map(|ref_hue| calculate_hue_distance(f64::from(lch.hue.into_degrees()), ref_hue))
        .unwrap_or(0.0);

    HueAnalysisResult {
        color: lch,
        name,
        hue_distance,
        saturation: f64::from(lch.chroma),
        lightness: f64::from(lch.l),
        collection,
    }
}

/// Main hue analysis function
///
/// Analyzes a collection of colors based on hue relationships with an input color.
/// Applies filtering, sorting, and limiting according to the provided options.
///
/// # Arguments
/// * `input_color` - Reference color in LCH color space
/// * `color_collection` - Collection of colors to analyze
/// * `options` - Analysis options and filtering criteria
/// * `sort_criteria` - How to sort the results
/// * `limit` - Maximum number of results to return
///
/// # Returns
/// Result containing sorted and filtered hue analysis results
///
/// # Errors
/// Returns `ColorError` if any processing step fails
pub fn analyze_hue_relationships(
    input_color: &Lch,
    color_collection: &[HueAnalysisResult],
    options: &HueAnalysisOptions,
    sort_criteria: SortCriteria,
    limit: usize,
) -> Result<Vec<HueAnalysisResult>> {
    // Determine reference hue (target hue or input color hue)
    let reference_hue = options
        .target_hue
        .unwrap_or_else(|| f64::from(input_color.hue.into_degrees()));

    // Filter colors based on criteria
    let mut filtered_colors =
        filter_by_hue_criteria(color_collection, Some(reference_hue), options);

    // Update hue distances based on reference hue
    for result in &mut filtered_colors {
        result.hue_distance =
            calculate_hue_distance(f64::from(result.color.hue.into_degrees()), reference_hue);
    }

    // Sort by specified criteria
    sort_by_criteria(&mut filtered_colors, sort_criteria, Some(reference_hue));

    // Apply limit
    filtered_colors.truncate(limit);

    Ok(filtered_colors)
}

/// Load colors from a specific collection
///
/// Pure function that loads and converts colors from the specified collection type.
///
/// # Arguments
/// * `collection_type` - Type of collection to load
///
/// # Returns
/// Result containing vector of hue analysis results from the collection
///
/// # Errors
/// Returns `ColorError` if collection loading fails
pub fn load_collection_colors(
    collection_type: &ColorCollectionType,
) -> Result<Vec<HueAnalysisResult>> {
    match collection_type {
        ColorCollectionType::Css => {
            let collection = CssColorCollection::new().map_err(|e| {
                ColorError::InvalidArguments(format!("Failed to load CSS collection: {}", e))
            })?;
            Ok(convert_collection_to_results(&collection, "css"))
        }
        ColorCollectionType::RalClassic => {
            let collection = RalClassicCollection::new().map_err(|e| {
                ColorError::InvalidArguments(format!(
                    "Failed to load RAL Classic collection: {}",
                    e
                ))
            })?;
            Ok(convert_collection_to_results(&collection, "ral-classic"))
        }
        ColorCollectionType::RalDesign => {
            let collection = RalDesignCollection::new().map_err(|e| {
                ColorError::InvalidArguments(format!("Failed to load RAL Design collection: {}", e))
            })?;
            Ok(convert_collection_to_results(&collection, "ral-design"))
        }
        ColorCollectionType::All => {
            let mut all_colors = Vec::new();

            // Load all collections
            for collection_type in &ColorCollectionType::all_collections() {
                let mut colors = load_collection_colors(collection_type)?;
                all_colors.append(&mut colors);
            }

            Ok(all_colors)
        }
    }
}

/// Convert a color collection to hue analysis results
///
/// Helper function to convert from the collection trait to our domain types.
///
/// # Arguments
/// * `collection` - Color collection implementing the `ColorCollection` trait
/// * `collection_name` - Name identifier for the collection
///
/// # Returns
/// Vector of hue analysis results
fn convert_collection_to_results(
    collection: &dyn ColorCollection,
    collection_name: &str,
) -> Vec<HueAnalysisResult> {
    collection
        .colors()
        .iter()
        .map(|entry| {
            let rgb = Srgb::new(
                f32::from(entry.color.rgb[0]) / 255.0,
                f32::from(entry.color.rgb[1]) / 255.0,
                f32::from(entry.color.rgb[2]) / 255.0,
            );
            create_analysis_result(
                rgb,
                Some(entry.metadata.name.clone()),
                collection_name.to_string(),
                None, // No reference hue initially
            )
        })
        .collect()
}

/// Analyze hue relationships within a specific color collection
///
/// Main integration function that loads a collection and performs hue analysis.
///
/// # Arguments
/// * `collection_type` - Type of collection to analyze
/// * `input_color` - Reference color for analysis
/// * `options` - Analysis options and filtering criteria
/// * `sort_criteria` - How to sort the results
/// * `limit` - Maximum number of results to return
///
/// # Returns
/// Result containing sorted and filtered hue analysis results
///
/// # Errors
/// Returns `ColorError` if collection loading or analysis fails
pub fn analyze_collection_hues(
    collection_type: &ColorCollectionType,
    input_color: &Lch,
    options: &HueAnalysisOptions,
    sort_criteria: SortCriteria,
    limit: usize,
) -> Result<Vec<HueAnalysisResult>> {
    // Load colors from the specified collection
    let color_collection = load_collection_colors(collection_type)?;

    // Perform hue analysis using the main analysis function
    analyze_hue_relationships(
        input_color,
        &color_collection,
        options,
        sort_criteria,
        limit,
    )
}

/// Display item for hue analysis terminal output
///
/// Structured representation of a single color for terminal display.
#[derive(Debug, Clone, PartialEq)]
pub struct HueDisplayItem {
    /// Hue angle in degrees (0-360)
    pub hue: f64,
    /// Color code or identifier
    pub code: String,
    /// Hexadecimal color representation
    pub hex: String,
    /// LCH color space values
    pub lch: String,
    /// Color name
    pub name: String,
    /// Hue shift from previous color (optional)
    pub hue_shift: Option<f64>,
}

impl HueDisplayItem {
    /// Create a new hue display item from analysis result
    ///
    /// # Arguments
    /// * `result` - Hue analysis result to convert
    /// * `previous_hue` - Previous color's hue for calculating shift
    ///
    /// # Returns
    /// New `HueDisplayItem` for display
    pub fn from_analysis_result(result: &HueAnalysisResult, previous_hue: Option<f64>) -> Self {
        let hue = f64::from(result.color.hue.into_degrees());
        let hue_shift = previous_hue.map(|prev| {
            // Calculate the shortest hue shift
            let diff = hue - prev;
            if diff > 180.0 {
                diff - 360.0
            } else if diff < -180.0 {
                diff + 360.0
            } else {
                diff
            }
        });

        // Convert LCH to RGB for hex representation
        let rgb: Srgb = result.color.into_color();
        let hex = format!(
            "#{:02X}{:02X}{:02X}",
            (rgb.red * 255.0).round() as u8,
            (rgb.green * 255.0).round() as u8,
            (rgb.blue * 255.0).round() as u8
        );

        let lch = format!(
            "L:{:.1} C:{:.1} H:{:.1}",
            result.color.l, result.color.chroma, hue
        );

        Self {
            hue,
            code: format!(
                "{}-{}",
                result.collection.to_uppercase(),
                result.name.as_ref().unwrap_or(&"Unknown".to_string())
            ),
            hex,
            lch,
            name: result.name.clone().unwrap_or_else(|| "Unknown".to_string()),
            hue_shift,
        }
    }
}

/// Format hue analysis results for terminal display
///
/// Creates a formatted table showing hue analysis results.
///
/// # Arguments
/// * `results` - Vector of hue analysis results to format
///
/// # Returns
/// Formatted string for terminal display
pub fn format_hue_analysis_terminal(results: &[HueAnalysisResult]) -> String {
    if results.is_empty() {
        return "No colors found matching the criteria.".to_string();
    }

    let mut output = String::new();

    // Header
    output.push_str("Hue Analysis Results:\n");
    output.push_str("┌──────────┬─────────────────────┬──────────┬─────────────────┬──────────────────────┬─────────────┐\n");
    output.push_str("│ Hue      │ Code                │ HEX      │ LCH             │ Name                 │ Hue Shift   │\n");
    output.push_str("├──────────┼─────────────────────┼──────────┼─────────────────┼──────────────────────┼─────────────┤\n");

    // Convert to display items and format each row
    let mut previous_hue = None;
    for result in results {
        let display_item = HueDisplayItem::from_analysis_result(result, previous_hue);

        let hue_shift_str = match display_item.hue_shift {
            Some(shift) => format!("{:+6.1}°", shift),
            None => "    —   ".to_string(),
        };

        output.push_str(&format!(
            "│ {:>6.1}°  │ {:<19} │ {:<8} │ {:<15} │ {:<20} │ {:<11} │\n",
            display_item.hue,
            &display_item.code[..display_item.code.len().min(19)],
            display_item.hex,
            display_item.lch,
            &display_item.name[..display_item.name.len().min(20)],
            hue_shift_str
        ));

        previous_hue = Some(display_item.hue);
    }

    // Footer
    output.push_str("└──────────┴─────────────────────┴──────────┴─────────────────┴──────────────────────┴─────────────┘\n");
    output.push_str(&format!("\nTotal: {} colors\n", results.len()));

    output
}

/// Serializable hue analysis output for file export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HueAnalysisOutput {
    /// Program metadata
    pub metadata: HueAnalysisMetadata,
    /// Input information
    pub input: HueInputInfo,
    /// Analysis results
    pub results: Vec<HueDisplayItemSerialized>,
}

/// Metadata for hue analysis output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HueAnalysisMetadata {
    /// Program name
    pub program_name: String,
    /// Version
    pub version: String,
    /// Author
    pub author: String,
    /// Description
    pub description: String,
    /// Generation timestamp
    pub generated_at: String,
    /// Analysis mode
    pub analysis_mode: String,
}

/// Input information for hue analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HueInputInfo {
    /// Input color
    pub color: String,
    /// Target hue
    pub target_hue: Option<f64>,
    /// Tolerance
    pub tolerance: f64,
    /// Collection analyzed
    pub collection: String,
    /// Sort criteria
    pub sort_criteria: String,
}

/// Serializable version of `HueDisplayItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HueDisplayItemSerialized {
    /// Hue angle in degrees
    pub hue: f64,
    /// Color code
    pub code: String,
    /// Hexadecimal representation
    pub hex: String,
    /// LCH values
    pub lch: LchValues,
    /// Color name
    pub name: String,
    /// Hue shift from previous
    pub hue_shift: Option<f64>,
}

/// LCH color values for serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LchValues {
    /// Lightness
    pub l: f64,
    /// Chroma
    pub c: f64,
    /// Hue
    pub h: f64,
}

impl HueDisplayItemSerialized {
    /// Convert from HueDisplayItem
    pub fn from_display_item(item: &HueDisplayItem, lch: &Lch) -> Self {
        Self {
            hue: item.hue,
            code: item.code.clone(),
            hex: item.hex.clone(),
            lch: LchValues {
                l: f64::from(lch.l),
                c: f64::from(lch.chroma),
                h: f64::from(lch.hue.into_degrees()),
            },
            name: item.name.clone(),
            hue_shift: item.hue_shift,
        }
    }
}

/// Export hue analysis results to file
///
/// # Arguments
/// * `results` - Hue analysis results to export
/// * `input_color` - Original input color
/// * `options` - Analysis options used
/// * `collection_type` - Collection type analyzed
/// * `sort_criteria` - Sort criteria used
/// * `format` - Output format (YAML or TOML)
/// * `filename` - Output filename
///
/// # Returns
/// Result indicating success or failure
///
/// # Errors
/// Returns `ColorError` if file writing fails
pub fn export_hue_analysis(
    results: &[HueAnalysisResult],
    input_color: &Lch,
    options: &HueAnalysisOptions,
    collection_type: &ColorCollectionType,
    sort_criteria: &SortCriteria,
    format: crate::cli::OutputFormat,
    filename: &str,
) -> Result<()> {
    // Convert results to serializable format
    let mut previous_hue = None;
    let serialized_results: Vec<HueDisplayItemSerialized> = results
        .iter()
        .map(|result| {
            let display_item = HueDisplayItem::from_analysis_result(result, previous_hue);
            previous_hue = Some(display_item.hue);
            HueDisplayItemSerialized::from_display_item(&display_item, &result.color)
        })
        .collect();

    // Create output structure
    let output = HueAnalysisOutput {
        metadata: HueAnalysisMetadata {
            program_name: "color-rs".to_string(),
            version: "0.19.0".to_string(),
            author: "al-siv <https://github.com/al-siv>".to_string(),
            description: "Hue analysis and color harmony patterns".to_string(),
            generated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
            analysis_mode: "hue".to_string(),
        },
        input: HueInputInfo {
            color: format!(
                "L:{:.1} C:{:.1} H:{:.1}",
                input_color.l,
                input_color.chroma,
                f64::from(input_color.hue.into_degrees())
            ),
            target_hue: options.target_hue,
            tolerance: options.tolerance,
            collection: format!("{:?}", collection_type).to_lowercase(),
            sort_criteria: format!("{:?}", sort_criteria).to_lowercase(),
        },
        results: serialized_results,
    };

    // Serialize and write to file
    match format {
        crate::cli::OutputFormat::Yaml => {
            let content = serde_yml::to_string(&output).map_err(|e| {
                ColorError::InvalidArguments(format!("YAML serialization failed: {}", e))
            })?;
            let full_filename = if filename.ends_with(".yaml") || filename.ends_with(".yml") {
                filename.to_string()
            } else {
                format!("{}.yaml", filename)
            };
            std::fs::write(&full_filename, content).map_err(|e| {
                ColorError::InvalidArguments(format!(
                    "Failed to write file {}: {}",
                    full_filename, e
                ))
            })?;
        }
        crate::cli::OutputFormat::Toml => {
            let content = toml::to_string_pretty(&output).map_err(|e| {
                ColorError::InvalidArguments(format!("TOML serialization failed: {}", e))
            })?;
            let full_filename = if filename.ends_with(".toml") {
                filename.to_string()
            } else {
                format!("{}.toml", filename)
            };
            std::fs::write(&full_filename, content).map_err(|e| {
                ColorError::InvalidArguments(format!(
                    "Failed to write file {}: {}",
                    full_filename, e
                ))
            })?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_calculate_hue_distance() {
        assert_eq!(calculate_hue_distance(10.0, 350.0), 20.0);
        assert_eq!(calculate_hue_distance(350.0, 10.0), 20.0);
        assert_eq!(calculate_hue_distance(180.0, 0.0), 180.0);
        assert_eq!(calculate_hue_distance(100.0, 120.0), 20.0);
        assert_eq!(calculate_hue_distance(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_normalize_hue() {
        assert_eq!(normalize_hue(-30.0), 330.0);
        assert_eq!(normalize_hue(400.0), 40.0);
        assert_eq!(normalize_hue(180.0), 180.0);
        assert_eq!(normalize_hue(0.0), 0.0);
        assert_eq!(normalize_hue(360.0), 0.0);
    }

    #[test]
    fn test_meets_criteria() {
        let color = Lch::new(50.0, 40.0, 120.0);
        let options = HueAnalysisOptions {
            target_hue: Some(120.0),
            tolerance: 15.0,
            min_saturation: Some(30.0),
            min_lightness: Some(40.0),
        };

        assert!(meets_criteria(&color, Some(120.0), &options));

        let options_strict = HueAnalysisOptions {
            target_hue: Some(120.0),
            tolerance: 15.0,
            min_saturation: Some(50.0), // Too high
            min_lightness: Some(40.0),
        };

        assert!(!meets_criteria(&color, Some(120.0), &options_strict));
    }

    #[test]
    fn test_sort_criteria_from_str() {
        assert_eq!(
            SortCriteria::from_str("hue-distance").unwrap(),
            SortCriteria::HueDistance
        );
        assert_eq!(
            SortCriteria::from_str("saturation").unwrap(),
            SortCriteria::Saturation
        );
        assert_eq!(
            SortCriteria::from_str("lightness").unwrap(),
            SortCriteria::Lightness
        );
        assert_eq!(SortCriteria::from_str("name").unwrap(), SortCriteria::Name);

        assert!(SortCriteria::from_str("invalid").is_err());
    }

    #[test]
    fn test_color_collection_type_from_str() {
        assert_eq!(
            ColorCollectionType::from_str("css").unwrap(),
            ColorCollectionType::Css
        );
        assert_eq!(
            ColorCollectionType::from_str("ral-classic").unwrap(),
            ColorCollectionType::RalClassic
        );
        assert_eq!(
            ColorCollectionType::from_str("ralc").unwrap(),
            ColorCollectionType::RalClassic
        );
        assert_eq!(
            ColorCollectionType::from_str("ral-design").unwrap(),
            ColorCollectionType::RalDesign
        );
        assert_eq!(
            ColorCollectionType::from_str("rald").unwrap(),
            ColorCollectionType::RalDesign
        );
        assert_eq!(
            ColorCollectionType::from_str("all").unwrap(),
            ColorCollectionType::All
        );

        assert!(ColorCollectionType::from_str("invalid").is_err());
    }

    #[test]
    fn test_color_collection_type_all_collections() {
        let all = ColorCollectionType::all_collections();
        assert_eq!(all.len(), 3);
        assert!(all.contains(&ColorCollectionType::Css));
        assert!(all.contains(&ColorCollectionType::RalClassic));
        assert!(all.contains(&ColorCollectionType::RalDesign));
    }

    #[test]
    fn test_load_css_collection() {
        let result = load_collection_colors(&ColorCollectionType::Css);
        assert!(result.is_ok(), "CSS collection should load successfully");

        let colors = result.unwrap();
        assert!(!colors.is_empty(), "CSS collection should contain colors");
        assert!(
            colors.len() > 10,
            "CSS collection should contain at least 10 colors"
        );

        // Check that colors have proper collection name
        assert_eq!(colors[0].collection, "css");
    }

    #[test]
    fn test_analyze_collection_hues() {
        // Create a test color (red)
        let input_color = Lch::new(50.0, 40.0, 0.0); // Red-ish hue

        let options = HueAnalysisOptions {
            target_hue: Some(0.0), // Red hue
            tolerance: 30.0,       // Wide tolerance
            min_saturation: None,
            min_lightness: None,
        };

        let result = analyze_collection_hues(
            &ColorCollectionType::Css,
            &input_color,
            &options,
            SortCriteria::HueDistance,
            5, // Limit to 5 results
        );

        assert!(result.is_ok(), "Hue analysis should complete successfully");
        let analysis_results = result.unwrap();
        assert!(
            !analysis_results.is_empty(),
            "Should find some matching colors"
        );
        assert!(analysis_results.len() <= 5, "Should respect the limit");

        // Results should be sorted by hue distance (closest first)
        if analysis_results.len() > 1 {
            assert!(
                analysis_results[0].hue_distance <= analysis_results[1].hue_distance,
                "Results should be sorted by hue distance"
            );
        }
    }

    #[test]
    fn test_export_hue_analysis() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_hue_analysis.yaml");

        // Clean up any existing test file
        let _ = std::fs::remove_file(&test_file);

        let results = vec![HueAnalysisResult {
            color: Lch::new(70.0, 30.0, 180.0),
            name: Some("Cyan".to_string()),
            hue_distance: 0.0,
            saturation: 30.0,
            lightness: 70.0,
            collection: "css".to_string(),
        }];

        let input_color = Lch::new(70.0, 30.0, 0.0);
        let options = HueAnalysisOptions {
            target_hue: Some(180.0),
            tolerance: 10.0,
            min_saturation: Some(20.0),
            min_lightness: Some(20.0),
        };
        let collection_type = ColorCollectionType::Css;
        let sort_criteria = SortCriteria::HueDistance;

        // Test YAML export
        let result = export_hue_analysis(
            &results,
            &input_color,
            &options,
            &collection_type,
            &sort_criteria,
            crate::cli::OutputFormat::Yaml,
            test_file.to_str().unwrap(),
        );
        assert!(result.is_ok(), "YAML export should succeed");
        assert!(test_file.exists(), "YAML file should be created");

        // Clean up
        let _ = std::fs::remove_file(&test_file);

        // Test TOML export
        let toml_file = temp_dir.join("test_hue_analysis.toml");
        let _ = std::fs::remove_file(&toml_file);

        let result = export_hue_analysis(
            &results,
            &input_color,
            &options,
            &collection_type,
            &sort_criteria,
            crate::cli::OutputFormat::Toml,
            toml_file.to_str().unwrap(),
        );
        assert!(result.is_ok(), "TOML export should succeed");
        assert!(toml_file.exists(), "TOML file should be created");

        // Clean up
        let _ = std::fs::remove_file(&toml_file);
    }

    #[test]
    fn test_export_hue_analysis_empty_results() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_empty.yaml");

        let results = vec![];
        let input_color = Lch::new(70.0, 30.0, 0.0);
        let options = HueAnalysisOptions {
            target_hue: Some(180.0),
            tolerance: 10.0,
            min_saturation: Some(20.0),
            min_lightness: Some(20.0),
        };
        let collection_type = ColorCollectionType::Css;
        let sort_criteria = SortCriteria::HueDistance;

        let result = export_hue_analysis(
            &results,
            &input_color,
            &options,
            &collection_type,
            &sort_criteria,
            crate::cli::OutputFormat::Yaml,
            test_file.to_str().unwrap(),
        );
        assert!(result.is_ok(), "Export with empty results should succeed");
        assert!(
            test_file.exists(),
            "YAML file should be created even with empty results"
        );

        // Clean up
        let _ = std::fs::remove_file(&test_file);
    }
}
