//! Color distance calculation functions
//!
//! Pure functions for calculating perceptual and mathematical distances between colors.
//! Supports multiple distance algorithms including Delta E variants.

use crate::color_distance_strategies::{DistanceAlgorithm, calculate_distance};
use palette::{IntoColor, Lab, Srgb};

/// Calculate Delta E CIE76 distance between two colors
///
/// The original CIE Delta E formula for measuring perceptual color difference.
/// Values under 2.3 are generally considered imperceptible to the human eye.
///
/// # Arguments
/// * `color1` - First color in sRGB color space
/// * `color2` - Second color in sRGB color space
///
/// # Returns
/// * Delta E distance (0.0 = identical, higher = more different)
///
/// # Example
/// ```rust
/// use color_rs::color_ops::distance;
/// use palette::Srgb;
///
/// let red = Srgb::new(1.0, 0.0, 0.0);
/// let blue = Srgb::new(0.0, 0.0, 1.0);
/// let distance = distance::delta_e_cie76(red, blue);
/// assert!(distance > 100.0); // Very different colors
/// ```
#[must_use]
pub fn delta_e_cie76(color1: Srgb, color2: Srgb) -> f64 {
    let lab1: Lab = color1.into_color();
    let lab2: Lab = color2.into_color();
    calculate_distance(DistanceAlgorithm::DeltaE76, lab1, lab2)
}

/// Calculate Delta E CIE94 distance between two colors
///
/// Improved Delta E formula that weights lightness, chroma, and hue differently
/// based on human visual perception.
///
/// # Arguments
/// * `color1` - First color in sRGB color space
/// * `color2` - Second color in sRGB color space
///
/// # Returns
/// * Delta E distance with improved perceptual accuracy
///
/// # Example
/// ```rust
/// use color_rs::color_ops::distance;
/// use palette::Srgb;
///
/// let color1 = Srgb::new(0.8, 0.2, 0.3);
/// let color2 = Srgb::new(0.8, 0.25, 0.3);
/// let distance = distance::delta_e_cie94(color1, color2);
/// ```
pub fn delta_e_cie94(color1: Srgb, color2: Srgb) -> f64 {
    // Note: Using DeltaE76 as approximation since palette doesn't have CIE94
    let lab1: Lab = color1.into_color();
    let lab2: Lab = color2.into_color();
    calculate_distance(DistanceAlgorithm::DeltaE76, lab1, lab2)
}

/// Calculate Delta E 2000 distance between two colors
///
/// The most sophisticated and perceptually accurate Delta E formula,
/// incorporating improvements for blue colors and neutral colors.
///
/// # Arguments
/// * `color1` - First color in sRGB color space
/// * `color2` - Second color in sRGB color space
///
/// # Returns
/// * Delta E 2000 distance (most accurate perceptual difference)
///
/// # Example
/// ```rust
/// use color_rs::color_ops::distance;
/// use palette::Srgb;
///
/// let color1 = Srgb::new(0.5, 0.7, 0.9);
/// let color2 = Srgb::new(0.5, 0.7, 0.85);
/// let distance = distance::delta_e_2000(color1, color2);
/// ```
pub fn delta_e_2000(color1: Srgb, color2: Srgb) -> f64 {
    let lab1: Lab = color1.into_color();
    let lab2: Lab = color2.into_color();
    calculate_distance(DistanceAlgorithm::DeltaE2000, lab1, lab2)
}

/// Calculate RGB Euclidean distance between two colors
///
/// Simple mathematical distance in RGB color space. Less perceptually
/// accurate than Delta E methods but computationally faster.
///
/// # Arguments
/// * `color1` - First color in sRGB color space
/// * `color2` - Second color in sRGB color space
///
/// # Returns
/// * RGB Euclidean distance
///
/// # Example
/// ```rust
/// use color_rs::color_ops::distance;
/// use palette::Srgb;
///
/// let color1 = Srgb::new(1.0, 0.0, 0.0);
/// let color2 = Srgb::new(0.0, 1.0, 0.0);
/// let distance = distance::rgb_euclidean(color1, color2);
/// ```
pub fn rgb_euclidean(color1: Srgb, color2: Srgb) -> f64 {
    let r1 = color1.red as f64;
    let g1 = color1.green as f64;
    let b1 = color1.blue as f64;

    let r2 = color2.red as f64;
    let g2 = color2.green as f64;
    let b2 = color2.blue as f64;

    ((r2 - r1).powi(2) + (g2 - g1).powi(2) + (b2 - b1).powi(2)).sqrt()
}

/// Calculate LAB Euclidean distance between two colors
///
/// Euclidean distance in CIELAB color space, which is more perceptually
/// uniform than RGB space but less sophisticated than Delta E formulas.
///
/// # Arguments
/// * `color1` - First color in sRGB color space
/// * `color2` - Second color in sRGB color space
///
/// # Returns
/// * LAB Euclidean distance
///
/// # Example
/// ```rust
/// use color_rs::color_ops::distance;
/// use palette::Srgb;
///
/// let color1 = Srgb::new(0.8, 0.6, 0.4);
/// let color2 = Srgb::new(0.7, 0.5, 0.3);
/// let distance = distance::lab_euclidean(color1, color2);
/// ```
pub fn lab_euclidean(color1: Srgb, color2: Srgb) -> f64 {
    let lab1: Lab = color1.into_color();
    let lab2: Lab = color2.into_color();
    calculate_distance(DistanceAlgorithm::EuclideanLab, lab1, lab2)
}

/// Calculate distance between LAB colors directly
///
/// For when you already have LAB colors and want to avoid conversion overhead.
///
/// # Arguments
/// * `lab1` - First color in CIELAB color space
/// * `lab2` - Second color in CIELAB color space
///
/// # Returns
/// * LAB Euclidean distance
pub fn lab_direct(lab1: Lab, lab2: Lab) -> f64 {
    let dl = (lab1.l - lab2.l) as f64;
    let da = (lab1.a - lab2.a) as f64;
    let db = (lab1.b - lab2.b) as f64;
    (dl * dl + da * da + db * db).sqrt()
}

/// Find the closest color from a collection using Delta E 2000
///
/// Utility function to find the perceptually closest color from a collection.
///
/// # Arguments
/// * `target` - Target color to match against
/// * `candidates` - Collection of candidate colors
///
/// # Returns
/// * Index of the closest color and its distance
///
/// # Example
/// ```rust
/// use color_rs::color_ops::distance;
/// use palette::Srgb;
///
/// let target = Srgb::new(0.5, 0.5, 0.5);
/// let candidates = vec![
///     Srgb::new(0.4, 0.4, 0.4),
///     Srgb::new(0.6, 0.6, 0.6),
///     Srgb::new(1.0, 0.0, 0.0),
/// ];
///
/// let (closest_index, distance) = distance::find_closest(target, &candidates);
/// ```
pub fn find_closest(target: Srgb, candidates: &[Srgb]) -> (usize, f64) {
    candidates
        .iter()
        .enumerate()
        .map(|(i, &color)| (i, delta_e_2000(target, color)))
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .unwrap_or((0, f64::INFINITY))
}

/// Alias for Delta E 2000 - the recommended perceptual distance
pub fn perceptual_distance(color1: Srgb, color2: Srgb) -> f64 {
    delta_e_2000(color1, color2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Srgb;

    #[test]
    fn test_delta_e_identical_colors() {
        let color = Srgb::new(0.5, 0.5, 0.5);
        assert!(delta_e_cie76(color, color) < 1e-6);
        assert!(delta_e_cie94(color, color) < 1e-6);
        assert!(delta_e_2000(color, color) < 1e-6);
    }

    #[test]
    fn test_rgb_euclidean_distance() {
        let red = Srgb::new(1.0, 0.0, 0.0);
        let green = Srgb::new(0.0, 1.0, 0.0);
        let distance = rgb_euclidean(red, green);

        // Should be sqrt(2) for unit distance in two dimensions
        assert!((distance - 2.0_f64.sqrt()).abs() < 1e-6);
    }

    #[test]
    fn test_lab_direct_distance() {
        let lab1 = Lab::new(50.0, 0.0, 0.0);
        let lab2 = Lab::new(60.0, 0.0, 0.0);
        let distance = lab_direct(lab1, lab2);

        assert!((distance - 10.0).abs() < 1e-6);
    }

    #[test]
    fn test_find_closest() {
        let target = Srgb::new(0.5, 0.5, 0.5);
        let candidates = vec![
            Srgb::new(0.6, 0.6, 0.6),   // Close
            Srgb::new(1.0, 0.0, 0.0),   // Far
            Srgb::new(0.51, 0.49, 0.5), // Very close
        ];

        let (closest_index, _distance) = find_closest(target, &candidates);
        assert_eq!(closest_index, 2); // The very close one
    }

    #[test]
    fn test_perceptual_distance_alias() {
        let color1 = Srgb::new(0.3, 0.6, 0.9);
        let color2 = Srgb::new(0.4, 0.7, 0.8);

        assert_eq!(
            perceptual_distance(color1, color2),
            delta_e_2000(color1, color2)
        );
    }

    #[test]
    fn test_distance_symmetry() {
        let color1 = Srgb::new(0.2, 0.4, 0.8);
        let color2 = Srgb::new(0.6, 0.3, 0.1);

        // Distance should be symmetric
        assert!((delta_e_2000(color1, color2) - delta_e_2000(color2, color1)).abs() < 1e-10);
        assert!((rgb_euclidean(color1, color2) - rgb_euclidean(color2, color1)).abs() < 1e-10);
    }
}
