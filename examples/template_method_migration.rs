//! Template Method Pattern Migration Example
//! 
//! This example demonstrates the migration from Template Method pattern 
//! to functional programming using higher-order functions and composition.

use color_rs::{
    CollectionType, MatchingConfig, DistanceAlgorithm, 
    match_color_functional, match_color_by_type, match_across_all_collections,
    validate_ral_classic, post_process_ral_design,
    UniversalColor,
    color_parser::collections::ColorCollection
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Template Method Pattern → Functional Programming Migration ===\n");

    let target = UniversalColor::from_rgb([255, 0, 0]); // Red color
    println!("Target color: Red RGB(255, 0, 0)");
    println!("LAB: {:?}\n", target.lab);

    // 1. Basic functional matching (replaces simple template method usage)
    println!("1. Basic Functional Matching:");
    let css_matches = match_color_by_type(&target, CollectionType::Css, DistanceAlgorithm::Lch, 3)?;
    println!("CSS Colors (LCH distance):");
    for (i, color_match) in css_matches.iter().enumerate() {
        println!("  {}. {} - {:.3} distance", 
            i + 1, 
            color_match.entry.metadata.name, 
            color_match.distance
        );
    }

    // 2. Advanced functional pipeline (replaces complex template method with hooks)
    println!("\n2. Advanced Functional Pipeline with Custom Validation:");
    let config = MatchingConfig::new(CollectionType::RalClassic, DistanceAlgorithm::DeltaE2000)
        .with_limit(3)
        .with_validation(true)
        .with_post_processing(false);

    let ral_matches = match_color_functional(
        &target,
        &config,
        Some(validate_ral_classic), // Custom validation function
        None,                       // No preprocessing
        |target, algorithm, limit| {
            // Custom matching function (inline closure)
            let collection = color_rs::color_parser::ral_classic_collection::RalClassicCollection::new()?;
            Ok(collection.find_closest_with_algorithm(target, limit, None, algorithm))
        },
        None, // No post-processing
    )?;

    println!("RAL Classic Colors (Delta E 2000, custom validation):");
    for (i, color_match) in ral_matches.iter().enumerate() {
        println!("  {}. {} ({}) - {:.3} distance", 
            i + 1, 
            color_match.entry.metadata.name,
            color_match.entry.metadata.code.as_deref().unwrap_or("N/A"),
            color_match.distance
        );
    }

    // 3. Function composition with post-processing (replaces template method inheritance)
    println!("\n3. Function Composition with Post-Processing:");
    let ral_design_config = MatchingConfig::new(CollectionType::RalDesign, DistanceAlgorithm::EuclideanLab)
        .with_limit(3)
        .with_validation(true)
        .with_post_processing(true);

    let design_matches = match_color_functional(
        &target,
        &ral_design_config,
        None, // Use default validation
        None, // No preprocessing
        |target, algorithm, limit| {
            // RAL Design matching
            let collection = color_rs::color_parser::ral_design_collection::RalDesignCollection::new()?;
            Ok(collection.find_closest_with_algorithm(target, limit, None, algorithm))
        },
        Some(post_process_ral_design), // Hue-based post-processing
    )?;

    println!("RAL Design System+ Colors (Euclidean LAB, hue-sorted):");
    for (i, color_match) in design_matches.iter().enumerate() {
        println!("  {}. {} ({}) - {:.3} distance", 
            i + 1, 
            color_match.entry.metadata.name,
            color_match.entry.metadata.code.as_deref().unwrap_or("N/A"),
            color_match.distance
        );
    }

    // 4. Unified matching across all collections (replaces UnifiedColorMatcher)
    println!("\n4. Unified Matching Across All Collections:");
    let unified_matches = match_across_all_collections(&target, DistanceAlgorithm::Lch, 2)?;
    
    println!("Best matches from all collections (LCH distance):");
    for (i, color_match) in unified_matches.iter().enumerate() {
        let collection_name = match color_match.entry.metadata.code {
            Some(ref code) if code.starts_with("RAL") => {
                if code.contains(' ') { "RAL Classic" } else { "RAL Design" }
            }
            _ => "CSS"
        };
        println!("  {}. {} ({}) - {:.3} distance - {}", 
            i + 1, 
            color_match.entry.metadata.name,
            color_match.entry.metadata.code.as_deref().unwrap_or("css"),
            color_match.distance,
            collection_name
        );
    }

    // 5. Configuration flexibility (shows builder pattern replacement)
    println!("\n5. Configuration Flexibility:");
    let flexible_configs = vec![
        MatchingConfig::new(CollectionType::Css, DistanceAlgorithm::DeltaE76)
            .with_limit(2)
            .with_validation(false),
        MatchingConfig::new(CollectionType::RalClassic, DistanceAlgorithm::DeltaE2000)
            .with_limit(2)
            .with_validation(true),
        MatchingConfig::new(CollectionType::RalDesign, DistanceAlgorithm::Lch)
            .with_limit(2)
            .with_post_processing(true),
    ];

    for (i, config) in flexible_configs.iter().enumerate() {
        let matches = match_color_by_type(&target, config.collection_type, config.algorithm, config.limit)?;
        println!("  Config {}: {} with {:?} - {} matches", 
            i + 1, 
            config.collection_type.name(),
            config.algorithm,
            matches.len()
        );
    }

    println!("\n=== Migration Benefits ===");
    println!("✅ Trait inheritance → Function composition");
    println!("✅ Abstract methods → Required function parameters"); 
    println!("✅ Hook methods → Optional function parameters with defaults");
    println!("✅ Template method → Configurable pipeline function");
    println!("✅ Virtual dispatch → Zero-cost enum dispatch");
    println!("✅ Mutable state → Immutable functional pipeline");
    println!("✅ Complex inheritance → Simple function composition");
    
    println!("\n=== Performance Characteristics ===");
    println!("🚀 Zero heap allocation for function calls");
    println!("🚀 Compile-time optimization of function composition");
    println!("🚀 No virtual function table lookups");
    println!("🚀 Inlined function calls in release builds");

    Ok(())
}
