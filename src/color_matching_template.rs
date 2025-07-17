//! Template Method pattern for color matching algorithms
//!
//! This module implements the Template Method pattern to provide a consistent
//! structure for color matching algorithms across different collection types.

use crate::color_distance_strategies::ColorDistanceStrategy;
use crate::color_parser::{ColorMatch, UniversalColor, collections::ColorCollection};
use crate::error::{ColorError, Result};

/// Template method for color matching algorithms
///
/// Implements the Template Method pattern to define the skeleton of color
/// matching algorithms while allowing subclasses to override specific steps.
///
/// The algorithm follows these steps:
/// 1. Validate input color
/// 2. Preprocess target color
/// 3. Perform collection-specific matching
/// 4. Post-process results
/// 5. Format final output
pub trait ColorMatchingTemplate {
    /// Template method defining the color matching algorithm structure
    ///
    /// This is the main template method that orchestrates the matching process.
    /// Subclasses should not override this method.
    fn match_color(
        &self,
        target: &UniversalColor,
        limit: usize,
        strategy: &dyn ColorDistanceStrategy,
    ) -> Result<Vec<ColorMatch>> {
        // Step 1: Validate input
        self.validate_input(target)?;

        // Step 2: Preprocess target
        let processed_target = self.preprocess_target(target)?;

        // Step 3: Perform collection-specific matching (abstract method)
        let mut matches = self.find_matches(&processed_target, strategy)?;

        // Step 4: Post-process results
        matches = self.post_process_matches(matches)?;

        // Step 5: Apply limit and sort
        matches.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        matches.truncate(limit);

        Ok(matches)
    }

    /// Validate the input color (hook method)
    ///
    /// Default implementation performs basic validation.
    /// Subclasses can override for collection-specific validation.
    fn validate_input(&self, target: &UniversalColor) -> Result<()> {
        // Basic LAB validation
        let lab = target.lab;
        if lab[0] < 0.0 || lab[0] > 100.0 {
            return Err(ColorError::InvalidColor(
                "LAB L* component out of range (0-100)".to_string(),
            ));
        }
        if lab[1] < -128.0 || lab[1] > 127.0 {
            return Err(ColorError::InvalidColor(
                "LAB a* component out of range (-128 to 127)".to_string(),
            ));
        }
        if lab[2] < -128.0 || lab[2] > 127.0 {
            return Err(ColorError::InvalidColor(
                "LAB b* component out of range (-128 to 127)".to_string(),
            ));
        }
        Ok(())
    }

    /// Preprocess the target color (hook method)
    ///
    /// Default implementation returns the target unchanged.
    /// Subclasses can override for collection-specific preprocessing.
    fn preprocess_target(&self, target: &UniversalColor) -> Result<UniversalColor> {
        Ok(target.clone())
    }

    /// Find matches in the collection (abstract method)
    ///
    /// This method must be implemented by concrete classes to perform
    /// the actual matching against their specific color collections.
    fn find_matches(
        &self,
        target: &UniversalColor,
        strategy: &dyn ColorDistanceStrategy,
    ) -> Result<Vec<ColorMatch>>;

    /// Post-process the matching results (hook method)
    ///
    /// Default implementation returns results unchanged.
    /// Subclasses can override for collection-specific post-processing.
    fn post_process_matches(&self, matches: Vec<ColorMatch>) -> Result<Vec<ColorMatch>> {
        Ok(matches)
    }

    /// Get the collection name (for debugging and logging)
    fn get_collection_name(&self) -> &'static str;
}

/// Concrete implementation for CSS color collections
pub struct CssColorMatcher<'a> {
    collection: &'a crate::color_parser::css_collection::CssColorCollection,
}

impl<'a> CssColorMatcher<'a> {
    pub fn new(collection: &'a crate::color_parser::css_collection::CssColorCollection) -> Self {
        Self { collection }
    }
}

impl<'a> ColorMatchingTemplate for CssColorMatcher<'a> {
    fn find_matches(
        &self,
        target: &UniversalColor,
        strategy: &dyn ColorDistanceStrategy,
    ) -> Result<Vec<ColorMatch>> {
        Ok(self
            .collection
            .find_closest_with_strategy(target, usize::MAX, None, strategy))
    }

    fn get_collection_name(&self) -> &'static str {
        "CSS Colors"
    }
}

/// Concrete implementation for RAL Classic collections
pub struct RalClassicMatcher<'a> {
    collection: &'a crate::color_parser::ral_classic_collection::RalClassicCollection,
}

impl<'a> RalClassicMatcher<'a> {
    pub fn new(
        collection: &'a crate::color_parser::ral_classic_collection::RalClassicCollection,
    ) -> Self {
        Self { collection }
    }
}

impl<'a> ColorMatchingTemplate for RalClassicMatcher<'a> {
    fn validate_input(&self, target: &UniversalColor) -> Result<()> {
        // Call parent validation first using explicit syntax
        <dyn ColorMatchingTemplate>::validate_input(self, target)?;

        // RAL-specific validation: ensure LAB values are reasonable for RAL colors
        let lab = target.lab;
        if lab[0] < 5.0 || lab[0] > 95.0 {
            // RAL colors typically don't include pure black or white
            return Err(ColorError::InvalidColor(
                "Color outside typical RAL range".to_string(),
            ));
        }
        Ok(())
    }

    fn find_matches(
        &self,
        target: &UniversalColor,
        strategy: &dyn ColorDistanceStrategy,
    ) -> Result<Vec<ColorMatch>> {
        Ok(self
            .collection
            .find_closest_with_strategy(target, usize::MAX, None, strategy))
    }

    fn get_collection_name(&self) -> &'static str {
        "RAL Classic"
    }
}

/// Concrete implementation for RAL Design System+ collections
pub struct RalDesignMatcher<'a> {
    collection: &'a crate::color_parser::ral_design_collection::RalDesignCollection,
}

impl<'a> RalDesignMatcher<'a> {
    pub fn new(
        collection: &'a crate::color_parser::ral_design_collection::RalDesignCollection,
    ) -> Self {
        Self { collection }
    }
}

impl<'a> ColorMatchingTemplate for RalDesignMatcher<'a> {
    fn validate_input(&self, target: &UniversalColor) -> Result<()> {
        // Call parent validation first using explicit syntax
        <dyn ColorMatchingTemplate>::validate_input(self, target)?;

        // RAL Design System+ specific validation
        let lab = target.lab;
        if lab[0] < 10.0 || lab[0] > 90.0 {
            return Err(ColorError::InvalidColor(
                "Color outside RAL Design System+ range".to_string(),
            ));
        }
        Ok(())
    }

    fn find_matches(
        &self,
        target: &UniversalColor,
        strategy: &dyn ColorDistanceStrategy,
    ) -> Result<Vec<ColorMatch>> {
        Ok(self
            .collection
            .find_closest_with_strategy(target, usize::MAX, None, strategy))
    }

    fn post_process_matches(&self, mut matches: Vec<ColorMatch>) -> Result<Vec<ColorMatch>> {
        // RAL Design System+ specific post-processing: group by hue families
        matches.sort_by(|a, b| {
            // First sort by distance, then by hue group for tie-breaking
            let distance_cmp = a
                .distance
                .partial_cmp(&b.distance)
                .unwrap_or(std::cmp::Ordering::Equal);
            if distance_cmp == std::cmp::Ordering::Equal {
                // Extract hue information from color codes for secondary sorting
                let hue_a = extract_hue_from_code(a.entry.metadata.code.as_deref().unwrap_or(""));
                let hue_b = extract_hue_from_code(b.entry.metadata.code.as_deref().unwrap_or(""));
                hue_a.cmp(&hue_b)
            } else {
                distance_cmp
            }
        });
        Ok(matches)
    }

    fn get_collection_name(&self) -> &'static str {
        "RAL Design System+"
    }
}

/// Extract hue value from RAL Design System+ code for sorting
fn extract_hue_from_code(code: &str) -> u32 {
    // RAL Design codes follow pattern H###L##C##
    if code.starts_with('H') && code.len() >= 4 {
        code[1..4].parse().unwrap_or(0)
    } else {
        0
    }
}

/// Unified color matcher that uses template method pattern
pub struct UnifiedColorMatcher {
    css_collection: crate::color_parser::css_collection::CssColorCollection,
    ral_classic_collection: crate::color_parser::ral_classic_collection::RalClassicCollection,
    ral_design_collection: crate::color_parser::ral_design_collection::RalDesignCollection,
}

impl UnifiedColorMatcher {
    /// Create a new unified matcher with all collections
    pub fn new() -> Result<Self> {
        Ok(Self {
            css_collection: crate::color_parser::css_collection::CssColorCollection::new()?,
            ral_classic_collection:
                crate::color_parser::ral_classic_collection::RalClassicCollection::new()?,
            ral_design_collection:
                crate::color_parser::ral_design_collection::RalDesignCollection::new()?,
        })
    }

    /// Find closest matches across all collections using template method
    pub fn find_closest_across_all(
        &self,
        target: &UniversalColor,
        limit_per_collection: usize,
        strategy: &dyn ColorDistanceStrategy,
    ) -> Result<Vec<ColorMatch>> {
        let mut all_matches = Vec::new();

        // Use template method for each collection
        let css_matcher = CssColorMatcher::new(&self.css_collection);
        let css_matches = css_matcher.match_color(target, limit_per_collection, strategy)?;
        all_matches.extend(css_matches);

        let ral_classic_matcher = RalClassicMatcher::new(&self.ral_classic_collection);
        let ral_classic_matches =
            ral_classic_matcher.match_color(target, limit_per_collection, strategy)?;
        all_matches.extend(ral_classic_matches);

        let ral_design_matcher = RalDesignMatcher::new(&self.ral_design_collection);
        let ral_design_matches =
            ral_design_matcher.match_color(target, limit_per_collection, strategy)?;
        all_matches.extend(ral_design_matches);

        // Final sorting and limiting
        all_matches.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        all_matches.truncate(limit_per_collection * 3); // Total limit across all collections

        Ok(all_matches)
    }
}

impl Default for UnifiedColorMatcher {
    fn default() -> Self {
        Self::new().expect("Failed to create unified matcher")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color_distance_strategies::create_strategy;

    #[test]
    fn test_template_method_css_matcher() {
        let collection = crate::color_parser::css_collection::CssColorCollection::new().unwrap();
        let matcher = CssColorMatcher::new(&collection);
        let strategy = create_strategy("deltae76");

        let target = UniversalColor::from_rgb([255, 0, 0]); // Red
        let matches = matcher.match_color(&target, 5, strategy.as_ref()).unwrap();

        assert!(!matches.is_empty());
        assert!(matches.len() <= 5);

        // Verify sorting by distance
        for i in 1..matches.len() {
            assert!(matches[i].distance >= matches[i - 1].distance);
        }
    }

    #[test]
    fn test_template_method_validation() {
        let collection = crate::color_parser::css_collection::CssColorCollection::new().unwrap();
        let matcher = CssColorMatcher::new(&collection);

        // Test invalid LAB values
        let invalid_target = UniversalColor {
            lab: [150.0, 0.0, 0.0], // L* > 100
            rgb: [255, 0, 0],
            luminance: None,
        };

        let strategy = create_strategy("deltae76");
        let result = matcher.match_color(&invalid_target, 5, strategy.as_ref());
        assert!(result.is_err());
    }

    #[test]
    fn test_ral_design_hue_extraction() {
        assert_eq!(extract_hue_from_code("H040L50C20"), 40);
        assert_eq!(extract_hue_from_code("H120L60C30"), 120);
        assert_eq!(extract_hue_from_code("INVALID"), 0);
    }
}
