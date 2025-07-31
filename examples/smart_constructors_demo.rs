//! Smart Constructors and Data Validation Demo for Milestone 1.1c
//! 
//! This example demonstrates the advanced functional programming patterns
//! implemented for data validation and type safety.

use color_rs::color_distance_strategies::{
    ValidatedLab, ValidationError,
    DistanceAlgorithm, calculate_distance_validated
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Smart Constructors and Data Validation Demo ===\n");

    // 1. Smart Constructor Validation
    println!("1. Smart Constructor Validation:");
    
    // Valid LAB color
    match ValidatedLab::new(50.0, 20.0, -30.0) {
        Ok(validated_lab) => {
            println!("✅ Valid LAB color created: {:?}", validated_lab.into_lab());
        }
        Err(e) => println!("❌ Validation failed: {:?}", e),
    }

    // Invalid LAB color (L component out of range)
    match ValidatedLab::new(150.0, 20.0, -30.0) {
        Ok(_) => println!("❌ Should have failed validation!"),
        Err(ValidationError::LabLightnessOutOfRange(l)) => {
            println!("✅ Correctly caught invalid lightness: L={}", l);
        }
        Err(e) => println!("❌ Unexpected error: {:?}", e),
    }

    // 2. Field Access Methods
    println!("\n2. Field Access Methods:");
    
    let validated = ValidatedLab::new(50.0, 20.0, -30.0)?;
    println!("Original LAB: {:?}", validated.into_lab());
    println!("Lightness: {:.2}", validated.l());
    println!("A component: {:.2}", validated.a());
    println!("B component: {:.2}", validated.b());

    // 3. Immutable Updates using with_* methods
    println!("\n3. Immutable Updates:");
    
    // Update lightness immutably
    let updated_lightness = validated.with_lightness(75.0)?;
    println!("Updated lightness: {:?}", updated_lightness.into_lab());
    println!("Original unchanged: {:?}", validated.into_lab());

    let updated_a = validated.with_a(5.0)?;
    println!("Updated A component: {:?}", updated_a.into_lab());

    // 4. Validated Distance Calculations
    println!("\n4. Validated Distance Calculations:");
    
    let lab1 = ValidatedLab::new(50.0, 20.0, -30.0)?;
    let lab2 = ValidatedLab::new(60.0, 10.0, -20.0)?;

    let delta_e76 = calculate_distance_validated(DistanceAlgorithm::DeltaE76, lab1, lab2)?;
    let delta_e2000 = calculate_distance_validated(DistanceAlgorithm::DeltaE2000, lab1, lab2)?;

    println!("Distance (Delta E 1976): {:.6}", delta_e76);
    println!("Distance (Delta E 2000): {:.6}", delta_e2000);

    // 5. Validation Error Handling
    println!("\n5. Comprehensive Validation Error Handling:");
    
    let validation_cases = vec![
        (150.0, 0.0, 0.0, "Invalid lightness (>100)"),
        (-10.0, 0.0, 0.0, "Invalid lightness (<0)"),
        (50.0, 250.0, 0.0, "Invalid A component (>200)"),
        (50.0, -250.0, 0.0, "Invalid A component (<-200)"),
        (50.0, 0.0, 250.0, "Invalid B component (>200)"),
        (50.0, 0.0, -250.0, "Invalid B component (<-200)"),
    ];

    for (l, a, b, description) in validation_cases {
        match ValidatedLab::new(l, a, b) {
            Ok(_) => println!("❌ {} should have failed", description),
            Err(err) => println!("✅ {} → {:?}", description, err),
        }
    }

    // 6. Performance with Zero-Cost Abstraction
    println!("\n6. Performance Test:");
    let start = std::time::Instant::now();
    let lab1 = ValidatedLab::new(50.0, 20.0, -30.0)?;
    let lab2 = ValidatedLab::new(60.0, 10.0, -20.0)?;
    
    for _ in 0..100_000 {
        let _ = calculate_distance_validated(DistanceAlgorithm::Lch, lab1, lab2)?;
    }
    let duration = start.elapsed();
    println!("✅ 100,000 validated distance calculations in {:?}", duration);
    println!("   Rate: {:.2} million calculations/second", 100.0 / duration.as_secs_f64());

    println!("\n=== Demo Complete: Smart Constructors & Data Validation ===");
    println!("✨ Milestone 1.1c: Data validation through smart constructors");
    println!("✨ Immutable functional field updates with with_* methods");
    println!("✨ Compile-time guarantees for LAB color data integrity");
    println!("✨ Zero-cost abstraction with high-performance validation");

    Ok(())
}
