//! Color Name Resolver
//!
//! Modernized and integrated version of color-name
//! Original: https://github.com/annymosse/color-name
//! Author: annymosse

use crate::color_utils::ColorUtils;

/// Color name resolver for finding closest color names
pub struct ColorNameResolver {
    colors: &'static [(&'static str, [u8; 3])],
}

impl ColorNameResolver {
    /// Create a new color name resolver
    pub fn new() -> Self {
        Self {
            colors: &CSS_COLOR_DATA,
        }
    }

    /// Find the closest color name for given RGB values using LAB color space distance
    pub fn find_closest_name(&self, rgb: [u8; 3]) -> String {
        let mut min_distance = f32::MAX;
        let mut closest_name = "unknown";

        // Convert input RGB to LAB for perceptually accurate comparison
        let input_lab = ColorUtils::rgb_to_lab(rgb);

        for &(name, color_rgb) in self.colors {
            let color_lab = ColorUtils::rgb_to_lab(color_rgb);
            let distance = ColorUtils::lab_distance(input_lab, color_lab);
            if distance < min_distance {
                min_distance = distance;
                closest_name = name;
            }
        }

        closest_name.to_string()
    }

    /// Get exact color name if it exists
    pub fn get_exact_name(&self, rgb: [u8; 3]) -> Option<String> {
        for &(name, color_rgb) in self.colors {
            if color_rgb == rgb {
                return Some(name.to_string());
            }
        }
        None
    }

    /// Get all available color names
    pub fn get_all_names(&self) -> Vec<String> {
        self.colors
            .iter()
            .map(|(name, _)| name.to_string())
            .collect()
    }

    /// Get color RGB by name (case insensitive)
    pub fn get_color_by_name(&self, name: &str) -> Option<[u8; 3]> {
        let name_lower = name.to_lowercase();
        for &(color_name, rgb) in self.colors {
            if color_name.to_lowercase() == name_lower {
                return Some(rgb);
            }
        }
        None
    }
}

impl Default for ColorNameResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Lab;

    #[test]
    fn test_lab_distance_calculation() {
        let _resolver = ColorNameResolver::new();

        // Test Delta E calculation with known values
        let red_lab = Lab::new(53.24, 80.09, 67.20);
        let blue_lab = Lab::new(32.30, 79.20, -107.86);

        // Distance should be perceptually meaningful using ImprovedCiede2000
        let distance = ColorUtils::lab_distance(red_lab, blue_lab);
        assert!(distance > 20.0); // Red and blue should be quite different (actual ~23)

        // Test identity (same color should have distance 0)
        let identity_distance = ColorUtils::lab_distance(red_lab, red_lab);
        assert!(identity_distance < 0.001);

        // Test symmetry
        let distance_ab = ColorUtils::lab_distance(red_lab, blue_lab);
        let distance_ba = ColorUtils::lab_distance(blue_lab, red_lab);
        assert!((distance_ab - distance_ba).abs() < 0.001);
    }

    #[test]
    fn test_rgb_to_lab_conversion() {
        let _resolver = ColorNameResolver::new();

        // Test known RGB to LAB conversion
        let red_lab = ColorUtils::rgb_to_lab([255, 0, 0]);
        assert!((red_lab.l - 53.24).abs() < 1.0); // Red lightness around 53
        assert!(red_lab.a > 70.0); // Positive a (green-red axis)
        assert!(red_lab.b > 60.0); // Positive b (blue-yellow axis)

        let black_lab = ColorUtils::rgb_to_lab([0, 0, 0]);
        assert!(black_lab.l < 1.0); // Black should have very low lightness

        let white_lab = ColorUtils::rgb_to_lab([255, 255, 255]);
        assert!(white_lab.l > 95.0); // White should have high lightness
    }

    #[test]
    fn test_find_closest_color() {
        let resolver = ColorNameResolver::new();

        // Test finding closest color to pure red
        let red_rgb = [255, 0, 0];
        let closest = resolver.find_closest_name(red_rgb);
        assert_eq!(closest, "red");

        // Test finding closest color to pure blue
        let blue_rgb = [0, 0, 255];
        let closest = resolver.find_closest_name(blue_rgb);
        assert_eq!(closest, "blue");

        // Test finding closest color to white
        let white_rgb = [255, 255, 255];
        let closest = resolver.find_closest_name(white_rgb);
        assert_eq!(closest, "white");
    }
}

/// Extended color database with 148 color names
/// Integrated from the color-name library - public for use across color_parser modules
pub static CSS_COLOR_DATA: &[(&str, [u8; 3])] = &[
    ("antiquewhite", [250, 235, 215]),
    ("aliceblue", [240, 248, 255]),
    ("aqua", [0, 255, 255]),
    ("aquamarine", [127, 255, 212]),
    ("azure", [240, 255, 255]),
    ("beige", [245, 245, 220]),
    ("bisque", [255, 228, 196]),
    ("black", [0, 0, 0]),
    ("blanchedalmond", [255, 235, 205]),
    ("blue", [0, 0, 255]),
    ("blueviolet", [138, 43, 226]),
    ("brown", [165, 42, 42]),
    ("burlywood", [222, 184, 135]),
    ("cadetblue", [95, 158, 160]),
    ("chartreuse", [127, 255, 0]),
    ("chocolate", [210, 105, 30]),
    ("coral", [255, 127, 80]),
    ("cornflowerblue", [100, 149, 237]),
    ("cornsilk", [255, 248, 220]),
    ("crimson", [220, 20, 60]),
    ("cyan", [0, 255, 255]),
    ("darkblue", [0, 0, 139]),
    ("darkcyan", [0, 139, 139]),
    ("darkgoldenrod", [184, 134, 11]),
    ("darkgray", [169, 169, 169]),
    ("darkgreen", [0, 100, 0]),
    ("darkgrey", [169, 169, 169]),
    ("darkkhaki", [189, 183, 107]),
    ("darkmagenta", [139, 0, 139]),
    ("darkolivegreen", [85, 107, 47]),
    ("darkorange", [255, 140, 0]),
    ("darkorchid", [153, 50, 204]),
    ("darkred", [139, 0, 0]),
    ("darksalmon", [233, 150, 122]),
    ("darkseagreen", [143, 188, 143]),
    ("darkslateblue", [72, 61, 139]),
    ("darkslategray", [47, 79, 79]),
    ("darkslategrey", [47, 79, 79]),
    ("darkturquoise", [0, 206, 209]),
    ("darkviolet", [148, 0, 211]),
    ("deeppink", [255, 20, 147]),
    ("deepskyblue", [0, 191, 255]),
    ("dimgray", [105, 105, 105]),
    ("dimgrey", [105, 105, 105]),
    ("dodgerblue", [30, 144, 255]),
    ("firebrick", [178, 34, 34]),
    ("floralwhite", [255, 250, 240]),
    ("forestgreen", [34, 139, 34]),
    ("fuchsia", [255, 0, 255]),
    ("gainsboro", [220, 220, 220]),
    ("ghostwhite", [248, 248, 255]),
    ("gold", [255, 215, 0]),
    ("goldenrod", [218, 165, 32]),
    ("gray", [128, 128, 128]),
    ("green", [0, 128, 0]),
    ("greenyellow", [173, 255, 47]),
    ("grey", [128, 128, 128]),
    ("honeydew", [240, 255, 240]),
    ("hotpink", [255, 105, 180]),
    ("indianred", [205, 92, 92]),
    ("indigo", [75, 0, 130]),
    ("ivory", [255, 255, 240]),
    ("khaki", [240, 230, 140]),
    ("lavender", [230, 230, 250]),
    ("lavenderblush", [255, 240, 245]),
    ("lawngreen", [124, 252, 0]),
    ("lemonchiffon", [255, 250, 205]),
    ("lightblue", [173, 216, 230]),
    ("lightcoral", [240, 128, 128]),
    ("lightcyan", [224, 255, 255]),
    ("lightgoldenrodyellow", [250, 250, 210]),
    ("lightgray", [211, 211, 211]),
    ("lightgreen", [144, 238, 144]),
    ("lightgrey", [211, 211, 211]),
    ("lightpink", [255, 182, 193]),
    ("lightsalmon", [255, 160, 122]),
    ("lightseagreen", [32, 178, 170]),
    ("lightskyblue", [135, 206, 250]),
    ("lightslategray", [119, 136, 153]),
    ("lightslategrey", [119, 136, 153]),
    ("lightsteelblue", [176, 196, 222]),
    ("lightyellow", [255, 255, 224]),
    ("lime", [0, 255, 0]),
    ("limegreen", [50, 205, 50]),
    ("linen", [250, 240, 230]),
    ("magenta", [255, 0, 255]),
    ("maroon", [128, 0, 0]),
    ("mediumaquamarine", [102, 205, 170]),
    ("mediumblue", [0, 0, 205]),
    ("mediumorchid", [186, 85, 211]),
    ("mediumpurple", [147, 112, 219]),
    ("mediumseagreen", [60, 179, 113]),
    ("mediumslateblue", [123, 104, 238]),
    ("mediumspringgreen", [0, 250, 154]),
    ("mediumturquoise", [72, 209, 204]),
    ("mediumvioletred", [199, 21, 133]),
    ("midnightblue", [25, 25, 112]),
    ("mintcream", [245, 255, 250]),
    ("mistyrose", [255, 228, 225]),
    ("moccasin", [255, 228, 181]),
    ("navajowhite", [255, 222, 173]),
    ("navy", [0, 0, 128]),
    ("oldlace", [253, 245, 230]),
    ("olive", [128, 128, 0]),
    ("olivedrab", [107, 142, 35]),
    ("orange", [255, 165, 0]),
    ("orangered", [255, 69, 0]),
    ("orchid", [218, 112, 214]),
    ("palegoldenrod", [238, 232, 170]),
    ("palegreen", [152, 251, 152]),
    ("paleturquoise", [175, 238, 238]),
    ("palevioletred", [219, 112, 147]),
    ("papayawhip", [255, 239, 213]),
    ("peachpuff", [255, 218, 185]),
    ("peru", [205, 133, 63]),
    ("pink", [255, 192, 203]),
    ("plum", [221, 160, 221]),
    ("powderblue", [176, 224, 230]),
    ("purple", [128, 0, 128]),
    ("rebeccapurple", [102, 51, 153]),
    ("red", [255, 0, 0]),
    ("rosybrown", [188, 143, 143]),
    ("royalblue", [65, 105, 225]),
    ("saddlebrown", [139, 69, 19]),
    ("salmon", [250, 128, 114]),
    ("sandybrown", [244, 164, 96]),
    ("seagreen", [46, 139, 87]),
    ("seashell", [255, 245, 238]),
    ("sienna", [160, 82, 45]),
    ("silver", [192, 192, 192]),
    ("skyblue", [135, 206, 235]),
    ("slateblue", [106, 90, 205]),
    ("slategray", [112, 128, 144]),
    ("slategrey", [112, 128, 144]),
    ("snow", [255, 250, 250]),
    ("springgreen", [0, 255, 127]),
    ("steelblue", [70, 130, 180]),
    ("tan", [210, 180, 140]),
    ("teal", [0, 128, 128]),
    ("thistle", [216, 191, 216]),
    ("tomato", [255, 99, 71]),
    ("turquoise", [64, 224, 208]),
    ("violet", [238, 130, 238]),
    ("wheat", [245, 222, 179]),
    ("white", [255, 255, 255]),
    ("whitesmoke", [245, 245, 245]),
    ("yellow", [255, 255, 0]),
    ("yellowgreen", [154, 205, 50]),
];
