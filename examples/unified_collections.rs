//! Unified Color Collection System Example
//!
//! This example demonstrates the new unified color collection system capabilities.

use color_rs::color_parser::{
    UnifiedColorManager, SearchFilter, UniversalColor, ColorCollection,
    CssColorCollection
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Unified Color Collection System Demo ===\n");

    // Create the unified manager
    let manager = UnifiedColorManager::new();

    // Example 1: Find closest colors across all collections
    println!("1. Finding closest colors to red (255, 0, 0) across all collections:");
    let red_rgb = [255, 0, 0];
    let results = manager.find_closest_across_all(red_rgb, 2);
    
    for (collection_name, matches) in results {
        println!("\n   {} Collection:", collection_name);
        for (i, color_match) in matches.iter().enumerate() {
            println!("     {}. {} - Distance: {:.2}, Confidence: {:.1}%", 
                i + 1, 
                color_match.entry.metadata.name,
                color_match.distance,
                color_match.confidence * 100.0
            );
            if let Some(ref code) = color_match.entry.metadata.code {
                println!("        Code: {}", code);
            }
        }
    }

    // Example 2: RAL Classic group filtering
    println!("\n\n2. Finding red colors in RAL Classic red group (RAL 3000):");
    let ral_red_groups = vec!["RAL 3000".to_string()];
    let ral_classic_reds = manager.find_ral_classic_in_groups(red_rgb, &ral_red_groups, 3);
    
    for (i, color_match) in ral_classic_reds.iter().enumerate() {
        println!("   {}. {} ({}) - Distance: {:.2}", 
            i + 1,
            color_match.entry.metadata.name,
            color_match.entry.metadata.code.as_ref().unwrap(),
            color_match.distance
        );
    }

    // Example 3: RAL Design System+ hue filtering
    println!("\n\n3. Finding colors in RAL Design System+ red hue group:");
    let red_hue_groups = vec!["Red".to_string()];
    let ral_design_reds = manager.find_ral_design_in_hue_groups(red_rgb, &red_hue_groups, 3);
    
    for (i, color_match) in ral_design_reds.iter().enumerate() {
        println!("   {}. {} ({}) - Distance: {:.2}", 
            i + 1,
            color_match.entry.metadata.name,
            color_match.entry.metadata.code.as_ref().unwrap(),
            color_match.distance
        );
    }

    // Example 4: Search by exact code
    println!("\n\n4. Finding colors by exact codes:");
    
    if let Some((collection, entry)) = manager.find_by_code("RAL 1000") {
        println!("   RAL 1000: {} from {} collection", entry.metadata.name, collection);
        println!("   RGB: {:?}", entry.color.rgb);
    }
    
    if let Some((collection, entry)) = manager.find_by_code("H010L20C10") {
        println!("   H010L20C10: {} from {} collection", entry.metadata.name, collection);
        println!("   RGB: {:?}", entry.color.rgb);
    }

    // Example 5: Advanced filtering with SearchFilter
    println!("\n\n5. Advanced filtering - Find colors with medium to high luminance:");
    let filter = SearchFilter {
        luminance_range: Some([0.3, 0.8]), // 30% to 80% luminance
        max_distance: Some(50.0), // Within reasonable color distance
        ..Default::default()
    };
    
    let filtered_results = manager.search_with_filter([100, 150, 200], &filter, 2);
    for (collection_name, matches) in filtered_results {
        println!("\n   {} Collection (filtered):", collection_name);
        for (i, color_match) in matches.iter().enumerate() {
            println!("     {}. {} - Distance: {:.2}", 
                i + 1, 
                color_match.entry.metadata.name,
                color_match.distance
            );
        }
    }

    // Example 6: Individual collection usage
    println!("\n\n6. Using individual collections directly:");
    
    let css_collection = CssColorCollection::new();
    let blue_target = UniversalColor::from_rgb([0, 0, 255]);
    let css_blues = css_collection.find_closest(&blue_target, 3, None);
    
    println!("\n   Closest CSS colors to blue:");
    for (i, color_match) in css_blues.iter().enumerate() {
        println!("     {}. {} - Distance: {:.2}", 
            i + 1,
            color_match.entry.metadata.name,
            color_match.distance
        );
    }

    // Example 7: Available groups
    println!("\n\n7. Available color groups:");
    
    println!("\n   RAL Classic Groups:");
    for group in manager.get_ral_classic_groups() {
        println!("     - {}", group);
    }
    
    println!("\n   RAL Design Hue Groups:");
    for group in manager.get_ral_design_hue_groups() {
        println!("     - {}", group);
    }

    println!("\n=== End of Demo ===");
    Ok(())
}
