//! RAL Classic Color Collection Implementation
//!
//! Implementation of the unified color collection system for RAL Classic colors.

use super::collections::{ColorCollection, ColorEntry, UniversalColor, SearchFilter, ColorMatch};
use super::ral_data::RAL_CLASSIC_DATA;

/// RAL Classic Colors Collection
pub struct RalClassicCollection {
    colors: Vec<ColorEntry>,
}

impl RalClassicCollection {
    /// Create a new RAL Classic color collection
    pub fn new() -> Self {
        let colors = RAL_CLASSIC_DATA
            .iter()
            .map(|&(code, name, hex, lab_l, lab_a, lab_b, cmyk_c, cmyk_m, cmyk_y, cmyk_k, lrv)| {
                // Use the provided LAB values from the data
                let color = UniversalColor::from_lab([lab_l, lab_a, lab_b]);
                
                // Extract RAL group from code (e.g., "RAL 1000" -> "RAL 1000")
                let group = Self::extract_ral_group(code);
                
                let mut entry = ColorEntry::new(color, name.to_string())
                    .with_code(code.to_string())
                    .with_group(group)
                    .with_original_format(hex.to_string());
                
                // Add extra metadata
                entry.metadata.extra_data.insert("cmyk".to_string(), 
                    format!("cmyk({:.1}, {:.1}, {:.1}, {:.1})", cmyk_c, cmyk_m, cmyk_y, cmyk_k));
                entry.metadata.extra_data.insert("lrv".to_string(), lrv.to_string());
                entry.metadata.extra_data.insert("hex".to_string(), hex.to_string());
                
                entry
            })
            .collect();

        Self { colors }
    }

    /// Extract RAL group from code (e.g., "RAL 1000" -> "1000")
    fn extract_ral_group(code: &str) -> String {
        if let Some(space_pos) = code.find(' ') {
            let number_part = &code[space_pos + 1..];
            if number_part.len() >= 4 {
                // Group by first digit (1000-1999, 2000-2999, etc.)
                let group_number = &number_part[..1];
                format!("RAL {}000", group_number)
            } else {
                "RAL Other".to_string()
            }
        } else {
            "RAL Other".to_string()
        }
    }

    /// Get available RAL groups (1000, 2000, 3000, etc.)
    pub fn get_ral_groups() -> Vec<String> {
        vec![
            "RAL 1000".to_string(), // Yellow and Beige
            "RAL 2000".to_string(), // Orange
            "RAL 3000".to_string(), // Red
            "RAL 4000".to_string(), // Violet
            "RAL 5000".to_string(), // Blue
            "RAL 6000".to_string(), // Green
            "RAL 7000".to_string(), // Grey
            "RAL 8000".to_string(), // Brown
            "RAL 9000".to_string(), // White and Black
        ]
    }

    /// Find colors within specific RAL groups
    pub fn find_in_groups(&self, target: &UniversalColor, groups: &[String], max_results: usize) -> Vec<ColorMatch> {
        let filter = SearchFilter {
            groups: Some(groups.to_vec()),
            ..Default::default()
        };
        self.find_closest(target, max_results, Some(&filter))
    }
}

impl ColorCollection for RalClassicCollection {
    fn name(&self) -> &'static str {
        "RAL Classic"
    }

    fn colors(&self) -> &[ColorEntry] {
        &self.colors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ral_classic_collection_creation() {
        let collection = RalClassicCollection::new();
        assert!(!collection.colors().is_empty());
        assert_eq!(collection.name(), "RAL Classic");
    }

    #[test]
    fn test_ral_group_extraction() {
        assert_eq!(RalClassicCollection::extract_ral_group("RAL 1000"), "RAL 1000");
        assert_eq!(RalClassicCollection::extract_ral_group("RAL 3020"), "RAL 3000");
        assert_eq!(RalClassicCollection::extract_ral_group("RAL 7040"), "RAL 7000");
    }

    #[test]
    fn test_ral_find_by_code() {
        let collection = RalClassicCollection::new();
        let ral1000 = collection.find_by_code("RAL 1000");
        assert!(ral1000.is_some());
        
        let entry = ral1000.unwrap();
        assert_eq!(entry.metadata.name, "Green beige");
        assert_eq!(entry.metadata.code, Some("RAL 1000".to_string()));
    }

    #[test]
    fn test_ral_group_filtering() {
        let collection = RalClassicCollection::new();
        let target = UniversalColor::from_rgb([255, 0, 0]); // Red
        let matches = collection.find_in_groups(&target, &["RAL 3000".to_string()], 3);
        
        // All matches should be from RAL 3000 group (reds)
        for m in matches {
            assert!(m.entry.metadata.group.as_ref().unwrap().starts_with("RAL 3"));
        }
    }
}
