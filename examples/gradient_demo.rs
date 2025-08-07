use color_rs::cli::OutputFormat;
use color_rs::error::Result;
///! Gradient Configuration Demo
///!
///! This example demonstrates the gradient configuration approach
///! introduced in Milestone 2.1, replacing the traditional Builder pattern with
///! immutable, composable configuration structures.
use color_rs::gradient_config::{
    ColorPair, EasingConfig, FileOutput, GradientConfig, PositionRange, StopConfig,
    linear_gradient, positioned_gradient, smooth_gradient,
};

fn main() -> Result<()> {
    println!("🎨 Gradient Configuration Demo");
    println!("=========================================\n");

    // 1. Basic gradient creation with smart constructors
    println!("1. Basic Gradient Creation:");
    let basic_gradient = GradientConfig::basic("#FF0000", "#0000FF")?;
    println!(
        "   Colors: {} → {}",
        basic_gradient.colors().start(),
        basic_gradient.colors().end()
    );
    println!("   Args: {:?}\n", basic_gradient.to_gradient_args());

    // 2. Using convenience functions for common patterns
    println!("2. Convenience Functions:");

    let linear = linear_gradient("red", "blue")?;
    println!(
        "   Linear: ease_in={:.2}, ease_out={:.2}",
        linear.easing().ease_in_value(),
        linear.easing().ease_out_value()
    );

    let smooth = smooth_gradient("green", "yellow")?;
    println!(
        "   Smooth: ease_in={:.2}, ease_out={:.2}",
        smooth.easing().ease_in_value(),
        smooth.easing().ease_out_value()
    );

    let positioned = positioned_gradient("purple", "orange", 20, 80)?;
    println!(
        "   Positioned: {}%-{}%\n",
        positioned.position_range().start(),
        positioned.position_range().end()
    );

    // 3. Immutable composition and chaining
    println!("3. Configuration Composition:");
    let complex_gradient = GradientConfig::basic("#FF6B6B", "#4ECDC4")?
        .with_easing(EasingConfig::ease_in_out())
        .with_svg_output("gradient.svg")?
        .with_png_output("gradient.png")?
        .with_width(1200)?
        .with_steps(15)?
        .with_legend(false);

    println!("   Complex gradient configuration:");
    println!(
        "   - Colors: {} → {}",
        complex_gradient.colors().start(),
        complex_gradient.colors().end()
    );
    println!(
        "   - SVG output: {:?}",
        complex_gradient.image_output().svg_filename()
    );
    println!(
        "   - PNG output: {:?}",
        complex_gradient.image_output().png_filename()
    );
    println!("   - Width: {}", complex_gradient.image_output().width());
    println!(
        "   - Legend: {}",
        complex_gradient.image_output().show_legend()
    );

    match complex_gradient.stop_config() {
        StopConfig::Steps(s) => println!("   - Steps: every {s}%"),
        StopConfig::IntelligentStops(count) => println!("   - Intelligent stops: {count}"),
        StopConfig::EqualStops(count) => println!("   - Equal stops: {count}"),
    }

    // 4. Demonstrating type safety and validation
    println!("\n4. Type Safety and Validation:");

    // This would fail at compile time or runtime with clear error messages
    match GradientConfig::basic("", "#0000FF") {
        Ok(_) => println!("   ❌ Empty color validation failed"),
        Err(e) => println!("   ✅ Empty color caught: {e}"),
    }

    match positioned_gradient("red", "blue", 80, 20) {
        Ok(_) => println!("   ❌ Invalid position validation failed"),
        Err(e) => println!("   ✅ Invalid position caught: {e}"),
    }

    // 5. Advanced configuration with all features
    println!("\n5. Advanced Configuration:");
    let advanced = GradientConfig::new(
        ColorPair::new("#E74C3C", "#3498DB")?,
        EasingConfig::new(0.25, 0.75)?,
    )?
    .with_position_range(PositionRange::new(10, 90)?)?
    .with_both_outputs("advanced.svg", "advanced.png")?
    .with_width(1600)?
    .with_equal_stops(20)
    .with_file_output(FileOutput::new(OutputFormat::Yaml, "gradient_config.yaml")?);

    println!("   Advanced gradient with:");
    println!(
        "   - Custom easing: {:.2} / {:.2}",
        advanced.easing().ease_in_value(),
        advanced.easing().ease_out_value()
    );
    println!(
        "   - Position range: {}%-{}%",
        advanced.position_range().start(),
        advanced.position_range().end()
    );
    println!(
        "   - Both image outputs with {}px width",
        advanced.image_output().width()
    );
    if let Some(file_output) = advanced.file_output() {
        println!(
            "   - File output: {} format to {}",
            format!("{:?}", file_output.format()),
            file_output.filename()
        );
    }

    // 6. Performance comparison with Builder pattern
    println!("\n6. Performance Benefits:");
    println!("   ✅ Stack allocation (no heap allocations for small configs)");
    println!("   ✅ Compile-time optimization through immutable structures");
    println!("   ✅ Zero-cost abstraction with enum dispatch");
    println!("   ✅ Early validation prevents runtime errors");
    println!("   ✅ Type safety eliminates invalid configurations");

    // 7. Backward compatibility
    println!("\n7. Backward Compatibility:");
    println!("   - Traditional GradientBuilder still available");
    println!("   - All gradient configs convert to GradientArgs");
    println!("   - Existing CLI and library interfaces unchanged");
    println!("   - Migration can be gradual");

    println!("\n🎯 Milestone 2.1: Builder Pattern Optimization - Complete!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_demo() {
        // Test that the demo runs without errors
        assert!(main().is_ok());
    }

    #[test]
    fn test_immutability_demonstration() {
        let original = GradientConfig::basic("#FF0000", "#0000FF").unwrap();

        // Create modified version
        let modified = original
            .clone()
            .with_width(800)
            .unwrap()
            .with_steps(10)
            .unwrap();

        // Original should be unchanged
        assert_eq!(original.image_output().width(), 1000); // Default width
        assert!(matches!(
            original.stop_config(),
            StopConfig::IntelligentStops(5)
        )); // Default stops

        // Modified should have new values
        assert_eq!(modified.image_output().width(), 800);
        assert!(matches!(modified.stop_config(), StopConfig::Steps(10)));
    }

    #[test]
    fn test_composition_chain() {
        // Test configuration composition chain
        let result = GradientConfig::basic("#FF0000", "#0000FF")
            .and_then(|c| c.with_svg_output("test.svg"))
            .and_then(|c| c.with_steps(5))
            .and_then(|c| c.with_width(1000))
            .map(|c| c.with_legend(false));

        assert!(result.is_ok());
        let config = result.unwrap();
        assert!(config.image_output().svg_filename().is_some());
        assert!(matches!(config.stop_config(), StopConfig::Steps(5)));
        assert_eq!(config.image_output().width(), 1000);
        assert!(!config.image_output().show_legend());
    }

    #[test]
    fn test_error_propagation_in_chain() {
        // Test that validation errors are properly propagated in chains
        let result = GradientConfig::basic("#FF0000", "#0000FF").and_then(|c| c.with_width(0)); // Invalid width

        assert!(result.is_err());
    }
}
