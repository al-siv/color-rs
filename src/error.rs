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

/// Extension trait for converting `std::io::Result` to `crate::error::Result`
pub trait IoResultExt<T> {
    /// Convert `std::io::Result<T>` to `crate::error::Result<T>`
    ///
    /// This provides a convenient way to convert IO errors to ColorError
    /// without the verbose `.map_err(|e| ColorError::InvalidColor(format!("IO error: {}", e)))`
    ///
    /// # Example
    /// ```rust
    /// use color_rs::error::{IoResultExt, Result};
    /// use std::fs;
    ///
    /// # fn main() -> Result<()> {
    /// // Instead of:
    /// // fs::read_to_string("file.txt").map_err(|e| ColorError::InvalidColor(format!("IO error: {}", e)))?;
    ///
    /// // You can write:
    /// match fs::read_to_string("nonexistent.txt").to_err() {
    ///     Ok(_content) => println!("File found"),
    ///     Err(e) => println!("Expected error: {}", e),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn to_err(self) -> Result<T>;
}

impl<T> IoResultExt<T> for std::io::Result<T> {
    fn to_err(self) -> Result<T> {
        self.map_err(|e| ColorError::InvalidColor(format!("IO error: {}", e)))
    }
}

/// Extension trait for converting `Result<String, FromUtf8Error>` to `crate::error::Result<String>`
pub trait Utf8ResultExt {
    /// Convert `Result<String, FromUtf8Error>` to `crate::error::Result<String>`
    ///
    /// This provides a convenient way to convert UTF-8 conversion errors to ColorError
    ///
    /// # Example
    /// ```rust
    /// use color_rs::error::{Utf8ResultExt, Result};
    ///
    /// # fn main() -> Result<()> {
    /// let bytes = vec![0xc3, 0x28]; // Invalid UTF-8 sequence
    /// // Instead of:
    /// // String::from_utf8(bytes).map_err(|e| ColorError::InvalidColor(format!("UTF-8 error: {}", e)))?;
    ///
    /// // You can write:
    /// match String::from_utf8(bytes).to_err() {
    ///     Ok(_string) => println!("Valid UTF-8"),
    ///     Err(e) => println!("Expected error: {}", e),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn to_err(self) -> Result<String>;
}

impl Utf8ResultExt for std::result::Result<String, std::string::FromUtf8Error> {
    fn to_err(self) -> Result<String> {
        self.map_err(|e| ColorError::InvalidColor(format!("UTF-8 conversion error: {}", e)))
    }
}
