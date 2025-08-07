#!/usr/bin/env cargo
//! # Hue Visual Output Demonstration
//!
//! This example demonstrates the library usage for generating visual output
//! from color collections with hue analysis functionality.
//!
//! ## Features Demonstrated
//! - Color collection loading and filtering
//! - Hue range filtering with wraparound support
//! - Lightness and chroma filtering
//! - Visual output generation (SVG/PNG)
//! - Both gradient and palette modes
//!
//! ## Usage
//! ```bash
//! cargo run --example hue_visual_demo
//! ```

use color_rs::{
    Result,
    cli::{HueArgs, OutputFormat},
    command_execution::execute_hue_analysis,
};

fn main() -> Result<()> {
    println!("🎨 Color-rs Hue Visual Output Demo");
    println!("===================================\n");

    // Example 1: CSS Colors with Warm Hue Range
    println!("📊 Example 1: CSS Colors - Warm Spectrum (0-60°)");
    let warm_args = HueArgs {
        collection: "css".to_string(),
        hue_range: Some("[0...60]".to_string()),
        lightness_range: Some("[50...80]".to_string()),
        chroma_range: Some("[30...70]".to_string()),
        grad: false,
        pal: true,
        svg: Some("examples/warm-colors-palette.svg".to_string()),
        png: None,
        width: 1000,
        no_labels: false,
        output_format: Some(OutputFormat::Yaml),
        output_file: Some("examples/warm-colors".to_string()),
        color_height: Some(40),
        font_size: 12,
        border_width: 5,
        border_color: "white".to_string(),
        header_text: None,
        vectorized_text: false,
    };

    match execute_hue_analysis(&warm_args, None) {
        Ok(_) => println!("✅ Generated: examples/warm-colors-palette.svg"),
        Err(e) => println!("❌ Error: {}", e),
    }

    // Example 2: RAL Classic with Cool Hue Range
    println!("\n📊 Example 2: RAL Classic - Cool Spectrum (180-270°)");
    let cool_args = HueArgs {
        collection: "ralc".to_string(),
        hue_range: Some("[180...270]".to_string()),
        lightness_range: None,
        chroma_range: None,
        grad: true,
        pal: false,
        svg: Some("examples/cool-colors-gradient.svg".to_string()),
        png: Some("examples/cool-colors-gradient.png".to_string()),
        width: 1200,
        no_labels: false,
        output_format: Some(OutputFormat::Toml),
        output_file: Some("examples/cool-colors".to_string()),
        color_height: None,
        font_size: 12,
        border_width: 5,
        border_color: "white".to_string(),
        header_text: None,
        vectorized_text: false,
    };

    match execute_hue_analysis(&cool_args, None) {
        Ok(_) => {
            println!("✅ Generated: examples/cool-colors-gradient.svg");
            println!("✅ Generated: examples/cool-colors-gradient.png");
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    // Example 3: Purple-Red Wraparound Range
    println!("\n📊 Example 3: CSS Colors - Purple-Red Wraparound (300-30°)");
    let wraparound_args = HueArgs {
        collection: "css".to_string(),
        hue_range: Some("[300...30]".to_string()),
        lightness_range: Some("[40...90]".to_string()),
        chroma_range: Some("[20...80]".to_string()),
        grad: false,
        pal: true,
        svg: Some("examples/purple-red-palette.svg".to_string()),
        png: None,
        width: 800,
        no_labels: true, // Clean output without labels
        output_format: Some(OutputFormat::Yaml),
        output_file: None,
        color_height: Some(50),
        font_size: 12,
        border_width: 5,
        border_color: "white".to_string(),
        header_text: None,
        vectorized_text: false,
    };

    match execute_hue_analysis(&wraparound_args, None) {
        Ok(_) => println!("✅ Generated: examples/purple-red-palette.svg (no labels)"),
        Err(e) => println!("❌ Error: {}", e),
    }

    // Example 4: Comprehensive Filtering Demo
    println!("\n📊 Example 4: RAL Design - Complex Filtering");
    let complex_args = HueArgs {
        collection: "rald".to_string(),
        hue_range: Some("[120...180]".to_string()), // Green range
        lightness_range: Some("[50...70]".to_string()), // Medium brightness
        chroma_range: Some("[30...60]".to_string()), // Moderate saturation
        grad: true,
        pal: false,
        svg: Some("examples/green-complex-gradient.svg".to_string()),
        png: None,
        width: 1400,
        no_labels: false,
        output_format: Some(OutputFormat::Yaml),
        output_file: Some("examples/green-complex".to_string()),
        color_height: None,
        font_size: 12,
        border_width: 5,
        border_color: "white".to_string(),
        header_text: None,
        vectorized_text: false,
    };

    match execute_hue_analysis(&complex_args, None) {
        Ok(_) => println!("✅ Generated: examples/green-complex-gradient.svg"),
        Err(e) => println!("❌ Error: {}", e),
    }

    println!("\n🎯 Demo Complete!");
    println!("Generated files:");
    println!("  - examples/warm-colors-palette.svg");
    println!("  - examples/warm-colors.yaml");
    println!("  - examples/cool-colors-gradient.svg");
    println!("  - examples/cool-colors-gradient.png");
    println!("  - examples/cool-colors.toml");
    println!("  - examples/purple-red-palette.svg");
    println!("  - examples/green-complex-gradient.svg");
    println!("  - examples/green-complex.yaml");

    println!("\n💡 Tips:");
    println!("  - Use palette mode (-p) for color picking and precise values");
    println!("  - Use gradient mode (-g) for smooth transitions and color relationships");
    println!("  - Combine filters for targeted color analysis");
    println!("  - Wraparound ranges work great for purple-red spectrums");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_args_validation() {
        let args = HueArgs {
            collection: "css".to_string(),
            hue_range: Some("[0...60]".to_string()),
            lightness_range: Some("[50...80]".to_string()),
            chroma_range: Some("[30...70]".to_string()),
            grad: false,
            pal: true,
            svg: Some("test.svg".to_string()),
            png: None,
            width: 1000,
            no_labels: false,
            output_format: Some(OutputFormat::Yaml),
            output_file: None,
            color_height: Some(40),
            font_size: 12,
            border_width: 5,
            border_color: "white".to_string(),
            header_text: None,
            vectorized_text: false,
        };

        // Validate that our demo arguments are valid
        assert!(args.validate().is_ok());
    }

    #[test]
    fn test_wraparound_range_parsing() {
        use color_rs::cli::Range;

        let range = Range::parse("[300...30]").unwrap();
        assert_eq!(range.min, 300.0);
        assert_eq!(range.max, 30.0);

        // Test wraparound logic
        assert!(range.contains_with_wrap(350.0, 360.0)); // Should be true
        assert!(range.contains_with_wrap(10.0, 360.0)); // Should be true
        assert!(!range.contains_with_wrap(180.0, 360.0)); // Should be false
    }
}
