//! Algorithm implementations for color distance calculations
//!
//! This module contains the actual implementations of various color distance
//! algorithms, all using pure functional programming patterns.

use super::types::{DistanceAlgorithm, ValidationError, ValidatedLab};
use crate::config::algorithm_constants;
use std::str::FromStr;

impl DistanceAlgorithm {
    /// Smart constructor from string with validation
    ///
    /// Supports multiple name formats for user-friendly parsing:
    /// - "delta_e_76", "deltae76", "cie76", "de76" -> DeltaE76
    /// - "delta_e_2000", "deltae2000", "ciede2000", "de2000" -> DeltaE2000  
    /// - "euclidean", "euclidean_lab", "lab" -> EuclideanLab
    /// - "lch" -> Lch
    pub fn from_validated_str(s: &str) -> Result<Self, ValidationError> {
        // Pre-validation
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err(ValidationError::EmptyAlgorithmName);
        }
        
        if trimmed.len() > 50 {
            return Err(ValidationError::AlgorithmNameTooLong(trimmed.len()));
        }

        // Normalize input: lowercase, replace spaces/hyphens with underscores
        let normalized = trimmed
            .to_lowercase()
            .replace([' ', '-'], "_");

        // Match against known algorithm names
        match normalized.as_str() {
            // Delta E 76 variants
            "delta_e_76" | "deltae76" | "cie76" | "de76" | "delta_e76" => Ok(Self::DeltaE76),
            
            // Delta E 2000 variants
            "delta_e_2000" | "deltae2000" | "ciede2000" | "de2000" | "delta_e2000" => Ok(Self::DeltaE2000),
            
            // Euclidean Lab variants
            "euclidean" | "euclidean_lab" | "lab" => Ok(Self::EuclideanLab),
            
            // LCH variants  
            "lch" => Ok(Self::Lch),
            
            _ => Err(ValidationError::UnknownAlgorithm(s.to_string())),
        }
    }

    /// Validate algorithm is suitable for performance requirements
    pub fn validate_performance(self, require_fast: bool) -> Result<Self, ValidationError> {
        if require_fast && !self.is_fast() {
            Err(ValidationError::AlgorithmTooSlow(self))
        } else {
            Ok(self)
        }
    }

    /// Calculate distance between two validated LAB colors
    ///
    /// This is the main dispatch function that calls the appropriate algorithm.
    /// All calculations are pure functions with no side effects.
    #[must_use]
    pub fn calculate_distance(self, lab1: ValidatedLab, lab2: ValidatedLab) -> f64 {
        match self {
            Self::DeltaE76 => calculate_delta_e_76(lab1, lab2),
            Self::DeltaE2000 => calculate_delta_e_2000(lab1, lab2),
            Self::EuclideanLab => calculate_euclidean_lab(lab1, lab2),
            Self::Lch => calculate_lch_distance(lab1, lab2),
        }
    }

    /// Calculate multiple distances efficiently
    ///
    /// Optimized for batch processing - avoids repeated algorithm dispatch
    #[must_use]
    pub fn calculate_distances(self, pairs: &[(ValidatedLab, ValidatedLab)]) -> Vec<f64> {
        pairs.iter().map(|(lab1, lab2)| self.calculate_distance(*lab1, *lab2)).collect()
    }

    /// Calculate distance matrix for a set of colors
    ///
    /// Returns triangular matrix (upper triangle) for efficiency
    #[must_use]
    pub fn calculate_distance_matrix(self, colors: &[ValidatedLab]) -> Vec<Vec<f64>> {
        let n = colors.len();
        let mut matrix = Vec::with_capacity(n);
        
        for i in 0..n {
            let mut row = Vec::with_capacity(n - i);
            for j in i..n {
                if i == j {
                    row.push(0.0); // Distance from color to itself is 0
                } else {
                    row.push(self.calculate_distance(colors[i], colors[j]));
                }
            }
            matrix.push(row);
        }
        
        matrix
    }

    /// Find closest color from a set to a target color
    ///
    /// Returns (index, distance) of the closest match
    #[must_use]
    pub fn find_closest(self, target: ValidatedLab, candidates: &[ValidatedLab]) -> Option<(usize, f64)> {
        candidates
            .iter()
            .enumerate()
            .map(|(i, &candidate)| (i, self.calculate_distance(target, candidate)))
            .min_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap_or(std::cmp::Ordering::Equal))
    }
}

impl FromStr for DistanceAlgorithm {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_validated_str(s)
    }
}

impl std::fmt::Display for DistanceAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Delta E 76 (CIE76) distance calculation
///
/// Pure function implementing the original 1976 CIE color difference formula.
/// Fast but less perceptually accurate than newer methods.
///
/// Formula: √((ΔL)² + (Δa)² + (Δb)²)
#[must_use]
fn calculate_delta_e_76(lab1: ValidatedLab, lab2: ValidatedLab) -> f64 {
    let dl = (lab1.l() - lab2.l()) as f64;
    let da = (lab1.a() - lab2.a()) as f64;
    let db = (lab1.b() - lab2.b()) as f64;
    
    (dl * dl + da * da + db * db).sqrt()
}

/// Delta E 2000 (CIEDE2000) distance calculation
///
/// Pure function implementing the CIEDE2000 color difference formula.
/// Most perceptually accurate but computationally more expensive.
/// 
/// This is a simplified implementation focusing on correctness.
#[must_use]
fn calculate_delta_e_2000(lab1: ValidatedLab, lab2: ValidatedLab) -> f64 {
    let l1 = lab1.l() as f64;
    let a1 = lab1.a() as f64;
    let b1 = lab1.b() as f64;
    let l2 = lab2.l() as f64;
    let a2 = lab2.a() as f64;
    let b2 = lab2.b() as f64;

    // Calculate intermediate values
    let dl = l2 - l1;
    let da = a2 - a1;
    let db = b2 - b1;

    // Calculate C (chroma) values
    let c1 = (a1 * a1 + b1 * b1).sqrt();
    let c2 = (a2 * a2 + b2 * b2).sqrt();
    let dc = c2 - c1;

    // Calculate H (hue) values  
    let dh_squared = da * da + db * db - dc * dc;
    let dh = if dh_squared > 0.0 { dh_squared.sqrt() } else { 0.0 };

    // Weighting functions (simplified - full CIEDE2000 has more complex weighting)
    let l_avg = (l1 + l2) / 2.0;
    let c_avg = (c1 + c2) / 2.0;

    // Lightness weighting
    let sl = 1.0 + (algorithm_constants::DELTA_E_LIGHTNESS_FACTOR * (l_avg - algorithm_constants::DELTA_E_LIGHTNESS_OFFSET).powi(2)) / (algorithm_constants::DELTA_E_LIGHTNESS_DENOMINATOR_OFFSET + (l_avg - algorithm_constants::DELTA_E_LIGHTNESS_OFFSET).powi(2)).sqrt();
    
    // Chroma weighting  
    let sc = 1.0 + algorithm_constants::DELTA_E_CHROMA_FACTOR * c_avg;
    
    // Hue weighting
    let sh = 1.0 + algorithm_constants::DELTA_E_HUE_FACTOR * c_avg;

    // Parametric factors (standard values)
    let kl = algorithm_constants::DELTA_E_PARAMETRIC_FACTOR;
    let kc = algorithm_constants::DELTA_E_PARAMETRIC_FACTOR; 
    let kh = algorithm_constants::DELTA_E_PARAMETRIC_FACTOR;

    // Final Delta E 2000 calculation
    let delta_l = dl / (kl * sl);
    let delta_c = dc / (kc * sc);
    let delta_h = dh / (kh * sh);

    (delta_l * delta_l + delta_c * delta_c + delta_h * delta_h).sqrt()
}

/// Euclidean distance in LAB space
///
/// Pure function implementing simple Euclidean distance.
/// Fast but not perceptually uniform - same as Delta E 76.
#[must_use]
fn calculate_euclidean_lab(lab1: ValidatedLab, lab2: ValidatedLab) -> f64 {
    // This is actually identical to Delta E 76
    calculate_delta_e_76(lab1, lab2)
}

/// LCH Color Space distance calculation
///
/// Pure function implementing distance in cylindrical LCH color space.
/// Separates lightness from chroma and provides better perceptual uniformity.
#[must_use]
fn calculate_lch_distance(lab1: ValidatedLab, lab2: ValidatedLab) -> f64 {
    let l1 = lab1.l() as f64;
    let a1 = lab1.a() as f64;
    let b1 = lab1.b() as f64;
    let l2 = lab2.l() as f64;
    let a2 = lab2.a() as f64;
    let b2 = lab2.b() as f64;

    // Convert LAB to LCH (Lightness, Chroma, Hue)
    let c1 = (a1 * a1 + b1 * b1).sqrt();
    let h1 = b1.atan2(a1);
    
    let c2 = (a2 * a2 + b2 * b2).sqrt();
    let h2 = b2.atan2(a2);

    // Calculate differences
    let dl = l2 - l1;
    let dc = c2 - c1;
    
    // Hue difference (handle circular nature)
    let mut dh = h2 - h1;
    if dh > std::f64::consts::PI {
        dh -= 2.0 * std::f64::consts::PI;
    } else if dh < -std::f64::consts::PI {
        dh += 2.0 * std::f64::consts::PI;
    }

    // Hue difference in chroma units
    let dh_chroma = 2.0 * (c1 * c2).sqrt() * (dh / 2.0).sin();

    // LCH distance calculation
    (dl * dl + dc * dc + dh_chroma * dh_chroma).sqrt()
}

/// Functional composition helpers for algorithm chaining and filtering
/// Filter algorithms by performance characteristics
#[must_use]
pub fn filter_fast_algorithms() -> Vec<DistanceAlgorithm> {
    DistanceAlgorithm::all()
        .into_iter()
        .filter(|alg| alg.is_fast())
        .collect()
}

/// Filter algorithms by perceptual accuracy
#[must_use]
pub fn filter_perceptual_algorithms() -> Vec<DistanceAlgorithm> {
    DistanceAlgorithm::all()
        .into_iter()
        .filter(|alg| alg.is_perceptually_accurate())
        .collect()
}

/// Get recommended algorithm based on requirements
#[must_use]
pub fn recommend_algorithm(require_fast: bool, require_perceptual: bool) -> Option<DistanceAlgorithm> {
    match (require_fast, require_perceptual) {
        (true, true) => None, // No algorithm is both fast AND highly perceptual
        (true, false) => Some(DistanceAlgorithm::DeltaE76), // Fast and adequate
        (false, true) => Some(DistanceAlgorithm::DeltaE2000), // Most perceptual
        (false, false) => Some(DistanceAlgorithm::EuclideanLab), // Simple default
    }
}

/// Algorithm comparison helper - compare multiple algorithms on same color pair
#[must_use]
pub fn compare_algorithms(lab1: ValidatedLab, lab2: ValidatedLab) -> Vec<(DistanceAlgorithm, f64)> {
    DistanceAlgorithm::all()
        .into_iter()
        .map(|alg| (alg, alg.calculate_distance(lab1, lab2)))
        .collect()
}
