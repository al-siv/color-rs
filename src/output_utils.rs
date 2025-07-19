use crate::color_utils::*;
use crate::config::*;
use colored::*;
use palette::{Lab, Srgb};

/// Enum for selecting the color output type.
#[derive(PartialEq)]
pub enum ColorFormat {
    Hex,
    Lab,
    Rgb,
    Hsl,
    Hsv,
    Cmyk,
    Xyz,
    Oklch,
    Lch,
}

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

    fn make_lab(lab: Lab) -> String {
        format!("lab({:.2}, {:.2}, {:.2})", lab.l, lab.a, lab.b)
    }

    fn make_rgb(lab: Lab) -> String {
        let (r, g, b) = ColorUtils::lab_to_rgb(lab);
        format!("rgb({}, {}, {})", r, g, b)
    }

    fn make_hsl(lab: Lab) -> String {
        let (h, s, l) = ColorUtils::lab_to_hsl_tuple(lab);
        format!("hsl({:.1}, {:.2}%, {:.2}%)", h, s * 100.0, l * 100.0)
    }

    fn make_hsv(lab: Lab) -> String {
        let (h, s, v) = ColorUtils::lab_to_hsv_tuple(lab);
        format!("hsv({:.1}, {:.2}%, {:.2}%)", h, s * 100.0, v * 100.0)
    }

    fn make_cmyk(lab: Lab) -> String {
        let (c, m, y, k) = ColorUtils::lab_to_cmyk_tuple(lab);
        format!(
            "cmyk({:.1}%, {:.1}%, {:.1}%, {:.1}%)",
            c * 100.0,
            m * 100.0,
            y * 100.0,
            k * 100.0
        )
    }

    fn make_xyz(lab: Lab) -> String {
        let (x, y, z) = ColorUtils::lab_to_xyz_tuple(lab);
        format!("xyz({:.2}, {:.2}, {:.2})", x, y, z)
    }

    fn make_hex(lab: Lab) -> String {
        let srgb: Srgb = ColorUtils::lab_to_srgb(lab);
        ColorUtils::srgb_to_hex(srgb)
    }

    fn make_oklch(lab: Lab) -> String {
        let (l, c, h) = ColorUtils::lab_to_oklch_tuple(lab);
        format!("oklch({:.3}, {:.3}, {:.1})", l, c, h)
    }

    fn make_lch(lab: Lab) -> String {
        let (l, c, h) = ColorUtils::lab_to_lch_tuple(lab);
        format!("lch({:.3}, {:.3}, {:.1})", l, c, h)
    }

    fn make_format(lab: Lab, color_format: &ColorFormat) -> String {
        match color_format {
            ColorFormat::Lab => Self::make_lab(lab),
            ColorFormat::Rgb => Self::make_rgb(lab),
            ColorFormat::Hsl => Self::make_hsl(lab),
            ColorFormat::Hsv => Self::make_hsv(lab),
            ColorFormat::Cmyk => Self::make_cmyk(lab),
            ColorFormat::Hex => Self::make_hex(lab),
            ColorFormat::Xyz => Self::make_xyz(lab),
            ColorFormat::Oklch => Self::make_oklch(lab),
            ColorFormat::Lch => Self::make_lch(lab),
        }
    }

    /// Writes a formatted header with a specific width and style.
    pub fn print_pair_ln(label: &str, value: &str) {
        println!("{} {}", Self::make_label(label), value);
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
}
