//! Gradient output formatting

use super::calculator::GradientValue;
use crate::config::math_constants;
use crate::error::Result;

/// Simple gradient output formatter
pub fn format_as_json(values: &[GradientValue]) -> Result<String> {
    let json_values: Vec<serde_json::Value> = values.iter().map(|v| {
        serde_json::json!({
            "position": v.position,
            "rgb": format!("rgb({:.0}, {:.0}, {:.0})", v.rgb.red * math_constants::RGB_MAX_VALUE, v.rgb.green * math_constants::RGB_MAX_VALUE, v.rgb.blue * math_constants::RGB_MAX_VALUE),
            "hex": format!("#{:02x}{:02x}{:02x}", 
                (v.rgb.red * math_constants::RGB_MAX_VALUE) as u8, 
                (v.rgb.green * math_constants::RGB_MAX_VALUE) as u8, 
                (v.rgb.blue * math_constants::RGB_MAX_VALUE) as u8),
            "lab": {
                "l": v.lab.l,
                "a": v.lab.a,
                "b": v.lab.b
            }
        })
    }).collect();
    
    Ok(serde_json::to_string_pretty(&json_values).map_err(|e| {
        crate::error::ColorError::InvalidInput(format!("JSON serialization error: {}", e))
    })?)
}

/// Simple text output formatter  
pub fn format_as_text(values: &[GradientValue]) -> Result<String> {
    let header = [
        "Position | RGB                | Hex     | Lab",
        "---------|--------------------|---------|-----------",
    ];
    let body = values
        .iter()
        .map(|v| {
            format!(
                "{:8.2} | rgb({:3.0}, {:3.0}, {:3.0}) | #{:02x}{:02x}{:02x} | L:{:5.1} a:{:5.1} b:{:5.1}",
                v.position,
                v.rgb.red * 255.0,
                v.rgb.green * 255.0,
                v.rgb.blue * 255.0,
                (v.rgb.red * 255.0) as u8,
                (v.rgb.green * 255.0) as u8,
                (v.rgb.blue * 255.0) as u8,
                v.lab.l,
                v.lab.a,
                v.lab.b
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
    let header = "position,r,g,b,hex,l,a,b";
    let body = values
        .iter()
        .map(|v| {
            format!(
                "{:.3},{},{},{},#{:02x}{:02x}{:02x},{:.3},{:.3},{:.3}",
                v.position,
                (v.rgb.red * 255.0) as u8,
                (v.rgb.green * 255.0) as u8,
                (v.rgb.blue * 255.0) as u8,
                (v.rgb.red * 255.0) as u8,
                (v.rgb.green * 255.0) as u8,
                (v.rgb.blue * 255.0) as u8,
                v.lab.l,
                v.lab.a,
                v.lab.b
            )
        })
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
