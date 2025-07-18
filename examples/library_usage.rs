//! Example demonstrating color-rs library usage
//!
//! This example shows how to use color-rs as a library to:
//! - Parse hex colors
//! - Generate gradients with cubic-bezier easing
//! - Create SVG and PNG outputs programmatically

use color_rs::{
    ColorRs, ColorUtils, Result, cli::GradientArgs, color::ColorProcessor,
    gradient::GradientCalculator,
};

fn main() -> Result<()> {
    println!("Color-rs Library Example");
    println!("========================\n");

    // Example 1: Basic color parsing and conversion
    println!("1. Color Parsing and Conversion:");
    let red_lab = ColorUtils::parse_hex_color("#FF0000")?;
    let blue_lab = ColorUtils::parse_hex_color("#0000FF")?;

    println!(
        "Red in LAB: Lab({:.1}, {:.1}, {:.1})",
        red_lab.l, red_lab.a, red_lab.b
    );
    println!(
        "Blue in LAB: Lab({:.1}, {:.1}, {:.1})",
        blue_lab.l, blue_lab.a, blue_lab.b
    );

    // Example 2: Gradient calculation with easing
    println!("\n2. Gradient Calculation with Cubic-Bezier Easing:");
    let ease_in = 0.25;
    let ease_out = 0.75;

    for i in 0..=10 {
        let t = i as f64 / 10.0;
        let eased_t = GradientCalculator::cubic_bezier_ease(t, ease_in, ease_out);
        let interpolated = ColorUtils::interpolate_lab(red_lab, blue_lab, eased_t);
        let hex = ColorUtils::lab_to_hex(interpolated);

        println!("t={:.1} -> eased_t={:.3} -> color={}", t, eased_t, hex);
    }

    // Example 3: Generate gradient using high-level API
    println!("\n3. High-Level Gradient Generation:");
    let color_rs = ColorRs::new();

    let args = GradientArgs {
        start_color: "FF6B35".to_string(), // Orange
        end_color: "7209B7".to_string(),   // Purple
        start_position: 20,
        end_position: 80,
        ease_in: 0.42,
        ease_out: 0.58,
        svg: true,
        png: false,
        no_legend: false,
        width: 800,
        svg_name: "example-gradient.svg".to_string(),
        png_name: "example-gradient.png".to_string(),
        grad_step: Some(10),
        grad_stops: 5, // Default value
        grad_stops_simple: Some(7),
    };

    // This will generate the gradient and save SVG file
    color_rs.generate_gradient(args)?;

    println!("\n4. Intelligent Stop Calculation:");
    let intelligent_stops =
        GradientCalculator::calculate_intelligent_stops_integer(8, 0.9, 0.1, 0, 100);
    println!("Intelligent stops: {:?}", intelligent_stops);

    let equal_stops: Vec<u8> = (0..8).map(|i| (i * 100 / 7) as u8).collect();
    println!("Equal stops:       {:?}", equal_stops);

    println!("\nExample completed successfully!");
    println!("Check 'example-gradient.svg' for the generated output.");

    Ok(())
}
