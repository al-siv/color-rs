//! Gradient Stop Calculator
//!
//! This module provides functional gradient stop calculation using enum dispatch
//! and pure functional alternatives for stop positioning algorithms.

use super::calculator::GradientValue;
use crate::config::algorithm_constants;
use crate::error::Result;
use palette::Lab;

/// Gradient stop calculation strategy using enum dispatch
#[derive(Debug, Clone, PartialEq)]
pub enum StopCalculationStrategy {
    /// Equal spacing between stops
    EqualSpacing,
    /// Intelligent spacing with easing
    IntelligentSpacing { ease_in: f64, ease_out: f64 },
}

impl StopCalculationStrategy {
    /// Calculate stop positions using functional approach
    pub fn calculate_stops(&self, num_stops: usize) -> Result<Vec<f64>> {
        // Validation function
        if num_stops == 0 {
            return Err(crate::error::ColorError::InvalidGradient(
                "Number of stops must be positive".to_string(),
            ));
        }

        // Generate stops based on strategy
        let stops = match self {
            Self::EqualSpacing => generate_equal_spacing_stops(num_stops),
            Self::IntelligentSpacing { ease_in, ease_out } => {
                generate_intelligent_stops(num_stops, *ease_in, *ease_out)
            }
        };

        // Post-process (clamp and sort)
        let processed_stops = post_process_stops(stops);
        Ok(processed_stops)
    }

    /// Get strategy name
    pub const fn name(&self) -> &'static str {
        match self {
            Self::EqualSpacing => "Equal Spacing",
            Self::IntelligentSpacing { .. } => "Intelligent Spacing",
        }
    }
}

/// Pure function for equal spacing calculation
fn generate_equal_spacing_stops(num_stops: usize) -> Vec<f64> {
    if num_stops == 1 {
        return vec![0.5];
    }

    (0..num_stops)
        .map(|i| i as f64 / (num_stops - 1) as f64)
        .collect()
}

/// Pure function for intelligent spacing calculation
fn generate_intelligent_stops(num_stops: usize, ease_in: f64, ease_out: f64) -> Vec<f64> {
    if num_stops == 1 {
        return vec![0.5];
    }

    let mut stops = Vec::with_capacity(num_stops);
    
    for i in 0..num_stops {
        let linear_position = i as f64 / (num_stops - 1) as f64;
        
        // Apply easing transformation
        let eased_position = if linear_position <= algorithm_constants::BEZIER_TRANSITION_POINT {
            // First half: ease-in
            algorithm_constants::BEZIER_TRANSITION_POINT * ease_function(algorithm_constants::BEZIER_CALCULATION_FACTOR * linear_position, ease_in)
        } else {
            // Second half: ease-out
            algorithm_constants::BEZIER_TRANSITION_POINT + algorithm_constants::BEZIER_TRANSITION_POINT * ease_function(algorithm_constants::BEZIER_CALCULATION_FACTOR * (linear_position - algorithm_constants::BEZIER_TRANSITION_POINT), ease_out)
        };
        
        stops.push(eased_position);
    }
    
    stops
}

/// Pure easing function
fn ease_function(t: f64, ease_factor: f64) -> f64 {
    if ease_factor == 0.0 {
        return t; // Linear
    }
    
    // Cubic easing formula
    let normalized_ease = ease_factor.clamp(0.0, 1.0);
    let power = 1.0 + algorithm_constants::BEZIER_CALCULATION_FACTOR * normalized_ease;
    t.powf(power)
}

/// Pure function for post-processing stops
fn post_process_stops(mut stops: Vec<f64>) -> Vec<f64> {
    // Clamp all values to [0.0, 1.0]
    for stop in &mut stops {
        *stop = stop.clamp(0.0, 1.0);
    }
    
    // Ensure first is 0.0 and last is 1.0 for gradient endpoints
    if let Some(first) = stops.first_mut() {
        *first = 0.0;
    }
    if let Some(last) = stops.last_mut() {
        *last = 1.0;
    }
    
    stops
}

/// Gradient stop calculator
#[derive(Debug, Clone)]
pub struct GradientStopCalculator {
    strategy: StopCalculationStrategy,
}

impl GradientStopCalculator {
    /// Create calculator with intelligent stops
    pub fn with_intelligent_stops(ease_in: f64, ease_out: f64) -> Self {
        Self {
            strategy: StopCalculationStrategy::IntelligentSpacing { ease_in, ease_out },
        }
    }

    /// Create calculator with equal spacing
    pub fn with_equal_spacing() -> Self {
        Self {
            strategy: StopCalculationStrategy::EqualSpacing,
        }
    }

    /// Calculate stops using functional approach
    pub fn calculate_stops(&self, num_stops: usize) -> Result<Vec<f64>> {
        self.strategy.calculate_stops(num_stops)
    }

    /// Generate gradient values using functional composition
    pub fn generate_gradient_values(
        &self,
        start_lab: Lab,
        end_lab: Lab,
        num_stops: usize,
        start_position: u8,
        end_position: u8,
        easing_function: &crate::gradient::easing::EasingFunction,
    ) -> Result<Vec<GradientValue>> {
        if num_stops == 0 {
            return Ok(Vec::new());
        }

        // Generate stops using functional approach
        let stops = self.calculate_stops(num_stops)?;
        
        // Functional composition pipeline for value generation
        let gradient_values = stops
            .into_iter()
            .map(|stop| {
                // Apply easing function
                let eased_t = easing_function.ease(stop);
                
                // Interpolate color in LAB space
                let interpolated_lab = interpolate_lab(start_lab, end_lab, eased_t);
                
                // Calculate position
                let position_range = f64::from(end_position) - f64::from(start_position);
                let position = f64::from(start_position) + (stop * position_range);
                
                // Convert to gradient value
                create_gradient_value(interpolated_lab, position)
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(gradient_values)
    }
}

/// Pure function for LAB color interpolation
fn interpolate_lab(start: Lab, end: Lab, t: f64) -> Lab {
    Lab::new(
        start.l + (t as f32) * (end.l - start.l),
        start.a + (t as f32) * (end.a - start.a),
        start.b + (t as f32) * (end.b - start.b),
    )
}

/// Pure function for creating gradient values
fn create_gradient_value(lab: Lab, position: f64) -> Result<GradientValue> {
    use palette::{IntoColor, Srgb};
    
    let srgb: Srgb = lab.into_color();
    let rgb = [
        (srgb.red * 255.0).round() as u8,
        (srgb.green * 255.0).round() as u8,
        (srgb.blue * 255.0).round() as u8,
    ];
    
    // Calculate WCAG luminance
    let luminance = crate::color_ops::luminance::from_rgb((rgb[0], rgb[1], rgb[2]));
    
    Ok(GradientValue {
        position: format!("{}%", position.round() as u8),
        hex: format!("#{:02X}{:02X}{:02X}", rgb[0], rgb[1], rgb[2]),
        rgb: format!("rgb({}, {}, {})", rgb[0], rgb[1], rgb[2]),
        wcag_luminance: format!("{:.3}", luminance),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Lab;

    #[test]
    fn test_equal_spacing_calculation() {
        let strategy = StopCalculationStrategy::EqualSpacing;
        let stops = strategy.calculate_stops(5).unwrap();
        
        assert_eq!(stops.len(), 5);
        assert_eq!(stops[0], 0.0);
        assert_eq!(stops[4], 1.0);
        assert!((stops[2] - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_intelligent_spacing_calculation() {
        let strategy = StopCalculationStrategy::IntelligentSpacing {
            ease_in: 0.5,
            ease_out: 0.5,
        };
        let stops = strategy.calculate_stops(3).unwrap();
        
        assert_eq!(stops.len(), 3);
        assert_eq!(stops[0], 0.0);
        assert_eq!(stops[2], 1.0);
    }

    #[test]
    fn test_gradient_stop_calculator() {
        let calculator = GradientStopCalculator::with_equal_spacing();
        let stops = calculator.calculate_stops(3).unwrap();
        
        assert_eq!(stops, vec![0.0, 0.5, 1.0]);
    }

    #[test]
    fn test_lab_interpolation() {
        let start = Lab::new(50.0, 10.0, 20.0);
        let end = Lab::new(70.0, 30.0, 40.0);
        let mid = interpolate_lab(start, end, 0.5);
        
        assert!((mid.l - 60.0).abs() < 1e-6);
        assert!((mid.a - 20.0).abs() < 1e-6);
        assert!((mid.b - 30.0).abs() < 1e-6);
    }

    #[test]
    fn test_strategy_names() {
        assert_eq!(
            StopCalculationStrategy::EqualSpacing.name(),
            "Equal Spacing"
        );
        assert_eq!(
            StopCalculationStrategy::IntelligentSpacing {
                ease_in: 0.5,
                ease_out: 0.5
            }
            .name(),
            "Intelligent Spacing"
        );
    }

    #[test]
    fn test_validation_error() {
        let strategy = StopCalculationStrategy::EqualSpacing;
        let result = strategy.calculate_stops(0);
        
        assert!(result.is_err());
    }
}
