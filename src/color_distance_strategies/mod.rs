//! Color Distance Calculation Strategies
//!
//! This module provides a functional programming approach to color distance calculations
//! using validated types, smart constructors, and pure algorithms. It replaces the traditional
//! Strategy Pattern with functional enum dispatch for better performance and type safety.
//!
//! # Architecture
//!
//! - **types.rs**: Core types, validation, and smart constructors
//! - **algorithms.rs**: Pure algorithm implementations with functional dispatch  
//! - **validation.rs**: Advanced validation patterns and smart constructors
//!
//! # Key Features
//!
//! - **Type Safety**: `ValidatedLab` ensures all color values are valid at compile time
//! - **Smart Constructors**: Multiple safe ways to create validated colors
//! - **Functional Dispatch**: Enum-based algorithm selection instead of trait objects
//! - **Lens Patterns**: Functional optics for immutable field updates
//! - **Zero-Cost Abstractions**: Compile-time optimizations with runtime safety
//!
//! # Usage Examples
//!
//! ```rust
//! # use color_rs::color_distance_strategies::*;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create validated colors using smart constructors
//! let lab1 = ValidatedLab::new(50.0, 0.0, 0.0)?;
//! let lab2 = SmartConstructors::from_hex("#FF0000")?;
//!
//! // Choose algorithm and calculate distance
//! let algorithm = DistanceAlgorithm::DeltaE2000;
//! let distance = algorithm.calculate_distance(lab1, lab2);
//!
//! // Functional lens operations
//! let brighter = lab1.with_lightness(75.0)?;
//! let lens_update = lab1.modify(|l, a, b| (l + 10.0, a, b))?;
//! # Ok(())
//! # }
//! ```

// Re-export all public types and functions for backward compatibility
pub use types::{
    ALens, BLens, DistanceAlgorithm, LabLens, LightnessLens, ValidatedLab, ValidationError,
};

pub use algorithms::{
    compare_algorithms, filter_fast_algorithms, filter_perceptual_algorithms, recommend_algorithm,
};

pub use validation::{
    BatchValidator, ColorSource, SmartConstructors, ValidationConstraints, combinators,
};

// Module declarations
mod algorithms;
mod types;
mod validation;

// Additional convenience re-exports for common patterns
pub use types::DistanceAlgorithm as Algorithm;
pub use types::ValidatedLab as Lab;
pub use validation::SmartConstructors as Constructors;

/// Prelude module for common imports
///
/// Use `use crate::color_distance_strategies::prelude::*;` to import
/// the most commonly used types and functions.
pub mod prelude {
    pub use super::{DistanceAlgorithm, SmartConstructors, ValidatedLab, ValidationError};

    // Common algorithm variants for convenience
    pub use super::DistanceAlgorithm::{DeltaE76, DeltaE2000, EuclideanLab, Lch};
}

// Legacy compatibility layer - these functions maintain the old API

/// Trait for types that can be converted to `ValidatedLab` for distance calculation
pub trait IntoValidatedLab {
    /// Convert validated lab to distance algorithm trait object
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` if the LAB values cannot be converted to ValidatedLab:
    /// - Invalid LAB color values (L not in [0,100], A/B not in [-128,127])
    /// - Values outside valid color space ranges
    fn into_validated_lab(self) -> Result<ValidatedLab, ValidationError>;
}

impl IntoValidatedLab for [f32; 3] {
    fn into_validated_lab(self) -> Result<ValidatedLab, ValidationError> {
        ValidatedLab::new(self[0], self[1], self[2])
    }
}

impl IntoValidatedLab for palette::Lab {
    fn into_validated_lab(self) -> Result<ValidatedLab, ValidationError> {
        ValidatedLab::new(self.l, self.a, self.b)
    }
}

impl IntoValidatedLab for ValidatedLab {
    fn into_validated_lab(self) -> Result<ValidatedLab, ValidationError> {
        Ok(self)
    }
}

/// Legacy function: Main `calculate_distance` for backward compatibility
///
/// This function works with both [f32; 3] arrays and Lab objects.
/// New code should use the enum dispatch methods directly.
#[must_use]
pub fn calculate_distance<T1, T2>(algorithm: DistanceAlgorithm, lab1: T1, lab2: T2) -> f64
where
    T1: IntoValidatedLab,
    T2: IntoValidatedLab,
{
    match (lab1.into_validated_lab(), lab2.into_validated_lab()) {
        (Ok(l1), Ok(l2)) => algorithm.calculate_distance(l1, l2),
        _ => f64::INFINITY, // Invalid input returns infinity
    }
}

impl DistanceAlgorithm {
    /// Legacy method: `from_str_or_default` for backward compatibility
    ///
    /// Returns `DeltaE2000` as default if parsing fails
    #[must_use]
    pub fn from_str_or_default(s: &str) -> Self {
        s.parse().unwrap_or(Self::DeltaE2000)
    }
}

/// Legacy function: Calculate distance using Delta E 76
///
/// Maintained for backward compatibility. New code should use the enum dispatch.
#[deprecated(
    since = "0.16.0",
    note = "Use DistanceAlgorithm::DeltaE76.calculate_distance() instead"
)]
#[must_use]
pub fn calculate_delta_e_76_legacy(lab1: [f32; 3], lab2: [f32; 3]) -> f64 {
    calculate_distance(DistanceAlgorithm::DeltaE76, lab1, lab2)
}

/// Legacy function: Calculate distance using Delta E 2000
///
/// Maintained for backward compatibility. New code should use the enum dispatch.
#[deprecated(
    since = "0.16.0",
    note = "Use DistanceAlgorithm::DeltaE2000.calculate_distance() instead"
)]
#[must_use]
pub fn calculate_delta_e_2000_legacy(lab1: [f32; 3], lab2: [f32; 3]) -> f64 {
    calculate_distance(DistanceAlgorithm::DeltaE2000, lab1, lab2)
}

/// Legacy function: Calculate Euclidean distance
///
/// Maintained for backward compatibility. New code should use the enum dispatch.
#[deprecated(
    since = "0.16.0",
    note = "Use DistanceAlgorithm::EuclideanLab.calculate_distance() instead"
)]
#[must_use]
pub fn calculate_euclidean_distance_legacy(lab1: [f32; 3], lab2: [f32; 3]) -> f64 {
    calculate_distance(DistanceAlgorithm::EuclideanLab, lab1, lab2)
}

/// Legacy function: Parse algorithm from string
///
/// # Errors
///
/// Returns error string if the algorithm name is not recognized:
/// - Invalid algorithm name (not one of: delta_e_76, delta_e_2000, euclidean_lab, lch)
/// - Case-sensitive matching required
///
/// Maintained for backward compatibility. New code should use `FromStr` trait.
#[deprecated(
    since = "0.16.0",
    note = "Use DistanceAlgorithm::from_str() or parse() instead"
)]
pub fn parse_algorithm_legacy(name: &str) -> Result<DistanceAlgorithm, String> {
    name.parse().map_err(|e: ValidationError| e.to_string())
}

/// Migration helper: Convert old array format to `ValidatedLab`
///
/// # Errors
///
/// Returns `ValidationError` if LAB values are invalid:
/// - L value not in [0,100] range
/// - A or B values not in [-128,127] range
/// - Values outside valid color space ranges
///
/// Utility function to help migrate from old [f32; 3] format to `ValidatedLab`
pub fn array_to_validated_lab(lab: [f32; 3]) -> Result<ValidatedLab, ValidationError> {
    ValidatedLab::new(lab[0], lab[1], lab[2])
}

/// Migration helper: Convert `ValidatedLab` to old array format
///
/// Utility function for interfacing with code that still uses [f32; 3] format
#[must_use]
pub const fn validated_lab_to_array(lab: ValidatedLab) -> [f32; 3] {
    lab.to_array()
}

// Integration tests are included in this module for proximity to implementation
#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_complete_workflow() {
        // Test the complete workflow from input to output
        let lab1 = ValidatedLab::new(50.0, 0.0, 0.0).unwrap();
        let lab2 = SmartConstructors::from_hex("#FF0000").unwrap();

        let algorithm = DistanceAlgorithm::DeltaE2000;
        let distance = algorithm.calculate_distance(lab1, lab2);

        assert!(distance > 0.0);
        assert!(distance.is_finite());
    }

    #[test]
    fn test_algorithm_parsing() {
        // Test all algorithm parsing variants
        assert_eq!(
            DistanceAlgorithm::from_str("deltae2000").unwrap(),
            DistanceAlgorithm::DeltaE2000
        );
        assert_eq!(
            DistanceAlgorithm::from_str("Delta E 2000").unwrap(),
            DistanceAlgorithm::DeltaE2000
        );
        assert_eq!(
            DistanceAlgorithm::from_str("CIEDE2000").unwrap(),
            DistanceAlgorithm::DeltaE2000
        );

        assert!(DistanceAlgorithm::from_str("invalid").is_err());
        assert!(DistanceAlgorithm::from_str("").is_err());
    }

    #[test]
    fn test_smart_constructors() {
        // Test various smart constructor inputs
        let from_lab = ValidatedLab::new(50.0, 0.0, 0.0).unwrap();
        let from_rgb = SmartConstructors::from_rgb(255, 0, 0).unwrap();
        let from_hex = SmartConstructors::from_hex("#FF0000").unwrap();
        let from_hsl = SmartConstructors::from_hsl(0.0, 100.0, 50.0).unwrap();

        // All should be valid
        assert!(from_lab.l() >= 0.0 && from_lab.l() <= 100.0);
        assert!(from_rgb.l() >= 0.0 && from_rgb.l() <= 100.0);
        assert!(from_hex.l() >= 0.0 && from_hex.l() <= 100.0);
        assert!(from_hsl.l() >= 0.0 && from_hsl.l() <= 100.0);
    }

    #[test]
    fn test_lens_operations() {
        // Test functional lens patterns
        let lab = ValidatedLab::new(50.0, 10.0, -5.0).unwrap();
        let lens = ValidatedLab::lens();

        // Test individual lens operations
        let brighter = lens.lightness().set(lab, 75.0).unwrap();
        assert_eq!(brighter.l(), 75.0);
        assert_eq!(brighter.a(), lab.a());
        assert_eq!(brighter.b(), lab.b());

        let modified_a = lens.a_component().modify(lab, |a| a + 5.0).unwrap();
        assert_eq!(modified_a.a(), 15.0);
        assert_eq!(modified_a.l(), lab.l());
        assert_eq!(modified_a.b(), lab.b());
    }

    #[test]
    fn test_batch_operations() {
        // Test batch distance calculations
        let colors = vec![
            ValidatedLab::new(0.0, 0.0, 0.0).unwrap(),
            ValidatedLab::new(50.0, 0.0, 0.0).unwrap(),
            ValidatedLab::new(100.0, 0.0, 0.0).unwrap(),
        ];

        let algorithm = DistanceAlgorithm::DeltaE76;
        let matrix = algorithm.calculate_distance_matrix(&colors);

        // Check matrix properties (upper triangular matrix)
        assert_eq!(matrix.len(), 3);
        assert_eq!(matrix[0][0], 0.0); // Distance to self is 0
        assert!(matrix[0][1] > 0.0); // Distance to different color > 0
        assert_eq!(matrix[1][0], 0.0); // Distance to self is 0 (second color)
        assert!(matrix[0][2] > 0.0); // Distance to different color > 0
    }

    #[test]
    fn test_validation_constraints() {
        // Test advanced validation patterns
        let constraints = ValidationConstraints::srgb_only();

        // Valid sRGB color
        let valid_lab = SmartConstructors::with_constraints(50.0, 0.0, 0.0, &constraints);
        assert!(valid_lab.is_ok());

        // Constraints should be enforced
        let grayscale_constraints = ValidationConstraints::grayscale_only();
        let colorful_lab =
            SmartConstructors::with_constraints(50.0, 50.0, 50.0, &grayscale_constraints);
        assert!(colorful_lab.is_err());
    }

    #[test]
    fn test_algorithm_characteristics() {
        // Test algorithm metadata and characteristics
        assert!(DistanceAlgorithm::DeltaE76.is_fast());
        assert!(!DistanceAlgorithm::DeltaE76.is_perceptually_accurate());

        assert!(!DistanceAlgorithm::DeltaE2000.is_fast());
        assert!(DistanceAlgorithm::DeltaE2000.is_perceptually_accurate());

        // Test recommendations
        let fast_algo = recommend_algorithm(true, false);
        assert_eq!(fast_algo, Some(DistanceAlgorithm::DeltaE76));

        let perceptual_algo = recommend_algorithm(false, true);
        assert_eq!(perceptual_algo, Some(DistanceAlgorithm::DeltaE2000));
    }

    #[test]
    fn test_legacy_compatibility() {
        // Test that legacy functions still work
        #[allow(deprecated)]
        {
            let distance = calculate_delta_e_76_legacy([50.0, 0.0, 0.0], [60.0, 0.0, 0.0]);
            assert!(distance > 0.0);
            assert!(distance.is_finite());
        }

        // Test migration helpers
        let array = [50.0, 10.0, -5.0];
        let lab = array_to_validated_lab(array).unwrap();
        let back_to_array = validated_lab_to_array(lab);
        assert_eq!(array, back_to_array);
    }

    #[test]
    fn test_error_handling() {
        // Test comprehensive error handling
        assert!(ValidatedLab::new(f32::NAN, 0.0, 0.0).is_err());
        assert!(ValidatedLab::new(-10.0, 0.0, 0.0).is_err());
        assert!(ValidatedLab::new(110.0, 0.0, 0.0).is_err());

        assert!(SmartConstructors::from_hex("invalid").is_err());
        assert!(SmartConstructors::from_hsl(400.0, 50.0, 50.0).is_err());

        let error = DistanceAlgorithm::from_str("nonexistent").unwrap_err();
        assert!(matches!(error, ValidationError::UnknownAlgorithm(_)));
    }
}
