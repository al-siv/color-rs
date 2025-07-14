# color-rs 🎨

A professional CLI tool for color gradient calculations using perceptually uniform LAB color space with CSS cubic-bezier easing functions.

## ✨ Features

- **Perceptually Uniform Gradients**: Uses LAB color space for visually smooth color transitions
- **CSS Cubic-Bezier Timing**: Professional easing functions matching web standards
- **Intelligent Stop Placement**: Automatically places gradient stops where colors change most rapidly
- **Multiple Output Formats**: 
  - Beautiful terminal tables with color information
  - SVG gradients with optional legends
  - High-quality PNG exports
- **Proportional Design**: All dimensions scale with width (1:5 aspect ratio)
- **Integer Percentages**: CSS-friendly percentage values for practical use
- **Rich Color Information**: RGB, HSL, and LAB values for both start and end colors

## 🚀 Installation

### From Source

```bash
git clone https://github.com/al-siv/color-rs.git
cd color-rs
cargo build --release
```

The binary will be available at `target/release/color-rs`.

## 📖 Usage

### Basic Gradient

```bash
color-rs gradient --start-color FF0000 --end-color 0000FF
```

### Custom Easing

```bash
color-rs gradient --start-color FF6B35 --end-color 7209B7 \
    --ease-in 0.25 --ease-out 0.75
```

### Generate Images

```bash
# SVG with legend
color-rs gradient --start-color FF0000 --end-color 0000FF \
    --svg --svg-name my-gradient.svg

# PNG without legend
color-rs gradient --start-color FF0000 --end-color 0000FF \
    --png --no-legend --png-name clean-gradient.png

# Both formats with custom size
color-rs gradient --start-color FF0000 --end-color 0000FF \
    --svg --png --width 1600
```

### Intelligent Stop Placement

```bash
# 8 intelligently placed stops
color-rs gradient --start-color FF0000 --end-color 0000FF \
    --grad-stops 8 --ease-in 0.9 --ease-out 0.1

# 10 equally spaced stops
color-rs gradient --start-color FF0000 --end-color 0000FF \
    --grad-stops-simple 10
```

### Partial Gradients

```bash
color-rs gradient --start-color FF0000 --end-color 0000FF \
    --start-position 20 --end-position 80
```

## 📊 Output Examples

### Color Information Table
```
Color Information:
╭─────────────┬─────────┬────────────────┬────────────────────────────┬─────────────────────────╮
│ Color       │ Hex     │ RGB            │ HSL                        │ Lab                     │
├─────────────┼─────────┼────────────────┼────────────────────────────┼─────────────────────────┤
│ Start Color │ #FF0000 │ RGB(255, 0, 0) │ HSL(0.0°, 100.0%, 50.0%)   │ Lab(53.2, 80.1, 67.2)   │
│ End Color   │ #0000FF │ RGB(0, 0, 255) │ HSL(240.0°, 100.0%, 50.0%) │ Lab(32.3, 79.2, -107.9) │
╰─────────────┴─────────┴────────────────┴────────────────────────────┴─────────────────────────╯
```

### Gradient Values Table
```
Gradient Values:
╭──────────┬─────────┬──────────────────╮
│ Position │ Hex     │ RGB              │
├──────────┼─────────┼──────────────────┤
│ 0%       │ #FF0000 │ rgb(255, 0, 0)   │
│ 24%      │ #F0003D │ rgb(240, 0, 61)  │
│ 35%      │ #E2005C │ rgb(226, 0, 92)  │
│ 45%      │ #D30079 │ rgb(211, 0, 121) │
│ 55%      │ #BF0098 │ rgb(191, 0, 152) │
│ 65%      │ #A700B6 │ rgb(167, 0, 182) │
│ 76%      │ #8400D5 │ rgb(132, 0, 213) │
│ 100%     │ #0000FF │ rgb(0, 0, 255)   │
╰──────────┴─────────┴──────────────────╯
```

## 🎛️ Command Line Options

```
color-rs gradient [OPTIONS] --start-color <HEX> --end-color <HEX>

OPTIONS:
    --start-color <HEX>              Starting color (e.g., #FF0000 or FF0000)
    --end-color <HEX>                Ending color (e.g., #0000FF or 0000FF)
    --start-position <PERCENT>       Starting position [default: 0]
    --end-position <PERCENT>         Ending position [default: 100]
    --ease-in <EASE_IN>              Ease-in control point [default: 0.65]
    --ease-out <EASE_OUT>            Ease-out control point [default: 0.35]
    --svg                            Generate SVG image
    --png                            Generate PNG image
    --no-legend                      Disable legend (only with --svg or --png)
    --width <WIDTH>                  Image width in pixels [default: 1000]
    --svg-name <SVG_NAME>            SVG filename [default: gradient.svg]
    --png-name <PNG_NAME>            PNG filename [default: gradient.png]
    --grad-step <GRAD_STEP>          Output every X percent [default: 5]
    --grad-stops <GRAD_STOPS>        Number of intelligent stops
    --grad-stops-simple <GRAD_STOPS> Number of equal stops
```

## 🎨 Color Spaces

### LAB Color Space
- **Perceptually uniform**: Equal numerical differences appear as equal visual differences
- **Device independent**: Consistent across different displays and printers
- **Wide gamut**: Encompasses all colors visible to the human eye

### RGB → LAB → RGB Pipeline
1. Input colors parsed as sRGB hex values
2. Converted to LAB for perceptually uniform interpolation
3. Converted back to sRGB for output

## ⚙️ Cubic-Bezier Easing

The tool uses industry-standard cubic-bezier curves matching CSS timing functions:

- `cubic-bezier(ease-in, 0, ease-out, 1)`
- **Linear**: `--ease-in 0 --ease-out 1`
- **Ease**: `--ease-in 0.25 --ease-out 1` (default-ish)
- **Ease-in**: `--ease-in 0.42 --ease-out 1`
- **Ease-out**: `--ease-in 0 --ease-out 0.58`
- **Ease-in-out**: `--ease-in 0.42 --ease-out 0.58`

## 🧠 Intelligent Stop Placement

The `--grad-stops` option uses curve derivatives to automatically place gradient stops where colors change most rapidly:

- Analyzes the cubic-bezier curve's rate of change
- Places more stops in areas of rapid color transition
- Results in smoother gradients with fewer visible bands
- Always uses integer percentages for CSS compatibility

## 🖼️ Image Generation

### SVG Features
- Scalable vector format
- Optional typography-rich legends
- Professional font stacks
- Text automatically converted to paths for PNG export

### PNG Features
- High-quality rasterization via resvg
- System font loading for text rendering
- Consistent output across platforms
- Optional legend control

### Proportional Design
- Gradient height = width × 0.2 (1:5 aspect ratio)
- Legend height = gradient height × 0.2 (when enabled)
- Font size = legend height × 0.6
- All dimensions scale proportionally

## 🔧 Technical Details

### Dependencies
- **kurbo**: Industry-standard 2D curve operations
- **palette**: Professional color space conversions
- **usvg/resvg**: SVG parsing and PNG rendering
- **clap**: Modern CLI argument parsing
- **tabled**: Beautiful terminal table formatting

### Performance
- Optimized curve calculations with binary search
- High-resolution sampling (10,000 points) for intelligent stops
- Efficient LAB color space interpolation
- Minimal memory allocation

## 📝 Examples

### Web Development
```bash
# Generate CSS-ready gradient
color-rs gradient --start-color "FF6B35" --end-color "7209B7" \
    --grad-stops 5 --ease-in 0.25 --ease-out 0.75
```

Output for CSS:
```css
background: linear-gradient(
    to right,
    rgb(255, 107, 53) 0%,
    rgb(226, 78, 99) 35%,
    rgb(189, 53, 132) 55%,
    rgb(151, 28, 161) 75%,
    rgb(114, 9, 183) 100%
);
```

### Design Assets
```bash
# High-resolution design asset
color-rs gradient --start-color "FF6B35" --end-color "7209B7" \
    --svg --png --width 3000 --no-legend
```

### Color Analysis
```bash
# Analyze color relationships
color-rs gradient --start-color "FF6B35" --end-color "7209B7" \
    --grad-step 10
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- **kurbo**: Rust graphics ecosystem for curve mathematics
- **palette**: Comprehensive color science library
- **usvg/resvg**: SVG processing pipeline
- **LAB color space**: Perceptually uniform color representation

## 🔗 Links

- [Repository](https://github.com/al-siv/color-rs)
- [Issues](https://github.com/al-siv/color-rs/issues)
- [CSS cubic-bezier reference](https://developer.mozilla.org/en-US/docs/Web/CSS/easing-function)
- [LAB color space](https://en.wikipedia.org/wiki/CIELAB_color_space)

---

**color-rs** - Professional color gradients for modern workflows 🎨✨

- **LAB Color Space**: All calculations are performed in the perceptually uniform LAB color space
- **CSS Cubic-Bezier**: Professional timing functions matching CSS cubic-bezier specifications
- **Kurbo Library**: Uses the industry-standard kurbo 2D graphics library for robust curve calculations
- **SVG Output**: Generate SVG images with native cubic-bezier animation support
- **Precise Positioning**: Specify start and end positions as percentages
- **Mathematical Accuracy**: Binary search algorithm for accurate bezier curve solving

## Installation

### From Source Code

```bash
git clone https://github.com/al-siv/color-rs.git
cd color-rs
cargo build --release
```

The executable will be located at `target/release/color-rs`.

### Requirements

- Rust 1.70+
- Cargo

## Usage

### Main Command - Gradient

```bash
color-rs gradient --start-color #FF0000 --end-color #0000FF
```

With custom positions and cubic-bezier timing:
```bash
color-rs gradient --start-color #FF0000 --start-position 20 --end-color #0000FF --end-position 80 --ease-in 0.25 --ease-out 0.75
```

Generate SVG output:
```bash
color-rs gradient --start-color #FF0000 --end-color #0000FF --img --img-name "gradient.svg"
```

### Parameters

- `--start-color <HEX>` - Starting color in HEX format (e.g., #FF0000 or FF0000)
- `--start-position <PERCENT>` - Starting position as percentage (e.g., 20 or 20%, default: 0%)
- `--end-color <HEX>` - Ending color in HEX format (e.g., #0000FF or 0000FF)
- `--end-position <PERCENT>` - Ending position as percentage (e.g., 80 or 80%, default: 100%)
- `--ease-in <FLOAT>` - Ease-in control point for cubic-bezier (0.0-1.0, default: 0.42)
- `--ease-out <FLOAT>` - Ease-out control point for cubic-bezier (0.0-1.0, default: 0.58)
- `--img` - Generate SVG image
- `--img-name <FILENAME>` - SVG output filename (default: gradient.svg)
- `--width <PIXELS>` - SVG width in pixels (default: 1000)  
- `--end-position <PERCENT>` - Ending position as percentage (e.g., 80 or 80%, default: 100%)
- `--smoothing <FLOAT>` - Smoothing coefficient (default: 2.0)
- `--tension <FLOAT>` - Curve tension coefficient (default: 0.5)
- `--img` - Generate SVG image of the gradient
- `--width <PIXELS>` - Width of the SVG image in pixels (default: 1000)
- `--img-name <FILENAME>` - Output filename for SVG image (default: gradient.svg)

### Examples

#### Basic Red to Blue Gradient

```bash
color-rs gradient --start-color #FF0000 --end-color #0000FF
```

#### Gradient with Custom Positions

```bash
color-rs gradient --start-color #FF0000 --start-position 20 --end-color #00FF00 --end-position 80
```

#### Gradient with Custom Smoothing Settings

```bash
color-rs gradient \
  --start-color #FF0000 \
  --start-position 10 \
  --end-color #0000FF \
  --end-position 90 \
  --smoothing 3.0 \
  --tension 0.8
```

#### Generate SVG Image

```bash
color-rs gradient --start-color #FF0000 --end-color #00FF00 --img
```

#### Generate Custom SVG Image

```bash
color-rs gradient \
  --start-color ff0000 \
  --start-position 10% \
  --end-color 0000ff \
  --end-position 90% \
  --img \
  --width 800 \
  --img-name my_gradient.svg
```

### Output Format

#### Console Output

The program outputs a list of percentages with corresponding HEX colors:

```
20%: #FF0000
21%: #FE0605
22%: #FD0C0A
...
80%: #0000FF
```

#### SVG Output

When the `--img` flag is used, the program generates an SVG file containing:
- Left section: filled with the starting color (from 0% to start position)
- Middle section: smooth gradient (from start position to end position)  
- Right section: filled with the ending color (from end position to 100%)

The SVG file can be opened in web browsers or vector graphics applications.

## How It Works

1. **LAB Conversion**: Input HEX colors are converted to LAB color space
2. **Smooth Interpolation**: Uses a combination of smoothstep and power functions to create natural transition curves
3. **LAB Interpolation**: Colors are interpolated component-wise in LAB space (L*, a*, b*)
4. **HEX Conversion**: Resulting LAB colors are converted back to HEX

## LAB Color Space

LAB is a perceptually uniform color space where:
- **L*** - Lightness (0-100)
- **a*** - Position between green and red (-128 to +127)
- **b*** - Position between blue and yellow (-128 to +127)

Advantages of LAB for gradients:
- More uniform visual transitions
- No unwanted color artifacts
- Better correspondence to human color perception

## Smoothing Algorithm

## Technical Details

- Uses LAB color space for perceptually uniform color interpolation
- Implements CSS cubic-bezier curve mathematics using the kurbo library
- SVG output includes native CSS cubic-bezier animation support
- Binary search algorithm for accurate bezier curve parameter solving
- Built with Rust for performance and safety

### Libraries Used

- **kurbo**: Industry-standard 2D graphics library for robust curve calculations
- **palette**: Color space conversions and manipulations
- **clap**: Command-line argument parsing

## License

MIT License

## Author

al-siv
