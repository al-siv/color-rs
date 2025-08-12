//! Pure image/SVG construction utilities extracted from image.rs
//!
//! This module contains side-effect free functions (pure core) for building
//! gradient, hue gradient, and hue palette SVG strings plus color format
//! helpers. All filesystem IO, PNG rendering, and vectorization remain in
//! image.rs (imperative shell). This separation supports the functional
//! core / imperative shell pattern and enables easier unit testing.

use palette::{IntoColor, Lab, Lch};

use crate::cli::{GradientArgs, HueArgs};
use crate::color_ops::analysis::hue::HueAnalysisResult;
use crate::config::{algorithm_constants, display_constants, math_constants};
use crate::error::{ColorError, Result};
use crate::gradient::GradientCalculator;

/// Convert a color component from 0.0-1.0 range to 0-255 u8
#[must_use]
fn component_to_u8(component: f32) -> u8 {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    {
        (component * math_constants::RGB_MAX_VALUE)
            .round()
            .clamp(0.0, math_constants::RGB_MAX_VALUE) as u8
    }
}

/// Convert LAB color to hex string for image generation
#[must_use]
pub fn lab_to_hex(lab: Lab) -> String {
    let srgb: palette::Srgb = lab.into_color();
    format!(
        "#{:02X}{:02X}{:02X}",
        component_to_u8(srgb.red),
        component_to_u8(srgb.green),
        component_to_u8(srgb.blue),
    )
}

/// Convert LCH color to hex string for image generation
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn lch_to_hex(lch: Lch) -> String {
    let lab: Lab = lch.into_color();
    lab_to_hex(lab)
}

/// Build SVG content for a LAB gradient.
/// Kept as Result for forward compatibility (may add validation errors later).
pub fn create_svg_content(
    args: &GradientArgs,
    start_lab: Lab,
    end_lab: Lab,
) -> Result<String> {
    let width = args.width;
    let gradient_height = (f64::from(width) * display_constants::HEIGHT_RATIO) as u32;
    let legend_height = if args.no_legend {
        0
    } else {
        (f64::from(gradient_height) * display_constants::DEFAULT_LEGEND_HEIGHT_RATIO)
            .max(display_constants::MIN_LEGEND_HEIGHT) as u32
    };
    let total_height = gradient_height + legend_height;

    let start_hex = lab_to_hex(start_lab);
    let end_hex = lab_to_hex(end_lab);

    let mut svg = String::new();
    svg.push_str(&format!(
        r#"<svg width="{width}" height="{total_height}" xmlns="http://www.w3.org/2000/svg">"#
    ));
    svg.push('\n');

    // Gradient definition
    svg.push_str("  <defs>\n");
    svg.push_str(&format!(
        "    <linearGradient id=\"grad\" x1=\"{}%\" y1=\"0%\" x2=\"{}%\" y2=\"0%\">\n",
        args.start_position, args.end_position
    ));

    let svg_steps = 400; // High resolution for smooth gradients
    let cfg = crate::gradient::unified_calculator::GradientConfigBuilder::new()
        .start_lab(start_lab)
        .end_lab(end_lab)
        .start_position(args.start_position)?
        .end_position(args.end_position)?
        .ease_in(args.ease_in)?
        .ease_out(args.ease_out)?
        .steps(svg_steps)?
        .simple_mode(args.stops_simple)
        .build()?;
    let unified_stops = GradientCalculator::calculate_unified_gradient_cfg(cfg);

    let position_range = args.end_position - args.start_position;
    // Iterator pipeline: map stops -> (rounded_offset, hex) -> dedup consecutive -> format
    unified_stops
        .into_iter()
        .map(|stop| {
            let relative_offset_precise =
                (stop.position - args.start_position) as f64 / position_range as f64 * 100.0;
            let relative_offset = (relative_offset_precise
                * algorithm_constants::GRADIENT_OFFSET_PRECISION)
                .round()
                / algorithm_constants::GRADIENT_OFFSET_PRECISION;
            (relative_offset, lab_to_hex(stop.lab_color))
        })
        .scan(None, |last: &mut Option<f64>, (offset, hex)| {
            let emit = match *last {
                Some(prev) => (offset - prev).abs() >= 0.5,
                None => true,
            };
            if emit {
                *last = Some(offset);
                let offset_str = if offset.fract() == 0.0 {
                    format!("{}%", offset.round() as u8)
                } else {
                    format!("{offset:.1}%")
                };
                Some(Some(format!(
                    "      <stop offset=\"{offset_str}\" stop-color=\"{hex}\" />\n"
                )))
            } else {
                Some(None)
            }
        })
        .flatten()
        .for_each(|line| svg.push_str(&line));
    svg.push_str("    </linearGradient>\n");
    svg.push_str("  </defs>\n");

    svg.push_str(&format!(
        "  <rect x=\"0\" y=\"0\" width=\"{width}\" height=\"{gradient_height}\" fill=\"url(#grad)\" />\n"
    ));

    if !args.no_legend {
        let font_size = (f64::from(legend_height) * display_constants::DEFAULT_FONT_SIZE_RATIO)
            .max(display_constants::MIN_FONT_SIZE) as u32;
        let text_y = gradient_height
            + (f64::from(legend_height) * display_constants::DEFAULT_TEXT_Y_RATIO) as u32;
        svg.push_str(&format!(
            "  <rect x=\"0\" y=\"{gradient_height}\" width=\"100%\" height=\"{legend_height}\" fill=\"rgb(0,0,0)\" />\n"
        ));
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" fill=\"white\">\n",
            width / 100,
            text_y,
            display_constants::FONT_FAMILY,
            font_size
        ));
        svg.push_str(&format!(
            "    cubic-bezier({}, 0, {}, 1) | positions: {}%-{}% | colors: {}-{}\n",
            args.ease_in,
            args.ease_out,
            args.start_position,
            args.end_position,
            start_hex,
            end_hex
        ));
        svg.push_str("  </text>\n");
    }

    svg.push_str("</svg>");
    Ok(svg)
}

/// Build hue gradient SVG (horizontal) from analysis results.
pub fn create_hue_gradient_svg(args: &HueArgs, colors: &[HueAnalysisResult]) -> Result<String> {
    if colors.is_empty() {
        return Err(ColorError::InvalidArguments(
            "Cannot create gradient from empty color collection".to_string(),
        ));
    }

    let width = args.width;
    let height = (f64::from(width) * display_constants::HEIGHT_RATIO) as u32;
    let mut svg = String::new();
    svg.push_str(&format!(
        r#"<svg width="{width}" height="{height}" xmlns="http://www.w3.org/2000/svg">"#
    ));
    svg.push('\n');
    svg.push_str(&format!(
        "  <rect x=\"0\" y=\"0\" width=\"{width}\" height=\"{height}\" fill=\"white\" />\n"
    ));
    svg.push_str("  <defs>\n");
    svg.push_str(
        "    <linearGradient id=\"huegrad\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"0%\">\n",
    );
    let step = 100.0 / (colors.len() - 1).max(1) as f64;
    // Iterator pipeline replicating imperative logic with explicit edge handling
    colors
        .iter()
        .enumerate()
        .flat_map(|(i, color)| {
            let hex_color = lch_to_hex(color.color);
            if i == 0 {
                // Initial stop at 0%
                vec![format!("      <stop offset=\"0%\" stop-color=\"{hex_color}\" />\n")] // single element
            } else if i == colors.len() - 1 {
                let end_position = (i as f64 * step) - 0.5;
                let prev_hex = lch_to_hex(colors[i - 1].color);
                vec![
                    format!(
                        "      <stop offset=\"{end_position:.1}%\" stop-color=\"{prev_hex}\" />\n"
                    ),
                    format!(
                        "      <stop offset=\"{:.1}%\" stop-color=\"{hex_color}\" />\n",
                        end_position + 0.1
                    ),
                    format!(
                        "      <stop offset=\"100%\" stop-color=\"{hex_color}\" />\n"
                    ),
                ]
            } else {
                let start_position = (i as f64 * step) - 0.5;
                let end_position = (i as f64 * step) + 0.5;
                let prev_hex = lch_to_hex(colors[i - 1].color);
                vec![
                    format!(
                        "      <stop offset=\"{start_position:.1}%\" stop-color=\"{prev_hex}\" />\n"
                    ),
                    format!(
                        "      <stop offset=\"{:.1}%\" stop-color=\"{hex_color}\" />\n",
                        start_position + 0.1
                    ),
                    format!(
                        "      <stop offset=\"{end_position:.1}%\" stop-color=\"{hex_color}\" />\n"
                    ),
                ]
            }
        })
        .for_each(|line| svg.push_str(&line));
    svg.push_str("    </linearGradient>\n");
    svg.push_str("  </defs>\n");
    svg.push_str(&format!(
        "  <rect x=\"0\" y=\"0\" width=\"{width}\" height=\"{height}\" fill=\"url(#huegrad)\" />\n"
    ));
    if !args.no_labels {
        let font_size = 24;
        let title = format!(
            "{} Collection Hue Gradient ({} colors)",
            args.collection.to_uppercase(),
            colors.len()
        );
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" fill=\"white\" text-anchor=\"middle\" stroke=\"black\" stroke-width=\"1\">\n",
            width / 2,
            font_size + 10,
            display_constants::FONT_FAMILY,
            font_size
        ));
        svg.push_str(&format!("    {title}\n"));
        svg.push_str("  </text>\n");
    }
    svg.push_str("</svg>");
    Ok(svg)
}

/// Build hue palette SVG (vertical list of swatches) from analysis results.
pub fn create_hue_palette_svg(args: &HueArgs, colors: &[HueAnalysisResult]) -> Result<String> {
    if colors.is_empty() {
        return Err(ColorError::InvalidArguments(
            "Cannot create palette from empty color collection".to_string(),
        ));
    }
    let width = args.width;
    let swatch_height = args.color_height.unwrap_or(80u32);
    let header_height = if args.no_labels { 0 } else { args.font_size * 4 };
    let total_height = header_height + (colors.len() as u32 * swatch_height);
    let mut svg = String::new();
    svg.push_str(&format!(
        r#"<svg width="{width}" height="{total_height}" xmlns="http://www.w3.org/2000/svg">"#
    ));
    svg.push('\n');
    svg.push_str(&format!(
        "  <rect x=\"0\" y=\"0\" width=\"{width}\" height=\"{total_height}\" fill=\"white\" />\n"
    ));
    let mut y_offset = 0;
    if !args.no_labels {
        let font_size = args.font_size * 15 / 10;
        let title = if let Some(ref custom_header) = args.header_text {
            custom_header.clone()
        } else {
            format!(
                "{} Collection Color Palette ({} colors)",
                args.collection.to_uppercase(),
                colors.len()
            )
        };
        let header_padding = (args.color_height.unwrap_or(50) + args.border_width) / 2
            - font_size / 2;
        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" fill=\"black\" text-anchor=\"start\">\n",
            header_padding,
            header_height / 2 + font_size / 2,
            display_constants::HEADER_FONT_FAMILY,
            font_size
        ));
        svg.push_str(&format!("{title}\n"));
        svg.push_str("</text>\n");
        y_offset = header_height;
    }
    colors
        .iter()
        .enumerate()
        .scan(None, |prev_hue: &mut Option<f32>, (i, color)| {
            let lch = color.color;
            let current_hue = lch.hue.into_degrees();
            let hue_delta_str = if let Some(prev) = *prev_hue {
                format!("{:+.2}", current_hue - prev)
            } else {
                "—".to_string()
            };
            *prev_hue = Some(current_hue);
            let y = y_offset + (i as u32 * swatch_height);
            let hex_color = lch_to_hex(lch);
            let base_rect = format!(
                "  <rect x=\"0\" y=\"{y}\" width=\"{width}\" height=\"{swatch_height}\" fill=\"{hex_color}\" stroke=\"{}\" stroke-width=\"{}\" />\n",
                args.border_color, args.border_width
            );
            if args.no_labels {
                Some(vec![base_rect])
            } else {
                let font_size = args.font_size;
                let text_y = y + swatch_height / 2 + font_size / 2;
                let hue_str = format!("{current_hue:.0}");
                let hex_str = hex_color.to_uppercase();
                let lch_str = format!("lch({:.1}, {:.1}, {:.1})", lch.l, lch.chroma, current_hue);
                let code_str = color.code.as_deref().unwrap_or("Unknown");
                let name_str = color.name.as_deref().unwrap_or("Unknown");
                let display_text = format!(
                    "{hue_str} | {hex_str} | {lch_str} | {code_str} | {name_str} | {hue_delta_str}"
                );
                let text_color = if is_dark_color(&hex_color) { "white" } else { "black" };
                let text_padding = (args.color_height.unwrap_or(50) + args.border_width) / 2
                    - font_size / 2;
                let text_block = format!(
                    "  <text x=\"{}\" y=\"{text_y}\" font-family=\"{}\" font-size=\"{font_size}\" fill=\"{text_color}\" text-anchor=\"start\">\n    {display_text}\n  </text>\n",
                    text_padding, display_constants::FONT_FAMILY
                );
                Some(vec![base_rect, text_block])
            }
        })
        .flatten()
        .for_each(|segment| svg.push_str(&segment));
    svg.push_str("</svg>\n");
    Ok(svg)
}

/// Determine if a hex color is dark (for contrast decisions)
#[must_use]
pub fn is_dark_color(hex_color: &str) -> bool {
    let hex = hex_color.trim_start_matches('#');
    if hex.len() != 6 {
        return false;
    }
    if let (Ok(r), Ok(g), Ok(b)) = (
        u8::from_str_radix(&hex[0..2], 16),
        u8::from_str_radix(&hex[2..4], 16),
        u8::from_str_radix(&hex[4..6], 16),
    ) {
        let luminance = 0.299 * f64::from(r) + 0.587 * f64::from(g) + 0.114 * f64::from(b);
        luminance < 128.0
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color_parser::ColorParser;

    fn test_gradient_args() -> GradientArgs {
        GradientArgs {
            start_color: "FF0000".to_string(),
            end_color: "0000FF".to_string(),
            start_position: 0,
            end_position: 100,
            ease_in: 0.25,
            ease_out: 0.75,
            svg: Some("test.svg".to_string()),
            png: None,
            no_legend: false,
            width: 800,
            step: None,
            stops: 5,
            stops_simple: false,
            output_format: None,
            output_file: None,
            func_filter: None,
            vectorized_text: false,
        }
    }

    #[test]
    fn builds_basic_svg_gradient() {
        let parser = ColorParser::new();
        let args = test_gradient_args();
        let (start_lab, _) = parser.parse(&args.start_color).unwrap();
        let (end_lab, _) = parser.parse(&args.end_color).unwrap();
        let svg = create_svg_content(&args, start_lab, end_lab).unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("linearGradient"));
        assert!(svg.contains("</svg>"));
    }
}
