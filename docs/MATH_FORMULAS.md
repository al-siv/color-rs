# Mathematical Formulas and Algorithms

This document provides detailed mathematical formulas and algorithmic descriptions for the core color operations in color-rs v0.19.0.

## Table of Contents

1. [Color Space Conversions](#color-space-conversions)
2. [Color Distance Calculations](#color-distance-calculations)
3. [Hue Analysis](#hue-analysis)
4. [Color Interpolation](#color-interpolation)
5. [Contrast and Accessibility](#contrast-and-accessibility)

## Color Space Conversions

### RGB to HSL Conversion

The RGB to HSL conversion transforms colors from the Red-Green-Blue color space to Hue-Saturation-Lightness.

#### Algorithm

1. **Normalize RGB values**: Convert from [0,255] to [0,1]
   ```
   R' = R / 255
   G' = G / 255  
   B' = B / 255
   ```

2. **Find min and max values**:
   ```
   Cmax = max(R', G', B')
   Cmin = min(R', G', B')
   Δ = Cmax - Cmin
   ```

3. **Calculate Lightness**:
   ```
   L = (Cmax + Cmin) / 2
   ```

4. **Calculate Saturation**:
   ```
   S = {
     0,                    if Δ = 0
     Δ / (2 - Cmax - Cmin), if L > 0.5
     Δ / (Cmax + Cmin),     if L ≤ 0.5
   }
   ```

5. **Calculate Hue**:
   ```
   H = {
     undefined,                           if Δ = 0
     60° × ((G' - B') / Δ mod 6),        if Cmax = R'
     60° × ((B' - R') / Δ + 2),          if Cmax = G'
     60° × ((R' - G') / Δ + 4),          if Cmax = B'
   }
   ```

### RGB to LAB Conversion

RGB to LAB conversion goes through XYZ color space using the D65 illuminant.

#### Step 1: RGB to XYZ

1. **Apply gamma correction**:
   ```
   R_linear = {
     R/12.92,                    if R ≤ 0.04045
     ((R + 0.055)/1.055)^2.4,    if R > 0.04045
   }
   ```
   (Same for G and B)

2. **Apply transformation matrix** (sRGB to XYZ D65):
   ```
   [X]   [0.4124564  0.3575761  0.1804375] [R_linear]
   [Y] = [0.2126729  0.7151522  0.0721750] [G_linear]
   [Z]   [0.0193339  0.1191920  0.9503041] [B_linear]
   ```

#### Step 2: XYZ to LAB

1. **Normalize using D65 white point**:
   ```
   Xn = X / 95.047
   Yn = Y / 100.000
   Zn = Z / 108.883
   ```

2. **Apply LAB transformation**:
   ```
   fx = {
     (Xn)^(1/3),              if Xn > δ³
     (κ × Xn + 16) / 116,     if Xn ≤ δ³
   }
   ```
   Where δ = 6/29, κ = (29/6)² × (1/3)

3. **Calculate LAB values**:
   ```
   L* = 116 × fy - 16
   a* = 500 × (fx - fy)
   b* = 200 × (fy - fz)
   ```

### LAB to LCH Conversion

LCH (Lightness, Chroma, Hue) provides a cylindrical representation of LAB.

```
L = L*  (unchanged)
C = √(a*² + b*²)
H = atan2(b*, a*) × (180/π)
```

If H < 0, then H = H + 360°

## Color Distance Calculations

### Delta E CIE76

The original Delta E formula using Euclidean distance in LAB space:

```
ΔE*₇₆ = √((ΔL*)² + (Δa*)² + (Δb*)²)
```

Where:
- ΔL* = L₁* - L₂*
- Δa* = a₁* - a₂*  
- Δb* = b₁* - b₂*

### Delta E CIE94

An improved formula that accounts for perceptual non-uniformities:

```
ΔE*₉₄ = √((ΔL*/kₗSₗ)² + (ΔC*/kᶜSᶜ)² + (ΔH*/kʰSʰ)²)
```

Where:
- ΔC* = C₁* - C₂*
- ΔH* = √(Δa*² + Δb*² - ΔC*²)

Weighting factors:
- kₗ = kᶜ = kʰ = 1 (for graphic arts)
- Sₗ = 1
- Sᶜ = 1 + 0.045 × C₁*
- Sʰ = 1 + 0.015 × C₁*

### Delta E 2000 (CIEDE2000)

The most perceptually accurate color difference formula:

```
ΔE₀₀ = √((ΔL'/kₗSₗ)² + (ΔC'/kᶜSᶜ)² + (ΔH'/kʰSʰ)² + Rₜ(ΔC'/kᶜSᶜ)(ΔH'/kʰSʰ))
```

This formula includes:
- L', C', H' adjustments for improved perceptual uniformity
- Rotation function Rₜ for blue-purple region correction
- Complex weighting functions Sₗ, Sᶜ, Sʰ

## Hue Analysis

### Hue Distance Calculation

For circular hue values (0-360°), the shortest angular distance:

```
hue_distance(h₁, h₂) = min(|h₁ - h₂|, 360° - |h₁ - h₂|)
```

### Hue Range Filtering

For a hue range [min, max]:

1. **Normal range** (min < max):
   ```
   contains(h) = min ≤ h ≤ max
   ```

2. **Wraparound range** (min > max):
   ```
   contains(h) = h ≥ min OR h ≤ max
   ```

### Hue Normalization

Convert any hue value to 0-360° range:

```
normalize_hue(h) = ((h mod 360) + 360) mod 360
```

## Color Interpolation

### Linear RGB Interpolation

Simple component-wise linear interpolation:

```
R(t) = R₁ + t × (R₂ - R₁)
G(t) = G₁ + t × (G₂ - G₁)  
B(t) = B₁ + t × (B₂ - B₁)
```

Where t ∈ [0,1]

### LAB Interpolation

Perceptually uniform interpolation in LAB space:

```
L*(t) = L₁* + t × (L₂* - L₁*)
a*(t) = a₁* + t × (a₂* - a₁*)
b*(t) = b₁* + t × (b₂* - b₁*)
```

### LCH Interpolation

Interpolation in cylindrical coordinates with hue handling:

```
L(t) = L₁ + t × (L₂ - L₁)
C(t) = C₁ + t × (C₂ - C₁)
H(t) = H₁ + t × hue_difference(H₁, H₂)
```

Where `hue_difference` accounts for circular interpolation:

```
hue_difference(h₁, h₂) = {
  h₂ - h₁,           if |h₂ - h₁| ≤ 180°
  h₂ - h₁ - 360°,    if h₂ - h₁ > 180°
  h₂ - h₁ + 360°,    if h₂ - h₁ < -180°
}
```

## Contrast and Accessibility

### Relative Luminance (WCAG)

Based on ITU-R BT.709 standard:

```
Y = 0.2126 × R_linear + 0.7152 × G_linear + 0.0722 × B_linear
```

Where R_linear, G_linear, B_linear are gamma-corrected values.

### Contrast Ratio (WCAG)

```
contrast_ratio = (L₁ + 0.05) / (L₂ + 0.05)
```

Where L₁ is the relative luminance of the lighter color and L₂ is the relative luminance of the darker color.

### WCAG Compliance Levels

- **AA Normal**: contrast_ratio ≥ 4.5:1
- **AA Large**: contrast_ratio ≥ 3:1
- **AAA Normal**: contrast_ratio ≥ 7:1
- **AAA Large**: contrast_ratio ≥ 4.5:1

## Gradient Generation with Cubic Bézier Easing

### Cubic Bézier Function

For control points P₀(0,0), P₁(x₁,y₁), P₂(x₂,y₂), P₃(1,1):

```
B(t) = (1-t)³P₀ + 3(1-t)²tP₁ + 3(1-t)t²P₂ + t³P₃
```

For our easing curve with ease_in and ease_out parameters:
- P₁ = (ease_in, 0)
- P₂ = (1 - ease_out, 1)

### Gradient Stop Calculation

1. **Position calculation**:
   ```
   position(i) = start_pos + (end_pos - start_pos) × B(i/steps)
   ```

2. **Color interpolation**:
   ```
   color(t) = interpolate_lab(start_color, end_color, t)
   ```

Where t is the bezier-adjusted parameter.

## Implementation Notes

### Numerical Stability

1. **Epsilon comparisons**: Use ε = 1e-10 for floating-point comparisons
2. **Domain clamping**: Ensure color values stay within valid ranges
3. **Hue wrap handling**: Always normalize hue values to [0,360°)

### Performance Optimizations

1. **Lookup tables**: Pre-computed gamma correction for RGB conversions
2. **SIMD operations**: Vectorized color distance calculations for large datasets
3. **Early termination**: Stop calculations when precision targets are met

### Error Handling

1. **Invalid color values**: Return ColorError::InvalidArguments
2. **Mathematical domain errors**: Handle division by zero, invalid roots
3. **Conversion failures**: Graceful degradation with sensible defaults

---

*This documentation corresponds to color-rs v0.19.0. For implementation details, see the source code in the respective modules.*
