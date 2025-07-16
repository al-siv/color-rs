// Advanced Unified Color Collection System Demo
// Demonstrates the complete capabilities of the v0.10.0 color-rs system

use color_rs::UnifiedColorManager;

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
        for result in matches.iter().take(1) {
            // Show just one per collection for brevity
            println!(
                "    {} - {} (distance: {:.2})",
                result
                    .entry
                    .metadata
                    .code
                    .as_ref()
                    .unwrap_or(&"N/A".to_string()),
                result.entry.metadata.name,
                result.distance
            );
        }
    }

    // 2. RAL Classic group filtering
    println!("\n2. RAL Classic red groups (RAL 3000):");
    let ral_groups = vec!["RAL 3000".to_string()];
    let ral_reds = manager.find_ral_classic_in_groups(target_color, &ral_groups, 3);

    for result in &ral_reds {
        println!(
            "    {} - {} (group: {})",
            result
                .entry
                .metadata
                .code
                .as_ref()
                .unwrap_or(&"N/A".to_string()),
            result.entry.metadata.name,
            result
                .entry
                .metadata
                .group
                .as_ref()
                .unwrap_or(&"N/A".to_string())
        );
    }

    // 3. RAL Design System+ hue filtering
    println!("\n3. RAL Design System+ red hues:");
    let design_reds = manager.find_ral_design_in_hue_groups(target_color, &["Red".to_string()], 3);

    for result in &design_reds {
        println!(
            "    {} - {} (hue group: {})",
            result
                .entry
                .metadata
                .code
                .as_ref()
                .unwrap_or(&"N/A".to_string()),
            result.entry.metadata.name,
            result
                .entry
                .metadata
                .group
                .as_ref()
                .unwrap_or(&"N/A".to_string())
        );
    }

    // 4. Lightness filtering for RAL Design System+
    println!("\n4. RAL Design System+ medium lightness range:");
    let lightness_results = manager.find_ral_design_in_lightness_range(target_color, 30.0, 70.0, 3);

    for result in &lightness_results {
        println!(
            "    {} - {}",
            result
                .entry
                .metadata
                .code
                .as_ref()
                .unwrap_or(&"N/A".to_string()),
            result.entry.metadata.name
        );
    }

    // 5. Pattern-based name search for RAL colors
    println!("\n5. Pattern search for 'red' in RAL colors:");
    let red_patterns = manager.find_ral_by_name_pattern("red");

    for (collection_name, entry) in red_patterns.iter().take(3) {
        println!(
            "    {} - {} (collection: {})",
            entry.metadata.code.as_ref().unwrap_or(&"N/A".to_string()),
            entry.metadata.name,
            collection_name
        );
    }

    // 6. Exact code lookup demonstration
    println!("\n6. Exact code lookups:");

    let test_codes = vec!["RAL 1000", "H010L20C10"];

    for code in test_codes {
        if let Some((collection_name, entry)) = manager.find_by_code(code) {
            println!(
                "    {} -> {} ({})",
                code, entry.metadata.name, collection_name
            );
        } else {
            println!("    {} -> Not found", code);
        }
    }

    println!("\n=== Demo Complete ===");
    Ok(())
}
