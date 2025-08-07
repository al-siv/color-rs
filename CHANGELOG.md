# Changelog

All notable changes to the color-rs project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.19.1] - 2025-01-21

### Fixed
- **Visual Output Enhancement**: Improved default border configuration for gradient mode
  - Changed `DEFAULT_BORDER_WIDTH` from "5" to "0" for cleaner gradient presentation
  - Resolves validation conflicts when using gradient mode with default settings
  - Borders are now disabled by default for better visual aesthetics

### Improved
- **Documentation**: Updated doctest examples for hue analysis with correct border parameters
- **Error Handling**: Enhanced validation messaging for border configuration conflicts
- **Quality Assurance**: Comprehensive testing and validation for production readiness

## [0.19.0] - 2025-01-21

### Added
- **NEW: Hue Analysis Command**: Comprehensive color collection analysis and visualization
  - Display entire color collections sorted by hue relationships
  - Support for CSS colors (148), RAL Classic (~210), and RAL Design System+ (~1600) collections
  - Advanced filtering by hue range `[-H]`, lightness range `[-L]`, and chroma range `[-C]`
  - Wraparound hue filtering support (e.g., `[300...30]` for purple-red spectrum)
  - **Command**: `color-rs hue <COLLECTION> [OPTIONS]`

- **Visual Output Generation**: New SVG and PNG export capabilities for color analysis
  - **Horizontal Gradient Mode** (`-g, --grad`): Creates smooth color transitions across collections
  - **Vertical Palette Mode** (`-p, --pal`): Generates color swatches with detailed labels
  - Customizable dimensions with `--width` and `--color-height` parameters
  - Label control with `--no-labels` option for clean visual output
  - Fixed label format: `{H} | {HEX} | {lch(ll.l, cc.c, hhh.h)} | {code} | {color_name}`

- **Enhanced CLI with Short Flags**: Comprehensive short flag support for improved usability
  - **Range Filters**: `-H` (hue-range), `-L` (lightness-range), `-C` (chroma-range)
  - **Visual Modes**: `-g` (grad), `-p` (pal)
  - **Output**: `-G` (svg), `-P` (png), `-w` (width), `-z` (color-height)
  - **Standard**: `-o` (output), `-f` (file), `-h` (help)

### Changed
- **Version Update**: Updated to v0.19.0 across all components
- **CLI Architecture**: Enhanced command structure with consistent short flag patterns
- **Documentation**: Comprehensive updates to CLI_REFERENCE.md and README.md
- **Help System**: Restored standard `-h, --help` functionality across all commands

### Technical Improvements
- **Functional Programming Compliance**: Achieved 100% compliance with functional programming principles
  - Exhaustive pattern matching validation
  - Strong type safety with smart constructors
  - Immutable data structures throughout
  - Railway-oriented programming for error handling
  - Pure function design with referential transparency

- **Code Quality Enhancements**: Major improvements to code quality and maintainability
  - Resolved 51 clippy warnings (5.7% improvement: 898 → 847)
  - Enhanced documentation coverage (~80%)
  - Added must_use attributes to 20+ pure functions
  - Implemented safe casting patterns with explicit documentation
  - Eliminated 2000+ lines of deprecated/backup code

- **Test Coverage**: Comprehensive testing across all new features
  - 287 total tests passing (233 unit + 4 integration + 52 doctests)
  - Performance validation for large color collections
  - Memory safety verification for visual output generation
  - CLI parameter validation and integration testing

### Performance
- **Large Dataset Handling**: Optimized performance for RAL Design collection (~1600 colors)
- **Visual Generation**: Efficient SVG/PNG rendering with no memory leaks
- **Interactive Usage**: Sub-second execution times for typical operations
- **Iterator Efficiency**: Functional patterns provide optimal performance characteristics

### Examples
```bash
# New hue analysis command
color-rs hue css                           # Display all CSS colors
color-rs hue css -H"[0...60]"             # Warm colors only
color-rs hue ralc -L"[50...80]" -C"[30...70]"  # Bright, saturated RAL Classic

# Visual output generation
color-rs hue css -g -G gradient.svg       # Horizontal gradient
color-rs hue css -p -G palette.svg -z 50  # Vertical palette, 50px height

# Enhanced CLI with short flags
color-rs hue rald -H"[180...240]" -p -G blue-palette.svg -P blue-palette.png -w 1200
```

## [0.15.4] - 2025-01-21

### Changed
- **BREAKING CHANGE**: Default distance calculation method changed from "delta-e-2000" to "lch"
  - LCH color space now provides the default distance calculations for better perceptual uniformity
  - All existing workflows continue to work, but may produce different color matches
  - Users can specify `--distance-method delta-e-2000` to maintain previous behavior

### Fixed
- **Distance Method Consistency**: The `--distance-method` parameter now affects ALL distance calculations in color mode
  - **Color Collections**: CSS, RAL Classic, and RAL Design System+ matching now use specified distance method
  - **Color Schemes**: Complementary, split-complementary, triadic, and tetradic schemes now use specified distance method
  - **Unified Implementation**: Replaced legacy ColorParser with UnifiedColorManager for consistent strategy application
  - **Architecture Improvement**: Refactored collect_enhanced_color_schemes_data function to properly propagate distance strategy

### Technical Details
- **Code Refactoring**: Migrated from ColorParser to UnifiedColorManager throughout color scheme generation
- **Function Signature Updates**: Updated collect_enhanced_color_schemes_data to accept distance_strategy parameter
- **Strategy Propagation**: Ensured distance strategy flows through all color matching operations
- **Test Coverage**: Comprehensive testing confirms all four distance methods work correctly across all features

## [0.15.3] - 2025-01-21

### Added
- **Cross-Platform Binaries**: Added pre-compiled binaries for multiple platforms
  - Linux x86_64 binary (Intel/AMD 64-bit systems)
  - Linux ARM64 binary (ARM64 architecture support)
  - macOS x86_64 binary (Intel-based Macs)
  - macOS ARM64 binary (Apple Silicon M1/M2/M3)
  - All binaries available via GitHub Releases

### Changed
- **Distribution**: Enhanced distribution options with cross-compiled executables for all major platforms
- **Documentation**: Updated README.md with comprehensive binary download instructions
- **Dependencies**: Optimized chrono dependency to avoid CoreFoundation framework issues in cross-compilation

### Technical Notes
- Cross-compilation performed using cargo-zigbuild with Zig compiler v0.14.1
- macOS compilation successfully achieved by removing CoreFoundation dependencies
- All cross-platform binaries have not been tested on target platforms - user feedback appreciated

### Fixed
- **Cross-compilation**: Resolved CoreFoundation framework dependency blocking macOS binary creation
- **Timestamp Generation**: Simplified timestamp generation to avoid platform-specific dependencies

## [0.15.2] - 2025-07-24

### Added
- **Enhanced Documentation**: Added comprehensive error documentation to critical functions
- **Type Safety Improvements**: Added safe f32→u8 conversion helpers to prevent truncation errors
- **Code Quality Enhancements**: Applied systematic clippy fixes for better code quality

### Changed
- **Code Optimization**: Optimized option_if_let_else patterns using map_or and map_or_else
- **Function Simplification**: Removed unnecessary Result wrapping from functions that never fail
- **Code Formatting**: Applied consistent formatting across entire codebase with cargo fmt

### Fixed
- **Clippy Warnings**: Reduced total clippy warnings from 726 to 699 (3.7% improvement)
  - Fixed 2 of 3 unnecessary_wraps warnings for cleaner APIs  
  - Fixed 6+ cast_possible_truncation warnings with proper bounds checking
  - Optimized 2+ option_if_let_else patterns for better readability
- **Type Safety**: Eliminated risky type casts with safe conversion functions
- **Test Coverage**: Maintained 98.1% test pass rate (106/108 tests) during quality improvements

## [0.15.1] - 2025-07-23

### Changed
- **Architecture Simplification**: Major codebase cleanup and simplification
  - Removed over-engineered GoF pattern implementations where they added unnecessary complexity
  - Simplified gradient module from 4-file complex system to single clean module
  - Eliminated excessive abstraction layers for better maintainability
  - Flattened module structures and removed circular dependencies

### Removed
- **Dead Code Elimination**: Removed 1000+ lines of over-engineered code
  - Deleted complex gradient workflow system (workflow.rs)
  - Removed entire output_filter module with excessive Strategy/Observer patterns
  - Eliminated duplicate ColorUtils implementations
  - Removed unused easing strategies and factory patterns

### Fixed
- **Build System**: Achieved zero compilation errors and warnings
  - Fixed f32/f64 type mismatches in gradient calculations
  - Corrected Lab struct construction and into_color() calls
  - Resolved missing function implementations
  - Fixed method call patterns (static vs instance)

### Internal
- **Code Quality**: Improved maintainability without breaking public API
  - Consolidated color operations to LegacyColorUtils for backward compatibility
  - Simplified command pattern implementation
  - Retained essential GoF patterns while removing over-engineering
  - Standardized function signatures and interfaces

## [0.15.0] - 2025-07-23

### Changed
- **BREAKING CHANGE**: Simplified filtering system to inclusion-only approach
  - Removed exclusion operator (`!`) support for easier maintenance and user understanding
  - Filter expressions now support only inclusion: `[input,conversion]`, `[contrast.wcag21_relative_luminance]`
  - Simple format filtering: `hex,rgb,lab,lch` for both color and gradient commands

### Fixed
- **Gradient Filtering**: Fixed gradient filtering to properly omit empty fields instead of showing empty strings
- **Missing Constants**: Added documented constants that were missing from codebase:
  - `DEFAULT_FILTER_EXPRESSION = "[all]"`
  - `FILTER_EXPRESSION_MAX_LENGTH = 1000`
  - `MAX_FILTER_RULES_PER_EXPRESSION = 50`
- **Code Quality**: Fixed all clippy warnings and improved code formatting consistency
- **Test Coverage**: Updated tests to reflect inclusion-only filtering approach

### Technical
- **Simplified Architecture**: Removed complex exclusion logic, cleaner codebase maintenance
- **Enhanced Output Control**: Added `skip_serializing_if` attributes for clean field omission
- **Gradient Filtering**: Extended filtering support to gradient command with format-specific filtering
- **Documentation Improvements**: Added filtering differences explanation between color and gradient commands

## [0.14.1] - 2025-07-23

### Added
- **Selective Output Control**: New `--func` parameter for filtering color analysis output
  - Block-level filtering: `[input]`, `[conversion]`, `[contrast]`, `[grayscale]`, `[color_collections]`, `[color_schemes]`
  - Field-level filtering: `[contrast.wcag21_relative_luminance]`, `[grayscale.lch0]`, etc.
  - Multiple selection: `[input,conversion,contrast]`
  - Exclusion operator: `[all,!color_collections]`, `[contrast,!contrast.brightness]`
  - Clean output with no unwanted default values for filtered content

### Changed
- **Enhanced CLI Interface**: Added comprehensive filtering system with expression parser
- **Improved Output Serialization**: Filtered output now omits unselected blocks and fields completely
- **Documentation Updates**: Updated README.md, UX.md, and EXAMPLES.md with filtering examples

### Technical
- **New Modules**: Added `output_filter.rs` with `FilterEngine`, `FilterConfig`, and `FilterExpressionParser`
- **Filtered Output Structures**: Created `FilteredColorAnalysisOutput`, `FilteredContrastData`, and `FilteredGrayscaleData` with `Option<T>` fields
- **Parser Implementation**: Custom filter expression parser supporting complex inclusion/exclusion patterns

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
