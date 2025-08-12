//! Gradient output formatting

use super::calculator::GradientValue;
use crate::error::Result;

/// Simple gradient output formatter
pub fn format_as_json(values: &[GradientValue]) -> Result<String> {
    let json_values: Vec<serde_json::Value> = values
        .iter()
        .map(|v| {
            serde_json::json!({
                "position": v.position,
                "hex": v.hex,
                "rgb": v.rgb,
                "wcag_luminance": v.wcag_luminance
            })
        })
        .collect();

    serde_json::to_string_pretty(&json_values)
        .map_err(|e| crate::error::ColorError::General(format!("JSON serialization error: {e}")))
}

/// Simple text output formatter  
pub fn format_as_text(values: &[GradientValue]) -> Result<String> {
    // Iterator pipeline version (removes imperative loop & explicit mutable writes)
    let header = [
        "Position | Hex     | RGB          | WCAG Luminance",
        "---------|---------|--------------|---------------",
    ];

    let body = values
        .iter()
        .map(|v| {
            // Allocation per line acceptable (output size small / CLI oriented)
            format!(
                "{:8} | {:7} | {:12} | {}",
                v.position, v.hex, v.rgb, v.wcag_luminance
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut out = header.join("\n");
    if !body.is_empty() {
        out.push('\n');
        out.push_str(&body);
        out.push('\n');
    } else {
        out.push('\n');
    }
    Ok(out)
}

/// Simple CSV output formatter
pub fn format_as_csv(values: &[GradientValue]) -> Result<String> {
    let header = "position,hex,rgb,wcag_luminance";
    let body = values
        .iter()
        .map(|v| format!("{},{},{},{}", v.position, v.hex, v.rgb, v.wcag_luminance))
        .collect::<Vec<_>>()
        .join("\n");
    let mut out = String::with_capacity(header.len() + 1 + body.len() + 1);
    out.push_str(header);
    out.push('\n');
    if !body.is_empty() {
        out.push_str(&body);
        out.push('\n');
    }
    Ok(out)
}
