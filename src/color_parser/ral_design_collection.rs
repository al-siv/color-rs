//! RAL Design System+ Color Collection Implementation
//!
//! Implementation of the unified color collection system for RAL Design System+ colors.

use super::collections::{ColorCollection, ColorEntry, ColorMatch, SearchFilter, UniversalColor};
use super::csv_loader::CsvLoader;
use anyhow::Result;

/// RAL Design System+ Colors Collection
pub struct RalDesignCollection {
    colors: Vec<ColorEntry>,
}

impl RalDesignCollection {
    /// Create a new RAL Design System+ color collection
    pub fn new() -> Result<Self> {
        let csv_colors = CsvLoader::load_colors_from_csv("color-table/ral-design.csv")?;

        let colors = csv_colors
            .iter()
            .map(|entry| {
                let rgb = CsvLoader::hex_to_rgb(&entry.hex).unwrap_or([0, 0, 0]); // Fallback to black on error

                let color = UniversalColor::from_rgb(rgb);

                // Extract group information from RAL Design code
                // RAL Design codes are like "RAL 000 15 00" - we'll use the main part as group
                let group = Self::extract_design_group(&entry.code);

                ColorEntry::new(color, entry.name.clone())
                    .with_code(entry.code.clone())
                    .with_group(group)
                    .with_original_format(entry.hex.clone())
            })
            .collect();

        Ok(Self { colors })
    }

    /// Extract design group from code (e.g., "RAL 000 15 00" -> "RAL 000")
    fn extract_design_group(code: &str) -> String {
        let parts: Vec<&str> = code.split_whitespace().collect();
        if parts.len() >= 2 {
            format!("{} {}", parts[0], parts[1])
        } else {
            code.to_string()
        }
    }

    /// Extract hue group from hue value (for compatibility)
    #[must_use]
    pub fn extract_hue_group(hue: f64) -> String {
        let hue_range = ((hue / 10.0).floor() * 10.0) as u32;
        format!("H{hue_range:03}")
    }

    /// Extract lightness group from lightness value (for compatibility)
    #[must_use]
    pub fn extract_lightness_group(lightness: f64) -> String {
        let lightness_range = ((lightness / 10.0).floor() * 10.0) as u32;
        format!("L{lightness_range:02}")
    }

    /// Extract chroma group from chromaticity value (for compatibility)
    #[must_use]
    pub fn extract_chroma_group(chromaticity: f64) -> String {
        let chroma_range = ((chromaticity / 10.0).floor() * 10.0) as u32;
        format!("C{chroma_range:02}")
    }
}

impl ColorCollection for RalDesignCollection {
    fn name(&self) -> &'static str {
        "RAL Design System+"
    }

    fn colors(&self) -> &[ColorEntry] {
        &self.colors
    }

    fn find_by_code(&self, code: &str) -> Option<ColorEntry> {
        self.colors
            .iter()
            .find(|entry| entry.metadata.code.as_ref() == Some(&code.to_string()))
            .cloned()
    }

    fn find_closest(
        &self,
        target: &UniversalColor,
        limit: usize,
        filter: Option<&SearchFilter>,
    ) -> Vec<ColorMatch> {
        let mut distances: Vec<_> = self
            .colors
            .iter()
            .filter(|entry| {
                if let Some(filter) = filter {
                    if let Some(groups_filter) = &filter.groups {
                        if let Some(entry_group) = &entry.metadata.group {
                            if !groups_filter.contains(entry_group) {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                }
                true
            })
            .map(|entry| {
                let distance = target.distance_to(&entry.color);
                ColorMatch::new(entry.clone(), distance)
            })
            .collect();

        distances.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        distances.truncate(limit);
        distances
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ral_design_collection_creation() {
        let collection =
            RalDesignCollection::new().expect("Failed to create RAL Design collection");
        assert!(!collection.colors().is_empty());
        assert_eq!(collection.name(), "RAL Design System+");
    }

    #[test]
    fn test_ral_design_find_by_code() {
        let collection =
            RalDesignCollection::new().expect("Failed to create RAL Design collection");
        // Use a code that should exist in the CSV
        if let Some(first_color) = collection.colors().first() {
            if let Some(code) = &first_color.metadata.code {
                let found = collection.find_by_code(code);
                assert!(found.is_some());
            }
        }
    }

    #[test]
    fn test_design_group_extraction() {
        assert_eq!(
            RalDesignCollection::extract_design_group("RAL 000 15 00"),
            "RAL 000"
        );
        assert_eq!(
            RalDesignCollection::extract_design_group("RAL 010 20 30"),
            "RAL 010"
        );
        assert_eq!(RalDesignCollection::extract_design_group("RAL"), "RAL");
    }

    #[test]
    fn test_hue_group_extraction() {
        assert_eq!(RalDesignCollection::extract_hue_group(0.0), "H000");
        assert_eq!(RalDesignCollection::extract_hue_group(15.0), "H010");
        assert_eq!(RalDesignCollection::extract_hue_group(25.0), "H020");
        assert_eq!(RalDesignCollection::extract_hue_group(355.0), "H350");
    }

    #[test]
    fn test_lightness_group_extraction() {
        assert_eq!(RalDesignCollection::extract_lightness_group(15.0), "L10");
        assert_eq!(RalDesignCollection::extract_lightness_group(25.0), "L20");
        assert_eq!(RalDesignCollection::extract_lightness_group(55.0), "L50");
        assert_eq!(RalDesignCollection::extract_lightness_group(85.0), "L80");
    }

    #[test]
    fn test_chroma_group_extraction() {
        assert_eq!(RalDesignCollection::extract_chroma_group(0.0), "C00");
        assert_eq!(RalDesignCollection::extract_chroma_group(15.0), "C10");
        assert_eq!(RalDesignCollection::extract_chroma_group(35.0), "C30");
        assert_eq!(RalDesignCollection::extract_chroma_group(65.0), "C60");
    }
}
