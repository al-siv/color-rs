//! RAL Classic Color Collection Implementation
//!
//! Implementation of the unified color collection system for RAL Classic colors.

use super::collections::{ColorCollection, ColorEntry, ColorMatch, SearchFilter, UniversalColor};
use super::csv_loader::CsvLoader;
use anyhow::Result;

/// RAL Classic Colors Collection
pub struct RalClassicCollection {
    colors: Vec<ColorEntry>,
}

impl RalClassicCollection {
    /// Create a new RAL Classic color collection
    pub fn new() -> Result<Self> {
        let csv_colors = CsvLoader::load_colors_from_csv("color-table/ral-classic.csv")?;

        let colors = csv_colors
            .iter()
            .map(|entry| {
                let rgb = CsvLoader::hex_to_rgb(&entry.hex).unwrap_or([0, 0, 0]); // Fallback to black on error

                let color = UniversalColor::from_rgb(rgb);

                // Extract RAL group from code (e.g., "RAL 1000" -> "RAL 1000")
                let group = Self::extract_ral_group(&entry.code);

                ColorEntry::new(color, entry.name.clone())
                    .with_code(entry.code.clone())
                    .with_group(group)
                    .with_original_format(entry.hex.clone())
            })
            .collect();

        Ok(Self { colors })
    }

    /// Extract RAL group from code (e.g., "RAL 1000" -> "1000")
    fn extract_ral_group(code: &str) -> String {
        if let Some(space_pos) = code.find(' ') {
            let number_part = &code[space_pos + 1..];
            if number_part.len() >= 4 {
                // Group by first digit (1000-1999, 2000-2999, etc.)
                let group_number = &number_part[..1];
                format!("RAL {group_number}000")
            } else {
                code.to_string()
            }
        } else {
            code.to_string()
        }
    }
}

impl ColorCollection for RalClassicCollection {
    fn name(&self) -> &'static str {
        "RAL Classic"
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

    fn groups(&self) -> Vec<String> {
        let mut groups: Vec<String> = self
            .colors
            .iter()
            .filter_map(|entry| entry.metadata.group.clone())
            .collect();
        groups.sort();
        groups.dedup();
        groups
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ral_classic_collection_creation() {
        let collection =
            RalClassicCollection::new().expect("Failed to create RAL Classic collection");
        assert!(!collection.colors().is_empty());
        assert_eq!(collection.name(), "RAL Classic");
    }

    #[test]
    fn test_ral_find_by_code() {
        let collection =
            RalClassicCollection::new().expect("Failed to create RAL Classic collection");
        let color = collection.find_by_code("RAL 1000");
        assert!(color.is_some());

        if let Some(entry) = color {
            assert_eq!(entry.metadata.code.as_ref().unwrap(), "RAL 1000");
        }
    }

    #[test]
    fn test_ral_group_extraction() {
        assert_eq!(
            RalClassicCollection::extract_ral_group("RAL 1000"),
            "RAL 1000"
        );
        assert_eq!(
            RalClassicCollection::extract_ral_group("RAL 2000"),
            "RAL 2000"
        );
        assert_eq!(
            RalClassicCollection::extract_ral_group("RAL 9999"),
            "RAL 9000"
        );
    }

    #[test]
    fn test_ral_group_filtering() {
        let collection =
            RalClassicCollection::new().expect("Failed to create RAL Classic collection");
        let target = UniversalColor::from_rgb([255, 0, 0]); // Red

        let filter = SearchFilter {
            groups: Some(vec!["RAL 3000".to_string()]),
            ..Default::default()
        };

        let matches = collection.find_closest(&target, 5, Some(&filter));

        for color_match in matches {
            if let Some(group) = &color_match.entry.metadata.group {
                assert_eq!(group, "RAL 3000");
            }
        }
    }
}
