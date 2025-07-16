//! Unified Color Collection System Example
//!
//! This example demonstrates the new unified color collection system capabilities.

use color_rs::color_parser::{
    ColorCollection, CssColorCollection, RalClassicCollection, RalDesignCollection, SearchFilter,
    UniversalColor,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Unified Color Collection System Demo ===\n");

    // Create individual collections
    let css_collection = CssColorCollection::new()?;
    let ral_classic_collection = RalClassicCollection::new()?;
    let ral_design_collection = RalDesignCollection::new()?;

    // Example 1: Find closest colors in each collection
    println!("1. Finding closest colors to red (255, 0, 0) in each collection:");
    let red_target = UniversalColor::from_rgb([255, 0, 0]);

    // CSS Colors
    println!("\n   CSS Colors:");
    let css_matches = css_collection.find_closest(&red_target, 2, None);
    for (i, color_match) in css_matches.iter().enumerate() {
        println!(
            "     {}. {} - Distance: {:.2}",
            i + 1,
            color_match.entry.metadata.name,
            color_match.distance
        );
    }

    // RAL Classic
    println!("\n   RAL Classic:");
    let ral_classic_matches = ral_classic_collection.find_closest(&red_target, 2, None);
    for (i, color_match) in ral_classic_matches.iter().enumerate() {
        println!(
            "     {}. {} ({}) - Distance: {:.2}",
            i + 1,
            color_match.entry.metadata.name,
            color_match
                .entry
                .metadata
                .code
                .as_ref()
                .unwrap_or(&"Unknown".to_string()),
            color_match.distance
        );
    }

    // RAL Design System+
    println!("\n   RAL Design System+:");
    let ral_design_matches = ral_design_collection.find_closest(&red_target, 2, None);
    for (i, color_match) in ral_design_matches.iter().enumerate() {
        println!(
            "     {}. {} ({}) - Distance: {:.2}",
            i + 1,
            color_match.entry.metadata.name,
            color_match
                .entry
                .metadata
                .code
                .as_ref()
                .unwrap_or(&"Unknown".to_string()),
            color_match.distance
        );
    }

    // Example 2: RAL Classic group filtering
    println!("\n\n2. Finding red colors in RAL Classic red group (RAL 3000):");
    let filter = SearchFilter {
        groups: Some(vec!["RAL 3000".to_string()]),
        ..Default::default()
    };
    let ral_classic_reds = ral_classic_collection.find_closest(&red_target, 3, Some(&filter));

    for (i, color_match) in ral_classic_reds.iter().enumerate() {
        println!(
            "   {}. {} ({}) - Distance: {:.2}",
            i + 1,
            color_match.entry.metadata.name,
            color_match.entry.metadata.code.as_ref().unwrap(),
            color_match.distance
        );
    }

    // Example 3: Find specific colors by code
    println!("\n\n3. Finding specific colors by code:");

    if let Some(entry) = ral_classic_collection.find_by_code("RAL 1000") {
        println!("   Found RAL 1000: {}", entry.metadata.name);
    }

    if let Some(entry) = css_collection.find_by_code("red") {
        println!("   Found CSS red: {}", entry.metadata.name);
    }

    // Example 4: Working with blue colors
    println!("\n\n4. Finding blue colors across collections:");
    let blue_target = UniversalColor::from_rgb([0, 100, 200]);

    let css_blues = css_collection.find_closest(&blue_target, 2, None);
    println!("\n   CSS Blues:");
    for (i, color_match) in css_blues.iter().enumerate() {
        println!(
            "     {}. {} - Distance: {:.2}",
            i + 1,
            color_match.entry.metadata.name,
            color_match.distance
        );
    }

    // Example 5: Exploring collection groups
    println!("\n\n5. Available groups in collections:");

    let ral_classic_groups = ral_classic_collection.groups();
    println!(
        "   RAL Classic groups: {} groups available",
        ral_classic_groups.len()
    );
    for group in ral_classic_groups.iter().take(5) {
        println!("     - {}", group);
    }

    let ral_design_groups = ral_design_collection.groups();
    println!(
        "   RAL Design groups: {} groups available",
        ral_design_groups.len()
    );
    for group in ral_design_groups.iter().take(5) {
        println!("     - {}", group);
    }

    println!("\n=== Demo Complete ===");
    Ok(())
}
