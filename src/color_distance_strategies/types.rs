//! Core types and data structures for color distance calculations
//!
//! This module defines the fundamental types used throughout the color distance
//! system, including validated LAB colors and algorithm enumeration.

use palette::Lab;

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
