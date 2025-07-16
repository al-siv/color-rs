// Advanced Unified Color Collection System Demo
// Demonstrates the complete capabilities of the v0.10.0 color-rs system

use color_rs::{UnifiedColorManager, SearchFilter, ColorUtils};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Color-rs v0.10.0 Unified Collection System Demo ===\n");
    
    // Initialize the unified manager
    let manager = UnifiedColorManager::new();
    
    // 1. Cross-collection search
    println!("1. Finding closest colors across all collections:");
    let target_color = [220, 40, 40]; // Deep red
    let cross_results = manager.find_closest_across_all(target_color, 3);
    
    for (collection_name, matches) in &cross_results {
        println!("  From {}:", collection_name);
        for result in matches.iter().take(1) { // Show just one per collection for brevity
            println!("    {} - {} (distance: {:.2})", 
                    result.color.code, result.color.name, result.distance);
        }
    }
    
    // 2. RAL Classic group filtering
    println!("\n2. RAL Classic red groups (RAL 3000):");
    let ral_groups = vec!["RAL 3000".to_string()];
    let ral_reds = manager.find_ral_classic_in_groups(target_color, &ral_groups, 3);
    
    for result in &ral_reds {
        println!("  {} - {} (group: {})", 
                result.color.code, result.color.name, 
                result.color.group.as_ref().unwrap_or(&"N/A".to_string()));
    }
    
    // 3. RAL Design System+ filtering by hue
    println!("\n3. RAL Design System+ red hues:");
    let design_reds = manager.find_ral_design_by_hue(target_color, &["Red".to_string()], 3);
    
    for result in &design_reds {
        println!("  {} - {} (hue: {})", 
                result.color.code, result.color.name,
                result.color.group.as_ref().unwrap_or(&"N/A".to_string()));
    }
    
    // 4. Advanced search with multiple filters
    println!("\n4. Advanced filtering - medium luminance blues:");
    let blue_target = [70, 130, 180]; // Steel blue
    
    let filter = SearchFilter {
        groups: Some(vec!["Blue".to_string(), "RAL 5000".to_string()]),
        luminance_range: Some([0.3, 0.7]), // Medium luminance
        max_distance: Some(30.0),
        pattern: None,
        name_contains: None,
    };
    
    let filtered_results = manager.search_with_filter(blue_target, &filter, 5);
    
    for result in &filtered_results {
        println!("  {} - {} (luminance: {:.3}, distance: {:.2})", 
                result.color.code, result.color.name, 
                result.color.luminance, result.distance);
    }
    
    // 5. Pattern-based name search
    println!("\n5. Pattern search for 'green' colors:");
    let green_patterns = manager.find_by_name_pattern("green", 4);
    
    for result in &green_patterns {
        println!("  {} - {} (collection: {})", 
                result.color.code, result.color.name, result.color.collection);
    }
    
    // 6. Exact code lookup demonstration
    println!("\n6. Exact code lookups:");
    
    let test_codes = vec!["RAL 1000", "H010L20C10", "crimson"];
    
    for code in test_codes {
        if let Some(result) = manager.find_exact_code(code) {
            println!("  {} -> {} ({})", 
                    code, result.color.name, result.color.collection);
        } else {
            println!("  {} -> Not found", code);
        }
    }
    
    // 7. Color space conversion demonstration
    println!("\n7. Native color space handling:");
    
    // Find a RAL Design color to show HLC native values
    if let Some(design_color) = manager.find_exact_code("H010L20C10") {
        println!("  RAL Design System+ color in native HLC:");
        println!("    Code: {}", design_color.color.code);
        println!("    Name: {}", design_color.color.name);
        println!("    Native RGB: {:?}", design_color.color.rgb);
        if let Some(native) = &design_color.color.native_values {
            println!("    Native HLC: {}", native);
        }
    }
    
    // 8. Library integration example
    println!("\n8. Library integration - ColorUtils usage:");
    
    // Convert hex to RGB for matching
    let hex_color = "#DC143C"; // Crimson
    println!("  Input: {}", hex_color);
    
    // Parse hex color manually (since we don't have Color::from_hex)
    let rgb = if hex_color.starts_with('#') && hex_color.len() == 7 {
        let r = u8::from_str_radix(&hex_color[1..3], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex_color[3..5], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex_color[5..7], 16).unwrap_or(0);
        [r, g, b]
    } else {
        [220, 20, 60] // Fallback crimson
    };
    
    println!("  RGB: {:?}", rgb);
    
    // Find matches using the RGB values
    let matches = manager.find_closest_across_all(rgb, 2);
    println!("  Closest matches:");
    for result in matches {
        println!("    {} - {} (distance: {:.2})", 
                result.color.code, result.color.name, result.distance);
    }
    
    println!("\n=== Demo Complete ===");
    println!("The unified collection system provides:");
    println!("• Consistent API across all color collections");
    println!("• Advanced filtering and search capabilities");
    println!("• Support for different native color spaces");
    println!("• Library-friendly design for external usage");
    println!("• Complete backward compatibility");
    
    Ok(())
}
