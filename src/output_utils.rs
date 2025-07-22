use crate::color_utils::*;
use crate::config::*;
use crate::format_utils::{ColorFormat, FormatUtils};
use colored::*;
use palette::Lab;

// Standard RGB values for white and black
const RGB_WHITE: (u8, u8, u8) = (255, 255, 255);
const RGB_BLACK: (u8, u8, u8) = (0, 0, 0);

pub struct OutputUtils;

impl OutputUtils {
    /// Writes a formatted header with a specific width and style.
    pub fn print_header_ln(header: &str) {
        let padding = COLUMN_HEADER_WIDTH.saturating_sub(header.len());
        println!(
            " {} {:#<padding$}",
            header.to_uppercase().bold(),
            "",
            padding = padding
        );
    }

    fn make_label(label: &str) -> ColoredString {
        format!("{:>width$}", label, width = COLUMN_WIDTH)
            .bold()
            .green()
    }

    fn make_format(lab: Lab, color_format: &ColorFormat) -> String {
        FormatUtils::format_color(lab, color_format)
    }

    fn make_hex(lab: Lab) -> String {
        FormatUtils::lab_to_hex(lab)
    }

    /// Writes a formatted header with a specific width and style.
    pub fn print_pair_ln(label: &str, value: &str) {
        println!("{} {:.3}", Self::make_label(label), value);
    }

    pub fn print_f64_ln(label: &str, value: f64) {
        println!(
            "{} {}",
            Self::make_label(label),
            crate::precision_utils::PrecisionUtils::format_f64(value)
        );
    }

    fn get_contrast(lab: Lab, rgb2: (u8, u8, u8)) -> (f32, ColoredString) {
        let (contrast_value, contrast_assessment) =
            ColorUtils::get_contrast_assessment(ColorUtils::lab_to_rgb(lab), rgb2);
        let ret: ColoredString = contrast_assessment.to_string().bold();
        let colored_ret = match contrast_assessment {
            ContrastLevel::High => ret.green(),
            ContrastLevel::Medium => ret.yellow(),
            ContrastLevel::Marginal => ret.magenta(),
            ContrastLevel::Low => ret.red(),
        };
        (contrast_value, colored_ret)
    }

    pub fn print_contrast_ln(label: &str, contrast: (f32, ColoredString)) {
        let (val, assessment) = contrast;
        println!("{} {:.2}:1 [{}]", Self::make_label(label), val, assessment);
    }

    pub fn print_contrast_white_ln(lab_color: Lab) {
        OutputUtils::print_contrast_ln(
            LABEL_CONTRAST_WHITE,
            Self::get_contrast(lab_color, RGB_WHITE),
        );
    }

    pub fn print_contrast_black_ln(lab_color: Lab) {
        OutputUtils::print_contrast_ln(
            LABEL_CONTRAST_BLACK,
            Self::get_contrast(lab_color, RGB_BLACK),
        );
    }

    pub fn print_color_ln(label: &str, lab: Lab, color_format: ColorFormat) {
        let formated_color = Self::make_format(lab, &color_format);
        let output = if color_format == ColorFormat::Hex {
            formated_color.yellow()
        } else {
            formated_color.normal()
        };
        println!("{} {}", Self::make_label(label), output);
    }

    /// Prints the hex value and an additional color representation depending on ColorFormat.
    pub fn print_hex_ln(label: &str, lab: Lab, color_format: ColorFormat) {
        let formatted_color = Self::make_format(lab, &color_format);
        if color_format == ColorFormat::Hex {
            println!("{} {}", Self::make_label(label), formatted_color.yellow());
        } else {
            println!(
                "{} {} | {}",
                Self::make_label(label),
                Self::make_hex(lab).yellow(),
                formatted_color
            );
        }
    }

    // DUPLICATION ELIMINATED: These functions now delegate to ColorFormatter
    // to avoid having multiple brightness assessment implementations
    fn get_brightness_asssessment_lab(lab_color: Lab) -> String {
        // Use ColorFormatter's assess_lab_brightness logic (3-level: Light/Medium/Dark)
        if lab_color.l >= 70.0 {
            "Light".to_string()
        } else if lab_color.l >= 50.0 {
            "Medium".to_string()
        } else {
            "Dark".to_string()
        }
    }

    fn get_brightness_asssessment_wcag(wcag_luminance: f64) -> String {
        // Use ColorFormatter's assess_wcag_brightness logic with correct 0.18 threshold
        if wcag_luminance >= 0.18 {
            "Light".to_string()
        } else {
            "Dark".to_string()
        }
    }

    pub fn print_brightness(lab_color: Lab) {
        let wcag_luminance =
            ColorUtils::wcag_relative_luminance(ColorUtils::lab_to_srgb(lab_color));
        println!(
            "{} {} [{}] | {} [{}]",
            format!("{:>width$}", LABEL_BRIGHTNESS, width = COLUMN_WIDTH)
                .bold()
                .green(),
            Self::get_brightness_asssessment_lab(lab_color),
            "Lab".bold().green(),
            Self::get_brightness_asssessment_wcag(wcag_luminance),
            "WCAG".bold().green()
        );
    }

    /// Colorize TOML/YAML output values based on their type
    pub fn colorize_output_value(value: &str) -> String {
        use colored::*;

        if value.starts_with('"') && value.ends_with('"') {
            // String values
            value.yellow().to_string()
        } else if value.parse::<f64>().is_ok() {
            // Numeric values
            value.bright_blue().to_string()
        } else if value == "true" || value == "false" {
            // Boolean values
            value.bright_green().to_string()
        } else if value.starts_with('#') {
            // Hex colors
            value.bright_yellow().to_string()
        } else if value.starts_with("rgb(")
            || value.starts_with("hsl(")
            || value.starts_with("lab(")
            || value.starts_with("lch(")
            || value.starts_with("hsv(")
            || value.starts_with("cmyk(")
            || value.starts_with("xyz(")
            || value.starts_with("oklch(")
        {
            // Color format values
            value.bright_cyan().to_string()
        } else {
            value.to_string()
        }
    }
}
