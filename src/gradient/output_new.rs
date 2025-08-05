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
    let mut output = String::new();
    output.push_str("Position | RGB                | Hex     | Lab\n");
    output.push_str("---------|--------------------|---------|-----------\n");
    
    for value in values {
        output.push_str(&format!(
            "{:8.2} | rgb({:3.0}, {:3.0}, {:3.0}) | #{:02x}{:02x}{:02x} | L:{:5.1} a:{:5.1} b:{:5.1}\n",
            value.position,
            value.rgb.red * 255.0,
            value.rgb.green * 255.0,
            value.rgb.blue * 255.0,
            (value.rgb.red * 255.0) as u8,
            (value.rgb.green * 255.0) as u8,
            (value.rgb.blue * 255.0) as u8,
            value.lab.l,
            value.lab.a,
            value.lab.b
        ));
    }
    
    Ok(output)
}

/// Simple CSV output formatter
pub fn format_as_csv(values: &[GradientValue]) -> Result<String> {
    let mut output = String::new();
    output.push_str("position,r,g,b,hex,l,a,b\n");
    
    for value in values {
        output.push_str(&format!(
            "{:.3},{},{},{},#{:02x}{:02x}{:02x},{:.3},{:.3},{:.3}\n",
            value.position,
            (value.rgb.red * 255.0) as u8,
            (value.rgb.green * 255.0) as u8,
            (value.rgb.blue * 255.0) as u8,
            (value.rgb.red * 255.0) as u8,
            (value.rgb.green * 255.0) as u8,
            (value.rgb.blue * 255.0) as u8,
            value.lab.l,
            value.lab.a,
            value.lab.b
        ));
    }
    
    Ok(output)
}
