//! Types for the color parser module

/// Represents a parsed color with its original format
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedColor {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
    /// Alpha component (0.0-1.0)
    pub a: f64,
    /// Original format detected
    pub format: ColorFormat,
}

/// Color format detection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorFormat {
    /// Hex format (#rgb or #rrggbb)
    Hex,
    /// RGB format (rgb(r,g,b))
    Rgb,
    /// RGBA format (rgba(r,g,b,a))
    Rgba,
    /// HSL format (hsl(h,s%,l%))
    Hsl,
    /// HSLA format (hsla(h,s%,l%,a))
    Hsla,
    /// Named color (red, blue, etc.)
    Named,
    /// LAB format (lab(L,a,b))
    Lab,
    /// LCH format (lch(L,C,H))
    Lch,
}

impl ParsedColor {
    /// Create a new parsed color
    pub fn new(r: u8, g: u8, b: u8, a: f64, format: ColorFormat) -> Self {
        Self { r, g, b, a, format }
    }

    /// Create from RGB values with full opacity
    pub fn from_rgb(r: u8, g: u8, b: u8, format: ColorFormat) -> Self {
        Self::new(r, g, b, 1.0, format)
    }

    /// Get RGB tuple
    pub fn rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    /// Get RGBA tuple
    pub fn rgba(&self) -> (u8, u8, u8, f64) {
        (self.r, self.g, self.b, self.a)
    }

    /// Check if color has transparency
    pub fn has_alpha(&self) -> bool {
        self.a < 1.0
    }
}
