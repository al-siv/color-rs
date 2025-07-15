# Color-rs Usage Examples

## Basic Usage - Simplified Interface

```bash
# Simple red to blue gradient - no more flags needed!
color-rs gradient red blue

# Using HEX colors directly
color-rs gradient FF0000 0000FF

# Mixed color formats
color-rs gradient "#FF6B35" "hsl(270, 100%, 50%)"

# Gradient with custom positions
color-rs gradient FF6B35 7209B7 --start-position 20 --end-position 80
```

## Advanced Features

```bash
# Intelligent gradient stops with custom easing
color-rs gradient FF0000 0000FF --grad-stops 8 --ease-in 0.9 --ease-out 0.1

# Generate image files with simplified syntax
color-rs gradient FF6B35 7209B7 --svg --png --width 1600 --no-legend

# Perfect for design workflows
color-rs gradient "coral" "rebeccapurple" --svg --width 2000
```

## Color Analysis with WCAG Compliance

```bash
# Get comprehensive color analysis including accessibility data
color-rs color-match "#FF5733"

# Check any color format
color-rs color-match "rgb(255, 87, 51)"
color-rs color-match "tomato"
color-rs color-match "hsl(11, 100%, 60%)"
```

Example output shows WCAG compliance data:
```
• WCAG Relative Luminance: 0.283
• Contrast vs White: 3.15:1  (Doesn't meet WCAG AA 4.5:1 for normal text)
• Contrast vs Black: 6.66:1  (Meets WCAG AA requirements)
```

## CSS Integration

```bash
# Generate CSS-ready values with the new simple syntax
color-rs gradient "FF6B35" "7209B7" --grad-stops 5 --ease-in 0.25 --ease-out 0.75
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

1. **Design your gradient**: `color-rs gradient primary-color secondary-color`
2. **Check accessibility**: `color-rs color-match your-color` (verify contrast ratios)
3. **Export for design**: Copy RGB values directly into your CSS
4. **Create assets**: Use `--svg` and `--png` flags for design mockups
5. **Validate compliance**: Ensure colors meet WCAG AA (4.5:1) or AAA (7:1) contrast requirements

## Real-World Examples

```bash
# Brand gradient for web design
color-rs gradient "#3B82F6" "#8B5CF6" --grad-stops 3 --svg

# Accessibility-focused color palette
color-rs color-match "#1F2937"  # Check if this dark color has good contrast
color-rs color-match "#F9FAFB"  # Check if this light color works as background

# Custom easing for smooth animations
color-rs gradient "#EF4444" "#10B981" --ease-in 0.42 --ease-out 0.58

# High-resolution export for print design
color-rs gradient coral navy --png --width 4000 --no-legend
```

Perfect for modern web development with accessibility in mind!
