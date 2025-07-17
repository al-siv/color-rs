# Color-rs CLI Reference

This document provides comprehensive reference for all color-rs command-line options, subcommands, flags, and environment variables.

## Table of Contents

- [Global Options](#global-options)
- [Subcommands](#subcommands)
- [Gradient Command](#gradient-command)
- [Color-Match Command](#color-match-command)
- [Environment Variables](#environment-variables)
- [Usage Examples](#usage-examples)

## Global Options

The color-rs CLI application supports the following global options that apply to all subcommands:

```bash
color-rs [SUBCOMMAND] [OPTIONS]
```

### Available Global Flags

- `--help, -h`: Display help information
- `--version, -V`: Display version information

## Subcommands

### Overview

color-rs provides two main subcommands:

1. **`gradient`** - Generate color gradients using LAB color space with cubic-bezier timing
2. **`color-match`** - Match and convert colors between different color spaces

---

## Gradient Command

Generate perceptually uniform color gradients with professional easing functions.

### Basic Syntax

```bash
color-rs gradient <START_COLOR> <END_COLOR> [OPTIONS]
```

### Positional Arguments

- `<START_COLOR>` - Starting color (required)
  - **Formats**: HEX (#FF0000), RGB (rgb(255,0,0)), HSL (hsl(0,100%,50%)), named colors (red)
  - **RAL Support**: RAL Classic (RAL 3020) and RAL Design+ (H040L50C70)

- `<END_COLOR>` - Ending color (required)
  - **Formats**: Same as START_COLOR

### Options

#### Position Control
- `--start-position <PERCENT>` - Starting position as percentage (default: 0%)
- `--end-position <PERCENT>` - Ending position as percentage (default: 100%)

#### Easing Control  
- `--ease-in <FLOAT>` - Ease-in control point for cubic-bezier (0.0-1.0, default: 0.65)
- `--ease-out <FLOAT>` - Ease-out control point for cubic-bezier (0.0-1.0, default: 0.35)

#### Output Control
- `--grad-step <PERCENT>` - Output gradient values every X percent (default: 5%)
- `--grad-stops <COUNT>` - Number of intelligently placed gradient stops using curve derivatives
- `--grad-stops-simple <COUNT>` - Number of equally spaced gradient stops

**Note**: `--grad-step`, `--grad-stops`, and `--grad-stops-simple` are mutually exclusive.

#### Image Generation
- `--svg` - Generate SVG image of the gradient
- `--png` - Generate PNG image of the gradient  
- `--width <PIXELS>` - Width of the image in pixels (default: 1000)
- `--svg-name <FILENAME>` - Output filename for SVG image (default: gradient.svg)
- `--png-name <FILENAME>` - Output filename for PNG image (default: gradient.png)
- `--no-legend` - Disable legend/caption on gradient images (only valid with --svg or --png)

### Usage Examples

#### Basic Gradient Generation

```bash
# Simple red to blue gradient
color-rs gradient red blue
```

```bash
# HEX colors with default 5% steps
color-rs gradient "#FF0000" "#0000FF"
```

```bash
# Mixed color formats
color-rs gradient "rgb(255, 107, 53)" "hsl(270, 100%, 50%)"
```

#### Advanced Gradient Control

```bash
# Custom position range with ease-in-out timing
color-rs gradient "#FF6B35" "#7209B7" --start-position 20 --end-position 80 --ease-in 0.42 --ease-out 0.58
```

```bash
# 10% gradient steps with custom easing
color-rs gradient red blue --grad-step 10 --ease-in 0.25 --ease-out 0.75
```

```bash
# Intelligent gradient stops based on curve analysis
color-rs gradient "RAL 3020" "RAL 5005" --grad-stops 8
```

#### Image Output

```bash
# Generate SVG with default settings
color-rs gradient red blue --svg
```

```bash
# Generate both SVG and PNG with custom dimensions
color-rs gradient "#FF6B35" "#7209B7" --svg --png --width 1600
```

```bash
# Custom filenames with no legend
color-rs gradient "signal yellow" "traffic red" --svg --svg-name "corporate-gradient.svg" --no-legend
```

#### RAL Color System

```bash
# RAL Classic colors
color-rs gradient "RAL 1000" "RAL 5005"
```

```bash
# RAL Design System+ colors  
color-rs gradient "H010L20C10" "H040L50C70" --svg --width 800
```

```bash
# RAL named colors
color-rs gradient "signal yellow" "pure blue" --grad-stops 6
```

---

## Color-Match Command

Match and analyze colors with comprehensive format conversion and WCAG compliance data.

### Basic Syntax

```bash
color-rs color-match <COLOR> [OPTIONS]
```

### Positional Arguments

- `<COLOR>` - Input color value (required)
  - **Formats**: HEX, RGB, RGBA, HSL, HSLA, named colors, RAL colors
  - **Examples**: #FF5722, rgb(255,87,34), hsl(14,100%,57%), "deep orange", "RAL 3020"

### Options

- `--distance-method <METHOD>` - Distance calculation method for color matching (default: delta-e-2000)
  - **Available methods**:
    - `delta-e-76` - Fast CIE Delta E 1976 formula
    - `delta-e-2000` - Industry-standard CIE Delta E 2000 (perceptually accurate)
    - `euclidean-lab` - Simple Euclidean distance in LAB space

### Usage Examples

#### Basic Color Analysis

```bash
# Analyze a HEX color
color-rs color-match "#FF5722"
```

Output includes comprehensive color format conversions (RGB, HEX, HSL, HSB, CMYK, LAB, XYZ, OKLCH), WCAG compliance data, closest matches from CSS/RAL collections, and color harmony schemes calculated automatically in both HSL and Lab color space strategies.

```bash
# Analyze RGB color with comprehensive output
color-rs color-match "rgb(255, 87, 34)"
```

```bash
# Named color analysis
color-rs color-match "deep orange"
```

#### RAL Color Matching

```bash
# RAL Classic color analysis
color-rs color-match "RAL 3020"
```

```bash
# RAL Design System+ color
color-rs color-match "H040L50C70"  
```

```bash
# Search by RAL color name
color-rs color-match "traffic red"
```

#### Advanced Distance Calculations

```bash
# Use fast Delta E 76 algorithm
color-rs color-match "#3498DB" --distance-method delta-e-76
```

```bash
# Use Euclidean distance for rapid matching
color-rs color-match "rgb(52, 152, 219)" --distance-method euclidean-lab
```

#### Complex Color Formats

```bash
# HSL color with alpha
color-rs color-match "hsla(207, 74%, 53%, 0.8)"
```

```bash
# RGBA format
color-rs color-match "rgba(52, 152, 219, 0.9)"
```

---

## Environment Variables

Currently, color-rs does not use environment variables for configuration. All settings are controlled via command-line arguments.

Future versions may support:

- `COLOR_RS_CONFIG_PATH` - Path to configuration file
- `COLOR_RS_CACHE_DIR` - Directory for caching color collection data  
- `COLOR_RS_DEFAULT_WIDTH` - Default image width
- `COLOR_RS_DEFAULT_DISTANCE_METHOD` - Default color distance calculation method

---

## Output Formats

### Terminal Table Output

All commands output beautiful terminal tables with:

- Color information with RGB, HEX, HSL, HSB, CMYK, LAB, XYZ, OKLCH values
- WCAG compliance data (relative luminance, contrast ratios)
- RAL color matches with distance calculations
- Color schemes calculated in both HSL and Lab color space strategies
- Right-aligned numeric columns for easy reading

### Image Outputs

#### SVG Format
- Scalable vector graphics with optional legends
- CSS-compatible color values
- Professional typography using system fonts

#### PNG Format  
- High-quality raster images
- Configurable dimensions (1:5 aspect ratio maintained)
- Embedded metadata with color information

---

## Error Handling

The CLI provides helpful error messages for common issues:

### Invalid Color Formats
```bash
$ color-rs gradient "invalid-color" blue
Error: Unable to parse color: invalid-color
```

### Invalid Arguments
```bash
$ color-rs gradient red blue --start-position 80 --end-position 20
Error: Start position must be less than end position
```

### Invalid Bezier Values
```bash
$ color-rs gradient red blue --ease-in 1.5
Error: Ease-in value must be between 0.0 and 1.0
```

### File Writing Issues
```bash
$ color-rs gradient red blue --svg --svg-name "/readonly/path.svg"
Error: Unable to write SVG file: Permission denied
```

---

## Performance Notes

- **Color Collection Loading**: Initial load of RAL collections (~2000 colors) may take 100-200ms
- **LAB Conversions**: Perceptually accurate but computationally intensive
- **Image Generation**: PNG rendering for large images (>2000px) may take 1-2 seconds
- **Gradient Calculations**: Optimized for typical gradients (5-20 stops), scales linearly

---

## Exit Codes

- `0` - Success
- `1` - General error (invalid arguments, parsing failures)
- `2` - File I/O error (unable to write output files)
- `64` - Usage error (invalid command-line usage)

---

## Compatibility

- **Rust Version**: Requires Rust 2024 edition (1.75+)
- **Operating Systems**: Windows, macOS, Linux
- **Terminal**: Supports ANSI color codes for enhanced output
- **Image Formats**: SVG (all browsers), PNG (universal support)
