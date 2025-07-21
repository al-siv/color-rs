//! Configuration constants and default values for color-rs

/// Application metadata
pub const APP_NAME: &str = "Color-rs";
pub const APP_ABOUT: &str = "A CLI tool for color gradient calculations using LAB color space with cubic-bezier easing functions";
pub const APP_AUTHOR: &str = "https://github.com/al-siv";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Formatting constant: width for formatted columns in the output
pub const COLUMN_WIDTH: usize = 27;
pub const COLUMN_MULTIPLIER: usize = 3; // For double-width columns
pub const COLUMN_HEADER_WIDTH: usize = COLUMN_WIDTH * COLUMN_MULTIPLIER; // For header columns

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
pub const RGB_MAX_F32: f64 = 255.0;
pub const PERCENTAGE_MULTIPLIER: f64 = 100.0;
pub const PERCENTAGE_DIVISOR: f64 = 100.0;

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
pub const HEADER_COLOR_ANALYSIS: &str = "# Color Analysis ";
pub const HEADER_FORMAT_CONVERSIONS: &str = "# Format Conversions ";
pub const HEADER_ADDITIONAL_INFO: &str = "# Additional Information ";
pub const HEADER_COLOR_COLLECTIONS: &str = "# Color Collections ";
pub const HEADER_COLOR_SCHEMES: &str = "# Color Schemes ";

/// Input field labels
pub const LABEL_INPUT_COLOR: &str = "Input Color:";
pub const LABEL_BASE_COLOR: &str = "Base Color:";

/// Color formatter field labels
pub const LABEL_RGB: &str = "RGB:";
pub const LABEL_HEX: &str = "HEX:";
pub const LABEL_HSL: &str = "HSL:";
pub const LABEL_HSB: &str = "HSB (HSV):";
pub const LABEL_LCH: &str = "LCH:";
pub const LABEL_LAB: &str = "LAB:";
pub const LABEL_XYZ: &str = "XYZ:";
pub const LABEL_OKLCH: &str = "OKLCH:";
pub const LABEL_CMYK: &str = "CMYK:";
pub const LABEL_GRAYSCALE_LAB: &str = "Grayscale (Lab):";
pub const LABEL_GRAYSCALE_LCH_0: &str = "Grayscale (LCH C ~0%):";
pub const LABEL_GRAYSCALE_LCH_2: &str = "Grayscale (LCH C ~2%):";
pub const LABEL_GRAYSCALE_LCH_4: &str = "Grayscale (LCH C ~4%):";
pub const LABEL_GRAYSCALE_LCH_6: &str = "Grayscale (LCH C ~6%):";
pub const LABEL_WCAG_LUMINANCE: &str = "Relative Luminance:";
pub const LABEL_WCAG_COMPARTIBLE: &str = "WCAG 2.1 Compatible";
pub const LABEL_CONTRAST_WHITE: &str = "Contrast vs White:";
pub const LABEL_CONTRAST_BLACK: &str = "Contrast vs Black:";
pub const LABEL_BRIGHTNESS: &str = "Brightness:";

/// Collection names
pub const COLLECTION_CSS: &str = "## CSS Colors";
pub const COLLECTION_RAL_CLASSIC: &str = "## RAL Classic";
pub const COLLECTION_RAL_DESIGN: &str = "## RAL Design System+";

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

/// Color schemes names
pub const HEADER_SCHEMA_COMPLIMENTARY: &str = "## Complementary Color";
pub const HEADER_SCHEMA_SPLIT_COMPLIMENTARY: &str = "## Split-Complementary Colors";
pub const HEADER_SCHEMA_TRIADIC: &str = "## Triadic Colors";
pub const HEADER_SCHEMA_TETRADIC: &str = "## Tetradic Colors";

/// Color schemes color names
pub const LABEL_SCHEMA_COMPLIMENTARY_COLOR: &str = "Complementary";
pub const LABEL_SCHEMA_SPLIT_COMPLIMENTARY_COLOR: &str = "Split";
pub const LABEL_SCHEMA_TRIADIC_COLOR: &str = "Triadic";
pub const LABEL_SCHEMA_OTHER_COLOR: &str = "Other Colors";

/// Section headers for color analysis and gradient values
pub const HEADER_BASE_COLORS: &str = " # Base colors";
pub const HEADER_GRADIENT_VALUES: &str = " # Gradient Values";

/// Labels for gradient start and end colors
pub const LABEL_GRADIENT_START_COLOR: &str = "Start Color";
pub const LABEL_GRADIENT_END_COLOR: &str = "End Color";

/// RGB color constants for white and black
pub const RGB_WHITE: (u8, u8, u8) = (255, 255, 255);
pub const RGB_BLACK: (u8, u8, u8) = (0, 0, 0);
pub const RGB_GRAY: (u8, u8, u8) = (128, 128, 128);
