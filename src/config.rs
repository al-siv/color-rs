//! Configuration constants and default values for color-rs

/// Application metadata
pub const APP_NAME: &str = "Color-rs";
pub const APP_ABOUT: &str = "A CLI tool for color gradient calculations using LAB color space with cubic-bezier easing functions";
pub const APP_AUTHOR: &str = "https://github.com/al-siv";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Image dimensions
/// Height ratio: gradient height = width * HEIGHT_RATIO
pub const HEIGHT_RATIO: f64 = 0.2; // 1/5 of width

/// Default values for CLI arguments
pub const DEFAULT_START_POSITION: &str = "0";
pub const DEFAULT_END_POSITION: &str = "100";
pub const DEFAULT_EASE_IN: &str = "0.65";
pub const DEFAULT_EASE_OUT: &str = "0.35";
pub const DEFAULT_WIDTH: &str = "1000";
pub const DEFAULT_SVG_NAME: &str = "gradient.svg";
pub const DEFAULT_PNG_NAME: &str = "gradient.png";
pub const DEFAULT_GRAD_STEP: &str = "5";

/// Gradient calculation parameters
/// Number of sample points for intelligent stop calculation
pub const INTELLIGENT_STOP_SAMPLE_POINTS: usize = 10000;

/// Numerical constants for calculations
pub const EPSILON: f64 = 1e-7;
pub const MAX_ITERATIONS: usize = 50;

/// Color validation limits
pub const MAX_PERCENTAGE: u8 = 100;
pub const HEX_COLOR_LENGTH: usize = 6;

/// RGB color channel limits
pub const RGB_MAX: u8 = 255;
pub const RGB_MIN: u8 = 0;

/// Color conversion constants
pub const RGB_MAX_F32: f32 = 255.0;
pub const PERCENTAGE_MULTIPLIER: f32 = 100.0;
pub const PERCENTAGE_DIVISOR: f32 = 100.0;

/// Cubic-bezier control point limits
pub const BEZIER_MIN: f64 = 0.0;
pub const BEZIER_MAX: f64 = 1.0;

/// Image generation defaults
pub const DEFAULT_FONT_SIZE_RATIO: f64 = 0.6;
pub const DEFAULT_LEGEND_HEIGHT_RATIO: f64 = 0.2;
pub const DEFAULT_TEXT_Y_RATIO: f64 = 0.75;

/// Font configuration for image generation
pub const FONT_FAMILY: &str = "'Montserrat', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif";

/// Color space conversion precision
pub const COLOR_PRECISION: usize = 1;

/// Default image quality settings
pub const DEFAULT_IMAGE_DPI: f64 = 96.0;
pub const DEFAULT_IMAGE_QUALITY: u8 = 90;

/// Color formatter section headers
pub const HEADER_COLOR_ANALYSIS: &str = "Color Analysis";
pub const HEADER_FORMAT_CONVERSIONS: &str = "Format Conversions";
pub const HEADER_ADDITIONAL_INFO: &str = "Additional Information";
pub const HEADER_COLOR_COLLECTIONS: &str = "Color Collections";

/// Color formatter field labels
pub const LABEL_COLOR: &str = "Color:";
pub const LABEL_RGB: &str = "RGB:";
pub const LABEL_HEX: &str = "Hex:";
pub const LABEL_HSL: &str = "HSL:";
pub const LABEL_LAB: &str = "LAB:";
pub const LABEL_XYZ: &str = "XYZ:";
pub const LABEL_OKLCH: &str = "OKLCH:";
pub const LABEL_GRAYSCALE: &str = "Grayscale:";
pub const LABEL_WCAG_LUMINANCE: &str = "WCAG Relative Luminance:";
pub const LABEL_CONTRAST_WHITE: &str = "Contrast vs White:";
pub const LABEL_CONTRAST_BLACK: &str = "Contrast vs Black:";
pub const LABEL_BRIGHTNESS: &str = "Brightness:";

/// Collection names
pub const COLLECTION_CSS: &str = "CSS Colors";
pub const COLLECTION_RAL_CLASSIC: &str = "RAL Classic";
pub const COLLECTION_RAL_DESIGN: &str = "RAL Design System+";

/// Status indicators
pub const STATUS_PASS: &str = "PASS";
pub const STATUS_WARNING: &str = "WARN";
pub const STATUS_FAIL: &str = "FAIL";

/// Brightness levels
pub const BRIGHTNESS_DARK: &str = "Dark";
pub const BRIGHTNESS_LIGHT: &str = "Light";

/// Default fallback values
pub const DEFAULT_CODE_CSS: &str = "CSS";
pub const DEFAULT_COLOR_UNKNOWN: &str = "Unknown";
pub const NO_MATCHES_MESSAGE: &str = "No close matches";

/// Format strings
pub const FORMAT_RGB: &str = "rgb({}, {}, {})";
pub const FORMAT_HEX: &str = "#{:02x}{:02x}{:02x}";
pub const FORMAT_HEX_UPPER: &str = "#{:02X}{:02X}{:02X}";
pub const FORMAT_HSL: &str = "hsl({:.0}, {:.1}%, {:.1}%)";
pub const FORMAT_LAB: &str = "lab({:.2}, {:.2}, {:.2})";
pub const FORMAT_XYZ: &str = "xyz({:.3}, {:.3}, {:.3})";
pub const FORMAT_OKLCH: &str = "oklch({:.3}, {:.3}, {:.1})";
pub const FORMAT_DELTA_E: &str = "[ΔE {:.2}] ";
pub const FORMAT_CONTRAST_RATIO: &str = "{:.2}:1";
