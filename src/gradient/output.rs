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
    use std::fmt::Write;
    let mut output = String::new();
    output.push_str("Position | Hex     | RGB          | WCAG Luminance\n");
    output.push_str("---------|---------|--------------|---------------\n");

    for value in values {
        writeln!(
            output,
            "{:8} | {:7} | {:12} | {}",
            value.position, value.hex, value.rgb, value.wcag_luminance
        )
        .unwrap(); // Writing to String never fails
    }

    Ok(output)
}

/// Simple CSV output formatter
pub fn format_as_csv(values: &[GradientValue]) -> Result<String> {
    let mut output = String::new();
    output.push_str("position,hex,rgb,wcag_luminance\n");

    for value in values {
        output.push_str(&format!(
            "{},{},{},{}\n",
            value.position, value.hex, value.rgb, value.wcag_luminance
        ));
    }

    Ok(output)
}
