//! Performance Validation Example for Milestone 7.2
//!
//! This example runs comprehensive performance validation tests to ensure
//! functional patterns maintain optimal performance and memory characteristics.

use color_rs::performance_validation::run_all_performance_validations;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Color-rs Performance Validation (Milestone 7.2) ===\n");

    // Run all performance validation tests
    match run_all_performance_validations() {
        Ok(()) => {
            println!("\n=== Performance Validation Summary ===");
            println!("✅ All performance requirements validated successfully");
            println!("✅ Stack allocation optimizations confirmed");
            println!("✅ Functional patterns maintain optimal performance");
            println!("✅ Zero-cost abstractions working as expected");
            println!("✅ Memory usage patterns are efficient and predictable");
            println!("\n🎯 Milestone 7.2 validation complete - ready for production!");
        }
        Err(error) => {
            eprintln!("❌ Performance validation failed: {}", error);
            std::process::exit(1);
        }
    }

    Ok(())
}
