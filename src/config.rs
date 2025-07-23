//! Configuration constants and default values for color-rs

/// Application metadata
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const APP_AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
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

/// Cubic-bezier control point limits
pub const BEZIER_MIN: f64 = 0.0;
pub const BEZIER_MAX: f64 = 1.0;

/// Image generation defaults
pub const DEFAULT_FONT_SIZE_RATIO: f64 = 0.6;
pub const DEFAULT_LEGEND_HEIGHT_RATIO: f64 = 0.2;
pub const DEFAULT_TEXT_Y_RATIO: f64 = 0.75;

/// Font configuration for image generation
pub const FONT_FAMILY: &str = "'Montserrat', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif";

/// Output filtering defaults (v0.14.1+)
pub const DEFAULT_FILTER_EXPRESSION: &str = "[all]";
pub const FILTER_EXPRESSION_MAX_LENGTH: usize = 1000;
pub const MAX_FILTER_RULES_PER_EXPRESSION: usize = 50;
