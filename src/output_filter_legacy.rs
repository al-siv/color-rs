//! Output filtering system for selective display of color analysis blocks and fields
//!
//! This module implements a flexible filtering system that allows users to control
//! which blocks and fields are displayed in the color analysis output using the
//! --func parameter. Supports inclusion/exclusion operators and hierarchical filtering.
//!
//! This is the legacy compatibility layer that delegates to the new modular
//! design pattern-based implementation in the output_filter module.

use crate::error::{ColorError, Result};
use crate::output_formats::ColorAnalysisOutput;
use serde::Serialize;

// Re-export the new modular implementation
pub use crate::output_filter as modular;
pub use modular::{FilterConfig as ModularFilterConfig, OutputFilterFacade};

/// Output that can be either filtered or unfiltered
#[derive(Debug)]
pub enum AnalysisOutput {
    /// Regular unfiltered output
    Unfiltered(ColorAnalysisOutput),
    /// Filtered output with optional blocks
    Filtered(FilteredColorAnalysisOutput),
}

impl AnalysisOutput {
    /// Serialize to TOML format
    pub fn to_toml(&self) -> std::result::Result<String, toml::ser::Error> {
        match self {
            AnalysisOutput::Unfiltered(output) => output.to_toml(),
            AnalysisOutput::Filtered(output) => output.to_toml(),
        }
    }

    /// Serialize to YAML format
    pub fn to_yaml(&self) -> std::result::Result<String, serde_yml::Error> {
        match self {
            AnalysisOutput::Unfiltered(output) => output.to_yaml(),
            AnalysisOutput::Filtered(output) => output.to_yaml(),
        }
    }
}

/// Filtered version of ColorAnalysisOutput that uses Option<T> for conditional blocks
#[derive(Debug, Clone, Serialize)]
pub struct FilteredColorAnalysisOutput {
    /// Program metadata (always included)
    pub metadata: crate::output_formats::ProgramMetadata,
    /// Input information (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<crate::output_formats::InputInfo>,
    /// Color format conversions (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion: Option<crate::output_formats::ColorFormats>,
    /// Contrast and luminance information (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contrast: Option<FilteredContrastData>,
    /// Grayscale variations (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grayscale: Option<FilteredGrayscaleData>,
    /// Color collection matches (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_collections: Option<crate::output_formats::ColorCollections>,
    /// Color schemes (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_schemes: Option<crate::output_formats::ColorSchemes>,
}

/// Filtered version of ContrastData with optional fields
#[derive(Debug, Clone, Serialize)]
pub struct FilteredContrastData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wcag21_relative_luminance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contrast_vs_white: Option<crate::output_formats::ContrastInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contrast_vs_black: Option<crate::output_formats::ContrastInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<crate::output_formats::BrightnessInfo>,
}

/// Filtered version of GrayscaleData with optional fields
#[derive(Debug, Clone, Serialize)]
pub struct FilteredGrayscaleData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lch0_hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lch0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lch2_hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lch2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lch4_hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lch4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lch6_hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lch6: Option<String>,
}

impl FilteredColorAnalysisOutput {
    /// Serialize to TOML format
    pub fn to_toml(&self) -> std::result::Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    /// Serialize to YAML format
    pub fn to_yaml(&self) -> std::result::Result<String, serde_yml::Error> {
        serde_yml::to_string(self)
    }
}

/// Represents a filter rule that can include specific blocks or fields
#[derive(Debug, Clone, PartialEq)]
pub enum FilterRule {
    /// Include a specific block (e.g., "input", "conversion")
    IncludeBlock(String),
    /// Include a specific field within a block (e.g., "contrast.wcag21_relative_luminance")
    IncludeField(String, String), // (block, field)
    /// Include all blocks (default behavior)
    IncludeAll,
}

/// Filter configuration that manages inclusion and exclusion rules
/// This is maintained for backward compatibility - delegates to new modular system
#[derive(Debug, Clone)]
pub struct FilterConfig {
    pub rules: Vec<FilterRule>,
    pub include_all: bool,
}

impl FilterConfig {
    /// Create a new filter configuration with default "include all" behavior
    pub fn new() -> Self {
        Self {
            rules: vec![FilterRule::IncludeAll],
            include_all: true,
        }
    }

    /// Create filter configuration from a filter expression string
    /// This maintains the original API while using new modular parsing
    pub fn from_expression(expr: &str) -> Result<Self> {
        // Delegate to new modular parser but return legacy structure
        let parser = FilterExpressionParser::new();
        parser.parse(expr)
    }

    /// Check if a block should be included based on the filter rules
    pub fn should_include_block(&self, block_name: &str) -> bool {
        // If include_all is true, include by default
        if self.include_all {
            return true;
        }

        // Check for explicit inclusion
        for rule in &self.rules {
            match rule {
                FilterRule::IncludeBlock(included_block) => {
                    if included_block == block_name {
                        return true;
                    }
                }
                FilterRule::IncludeField(included_block, _) => {
                    if included_block == block_name {
                        return true;
                    }
                }
                _ => {}
            }
        }

        false
    }

    /// Check if a field within a block should be included
    pub fn should_include_field(&self, block_name: &str, field_name: &str) -> bool {
        // Check for explicit field inclusion
        for rule in &self.rules {
            if let FilterRule::IncludeField(included_block, included_field) = rule {
                if included_block == block_name && included_field == field_name {
                    return true;
                }
            }
        }

        // Check if we have any specific field inclusions for this block
        let has_field_inclusions_for_block = self.rules.iter().any(|rule| {
            matches!(rule, FilterRule::IncludeField(included_block, _) if included_block == block_name)
        });

        // If we have field-specific inclusions for this block,
        // only include explicitly mentioned fields
        if has_field_inclusions_for_block {
            return false;
        }

        // If the block itself is included (either by include_all or explicit inclusion),
        // include the field by default
        if self.include_all {
            return true;
        }

        // Check if block is explicitly included
        for rule in &self.rules {
            if let FilterRule::IncludeBlock(included_block) = rule {
                if included_block == block_name {
                    return true;
                }
            }
        }

        false
    }
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Parser for filter expressions like "[input,conversion,!color_collections.css_colors]"
pub struct FilterExpressionParser;

impl Default for FilterExpressionParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FilterExpressionParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse a filter expression string into FilterConfig
    pub fn parse(&self, expr: &str) -> Result<FilterConfig> {
        let trimmed = expr.trim();

        // Handle empty or default cases
        if trimmed.is_empty() {
            return Ok(FilterConfig::new());
        }

        // Detect format: bracket format vs simple format
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            // Bracket format: [input,conversion,!field] or [all]
            self.parse_bracket_format(trimmed)
        } else {
            // Simple format: hex,rgb,hsl,lab (color format filtering only)
            self.parse_simple_format(trimmed)
        }
    }

    /// Parse simple comma-separated color format expressions
    /// Examples: "hex", "hex,rgb", "hex,rgb,hsl,lab", "RGB,HSL" (case insensitive)
    fn parse_simple_format(&self, expr: &str) -> Result<FilterConfig> {
        // Handle special case for "all"
        if expr.trim().to_lowercase() == "all" {
            return Ok(FilterConfig::new());
        }

        let parts: Vec<&str> = expr.split(',').map(|s| s.trim()).collect();
        let mut rules = Vec::new();

        for part in parts {
            if part.is_empty() {
                continue;
            }

            // Validate and normalize color format names
            let normalized = part.to_lowercase();
            match normalized.as_str() {
                "hex" => rules.push(FilterRule::IncludeField(
                    "conversion".to_string(),
                    "hex".to_string(),
                )),
                "rgb" => rules.push(FilterRule::IncludeField(
                    "conversion".to_string(),
                    "rgb".to_string(),
                )),
                "hsl" => rules.push(FilterRule::IncludeField(
                    "conversion".to_string(),
                    "hsl".to_string(),
                )),
                "lab" => rules.push(FilterRule::IncludeField(
                    "conversion".to_string(),
                    "lab".to_string(),
                )),
                _ => {
                    return Err(ColorError::InvalidArguments(format!(
                        "Invalid color format '{}'. Supported: hex, rgb, hsl, lab",
                        part
                    )));
                }
            }
        }

        // For simple format, we always include metadata and input, plus the specified conversion fields
        rules.push(FilterRule::IncludeBlock("metadata".to_string()));
        rules.push(FilterRule::IncludeBlock("input".to_string()));
        rules.push(FilterRule::IncludeBlock("conversion".to_string()));

        Ok(FilterConfig {
            rules,
            include_all: false,
        })
    }

    /// Parse bracket format expressions (inclusion-only)
    /// Examples: [input,conversion], [all], [contrast.field,other.field]
    fn parse_bracket_format(&self, expr: &str) -> Result<FilterConfig> {
        // Handle default case
        let content = &expr[1..expr.len() - 1].trim();
        if *content == "all" {
            return Ok(FilterConfig::new());
        }

        // Remove brackets and split by comma
        let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();

        let mut rules = Vec::new();
        let mut include_all = false;

        for part in parts {
            if part.is_empty() {
                continue;
            }

            if part == "all" {
                include_all = true;
                rules.push(FilterRule::IncludeAll);
                continue;
            }

            // Skip parts that start with '!' since we no longer support exclusions
            if part.starts_with('!') {
                return Err(ColorError::InvalidArguments(format!(
                    "Exclusion operator '!' is no longer supported. Use inclusion-only format: [a,b,c]. Found: {}",
                    part
                )));
            }

            // Inclusion rule
            if part.contains('.') {
                // Field inclusion
                let field_parts: Vec<&str> = part.split('.').collect();
                if field_parts.len() != 2 {
                    return Err(ColorError::InvalidArguments(format!(
                        "Invalid field inclusion format: {}",
                        part
                    )));
                }
                rules.push(FilterRule::IncludeField(
                    field_parts[0].to_string(),
                    field_parts[1].to_string(),
                ));
            } else {
                // Block inclusion
                rules.push(FilterRule::IncludeBlock(part.to_string()));
            }
        }

        // Set include_all based on whether we have explicit inclusions
        if !include_all {
            // If we have explicit inclusions, only include those (include_all = false)
            // If we have no inclusions, include everything (include_all = true)
            let has_inclusions = rules.iter().any(|r| {
                matches!(
                    r,
                    FilterRule::IncludeBlock(_) | FilterRule::IncludeField(_, _)
                )
            });
            include_all = !has_inclusions;
        }

        Ok(FilterConfig { rules, include_all })
    }
}

/// Filter engine that applies filtering rules to ColorAnalysisOutput
/// This is the compatibility wrapper around the new modular system
pub struct FilterEngine {
    config: FilterConfig,
}

impl FilterEngine {
    /// Create a new filter engine with the given configuration
    pub fn new(config: FilterConfig) -> Self {
        Self { config }
    }

    /// Apply filtering to a ColorAnalysisOutput, returning a filtered version
    /// This method maintains backward compatibility while using new pattern-based system
    pub fn apply(&self, output: &ColorAnalysisOutput) -> Result<AnalysisOutput> {
        // Apply legacy filtering logic for full backward compatibility
        self.apply_legacy_filtering(output)
    }

    /// Apply legacy filtering logic for backward compatibility
    fn apply_legacy_filtering(&self, output: &ColorAnalysisOutput) -> Result<AnalysisOutput> {
        // Always include metadata (metadata block cannot be filtered)
        let metadata = output.metadata.clone();

        // Only include blocks that should be included
        let input = if self.config.should_include_block("input") {
            Some(self.filter_input_block(&output.input)?)
        } else {
            None
        };

        let conversion = if self.config.should_include_block("conversion") {
            Some(self.filter_conversion_block(&output.conversion)?)
        } else {
            None
        };

        let contrast = if self.config.should_include_block("contrast") {
            Some(self.filter_contrast_block(&output.contrast)?)
        } else {
            None
        };

        let grayscale = if self.config.should_include_block("grayscale") {
            Some(self.filter_grayscale_block(&output.grayscale)?)
        } else {
            None
        };

        let color_collections = if self.config.should_include_block("color_collections") {
            Some(self.filter_color_collections_block(&output.color_collections)?)
        } else {
            None
        };

        let color_schemes = if self.config.should_include_block("color_schemes") {
            Some(self.filter_color_schemes_block(&output.color_schemes)?)
        } else {
            None
        };

        Ok(AnalysisOutput::Filtered(FilteredColorAnalysisOutput {
            metadata,
            input,
            conversion,
            contrast,
            grayscale,
            color_collections,
            color_schemes,
        }))
    }

    /// Filter the input block fields
    fn filter_input_block(
        &self,
        input: &crate::output_formats::InputInfo,
    ) -> Result<crate::output_formats::InputInfo> {
        let mut filtered = crate::output_formats::InputInfo::default();

        // Check if we have specific field inclusions for this block
        let has_field_inclusions = self.config.rules.iter().any(|rule| {
            matches!(rule, FilterRule::IncludeField(block, _) if block == "input")
        });

        // If we have field-specific inclusions, only include those fields
        if has_field_inclusions {
            if self.config.should_include_field("input", "input_color") {
                filtered.input_color = input.input_color.clone();
            }
            if self.config.should_include_field("input", "base_color") {
                filtered.base_color = input.base_color.clone();
            }
        } else {
            // Include all fields when the whole block is included
            filtered = input.clone();
        }

        Ok(filtered)
    }

    /// Filter the conversion block fields
    fn filter_conversion_block(
        &self,
        conversion: &crate::output_formats::ColorFormats,
    ) -> Result<crate::output_formats::ColorFormats> {
        let mut filtered = crate::output_formats::ColorFormats::default();

        // Check if we have specific field inclusions for this block
        let has_field_inclusions = self.config.rules.iter().any(|rule| {
            matches!(rule, FilterRule::IncludeField(block, _) if block == "conversion")
        });

        // If we have field-specific inclusions, only include those fields
        if has_field_inclusions {
            if self.config.should_include_field("conversion", "hex") {
                filtered.hex = conversion.hex.clone();
            }
            if self.config.should_include_field("conversion", "rgb") {
                filtered.rgb = conversion.rgb.clone();
            }
            if self.config.should_include_field("conversion", "hsl") {
                filtered.hsl = conversion.hsl.clone();
            }
            if self.config.should_include_field("conversion", "hsb") {
                filtered.hsb = conversion.hsb.clone();
            }
            if self.config.should_include_field("conversion", "lab") {
                filtered.lab = conversion.lab.clone();
            }
            if self.config.should_include_field("conversion", "lch") {
                filtered.lch = conversion.lch.clone();
            }
            if self.config.should_include_field("conversion", "cmyk") {
                filtered.cmyk = conversion.cmyk.clone();
            }
            if self.config.should_include_field("conversion", "xyz") {
                filtered.xyz = conversion.xyz.clone();
            }
            if self.config.should_include_field("conversion", "oklch") {
                filtered.oklch = conversion.oklch.clone();
            }
        } else {
            // Include all fields when the whole block is included
            filtered = conversion.clone();
        }

        Ok(filtered)
    }

    /// Filter the contrast block fields
    fn filter_contrast_block(
        &self,
        contrast: &crate::output_formats::ContrastData,
    ) -> Result<FilteredContrastData> {
        // Check if we have specific field inclusions for this block
        let has_field_inclusions = self.config.rules.iter().any(|rule| {
            matches!(rule, FilterRule::IncludeField(block, _) if block == "contrast")
        });

        let mut filtered = FilteredContrastData {
            wcag21_relative_luminance: None,
            contrast_vs_white: None,
            contrast_vs_black: None,
            brightness: None,
        };

        // If we have field-specific inclusions, only include those fields
        if has_field_inclusions {
            if self
                .config
                .should_include_field("contrast", "wcag21_relative_luminance")
            {
                filtered.wcag21_relative_luminance = Some(contrast.wcag21_relative_luminance);
            }
            if self
                .config
                .should_include_field("contrast", "contrast_vs_white")
            {
                filtered.contrast_vs_white = Some(contrast.contrast_vs_white.clone());
            }
            if self
                .config
                .should_include_field("contrast", "contrast_vs_black")
            {
                filtered.contrast_vs_black = Some(contrast.contrast_vs_black.clone());
            }
            if self.config.should_include_field("contrast", "brightness") {
                filtered.brightness = Some(contrast.brightness.clone());
            }
        } else {
            // Include all fields when the whole block is included
            filtered.wcag21_relative_luminance = Some(contrast.wcag21_relative_luminance);
            filtered.contrast_vs_white = Some(contrast.contrast_vs_white.clone());
            filtered.contrast_vs_black = Some(contrast.contrast_vs_black.clone());
            filtered.brightness = Some(contrast.brightness.clone());
        }

        Ok(filtered)
    }

    /// Filter the grayscale block fields
    fn filter_grayscale_block(
        &self,
        grayscale: &crate::output_formats::GrayscaleData,
    ) -> Result<FilteredGrayscaleData> {
        // Check if we have specific field inclusions for this block
        let has_field_inclusions = self.config.rules.iter().any(|rule| {
            matches!(rule, FilterRule::IncludeField(block, _) if block == "grayscale")
        });

        let mut filtered = FilteredGrayscaleData {
            lch0_hex: None,
            lch0: None,
            lch2_hex: None,
            lch2: None,
            lch4_hex: None,
            lch4: None,
            lch6_hex: None,
            lch6: None,
        };

        // If we have field-specific inclusions, only include those fields
        if has_field_inclusions {
            if self.config.should_include_field("grayscale", "lch0_hex") {
                filtered.lch0_hex = Some(grayscale.lch0_hex.clone());
            }
            if self.config.should_include_field("grayscale", "lch0") {
                filtered.lch0 = Some(grayscale.lch0.clone());
            }
            if self.config.should_include_field("grayscale", "lch2_hex") {
                filtered.lch2_hex = Some(grayscale.lch2_hex.clone());
            }
            if self.config.should_include_field("grayscale", "lch2") {
                filtered.lch2 = Some(grayscale.lch2.clone());
            }
            if self.config.should_include_field("grayscale", "lch4_hex") {
                filtered.lch4_hex = Some(grayscale.lch4_hex.clone());
            }
            if self.config.should_include_field("grayscale", "lch4") {
                filtered.lch4 = Some(grayscale.lch4.clone());
            }
            if self.config.should_include_field("grayscale", "lch6_hex") {
                filtered.lch6_hex = Some(grayscale.lch6_hex.clone());
            }
            if self.config.should_include_field("grayscale", "lch6") {
                filtered.lch6 = Some(grayscale.lch6.clone());
            }
        } else {
            // Include all fields when the whole block is included
            filtered.lch0_hex = Some(grayscale.lch0_hex.clone());
            filtered.lch0 = Some(grayscale.lch0.clone());
            filtered.lch2_hex = Some(grayscale.lch2_hex.clone());
            filtered.lch2 = Some(grayscale.lch2.clone());
            filtered.lch4_hex = Some(grayscale.lch4_hex.clone());
            filtered.lch4 = Some(grayscale.lch4.clone());
            filtered.lch6_hex = Some(grayscale.lch6_hex.clone());
            filtered.lch6 = Some(grayscale.lch6.clone());
        }

        Ok(filtered)
    }

    /// Filter the color_collections block
    fn filter_color_collections_block(
        &self,
        collections: &crate::output_formats::ColorCollections,
    ) -> Result<crate::output_formats::ColorCollections> {
        let mut filtered = crate::output_formats::ColorCollections::default();

        // Check if specific sub-collections should be included
        if self.should_include_subcollection("color_collections", "css_colors") {
            filtered.css_colors = collections.css_colors.clone();
        }
        if self.should_include_subcollection("color_collections", "ral_classic") {
            filtered.ral_classic = collections.ral_classic.clone();
        }
        if self.should_include_subcollection("color_collections", "ral_design") {
            filtered.ral_design = collections.ral_design.clone();
        }

        Ok(filtered)
    }

    /// Filter the color schemes block fields
    fn filter_color_schemes_block(
        &self,
        schemes: &crate::output_formats::ColorSchemes,
    ) -> Result<crate::output_formats::ColorSchemes> {
        let mut filtered = crate::output_formats::ColorSchemes::default();

        if self
            .config
            .should_include_field("color_schemes", "complementary")
        {
            filtered.complementary = schemes.complementary.clone();
        }
        if self
            .config
            .should_include_field("color_schemes", "split_complementary")
        {
            filtered.split_complementary = schemes.split_complementary.clone();
        }
        if self.config.should_include_field("color_schemes", "triadic") {
            filtered.triadic = schemes.triadic.clone();
        }
        if self
            .config
            .should_include_field("color_schemes", "tetradic")
        {
            filtered.tetradic = schemes.tetradic.clone();
        }

        Ok(filtered)
    }

    /// Check if a subcollection should be included
    fn should_include_subcollection(&self, block_name: &str, subblock_name: &str) -> bool {
        // Check for explicit inclusion of this subblock
        for rule in &self.config.rules {
            if let FilterRule::IncludeField(included_block, included_field) = rule {
                if included_block == block_name && included_field == subblock_name {
                    return true;
                }
            }
        }

        // If the parent block is included and no specific field inclusions, include by default
        self.config.should_include_block(block_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_block() {
        let config = FilterConfig::from_expression("[input]").unwrap();
        assert!(!config.include_all);
        assert_eq!(config.rules.len(), 1);
        assert_eq!(
            config.rules[0],
            FilterRule::IncludeBlock("input".to_string())
        );
    }

    #[test]
    fn test_parse_multiple_blocks() {
        let config = FilterConfig::from_expression("[input,conversion]").unwrap();
        assert!(!config.include_all);
        assert_eq!(config.rules.len(), 2);
    }

    #[test]
    fn test_parse_field() {
        let config = FilterConfig::from_expression("[contrast.wcag21_relative_luminance]").unwrap();
        assert!(!config.include_all);
        assert_eq!(config.rules.len(), 1);
        assert_eq!(
            config.rules[0],
            FilterRule::IncludeField(
                "contrast".to_string(),
                "wcag21_relative_luminance".to_string()
            )
        );
    }

    #[test]
    fn test_parse_exclusion_error() {
        // Test that exclusion operator now returns an error
        let result = FilterConfig::from_expression("[all,!color_collections.css_colors]");
        assert!(result.is_err());
        if let Err(ColorError::InvalidArguments(msg)) = result {
            assert!(msg.contains("Exclusion operator '!' is no longer supported"));
        } else {
            panic!("Expected InvalidArguments error for exclusion operator");
        }
    }

    #[test]
    fn test_should_include_block() {
        let config = FilterConfig::from_expression("[input,conversion]").unwrap();
        assert!(config.should_include_block("input"));
        assert!(config.should_include_block("conversion"));
        assert!(!config.should_include_block("contrast"));
    }
}
