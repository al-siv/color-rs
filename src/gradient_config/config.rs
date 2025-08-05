//! Gradient Configuration Core Implementation
//!
//! This module contains the main GradientConfig implementation with
//! immutable configuration methods and CLI integration.

use super::types::*;
use crate::cli::GradientArgs;
use crate::error::{ColorError, Result};

impl GradientConfig {
    /// Create a new gradient configuration with required parameters
    ///
    /// # Errors
    /// Returns `ColorError` if color pair or easing configuration is invalid
    pub fn new(
        colors: ColorPair,
        easing: EasingConfig,
    ) -> Result<Self> {
        Ok(Self {
            colors,
            easing,
            position_range: PositionRange::full_range(),
            image_output: ImageOutput::default_config(),
            stop_config: StopConfig::default_config(),
            file_output: None,
        })
    }

    /// Create a basic gradient configuration with defaults
    ///
    /// # Errors
    /// Returns `ColorError` if color validation fails
    pub fn basic(start_color: &str, end_color: &str) -> Result<Self> {
        let colors = ColorPair::new(start_color, end_color)?;
        let easing = EasingConfig::default_config();
        Self::new(colors, easing)
    }

    /// Update position range (immutable)
    ///
    /// # Errors
    /// Returns `ColorError` if position range is invalid
    pub fn with_position_range(self, range: PositionRange) -> Result<Self> {
        Ok(Self {
            position_range: range,
            ..self
        })
    }

    /// Update easing configuration (immutable)
    pub fn with_easing(self, easing: EasingConfig) -> Self {
        Self { easing, ..self }
    }

    /// Update stop configuration (immutable)
    pub fn with_stop_config(self, stop_config: StopConfig) -> Self {
        Self { stop_config, ..self }
    }

    /// Add SVG output (immutable)
    ///
    /// # Errors
    /// Returns `ColorError` if filename validation fails
    pub fn with_svg_output(self, filename: &str) -> Result<Self> {
        let image_output = ImageOutput::svg(filename, self.image_output.width, self.image_output.show_legend)?;
        Ok(Self { image_output, ..self })
    }

    /// Add PNG output (immutable)
    ///
    /// # Errors
    /// Returns `ColorError` if filename validation fails
    pub fn with_png_output(self, filename: &str) -> Result<Self> {
        let image_output = ImageOutput::png(filename, self.image_output.width, self.image_output.show_legend)?;
        Ok(Self { image_output, ..self })
    }

    /// Add both SVG and PNG output (immutable)
    ///
    /// # Errors
    /// Returns `ColorError` if filename validation fails
    pub fn with_both_outputs(self, svg_name: &str, png_name: &str) -> Result<Self> {
        let image_output = ImageOutput::both(svg_name, png_name, self.image_output.width, self.image_output.show_legend)?;
        Ok(Self { image_output, ..self })
    }

    /// Update image width (immutable)
    ///
    /// # Errors
    /// Returns `ColorError` if width is invalid
    pub fn with_width(self, width: u32) -> Result<Self> {
        if width == 0 {
            return Err(GradientValidationError::InvalidWidth(width).into());
        }

        let image_output = Self::update_image_width(self.image_output, width);
        Ok(Self { image_output, ..self })
    }

    /// Toggle legend display (immutable)
    pub fn with_legend(self, show_legend: bool) -> Self {
        let image_output = Self::update_image_legend(self.image_output, show_legend);
        Self { image_output, ..self }
    }

    /// Add file output (immutable)
    ///
    /// # Errors
    /// Returns `ColorError` if file output configuration is invalid
    pub fn with_file_output(self, file_output: FileOutput) -> Self {
        Self {
            file_output: Some(file_output),
            ..self
        }
    }

    /// Convenience method to add steps configuration
    ///
    /// # Errors
    /// Returns `ColorError` if step value is invalid
    pub fn with_steps(self, step: u8) -> Result<Self> {
        let stop_config = StopConfig::steps(step)?;
        Ok(self.with_stop_config(stop_config))
    }

    /// Convenience method to add intelligent stops
    pub fn with_intelligent_stops(self, count: usize) -> Self {
        let stop_config = StopConfig::intelligent_stops(count);
        self.with_stop_config(stop_config)
    }

    /// Convenience method to add equal stops
    pub fn with_equal_stops(self, count: usize) -> Self {
        let stop_config = StopConfig::equal_stops(count);
        self.with_stop_config(stop_config)
    }

    /// Helper function to update image width while preserving other settings
    fn update_image_width(mut image_output: ImageOutput, width: u32) -> ImageOutput {
        image_output.width = width;
        image_output
    }

    /// Helper function to update legend setting while preserving other settings
    fn update_image_legend(mut image_output: ImageOutput, show_legend: bool) -> ImageOutput {
        image_output.show_legend = show_legend;
        image_output
    }

    /// Convert to `GradientArgs` for CLI compatibility
    pub fn to_gradient_args(self) -> GradientArgs {
        let (step, stops, stops_simple) = match self.stop_config {
            StopConfig::Steps(s) => (Some(s), 5, false),
            StopConfig::IntelligentStops(count) => (None, count, false),
            StopConfig::EqualStops(count) => (None, count, true),
        };

        GradientArgs {
            start_color: self.colors.start,
            end_color: self.colors.end,
            start_position: self.position_range.start,
            end_position: self.position_range.end,
            ease_in: self.easing.ease_in,
            ease_out: self.easing.ease_out,
            svg: self.image_output.svg_filename.clone(),
            png: self.image_output.png_filename.clone(),
            no_legend: !self.image_output.show_legend,
            width: self.image_output.width,
            step,
            stops,
            stops_simple,
            output_format: self.file_output.as_ref().map(|f| f.format.clone()),
            output_file: self.file_output.map(|f| f.filename),
            func_filter: None,
        }
    }

    /// Create `GradientConfig` from CLI `GradientArgs` (CLI integration)
    pub fn from_gradient_args(args: GradientArgs) -> Result<Self> {
        let colors = Self::validate_and_create_colors(&args)?;
        let easing = Self::create_easing_config(&args)?;
        let position_range = Self::create_position_range(&args)?;
        let stop_config = Self::create_stop_config(&args);
        let file_output = Self::create_file_output(&args)?;
        
        Self::build_configured_gradient(colors, easing, position_range, stop_config, file_output, &args)
    }

    /// Validate and create color pair from CLI arguments
    fn validate_and_create_colors(args: &GradientArgs) -> Result<ColorPair> {
        ColorPair::new(&args.start_color, &args.end_color).map_err(|e| ColorError::InvalidGradient(e.to_string()))
    }

    /// Create easing configuration from CLI arguments
    fn create_easing_config(args: &GradientArgs) -> Result<EasingConfig> {
        EasingConfig::new(args.ease_in, args.ease_out).map_err(|e| ColorError::InvalidGradient(e.to_string()))
    }

    /// Create position range from CLI arguments
    fn create_position_range(args: &GradientArgs) -> Result<PositionRange> {
        PositionRange::new(args.start_position, args.end_position).map_err(|e| ColorError::InvalidGradient(e.to_string()))
    }

    /// Create stop configuration from CLI arguments
    fn create_stop_config(args: &GradientArgs) -> StopConfig {
        if let Some(step) = args.step {
            StopConfig::Steps(step)
        } else if args.stops_simple {
            StopConfig::EqualStops(args.stops)
        } else {
            StopConfig::IntelligentStops(args.stops)
        }
    }

    /// Create file output configuration from CLI arguments
    fn create_file_output(args: &GradientArgs) -> Result<Option<FileOutput>> {
        match (&args.output_format, &args.output_file) {
            (Some(format), Some(filename)) => Ok(Some(FileOutput::new(format.clone(), filename)?)),
            (Some(format), None) => {
                let default_name = Self::get_default_filename(format.clone());
                Ok(Some(FileOutput::new(format.clone(), default_name)?))
            }
            _ => Ok(None),
        }
    }

    /// Get default filename for output format
    fn get_default_filename(format: crate::cli::OutputFormat) -> &'static str {
        match format {
            crate::cli::OutputFormat::Toml => "gradient.toml",
            crate::cli::OutputFormat::Yaml => "gradient.yaml",
        }
    }

    /// Build the final configured gradient with all settings applied
    fn build_configured_gradient(
        colors: ColorPair,
        easing: EasingConfig,
        position_range: PositionRange,
        stop_config: StopConfig,
        file_output: Option<FileOutput>,
        args: &GradientArgs,
    ) -> Result<Self> {
        let base_config = Self::new(colors, easing)?;
        let positioned_config = base_config.with_position_range(position_range)?;
        let stop_configured = positioned_config.with_stop_config(stop_config);
        let image_configured = Self::apply_image_output(stop_configured, args)?;
        let sized_config = image_configured.with_width(args.width)?;
        let legend_config = sized_config.with_legend(!args.no_legend);
        
        let final_config = if let Some(file_out) = file_output {
            legend_config.with_file_output(file_out)
        } else {
            legend_config
        };

        Ok(final_config)
    }

    /// Apply image output configuration based on CLI arguments
    fn apply_image_output(config: Self, args: &GradientArgs) -> Result<Self> {
        match (&args.svg, &args.png) {
            (Some(svg_name), Some(png_name)) => {
                config.with_both_outputs(svg_name, png_name)
            }
            (Some(svg_name), None) => {
                config.with_svg_output(svg_name)
            }
            (None, Some(png_name)) => {
                config.with_png_output(png_name)
            }
            (None, None) => {
                Ok(config)
            }
        }
    }

    /// Get color pair
    pub fn colors(&self) -> &ColorPair {
        &self.colors
    }

    /// Get easing configuration
    pub fn easing(&self) -> &EasingConfig {
        &self.easing
    }

    /// Get position range
    pub fn position_range(&self) -> &PositionRange {
        &self.position_range
    }

    /// Get image output configuration
    pub fn image_output(&self) -> &ImageOutput {
        &self.image_output
    }

    /// Get stop configuration
    pub fn stop_config(&self) -> &StopConfig {
        &self.stop_config
    }

    /// Get file output configuration
    pub fn file_output(&self) -> Option<&FileOutput> {
        self.file_output.as_ref()
    }
}
