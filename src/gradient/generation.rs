//! Gradient generation helpers extracted from the former monolithic `generate_gradient` function.
//! Pure helpers isolate computation (F2/F4). Orchestrator handles IO and formatting.

use palette::{Lab, Srgb, IntoColor};
use crate::color_report_formatting::lab_to_rgb;
use crate::color_distance_strategies::{calculate_distance, DistanceAlgorithm};
use crate::color_parser::unified_manager::UnifiedColorManager;
use crate::output_formats::{ContrastAnalysis, ColorCollectionMatches, ColorInfo, GradientColors, GradientConfiguration, GradientStop, EnhancedGradientStop, NestedColorInfo, ProgramMetadata, GradientAnalysisOutput, EnhancedGradientAnalysisOutput};

pub(crate) struct BaseColors { pub start_lab: Lab, pub end_lab: Lab, pub start_rgb: (u8,u8,u8), pub end_rgb: (u8,u8,u8), pub steps: usize }
pub(crate) struct ColorMetrics { pub distance: f32, pub relative_contrast: f32, pub start_luminance: f64, pub end_luminance: f64 }

pub(crate) fn parse_base_colors(args: &crate::cli::GradientArgs) -> crate::error::Result<BaseColors> {
    let parser = crate::color_parser::ColorParser::new();
    let (start_lab, _) = parser.parse(&args.start_color)?; let (end_lab, _) = parser.parse(&args.end_color)?;
    let (sr, sg, sb) = lab_to_rgb(start_lab); let (er, eg, eb) = lab_to_rgb(end_lab);
    let start_srgb = Srgb::new(sr as f32 / 255.0, sg as f32 / 255.0, sb as f32 / 255.0);
    let end_srgb   = Srgb::new(er as f32 / 255.0, eg as f32 / 255.0, eb as f32 / 255.0);
    let start_lab: Lab = start_srgb.into_color(); let end_lab: Lab = end_srgb.into_color();
    let steps = if let Some(step_percent) = args.step { (100 / step_percent as usize).max(2) } else { args.stops };
    Ok(BaseColors { start_lab, end_lab, start_rgb: (sr,sg,sb), end_rgb: (er,eg,eb), steps })
}

pub(crate) fn compute_metrics(base: &BaseColors) -> ColorMetrics {
    let distance = calculate_distance(DistanceAlgorithm::DeltaE2000, base.start_lab, base.end_lab) as f32;
    let relative_contrast = contrast_ratio(base.start_rgb, base.end_rgb) as f32;
    let start_luminance = wcag_relative_luminance_rgb(base.start_rgb);
    let end_luminance = wcag_relative_luminance_rgb(base.end_rgb);
    ColorMetrics { distance, relative_contrast, start_luminance, end_luminance }
}

pub(crate) fn unified_stops(args: &crate::cli::GradientArgs, base: &BaseColors) -> Vec<crate::gradient::UnifiedGradientStop> {
    let cfg = crate::gradient::unified_calculator::GradientCalculationConfig { start_lab: base.start_lab, end_lab: base.end_lab, start_position: args.start_position, end_position: args.end_position, ease_in: args.ease_in, ease_out: args.ease_out, steps: base.steps, use_simple_mode: args.stops_simple, algorithm: DistanceAlgorithm::DeltaE2000 };
    crate::gradient::GradientCalculator::calculate_unified_gradient_cfg(cfg)
}

pub(crate) fn find_color_collections(manager: &UnifiedColorManager, rgb: (u8,u8,u8)) -> ColorCollectionMatches {
    let css_matches = manager.find_closest_css_colors([rgb.0, rgb.1, rgb.2], 1);
    let ral_classic_matches = manager.find_closest_ral_classic([rgb.0, rgb.1, rgb.2], 1);
    let ral_design_matches = manager.find_closest_ral_design([rgb.0, rgb.1, rgb.2], 1);
    let (css, css_distance) = if let Some(m) = css_matches.first() { let hex = format!("#{:02X}{:02X}{:02X}", m.entry.color.rgb[0], m.entry.color.rgb[1], m.entry.color.rgb[2]); (format!("{} | {} | {}", m.entry.metadata.code.as_deref().unwrap_or("unknown"), m.entry.metadata.name, hex), m.distance) } else { ("Unknown | Unknown | #000000".into(), 999.0) };
    let (ralc, ralc_distance) = if let Some(m) = ral_classic_matches.first() { let hex = format!("#{:02X}{:02X}{:02X}", m.entry.color.rgb[0], m.entry.color.rgb[1], m.entry.color.rgb[2]); (format!("{} | {} | {}", m.entry.metadata.code.as_deref().unwrap_or("unknown"), m.entry.metadata.name, hex), m.distance) } else { ("Unknown | Unknown | #000000".into(), 999.0) };
    let (raldsp, raldsp_distance) = if let Some(m) = ral_design_matches.first() { let hex = format!("#{:02X}{:02X}{:02X}", m.entry.color.rgb[0], m.entry.color.rgb[1], m.entry.color.rgb[2]); (format!("{} | {} | {}", m.entry.metadata.code.as_deref().unwrap_or("unknown"), m.entry.metadata.name, hex), m.distance) } else { ("Unknown | Unknown | #000000".into(), 999.0) };
    ColorCollectionMatches { css, css_distance, ralc, ralc_distance, raldsp, raldsp_distance }
}

pub(crate) fn build_gradient_stops(unified: &[crate::gradient::UnifiedGradientStop], base: &BaseColors, manager: &UnifiedColorManager) -> Vec<GradientStop> {
    let mut out = Vec::with_capacity(unified.len());
    for stop in unified { let hex = lab_to_hex(stop.lab_color); let luminance = wcag_relative_luminance_rgb(stop.rgb_color); let distance = calculate_distance(DistanceAlgorithm::DeltaE2000, base.start_lab, stop.lab_color) as f32; let closest_css = manager.find_closest_css_colors([stop.rgb_color.0, stop.rgb_color.1, stop.rgb_color.2], 1); let color_name = if closest_css.is_empty() { None } else { Some(crate::output_formats::ColorNameInfo { exact: None, nearest: Some(crate::output_formats::NearestColorMatch { name: closest_css[0].entry.metadata.name.clone(), collection: "CSS".into(), distance: closest_css[0].distance }), all_collections: None }) }; out.push(GradientStop { position: stop.position as u32, hex: hex.clone(), rgb: format!("rgb({}, {}, {})", stop.rgb_color.0, stop.rgb_color.1, stop.rgb_color.2), lab: format!("lab({:.2}, {:.2}, {:.2})", stop.lab_color.l, stop.lab_color.a, stop.lab_color.b), lch: format!("lch({:.2}, {:.2}, {:.1})", stop.lab_color.l, stop.lab_color.a.hypot(stop.lab_color.b), stop.lab_color.b.atan2(stop.lab_color.a).to_degrees()), wcag21_relative_luminance: luminance, distance, color_name }); }
    out
}

pub(crate) fn build_enhanced_stops(unified: &[crate::gradient::UnifiedGradientStop], base: &BaseColors, manager: &UnifiedColorManager) -> Vec<EnhancedGradientStop> {
    let mut out = Vec::with_capacity(unified.len());
    for stop in unified { let hex = lab_to_hex(stop.lab_color); let luminance = wcag_relative_luminance_rgb(stop.rgb_color); let distance = calculate_distance(DistanceAlgorithm::DeltaE2000, base.start_lab, stop.lab_color) as f32; let collections = find_color_collections(manager, (stop.rgb_color.0, stop.rgb_color.1, stop.rgb_color.2)); out.push(EnhancedGradientStop { position: stop.position as u32, color: NestedColorInfo { hex: hex.clone(), rgb: format!("rgb({}, {}, {})", stop.rgb_color.0, stop.rgb_color.1, stop.rgb_color.2), lab: format!("lab({:.2}, {:.2}, {:.2})", stop.lab_color.l, stop.lab_color.a, stop.lab_color.b), lch: format!("lch({:.2}, {:.2}, {:.1})", stop.lab_color.l, stop.lab_color.a.hypot(stop.lab_color.b), stop.lab_color.b.atan2(stop.lab_color.a).to_degrees()), wcag21_relative_luminance: luminance, distance }, collections }); }
    out
}

pub(crate) fn assemble_outputs(args: &crate::cli::GradientArgs, base: &BaseColors, metrics: &ColorMetrics, start_collections: ColorCollectionMatches, end_collections: ColorCollectionMatches, gradient_stops: Vec<GradientStop>, enhanced_gradient_stops: Vec<EnhancedGradientStop>) -> (GradientAnalysisOutput, EnhancedGradientAnalysisOutput) {
    let configuration = GradientConfiguration { start_color: args.start_color.clone(), end_color: args.end_color.clone(), start_position: args.start_position, end_position: args.end_position, ease_in: args.ease_in, ease_out: args.ease_out, gradient_steps: base.steps };
    let colors = GradientColors { start: ColorInfo { hex: lab_to_hex(base.start_lab), rgb: format!("rgb({}, {}, {})", base.start_rgb.0, base.start_rgb.1, base.start_rgb.2), lab: format!("lab({:.2}, {:.2}, {:.2})", base.start_lab.l, base.start_lab.a, base.start_lab.b), lch: format!("lch({:.2}, {:.2}, {:.1})", base.start_lab.l, base.start_lab.a.hypot(base.start_lab.b), base.start_lab.b.atan2(base.start_lab.a).to_degrees()), contrast: Some(ContrastAnalysis { distance: metrics.distance as f64, wcag21_relative_luminance: metrics.start_luminance, relative_contrast: metrics.relative_contrast }), collections: Some(start_collections.clone()) }, end: ColorInfo { hex: lab_to_hex(base.end_lab), rgb: format!("rgb({}, {}, {})", base.end_rgb.0, base.end_rgb.1, base.end_rgb.2), lab: format!("lab({:.2}, {:.2}, {:.2})", base.end_lab.l, base.end_lab.a, base.end_lab.b), lch: format!("lch({:.2}, {:.2}, {:.1})", base.end_lab.l, base.end_lab.a.hypot(base.end_lab.b), base.end_lab.b.atan2(base.end_lab.a).to_degrees()), contrast: Some(ContrastAnalysis { distance: metrics.distance as f64, wcag21_relative_luminance: metrics.end_luminance, relative_contrast: metrics.relative_contrast }), collections: Some(end_collections.clone()) } };
    let legacy = GradientAnalysisOutput { metadata: ProgramMetadata::new(Some("Delta E 2000")), configuration: configuration.clone(), colors: colors.clone(), gradient_stops };
    let enhanced = EnhancedGradientAnalysisOutput { metadata: ProgramMetadata::new(Some("Delta E 2000")), configuration, colors, gradient_stops: enhanced_gradient_stops };
    (legacy, enhanced)
}

fn wcag_relative_luminance_rgb(rgb: (u8,u8,u8)) -> f64 { let (r,g,b) = (rgb.0 as f64 / 255.0, rgb.1 as f64 / 255.0, rgb.2 as f64 / 255.0); let to_linear = |c: f64| if c <= 0.03928 { c / 12.92 } else { ((c + 0.055)/1.055).powf(2.4) }; 0.2126 * to_linear(r) + 0.7152 * to_linear(g) + 0.0722 * to_linear(b) }
fn contrast_ratio(a: (u8,u8,u8), b: (u8,u8,u8)) -> f64 { use crate::config::algorithm_constants; let l1 = wcag_relative_luminance_rgb(a); let l2 = wcag_relative_luminance_rgb(b); if l1 > l2 { (l1 + algorithm_constants::WCAG_LUMINANCE_OFFSET)/(l2 + algorithm_constants::WCAG_LUMINANCE_OFFSET) } else { (l2 + algorithm_constants::WCAG_LUMINANCE_OFFSET)/(l1 + algorithm_constants::WCAG_LUMINANCE_OFFSET) } }
fn lab_to_hex(lab: Lab) -> String { let srgb: Srgb = lab.into_color(); format!("#{:02x}{:02x}{:02x}", (srgb.red * 255.0) as u8, (srgb.green * 255.0) as u8, (srgb.blue * 255.0) as u8) }
