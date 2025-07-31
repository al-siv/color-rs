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
use crate::color_distance_strategies::DistanceAlgorithm;
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
    #[must_use]
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
    #[must_use]
    pub fn find_closest_css_colors(&self, rgb: [u8; 3], max_results: usize) -> Vec<ColorMatch> {
        let target = UniversalColor::from_rgb(rgb);
        self.css_collection.find_closest(&target, max_results, None)
    }

    /// Find closest RAL Classic colors (backward compatibility)
    #[must_use]
    pub fn find_closest_ral_classic(&self, rgb: [u8; 3], max_results: usize) -> Vec<ColorMatch> {
        let target = UniversalColor::from_rgb(rgb);
        self.ral_classic_collection
            .find_closest(&target, max_results, None)
    }

    /// Find closest RAL Design System+ colors (backward compatibility)
    #[must_use]
    pub fn find_closest_ral_design(&self, rgb: [u8; 3], max_results: usize) -> Vec<ColorMatch> {
        let target = UniversalColor::from_rgb(rgb);
        self.ral_design_collection
            .find_closest(&target, max_results, None)
    }

    /// Search by exact name across all collections
    #[must_use]
    pub fn find_by_name(&self, name: &str) -> Vec<(String, super::collections::ColorEntry)> {
        self.manager.search_by_name(name)
    }

    /// Find color by exact code (RAL codes, etc.)
    #[must_use]
    pub fn find_by_code(&self, code: &str) -> Option<(String, super::collections::ColorEntry)> {
        if let Some(entry) = self.ral_classic_collection.find_by_code(code) {
            return Some(("RAL Classic".to_string(), entry));
        }
        if let Some(entry) = self.ral_design_collection.find_by_code(code) {
            return Some(("RAL Design System+".to_string(), entry));
        }
        None
    }

    /// Search with advanced filtering
    #[must_use]
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

    // Functional API methods

    /// Find closest colors across all collections with custom distance algorithm
    pub fn find_closest_across_all_with_algorithm(
        &self,
        rgb: [u8; 3],
        max_results_per_collection: usize,
        algorithm: DistanceAlgorithm,
    ) -> Vec<(String, Vec<ColorMatch>)> {
        let target = UniversalColor::from_rgb(rgb);
        self.manager.find_closest_across_all_with_algorithm(
            &target,
            max_results_per_collection,
            None,
            algorithm,
        )
    }

    /// Find closest CSS named colors with custom distance algorithm
    pub fn find_closest_css_colors_with_algorithm(
        &self,
        rgb: [u8; 3],
        max_results: usize,
        algorithm: DistanceAlgorithm,
    ) -> Vec<ColorMatch> {
        let target = UniversalColor::from_rgb(rgb);
        self.css_collection
            .find_closest_with_algorithm(&target, max_results, None, algorithm)
    }

    /// Find closest RAL Classic colors with custom distance algorithm
    pub fn find_closest_ral_classic_with_algorithm(
        &self,
        rgb: [u8; 3],
        max_results: usize,
        algorithm: DistanceAlgorithm,
    ) -> Vec<ColorMatch> {
        let target = UniversalColor::from_rgb(rgb);
        self.ral_classic_collection
            .find_closest_with_algorithm(&target, max_results, None, algorithm)
    }

    /// Find closest RAL Design System+ colors with custom distance algorithm
    pub fn find_closest_ral_design_with_algorithm(
        &self,
        rgb: [u8; 3],
        max_results: usize,
        algorithm: DistanceAlgorithm,
    ) -> Vec<ColorMatch> {
        let target = UniversalColor::from_rgb(rgb);
        self.ral_design_collection
            .find_closest_with_algorithm(&target, max_results, None, algorithm)
    }
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
