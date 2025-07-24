//! Contrast calculation algorithms using Template Method pattern
//!
//! This module implements various contrast calculation methods with a unified interface
//! using the Template Method pattern to define the algorithm structure.

use crate::error::Result;
use palette::{IntoColor, Lab, Srgb, color_difference::Ciede2000};

/// Contrast level classifications
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContrastLevel {
    High,
    Medium,
    Marginal,
    Low,
}

impl ContrastLevel {
    pub fn to_string(&self) -> &'static str {
        match self {
            ContrastLevel::High => "High",
            ContrastLevel::Medium => "Medium",
            ContrastLevel::Marginal => "Marginal",
            ContrastLevel::Low => "Low",
        }
    }

    pub fn from_wcag_ratio(ratio: f64) -> Self {
        if ratio >= 7.0 {
            ContrastLevel::High
        } else if ratio >= 4.5 {
            ContrastLevel::Medium
        } else if ratio >= 3.0 {
            ContrastLevel::Marginal
        } else {
            ContrastLevel::Low
        }
    }

    pub fn from_delta_e(delta_e: f64) -> Self {
        if delta_e >= 40.0 {
            ContrastLevel::High
        } else if delta_e >= 20.0 {
            ContrastLevel::Medium
        } else if delta_e >= 10.0 {
            ContrastLevel::Marginal
        } else {
            ContrastLevel::Low
        }
    }
}

/// Result of contrast analysis
#[derive(Debug, Clone)]
pub struct ContrastAnalysis {
    pub contrast_ratio: f64,
    pub contrast_level: ContrastLevel,
    pub algorithm_used: &'static str,
    pub meets_wcag_aa: bool,
    pub meets_wcag_aaa: bool,
    pub color1_luminance: f64,
    pub color2_luminance: f64,
}

/// Template method interface for contrast calculation algorithms
pub trait ContrastCalculationTemplate {
    /// Main template method defining the algorithm structure
    fn calculate_contrast(&self, color1: Srgb, color2: Srgb) -> Result<ContrastAnalysis> {
        // Template Method pattern - define algorithm skeleton
        self.validate_inputs(color1, color2)?;
        let luminance1 = self.calculate_luminance(color1);
        let luminance2 = self.calculate_luminance(color2);
        let contrast = self.compute_contrast_value(luminance1, luminance2);
        let level = self.determine_contrast_level(contrast);
        let analysis = self.create_analysis_result(contrast, level, luminance1, luminance2);
        self.post_process_result(analysis)
    }

    /// Validate input colors (hook method)
    fn validate_inputs(&self, _color1: Srgb, _color2: Srgb) -> Result<()> {
        Ok(()) // Default implementation
    }

    /// Calculate luminance for a color (abstract method)
    fn calculate_luminance(&self, color: Srgb) -> f64;

    /// Compute contrast value from luminances (abstract method)
    fn compute_contrast_value(&self, luminance1: f64, luminance2: f64) -> f64;

    /// Determine contrast level (abstract method)
    fn determine_contrast_level(&self, contrast: f64) -> ContrastLevel;

    /// Create analysis result (concrete method)
    fn create_analysis_result(
        &self,
        contrast: f64,
        level: ContrastLevel,
        lum1: f64,
        lum2: f64,
    ) -> ContrastAnalysis {
        ContrastAnalysis {
            contrast_ratio: contrast,
            contrast_level: level,
            algorithm_used: self.algorithm_name(),
            meets_wcag_aa: contrast >= 4.5,
            meets_wcag_aaa: contrast >= 7.0,
            color1_luminance: lum1,
            color2_luminance: lum2,
        }
    }

    /// Post-process the result (hook method)
    fn post_process_result(&self, analysis: ContrastAnalysis) -> Result<ContrastAnalysis> {
        Ok(analysis)
    }

    /// Get algorithm name (abstract method)
    fn algorithm_name(&self) -> &'static str;
}

/// WCAG 2.1 contrast calculation algorithm
pub struct WcagContrastCalculator;

impl ContrastCalculationTemplate for WcagContrastCalculator {
    fn calculate_luminance(&self, color: Srgb) -> f64 {
        // WCAG 2.1 relative luminance calculation
        let to_linear = |c: f32| -> f64 {
            let c = c as f64;
            if c <= 0.03928 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };

        let r = to_linear(color.red);
        let g = to_linear(color.green);
        let b = to_linear(color.blue);

        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    fn compute_contrast_value(&self, luminance1: f64, luminance2: f64) -> f64 {
        let lighter = luminance1.max(luminance2);
        let darker = luminance1.min(luminance2);
        (lighter + 0.05) / (darker + 0.05)
    }

    fn determine_contrast_level(&self, contrast: f64) -> ContrastLevel {
        ContrastLevel::from_wcag_ratio(contrast)
    }

    fn algorithm_name(&self) -> &'static str {
        "WCAG 2.1"
    }
}

/// LAB-based perceptual contrast calculator
pub struct LabContrastCalculator;

impl ContrastCalculationTemplate for LabContrastCalculator {
    fn calculate_luminance(&self, color: Srgb) -> f64 {
        let lab: Lab = color.into_color();
        lab.l as f64 / 100.0 // Normalize to 0-1 range
    }

    fn compute_contrast_value(&self, luminance1: f64, luminance2: f64) -> f64 {
        (luminance1 - luminance2).abs() * 100.0 // Scale to 0-100 range
    }

    fn determine_contrast_level(&self, contrast: f64) -> ContrastLevel {
        if contrast >= 50.0 {
            ContrastLevel::High
        } else if contrast >= 30.0 {
            ContrastLevel::Medium
        } else if contrast >= 15.0 {
            ContrastLevel::Marginal
        } else {
            ContrastLevel::Low
        }
    }

    fn algorithm_name(&self) -> &'static str {
        "LAB Perceptual"
    }
}

/// Delta E contrast calculator using CIEDE2000
pub struct DeltaEContrastCalculator;

impl ContrastCalculationTemplate for DeltaEContrastCalculator {
    fn calculate_luminance(&self, color: Srgb) -> f64 {
        let lab: Lab = color.into_color();
        lab.l as f64
    }

    fn compute_contrast_value(&self, _luminance1: f64, _luminance2: f64) -> f64 {
        // Delta E calculation is handled differently, using color difference
        0.0 // Placeholder - actual calculation in specialized method
    }

    fn determine_contrast_level(&self, delta_e: f64) -> ContrastLevel {
        ContrastLevel::from_delta_e(delta_e)
    }

    fn algorithm_name(&self) -> &'static str {
        "CIEDE2000 Delta E"
    }

    /// Specialized method for Delta E calculation - override of calculate_contrast
    fn calculate_contrast(&self, color1: Srgb, color2: Srgb) -> Result<ContrastAnalysis> {
        let lab1: Lab = color1.into_color();
        let lab2: Lab = color2.into_color();

        let delta_e = lab1.difference(lab2);
        let level = self.determine_contrast_level(delta_e as f64);

        Ok(ContrastAnalysis {
            contrast_ratio: delta_e as f64,
            contrast_level: level,
            algorithm_used: self.algorithm_name(),
            meets_wcag_aa: delta_e >= 20.0, // Different thresholds for Delta E
            meets_wcag_aaa: delta_e >= 40.0,
            color1_luminance: lab1.l as f64,
            color2_luminance: lab2.l as f64,
        })
    }
}

/// Enhanced contrast calculator with multiple algorithms
pub struct EnhancedContrastCalculator {
    primary_algorithm: Box<dyn ContrastCalculationTemplate>,
    secondary_algorithm: Option<Box<dyn ContrastCalculationTemplate>>,
}

impl EnhancedContrastCalculator {
    pub fn new(primary: Box<dyn ContrastCalculationTemplate>) -> Self {
        Self {
            primary_algorithm: primary,
            secondary_algorithm: None,
        }
    }

    pub fn with_secondary(mut self, secondary: Box<dyn ContrastCalculationTemplate>) -> Self {
        self.secondary_algorithm = Some(secondary);
        self
    }

    pub fn calculate_enhanced_contrast(
        &self,
        color1: Srgb,
        color2: Srgb,
    ) -> Result<Vec<ContrastAnalysis>> {
        let mut results = Vec::new();

        // Primary analysis
        results.push(self.primary_algorithm.calculate_contrast(color1, color2)?);

        // Secondary analysis if available
        if let Some(ref secondary) = self.secondary_algorithm {
            results.push(secondary.calculate_contrast(color1, color2)?);
        }

        Ok(results)
    }

    /// Get comprehensive contrast assessment
    pub fn get_comprehensive_assessment(
        &self,
        color1: Srgb,
        color2: Srgb,
    ) -> Result<ContrastAssessment> {
        let analyses = self.calculate_enhanced_contrast(color1, color2)?;

        let wcag_analysis = &analyses[0]; // Primary should be WCAG
        let overall_level = if analyses.len() > 1 {
            // Use the more conservative assessment
            let secondary_level = analyses[1].contrast_level;
            let primary_level = wcag_analysis.contrast_level;

            match (primary_level, secondary_level) {
                (ContrastLevel::Low, _) | (_, ContrastLevel::Low) => ContrastLevel::Low,
                (ContrastLevel::Marginal, _) | (_, ContrastLevel::Marginal) => {
                    ContrastLevel::Marginal
                }
                (ContrastLevel::Medium, _) | (_, ContrastLevel::Medium) => ContrastLevel::Medium,
                (ContrastLevel::High, ContrastLevel::High) => ContrastLevel::High,
            }
        } else {
            wcag_analysis.contrast_level
        };

        Ok(ContrastAssessment {
            primary_analysis: wcag_analysis.clone(),
            secondary_analysis: analyses.get(1).cloned(),
            overall_assessment: overall_level,
            recommendation: generate_recommendation(&analyses),
        })
    }
}

/// Comprehensive contrast assessment result
#[derive(Debug, Clone)]
pub struct ContrastAssessment {
    pub primary_analysis: ContrastAnalysis,
    pub secondary_analysis: Option<ContrastAnalysis>,
    pub overall_assessment: ContrastLevel,
    pub recommendation: String,
}

/// Factory for creating contrast calculators
pub struct ContrastCalculatorFactory;

impl ContrastCalculatorFactory {
    pub fn create_wcag_calculator() -> Box<dyn ContrastCalculationTemplate> {
        Box::new(WcagContrastCalculator)
    }

    pub fn create_lab_calculator() -> Box<dyn ContrastCalculationTemplate> {
        Box::new(LabContrastCalculator)
    }

    pub fn create_delta_e_calculator() -> Box<dyn ContrastCalculationTemplate> {
        Box::new(DeltaEContrastCalculator)
    }

    pub fn create_enhanced_calculator() -> EnhancedContrastCalculator {
        EnhancedContrastCalculator::new(Self::create_wcag_calculator())
            .with_secondary(Self::create_lab_calculator())
    }

    pub fn create_by_name(name: &str) -> Box<dyn ContrastCalculationTemplate> {
        match name.to_lowercase().as_str() {
            "wcag" => Self::create_wcag_calculator(),
            "lab" => Self::create_lab_calculator(),
            "delta-e" | "deltae" => Self::create_delta_e_calculator(),
            _ => Self::create_wcag_calculator(), // Default
        }
    }
}

/// Generate recommendation based on contrast analyses
fn generate_recommendation(analyses: &[ContrastAnalysis]) -> String {
    let primary = &analyses[0];

    match primary.contrast_level {
        ContrastLevel::High => {
            "Excellent contrast - suitable for all text sizes and purposes.".to_string()
        }
        ContrastLevel::Medium => {
            "Good contrast - meets WCAG AA standards for normal text.".to_string()
        }
        ContrastLevel::Marginal => {
            "Marginal contrast - only suitable for large text or decorative purposes.".to_string()
        }
        ContrastLevel::Low => {
            "Poor contrast - not suitable for text. Consider using higher contrast colors."
                .to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Srgb;

    #[test]
    fn test_wcag_contrast_calculator() {
        let calculator = WcagContrastCalculator;
        let white = Srgb::new(1.0, 1.0, 1.0);
        let black = Srgb::new(0.0, 0.0, 0.0);

        let result = calculator.calculate_contrast(white, black).unwrap();
        assert!(result.contrast_ratio > 20.0); // Should be 21:1
        assert_eq!(result.contrast_level, ContrastLevel::High);
        assert!(result.meets_wcag_aaa);
    }

    #[test]
    fn test_lab_contrast_calculator() {
        let calculator = LabContrastCalculator;
        let white = Srgb::new(1.0, 1.0, 1.0);
        let black = Srgb::new(0.0, 0.0, 0.0);

        let result = calculator.calculate_contrast(white, black).unwrap();
        assert!(result.contrast_ratio > 50.0);
        assert_eq!(result.algorithm_used, "LAB Perceptual");
    }

    #[test]
    fn test_contrast_factory() {
        let wcag_calc = ContrastCalculatorFactory::create_wcag_calculator();
        assert_eq!(wcag_calc.algorithm_name(), "WCAG 2.1");

        let lab_calc = ContrastCalculatorFactory::create_lab_calculator();
        assert_eq!(lab_calc.algorithm_name(), "LAB Perceptual");
    }

    #[test]
    fn test_enhanced_calculator() {
        let calculator = ContrastCalculatorFactory::create_enhanced_calculator();
        let white = Srgb::new(1.0, 1.0, 1.0);
        let black = Srgb::new(0.0, 0.0, 0.0);

        let results = calculator
            .calculate_enhanced_contrast(white, black)
            .unwrap();
        assert_eq!(results.len(), 2); // Primary + secondary

        let assessment = calculator
            .get_comprehensive_assessment(white, black)
            .unwrap();
        assert_eq!(assessment.overall_assessment, ContrastLevel::High);
        assert!(!assessment.recommendation.is_empty());
    }
}
