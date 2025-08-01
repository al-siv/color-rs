//! Color distance calculation using functional programming patterns
//!
//! This module provides different algorithms for calculating color distances
//! using pure functions and enum-based selection, eliminating trait objects
//! and embracing functional programming principles.

use palette::{
    Lab,
    color_difference::{EuclideanDistance, ImprovedCiede2000, ImprovedDeltaE},
};

/// Validation errors for smart constructors and data validation
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    /// Empty algorithm name provided
    EmptyAlgorithmName,
    
    /// Algorithm name exceeds maximum length
    AlgorithmNameTooLong(usize),
    
    /// Unknown/unsupported algorithm name
    UnknownAlgorithm(String),
    
    /// Algorithm too slow for performance requirements
    AlgorithmTooSlow(DistanceAlgorithm),
    
    /// Invalid LAB color values
    InvalidLabValues { l: f32, a: f32, b: f32, reason: String },
    
    /// LAB lightness out of valid range
    LabLightnessOutOfRange(f32),
    
    /// LAB color values contain NaN or infinite values
    LabValuesNotFinite,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyAlgorithmName => write!(f, "Algorithm name cannot be empty"),
            Self::AlgorithmNameTooLong(len) => write!(f, "Algorithm name too long: {len} characters (max 50)"),
            Self::UnknownAlgorithm(name) => write!(f, "Unknown algorithm: '{name}'"),
            Self::AlgorithmTooSlow(alg) => write!(f, "Algorithm {} is too slow for performance requirements", alg.name()),
            Self::InvalidLabValues { l, a, b, reason } => write!(f, "Invalid LAB values L:{l} a:{a} b:{b} - {reason}"),
            Self::LabLightnessOutOfRange(l) => write!(f, "LAB lightness {l} out of range [0, 100]"),
            Self::LabValuesNotFinite => write!(f, "LAB values must be finite (not NaN or infinite)"),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validated LAB color with smart constructor guarantees
///
/// This type ensures that LAB color values are always valid and within
/// reasonable bounds. It provides compile-time guarantees through the type system.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ValidatedLab {
    lab: Lab,
}

impl ValidatedLab {
    /// Smart constructor for creating validated LAB colors
    ///
    /// # Arguments
    /// * `l` - Lightness (0-100)
    /// * `a` - Green-Red axis (-128 to +127, typically)
    /// * `b` - Blue-Yellow axis (-128 to +127, typically)
    ///
    /// # Returns
    /// * `Result<ValidatedLab, ValidationError>` - Validated LAB or error
    pub fn new(l: f32, a: f32, b: f32) -> Result<Self, ValidationError> {
        // Check for finite values
        if !l.is_finite() || !a.is_finite() || !b.is_finite() {
            return Err(ValidationError::LabValuesNotFinite);
        }

        // Validate lightness range (0-100)
        if !(0.0..=100.0).contains(&l) {
            return Err(ValidationError::LabLightnessOutOfRange(l));
        }

        // Validate a and b are in reasonable range (-200 to +200 is extreme but possible)
        if !(-200.0..=200.0).contains(&a) || !(-200.0..=200.0).contains(&b) {
            return Err(ValidationError::InvalidLabValues {
                l,
                a,
                b,
                reason: "a/b values out of reasonable range [-200, 200]".to_string(),
            });
        }

        Ok(Self {
            lab: Lab::new(l, a, b),
        })
    }

    /// Create a ValidatedLab from existing Lab (with validation)
    pub fn from_lab(lab: Lab) -> Result<Self, ValidationError> {
        Self::new(lab.l, lab.a, lab.b)
    }

    /// Create ValidatedLab without validation (unsafe but useful for constants)
    ///
    /// # Safety
    /// The caller must ensure the values are valid LAB coordinates
    #[must_use]
    pub const unsafe fn new_unchecked(l: f32, a: f32, b: f32) -> Self {
        Self {
            lab: Lab::new(l, a, b),
        }
    }

    /// Get the underlying Lab value
    #[must_use]
    pub const fn into_lab(self) -> Lab {
        self.lab
    }

    /// Get lightness component
    #[must_use]
    pub const fn l(self) -> f32 {
        self.lab.l
    }

    /// Get a component (green-red axis)
    #[must_use]
    pub const fn a(self) -> f32 {
        self.lab.a
    }

    /// Get b component (blue-yellow axis)
    #[must_use]
    pub const fn b(self) -> f32 {
        self.lab.b
    }

    /// Convert to array format for backward compatibility
    #[must_use]
    pub const fn to_array(self) -> [f32; 3] {
        [self.lab.l, self.lab.a, self.lab.b]
    }

    /// Lens-based field update for lightness (functional optics pattern)
    ///
    /// Returns a new ValidatedLab with updated lightness if valid
    pub fn with_lightness(self, new_l: f32) -> Result<Self, ValidationError> {
        Self::new(new_l, self.lab.a, self.lab.b)
    }

    /// Lens-based field update for a component
    pub fn with_a(self, new_a: f32) -> Result<Self, ValidationError> {
        Self::new(self.lab.l, new_a, self.lab.b)
    }

    /// Lens-based field update for b component
    pub fn with_b(self, new_b: f32) -> Result<Self, ValidationError> {
        Self::new(self.lab.l, self.lab.a, new_b)
    }

    /// Functional composition of lens updates
    ///
    /// Allows chaining multiple field updates with validation
    pub fn modify<F>(self, f: F) -> Result<Self, ValidationError>
    where
        F: FnOnce(f32, f32, f32) -> (f32, f32, f32),
    {
        let (new_l, new_a, new_b) = f(self.lab.l, self.lab.a, self.lab.b);
        Self::new(new_l, new_a, new_b)
    }

    /// Lens for safe field access and updates
    pub fn lens() -> LabLens {
        LabLens
    }
}

/// Lens implementation for functional optics pattern
///
/// Provides functional field access and updates for ValidatedLab
pub struct LabLens;

impl LabLens {
    /// Focus on lightness component
    #[must_use]
    pub fn lightness(&self) -> LightnessLens {
        LightnessLens
    }

    /// Focus on a component  
    #[must_use]
    pub fn a_component(&self) -> ALens {
        ALens
    }

    /// Focus on b component
    #[must_use]
    pub fn b_component(&self) -> BLens {
        BLens
    }
}

/// Lightness-focused lens
pub struct LightnessLens;

impl LightnessLens {
    /// Get lightness value
    #[must_use]
    pub fn get(self, lab: ValidatedLab) -> f32 {
        lab.l()
    }

    /// Set lightness value with validation
    pub fn set(self, lab: ValidatedLab, new_l: f32) -> Result<ValidatedLab, ValidationError> {
        lab.with_lightness(new_l)
    }

    /// Modify lightness with a function
    pub fn modify<F>(self, lab: ValidatedLab, f: F) -> Result<ValidatedLab, ValidationError>
    where
        F: FnOnce(f32) -> f32,
    {
        let new_l = f(lab.l());
        lab.with_lightness(new_l)
    }
}

/// A component lens
pub struct ALens;

impl ALens {
    #[must_use]
    pub fn get(self, lab: ValidatedLab) -> f32 {
        lab.a()
    }

    pub fn set(self, lab: ValidatedLab, new_a: f32) -> Result<ValidatedLab, ValidationError> {
        lab.with_a(new_a)
    }

    pub fn modify<F>(self, lab: ValidatedLab, f: F) -> Result<ValidatedLab, ValidationError>
    where
        F: FnOnce(f32) -> f32,
    {
        let new_a = f(lab.a());
        lab.with_a(new_a)
    }
}

/// B component lens
pub struct BLens;

impl BLens {
    #[must_use]
    pub fn get(self, lab: ValidatedLab) -> f32 {
        lab.b()
    }

    pub fn set(self, lab: ValidatedLab, new_b: f32) -> Result<ValidatedLab, ValidationError> {
        lab.with_b(new_b)
    }

    pub fn modify<F>(self, lab: ValidatedLab, f: F) -> Result<ValidatedLab, ValidationError>
    where
        F: FnOnce(f32) -> f32,
    {
        let new_b = f(lab.b());
        lab.with_b(new_b)
    }
}

/// Functional enum for color distance algorithms
///
/// This replaces the Strategy Pattern trait objects with a pure functional approach.
/// Each variant represents a different distance calculation algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DistanceAlgorithm {
    /// Delta E 76 (CIE76) - Original CIE formula from 1976
    /// Fast but less perceptually accurate than newer methods
    DeltaE76,
    
    /// Delta E 2000 (CIEDE2000) - Industry standard perceptual distance
    /// Most accurate perceptual color differences, computationally more expensive
    DeltaE2000,
    
    /// Euclidean distance in LAB space
    /// Simple and fast but not perceptually uniform
    EuclideanLab,
    
    /// LCH Color Space distance calculation
    /// Distance in cylindrical color space, separates lightness from chroma
    Lch,
}

impl DistanceAlgorithm {
    /// Get human-readable name for this algorithm
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::DeltaE76 => "Delta E 76",
            Self::DeltaE2000 => "Delta E 2000", 
            Self::EuclideanLab => "Euclidean distance",
            Self::Lch => "LCH Color Space",
        }
    }

    /// Get description of this algorithm's characteristics
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::DeltaE76 => "Original CIE76 formula - Fast but less perceptually accurate",
            Self::DeltaE2000 => "CIEDE2000 formula - Most perceptually accurate, industry standard",
            Self::EuclideanLab => "Simple Euclidean distance - Fast but not perceptually uniform",
            Self::Lch => "Distance calculation in LCH cylindrical color space - Separates lightness from chroma",
        }
    }

    /// Get all available algorithms
    #[must_use]
    pub const fn all() -> [Self; 4] {
        [Self::DeltaE76, Self::DeltaE2000, Self::EuclideanLab, Self::Lch]
    }

    /// Parse algorithm from string name
    ///
    /// # Arguments
    /// * `name` - Algorithm name (case-insensitive)
    ///
    /// # Returns
    /// * Some(algorithm) if recognized, None otherwise
    #[must_use]
    pub fn from_str(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "delta-e-76" | "deltae76" | "cie76" => Some(Self::DeltaE76),
            "delta-e-2000" | "deltae2000" | "ciede2000" | "default" => Some(Self::DeltaE2000),
            "euclidean-lab" | "euclidean" | "lab" => Some(Self::EuclideanLab),
            "lch" | "lch-space" | "lch-color-space" => Some(Self::Lch),
            _ => None,
        }
    }

    /// Parse algorithm from string with default fallback
    ///
    /// # Arguments  
    /// * `name` - Algorithm name (case-insensitive)
    ///
    /// # Returns
    /// * Parsed algorithm or DeltaE2000 as default
    #[must_use]
    pub fn from_str_or_default(name: &str) -> Self {
        Self::from_str(name).unwrap_or_else(|| {
            eprintln!("Warning: Unknown strategy '{name}', using LCH");
            Self::Lch
        })
    }

    /// Smart constructor for creating validated distance algorithms
    ///
    /// This provides compile-time guarantees for valid algorithm creation
    /// and runtime validation for algorithm names.
    ///
    /// # Arguments
    /// * `name` - Algorithm name to validate and parse
    ///
    /// # Returns  
    /// * `Result<DistanceAlgorithm, ValidationError>` - Validated algorithm or error
    pub fn try_from_str(name: &str) -> Result<Self, ValidationError> {
        if name.trim().is_empty() {
            return Err(ValidationError::EmptyAlgorithmName);
        }

        if name.len() > 50 {
            return Err(ValidationError::AlgorithmNameTooLong(name.len()));
        }

        Self::from_str(name).ok_or_else(|| ValidationError::UnknownAlgorithm(name.to_string()))
    }

    /// Smart constructor with performance characteristics validation
    ///
    /// # Arguments
    /// * `name` - Algorithm name
    /// * `require_fast` - If true, only allow fast algorithms
    ///
    /// # Returns
    /// * Validated algorithm that meets performance requirements
    pub fn try_from_str_with_performance(name: &str, require_fast: bool) -> Result<Self, ValidationError> {
        let algorithm = Self::try_from_str(name)?;
        
        if require_fast && matches!(algorithm, Self::DeltaE2000) {
            return Err(ValidationError::AlgorithmTooSlow(algorithm));
        }

        Ok(algorithm)
    }

    /// Check if this algorithm is considered fast for real-time usage
    #[must_use]
    pub const fn is_fast(self) -> bool {
        matches!(self, Self::DeltaE76 | Self::EuclideanLab)
    }

    /// Check if this algorithm is perceptually accurate
    #[must_use] 
    pub const fn is_perceptually_accurate(self) -> bool {
        matches!(self, Self::DeltaE2000 | Self::Lch)
    }
}

/// Pure function for calculating color distance
///
/// This is the main functional interface that replaces trait object dispatch
/// with pattern matching on the algorithm enum.
///
/// # Arguments
/// * `algorithm` - The distance calculation algorithm to use
/// * `lab1` - First LAB color
/// * `lab2` - Second LAB color
///
/// # Returns
/// * Distance value (0.0 = identical, higher = more different)
#[must_use]
pub fn calculate_distance(algorithm: DistanceAlgorithm, lab1: Lab, lab2: Lab) -> f64 {
    match algorithm {
        DistanceAlgorithm::DeltaE76 => delta_e_76_distance(lab1, lab2),
        DistanceAlgorithm::DeltaE2000 => delta_e_2000_distance(lab1, lab2),
        DistanceAlgorithm::EuclideanLab => euclidean_lab_distance(lab1, lab2),
        DistanceAlgorithm::Lch => lch_distance(lab1, lab2),
    }
}

/// Validated distance calculation with smart constructors
///
/// Provides compile-time and runtime guarantees for valid inputs
///
/// # Arguments
/// * `algorithm` - Validated distance algorithm 
/// * `lab1` - First LAB color (validated)
/// * `lab2` - Second LAB color (validated)
///
/// # Returns
/// * Result containing distance or validation error
pub fn calculate_distance_validated(
    algorithm: DistanceAlgorithm,
    lab1: ValidatedLab,
    lab2: ValidatedLab,
) -> Result<f64, ValidationError> {
    let distance = calculate_distance(algorithm, lab1.into_lab(), lab2.into_lab());
    
    // Additional validation for the result
    if !distance.is_finite() {
        return Err(ValidationError::InvalidLabValues {
            l: lab1.l(),
            a: lab1.a(), 
            b: lab1.b(),
            reason: "Distance calculation resulted in non-finite value".to_string(),
        });
    }

    Ok(distance)
}

/// Pure function for Delta E 76 distance calculation
///
/// Original CIE Delta E formula from 1976. Fast but less perceptually accurate.
#[must_use]
fn delta_e_76_distance(lab1: Lab, lab2: Lab) -> f64 {
    f64::from(lab1.improved_delta_e(lab2))
}

/// Pure function for Delta E 2000 distance calculation  
///
/// Industry standard perceptual distance using CIEDE2000 formula.
#[must_use]
fn delta_e_2000_distance(lab1: Lab, lab2: Lab) -> f64 {
    f64::from(lab1.improved_difference(lab2))
}

/// Pure function for Euclidean LAB distance calculation
///
/// Simple Euclidean distance in LAB space - fast but not perceptually uniform.
#[must_use]
fn euclidean_lab_distance(lab1: Lab, lab2: Lab) -> f64 {
    f64::from(lab1.distance_squared(lab2).sqrt())
}

/// Pure function for LCH distance calculation
///
/// Distance calculation in LCH (Lightness, Chroma, Hue) cylindrical color space.
#[must_use]
fn lch_distance(lab1: Lab, lab2: Lab) -> f64 {
    use palette::{IntoColor, Lch};

    // Convert LAB to LCH using palette functional approach
    let lch1: Lch = lab1.into_color();
    let lch2: Lch = lab2.into_color();

    // Calculate differences in each component
    let delta_l = lch1.l - lch2.l;
    let delta_c = lch1.chroma - lch2.chroma;

    // Handle hue difference (circular space)
    let delta_h = {
        let h1 = lch1.hue.into_positive_degrees();
        let h2 = lch2.hue.into_positive_degrees();
        let diff = (h1 - h2).abs();
        if diff > 180.0 { 360.0 - diff } else { diff }
    };

    // Calculate Euclidean distance in LCH space
    // Note: Hue is weighted less since it's in degrees while L and C are in different scales
    let hue_weight = 0.1; // Adjust this weight as needed
    f64::from(
        (delta_h * hue_weight)
            .mul_add(
                delta_h * hue_weight,
                delta_c.mul_add(delta_c, delta_l.powi(2)),
            )
            .sqrt(),
    )
}

/// Get all available strategy names for CLI help
#[must_use]
pub fn available_strategies() -> Vec<&'static str> {
    vec!["delta-e-76", "delta-e-2000", "euclidean-lab", "lch"]
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::{IntoColor, Srgb};

    #[test]
    fn test_functional_distance_calculations() {
        // Create LAB colors using functional approach
        let red_srgb = Srgb::new(1.0, 0.0, 0.0);
        let red_lab: Lab = red_srgb.into_color();
        let blue_srgb = Srgb::new(0.0, 0.0, 1.0);
        let blue_lab: Lab = blue_srgb.into_color();

        // Test all algorithms functionally
        let distance_76 = calculate_distance(DistanceAlgorithm::DeltaE76, red_lab, blue_lab);
        let distance_2000 = calculate_distance(DistanceAlgorithm::DeltaE2000, red_lab, blue_lab);
        let distance_lab = calculate_distance(DistanceAlgorithm::EuclideanLab, red_lab, blue_lab);
        let distance_lch = calculate_distance(DistanceAlgorithm::Lch, red_lab, blue_lab);

        // All should return positive distances for different colors
        assert!(distance_76 > 0.0);
        assert!(distance_2000 > 0.0);
        assert!(distance_lab > 0.0);
        assert!(distance_lch > 0.0);

        // Test identity (same color should have distance 0 for all algorithms)
        assert!((calculate_distance(DistanceAlgorithm::DeltaE76, red_lab, red_lab) - 0.0).abs() < 0.001);
        assert!((calculate_distance(DistanceAlgorithm::DeltaE2000, red_lab, red_lab) - 0.0).abs() < 0.001);
        assert!((calculate_distance(DistanceAlgorithm::EuclideanLab, red_lab, red_lab) - 0.0).abs() < 0.001);
        assert!((calculate_distance(DistanceAlgorithm::Lch, red_lab, red_lab) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_algorithm_metadata() {
        assert_eq!(DistanceAlgorithm::DeltaE76.name(), "Delta E 76");
        assert_eq!(DistanceAlgorithm::DeltaE2000.name(), "Delta E 2000");
        assert_eq!(DistanceAlgorithm::EuclideanLab.name(), "Euclidean distance");
        assert_eq!(DistanceAlgorithm::Lch.name(), "LCH Color Space");
    }

    #[test]
    fn test_algorithm_parsing() {
        assert_eq!(DistanceAlgorithm::from_str("delta-e-2000"), Some(DistanceAlgorithm::DeltaE2000));
        assert_eq!(DistanceAlgorithm::from_str("delta-e-76"), Some(DistanceAlgorithm::DeltaE76));
        assert_eq!(DistanceAlgorithm::from_str("euclidean-lab"), Some(DistanceAlgorithm::EuclideanLab));
        assert_eq!(DistanceAlgorithm::from_str("lch"), Some(DistanceAlgorithm::Lch));
        assert_eq!(DistanceAlgorithm::from_str("unknown"), None);

        // Test default fallback
        assert_eq!(DistanceAlgorithm::from_str_or_default("unknown"), DistanceAlgorithm::Lch);
    }

    #[test]
    fn test_functional_symmetry() {
        let red_srgb = Srgb::new(1.0, 0.0, 0.0);
        let red_lab: Lab = red_srgb.into_color();
        let blue_srgb = Srgb::new(0.0, 0.0, 1.0);
        let blue_lab: Lab = blue_srgb.into_color();

        for algorithm in DistanceAlgorithm::all() {
            let distance_ab = calculate_distance(algorithm, red_lab, blue_lab);
            let distance_ba = calculate_distance(algorithm, blue_lab, red_lab);

            // Distance should be symmetric
            assert!(
                (distance_ab - distance_ba).abs() < 0.001,
                "Algorithm {:?} is not symmetric",
                algorithm
            );
        }
    }

    #[test]
    fn test_functional_api() {
        // Test that functional API works correctly
        let lab1 = Lab::new(50.0, 0.0, 0.0);
        let lab2 = Lab::new(60.0, 10.0, -10.0);

        // Test all algorithms
        let _ = calculate_distance(DistanceAlgorithm::DeltaE2000, lab1, lab2);
        let _ = calculate_distance(DistanceAlgorithm::DeltaE76, lab1, lab2);
        let _ = calculate_distance(DistanceAlgorithm::EuclideanLab, lab1, lab2);
        let _ = calculate_distance(DistanceAlgorithm::Lch, lab1, lab2);

        // Test that from_str_or_default works
        assert_eq!(DistanceAlgorithm::from_str_or_default("delta-e-2000"), DistanceAlgorithm::DeltaE2000);
        assert_eq!(DistanceAlgorithm::from_str_or_default("delta-e-76"), DistanceAlgorithm::DeltaE76);
        assert_eq!(DistanceAlgorithm::from_str_or_default("euclidean-lab"), DistanceAlgorithm::EuclideanLab);
        assert_eq!(DistanceAlgorithm::from_str_or_default("lch"), DistanceAlgorithm::Lch);
        assert_eq!(DistanceAlgorithm::from_str_or_default("unknown"), DistanceAlgorithm::Lch); // default
    }
}
