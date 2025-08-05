//! Color report formatting module
//!
//! This module has been decomposed into focused submodules following functional
//! programming principles and single responsibility design.
//!
//! ## Submodule Organization
//! - `core` - Core formatting functions and data collection
//! - `output` - Output generation and file operations  
//! - `display` - Display formatting and terminal colorization
//! - `utilities` - Color collection matching and enhanced data generation
//!
//! ## Main Functions
//! - `collect_analysis_data()` - Gather structured analysis data
//! - `generate_formatted_output()` - Format data for output
//! - `display_terminal_output()` - Show colorized terminal output
//! - `write_output_file()` - Write data to files
//!
//! ## Example Usage
//! ```ignore
//! use color_rs::color_report_formatting;
//! use color_rs::cli::OutputFormat;
//!
//! // Collect and format analysis data
//! let data = color_report_formatting::collect_analysis_data(&schemes, &input, &name, algorithm, &args)?;
//! let formatted = color_report_formatting::generate_formatted_output(&data, &OutputFormat::Yaml)?;
//! 
//! // Display or save output
//! color_report_formatting::display_terminal_output(&formatted, &OutputFormat::Yaml);
//! color_report_formatting::write_output_file(&data, "output.yaml", &OutputFormat::Yaml)?;
//! ```

pub mod core;
pub mod output;
pub mod display;
pub mod utilities;

// Re-export main functions for backward compatibility
pub use core::{
    collect_analysis_data,
    generate_formatted_output,
    lab_to_hex,
    lab_to_rgb,
    lab_to_hsl_tuple,
    rgb_to_srgb,
    rgb_to_lab,
};

pub use output::{
    write_output_file,
};

pub use display::{
    display_terminal_output,
    colorize_structured_line,
};

pub use utilities::{
    collect_enhanced_color_schemes_data,
};

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Lab;

    #[test]
    fn test_color_conversions() {
        let red_lab = Lab::new(53.24, 80.09, 67.20);
        
        // Test LAB to hex conversion
        let hex = lab_to_hex(red_lab);
        assert!(hex.starts_with('#'));
        assert_eq!(hex.len(), 7);
        
        // Test LAB to RGB conversion
        let rgb = lab_to_rgb(red_lab);
        assert!(rgb.0 > 200); // Should be reddish
        assert!(rgb.1 < 50);  // Low green
        assert!(rgb.2 < 50);  // Low blue
        
        // Test round-trip conversion
        let _srgb = rgb_to_srgb(rgb);
        let lab_back = rgb_to_lab(rgb);
        
        // Should be approximately the same (within tolerance for conversion)
        assert!((lab_back.l - red_lab.l).abs() < 5.0);
    }

    #[test]
    fn test_hsl_conversion() {
        let white_lab = Lab::new(100.0, 0.0, 0.0);
        let hsl = lab_to_hsl_tuple(white_lab);
        
        // White should have high lightness
        assert!(hsl.2 > 0.9); // Lightness close to 1.0
        assert!(hsl.1 < 0.1); // Low saturation
    }

    #[test] 
    fn test_module_re_exports() {
        // Verify all expected functions are accessible
        let lab = Lab::new(50.0, 20.0, -30.0);
        
        // Core functions
        let _hex = lab_to_hex(lab);
        let _rgb = lab_to_rgb(lab);
        let _hsl = lab_to_hsl_tuple(lab);
        let _srgb = rgb_to_srgb((128, 128, 128));
        let _lab_back = rgb_to_lab((128, 128, 128));
    }
}
