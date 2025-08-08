//! CSS Color Collection Implementation
//!
//! Implementation of the unified color collection system for CSS named colors.

use super::collections::{ColorCollection, ColorEntry, UniversalColor};
use super::csv_loader::CsvLoader;
use anyhow::Result;

/// CSS Named Colors Collection
pub struct CssColorCollection {
    colors: Vec<ColorEntry>,
}

impl CssColorCollection {
    /// Create a new CSS color collection
    pub fn new() -> Result<Self> {
        let csv_colors = CsvLoader::load_colors_from_csv("color-table/css-colors.csv")?;

        let colors = csv_colors
            .iter()
            .map(|entry| {
                let rgb = CsvLoader::hex_to_rgb(&entry.hex).unwrap_or([0, 0, 0]); // Fallback to black on error

                let color = UniversalColor::from_rgb(rgb);
                ColorEntry::new(color, entry.name.clone())
                    .with_code(entry.code.clone())
                    .with_group("CSS".to_string())
                    .with_original_format(entry.hex.clone())
            })
            .collect();

        Ok(Self { colors })
    }

    /// Create an empty CSS collection (safe fallback)
    #[must_use]
    pub fn empty() -> Self {
        Self { colors: Vec::new() }
    }
}

impl ColorCollection for CssColorCollection {
    fn name(&self) -> &'static str {
        "CSS Named Colors"
    }

    fn colors(&self) -> &[ColorEntry] {
        &self.colors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_collection_creation() {
        let collection = CssColorCollection::new().expect("Failed to create CSS collection");
        assert!(!collection.colors().is_empty());
        assert_eq!(collection.name(), "CSS Named Colors");
    }

    #[test]
    fn test_css_find_by_name() {
        let collection = CssColorCollection::new().expect("Failed to create CSS collection");
        let red = collection.find_by_name("red");
        assert!(red.is_some());

        let red_entry = red.unwrap();
        assert_eq!(red_entry.metadata.name, "Red"); // Name is capitalized in CSV
        assert_eq!(red_entry.color.rgb, [255, 0, 0]);
    }

    #[test]
    fn test_css_closest_match() {
        let collection = CssColorCollection::new().expect("Failed to create CSS collection");
        let target = UniversalColor::from_rgb([254, 1, 1]); // Almost red
        let matches = collection.find_closest(&target, 1, None);

        assert!(!matches.is_empty());
        assert_eq!(matches[0].entry.metadata.name, "Red"); // Name is capitalized in CSV
    }
}
