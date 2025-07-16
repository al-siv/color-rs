//! CSS Color Collection Implementation
//!
//! Implementation of the unified color collection system for CSS named colors.

use super::collections::{ColorCollection, ColorEntry, UniversalColor};
use super::color_names::CSS_COLOR_DATA;

/// CSS Named Colors Collection
pub struct CssColorCollection {
    colors: Vec<ColorEntry>,
}

impl CssColorCollection {
    /// Create a new CSS color collection
    pub fn new() -> Self {
        let colors = CSS_COLOR_DATA
            .iter()
            .map(|&(name, rgb)| {
                let color = UniversalColor::from_rgb(rgb);
                ColorEntry::new(color, name.to_string())
                    .with_group("CSS".to_string())
                    .with_original_format(format!("rgb({}, {}, {})", rgb[0], rgb[1], rgb[2]))
            })
            .collect();

        Self { colors }
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
        let collection = CssColorCollection::new();
        assert!(!collection.colors().is_empty());
        assert_eq!(collection.name(), "CSS Named Colors");
    }

    #[test]
    fn test_css_find_by_name() {
        let collection = CssColorCollection::new();
        let red = collection.find_by_name("red");
        assert!(red.is_some());

        let red_entry = red.unwrap();
        assert_eq!(red_entry.metadata.name, "red");
        assert_eq!(red_entry.color.rgb, [255, 0, 0]);
    }

    #[test]
    fn test_css_closest_match() {
        let collection = CssColorCollection::new();
        let target = UniversalColor::from_rgb([254, 1, 1]); // Almost red
        let matches = collection.find_closest(&target, 1, None);

        assert!(!matches.is_empty());
        assert_eq!(matches[0].entry.metadata.name, "red");
    }
}
