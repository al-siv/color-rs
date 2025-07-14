# color-rs

A CLI utility for working with color gradients in LAB color space.

## Description

`color-rs` is a command-line tool that allows you to create smooth color gradients between two colors, using the LAB color space for more accurate and visually pleasing transitions.

### Features

- **LAB Color Space**: All calculations are performed in the perceptually uniform LAB color space
- **Smooth Gradients**: Uses spline-like interpolation for natural transitions
- **Flexible Settings**: Configurable smoothing and curve tension coefficients
- **Precise Positioning**: Specify start and end positions as percentages

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
color-rs gradient --start-color #FF0000 --start-position 20 --end-color #0000FF --end-position 80
```

### Parameters

- `--start-color <HEX>` - Starting color in HEX format (e.g., #FF0000 or FF0000)
- `--start-position <PERCENT>` - Starting position as percentage (e.g., 20 or 20%)
- `--end-color <HEX>` - Ending color in HEX format (e.g., #0000FF or 0000FF)  
- `--end-position <PERCENT>` - Ending position as percentage (e.g., 80 or 80%)
- `--smoothing <FLOAT>` - Smoothing coefficient (default: 2.0)
- `--tension <FLOAT>` - Curve tension coefficient (default: 0.5)
- `--img` - Generate SVG image of the gradient
- `--width <PIXELS>` - Width of the SVG image in pixels (default: 1000)
- `--img-name <FILENAME>` - Output filename for SVG image (default: gradient.svg)

### Examples

#### Basic Red to Blue Gradient

```bash
color-rs gradient --start-color #FF0000 --start-position 0 --end-color #0000FF --end-position 100
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
color-rs gradient \
  --start-color #FF0000 \
  --start-position 20 \
  --end-color #00FF00 \
  --end-position 80 \
  --img
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

The program uses a combined smoothing algorithm:

1. **Smoothstep**: Basic S-shaped curve
2. **Power Function**: Controls transition steepness via the `smoothing` parameter
3. **Tension**: Additional modulation via the `tension` parameter

This creates a gradient that starts slowly, accelerates in the middle, and slows down at the end.

## License

MIT License

## Author

al-siv
