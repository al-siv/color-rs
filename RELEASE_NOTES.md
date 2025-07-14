# Color-rs v0.7.2 - First Full Release 🎨

Welcome to the first stable release of **color-rs**! This is a professional CLI tool for color gradient calculations using perceptually uniform LAB color space with CSS cubic-bezier easing functions.

## 🎯 What's New in v0.7.2

### ✨ Professional Interface
- **Cargo-style Output**: Terminal formatting that matches Rust toolchain aesthetics
- **Right-aligned Tables**: Professional numeric alignment in gradient tables
- **Beautiful ASCII Tables**: Clean, bordered table output with consistent styling

### 🎨 Enhanced Color Features
- **Integer Percentages**: CSS-compatible percentage values (33%, 50%, 67%)
- **Solid PNG Backgrounds**: Fixed PNG rendering with proper solid backgrounds
- **LAB Color Space**: Perceptually uniform color transitions
- **Rich Color Information**: RGB, HSL, and LAB values for all colors

### 🔧 Advanced Functionality
- **Intelligent Stop Placement**: Derivative-based gradient stop calculation
- **CSS Cubic-Bezier**: Professional timing functions matching web standards
- **Multiple Output Formats**: Step-based, intelligent, and equal spacing options
- **SVG & PNG Export**: High-quality image generation with optional legends

### 🚀 Easy Installation
- **Windows Executable**: Pre-compiled `.exe` file for immediate use
- **No Dependencies**: Windows users can run directly without installing Rust
- **Cross-platform**: Source code available for all platforms

## 📦 Downloads

### Windows Users (Recommended)
Download `color-rs.exe` from the assets below - no installation required!

### All Platforms
Clone the repository and build from source with Rust/Cargo.

## 🔥 Quick Start

```bash
# Basic gradient
color-rs gradient --start-color FF0000 --end-color 0000FF

# Advanced gradient with custom easing
color-rs gradient --start-color FF6B35 --end-color 7209B7 \
  --ease-in 0.25 --ease-out 0.75 --grad-stops 8

# Export as images
color-rs gradient --start-color FF0000 --end-color 0000FF \
  --svg --png --width 1600
```

## 🎨 Example Output

```
 Application: Color-rs v0.7.2
       About: A CLI tool for color gradient calculations using LAB color space with cubic-bezier easing functions
      Author: https://github.com/al-siv

GRADIENT VALUES:
╭──────────┬─────────┬──────────────────╮
│ Position │ Hex     │ RGB              │
├──────────┼─────────┼──────────────────┤
│       0% │ #FF0000 │ rgb(255, 0, 0)   │
│      35% │ #E2005C │ rgb(226, 0, 92)  │
│      65% │ #A700B6 │ rgb(167, 0, 182) │
│     100% │ #0000FF │ rgb(0, 0, 255)   │
╰──────────┴─────────┴──────────────────╯
```

## 🛠️ Technical Highlights

- **LAB Color Space**: Perceptually uniform color interpolation
- **Kurbo Library**: Industry-standard 2D curve mathematics
- **Binary Search**: Accurate cubic-bezier curve solving
- **Professional Tables**: Tabled crate for beautiful terminal output
- **Comprehensive Error Handling**: Robust input validation and error messages

---

**Perfect for web developers, designers, and anyone working with color gradients!** 🌈

Ready to create beautiful, mathematically precise color gradients? Download and start using color-rs today!
