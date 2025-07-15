//! CSS Color Parser 
//! 
//! Modernized and integrated version of css-color-parser-rs
//! Original: https://github.com/7thSigil/css-color-parser-rs
//! Authors: Dean McNamee, Katkov Oleksandr

use std::collections::HashMap;
use std::str::FromStr;
use super::types::{ParsedColor, ColorFormat};
use crate::error::{ColorError, Result};

/// CSS color parser that handles various CSS color formats
pub struct CssColorParser {
    named_colors: HashMap<String, (u8, u8, u8)>,
}

impl CssColorParser {
    /// Create a new CSS color parser
    pub fn new() -> Self {
        Self {
            named_colors: Self::create_named_colors(),
        }
    }

    /// Parse a CSS color string
    pub fn parse(&self, input: &str) -> Result<ParsedColor> {
        let input = input.trim();
        if input.is_empty() {
            return Err(ColorError::InvalidColor("Empty color string".to_string()));
        }

        // Remove all whitespace and convert to lowercase
        let mut cleaned = input.replace(' ', "");
        cleaned.make_ascii_lowercase();

        // Check for named colors first
        if let Some(&(r, g, b)) = self.named_colors.get(&cleaned) {
            return Ok(ParsedColor::from_rgb(r, g, b, ColorFormat::Named));
        }

        // Check for hex colors
        if cleaned.starts_with('#') {
            return self.parse_hex(&cleaned);
        }

        // Check for functional notation (rgb, rgba, hsl, hsla)
        if let Some(open_paren) = cleaned.find('(') {
            if let Some(close_paren) = cleaned.find(')') {
                if close_paren + 1 == cleaned.len() && close_paren > open_paren {
                    return self.parse_functional(&cleaned, open_paren, close_paren);
                }
            }
        }

        Err(ColorError::InvalidColor(format!("Unrecognized color format: {}", input)))
    }

    /// Parse hex color (#rgb or #rrggbb)
    fn parse_hex(&self, input: &str) -> Result<ParsedColor> {
        let hex_part = &input[1..]; // Remove #
        
        match hex_part.len() {
            3 => {
                // #rgb format
                let r_hex = &hex_part[0..1];
                let g_hex = &hex_part[1..2];
                let b_hex = &hex_part[2..3];

                let r = u8::from_str_radix(&format!("{}{}", r_hex, r_hex), 16)
                    .map_err(|_| ColorError::InvalidColor("Invalid hex color".to_string()))?;
                let g = u8::from_str_radix(&format!("{}{}", g_hex, g_hex), 16)
                    .map_err(|_| ColorError::InvalidColor("Invalid hex color".to_string()))?;
                let b = u8::from_str_radix(&format!("{}{}", b_hex, b_hex), 16)
                    .map_err(|_| ColorError::InvalidColor("Invalid hex color".to_string()))?;

                Ok(ParsedColor::from_rgb(r, g, b, ColorFormat::Hex))
            }
            6 => {
                // #rrggbb format
                let r = u8::from_str_radix(&hex_part[0..2], 16)
                    .map_err(|_| ColorError::InvalidColor("Invalid hex color".to_string()))?;
                let g = u8::from_str_radix(&hex_part[2..4], 16)
                    .map_err(|_| ColorError::InvalidColor("Invalid hex color".to_string()))?;
                let b = u8::from_str_radix(&hex_part[4..6], 16)
                    .map_err(|_| ColorError::InvalidColor("Invalid hex color".to_string()))?;

                Ok(ParsedColor::from_rgb(r, g, b, ColorFormat::Hex))
            }
            _ => Err(ColorError::InvalidColor("Invalid hex color length".to_string())),
        }
    }

    /// Parse functional notation (rgb, rgba, hsl, hsla)
    fn parse_functional(&self, input: &str, open_paren: usize, close_paren: usize) -> Result<ParsedColor> {
        let function_name = &input[..open_paren];
        let params_str = &input[open_paren + 1..close_paren];
        let params: Vec<&str> = params_str.split(',').collect();

        match function_name {
            "rgb" => {
                if params.len() != 3 {
                    return Err(ColorError::InvalidColor("RGB requires 3 parameters".to_string()));
                }
                let (r, g, b) = self.parse_rgb_params(&params)?;
                Ok(ParsedColor::from_rgb(r, g, b, ColorFormat::Rgb))
            }
            "rgba" => {
                if params.len() != 4 {
                    return Err(ColorError::InvalidColor("RGBA requires 4 parameters".to_string()));
                }
                let (r, g, b) = self.parse_rgb_params(&params[..3])?;
                let a = self.parse_alpha(params[3])?;
                Ok(ParsedColor::new(r, g, b, a, ColorFormat::Rgba))
            }
            "hsl" => {
                if params.len() != 3 {
                    return Err(ColorError::InvalidColor("HSL requires 3 parameters".to_string()));
                }
                let (r, g, b) = self.parse_hsl_params(&params)?;
                Ok(ParsedColor::from_rgb(r, g, b, ColorFormat::Hsl))
            }
            "hsla" => {
                if params.len() != 4 {
                    return Err(ColorError::InvalidColor("HSLA requires 4 parameters".to_string()));
                }
                let (r, g, b) = self.parse_hsl_params(&params[..3])?;
                let a = self.parse_alpha(params[3])?;
                Ok(ParsedColor::new(r, g, b, a, ColorFormat::Hsla))
            }
            _ => Err(ColorError::InvalidColor(format!("Unknown function: {}", function_name))),
        }
    }

    /// Parse RGB parameters
    fn parse_rgb_params(&self, params: &[&str]) -> Result<(u8, u8, u8)> {
        if params.len() != 3 {
            return Err(ColorError::InvalidColor("Expected 3 RGB parameters".to_string()));
        }

        let r = self.parse_color_component(params[0])?;
        let g = self.parse_color_component(params[1])?;
        let b = self.parse_color_component(params[2])?;

        Ok((r, g, b))
    }

    /// Parse HSL parameters and convert to RGB
    fn parse_hsl_params(&self, params: &[&str]) -> Result<(u8, u8, u8)> {
        if params.len() != 3 {
            return Err(ColorError::InvalidColor("Expected 3 HSL parameters".to_string()));
        }

        let h = f32::from_str(params[0].trim())
            .map_err(|_| ColorError::InvalidColor("Invalid hue value".to_string()))?;
        let s = self.parse_percentage(params[1])?;
        let l = self.parse_percentage(params[2])?;

        // Normalize hue to 0-1 range
        let h_norm = (((h % 360.0) + 360.0) % 360.0) / 360.0;

        // Convert HSL to RGB
        let (r, g, b) = self.hsl_to_rgb(h_norm, s, l);
        Ok((r, g, b))
    }

    /// Parse color component (0-255 or percentage)
    fn parse_color_component(&self, value: &str) -> Result<u8> {
        let value = value.trim();
        
        if value.ends_with('%') {
            let percentage_str = &value[..value.len() - 1];
            let percentage = f32::from_str(percentage_str)
                .map_err(|_| ColorError::InvalidColor("Invalid percentage".to_string()))?;
            Ok(((percentage / 100.0 * 255.0).round().clamp(0.0, 255.0)) as u8)
        } else {
            let int_val = u32::from_str(value)
                .map_err(|_| ColorError::InvalidColor("Invalid color component".to_string()))?;
            Ok((int_val.clamp(0, 255)) as u8)
        }
    }

    /// Parse percentage value (returns 0.0-1.0)
    fn parse_percentage(&self, value: &str) -> Result<f32> {
        let value = value.trim();
        
        if value.ends_with('%') {
            let percentage_str = &value[..value.len() - 1];
            let percentage = f32::from_str(percentage_str)
                .map_err(|_| ColorError::InvalidColor("Invalid percentage".to_string()))?;
            Ok((percentage / 100.0).clamp(0.0, 1.0))
        } else {
            // Allow float values without % for convenience
            let float_val = f32::from_str(value)
                .map_err(|_| ColorError::InvalidColor("Invalid percentage".to_string()))?;
            Ok(float_val.clamp(0.0, 1.0))
        }
    }

    /// Parse alpha value (0.0-1.0)
    fn parse_alpha(&self, value: &str) -> Result<f32> {
        let value = value.trim();
        let alpha = f32::from_str(value)
            .map_err(|_| ColorError::InvalidColor("Invalid alpha value".to_string()))?;
        Ok(alpha.clamp(0.0, 1.0))
    }

    /// Convert HSL to RGB
    fn hsl_to_rgb(&self, h: f32, s: f32, l: f32) -> (u8, u8, u8) {
        let m2 = if l <= 0.5 {
            l * (s + 1.0)
        } else {
            l + s - l * s
        };

        let m1 = l * 2.0 - m2;

        let r = (self.hue_to_rgb(m1, m2, h + 1.0 / 3.0) * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (self.hue_to_rgb(m1, m2, h) * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (self.hue_to_rgb(m1, m2, h - 1.0 / 3.0) * 255.0).round().clamp(0.0, 255.0) as u8;

        (r, g, b)
    }

    /// Helper function for HSL to RGB conversion
    fn hue_to_rgb(&self, m1: f32, m2: f32, mut h: f32) -> f32 {
        if h < 0.0 {
            h += 1.0;
        } else if h > 1.0 {
            h -= 1.0;
        }

        if h * 6.0 < 1.0 {
            m1 + (m2 - m1) * h * 6.0
        } else if h * 2.0 < 1.0 {
            m2
        } else if h * 3.0 < 2.0 {
            m1 + (m2 - m1) * (2.0 / 3.0 - h) * 6.0
        } else {
            m1
        }
    }

    /// Create named colors map
    fn create_named_colors() -> HashMap<String, (u8, u8, u8)> {
        let mut colors = HashMap::new();
        
        // CSS3 named colors
        colors.insert("aliceblue".to_string(), (240, 248, 255));
        colors.insert("antiquewhite".to_string(), (250, 235, 215));
        colors.insert("aqua".to_string(), (0, 255, 255));
        colors.insert("aquamarine".to_string(), (127, 255, 212));
        colors.insert("azure".to_string(), (240, 255, 255));
        colors.insert("beige".to_string(), (245, 245, 220));
        colors.insert("bisque".to_string(), (255, 228, 196));
        colors.insert("black".to_string(), (0, 0, 0));
        colors.insert("blanchedalmond".to_string(), (255, 235, 205));
        colors.insert("blue".to_string(), (0, 0, 255));
        colors.insert("blueviolet".to_string(), (138, 43, 226));
        colors.insert("brown".to_string(), (165, 42, 42));
        colors.insert("burlywood".to_string(), (222, 184, 135));
        colors.insert("cadetblue".to_string(), (95, 158, 160));
        colors.insert("chartreuse".to_string(), (127, 255, 0));
        colors.insert("chocolate".to_string(), (210, 105, 30));
        colors.insert("coral".to_string(), (255, 127, 80));
        colors.insert("cornflowerblue".to_string(), (100, 149, 237));
        colors.insert("cornsilk".to_string(), (255, 248, 220));
        colors.insert("crimson".to_string(), (220, 20, 60));
        colors.insert("cyan".to_string(), (0, 255, 255));
        colors.insert("darkblue".to_string(), (0, 0, 139));
        colors.insert("darkcyan".to_string(), (0, 139, 139));
        colors.insert("darkgoldenrod".to_string(), (184, 134, 11));
        colors.insert("darkgray".to_string(), (169, 169, 169));
        colors.insert("darkgrey".to_string(), (169, 169, 169));
        colors.insert("darkgreen".to_string(), (0, 100, 0));
        colors.insert("darkkhaki".to_string(), (189, 183, 107));
        colors.insert("darkmagenta".to_string(), (139, 0, 139));
        colors.insert("darkolivegreen".to_string(), (85, 107, 47));
        colors.insert("darkorange".to_string(), (255, 140, 0));
        colors.insert("darkorchid".to_string(), (153, 50, 204));
        colors.insert("darkred".to_string(), (139, 0, 0));
        colors.insert("darksalmon".to_string(), (233, 150, 122));
        colors.insert("darkseagreen".to_string(), (143, 188, 143));
        colors.insert("darkslateblue".to_string(), (72, 61, 139));
        colors.insert("darkslategray".to_string(), (47, 79, 79));
        colors.insert("darkslategrey".to_string(), (47, 79, 79));
        colors.insert("darkturquoise".to_string(), (0, 206, 209));
        colors.insert("darkviolet".to_string(), (148, 0, 211));
        colors.insert("deeppink".to_string(), (255, 20, 147));
        colors.insert("deepskyblue".to_string(), (0, 191, 255));
        colors.insert("dimgray".to_string(), (105, 105, 105));
        colors.insert("dimgrey".to_string(), (105, 105, 105));
        colors.insert("dodgerblue".to_string(), (30, 144, 255));
        colors.insert("firebrick".to_string(), (178, 34, 34));
        colors.insert("floralwhite".to_string(), (255, 250, 240));
        colors.insert("forestgreen".to_string(), (34, 139, 34));
        colors.insert("fuchsia".to_string(), (255, 0, 255));
        colors.insert("gainsboro".to_string(), (220, 220, 220));
        colors.insert("ghostwhite".to_string(), (248, 248, 255));
        colors.insert("gold".to_string(), (255, 215, 0));
        colors.insert("goldenrod".to_string(), (218, 165, 32));
        colors.insert("gray".to_string(), (128, 128, 128));
        colors.insert("grey".to_string(), (128, 128, 128));
        colors.insert("green".to_string(), (0, 128, 0));
        colors.insert("greenyellow".to_string(), (173, 255, 47));
        colors.insert("honeydew".to_string(), (240, 255, 240));
        colors.insert("hotpink".to_string(), (255, 105, 180));
        colors.insert("indianred".to_string(), (205, 92, 92));
        colors.insert("indigo".to_string(), (75, 0, 130));
        colors.insert("ivory".to_string(), (255, 255, 240));
        colors.insert("khaki".to_string(), (240, 230, 140));
        colors.insert("lavender".to_string(), (230, 230, 250));
        colors.insert("lavenderblush".to_string(), (255, 240, 245));
        colors.insert("lawngreen".to_string(), (124, 252, 0));
        colors.insert("lemonchiffon".to_string(), (255, 250, 205));
        colors.insert("lightblue".to_string(), (173, 216, 230));
        colors.insert("lightcoral".to_string(), (240, 128, 128));
        colors.insert("lightcyan".to_string(), (224, 255, 255));
        colors.insert("lightgoldenrodyellow".to_string(), (250, 250, 210));
        colors.insert("lightgray".to_string(), (211, 211, 211));
        colors.insert("lightgrey".to_string(), (211, 211, 211));
        colors.insert("lightgreen".to_string(), (144, 238, 144));
        colors.insert("lightpink".to_string(), (255, 182, 193));
        colors.insert("lightsalmon".to_string(), (255, 160, 122));
        colors.insert("lightseagreen".to_string(), (32, 178, 170));
        colors.insert("lightskyblue".to_string(), (135, 206, 250));
        colors.insert("lightslategray".to_string(), (119, 136, 153));
        colors.insert("lightslategrey".to_string(), (119, 136, 153));
        colors.insert("lightsteelblue".to_string(), (176, 196, 222));
        colors.insert("lightyellow".to_string(), (255, 255, 224));
        colors.insert("lime".to_string(), (0, 255, 0));
        colors.insert("limegreen".to_string(), (50, 205, 50));
        colors.insert("linen".to_string(), (250, 240, 230));
        colors.insert("magenta".to_string(), (255, 0, 255));
        colors.insert("maroon".to_string(), (128, 0, 0));
        colors.insert("mediumaquamarine".to_string(), (102, 205, 170));
        colors.insert("mediumblue".to_string(), (0, 0, 205));
        colors.insert("mediumorchid".to_string(), (186, 85, 211));
        colors.insert("mediumpurple".to_string(), (147, 112, 219));
        colors.insert("mediumseagreen".to_string(), (60, 179, 113));
        colors.insert("mediumslateblue".to_string(), (123, 104, 238));
        colors.insert("mediumspringgreen".to_string(), (0, 250, 154));
        colors.insert("mediumturquoise".to_string(), (72, 209, 204));
        colors.insert("mediumvioletred".to_string(), (199, 21, 133));
        colors.insert("midnightblue".to_string(), (25, 25, 112));
        colors.insert("mintcream".to_string(), (245, 255, 250));
        colors.insert("mistyrose".to_string(), (255, 228, 225));
        colors.insert("moccasin".to_string(), (255, 228, 181));
        colors.insert("navajowhite".to_string(), (255, 222, 173));
        colors.insert("navy".to_string(), (0, 0, 128));
        colors.insert("oldlace".to_string(), (253, 245, 230));
        colors.insert("olive".to_string(), (128, 128, 0));
        colors.insert("olivedrab".to_string(), (107, 142, 35));
        colors.insert("orange".to_string(), (255, 165, 0));
        colors.insert("orangered".to_string(), (255, 69, 0));
        colors.insert("orchid".to_string(), (218, 112, 214));
        colors.insert("palegoldenrod".to_string(), (238, 232, 170));
        colors.insert("palegreen".to_string(), (152, 251, 152));
        colors.insert("paleturquoise".to_string(), (175, 238, 238));
        colors.insert("palevioletred".to_string(), (219, 112, 147));
        colors.insert("papayawhip".to_string(), (255, 239, 213));
        colors.insert("peachpuff".to_string(), (255, 218, 185));
        colors.insert("peru".to_string(), (205, 133, 63));
        colors.insert("pink".to_string(), (255, 192, 203));
        colors.insert("plum".to_string(), (221, 160, 221));
        colors.insert("powderblue".to_string(), (176, 224, 230));
        colors.insert("purple".to_string(), (128, 0, 128));
        colors.insert("rebeccapurple".to_string(), (102, 51, 153));
        colors.insert("red".to_string(), (255, 0, 0));
        colors.insert("rosybrown".to_string(), (188, 143, 143));
        colors.insert("royalblue".to_string(), (65, 105, 225));
        colors.insert("saddlebrown".to_string(), (139, 69, 19));
        colors.insert("salmon".to_string(), (250, 128, 114));
        colors.insert("sandybrown".to_string(), (244, 164, 96));
        colors.insert("seagreen".to_string(), (46, 139, 87));
        colors.insert("seashell".to_string(), (255, 245, 238));
        colors.insert("sienna".to_string(), (160, 82, 45));
        colors.insert("silver".to_string(), (192, 192, 192));
        colors.insert("skyblue".to_string(), (135, 206, 235));
        colors.insert("slateblue".to_string(), (106, 90, 205));
        colors.insert("slategray".to_string(), (112, 128, 144));
        colors.insert("slategrey".to_string(), (112, 128, 144));
        colors.insert("snow".to_string(), (255, 250, 250));
        colors.insert("springgreen".to_string(), (0, 255, 127));
        colors.insert("steelblue".to_string(), (70, 130, 180));
        colors.insert("tan".to_string(), (210, 180, 140));
        colors.insert("teal".to_string(), (0, 128, 128));
        colors.insert("thistle".to_string(), (216, 191, 216));
        colors.insert("tomato".to_string(), (255, 99, 71));
        colors.insert("turquoise".to_string(), (64, 224, 208));
        colors.insert("violet".to_string(), (238, 130, 238));
        colors.insert("wheat".to_string(), (245, 222, 179));
        colors.insert("white".to_string(), (255, 255, 255));
        colors.insert("whitesmoke".to_string(), (245, 245, 245));
        colors.insert("yellow".to_string(), (255, 255, 0));
        colors.insert("yellowgreen".to_string(), (154, 205, 50));

        colors
    }
}

impl Default for CssColorParser {
    fn default() -> Self {
        Self::new()
    }
}
