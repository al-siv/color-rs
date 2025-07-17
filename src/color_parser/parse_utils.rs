//! Parsing Utilities
//!
//! Common utilities for parsing operations to reduce code duplication

use crate::error::{ColorError, Result};
use std::str::FromStr;

/// Parsing utilities for common operations
pub struct ParseUtils;

impl ParseUtils {
    /// Parse a hex string with consistent error handling
    pub fn parse_hex_component(hex_str: &str) -> Result<u8> {
        u8::from_str_radix(hex_str, 16)
            .map_err(|_| ColorError::InvalidColor("Invalid hex color component".to_string()))
    }

    /// Parse a color component (0-255) with consistent error handling
    pub fn parse_color_component(value: &str) -> Result<u8> {
        let value = value.trim();

        if value.ends_with('%') {
            let percentage_str = &value[..value.len() - 1];
            let percentage = f32::from_str(percentage_str)
                .map_err(|_| ColorError::InvalidColor("Invalid percentage value".to_string()))?;
            Ok(((percentage / 100.0 * 255.0).round().clamp(0.0, 255.0)) as u8)
        } else {
            let int_val = u32::from_str(value)
                .map_err(|_| ColorError::InvalidColor("Invalid color component value".to_string()))?;
            Ok((int_val.clamp(0, 255)) as u8)
        }
    }

    /// Parse a percentage value (0.0-1.0) with consistent error handling
    pub fn parse_percentage(value: &str) -> Result<f32> {
        let value = value.trim();

        if value.ends_with('%') {
            let percentage_str = &value[..value.len() - 1];
            let percentage = f32::from_str(percentage_str)
                .map_err(|_| ColorError::InvalidColor("Invalid percentage value".to_string()))?;
            Ok((percentage / 100.0).clamp(0.0, 1.0))
        } else {
            // Allow float values without % for convenience
            let float_val = f32::from_str(value)
                .map_err(|_| ColorError::InvalidColor("Invalid percentage value".to_string()))?;
            Ok(float_val.clamp(0.0, 1.0))
        }
    }

    /// Parse an alpha value (0.0-1.0) with consistent error handling
    pub fn parse_alpha(value: &str) -> Result<f32> {
        let value = value.trim();
        let alpha = f32::from_str(value)
            .map_err(|_| ColorError::InvalidColor("Invalid alpha value".to_string()))?;
        Ok(alpha.clamp(0.0, 1.0))
    }

    /// Parse a hue value with consistent error handling
    pub fn parse_hue(value: &str) -> Result<f32> {
        f32::from_str(value.trim())
            .map_err(|_| ColorError::InvalidColor("Invalid hue value".to_string()))
    }

    /// Parse RGB component with consistent error handling (for existing color.rs code)
    pub fn parse_rgb_component(value: &str) -> Result<u8> {
        u32::from_str(value.trim())
            .map_err(|_| ColorError::InvalidColor("Invalid RGB value".to_string()))
            .map(|v| (v.clamp(0, 255)) as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_component() {
        assert_eq!(ParseUtils::parse_hex_component("FF").unwrap(), 255);
        assert_eq!(ParseUtils::parse_hex_component("00").unwrap(), 0);
        assert_eq!(ParseUtils::parse_hex_component("7F").unwrap(), 127);
        assert!(ParseUtils::parse_hex_component("GG").is_err());
    }

    #[test]
    fn test_parse_color_component() {
        assert_eq!(ParseUtils::parse_color_component("255").unwrap(), 255);
        assert_eq!(ParseUtils::parse_color_component("100%").unwrap(), 255);
        assert_eq!(ParseUtils::parse_color_component("50%").unwrap(), 128);
        assert!(ParseUtils::parse_color_component("invalid").is_err());
    }

    #[test]
    fn test_parse_percentage() {
        assert_eq!(ParseUtils::parse_percentage("100%").unwrap(), 1.0);
        assert_eq!(ParseUtils::parse_percentage("50%").unwrap(), 0.5);
        assert_eq!(ParseUtils::parse_percentage("0.5").unwrap(), 0.5);
        assert!(ParseUtils::parse_percentage("invalid").is_err());
    }

    #[test]
    fn test_parse_alpha() {
        assert_eq!(ParseUtils::parse_alpha("1.0").unwrap(), 1.0);
        assert_eq!(ParseUtils::parse_alpha("0.5").unwrap(), 0.5);
        assert!(ParseUtils::parse_alpha("invalid").is_err());
    }
}
