//! Precision utilities for consistent floating point formatting
//!
//! Provides centralized precision control for all floating point values
//! to ensure consistent formatting across console output and file export.

/// Maximum decimal places for floating point values
pub const MAX_DECIMAL_PLACES: usize = 3;

/// Precision utility for standardized floating point formatting
pub struct PrecisionUtils;

impl PrecisionUtils {
    /// Format a floating point value with maximum 3 decimal places
    /// Removes trailing zeros for cleaner output
    #[must_use]
    pub fn format_f64(value: f64) -> String {
        let formatted = format!("{value:.3}");
        // Remove trailing zeros and decimal point if not needed
        formatted
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }

    /// Format a floating point value with exactly the specified decimal places
    #[must_use]
    pub fn format_f64_fixed(value: f64, decimals: usize) -> String {
        let decimals = decimals.min(MAX_DECIMAL_PLACES);
        format!("{value:.decimals$}")
    }

    /// Format percentage with 2 decimal places maximum
    #[must_use]
    pub fn format_percentage(value: f64) -> String {
        Self::format_f64_fixed(value * 100.0, 2)
    }

    /// Format LAB values with standardized precision
    #[must_use]
    pub fn format_lab(l: f64, a: f64, b: f64) -> String {
        format!(
            "lab({}, {}, {})",
            Self::format_f64_fixed(l, 2),
            Self::format_f64_fixed(a, 2),
            Self::format_f64_fixed(b, 2)
        )
    }

    /// Format LCH values with standardized precision
    #[must_use]
    pub fn format_lch(l: f64, c: f64, h: f64) -> String {
        format!(
            "lch({}, {}, {})",
            Self::format_f64_fixed(l, 2),
            Self::format_f64_fixed(c, 2),
            Self::format_f64_fixed(h, 1)
        )
    }

    /// Format `OKLCh` values with standardized precision
    #[must_use]
    pub fn format_oklch(l: f64, c: f64, h: f64) -> String {
        format!(
            "oklch({}, {}, {})",
            Self::format_f64_fixed(l, 3),
            Self::format_f64_fixed(c, 3),
            Self::format_f64_fixed(h, 1)
        )
    }

    /// Format XYZ values with standardized precision
    #[must_use]
    pub fn format_xyz(x: f64, y: f64, z: f64) -> String {
        format!(
            "xyz({}, {}, {})",
            Self::format_f64_fixed(x, 3),
            Self::format_f64_fixed(y, 3),
            Self::format_f64_fixed(z, 3)
        )
    }

    /// Format HSL values with standardized precision
    #[must_use]
    pub fn format_hsl(h: f64, s: f64, l: f64) -> String {
        format!(
            "hsl({}, {}%, {}%)",
            Self::format_f64_fixed(h, 1),
            Self::format_percentage(s),
            Self::format_percentage(l)
        )
    }

    /// Format HSV/HSB values with standardized precision
    #[must_use]
    pub fn format_hsv(h: f64, s: f64, v: f64) -> String {
        format!(
            "hsv({}, {}%, {}%)",
            Self::format_f64_fixed(h, 1),
            Self::format_percentage(s),
            Self::format_percentage(v)
        )
    }

    /// Format CMYK values with standardized precision
    #[must_use]
    pub fn format_cmyk(c: f64, m: f64, y: f64, k: f64) -> String {
        format!(
            "cmyk({}%, {}%, {}%, {}%)",
            Self::format_percentage(c),
            Self::format_percentage(m),
            Self::format_percentage(y),
            Self::format_percentage(k)
        )
    }

    /// Format WCAG relative luminance with 4 decimal places
    #[must_use]
    pub fn format_wcag_relative_luminance(value: f64) -> String {
        format!("{value:.4}")
    }

    /// Serialize f64 values with 3 decimal places max
    pub fn serialize_f64_3<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f64((*value * 1000.0).round() / 1000.0)
    }

    /// Serialize WCAG relative luminance values with 4 decimal places
    pub fn serialize_wcag_luminance<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f64((*value * 10000.0).round() / 10000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_f64() {
        assert_eq!(PrecisionUtils::format_f64(1.234_567_89), "1.235");
        assert_eq!(PrecisionUtils::format_f64(1.0), "1");
        assert_eq!(PrecisionUtils::format_f64(1.10000), "1.1");
        assert_eq!(PrecisionUtils::format_f64(0.0), "0");
    }

    #[test]
    fn test_format_percentage() {
        assert_eq!(PrecisionUtils::format_percentage(0.1234), "12.34");
        assert_eq!(PrecisionUtils::format_percentage(1.0), "100.00");
        assert_eq!(PrecisionUtils::format_percentage(0.0), "0.00");
    }

    #[test]
    fn test_format_lab() {
        assert_eq!(
            PrecisionUtils::format_lab(50.123_456, 25.789_012, -15.345_678),
            "lab(50.12, 25.79, -15.35)"
        );
    }

    #[test]
    fn test_format_lch() {
        assert_eq!(
            PrecisionUtils::format_lch(53.24, 104.552, 40.0),
            "lch(53.24, 104.55, 40.0)"
        );
    }

    #[test]
    fn test_format_wcag_relative_luminance() {
        assert_eq!(
            PrecisionUtils::format_wcag_relative_luminance(0.283_456_789),
            "0.2835"
        );
        assert_eq!(
            PrecisionUtils::format_wcag_relative_luminance(1.0),
            "1.0000"
        );
    }
}
