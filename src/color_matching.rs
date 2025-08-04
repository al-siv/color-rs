//! Color matching pipeline using functional composition
//!
//! This module implements a functional approach to color matching using
//! higher-order functions and function composition.

use crate::color_distance_strategies::DistanceAlgorithm;
use crate::color_parser::{ColorMatch, UniversalColor, collections::ColorCollection};
use crate::error::{ColorError, Result};

/// Collection types supported by the functional matching pipeline
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionType {
    /// CSS named colors
    Css,
    /// RAL Classic color system
    RalClassic, 
    /// RAL Design System+ colors
    RalDesign,
}

impl CollectionType {
    /// Get the display name for the collection
    pub const fn name(self) -> &'static str {
        match self {
            Self::Css => "CSS Colors",
            Self::RalClassic => "RAL Classic",  
            Self::RalDesign => "RAL Design System+",
        }
    }
}

/// Configuration for color matching pipeline
#[derive(Debug, Clone)]
pub struct MatchingConfig {
    /// Collection type to search
    pub collection_type: CollectionType,
    /// Distance algorithm to use
    pub algorithm: DistanceAlgorithm,
    /// Maximum number of results
    pub limit: usize,
    /// Enable collection-specific validation
    pub enable_validation: bool,
    /// Enable collection-specific preprocessing
    pub enable_preprocessing: bool,
    /// Enable collection-specific post-processing  
    pub enable_post_processing: bool,
}

impl MatchingConfig {
    /// Create a basic configuration with defaults
    pub const fn new(collection_type: CollectionType, algorithm: DistanceAlgorithm) -> Self {
        Self {
            collection_type,
            algorithm,
            limit: 10,
            enable_validation: true,
            enable_preprocessing: false,
            enable_post_processing: false,
        }
    }

    /// Builder pattern for configuration
    pub const fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Builder pattern for validation
    pub const fn with_validation(mut self, enable: bool) -> Self {
        self.enable_validation = enable;
        self
    }

    /// Builder pattern for preprocessing
    pub const fn with_preprocessing(mut self, enable: bool) -> Self {
        self.enable_preprocessing = enable;
        self
    }

    /// Builder pattern for post-processing
    pub const fn with_post_processing(mut self, enable: bool) -> Self {
        self.enable_post_processing = enable;
        self
    }
}

/// Validation function type for input colors
pub type ValidationFn = fn(&UniversalColor) -> Result<()>;

/// Preprocessing function type for target colors
pub type PreprocessFn = fn(&UniversalColor) -> Result<UniversalColor>;

/// Core matching function type
pub type MatchFn = fn(&UniversalColor, DistanceAlgorithm, usize) -> Result<Vec<ColorMatch>>;

/// Post-processing function type for results
pub type PostProcessFn = fn(Vec<ColorMatch>) -> Result<Vec<ColorMatch>>;

/// Pipeline step results for composition
#[derive(Debug, Clone)]
pub struct PipelineState {
    /// Current target color being processed
    pub target: UniversalColor,
    /// Current matches found
    pub matches: Vec<ColorMatch>,
    /// Configuration used
    pub config: MatchingConfig,
}

/// Functional color matching pipeline - replaces the Template Method pattern
///
/// This function composes the matching algorithm from pure functions instead
/// of using trait inheritance and virtual method dispatch.
pub fn match_color(
    target: &UniversalColor,
    config: &MatchingConfig,
    validation_fn: Option<ValidationFn>,
    preprocess_fn: Option<PreprocessFn>, 
    match_fn: MatchFn,
    post_process_fn: Option<PostProcessFn>,
) -> Result<Vec<ColorMatch>> {
    
    // Step 1: Validation pipeline
    if config.enable_validation {
        // Apply collection-specific validation if provided
        if let Some(validator) = validation_fn {
            validator(target)?;
        } else {
            // Default validation
            validate_lab_basic(target)?;
        }
    }

    // Step 2: Preprocessing pipeline  
    let processed_target = if config.enable_preprocessing {
        if let Some(preprocessor) = preprocess_fn {
            preprocessor(target)?
        } else {
            target.clone()
        }
    } else {
        target.clone()
    };

    // Step 3: Core matching pipeline
    let mut matches = match_fn(&processed_target, config.algorithm, config.limit)?;

    // Step 4: Post-processing pipeline
    if config.enable_post_processing {
        if let Some(post_processor) = post_process_fn {
            matches = post_processor(matches)?;
        }
    }

    // Step 5: Final sorting and limiting pipeline
    matches.sort_by(|a, b| {
        a.distance
            .partial_cmp(&b.distance)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    matches.truncate(config.limit);

    Ok(matches)
}

/// Default LAB validation function  
pub fn validate_lab_basic(target: &UniversalColor) -> Result<()> {
    let lab = target.lab;
    if !(0.0..=100.0).contains(&lab[0]) {
        return Err(ColorError::InvalidColor(
            "LAB L* component out of range (0-100)".to_string(),
        ));
    }
    if !(-128.0..=127.0).contains(&lab[1]) {
        return Err(ColorError::InvalidColor(
            "LAB a* component out of range (-128 to 127)".to_string(),
        ));
    }
    if !(-128.0..=127.0).contains(&lab[2]) {
        return Err(ColorError::InvalidColor(
            "LAB b* component out of range (-128 to 127)".to_string(),
        ));
    }
    Ok(())
}

/// RAL Classic specific validation function
pub fn validate_ral_classic(target: &UniversalColor) -> Result<()> {
    // First apply basic validation  
    validate_lab_basic(target)?;
    
    // RAL-specific validation: ensure LAB values are reasonable for RAL colors
    let lab = target.lab;
    if !(5.0..=95.0).contains(&lab[0]) {
        // RAL colors typically don't include pure black or white
        return Err(ColorError::InvalidColor(
            "Color outside typical RAL range".to_string(),
        ));
    }
    Ok(())
}

/// RAL Design System+ specific validation function
pub fn validate_ral_design(target: &UniversalColor) -> Result<()> {
    // First apply basic validation
    validate_lab_basic(target)?;
    
    // RAL Design System+ specific validation
    let lab = target.lab;
    if !(10.0..=90.0).contains(&lab[0]) {
        return Err(ColorError::InvalidColor(
            "Color outside RAL Design System+ range".to_string(),
        ));
    }
    Ok(())
}

/// Extract hue value from RAL Design System+ code for sorting
pub fn extract_hue_from_code(code: &str) -> u32 {
    // RAL Design codes follow pattern H###L##C##
    if code.starts_with('H') && code.len() >= 4 {
        code[1..4].parse().unwrap_or(0)
    } else {
        0
    }
}

/// RAL Design System+ specific post-processing function
pub fn post_process_ral_design(mut matches: Vec<ColorMatch>) -> Result<Vec<ColorMatch>> {
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

/// CSS color matching function - pure function implementation
pub fn match_css_colors(
    target: &UniversalColor,
    algorithm: DistanceAlgorithm,
    limit: usize,
) -> Result<Vec<ColorMatch>> {
    let collection = crate::color_parser::css_collection::CssColorCollection::new()?;
    Ok(collection.find_closest_with_algorithm(target, limit, None, algorithm))
}

/// RAL Classic color matching function - pure function implementation  
pub fn match_ral_classic_colors(
    target: &UniversalColor,
    algorithm: DistanceAlgorithm,
    limit: usize,
) -> Result<Vec<ColorMatch>> {
    let collection = crate::color_parser::ral_classic_collection::RalClassicCollection::new()?;
    Ok(collection.find_closest_with_algorithm(target, limit, None, algorithm))
}

/// RAL Design color matching function - pure function implementation
pub fn match_ral_design_colors(
    target: &UniversalColor,
    algorithm: DistanceAlgorithm,
    limit: usize,
) -> Result<Vec<ColorMatch>> {
    let collection = crate::color_parser::ral_design_collection::RalDesignCollection::new()?;
    Ok(collection.find_closest_with_algorithm(target, limit, None, algorithm))
}

/// Get the appropriate matching function for a collection type
pub fn get_match_function(collection_type: CollectionType) -> MatchFn {
    match collection_type {
        CollectionType::Css => match_css_colors,
        CollectionType::RalClassic => match_ral_classic_colors,
        CollectionType::RalDesign => match_ral_design_colors,
    }
}

/// Get the appropriate validation function for a collection type
pub fn get_validation_function(collection_type: CollectionType) -> Option<ValidationFn> {
    match collection_type {
        CollectionType::Css => None, // Uses default validation
        CollectionType::RalClassic => Some(validate_ral_classic),
        CollectionType::RalDesign => Some(validate_ral_design),
    }
}

/// Get the appropriate post-processing function for a collection type  
pub fn get_post_process_function(collection_type: CollectionType) -> Option<PostProcessFn> {
    match collection_type {
        CollectionType::Css | CollectionType::RalClassic => None,
        CollectionType::RalDesign => Some(post_process_ral_design),
    }
}

/// High-level convenience function - matches the original Template Method interface
pub fn match_color_by_type(
    target: &UniversalColor,
    collection_type: CollectionType,
    algorithm: DistanceAlgorithm,
    limit: usize,
) -> Result<Vec<ColorMatch>> {
    let config = MatchingConfig::new(collection_type, algorithm)
        .with_limit(limit)
        .with_validation(true)
        .with_post_processing(true);

    let validation_fn = get_validation_function(collection_type);
    let match_fn = get_match_function(collection_type);
    let post_process_fn = get_post_process_function(collection_type);

    match_color(
        target,
        &config,
        validation_fn,
        None, // No preprocessing by default
        match_fn,
        post_process_fn,
    )
}

/// Unified color matcher using functional composition
pub fn match_across_all_collections(
    target: &UniversalColor,
    algorithm: DistanceAlgorithm,
    limit_per_collection: usize,
) -> Result<Vec<ColorMatch>> {
    let mut all_matches = Vec::new();

    // Use functional approach for each collection
    for collection_type in [CollectionType::Css, CollectionType::RalClassic, CollectionType::RalDesign] {
        let matches = match_color_by_type(target, collection_type, algorithm, limit_per_collection)?;
        all_matches.extend(matches);
    }

    // Final sorting and limiting
    all_matches.sort_by(|a, b| {
        a.distance
            .partial_cmp(&b.distance)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    all_matches.truncate(limit_per_collection * 3);

    Ok(all_matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functional_css_matching() {
        let target = UniversalColor::from_rgb([255, 0, 0]); // Red
        let config = MatchingConfig::new(CollectionType::Css, DistanceAlgorithm::DeltaE76)
            .with_limit(5);

        let matches = match_color(
            &target,
            &config,
            None, // No custom validation
            None, // No preprocessing  
            match_css_colors,
            None, // No post-processing
        ).unwrap();

        assert!(!matches.is_empty());
        assert!(matches.len() <= 5);

        // Verify sorting by distance
        for i in 1..matches.len() {
            assert!(matches[i].distance >= matches[i - 1].distance);
        }
    }

    #[test]
    fn test_functional_validation() {
        let invalid_target = UniversalColor {
            lab: [150.0, 0.0, 0.0], // L* > 100
            rgb: [255, 0, 0],
            luminance: None,
        };

        let config = MatchingConfig::new(CollectionType::Css, DistanceAlgorithm::DeltaE76)
            .with_validation(true);

        let result = match_color(
            &invalid_target,
            &config,
            Some(validate_lab_basic),
            None,
            match_css_colors,
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_convenience_function() {
        let target = UniversalColor::from_rgb([255, 0, 0]); // Red
        let matches = match_color_by_type(
            &target,
            CollectionType::Css,
            DistanceAlgorithm::Lch,
            5,
        ).unwrap();

        assert!(!matches.is_empty());
        assert!(matches.len() <= 5);
    }

    #[test]
    fn test_unified_matching() {
        let target = UniversalColor::from_rgb([255, 0, 0]); // Red
        let matches = match_across_all_collections(&target, DistanceAlgorithm::Lch, 3).unwrap();

        assert!(!matches.is_empty());
        assert!(matches.len() <= 9); // 3 per collection * 3 collections

        // Verify sorting by distance
        for i in 1..matches.len() {
            assert!(matches[i].distance >= matches[i - 1].distance);
        }
    }

    #[test]
    fn test_ral_design_hue_extraction() {
        assert_eq!(extract_hue_from_code("H040L50C20"), 40);
        assert_eq!(extract_hue_from_code("H120L60C30"), 120);
        assert_eq!(extract_hue_from_code("INVALID"), 0);
    }

    #[test]
    fn test_collection_type_names() {
        assert_eq!(CollectionType::Css.name(), "CSS Colors");
        assert_eq!(CollectionType::RalClassic.name(), "RAL Classic");
        assert_eq!(CollectionType::RalDesign.name(), "RAL Design System+");
    }

    #[test]
    fn test_config_builder() {
        let config = MatchingConfig::new(CollectionType::Css, DistanceAlgorithm::Lch)
            .with_limit(20)
            .with_validation(false)
            .with_preprocessing(true)
            .with_post_processing(true);

        assert_eq!(config.collection_type, CollectionType::Css);
        assert_eq!(config.algorithm, DistanceAlgorithm::Lch);
        assert_eq!(config.limit, 20);
        assert!(!config.enable_validation);
        assert!(config.enable_preprocessing);
        assert!(config.enable_post_processing);
    }
}
