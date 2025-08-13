//! Range parsing and validation utilities extracted from `cli.rs`.
//! Pure helpers to keep CLI module size manageable (Milestone 4 Phase 4.1).

use crate::error::{ColorError, Result};

/// Range specification for filtering (hue, lightness, chroma)
#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

impl Range {
    /// Parse range from bracket syntax: `[min...max]`.
    ///
    /// # Errors
    /// Returns an error if the format is invalid or numbers cannot be parsed.
    pub fn parse(input: &str) -> Result<Self> {
        if !input.starts_with('[') || !input.ends_with(']') {
            return Err(ColorError::ParseError(
                "Range must be in format [min...max]".to_string(),
            ));
        }
        let inner = &input[1..input.len() - 1];
        let parts: Vec<&str> = inner.split("...").collect();
        if parts.len() != 2 {
            return Err(ColorError::ParseError(
                "Range must contain exactly one '...' separator".to_string(),
            ));
        }
        let min = parts[0]
            .parse::<f64>()
            .map_err(|_| ColorError::ParseError(format!("Invalid minimum value: {}", parts[0])))?;
        let max = parts[1]
            .parse::<f64>()
            .map_err(|_| ColorError::ParseError(format!("Invalid maximum value: {}", parts[1])))?;
        Ok(Self { min, max })
    }

    /// Check if value is within range (wraparound if min>max, for hue).
    #[must_use]
    pub fn contains_with_wrap(&self, value: f64) -> bool {
        if self.min <= self.max {
            value >= self.min && value <= self.max
        } else {
            value >= self.min || value <= self.max
        }
    }

    /// Check if value is within range for linear metrics (no wraparound).
    #[must_use]
    pub fn contains_linear(&self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }
}

/// Validate a hue range string (allow extended wrap bounds) without allocating.
pub fn validate_hue_range(range: &Range) -> Result<()> {
    if range.min < -360.0 || range.max > 720.0 {
        return Err(ColorError::InvalidArguments(
            "Hue range values should be between -360 and 720 degrees".to_string(),
        ));
    }
    Ok(())
}

/// Validate a lightness range (0..=100, min<=max).
pub fn validate_lightness_range(range: &Range) -> Result<()> {
    if range.min < 0.0 || range.max > 100.0 || range.min > range.max {
        return Err(ColorError::InvalidArguments(
            "Lightness range must be 0-100% with min <= max".to_string(),
        ));
    }
    Ok(())
}

/// Validate chroma range (0..=200, min<=max).
pub fn validate_chroma_range(range: &Range) -> Result<()> {
    if range.min < 0.0 || range.max > 200.0 || range.min > range.max {
        return Err(ColorError::InvalidArguments(
            "Chroma range must be 0-200 with min <= max".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Range;

    #[test]
    fn parse_basic_range() {
        let r = Range::parse("[0...10]").unwrap();
        assert_eq!(r.min, 0.0);
        assert_eq!(r.max, 10.0);
    }

    #[test]
    fn wrap_contains() {
        let r = Range { min: 350.0, max: 30.0 };
        assert!(r.contains_with_wrap(355.0));
        assert!(r.contains_with_wrap(10.0));
        assert!(!r.contains_with_wrap(200.0));
    }
}
