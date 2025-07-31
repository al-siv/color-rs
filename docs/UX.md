# Color-rs User Experience Reference

This document provides comprehensive user experience guidelines, CLI interface documentation, and human-computer interaction (HCI) patterns for color-rs, designed with **functional programming principles** and user-centered design.

## Table of Contents

- [User Experience Philosophy](#user-experience-philosophy)
- [CLI Interface Reference](#cli-interface-reference)
- [Human-Computer Interaction Patterns](#human-computer-interaction-patterns)
- [Usability Guidelines](#usability-guidelines)
- [Accessibility Standards](#accessibility-standards)
- [Performance and Responsiveness](#performance-and-responsiveness)
- [Error Handling and Recovery](#error-handling-and-recovery)
- [Progressive Disclosure](#progressive-disclosure)

## User Experience Philosophy

### Functional Programming UX Benefits

Color-rs applies **functional programming principles** to create a superior user experience:

- **Predictable Behavior**: Pure functions ensure consistent, deterministic results
- **Immutable State**: No hidden side effects or unexpected state changes
- **Composable Operations**: Users can combine commands and options reliably
- **Error Transparency**: Explicit error handling provides clear feedback
- **Performance Predictability**: Pure functions enable consistent performance

### Design Principles

1. **Simplicity First**: Common operations should be simple, complex operations should be possible
2. **Discoverability**: Features should be discoverable through help and progressive disclosure
3. **Consistency**: Similar operations should work similarly across all commands
4. **Feedback**: Users should always know what's happening and what happened
5. **Efficiency**: Expert users should be able to work efficiently
6. **Forgiveness**: Users should be able to recover from mistakes easily

## CLI Interface Reference

### Global Command Structure

```bash
color-rs <COMMAND> [OPTIONS]
```

**Available Commands:**
- `color` - Analyze and convert colors between different color spaces
- `gradient` - Generate color gradients using LAB color space with cubic-bezier timing
- `help` - Print help information for commands and options

**Global Options:**
- `-h, --help` - Print help information
- `-V, --version` - Print version information

### Color Analysis Command

**Primary Use Case**: Comprehensive color analysis with format conversion, accessibility checking, and color matching.

#### Syntax
```bash
color-rs color [OPTIONS] <COLOR>
```

#### Arguments
- `<COLOR>` - Input color value supporting multiple formats:
  - **HEX**: `#FF0000`, `#ff0000`, `FF0000`
  - **RGB**: `rgb(255,0,0)`, `rgba(255,0,0,1.0)`
  - **HSL**: `hsl(0,100%,50%)`, `hsla(0,100%,50%,1.0)`
  - **Named Colors**: `red`, `blue`, `forestgreen`
  - **RAL Classic**: `RAL 3020`, `RAL1000`
  - **RAL Design System+**: `RAL 010 40 30`
  - **LAB**: `lab(60.18, 62.06, 54.34)`

#### Core Options

**Distance Calculation**:
```bash
--distance-method <METHOD>     # Color distance algorithm [default: lch]
```
- `lch` - LCH-based calculation (perceptually uniform, default)
- `delta-e-2000` - CIE Delta E 2000 (most accurate)
- `delta-e-76` - CIE Delta E 1976 (faster)
- `euclidean-lab` - Euclidean distance in LAB space

**Color Schemes**:
```bash
--schemes <STRATEGY>           # Color harmony strategy [default: lab]
```
- `lab` - LAB color space schemes (perceptually uniform)
- `hsl` - HSL color space schemes

**Luminance Adjustment**:
```bash
-r, --relative-luminance <VALUE>  # WCAG relative luminance (0.0-1.0)
-l, --luminance <VALUE>           # Lab luminance value
```

**Output Control**:
```bash
-o, --output <FORMAT>          # Output format [default: yaml]
    # yaml | toml
    
--func <FILTER>                # Selective output filtering
    # [all] | [input] | [conversion] | [contrast] | [grayscale] 
    # [color_collections] | [color_schemes]
    
-f, --file <FILENAME>          # Output to file (auto-extension)
```

#### Usage Examples

```bash
# Basic color analysis
color-rs color "#FF5733"
color-rs color "rgb(255, 87, 51)"
color-rs color "red"

# RAL color system
color-rs color "RAL 3020"
color-rs color "traffic red"

# Advanced distance methods
color-rs color "#FF5733" --distance-method delta-e-2000

# Selective output filtering
color-rs color "#FF5733" --func "[input,conversion]"
color-rs color "blue" --func "[contrast.wcag21_relative_luminance]"

# File output
color-rs color "#FF5733" --output toml --file analysis
# Creates: analysis.toml

# Luminance manipulation
color-rs color "#FF5733" --relative-luminance 0.5
```

### Gradient Generation Command

**Primary Use Case**: Generate smooth, perceptually accurate color gradients with mathematical precision.

#### Syntax
```bash
color-rs gradient [OPTIONS] <START_COLOR> <END_COLOR>
```

#### Arguments
- `<START_COLOR>` - Starting color (any supported format)
- `<END_COLOR>` - Ending color (any supported format)

#### Core Options

**Position Control**:
```bash
-s, --start-position <PERCENT>  # Starting position [default: 0]
-e, --end-position <PERCENT>    # Ending position [default: 100]
```

**Easing Control**:
```bash
--ease-in <VALUE>              # Ease-in control (0.0-1.0) [default: 0.65]
--ease-out <VALUE>             # Ease-out control (0.0-1.0) [default: 0.35]
```

**Gradient Steps**:
```bash
-t, --step <STEP>              # Output every X percent
-g, --stops <STOPS>            # Number of intelligent stops [default: 5]
--stops-simple                 # Use equal spacing instead of intelligent
```

**Image Generation**:
```bash
--svg                          # Generate SVG image
--png                          # Generate PNG image
--no-legend                    # Disable image legend
--width <WIDTH>                # Image width in pixels [default: 1000]
-v, --svg-name <NAME>          # SVG filename [default: gradient.svg]
-p, --png-name <NAME>          # PNG filename [default: gradient.png]
```

#### Usage Examples

```bash
# Basic gradients
color-rs gradient red blue
color-rs gradient "#FF0000" "#0000FF"

# Partial gradients
color-rs gradient red blue --start-position 20 --end-position 80

# Custom easing (CSS ease-in-out equivalent)
color-rs gradient red blue --ease-in 0.42 --ease-out 0.58

# Intelligent vs. simple stops
color-rs gradient red blue --stops 8          # Curve-based placement
color-rs gradient red blue --stops 6 --stops-simple  # Equal spacing

# Image generation
color-rs gradient red blue --svg --png --width 1600 --no-legend

# File output
color-rs gradient red blue --output toml --file gradient
```

## Human-Computer Interaction Patterns

### Progressive Disclosure Pattern

Color-rs implements progressive disclosure to balance simplicity with power:

#### Level 1: Basic Usage
```bash
# Simplest possible usage
color-rs color red
color-rs gradient red blue
```

#### Level 2: Common Customization
```bash
# Add output format and distance method
color-rs color red --output toml --distance-method delta-e-2000
color-rs gradient red blue --svg --stops 10
```

#### Level 3: Advanced Features
```bash
# Full feature utilization
color-rs color "RAL 010 40 30" --distance-method delta-e-2000 \
  --schemes lab --func "[conversion,color_collections]" \
  --output toml --file analysis

color-rs gradient "lab(50, 20, -30)" "lch(70, 40, 120)" \
  --ease-in 0.25 --ease-out 0.75 --stops 15 \
  --svg --png --width 2000 --no-legend
```

### Command Composition Pattern

Color-rs supports functional composition of operations:

```bash
# Analyze color, then use result in gradient
color-rs color "RAL 3020" --func "[conversion.hex]" | \
  xargs -I {} color-rs gradient {} blue

# Batch processing with consistent distance method
for color in red green blue; do
  color-rs color "$color" --distance-method delta-e-2000 \
    --output toml --file "analysis_$color"
done
```

### Default Value Pattern

Sensible defaults minimize cognitive load:

```bash
# These commands are equivalent due to defaults:
color-rs color red
color-rs color red --output yaml --distance-method lch --schemes lab

color-rs gradient red blue  
color-rs gradient red blue --ease-in 0.65 --ease-out 0.35 --stops 5 \
  --start-position 0 --end-position 100
```

### Format Flexibility Pattern

Accept multiple input formats without user specification:

```bash
# All of these work seamlessly:
color-rs color "#FF0000"        # Hex with hash
color-rs color "FF0000"         # Hex without hash
color-rs color "rgb(255,0,0)"   # RGB function
color-rs color "red"            # Named color
color-rs color "RAL 3020"       # RAL code
color-rs color "traffic red"    # RAL name
```

### Consistent Option Pattern

Similar options work the same way across commands:

```bash
# Output options work identically:
color-rs color red --output toml --file result
color-rs gradient red blue --output toml --file result

# Help options work consistently:
color-rs color --help
color-rs gradient --help
color-rs --help
```

## Usability Guidelines

### Discoverability

**Help System Design**:
```bash
# Hierarchical help system
color-rs --help                    # Global help
color-rs color --help              # Command-specific help
color-rs gradient --help           # Command-specific help
```

**Self-Documenting Options**:
- Clear, descriptive option names: `--distance-method` not `--dm`
- Consistent naming patterns: `--start-position`, `--end-position`
- Meaningful defaults shown in help

### Error Prevention

**Input Validation**:
```bash
# Clear validation messages
$ color-rs gradient red blue --start-position 80 --end-position 20
Error: Start position (80) must be less than end position (20)

# Range validation with suggestions
$ color-rs gradient red blue --ease-in 1.5
Error: Ease-in value must be between 0.0 and 1.0, got 1.5
Suggestion: Try --ease-in 1.0 for maximum ease-in effect
```

**Format Detection**:
```bash
# Helpful parsing errors
$ color-rs color "invalid-color"
Error: Unable to parse color: "invalid-color"
Supported formats: HEX (#FF0000), RGB (rgb(255,0,0)), HSL (hsl(0,100%,50%)), 
Named (red), RAL Classic (RAL 3020), RAL Design (RAL 010 40 30)
```

### Feedback and Confirmation

**Operation Confirmation**:
```bash
# File creation feedback
$ color-rs color red --output toml --file analysis
Analysis written to: analysis.toml (1.2 KB)

# Image generation feedback  
$ color-rs gradient red blue --svg --png
Generated: gradient.svg (2.4 KB)
Generated: gradient.png (45.7 KB)
```

**Progress Indication**:
```bash
# For longer operations
$ color-rs gradient red blue --stops 1000 --png --width 8000
Calculating gradient stops... ████████████████████ 100% (1000/1000)
Rendering PNG image... ████████████████████ 100%
Generated: gradient.png (2.3 MB)
```

### Efficiency for Expert Users

**Short Option Forms**:
```bash
# Common operations have short forms
color-rs color red -o toml -f result        # vs --output toml --file result
color-rs gradient red blue -g 10 --svg      # vs --stops 10 --svg
```

**Batch Operation Support**:
```bash
# Shell-friendly output for automation
color-rs color red --func "[conversion.hex]" --output yaml | \
  grep "hex:" | cut -d'"' -f2
```

## Accessibility Standards

### WCAG Integration

Color-rs directly supports WCAG accessibility standards:

**Contrast Analysis**:
```bash
# WCAG 2.1 compliance checking
color-rs color "#FF5733" 
# Output includes:
# - WCAG 2.1 relative luminance
# - Contrast ratios vs white/black
# - AA/AAA compliance assessment
```

**Luminance Manipulation**:
```bash
# Create accessible color variants
color-rs color "#FF5733" --relative-luminance 0.18  # AA large text minimum
color-rs color "#FF5733" --relative-luminance 0.04  # AAA large text minimum
```

### Color-Blind Accessibility

**Distance Method Consideration**:
- LCH distance (default) provides better perception for color vision differences
- Delta E 2000 method accounts for human visual system variations

**Output Format Accessibility**:
- Structured data (YAML/TOML) accessible to screen readers
- Clear numerical values instead of color-only indicators
- Comprehensive metadata for assistive technologies

### Terminal Accessibility

**High Contrast Support**:
- Clear text output without relying on color alone
- Structured formatting that works with screen readers
- Consistent spacing and alignment

## Performance and Responsiveness

### Response Time Guidelines

| Operation | Target Time | Typical Time |
|-----------|-------------|--------------|
| Simple color analysis | < 50ms | ~20ms |
| Complex distance calculation | < 100ms | ~60ms |
| Small gradient (≤20 stops) | < 200ms | ~80ms |
| Large gradient (≤100 stops) | < 500ms | ~200ms |
| SVG generation | < 300ms | ~150ms |
| PNG generation (1000px) | < 1s | ~400ms |
| PNG generation (4000px) | < 3s | ~1.2s |

### Performance Feedback

**Long Operation Indicators**:
```bash
# For operations >2 seconds
$ color-rs gradient red blue --png --width 8000
Calculating gradient... ████████████████████ 100%
Rendering PNG (8000x1600)... ████████████████████ 100%
```

**Memory Usage Awareness**:
- Efficient streaming for large gradients
- Memory usage warnings for extreme parameters
- Graceful degradation for resource constraints

### Functional Programming Performance Benefits

- **Pure Functions**: Enable compiler optimizations and predictable performance
- **Immutable Data**: Reduce memory allocation overhead
- **No Side Effects**: Eliminate performance unpredictability from hidden state
- **Composable Operations**: Users can build efficient pipelines

## Error Handling and Recovery

### Error Message Design

**Clear Problem Description**:
```bash
$ color-rs color "invalid-hex"
Error: Invalid HEX color format: "invalid-hex"
Expected: 6-character HEX code (e.g., #FF0000 or FF0000)
```

**Actionable Solutions**:
```bash
$ color-rs gradient red blue --step 0
Error: Gradient step must be greater than 0, got 0
Suggestion: Try --step 5 for 5% intervals, or use --stops 10 for intelligent placement
```

**Context-Aware Help**:
```bash
$ color-rs color unkown-color
Error: Unknown color name: "unkown-color"
Did you mean: "unknown" is not a standard color name
Suggestion: Try "red", "blue", "green", or use HEX format like #FF0000
Similar names: none found
```

### Graceful Degradation

**Partial Success Handling**:
```bash
$ color-rs color red --func "[conversion,invalid_block]"
Warning: Unknown filter block "invalid_block", ignoring
Showing: conversion block only
```

**Resource Constraint Handling**:
```bash
$ color-rs gradient red blue --png --width 50000
Warning: Requested width (50000px) may cause memory issues
Limiting to maximum safe width: 10000px
Continue? [y/N]
```

### Recovery Guidance

**Command Correction**:
```bash
$ color-rs gradent red blue  # Typo in command
Error: Unknown command 'gradent'
Did you mean: gradient
Usage: color-rs gradient <START_COLOR> <END_COLOR> [OPTIONS]
```

**Option Validation**:
```bash
$ color-rs color red --distance-method unknown
Error: Unknown distance method: "unknown"
Available methods: lch, delta-e-2000, delta-e-76, euclidean-lab
Default: lch
```

## Progressive Disclosure

### Information Architecture

**Layered Complexity**:

1. **Essential Information** (always shown):
   - Input color and detected format
   - Primary output (HEX, RGB)
   - Basic validation status

2. **Standard Information** (shown by default):
   - All color space conversions
   - WCAG compliance data
   - Color collection matches

3. **Advanced Information** (shown with flags):
   - Detailed color schemes
   - Complex color harmonies
   - Technical metadata

### Contextual Help

**Just-in-Time Assistance**:
```bash
# Context-sensitive tips
$ color-rs color red --help
Color Analysis Command

Analyze colors with comprehensive format conversion and accessibility checking.

USAGE:
    color-rs color [OPTIONS] <COLOR>

Quick Examples:
    color-rs color "#FF0000"     # Analyze HEX color
    color-rs color "red"         # Analyze named color
    color-rs color "RAL 3020"    # Analyze RAL color

For more examples: color-rs help color-examples
```

### Feature Discovery

**Guided Feature Introduction**:
- Help examples progress from simple to complex
- Option descriptions include use cases
- Related options grouped logically
- Performance implications clearly stated

This UX reference ensures that color-rs provides a powerful yet approachable interface that scales from simple color analysis to complex color science applications, all while maintaining the functional programming benefits of predictability, composability, and performance.
