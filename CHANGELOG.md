# Changelog

All notable changes to the color-rs project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.14.0] - 2025-01-18

### Changed
- **Documentation Rewrite**: Complete documentation refresh with factual, technical language
  - Removed promotional language from all documentation files
  - Updated README.md with direct, factual descriptions
  - Revised API documentation for technical accuracy
  - Updated CLI reference with mathematical terminology
  - Replaced "professional", "beautiful", "intelligent" with precise technical terms
- **Package Metadata**: Updated Cargo.toml with refined description, keywords, and categories
  - Description: "CLI tool and library for color analysis, gradient generation, and color space conversions with LAB/LCH color distance calculations"
  - Added keywords: lch, wcag, ral, color-distance, accessibility
  - Added accessibility category
- **Code Quality**: Fixed compilation issues and improved module structure
  - Resolved broken function calls in color.rs and gradient.rs
  - Fixed module exports in lib.rs
  - Removed problematic output_utils dependencies

### Fixed
- **Build System**: Resolved compilation errors from missing functions
- **Module Structure**: Corrected module visibility and exports
- **Documentation Consistency**: Ensured all version references point to 0.14.0

## [0.12.0] - 2025-01-18

### Added
- **HSB Color Support**: Added HSB (Hue, Saturation, Brightness) color format to Color Analysis output
- **CMYK Color Support**: Added CMYK (Cyan, Magenta, Yellow, Key) color format to Color Analysis output
- **Enhanced Color Schemes**: Color schemes now automatically display both HSL and Lab color space strategy calculations
- **Enhanced Color Parsing**: Added support for multiple new color input formats:
  - Hex colors without # symbol (e.g., `457FB3`)
  - LAB colors (e.g., `lab(83.81, 10.89, 11.48)`)
  - RAL Classic codes (e.g., `RAL 5012`)
  - RAL Design System+ codes (e.g., `RAL 010 40 30`)
  - RAL named colors (e.g., `luminous orange`)
  - Case-insensitive CSS named colors (e.g., `Light Blue`)

### Changed
- **CLI Command Simplification**: Renamed `color-match` command to `color` for easier usage
- **Simplified Color Schemes Interface**: Removed the need for users to choose between HSL and Lab strategies - both are now calculated and displayed automatically
- **Color Analysis Enhancement**: Color Analysis section now includes HSB and CMYK conversions alongside existing RGB, HEX, HSL, LAB, XYZ, OKLCH formats
- **Unified Color Parsing**: Both `gradient` and `color` commands now use the same enhanced parsing logic for consistent color format support

### Removed
- **Obsolete CLI Arguments**: Removed `--schema-lab` and `--schema-hsl` command-line flags as they are no longer needed
- **Schema Flag Logic**: Simplified color matching interface by removing schema selection complexity

### Fixed
- **Documentation Imports**: Fixed missing `FromColor` import in `color_utils.rs` doctest
- **Code Quality**: Removed unused import warnings
- **Test Compatibility**: Updated tests to match new output format labels

## [0.11.1] - 2025-07-17

### Fixed
- **Code Quality**: Fixed all Clippy warnings for better code quality
  - Used `strip_suffix()` instead of manual string manipulation
  - Used `unwrap_or()` instead of unnecessary closures
  - Applied `clamp()` method for range constraints
  - Used range contains for boundary checks
  - Used PI constant instead of hardcoded values
- **Dead Code**: Removed unused imports and added appropriate attributes
- **Default Implementation**: Added `Default` trait for `ColorCollectionManager`
- **Code Formatting**: Applied consistent formatting with `cargo fmt`

### Changed
- **Version**: Incremented patch version to 0.11.1

## [0.11.0] - 2025-07-17

### Added
- **GoF Design Patterns Implementation**: Implemented five Gang of Four design patterns for better architecture
  - **Strategy Pattern**: Enhanced color distance calculation with pluggable algorithms (DeltaE76, DeltaE2000, Euclidean)
  - **Builder Pattern**: Added `GradientBuilder` for fluent gradient configuration with method chaining
  - **Factory Pattern**: Implemented `ColorParserFactory` for creating different types of color parsers with configurations
  - **Facade Pattern**: Added `ColorOperationsFacade` for simplified color operation interface
  - **Template Method Pattern**: Implemented `ColorMatchingTemplate` for standardized color matching algorithms
- **Fluent Interface**: GradientBuilder provides intuitive method chaining: `.start_color("#FF0000").end_color("#0000FF").ease_in_out().build()`
- **Parser Factory**: ColorParserFactory supports different parser types (Css, Full, Custom) with preset configurations
- **Unified Error Handling**: Created `parse_utils.rs` module with standardized parsing functions
- **Enhanced Documentation**: Updated README.md and EXAMPLES.md with comprehensive GoF pattern usage examples
- **Constants Management**: Added RGB_MAX_F32, PERCENTAGE_MULTIPLIER constants to reduce magic numbers

### Removed
- **Massive Code Cleanup**: Removed 15,000+ lines of outdated hardcoded color arrays
  - Eliminated unused `RAL_CLASSIC_DATA` and `RAL_DESIGN_DATA` arrays (15,387 lines)
  - Removed outdated `CSS_COLOR_DATA` array in favor of CSV-based system
  - Deleted obsolete `ral_data.rs` and `color_names.rs` modules
- **Code Duplication**: Eliminated 50+ lines of duplicate formatting code in color_formatter.rs

### Fixed
- **Code Duplication**: Eliminated 18+ instances of duplicate error handling patterns
- **Parser Efficiency**: CSS color parsing now uses CSV data instead of hardcoded arrays
- **Unified Architecture**: All color data now consistently loads from CSV files
- **Collection Formatting**: Unified all collection output to use consistent ColorMatch-based formatting

### Changed
- **Architecture**: Implemented design patterns for better maintainability and extensibility
- **Code Quality**: Reduced color.rs parsing logic from 39 lines to 3 lines using unified utilities
- **Constants**: Replaced magic numbers with named constants throughout color conversion functions

### Technical
- **Pattern Applications**:
  - Strategy: `create_strategy()` factory method with multiple distance algorithms
  - Builder: Validation and preset methods (`.linear()`, `.ease_in_out()`, etc.)
  - Factory: `create_fast()`, `create_comprehensive()`, `create_strict()` preset parsers
- **Refactoring**: Major code cleanup with unified functions replacing duplicated implementations
- **Testing**: Added comprehensive tests for all GoF pattern implementations (14 new tests)
- **Documentation**: Complete coverage of new features with practical examples

## [0.10.3] - 2025-07-16

### Fixed
- **CSS Color Collection**: CSS colors now properly search for closest matches instead of just displaying the input name
- **Unified Output Format**: All color collections (CSS, RAL Classic, RAL Design System+) now use the same display format
- **Code Duplication**: Removed duplicate formatting code by unifying `write_ral_design_collection` with the shared `write_collection_search_results` function

### Changed
- **CSS Color Matching**: CSS colors now show closest color matches with distance (ΔE) values, matching the RAL collections format
- **Function Unification**: Made `write_collection_search_results` more generic to accept collection name parameter
- **Test Updates**: Updated tests to match the new consistent output format

### Technical
- Enhanced `find_closest_css_colors` function to convert ColorMatch to RalMatch format for unified display
- Improved code organization and maintainability through unified formatting functions

## [0.10.2] - 2025-07-16

### Changed
- **Unified Color Collections Output**: Reorganized color formatter to display all color collection information (CSS, RAL Classic, RAL Design) in a single organized section
- **Modular Constants**: Moved `COLUMN_WIDTH` constant to `lib.rs` for cross-module accessibility
- **Improved Architecture**: Created separate, configurable functions for each color collection output

### Fixed
- **Test Suite**: Updated all tests to match the new unified output format
- **Code Organization**: Removed unused functions and improved code maintainability

### Technical
- Enhanced documentation for the color formatter module
- Improved error handling and consistent formatting across all collection outputs
- Made the system easily extensible for adding new color collections

## [0.10.0] - 2025-07-16

### Added
- **Unified Color Collection System**: Complete architectural redesign for color collections
  - **Universal Color Representation**: LAB-based internal storage for perceptually accurate comparisons
  - **Trait-Based Architecture**: Extensible system supporting different native color spaces (RGB, HLC, CMYK)
  - **Advanced Filtering**: Group-based filtering for RAL Classic (RAL 1000-9000) and RAL Design System+ (Hue, Lightness, Chromaticity)
  - **Library-Friendly API**: Designed for external library usage with clean, consistent interfaces
  - **Multiple Search Methods**: Closest match, exact name, luminance-based, and pattern-based searching
  - **Future-Proof Design**: Easily extensible for Pantone and other color systems

### Improved
- **RAL Group Filtering**: Filter RAL Classic colors by groups (RAL 1000, 2000, 3000, etc.)
- **RAL Design Filtering**: Filter by Hue groups (Red, Orange, Yellow, etc.), Lightness ranges, and Chromaticity ranges
- **Search Performance**: Optimized color matching using unified LAB-based distance calculations
- **Code Organization**: Separated color collection logic from CLI implementation

### Maintained
- **Backward Compatibility**: All existing APIs continue to work unchanged
- **CLI Compatibility**: Existing command-line interface remains identical
- **Test Coverage**: All existing functionality validated with 57+ passing tests

### Library Features
- **UnifiedColorManager**: High-level interface for multiple color collections
- **ColorCollection Trait**: Standardized interface for all color systems
- **SearchFilter**: Advanced filtering capabilities for precise color discovery
- **ColorMatch Results**: Rich match information with distance and confidence metrics

## [0.9.1] - 2025-07-15

### Fixed
- **RAL Color Output Format**: RAL colors now display full comprehensive analysis reports
  - **Complete Information**: RAL colors show RGB, HSL, LAB, XYZ, OKLCH format conversions
  - **WCAG Compliance**: Include WCAG relative luminance and contrast ratio data for RAL colors
  - **Consistency**: All color input types now provide identical comprehensive output format
  - **User Request**: Addressed user feedback about compact format being insufficient

### Removed
- **Compact RAL Format**: Removed short-form RAL color display function in favor of comprehensive reports

## [0.9.0] - 2025-01-15

### Added
- **Complete RAL Color System Support**: Comprehensive integration of RAL color standards
  - **RAL Classic**: 213 standardized colors with CIELAB 1976, CMYK, and LRV data
  - **RAL Design System+**: 1825 colors with CIELAB 1931 hue/lightness/chromaticity data
  - **Multiple Input Formats**: Supports both "RAL 1000" and "RAL1000" (with and without space)
  - **Name-based Lookup**: Search RAL colors by name (e.g., "signal yellow", "traffic red")
  - **Intelligent CSS Filtering**: Prevents conflicts between RAL names and common CSS color names
- **Separate Classification Matching**: Shows 2 closest colors from each classification separately
  - **RAL Classic**: Dedicated matching for users tied to classic color system
  - **RAL Design System+**: Separate results for extended color palette users
  - **Delta E Calculations**: Perceptually accurate color distance measurements
- **Enhanced Color Analysis**: Extended color-match command with RAL integration
  - **Comprehensive Reports**: Includes closest RAL matches for any input color
  - **Professional Output**: Clean formatting with proper classification names
  - **Full RAL Information**: Code, name, hex value, and color distance for each match

### Changed
- **Version**: Updated from 0.8.4 to 0.9.0 to reflect major RAL functionality addition
- **Test Suite**: Expanded to 37+ tests including comprehensive RAL color system validation
- **Documentation**: Updated README.md and EXAMPLES.md with RAL usage examples
- **Color Name Handling**: Fixed CSS color name capitalization to use proper lowercase format

### Fixed
- **CSS Color Priority**: Ensured common CSS colors (red, blue, etc.) are processed as CSS colors, not RAL names
- **Test Compatibility**: Updated test expectations to work with enhanced color output format
- **Import Cleanup**: Removed unused imports to eliminate compiler warnings

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
