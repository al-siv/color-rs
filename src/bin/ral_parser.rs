//! RAL Color Table Parser
//!
//! Utility to parse RAL Classic and RAL Design System+ HTML tables
//! and convert them to Rust static data structures.

use std::fs;

/// RAL Classic color entry with complete data
#[derive(Debug)]
struct RalClassicColor {
    code: String,
    name: String,
    hex: String,
    // CIELAB 1976 values
    lab_l: f32,  // L*%
    lab_a: f32,  // a*
    lab_b: f32,  // b*
    // CMYK values
    cmyk_c: f32, // C%
    cmyk_m: f32, // M%
    cmyk_y: f32, // Y%
    cmyk_k: f32, // K%
    // Light Reflectance Value
    lrv: f32,
}

/// RAL Design System+ color entry with complete data
#[derive(Debug)]
struct RalDesignColor {
    name: String,
    code: String,
    rgb: (u8, u8, u8),
    // CIELAB 1931 values
    hue: f32,        // Hue in degrees
    lightness: f32,  // Lightness %
    chromaticity: f32, // Chromaticity %
}

/// Extract hex color from HTML color cell
#[allow(dead_code)]
fn extract_hex_color(cell: &str) -> Option<String> {
    // Look for #XXXXXX pattern
    if let Some(start) = cell.find('#') {
        let hex_part = &cell[start..];
        if let Some(end) = hex_part.find('"') {
            let hex = &hex_part[..end];
            if hex.len() == 7 && hex.chars().skip(1).all(|c| c.is_ascii_hexdigit()) {
                return Some(hex.to_string());
            }
        }
    }
    None
}

/// Extract RGB values from style attribute
#[allow(dead_code)]
fn extract_rgb_from_style(cell: &str) -> Option<(u8, u8, u8)> {
    if let Some(start) = cell.find("rgb(") {
        let rgb_part = &cell[start + 4..];
        if let Some(end) = rgb_part.find(')') {
            let rgb_values = &rgb_part[..end];
            let parts: Vec<&str> = rgb_values.split(',').collect();
            if parts.len() == 3 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    parts[0].trim().parse::<u8>(),
                    parts[1].trim().parse::<u8>(),
                    parts[2].trim().parse::<u8>(),
                ) {
                    return Some((r, g, b));
                }
            }
        }
    }
    None
}

/// Parse RAL Classic Markdown table
fn parse_ral_classic(md_content: &str) -> Vec<RalClassicColor> {
    let mut colors = Vec::new();
    
    for line in md_content.lines() {
        let line = line.trim();
        
        // Skip header lines, separator lines, and empty lines
        if line.is_empty() || line.starts_with('|') && (line.contains("RAL number") || line.contains("----")) {
            continue;
        }
        
        // Parse data rows that start with | RAL
        if line.starts_with("| RAL ") {
            let columns: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            
            // We expect at least 19 columns (0-18)
            if columns.len() >= 19 {
                let code = columns[1].to_string(); // RAL number
                let name = columns[2].to_string(); // English name
                let hex = columns[3].replace("\\#", "#"); // Sample hex (remove escaped #)
                
                // Parse CIELAB 1976 values (columns 11-13)
                let lab_l = columns[11].parse::<f32>().unwrap_or(0.0);
                let lab_a = columns[12].replace('−', "-").parse::<f32>().unwrap_or(0.0);
                let lab_b = columns[13].replace('−', "-").parse::<f32>().unwrap_or(0.0);
                
                // Parse CMYK values (columns 14-17)
                let cmyk_c = columns[14].parse::<f32>().unwrap_or(0.0);
                let cmyk_m = columns[15].parse::<f32>().unwrap_or(0.0);
                let cmyk_y = columns[16].parse::<f32>().unwrap_or(0.0);
                let cmyk_k = columns[17].parse::<f32>().unwrap_or(0.0);
                
                // Parse LRV (column 18)
                let lrv = columns[18].parse::<f32>().unwrap_or(0.0);
                
                if colors.len() < 3 {
                    println!("Parsed RAL Classic color {}: {} - {} - {} (L*:{}, a*:{}, b*:{}, CMYK:{},{},{},{}, LRV:{})", 
                        colors.len() + 1, code, name, hex, lab_l, lab_a, lab_b, cmyk_c, cmyk_m, cmyk_y, cmyk_k, lrv);
                }
                
                colors.push(RalClassicColor { 
                    code, 
                    name, 
                    hex, 
                    lab_l, 
                    lab_a, 
                    lab_b, 
                    cmyk_c, 
                    cmyk_m, 
                    cmyk_y, 
                    cmyk_k, 
                    lrv 
                });
            }
        }
    }
    
    colors
}

/// Parse a single RAL Classic table row
#[allow(dead_code)]
fn parse_ral_classic_row(row: &str) -> Option<RalClassicColor> {
    // Check if this row contains RAL data
    if !row.contains("RAL ") {
        return None;
    }
    
    // Extract RAL code (e.g., "RAL 1000")
    let code_pattern = "<td>RAL ";
    let code = if let Some(start) = row.find(code_pattern) {
        let code_start = start + code_pattern.len() - 4; // Include "RAL "
        if let Some(end) = row[code_start..].find("</td>") {
            row[code_start..code_start + end].to_string()
        } else {
            return None;
        }
    } else {
        return None;
    };
    
    // Extract hex color (look for #XXXXXX pattern)
    let hex = if let Some(start) = row.find('#') {
        let hex_part = &row[start..];
        if let Some(end) = hex_part.find('"') {
            let hex = &hex_part[..end];
            if hex.len() == 7 && hex.chars().skip(1).all(|c| c.is_ascii_hexdigit()) {
                hex.to_string()
            } else {
                return None;
            }
        } else {
            return None;
        }
    } else {
        return None;
    };
    
    // Extract all <td> content to find the name
    let mut td_contents = Vec::new();
    let mut in_td = false;
    let mut current_content = String::new();
    
    let chars: Vec<char> = row.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        if i + 4 < chars.len() && chars[i..i+4] == ['<', 't', 'd', '>'] {
            in_td = true;
            current_content.clear();
            i += 4;
            continue;
        }
        
        if i + 5 < chars.len() && chars[i..i+5] == ['<', '/', 't', 'd', '>'] {
            if in_td {
                td_contents.push(current_content.trim().to_string());
            }
            in_td = false;
            i += 5;
            continue;
        }
        
        if in_td {
            current_content.push(chars[i]);
        }
        
        i += 1;
    }
    
    // The name should be the second <td> element
    let name = if td_contents.len() >= 2 {
        td_contents[1].clone()
    } else {
        return None;
    };
    
    if !code.is_empty() && !name.is_empty() && !hex.is_empty() {
        Some(RalClassicColor { 
            code, 
            name, 
            hex, 
            lab_l: 0.0, 
            lab_a: 0.0, 
            lab_b: 0.0, 
            cmyk_c: 0.0, 
            cmyk_m: 0.0, 
            cmyk_y: 0.0, 
            cmyk_k: 0.0, 
            lrv: 0.0 
        })
    } else {
        None
    }
}

/// Parse RAL Design System+ Markdown table
fn parse_ral_design(md_content: &str) -> Vec<RalDesignColor> {
    let mut colors = Vec::new();
    
    for line in md_content.lines() {
        let line = line.trim();
        
        // Skip header lines, separator lines, and empty lines
        if line.is_empty() || line.starts_with('|') && (line.contains("Name") || line.contains("CIELAB") || line.contains("----")) {
            continue;
        }
        
        // Parse data rows that start with | and contain actual color data
        if line.starts_with('|') && !line.contains("Hue") {
            let columns: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
            
            // We expect at least 9 columns (0-8)
            if columns.len() >= 9 {
                let name = columns[1].to_string(); // Name
                
                // Skip if this is a header row (check if second column contains degree symbol)
                if !columns[2].contains('°') {
                    continue;
                }
                
                // Parse CIELAB 1931 values (columns 2-4)
                let hue = columns[2].trim_end_matches('°').parse::<f32>().unwrap_or(0.0);
                let lightness = columns[3].trim_end_matches('%').parse::<f32>().unwrap_or(0.0);
                let chromaticity = columns[4].trim_end_matches('%').parse::<f32>().unwrap_or(0.0);
                
                // Parse RGB values (columns 6-8, skip 5 which is Sample)
                let r = columns[6].parse::<u8>().unwrap_or(0);
                let g = columns[7].parse::<u8>().unwrap_or(0);
                let b = columns[8].parse::<u8>().unwrap_or(0);
                
                // Code is the last column (column 9)
                let code = columns[9].to_string();
                
                if !name.is_empty() && !code.is_empty() && code.starts_with('H') {
                    if colors.len() < 3 {
                        println!("Parsed RAL Design color {}: {} - ({},{},{}) - {} (H:{}, L:{}, C:{})", 
                            colors.len() + 1, name, r, g, b, code, hue, lightness, chromaticity);
                    }
                    colors.push(RalDesignColor { 
                        name, 
                        code, 
                        rgb: (r, g, b), 
                        hue, 
                        lightness, 
                        chromaticity 
                    });
                }
            }
        }
    }
    
    colors
}

/// Parse a single RAL Design System+ table row
#[allow(dead_code)]
fn parse_ral_design_row(row: &str) -> Option<RalDesignColor> {
    // Extract all <td> content
    let mut td_contents = Vec::new();
    let mut in_td = false;
    let mut current_content = String::new();
    
    let chars: Vec<char> = row.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        if i + 4 < chars.len() && chars[i..i+4] == ['<', 't', 'd', '>'] {
            in_td = true;
            current_content.clear();
            i += 4;
            continue;
        }
        
        if i + 5 < chars.len() && chars[i..i+5] == ['<', '/', 't', 'd', '>'] {
            if in_td {
                td_contents.push(current_content.trim().to_string());
            }
            in_td = false;
            i += 5;
            continue;
        }
        
        if in_td {
            current_content.push(chars[i]);
        }
        
        i += 1;
    }
    
    // We expect at least 8 td elements: name, hue, lightness, chromaticity, sample, r, g, b, code
    if td_contents.len() >= 8 {
        let name = td_contents[0].clone();
        
        // Extract RGB values (indices 5, 6, 7)
        let r = td_contents[5].parse::<u8>().ok()?;
        let g = td_contents[6].parse::<u8>().ok()?;
        let b = td_contents[7].parse::<u8>().ok()?;
        
        // Code is the last element
        let code = td_contents[td_contents.len() - 1].clone();
        
        if !name.is_empty() && !code.is_empty() {
            return Some(RalDesignColor { 
                name, 
                code, 
                rgb: (r, g, b), 
                hue: 0.0, 
                lightness: 0.0, 
                chromaticity: 0.0 
            });
        }
    }
    
    None
}

/// Generate Rust code for RAL Classic colors
/// Generate Rust code for RAL Classic colors
fn generate_ral_classic_rust(colors: &[RalClassicColor]) -> String {
    let mut output = String::new();
    output.push_str("/// RAL Classic color data: (code, name, hex, lab_l, lab_a, lab_b, cmyk_c, cmyk_m, cmyk_y, cmyk_k, lrv)\n");
    output.push_str("pub static RAL_CLASSIC_DATA: &[(&str, &str, &str, f32, f32, f32, f32, f32, f32, f32, f32)] = &[\n");
    
    for color in colors {
        output.push_str(&format!(
            "    (\"{}\", \"{}\", \"{}\", {:.3}, {:.3}, {:.3}, {:.1}, {:.1}, {:.1}, {:.1}, {:.1}),\n",
            color.code, color.name, color.hex, 
            color.lab_l, color.lab_a, color.lab_b,
            color.cmyk_c, color.cmyk_m, color.cmyk_y, color.cmyk_k,
            color.lrv
        ));
    }
    
    output.push_str("];\n\n");
    output
}

/// Generate Rust code for RAL Design System+ colors
fn generate_ral_design_rust(colors: &[RalDesignColor]) -> String {
    let mut output = String::new();
    output.push_str("/// RAL Design System+ color data: (name, code, rgb, hue, lightness, chromaticity)\n");
    output.push_str("pub static RAL_DESIGN_DATA: &[(&str, &str, [u8; 3], f32, f32, f32)] = &[\n");
    
    for color in colors {
        output.push_str(&format!(
            "    (\"{}\", \"{}\", [{}, {}, {}], {:.1}, {:.1}, {:.1}),\n",
            color.name, color.code, 
            color.rgb.0, color.rgb.1, color.rgb.2,
            color.hue, color.lightness, color.chromaticity
        ));
    }
    
    output.push_str("];\n");
    output
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read RAL Classic Markdown
    let ral_classic_content = fs::read_to_string("ral-classic.md")?;
    println!("RAL Classic file size: {} bytes", ral_classic_content.len());
    
    // Count table rows (lines starting with | RAL)
    let row_count = ral_classic_content.lines().filter(|line| line.trim().starts_with("| RAL ")).count();
    println!("Found {} RAL Classic data rows", row_count);
    
    let ral_classic_colors = parse_ral_classic(&ral_classic_content);
    
    // Read RAL Design System+ Markdown
    let ral_design_content = fs::read_to_string("ral-design-system-plus.md")?;
    println!("RAL Design file size: {} bytes", ral_design_content.len());
    
    let design_row_count = ral_design_content.lines().filter(|line| {
        let trimmed = line.trim();
        trimmed.starts_with('|') && !trimmed.contains("Name") && !trimmed.contains("CIELAB") && !trimmed.contains("----") && !trimmed.contains("Hue")
    }).count();
    println!("Found {} RAL Design data rows", design_row_count);
    
    let ral_design_colors = parse_ral_design(&ral_design_content);
    
    println!("Parsed {} RAL Classic colors", ral_classic_colors.len());
    println!("Parsed {} RAL Design System+ colors", ral_design_colors.len());
    
    // Print first few colors for debugging
    if !ral_classic_colors.is_empty() {
        println!("First RAL Classic color: {:?}", &ral_classic_colors[0]);
    }
    if !ral_design_colors.is_empty() {
        println!("First RAL Design color: {:?}", &ral_design_colors[0]);
    }
    
    // Generate Rust code
    let mut rust_code = String::new();
    rust_code.push_str("//! RAL Color Data\n//!\n//! Static color data for RAL Classic and RAL Design System+\n\n");
    rust_code.push_str(&generate_ral_classic_rust(&ral_classic_colors));
    rust_code.push_str(&generate_ral_design_rust(&ral_design_colors));
    
    // Write to file
    fs::write("src/color_parser/ral_data.rs", rust_code)?;
    
    println!("Generated RAL color data in src/color_parser/ral_data.rs");
    
    Ok(())
}
