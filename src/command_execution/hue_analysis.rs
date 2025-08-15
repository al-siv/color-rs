//! Pure helper functions for hue analysis (extracted from monolithic execute_hue_analysis)
//! Responsibilities: range parsing, collection loading, filtering, sorting, entry & config assembly.

use palette::{Lch, FromColor};
use crate::cli_range::Range;
use crate::color_parser::{CssColorCollection, RalClassicCollection, RalDesignCollection};
use crate::color_parser::collections::ColorCollectionKind;
use crate::output_formats::{HueCollectionConfiguration, HueColorEntry};

pub(crate) fn load_collection(id: &str) -> crate::error::Result<ColorCollectionKind> {
    match id {
        "css" => CssColorCollection::new()
            .map(ColorCollectionKind::Css)
            .map_err(|e| crate::error::ColorError::ParseError(format!("Failed to load CSS collection: {e}"))),
        "ralc" => RalClassicCollection::new()
            .map(ColorCollectionKind::RalClassic)
            .map_err(|e| crate::error::ColorError::ParseError(format!("Failed to load RAL Classic collection: {e}"))),
        "rald" => RalDesignCollection::new()
            .map(ColorCollectionKind::RalDesign)
            .map_err(|e| crate::error::ColorError::ParseError(format!("Failed to load RAL Design collection: {e}"))),
        other => Err(crate::error::ColorError::ParseError(format!("Unknown collection: {other}"))),
    }
}

pub(crate) struct ParsedRanges { pub hue: Option<Range>, pub lightness: Option<Range>, pub chroma: Option<Range> }

pub(crate) fn parse_ranges(args: &crate::cli::HueArgs) -> crate::error::Result<ParsedRanges> {
    let hue = match &args.hue_range { Some(r) => Some(Range::parse(r)?), None => None };
    let lightness = match &args.lightness_range { Some(r) => Some(Range::parse(r)?), None => None };
    let chroma = match &args.chroma_range { Some(r) => Some(Range::parse(r)?), None => None };
    Ok(ParsedRanges { hue, lightness, chroma })
}

pub(crate) type FilteredColor<'a> = (&'a crate::color_parser::ColorEntry, Lch);

pub(crate) fn collect_filtered_colors<'a>(
    collection: &'a ColorCollectionKind,
    ranges: &ParsedRanges,
) -> Vec<FilteredColor<'a>> {
    collection.colors().iter().filter_map(|color_entry| {
        let srgb = palette::Srgb::new(
            color_entry.color.rgb[0] as f32 / 255.0,
            color_entry.color.rgb[1] as f32 / 255.0,
            color_entry.color.rgb[2] as f32 / 255.0,
        );
        let lch: Lch = FromColor::from_color(srgb);
        if let Some(hr) = &ranges.hue { if !hr.contains_with_wrap(lch.hue.into_degrees() as f64) { return None; } }
        if let Some(lr) = &ranges.lightness { if !lr.contains_linear(lch.l as f64) { return None; } }
        if let Some(cr) = &ranges.chroma { if !cr.contains_linear(lch.chroma as f64) { return None; } }
        Some((color_entry, lch))
    }).collect()
}

pub(crate) fn sort_filtered_colors(colors: &mut Vec<FilteredColor<'_>>) {
    colors.sort_by(|a,b| {
        let hue_cmp = a.1.hue.into_degrees().partial_cmp(&b.1.hue.into_degrees()).unwrap_or(std::cmp::Ordering::Equal);
        if hue_cmp != std::cmp::Ordering::Equal { hue_cmp } else {
            a.0.metadata.code.as_ref().unwrap_or(&String::new())
                .cmp(b.0.metadata.code.as_ref().unwrap_or(&String::new()))
        }
    });
}

pub(crate) fn build_configuration(args: &crate::cli::HueArgs, total: usize) -> HueCollectionConfiguration {
    HueCollectionConfiguration { collection: args.collection.clone(), total_colors: total, hue_range: args.hue_range.clone(), lightness_range: args.lightness_range.clone(), chroma_range: args.chroma_range.clone() }
}

/// Compute minimal signed hue difference (current - previous) constrained to [-180,180].
/// Returns None if no previous hue.
pub fn compute_hue_shift(previous_hue: Option<f64>, current_hue: f64) -> Option<f64> {
    previous_hue.map(|prev| {
        let mut diff = current_hue - prev;
        if diff > 180.0 { diff -= 360.0; }
        else if diff < -180.0 { diff += 360.0; }
        diff
    })
}

pub(crate) fn build_hue_entries(filtered: &[FilteredColor<'_>]) -> Vec<HueColorEntry> {
    let mut previous_hue: Option<f64> = None;
    filtered.iter().map(|(color_entry, lch)| {
        let hue = lch.hue.into_degrees() as f64;
        let hue_shift = compute_hue_shift(previous_hue, hue);
        let hex = format!("#{:02X}{:02X}{:02X}", color_entry.color.rgb[0], color_entry.color.rgb[1], color_entry.color.rgb[2]);
        let lch_str = format!("lch({:>4.1}, {:>4.1}, {:>6.1})", lch.l, lch.chroma, hue);
        let code = color_entry.metadata.code.as_ref().unwrap_or(&"Unknown".to_string()).clone();
        let name = color_entry.metadata.name.clone();
        let hue_shift_str = match hue_shift { Some(shift) => format!("{:>6}", format!("+{:.2}", shift)), None => format!("{:>6}", "—") };
        let display = format!("{hue:>6.1} | {hex} | {lch_str} | {hue_shift_str} | {code} | {name}");
        previous_hue = Some(hue);
        HueColorEntry { display }
    }).collect()
}

pub(crate) fn build_visual_analysis_results<'a>(filtered: &[FilteredColor<'a>], collection_id: &str) -> Vec<crate::color_ops::analysis::hue::HueAnalysisResult> {
    filtered.iter().map(|(color_entry, lch)| crate::color_ops::analysis::hue::HueAnalysisResult {
        color: *lch,
        name: Some(color_entry.metadata.name.clone()),
        hue_distance: 0.0,
        saturation: lch.chroma as f64,
        lightness: lch.l as f64,
        collection: collection_id.to_string(),
        code: color_entry.metadata.code.clone(),
    }).collect()
}

pub(crate) fn build_metadata(args: &crate::cli::HueArgs, filtered_len: usize) -> std::collections::HashMap<String,String> {
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("collection".into(), args.collection.clone());
    metadata.insert("total_colors".into(), filtered_len.to_string());
    if let Some(r) = &args.hue_range { metadata.insert("hue_range".into(), r.clone()); }
    if let Some(r) = &args.lightness_range { metadata.insert("lightness_range".into(), r.clone()); }
    if let Some(r) = &args.chroma_range { metadata.insert("chroma_range".into(), r.clone()); }
    metadata
}
