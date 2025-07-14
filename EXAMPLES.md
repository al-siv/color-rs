# Color-rs Usage Examples

## Basic Usage

```bash
# Simple red to blue gradient
color-rs gradient --start-color FF0000 --end-color 0000FF

# Gradient with custom positions
color-rs gradient --start-color FF6B35 --end-color 7209B7 \
  --start-position 20 --end-position 80
```

## Advanced Features

```bash
# Intelligent gradient stops
color-rs gradient --start-color FF0000 --end-color 0000FF \
  --grad-stops 8 --ease-in 0.9 --ease-out 0.1

# Generate image files
color-rs gradient --start-color FF6B35 --end-color 7209B7 \
  --svg --png --width 1600 --no-legend
```

## CSS Integration

```bash
# Generate CSS-ready values
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

## Web Development Workflow

1. Design your gradient with color-rs
2. Copy the RGB values directly into your CSS
3. Use the SVG files for design mockups
4. Export PNG files for presentations

Perfect for modern web development!
