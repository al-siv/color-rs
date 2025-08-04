//! Example demonstrating color-rs library usage
//!
//! This example shows how to use color-rs as a library to:
//! - Parse hex colors
//! - Generate gradients with cubic-bezier easing
//! - Create SVG and PNG outputs programmatically

use color_rs::{ColorRs, Result, cli::GradientArgs, gradient::GradientCalculator};
use palette::{IntoColor, Lab, Srgb, Mix};

fn main() -> Result<()> {
    println!("Color-rs Library Example");
    println!("========================\n");

    // Example 1: Basic color parsing and conversion
    println!("1. Color Parsing and Conversion:");
    
    // Parse hex colors using functional approach
    let red_srgb = Srgb::new(1.0, 0.0, 0.0);
    let red_lab: Lab = red_srgb.into_color();
    let blue_srgb = Srgb::new(0.0, 0.0, 1.0); 
    let blue_lab: Lab = blue_srgb.into_color();

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
        let interpolated = red_lab.mix(blue_lab, eased_t as f32);
        
        // Convert LAB to hex
        let srgb: Srgb = interpolated.into_color();
        let hex = format!(
            "#{:02X}{:02X}{:02X}",
            (srgb.red * 255.0).round().clamp(0.0, 255.0) as u8,
            (srgb.green * 255.0).round().clamp(0.0, 255.0) as u8,
            (srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8,
        );

        println!("t={t:.1} -> eased_t={eased_t:.3} -> color={hex}");
    }

    // Example 3: Generate gradient using high-level API
    println!("\n3. High-Level Gradient Generation:");
    let color_rs = ColorRs::new();

    let args = GradientArgs {
        start_color: "#FF6B35".to_string(), // Orange
        end_color: "#7209B7".to_string(),   // Purple
        start_position: 20,
        end_position: 80,
        ease_in: 0.42,
        ease_out: 0.58,
        svg: Some("example-gradient.svg".to_string()),
        png: None,
        no_legend: false,
        width: 800,
        step: None,
        stops: 5,
        stops_simple: true,
        output_format: None,
        output_file: None,
        func_filter: None,
    };

    // This will generate the gradient and save SVG file
    color_rs.generate_gradient(args)?;

    println!("\n4. Intelligent Stop Calculation:");
    let calculator = GradientCalculator::with_intelligent_stops(0.9, 0.1);
    let intelligent_stops = calculator.calculate_stops_integer(8, 0, 100);
    println!("Intelligent stops: {intelligent_stops:?}");

    let equal_calculator = GradientCalculator::with_equal_spacing();
    let equal_stops = equal_calculator.calculate_stops_integer(8, 0, 100);
    println!("Equal stops:       {equal_stops:?}");

    println!("\nExample completed successfully!");
    println!("Check 'example-gradient.svg' for the generated output.");

    Ok(())
}
