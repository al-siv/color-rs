//! Unified Color Collection Manager
//!
//! High-level interface for managing and searching across multiple color collections.
//! Provides backward compatibility while enabling the new unified architecture.

use super::collections::{
    ColorCollection, ColorCollectionManager, ColorMatch, SearchFilter, UniversalColor,
};
use super::css_collection::CssColorCollection;
use super::ral_classic_collection::RalClassicCollection;
use super::ral_design_collection::RalDesignCollection;
use crate::color_distance_strategies::ColorDistanceStrategy;
use anyhow::Result;

/// Unified manager for all color collections with backward compatibility
pub struct UnifiedColorManager {
    manager: ColorCollectionManager,
    css_collection: CssColorCollection,
    ral_classic_collection: RalClassicCollection,
    ral_design_collection: RalDesignCollection,
}

impl UnifiedColorManager {
    /// Create a new unified color manager with all built-in collections
    pub fn new() -> Result<Self> {
        let mut manager = ColorCollectionManager::new();

        let css_collection = CssColorCollection::new()?;
        let ral_classic_collection = RalClassicCollection::new()?;
        let ral_design_collection = RalDesignCollection::new()?;

        // Add collections to manager
        manager.add_collection(Box::new(CssColorCollection::new()?));
        manager.add_collection(Box::new(RalClassicCollection::new()?));
        manager.add_collection(Box::new(RalDesignCollection::new()?));

        Ok(Self {
            manager,
            css_collection,
            ral_classic_collection,
            ral_design_collection,
        })
    }

    /// Find closest colors across all collections (new unified API)
    pub fn find_closest_across_all(
        &self,
        rgb: [u8; 3],
        max_results_per_collection: usize,
    ) -> Vec<(String, Vec<ColorMatch>)> {
        let target = UniversalColor::from_rgb(rgb);
        self.manager
            .find_closest_across_all(&target, max_results_per_collection, None)
    }

    /// Find closest CSS named colors (backward compatibility)
    pub fn find_closest_css_colors(&self, rgb: [u8; 3], max_results: usize) -> Vec<ColorMatch> {
        let target = UniversalColor::from_rgb(rgb);
        self.css_collection.find_closest(&target, max_results, None)
    }

    /// Find closest RAL Classic colors (backward compatibility)
    pub fn find_closest_ral_classic(&self, rgb: [u8; 3], max_results: usize) -> Vec<ColorMatch> {
        let target = UniversalColor::from_rgb(rgb);
        self.ral_classic_collection
            .find_closest(&target, max_results, None)
    }

    /// Find closest RAL Design System+ colors (backward compatibility)
    pub fn find_closest_ral_design(&self, rgb: [u8; 3], max_results: usize) -> Vec<ColorMatch> {
        let target = UniversalColor::from_rgb(rgb);
        self.ral_design_collection
            .find_closest(&target, max_results, None)
    }

    // /// Find RAL Classic colors within specific groups
    // TODO: Implement after CSV migration is complete
    // pub fn find_ral_classic_in_groups(
    //     &self,
    //     rgb: [u8; 3],
    //     groups: &[String],
    //     max_results: usize,
    // ) -> Vec<ColorMatch> {
    //     let target = UniversalColor::from_rgb(rgb);
    //     self.ral_classic_collection
    //         .find_in_groups(&target, groups, max_results)
    // }

    // /// Find RAL Design colors within specific hue groups
    // TODO: Implement after CSV migration is complete
    // pub fn find_ral_design_in_hue_groups(
    //     &self,
    //     rgb: [u8; 3],
    //     hue_groups: &[String],
    //     max_results: usize,
    // ) -> Vec<ColorMatch> {
    //     let target = UniversalColor::from_rgb(rgb);
    //     self.ral_design_collection
    //         .find_in_hue_groups(&target, hue_groups, max_results)
    // }

    // /// Find RAL Design colors within lightness range
    // TODO: Implement after CSV migration is complete
    // pub fn find_ral_design_in_lightness_range(
    //     &self,
    //     rgb: [u8; 3],
    //     min_lightness: f64,
    //     max_lightness: f64,
    //     max_results: usize,
    // ) -> Vec<ColorMatch> {
    //     let target = UniversalColor::from_rgb(rgb);
    //     self.ral_design_collection.find_in_lightness_range(
    //         &target,
    //         min_lightness,
    //         max_lightness,
    //         max_results,
    //     )
    // }

    // /// Find RAL Design colors within chroma range
    // TODO: Implement after CSV migration is complete
    // pub fn find_ral_design_in_chroma_range(
    //     &self,
    //     rgb: [u8; 3],
    //     min_chroma: f64,
    //     max_chroma: f64,
    //     max_results: usize,
    // ) -> Vec<ColorMatch> {
    //     let target = UniversalColor::from_rgb(rgb);
    //     self.ral_design_collection.find_in_chroma_range(
    //         &target,
    //         min_chroma,
    //         max_chroma,
    //         max_results,
    //     )
    // }

    /// Search by exact name across all collections
    pub fn find_by_name(&self, name: &str) -> Vec<(String, super::collections::ColorEntry)> {
        self.manager.search_by_name(name)
    }

    /// Find color by exact code (RAL codes, etc.)
    pub fn find_by_code(&self, code: &str) -> Option<(String, super::collections::ColorEntry)> {
        if let Some(entry) = self.ral_classic_collection.find_by_code(code) {
            return Some(("RAL Classic".to_string(), entry));
        }
        if let Some(entry) = self.ral_design_collection.find_by_code(code) {
            return Some(("RAL Design System+".to_string(), entry));
        }
        None
    }

    // /// Get available RAL Classic groups
    // TODO: Implement after CSV migration is complete
    // pub fn get_ral_classic_groups(&self) -> Vec<String> {
    //     RalClassicCollection::get_ral_groups()
    // }

    // /// Get available RAL Design hue groups
    // TODO: Implement after CSV migration is complete
    // pub fn get_ral_design_hue_groups(&self) -> Vec<String> {
    //     RalDesignCollection::get_hue_groups()
    // }

    // /// Get available RAL Design lightness groups
    // TODO: Implement after CSV migration is complete
    // pub fn get_ral_design_lightness_groups(&self) -> Vec<String> {
    //     RalDesignCollection::get_lightness_groups()
    // }

    // /// Get available RAL Design chroma groups
    // TODO: Implement after CSV migration is complete
    // pub fn get_ral_design_chroma_groups(&self) -> Vec<String> {
    //     RalDesignCollection::get_chroma_groups()
    // }

    /// Search with advanced filtering
    pub fn search_with_filter(
        &self,
        rgb: [u8; 3],
        filter: &SearchFilter,
        max_results: usize,
    ) -> Vec<(String, Vec<ColorMatch>)> {
        let target = UniversalColor::from_rgb(rgb);
        self.manager
            .find_closest_across_all(&target, max_results, Some(filter))
    }

    // Strategy-aware methods (new API)

    /// Find closest colors across all collections with custom distance strategy
    pub fn find_closest_across_all_with_strategy(
        &self,
        rgb: [u8; 3],
        max_results_per_collection: usize,
        strategy: &dyn ColorDistanceStrategy,
    ) -> Vec<(String, Vec<ColorMatch>)> {
        let target = UniversalColor::from_rgb(rgb);
        self.manager.find_closest_across_all_with_strategy(
            &target,
            max_results_per_collection,
            None,
            strategy,
        )
    }

    /// Find closest CSS named colors with custom distance strategy
    pub fn find_closest_css_colors_with_strategy(
        &self,
        rgb: [u8; 3],
        max_results: usize,
        strategy: &dyn ColorDistanceStrategy,
    ) -> Vec<ColorMatch> {
        let target = UniversalColor::from_rgb(rgb);
        self.css_collection
            .find_closest_with_strategy(&target, max_results, None, strategy)
    }

    /// Find closest RAL Classic colors with custom distance strategy
    pub fn find_closest_ral_classic_with_strategy(
        &self,
        rgb: [u8; 3],
        max_results: usize,
        strategy: &dyn ColorDistanceStrategy,
    ) -> Vec<ColorMatch> {
        let target = UniversalColor::from_rgb(rgb);
        self.ral_classic_collection
            .find_closest_with_strategy(&target, max_results, None, strategy)
    }

    /// Find closest RAL Design System+ colors with custom distance strategy
    pub fn find_closest_ral_design_with_strategy(
        &self,
        rgb: [u8; 3],
        max_results: usize,
        strategy: &dyn ColorDistanceStrategy,
    ) -> Vec<ColorMatch> {
        let target = UniversalColor::from_rgb(rgb);
        self.ral_design_collection
            .find_closest_with_strategy(&target, max_results, None, strategy)
    }

    // Find RAL colors by name pattern (for backward compatibility)
    // TODO: Implement after CSV migration is complete
    // pub fn find_ral_by_name_pattern(
    //     &self,
    //     pattern: &str,
    // ) -> Vec<(String, super::collections::ColorEntry)> {
    //     let mut results = Vec::new();

    //     // Search RAL Classic
    //     let classic_matches = self.ral_classic_collection.find_by_name_pattern(pattern);
    //     for entry in classic_matches {
    //         results.push(("RAL Classic".to_string(), entry));
    //     }

    //     // Search RAL Design System+
    //     let design_matches = self.ral_design_collection.find_by_name_pattern(pattern);
    //     for entry in design_matches {
    //         results.push(("RAL Design System+".to_string(), entry));
    //     }

    //     results
    // }
}

impl Default for UnifiedColorManager {
    fn default() -> Self {
        Self::new().expect("Failed to create UnifiedColorManager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_manager_creation() {
        let manager = UnifiedColorManager::new().expect("Failed to create UnifiedColorManager");
        let collections = manager.manager.collection_names();
        assert_eq!(collections.len(), 3);
        assert!(collections.contains(&"CSS Named Colors"));
        assert!(collections.contains(&"RAL Classic"));
        assert!(collections.contains(&"RAL Design System+"));
    }

    #[test]
    fn test_find_closest_across_all() {
        let manager = UnifiedColorManager::new().expect("Failed to create UnifiedColorManager");
        let results = manager.find_closest_across_all([255, 0, 0], 2);

        assert_eq!(results.len(), 3); // CSS, RAL Classic, RAL Design

        for (collection_name, matches) in results {
            assert!(matches.len() <= 2);
            assert!(
                ["CSS Named Colors", "RAL Classic", "RAL Design System+"]
                    .contains(&collection_name.as_str())
            );
        }
    }

    #[test]
    fn test_find_by_code() {
        let manager = UnifiedColorManager::new().expect("Failed to create UnifiedColorManager");

        // Test RAL Classic code - use a code that should exist
        if let Some(first_ral) = manager.ral_classic_collection.colors().first() {
            if let Some(code) = &first_ral.metadata.code {
                let found = manager.find_by_code(code);
                assert!(found.is_some());
                let (collection, _entry) = found.unwrap();
                assert_eq!(collection, "RAL Classic");
            }
        }

        // Test RAL Design code - use a code that should exist
        if let Some(first_design) = manager.ral_design_collection.colors().first() {
            if let Some(code) = &first_design.metadata.code {
                let found = manager.find_by_code(code);
                assert!(found.is_some());
                let (collection, _entry) = found.unwrap();
                assert_eq!(collection, "RAL Design System+");
            }
        }
    }

    #[test]
    fn test_group_filtering() {
        let manager = UnifiedColorManager::new().expect("Failed to create UnifiedColorManager");

        // Test RAL Classic groups
        let groups = manager.ral_classic_collection.groups();
        assert!(!groups.is_empty());

        // Test RAL Design groups
        let design_groups = manager.ral_design_collection.groups();
        assert!(!design_groups.is_empty());
    }
}
