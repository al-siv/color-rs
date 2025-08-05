//! Configuration constants and default values for color-rs

/// Application metadata
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const APP_AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

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

/// Output filtering defaults (v0.14.1+)
pub const DEFAULT_FILTER_EXPRESSION: &str = "[all]";
pub const FILTER_EXPRESSION_MAX_LENGTH: usize = 1000;
pub const MAX_FILTER_RULES_PER_EXPRESSION: usize = 50;

// ===== MILESTONE 5.1: MATHEMATICAL AND ALGORITHM CONSTANTS =====

/// Bezier curve presets for easing functions
pub mod bezier_presets {
    /// Linear easing: no acceleration or deceleration
    pub const LINEAR: (f64, f64) = (0.0, 1.0);
    
    /// Ease: starts slowly, then speeds up
    pub const EASE: (f64, f64) = (0.25, 1.0);
    
    /// Ease-in: starts slowly
    pub const EASE_IN: (f64, f64) = (0.42, 1.0);
    
    /// Ease-out: ends slowly
    pub const EASE_OUT: (f64, f64) = (0.0, 0.58);
    
    /// Ease-in-out: starts and ends slowly
    pub const EASE_IN_OUT: (f64, f64) = (0.42, 0.58);
}

/// Mathematical constants for calculations
pub mod math_constants {
    /// Multiplier for percentage conversion (0.0-1.0 → 0-100)
    pub const PERCENTAGE_MULTIPLIER: f64 = 100.0;
    
    /// RGB color component maximum value
    pub const RGB_MAX_VALUE: f32 = 255.0;
    
    /// Precision enhancement factor for mathematical operations
    pub const PRECISION_MULTIPLIER: f64 = 100.0;
    
    /// Tolerance factor for floating point comparisons
    pub const FLOAT_TOLERANCE_FACTOR: f64 = 10.0;
}

/// UI and display constants with minimum values
pub mod display_constants {
    /// Height ratio: gradient height = width * HEIGHT_RATIO
    pub const HEIGHT_RATIO: f64 = 0.2; // 1/5 of width
    
    /// Image generation ratio defaults
    pub const DEFAULT_FONT_SIZE_RATIO: f64 = 0.6;
    pub const DEFAULT_LEGEND_HEIGHT_RATIO: f64 = 0.2;
    pub const DEFAULT_TEXT_Y_RATIO: f64 = 0.75;
    
    /// Minimum legend height in pixels
    pub const MIN_LEGEND_HEIGHT: f64 = 20.0;
    
    /// Minimum font size in pixels
    pub const MIN_FONT_SIZE: f64 = 10.0;
    
    /// LAB luminance range (0-100)
    pub const LAB_LUMINANCE_MAX: f64 = 100.0;
    pub const LAB_LUMINANCE_MIN: f64 = 0.0;
    
    /// Font configuration for image generation
    pub const FONT_FAMILY: &str = "'Montserrat', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif";
}

/// Algorithm-specific constants
pub mod algorithm_constants {
    /// High luminance value for binary search algorithms
    pub const BINARY_SEARCH_HIGH_LUMINANCE: f32 = 100.0;
    
    /// Percentage display precision (decimal places)
    pub const PERCENTAGE_PRECISION: usize = 2;
    
    /// Luminance convergence tolerance for binary search
    pub const LUMINANCE_TOLERANCE: f64 = 0.001;
    
    /// Geometric position calculation tolerance
    pub const GEOMETRIC_TOLERANCE: f64 = 0.001;
    
    /// Default gradient distance tolerance
    pub const GRADIENT_DISTANCE_TOLERANCE: f64 = 0.01;
    
    /// Floating point comparison tolerance for color schemes
    pub const COLOR_COMPARISON_TOLERANCE: f64 = 0.1;
}
