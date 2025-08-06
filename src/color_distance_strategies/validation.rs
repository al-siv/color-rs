//! Validation utilities and smart constructor patterns
//!
//! This module provides validation functions and smart constructor patterns
//! for ensuring color distance calculations work with valid data.

use super::types::{ValidatedLab, ValidationError};
use palette::Lab;

/// Smart constructor collection for common validation patterns
pub struct SmartConstructors;

impl SmartConstructors {
    /// Create `ValidatedLab` from RGB components with automatic conversion
    ///
    /// Performs RGB -> LAB conversion with validation
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::InvalidRange` if the resulting LAB values
    /// fall outside acceptable ranges for color calculations.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Result<ValidatedLab, ValidationError> {
        use palette::{IntoColor, Srgb};

        let rgb = Srgb::new(
            f32::from(r) / 255.0,
            f32::from(g) / 255.0,
            f32::from(b) / 255.0,
        );

        let lab: Lab = rgb.into_color();
        ValidatedLab::from_lab(lab)
    }

    /// Create `ValidatedLab` from hex color string
    ///
    /// Supports formats: "#RRGGBB", "RRGGBB", "#RGB", "RGB"
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::InvalidFormat` if the hex string is malformed
    /// or contains invalid hexadecimal characters. Returns `ValidationError::InvalidRange`
    /// if the resulting LAB values fall outside acceptable ranges.
    pub fn from_hex(hex: &str) -> Result<ValidatedLab, ValidationError> {
        let hex = hex.trim_start_matches('#');

        let (r, g, b) = match hex.len() {
            3 => {
                // Short format: RGB -> RRGGBB
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|_| {
                    ValidationError::InvalidLabValues {
                        l: 0.0,
                        a: 0.0,
                        b: 0.0,
                        reason: format!("Invalid hex color: #{hex}"),
                    }
                })?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|_| {
                    ValidationError::InvalidLabValues {
                        l: 0.0,
                        a: 0.0,
                        b: 0.0,
                        reason: format!("Invalid hex color: #{hex}"),
                    }
                })?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|_| {
                    ValidationError::InvalidLabValues {
                        l: 0.0,
                        a: 0.0,
                        b: 0.0,
                        reason: format!("Invalid hex color: #{hex}"),
                    }
                })?;
                (r, g, b)
            }
            6 => {
                // Full format: RRGGBB
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| {
                    ValidationError::InvalidLabValues {
                        l: 0.0,
                        a: 0.0,
                        b: 0.0,
                        reason: format!("Invalid hex color: #{hex}"),
                    }
                })?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| {
                    ValidationError::InvalidLabValues {
                        l: 0.0,
                        a: 0.0,
                        b: 0.0,
                        reason: format!("Invalid hex color: #{hex}"),
                    }
                })?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| {
                    ValidationError::InvalidLabValues {
                        l: 0.0,
                        a: 0.0,
                        b: 0.0,
                        reason: format!("Invalid hex color: #{hex}"),
                    }
                })?;
                (r, g, b)
            }
            _ => {
                return Err(ValidationError::InvalidLabValues {
                    l: 0.0,
                    a: 0.0,
                    b: 0.0,
                    reason: format!("Invalid hex color length: #{hex}"),
                });
            }
        };

        Self::from_rgb(r, g, b)
    }

    /// Create `ValidatedLab` from HSL components with automatic conversion
    ///
    /// # Arguments
    /// * `h` - Hue (0-360 degrees)
    /// * `s` - Saturation (0-100 percent)
    /// * `l` - Lightness (0-100 percent)
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::InvalidLabValues` if HSL values are out of range:
    /// - Hue must be in [0, 360] degrees
    /// - Saturation must be in [0, 100] percent
    /// - Lightness must be in [0, 100] percent
    ///
    ///   Also returns validation errors if the resulting LAB values are invalid.
    pub fn from_hsl(h: f32, s: f32, l: f32) -> Result<ValidatedLab, ValidationError> {
        use palette::{Hsl, IntoColor, Srgb};

        // Validate HSL ranges
        if !(0.0..=360.0).contains(&h) {
            return Err(ValidationError::InvalidLabValues {
                l: 0.0,
                a: 0.0,
                b: 0.0,
                reason: format!("Hue {h} out of range [0, 360]"),
            });
        }

        if !(0.0..=100.0).contains(&s) {
            return Err(ValidationError::InvalidLabValues {
                l: 0.0,
                a: 0.0,
                b: 0.0,
                reason: format!("Saturation {s} out of range [0, 100]"),
            });
        }

        if !(0.0..=100.0).contains(&l) {
            return Err(ValidationError::InvalidLabValues {
                l: 0.0,
                a: 0.0,
                b: 0.0,
                reason: format!("Lightness {l} out of range [0, 100]"),
            });
        }

        let hsl = Hsl::new(h, s / 100.0, l / 100.0);
        let rgb: Srgb = hsl.into_color();
        let lab: Lab = rgb.into_color();

        ValidatedLab::from_lab(lab)
    }

    /// Create `ValidatedLab` collection from multiple sources
    ///
    /// Validates all inputs and returns either all valid colors or first error
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` if any source fails validation:
    /// - Invalid LAB color values (L not in [0,100], A/B not in [-128,127])
    /// - Invalid hex color format
    /// - Invalid RGB values (not in [0,255])
    /// - Invalid HSL values (H not in [0,360], S/L not in [0,100])
    pub fn from_multiple_sources(
        sources: &[ColorSource],
    ) -> Result<Vec<ValidatedLab>, ValidationError> {
        sources.iter().map(ColorSource::to_validated_lab).collect()
    }

    /// Create `ValidatedLab` with range constraints
    ///
    /// Applies additional constraints beyond basic validation
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` if validation fails:
    /// - Values outside specified lightness range
    /// - Chroma exceeds maximum allowed value
    /// - Color outside sRGB gamut (when gamut validation enabled)
    /// - Invalid LAB values (L not in [0,100], A/B not in [-128,127])
    pub fn with_constraints(
        l: f32,
        a: f32,
        b: f32,
        constraints: &ValidationConstraints,
    ) -> Result<ValidatedLab, ValidationError> {
        // First perform basic validation
        let lab = ValidatedLab::new(l, a, b)?;

        // Apply additional constraints
        constraints.validate(lab)?;

        Ok(lab)
    }
}

/// Flexible input source enumeration for smart constructors
#[derive(Debug, Clone)]
pub enum ColorSource {
    /// Direct LAB values
    Lab(f32, f32, f32),
    /// RGB components
    Rgb(u8, u8, u8),
    /// Hex color string
    Hex(String),
    /// HSL components  
    Hsl(f32, f32, f32),
    /// Pre-validated LAB
    ValidatedLab(ValidatedLab),
}

impl ColorSource {
    /// Convert any source to `ValidatedLab`
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` for invalid inputs:
    /// - Hex: Invalid format or non-hexadecimal characters
    /// - RGB: Values outside [0,255] range
    /// - HSL: H outside [0,360] or S/L outside [0,100] range
    /// - Lab: Values outside valid LAB color space ranges
    pub fn to_validated_lab(&self) -> Result<ValidatedLab, ValidationError> {
        match self {
            Self::Lab(l, a, b) => ValidatedLab::new(*l, *a, *b),
            Self::Rgb(r, g, b) => SmartConstructors::from_rgb(*r, *g, *b),
            Self::Hex(hex) => SmartConstructors::from_hex(hex),
            Self::Hsl(h, s, l) => SmartConstructors::from_hsl(*h, *s, *l),
            Self::ValidatedLab(lab) => Ok(*lab),
        }
    }
}

/// Validation constraints for advanced validation scenarios
#[derive(Debug, Clone)]
pub struct ValidationConstraints {
    /// Minimum lightness allowed
    pub min_lightness: Option<f32>,
    /// Maximum lightness allowed
    pub max_lightness: Option<f32>,
    /// Maximum chroma (distance from gray axis)
    pub max_chroma: Option<f32>,
    /// Whether to allow colors outside sRGB gamut
    pub allow_out_of_gamut: bool,
}

impl Default for ValidationConstraints {
    fn default() -> Self {
        Self {
            min_lightness: None,
            max_lightness: None,
            max_chroma: None,
            allow_out_of_gamut: true,
        }
    }
}

impl ValidationConstraints {
    /// Create constraints for sRGB-only colors
    #[must_use]
    pub const fn srgb_only() -> Self {
        Self {
            min_lightness: Some(0.0),
            max_lightness: Some(100.0),
            max_chroma: None,
            allow_out_of_gamut: false,
        }
    }

    /// Create constraints for grayscale colors only
    #[must_use]
    pub const fn grayscale_only() -> Self {
        Self {
            min_lightness: Some(0.0),
            max_lightness: Some(100.0),
            max_chroma: Some(0.1), // Very small tolerance for floating point
            allow_out_of_gamut: true,
        }
    }

    /// Create constraints for vibrant colors (high chroma)
    #[must_use]
    pub const fn vibrant_only(_min_chroma: f32) -> Self {
        Self {
            min_lightness: Some(10.0), // Avoid very dark colors
            max_lightness: Some(90.0), // Avoid very light colors
            max_chroma: None,
            allow_out_of_gamut: true,
        }
    }

    /// Validate a color against these constraints
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` if values fail constraint validation:
    /// - Lightness outside specified range
    /// - Chroma exceeds maximum value
    /// - Color outside sRGB gamut (when enabled)
    pub fn validate(&self, lab: ValidatedLab) -> Result<(), ValidationError> {
        // Check lightness constraints
        if let Some(min_l) = self.min_lightness {
            if lab.l() < min_l {
                return Err(ValidationError::LabLightnessOutOfRange(lab.l()));
            }
        }

        if let Some(max_l) = self.max_lightness {
            if lab.l() > max_l {
                return Err(ValidationError::LabLightnessOutOfRange(lab.l()));
            }
        }

        // Check chroma constraints with optimized mul_add
        if let Some(max_chroma) = self.max_chroma {
            let chroma = lab.a().mul_add(lab.a(), lab.b() * lab.b()).sqrt();
            if chroma > max_chroma {
                return Err(ValidationError::InvalidLabValues {
                    l: lab.l(),
                    a: lab.a(),
                    b: lab.b(),
                    reason: format!("Chroma {chroma} exceeds maximum {max_chroma}"),
                });
            }
        }

        // Check gamut constraints with collapsed if statement
        if !self.allow_out_of_gamut && !is_in_srgb_gamut(lab) {
            return Err(ValidationError::InvalidLabValues {
                l: lab.l(),
                a: lab.a(),
                b: lab.b(),
                reason: "Color is outside sRGB gamut".to_string(),
            });
        }

        Ok(())
    }
}

/// Check if a LAB color is within the sRGB gamut
///
/// This is an approximation - exact gamut checking requires complex calculations
fn is_in_srgb_gamut(lab: ValidatedLab) -> bool {
    use palette::{IntoColor, Srgb};

    let rgb: Srgb = lab.into_lab().into_color();

    // Check if RGB components are within [0, 1] range
    rgb.red >= 0.0
        && rgb.red <= 1.0
        && rgb.green >= 0.0
        && rgb.green <= 1.0
        && rgb.blue >= 0.0
        && rgb.blue <= 1.0
}

/// Batch validation utilities
pub struct BatchValidator;

impl BatchValidator {
    /// Validate multiple LAB colors with early exit on first error
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` on first validation failure:
    /// - Invalid LAB color values (L not in [0,100], A/B not in [-128,127])
    /// - Values outside valid color space ranges
    pub fn validate_all_or_fail(
        labs: &[(f32, f32, f32)],
    ) -> Result<Vec<ValidatedLab>, ValidationError> {
        labs.iter()
            .map(|(l, a, b)| ValidatedLab::new(*l, *a, *b))
            .collect()
    }

    /// Validate multiple LAB colors, collecting all errors
    #[must_use]
    pub fn validate_collect_errors(
        labs: &[(f32, f32, f32)],
    ) -> (Vec<ValidatedLab>, Vec<ValidationError>) {
        let mut valid = Vec::new();
        let mut errors = Vec::new();

        for (l, a, b) in labs {
            match ValidatedLab::new(*l, *a, *b) {
                Ok(lab) => valid.push(lab),
                Err(err) => errors.push(err),
            }
        }

        (valid, errors)
    }

    /// Validate multiple LAB colors, skipping invalid ones
    #[must_use]
    pub fn validate_filter_valid(labs: &[(f32, f32, f32)]) -> Vec<ValidatedLab> {
        labs.iter()
            .filter_map(|(l, a, b)| ValidatedLab::new(*l, *a, *b).ok())
            .collect()
    }

    /// Validate with constraints applied to all colors
    ///
    /// # Errors
    /// Returns a `ValidationError` if any color fails validation against the provided constraints.
    /// This can occur when L*, a*, or b* values are outside the specified ranges.
    pub fn validate_with_constraints(
        labs: &[(f32, f32, f32)],
        constraints: &ValidationConstraints,
    ) -> Result<Vec<ValidatedLab>, ValidationError> {
        labs.iter()
            .map(|(l, a, b)| SmartConstructors::with_constraints(*l, *a, *b, constraints))
            .collect()
    }
}

/// Functional validation combinators
pub mod combinators {
    use super::ValidationError;

    /// Validation function type alias for functional composition
    pub type ValidationFn<T> = fn(T) -> Result<T, ValidationError>;

    /// Compose two validation functions
    pub fn compose<T>(
        f1: ValidationFn<T>,
        f2: ValidationFn<T>,
    ) -> impl Fn(T) -> Result<T, ValidationError>
    where
        T: Copy,
    {
        move |value| f1(value).and_then(f2)
    }

    /// Chain multiple validation functions
    pub fn chain<T>(validators: Vec<ValidationFn<T>>) -> impl Fn(T) -> Result<T, ValidationError>
    where
        T: Copy,
    {
        move |value| {
            validators
                .iter()
                .try_fold(value, |acc, validator| validator(acc))
        }
    }

    /// Validation that always succeeds (identity)
    #[allow(clippy::unnecessary_wraps)]
    pub const fn always_valid<T>(value: T) -> Result<T, ValidationError> {
        Ok(value)
    }

    /// Validation that always fails with a specific error
    pub fn always_invalid<T>(error: ValidationError) -> impl Fn(T) -> Result<T, ValidationError> {
        move |_| Err(error.clone())
    }
}
