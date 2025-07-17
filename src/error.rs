//! Error types for the color-rs library

use std::fmt;

/// Result type alias for color-rs operations
pub type Result<T> = std::result::Result<T, ColorError>;

/// Custom error type for color-rs operations
#[derive(Debug)]
pub enum ColorError {
    /// Invalid color format or value
    InvalidColor(String),
    /// Invalid gradient configuration
    InvalidGradient(String),
    /// Image generation error
    ImageError(String),
    /// File I/O error
    IoError(std::io::Error),
    /// SVG parsing error
    SvgError(String),
    /// Invalid CLI arguments
    InvalidArguments(String),
    /// Color parsing error
    ParseError(String),
    /// Invalid operation error
    InvalidOperation(String),
    /// General error
    General(String),
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorError::InvalidColor(msg) => write!(f, "Invalid color: {}", msg),
            ColorError::InvalidGradient(msg) => write!(f, "Invalid gradient: {}", msg),
            ColorError::ImageError(msg) => write!(f, "Image error: {}", msg),
            ColorError::IoError(err) => write!(f, "I/O error: {}", err),
            ColorError::SvgError(msg) => write!(f, "SVG error: {}", msg),
            ColorError::InvalidArguments(msg) => write!(f, "Invalid arguments: {}", msg),
            ColorError::ParseError(msg) => write!(f, "Color parse error: {}", msg),
            ColorError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            ColorError::General(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for ColorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ColorError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ColorError {
    fn from(err: std::io::Error) -> Self {
        ColorError::IoError(err)
    }
}

impl From<std::num::ParseIntError> for ColorError {
    fn from(err: std::num::ParseIntError) -> Self {
        ColorError::InvalidColor(format!("Parse error: {}", err))
    }
}

impl From<image::ImageError> for ColorError {
    fn from(err: image::ImageError) -> Self {
        ColorError::ImageError(format!("Image processing error: {}", err))
    }
}

// For backward compatibility with anyhow
impl From<anyhow::Error> for ColorError {
    fn from(err: anyhow::Error) -> Self {
        ColorError::General(err.to_string())
    }
}

// For formatting errors
impl From<std::fmt::Error> for ColorError {
    fn from(err: std::fmt::Error) -> Self {
        ColorError::General(format!("Formatting error: {}", err))
    }
}
