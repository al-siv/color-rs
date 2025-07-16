//! Unified System Demo Example
//!
//! This example demonstrates comprehensive usage of the color collection system.

use color_rs::color_parser::{
    ColorCollection, CssColorCollection, RalClassicCollection, RalDesignCollection, SearchFilter,
    UniversalColor,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Unified Color System Comprehensive Demo ===\n");

    // Create collections
    let css_collection = CssColorCollection::new()?;
    let ral_classic_collection = RalClassicCollection::new()?;
    let ral_design_collection = RalDesignCollection::new()?;

    // Demo 1: Color matching across different collections
    println!("1. Color matching across different collections:");
    demo_color_matching(
        &css_collection,
        &ral_classic_collection,
        &ral_design_collection,
    )?;

    // Demo 2: Group-based filtering
    println!("\n\n2. Group-based filtering:");
    demo_group_filtering(&ral_classic_collection, &ral_design_collection)?;

    // Demo 3: Code-based lookups
    println!("\n\n3. Code-based lookups:");
    demo_code_lookups(
        &css_collection,
        &ral_classic_collection,
        &ral_design_collection,
    )?;

    // Demo 4: Collection statistics
    println!("\n\n4. Collection statistics:");
    demo_collection_stats(
        &css_collection,
        &ral_classic_collection,
        &ral_design_collection,
    )?;

    println!("\n=== Demo Complete ===");
    Ok(())
}

fn demo_color_matching(
    css: &CssColorCollection,
    ral_classic: &RalClassicCollection,
    ral_design: &RalDesignCollection,
) -> Result<(), Box<dyn std::error::Error>> {
    let target_color = UniversalColor::from_rgb([255, 0, 0]); // Red

    println!("   Target color: rgb(255, 0, 0) - Pure Red");

    // Find closest in each collection
    let css_matches = css.find_closest(&target_color, 2, None);
    println!("\n   CSS Color matches:");
    for (i, m) in css_matches.iter().enumerate() {
        println!(
            "     {}. {} (ΔE: {:.2})",
            i + 1,
            m.entry.metadata.name,
            m.distance
        );
    }

    let ral_classic_matches = ral_classic.find_closest(&target_color, 2, None);
    println!("\n   RAL Classic matches:");
    for (i, m) in ral_classic_matches.iter().enumerate() {
        println!(
            "     {}. {} ({}) (ΔE: {:.2})",
            i + 1,
            m.entry.metadata.name,
            m.entry.metadata.code.as_ref().unwrap(),
            m.distance
        );
    }

    let ral_design_matches = ral_design.find_closest(&target_color, 2, None);
    println!("\n   RAL Design System+ matches:");
    for (i, m) in ral_design_matches.iter().enumerate() {
        println!(
            "     {}. {} ({}) (ΔE: {:.2})",
            i + 1,
            m.entry.metadata.name,
            m.entry.metadata.code.as_ref().unwrap(),
            m.distance
        );
    }

    Ok(())
}

fn demo_group_filtering(
    ral_classic: &RalClassicCollection,
    ral_design: &RalDesignCollection,
) -> Result<(), Box<dyn std::error::Error>> {
    let target_color = UniversalColor::from_rgb([0, 0, 255]); // Blue

    println!("   Target color: rgb(0, 0, 255) - Pure Blue");

    // RAL Classic group filtering
    let blue_groups = vec!["RAL 5000".to_string()];
    let filter = SearchFilter {
        groups: Some(blue_groups),
        ..Default::default()
    };

    let filtered_matches = ral_classic.find_closest(&target_color, 3, Some(&filter));
    println!("\n   RAL Classic blue group (RAL 5000) matches:");
    for (i, m) in filtered_matches.iter().enumerate() {
        println!(
            "     {}. {} ({}) (ΔE: {:.2})",
            i + 1,
            m.entry.metadata.name,
            m.entry.metadata.code.as_ref().unwrap(),
            m.distance
        );
    }

    // RAL Design System+ filtering example
    let ral_design_matches = ral_design.find_closest(&target_color, 2, None);
    println!("\n   RAL Design System+ blue matches:");
    for (i, m) in ral_design_matches.iter().enumerate() {
        println!(
            "     {}. {} ({}) (ΔE: {:.2})",
            i + 1,
            m.entry.metadata.name,
            m.entry.metadata.code.as_ref().unwrap(),
            m.distance
        );
    }

    Ok(())
}

fn demo_code_lookups(
    css: &CssColorCollection,
    ral_classic: &RalClassicCollection,
    ral_design: &RalDesignCollection,
) -> Result<(), Box<dyn std::error::Error>> {
    // Test code lookups
    let test_codes = vec![
        ("red", "CSS"),
        ("RAL 1000", "RAL Classic"),
        ("RAL 010 20 20", "RAL Design System+"),
    ];

    for (code, collection_name) in test_codes {
        print!("   Looking up '{}' in {}: ", code, collection_name);

        let result = match collection_name {
            "CSS" => css.find_by_code(code),
            "RAL Classic" => ral_classic.find_by_code(code),
            "RAL Design System+" => ral_design.find_by_code(code),
            _ => None,
        };

        if let Some(entry) = result {
            println!("Found - {}", entry.metadata.name);
        } else {
            println!("Not found");
        }
    }

    Ok(())
}

fn demo_collection_stats(
    css: &CssColorCollection,
    ral_classic: &RalClassicCollection,
    ral_design: &RalDesignCollection,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("   CSS Colors: {} colors", css.colors().len());
    println!("   RAL Classic: {} colors", ral_classic.colors().len());
    println!(
        "   RAL Design System+: {} colors",
        ral_design.colors().len()
    );

    let total_colors = css.colors().len() + ral_classic.colors().len() + ral_design.colors().len();
    println!("   Total colors available: {}", total_colors);

    // Group statistics
    let ral_classic_groups = ral_classic.groups();
    let ral_design_groups = ral_design.groups();

    println!("\n   Group statistics:");
    println!("     RAL Classic groups: {}", ral_classic_groups.len());
    println!(
        "     RAL Design System+ groups: {}",
        ral_design_groups.len()
    );

    Ok(())
}
