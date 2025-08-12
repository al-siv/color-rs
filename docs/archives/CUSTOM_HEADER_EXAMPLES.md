# Custom Header Examples - Sprint 0.19.2

This document demonstrates the new `--header-text` feature introduced in version 0.19.2.

## Overview
The `--header-text` option allows users to replace the default collection title with custom text in palette mode, providing enhanced personalization for visual outputs.

## Basic Usage

### Default Behavior (Before)
```bash
cargo run -- hue css --h-range "[0...60]" --pal --svg default_header.svg
```
**Result**: Header shows "CSS Collection Color Palette (15 colors)"

### Custom Header (New)
```bash
cargo run -- hue css --h-range "[0...60]" --pal --svg custom_header.svg --header-text "Warm Color Spectrum"
```
**Result**: Header shows "Warm Color Spectrum"

## Practical Examples

### 1. Collection-Specific Customization
```bash
# RAL Classic with custom branding
cargo run -- hue ralc --h-range "[0...60]" --pal --svg ral_warm.svg \
  --header-text "Warm RAL Classic Colors - Project Phoenix"

# CSS colors for web design
cargo run -- hue css --h-range "[180...240]" --pal --svg web_blues.svg \
  --header-text "Web-Safe Blue Palette for UI Design"
```

### 2. Project-Specific Titles
```bash
# Interior design project
cargo run -- hue rald --h-range "[120...180]" --pal --svg interior_greens.svg \
  --header-text "Living Room Green Palette - Spring 2025"

# Brand color exploration
cargo run -- hue css --pal --svg brand_exploration.svg \
  --header-text "Brand Color Analysis - Complete CSS Spectrum"
```

### 3. Special Characters and Formatting
```bash
# With symbols and degrees
cargo run -- hue css --h-range "[270...330]" --pal --svg purple_range.svg \
  --header-text "Purple & Magenta Colors (270°-330°) - Special Analysis"

# With project codes
cargo run -- hue ralc --h-range "[90...150]" --pal --svg project_greens.svg \
  --header-text "Project #2025-GRN: Natural Green Palette"
```

### 4. Multi-Format Output
```bash
# Generate both SVG and PNG with custom header
cargo run -- hue css --h-range "[30...90]" --pal \
  --svg yellow_orange.svg --png yellow_orange.png \
  --header-text "Sunshine Palette - Yellow to Orange Transition"
```

## Advanced Customization

### Combined with Other Options
```bash
# Custom header with styling options
cargo run -- hue ralc --h-range "[200...260]" --pal --svg styled_blues.svg \
  --header-text "Corporate Blue Palette - Professional Tones" \
  --color-height 80 --font-size 16 --border-width 2 --border-color "#333333"
```

### Empty or Minimal Text
```bash
# Minimal header
cargo run -- hue css --h-range "[0...30]" --pal --svg minimal.svg \
  --header-text "Red"

# Note: Empty text is possible but not recommended for clarity
```

## Technical Details

### Compatibility
- **Works with**: `--pal` (palette mode) only
- **File formats**: Both SVG and PNG output
- **Collections**: CSS, RAL Classic, RAL Design
- **Backward compatibility**: 100% - existing commands unchanged

### Character Support
- **Special characters**: Symbols, degrees (°), parentheses, etc.
- **Unicode**: Full Unicode support for international text
- **Length**: No enforced limit (reasonable length recommended for visual clarity)

### Integration with Existing Features
- Combines seamlessly with all filter options (`--h-range`, `--l-range`, `--c-range`)
- Works with all styling options (`--color-height`, `--font-size`, `--border-width`, etc.)
- Supports both SVG and PNG generation

## Best Practices

### 1. Descriptive Headers
```bash
# Good: Descriptive and informative
--header-text "Warm Earth Tones for Autumn Collection"

# Avoid: Too generic
--header-text "Colors"
```

### 2. Project Context
```bash
# Include project or purpose context
--header-text "Brand Identity Colors - Primary Palette"
--header-text "Website Accessibility Compliant Blues"
--header-text "Print Design - CMYK Safe Greens"
```

### 3. Technical Specifications
```bash
# Include technical details when relevant
--header-text "RAL Design Palette (50-70% Lightness Range)"
--header-text "High Chroma Colors for Digital Displays"
```

## Version Information
- **Feature introduced**: v0.19.2
- **CLI option**: `--header-text <TEXT>`
- **Status**: Stable and production-ready
- **Testing**: Comprehensive validation across all supported collections and output formats
