//! RAL Design System+ Color Collection Implementation
//!
//! Implementation of the unified color collection system for RAL Design System+ colors.

use super::collections::{ColorCollection, ColorEntry, UniversalColor, SearchFilter, ColorMatch};
use super::ral_data::RAL_DESIGN_DATA;

/// RAL Design System+ Colors Collection
pub struct RalDesignCollection {
    colors: Vec<ColorEntry>,
}

impl RalDesignCollection {
    /// Create a new RAL Design System+ color collection
    pub fn new() -> Self {
        let colors = RAL_DESIGN_DATA
            .iter()
            .map(|&(name, code, rgb, hue, lightness, chromaticity)| {
                // Use RGB from the data and convert to LAB for internal storage
                let color = UniversalColor::from_rgb(rgb);
                
                // Extract groups from HLC values
                let hue_group = Self::extract_hue_group(hue);
                let lightness_group = Self::extract_lightness_group(lightness);
                let chroma_group = Self::extract_chroma_group(chromaticity);
                
                // Use hue group as primary group, but store all groups
                let mut entry = ColorEntry::new(color, name.to_string())
                    .with_code(code.to_string())
                    .with_group(hue_group.clone())
                    .with_original_format(format!("hlc({}, {}, {})", hue, lightness, chromaticity));
                
                // Add HLC metadata
                entry.metadata.extra_data.insert("hue".to_string(), hue.to_string());
                entry.metadata.extra_data.insert("lightness".to_string(), lightness.to_string());
                entry.metadata.extra_data.insert("chromaticity".to_string(), chromaticity.to_string());
                entry.metadata.extra_data.insert("hue_group".to_string(), hue_group);
                entry.metadata.extra_data.insert("lightness_group".to_string(), lightness_group);
                entry.metadata.extra_data.insert("chroma_group".to_string(), chroma_group);
                entry.metadata.extra_data.insert("hlc_code".to_string(), code.to_string());
                
                entry
            })
            .collect();

        Self { colors }
    }

    /// Extract hue group from hue value
    fn extract_hue_group(hue: f32) -> String {
        let hue_int = hue as i32;
        match hue_int {
            0..=29 => "Red".to_string(),
            30..=59 => "Orange".to_string(),
            60..=89 => "Yellow".to_string(),
            90..=149 => "Green".to_string(),
            150..=179 => "Cyan".to_string(),
            180..=209 => "Blue".to_string(),
            210..=269 => "Violet".to_string(),
            270..=329 => "Magenta".to_string(),
            330..=360 => "Red".to_string(),
            _ => "Neutral".to_string(),
        }
    }

    /// Extract lightness group from lightness value
    fn extract_lightness_group(lightness: f32) -> String {
        let l_int = lightness as i32;
        match l_int {
            0..=19 => "Very Dark".to_string(),
            20..=39 => "Dark".to_string(),
            40..=59 => "Medium".to_string(),
            60..=79 => "Light".to_string(),
            80..=100 => "Very Light".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    /// Extract chroma group from chromaticity value
    fn extract_chroma_group(chroma: f32) -> String {
        let c_int = chroma as i32;
        match c_int {
            0..=9 => "Neutral".to_string(),
            10..=29 => "Low Saturation".to_string(),
            30..=49 => "Medium Saturation".to_string(),
            50..=79 => "High Saturation".to_string(),
            80..=100 => "Very High Saturation".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    /// Get available hue groups
    pub fn get_hue_groups() -> Vec<String> {
        vec![
            "Red".to_string(),
            "Orange".to_string(),
            "Yellow".to_string(),
            "Green".to_string(),
            "Cyan".to_string(),
            "Blue".to_string(),
            "Violet".to_string(),
            "Magenta".to_string(),
            "Neutral".to_string(),
        ]
    }

    /// Get available lightness groups
    pub fn get_lightness_groups() -> Vec<String> {
        vec![
            "Very Dark".to_string(),
            "Dark".to_string(),
            "Medium".to_string(),
            "Light".to_string(),
            "Very Light".to_string(),
        ]
    }

    /// Get available chroma groups
    pub fn get_chroma_groups() -> Vec<String> {
        vec![
            "Neutral".to_string(),
            "Low Saturation".to_string(),
            "Medium Saturation".to_string(),
            "High Saturation".to_string(),
            "Very High Saturation".to_string(),
        ]
    }

    /// Find colors within specific hue groups
    pub fn find_in_hue_groups(&self, target: &UniversalColor, hue_groups: &[String], max_results: usize) -> Vec<ColorMatch> {
        let filter = SearchFilter {
            groups: Some(hue_groups.to_vec()),
            ..Default::default()
        };
        self.find_closest(target, max_results, Some(&filter))
    }

    /// Find colors within specific lightness range
    pub fn find_in_lightness_range(&self, target: &UniversalColor, min_lightness: f32, max_lightness: f32, max_results: usize) -> Vec<ColorMatch> {
        let filtered_colors: Vec<ColorMatch> = self.colors()
            .iter()
            .filter(|entry| {
                if let Some(lightness_str) = entry.metadata.extra_data.get("lightness") {
                    if let Ok(lightness) = lightness_str.parse::<f32>() {
                        lightness >= min_lightness && lightness <= max_lightness
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
            .map(|entry| {
                let distance = target.distance_to(&entry.color);
                ColorMatch::new(entry.clone(), distance)
            })
            .collect();

        let mut sorted_matches = filtered_colors;
        sorted_matches.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        sorted_matches.truncate(max_results);
        sorted_matches
    }

    /// Find colors within specific chroma range
    pub fn find_in_chroma_range(&self, target: &UniversalColor, min_chroma: f32, max_chroma: f32, max_results: usize) -> Vec<ColorMatch> {
        let filtered_colors: Vec<ColorMatch> = self.colors()
            .iter()
            .filter(|entry| {
                if let Some(chroma_str) = entry.metadata.extra_data.get("chromaticity") {
                    if let Ok(chroma) = chroma_str.parse::<f32>() {
                        chroma >= min_chroma && chroma <= max_chroma
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
            .map(|entry| {
                let distance = target.distance_to(&entry.color);
                ColorMatch::new(entry.clone(), distance)
            })
            .collect();

        let mut sorted_matches = filtered_colors;
        sorted_matches.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        sorted_matches.truncate(max_results);
        sorted_matches
    }
}

impl ColorCollection for RalDesignCollection {
    fn name(&self) -> &'static str {
        "RAL Design System+"
    }

    fn colors(&self) -> &[ColorEntry] {
        &self.colors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ral_design_collection_creation() {
        let collection = RalDesignCollection::new();
        assert!(!collection.colors().is_empty());
        assert_eq!(collection.name(), "RAL Design System+");
    }

    #[test]
    fn test_hue_group_extraction() {
        assert_eq!(RalDesignCollection::extract_hue_group(10.0), "Red");
        assert_eq!(RalDesignCollection::extract_hue_group(45.0), "Orange");
        assert_eq!(RalDesignCollection::extract_hue_group(120.0), "Green");
        assert_eq!(RalDesignCollection::extract_hue_group(240.0), "Violet");
    }

    #[test]
    fn test_lightness_group_extraction() {
        assert_eq!(RalDesignCollection::extract_lightness_group(15.0), "Very Dark");
        assert_eq!(RalDesignCollection::extract_lightness_group(35.0), "Dark");
        assert_eq!(RalDesignCollection::extract_lightness_group(50.0), "Medium");
        assert_eq!(RalDesignCollection::extract_lightness_group(70.0), "Light");
        assert_eq!(RalDesignCollection::extract_lightness_group(90.0), "Very Light");
    }

    #[test]
    fn test_chroma_group_extraction() {
        assert_eq!(RalDesignCollection::extract_chroma_group(5.0), "Neutral");
        assert_eq!(RalDesignCollection::extract_chroma_group(20.0), "Low Saturation");
        assert_eq!(RalDesignCollection::extract_chroma_group(40.0), "Medium Saturation");
        assert_eq!(RalDesignCollection::extract_chroma_group(60.0), "High Saturation");
        assert_eq!(RalDesignCollection::extract_chroma_group(90.0), "Very High Saturation");
    }

    #[test]
    fn test_ral_design_find_by_code() {
        let collection = RalDesignCollection::new();
        let hlc_color = collection.find_by_code("H010L20C10");
        assert!(hlc_color.is_some());
        
        let entry = hlc_color.unwrap();
        assert_eq!(entry.metadata.name, "Wenge Black");
        assert_eq!(entry.metadata.code, Some("H010L20C10".to_string()));
    }

    #[test]
    fn test_ral_design_hue_filtering() {
        let collection = RalDesignCollection::new();
        let target = UniversalColor::from_rgb([255, 0, 0]); // Red
        let matches = collection.find_in_hue_groups(&target, &["Red".to_string()], 3);
        
        // All matches should be from Red hue group
        for m in matches {
            assert!(m.entry.metadata.extra_data.get("hue_group").unwrap() == "Red");
        }
    }
}
