//! Unified Color Collection System
//!
//! A trait-based system for managing different color collections with unified search capabilities.
//! Supports different native color spaces while using LAB for perceptually accurate comparisons.

use crate::color_distance_strategies::{ColorDistanceStrategy, DeltaE2000Strategy};
use crate::color_utils::LegacyColorUtils as ColorUtils;
use palette::Lab;
use std::collections::HashMap;

/// Universal color representation using LAB color space for accurate comparisons
#[derive(Debug, Clone, PartialEq)]
pub struct UniversalColor {
    /// LAB color values for perceptually accurate comparisons
    pub lab: [f32; 3], // [L, a, b]
    /// RGB representation (derived from LAB or original if RGB-native)
    pub rgb: [u8; 3],
    /// Optional WCAG relative luminance (calculated on demand)
    pub luminance: Option<f64>,
}

impl UniversalColor {
    /// Create from RGB values
    #[must_use]
    pub fn from_rgb(rgb: [u8; 3]) -> Self {
        let lab = ColorUtils::rgb_array_to_lab(rgb);
        Self {
            lab,
            rgb,
            luminance: None,
        }
    }

    /// Create from LAB values
    #[must_use]
    pub fn from_lab(lab: [f32; 3]) -> Self {
        let rgb = ColorUtils::lab_array_to_rgb(lab);
        Self {
            lab,
            rgb,
            luminance: None,
        }
    }

    /// Create from HLC values (for RAL Design System+)
    #[must_use]
    pub fn from_hlc(hue: f64, lightness: f64, chroma: f64) -> Self {
        // Convert HLC to LAB (HLC is similar to LCH but with different scaling)
        let h_rad = hue.to_radians();
        let a = chroma * h_rad.cos();
        let b = chroma * h_rad.sin();
        let lab: [f32; 3] = [lightness as f32, a as f32, b as f32];
        Self::from_lab(lab)
    }

    /// Get WCAG relative luminance (cached)
    pub fn luminance(&mut self) -> f64 {
        if self.luminance.is_none() {
            let srgb = ColorUtils::rgb_to_srgb((self.rgb[0], self.rgb[1], self.rgb[2]));
            self.luminance = Some(ColorUtils::wcag_relative_luminance(srgb));
        }
        self.luminance.unwrap()
    }

    /// Calculate LAB distance to another color using the specified strategy
    #[must_use]
    pub fn distance_to(&self, other: &Self) -> f64 {
        // Use the default strategy (Delta E 2000) for backward compatibility
        self.distance_to_with_strategy(other, &DeltaE2000Strategy)
    }

    /// Calculate LAB distance to another color using a specific strategy
    pub fn distance_to_with_strategy(
        &self,
        other: &Self,
        strategy: &dyn ColorDistanceStrategy,
    ) -> f64 {
        let lab1 = Lab::new(self.lab[0], self.lab[1], self.lab[2]);
        let lab2 = Lab::new(other.lab[0], other.lab[1], other.lab[2]);
        strategy.calculate_distance(lab1, lab2)
    }
}

/// Metadata for a color entry in a collection
#[derive(Debug, Clone)]
pub struct ColorMetadata {
    /// Human-readable name
    pub name: String,
    /// Optional code (e.g., "RAL 1000", "H010L20C10")
    pub code: Option<String>,
    /// Collection-specific group identifier
    pub group: Option<String>,
    /// Original color representation (hex, CMYK, etc.)
    pub original_format: Option<String>,
    /// Additional metadata (LRV, etc.)
    pub extra_data: HashMap<String, String>,
}

/// A color entry in a collection
#[derive(Debug, Clone)]
pub struct ColorEntry {
    /// Universal color representation
    pub color: UniversalColor,
    /// Metadata about this color
    pub metadata: ColorMetadata,
}

impl ColorEntry {
    /// Create a new color entry
    #[must_use]
    pub fn new(color: UniversalColor, name: String) -> Self {
        Self {
            color,
            metadata: ColorMetadata {
                name,
                code: None,
                group: None,
                original_format: None,
                extra_data: HashMap::new(),
            },
        }
    }

    /// Set code for this color entry
    #[must_use]
    pub fn with_code(mut self, code: String) -> Self {
        self.metadata.code = Some(code);
        self
    }

    /// Set group for this color entry
    #[must_use]
    pub fn with_group(mut self, group: String) -> Self {
        self.metadata.group = Some(group);
        self
    }

    /// Set original format
    #[must_use]
    pub fn with_original_format(mut self, format: String) -> Self {
        self.metadata.original_format = Some(format);
        self
    }

    /// Add extra metadata
    #[must_use]
    pub fn with_extra_data(mut self, key: String, value: String) -> Self {
        self.metadata.extra_data.insert(key, value);
        self
    }
}

/// Filter criteria for searching color collections
#[derive(Debug, Clone, Default)]
pub struct SearchFilter {
    /// Limit to specific groups
    pub groups: Option<Vec<String>>,
    /// Luminance range filter [min, max]
    pub luminance_range: Option<[f64; 2]>,
    /// Maximum color distance for "close enough" matches
    pub max_distance: Option<f64>,
    /// Name pattern matching
    pub name_pattern: Option<String>,
}

/// Result of a color search
#[derive(Debug, Clone)]
pub struct ColorMatch {
    /// The matched color entry
    pub entry: ColorEntry,
    /// Distance from the search target
    pub distance: f64,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
}

impl ColorMatch {
    /// Create a new color match
    #[must_use]
    pub fn new(entry: ColorEntry, distance: f64) -> Self {
        // Calculate confidence based on distance (closer = higher confidence)
        let confidence = (50.0 - distance.min(50.0)) / 50.0;
        Self {
            entry,
            distance,
            confidence: confidence.max(0.0),
        }
    }
}

/// Trait for color collections that provides unified search capabilities
pub trait ColorCollection: Send + Sync {
    /// Get the name of this collection
    fn name(&self) -> &'static str;

    /// Get all colors in this collection
    fn colors(&self) -> &[ColorEntry];

    /// Get available groups in this collection
    fn groups(&self) -> Vec<String> {
        self.colors()
            .iter()
            .filter_map(|entry| entry.metadata.group.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    }

    /// Find the closest color matches to a target color
    fn find_closest(
        &self,
        target: &UniversalColor,
        max_results: usize,
        filter: Option<&SearchFilter>,
    ) -> Vec<ColorMatch> {
        // Use the default strategy for backward compatibility
        self.find_closest_with_strategy(target, max_results, filter, &DeltaE2000Strategy)
    }

    /// Find the closest color matches to a target color using a specific strategy
    fn find_closest_with_strategy(
        &self,
        target: &UniversalColor,
        max_results: usize,
        filter: Option<&SearchFilter>,
        strategy: &dyn ColorDistanceStrategy,
    ) -> Vec<ColorMatch> {
        let mut matches: Vec<ColorMatch> = self
            .colors()
            .iter()
            .filter(|entry| self.matches_filter(entry, filter))
            .map(|entry| {
                let distance = target.distance_to_with_strategy(&entry.color, strategy);
                ColorMatch::new(entry.clone(), distance)
            })
            .collect();

        // Sort by distance and limit results
        matches.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        matches.truncate(max_results);
        matches
    }

    /// Find exact color match by name
    fn find_by_name(&self, name: &str) -> Option<ColorEntry> {
        self.colors()
            .iter()
            .find(|entry| entry.metadata.name.eq_ignore_ascii_case(name))
            .cloned()
    }

    /// Find color by exact code
    fn find_by_code(&self, code: &str) -> Option<ColorEntry> {
        self.colors()
            .iter()
            .find(|entry| {
                entry
                    .metadata
                    .code
                    .as_ref()
                    .is_some_and(|c| c.eq_ignore_ascii_case(code))
            })
            .cloned()
    }

    /// Find colors with similar names
    fn find_by_name_pattern(&self, pattern: &str) -> Vec<ColorEntry> {
        let pattern_lower = pattern.to_lowercase();
        self.colors()
            .iter()
            .filter(|entry| entry.metadata.name.to_lowercase().contains(&pattern_lower))
            .cloned()
            .collect()
    }

    /// Find colors by luminance range
    fn find_by_luminance_range(&self, min_luminance: f64, max_luminance: f64) -> Vec<ColorEntry> {
        self.colors()
            .iter()
            .filter(|entry| {
                let mut color = entry.color.clone();
                let luminance = color.luminance();
                luminance >= min_luminance && luminance <= max_luminance
            })
            .cloned()
            .collect()
    }

    /// Check if an entry matches the given filter
    fn matches_filter(&self, entry: &ColorEntry, filter: Option<&SearchFilter>) -> bool {
        let Some(filter) = filter else {
            return true;
        };

        // Check group filter
        if let Some(ref allowed_groups) = filter.groups {
            if let Some(ref entry_group) = entry.metadata.group {
                if !allowed_groups.contains(entry_group) {
                    return false;
                }
            } else {
                return false; // Entry has no group but filter requires one
            }
        }

        // Check luminance range
        if let Some(range) = filter.luminance_range {
            let mut color = entry.color.clone();
            let luminance = color.luminance();
            if luminance < range[0] || luminance > range[1] {
                return false;
            }
        }

        // Check name pattern
        if let Some(ref pattern) = filter.name_pattern {
            if !entry
                .metadata
                .name
                .to_lowercase()
                .contains(&pattern.to_lowercase())
            {
                return false;
            }
        }

        true
    }
}

/// Manager for multiple color collections
pub struct ColorCollectionManager {
    collections: Vec<Box<dyn ColorCollection>>,
}

impl Default for ColorCollectionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorCollectionManager {
    /// Create a new collection manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            collections: Vec::new(),
        }
    }

    /// Add a collection to the manager
    pub fn add_collection(&mut self, collection: Box<dyn ColorCollection>) {
        self.collections.push(collection);
    }

    /// Get all collection names
    #[must_use]
    pub fn collection_names(&self) -> Vec<&'static str> {
        self.collections.iter().map(|c| c.name()).collect()
    }

    /// Find closest colors across all collections
    #[must_use]
    pub fn find_closest_across_all(
        &self,
        target: &UniversalColor,
        max_results_per_collection: usize,
        filter: Option<&SearchFilter>,
    ) -> Vec<(String, Vec<ColorMatch>)> {
        self.collections
            .iter()
            .map(|collection| {
                let matches = collection.find_closest(target, max_results_per_collection, filter);
                (collection.name().to_string(), matches)
            })
            .collect()
    }

    /// Find closest colors across all collections with custom distance strategy
    pub fn find_closest_across_all_with_strategy(
        &self,
        target: &UniversalColor,
        max_results_per_collection: usize,
        filter: Option<&SearchFilter>,
        strategy: &dyn ColorDistanceStrategy,
    ) -> Vec<(String, Vec<ColorMatch>)> {
        self.collections
            .iter()
            .map(|collection| {
                let matches = collection.find_closest_with_strategy(
                    target,
                    max_results_per_collection,
                    filter,
                    strategy,
                );
                (collection.name().to_string(), matches)
            })
            .collect()
    }

    /// Find closest colors from a specific collection
    #[must_use]
    pub fn find_closest_from_collection(
        &self,
        collection_name: &str,
        target: &UniversalColor,
        max_results: usize,
        filter: Option<&SearchFilter>,
    ) -> Option<Vec<ColorMatch>> {
        self.collections
            .iter()
            .find(|c| c.name() == collection_name)
            .map(|collection| collection.find_closest(target, max_results, filter))
    }

    /// Search by name across all collections
    #[must_use]
    pub fn search_by_name(&self, name: &str) -> Vec<(String, ColorEntry)> {
        self.collections
            .iter()
            .filter_map(|collection| {
                collection
                    .find_by_name(name)
                    .map(|entry| (collection.name().to_string(), entry))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_color_creation() {
        let rgb = [255, 0, 0];
        let color = UniversalColor::from_rgb(rgb);
        assert_eq!(color.rgb, rgb);
        assert!(color.lab[0] > 0.0); // Should have positive lightness
    }

    #[test]
    fn test_color_distance() {
        let red = UniversalColor::from_rgb([255, 0, 0]);
        let blue = UniversalColor::from_rgb([0, 0, 255]);
        let distance = red.distance_to(&blue);
        assert!(distance > 0.0);
    }

    #[test]
    fn test_color_entry_creation() {
        let color = UniversalColor::from_rgb([255, 0, 0]);
        let entry = ColorEntry::new(color, "Red".to_string())
            .with_code("R001".to_string())
            .with_group("Primary".to_string());

        assert_eq!(entry.metadata.name, "Red");
        assert_eq!(entry.metadata.code, Some("R001".to_string()));
        assert_eq!(entry.metadata.group, Some("Primary".to_string()));
    }
}
