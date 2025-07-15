# Changelog

All notable changes to the color-rs project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.4] - 2025-01-23

### Changed
- **Improved Color Accuracy**: Replaced custom color algorithms with palette library implementations
  - **Delta E Calculation**: Now uses `ImprovedCiede2000` algorithm instead of simple Euclidean distance for more accurate perceptual color differences
  - **WCAG Compliance**: Replaced custom relative luminance and contrast ratio calculations with palette's `Wcag21RelativeContrast` implementation
  - **Color Interpolation**: Now uses palette's `Mix` trait instead of manual LAB component interpolation
- **Enhanced HSL Processing**: Added alternative HSL→XYZ→LAB→RGB conversion path alongside direct HSL→RGB conversion
- **Comprehensive Testing**: Added comparison tests between direct and LAB-based HSL→RGB conversion methods with 10 test colors

### Added
- `hsl_to_rgb_via_lab()` method for alternative HSL to RGB conversion via LAB color space
- Detailed comparison tests showing differences between conversion methods (typically <1 RGB unit difference)

### Fixed
- Updated test expectations to match more accurate ImprovedCiede2000 delta E values (∼23 for red vs blue instead of ∼175 from Euclidean)
- Improved color distance calculations now provide more reliable perceptual color matching

## [0.8.3] - 2025-01-23

### Added
- New `color_formatter` module for centralized color display and formatting logic
- New `color_utils` module containing universal color utility functions
- Comprehensive test coverage for all color conversion and WCAG functions

### Changed  
- **Major Architectural Improvements**: Separated concerns across modules for better maintainability
  - Moved WCAG calculations to `ColorUtils` for reusability across modules
  - Refactored large `color_match` function (~140 lines) into smaller, focused functions
  - Created dedicated `ColorFormatter` for all color display and reporting logic
  - Replaced manual color conversions with reliable `palette` crate implementations
- **ColorNameResolver**: Removed duplicate color conversion functions, now uses `ColorUtils`
- **CssColorParser**: Updated to use `ColorUtils::hsl_to_rgb` instead of manual implementation
- **ColorProcessor**: Streamlined to focus on core color operations, removed duplicate WCAG methods

### Removed
- Duplicate WCAG relative luminance and contrast ratio implementations
- Manual HSL to RGB conversion functions (replaced with palette-based implementations)
- Misplaced color utility functions from domain-specific modules

### Fixed
- Type consistency in color conversion functions
- Import optimization and removal of unused dependencies
- Eliminated code duplication across multiple modules

## [0.8.0] - 2025-07-15

### Added
- Comprehensive refactoring of the codebase into modular structure
- New library structure with separated concerns:
  - `cli` module for command-line interface
  - `color` module for color operations and conversions
  - `gradient` module for gradient calculations and easing functions
  - `image` module for SVG and PNG generation
  - `error` module for custom error types
  - `config` module for constants and configuration
  - `utils` module for utility functions
- Custom error types with proper error handling
- Comprehensive unit test suite (16 tests)
- Library API for programmatic usage
- Enhanced documentation with inline code documentation
- Type safety improvements with custom Result type
- Better separation of CLI and library functionality

### Changed
- Restructured codebase from single main.rs file to modular library
- Improved error handling using custom ColorError enum instead of anyhow everywhere
- Enhanced code organization and maintainability
- Better abstraction of core functionality
- Simplified main.rs to focus only on CLI entry point

### Technical Details
- Added lib.rs as main library entry point
- All modules now have comprehensive documentation
- Error types properly implement std::error::Error trait
- Configuration constants centralized in config module
- Utility functions extracted and properly tested
- Image generation logic separated and modularized

## [0.7.2] - 2025-07-14

### Added
- Professional Table Formatting: Cargo-style output with right-aligned numeric columns
- Enhanced Visual Design: Improved terminal output with beautiful ASCII tables
- Integer Percentages: CSS-compatible integer percentage calculations
- Solid PNG Backgrounds: Fixed PNG rendering with proper solid backgrounds
- Comprehensive Documentation: Complete README with examples and usage guides
- Intelligent Stop Placement: Advanced derivative-based gradient stop calculation
- CSS Integration: Ready-to-use output for web development workflows
- Production Ready: Stable API and comprehensive error handling

### Features Summary
- LAB color space for perceptually uniform gradients
- CSS cubic-bezier timing functions
- SVG and PNG export capabilities  
- Multiple output formats (step-based, intelligent, equal spacing)
- Professional terminal interface matching Rust toolchain aesthetics
- Windows executable distribution for easy installation
