# THEORY.md

## Mathematical Foundations and Theoretical Considerations

This document provides comprehensive mathematical foundations, theoretical considerations, pure algorithms, and logical mathematical concepts underlying the color-rs library.

> Integration Note (Unreleased): Former standalone `MATH_FORMULAS.md` merged into this file to centralize mathematics and eliminate duplication (original formulas were versioned at v0.19.0).

## Table of Contents

1. [Color Space Mathematics](#color-space-mathematics)
2. [Distance Metrics Theory](#distance-metrics-theory)
3. [Color Harmony Mathematical Foundations](#color-harmony-mathematical-foundations)
4. [Gradient Mathematics](#gradient-mathematics)
5. [Perceptual Color Theory](#perceptual-color-theory)
6. [Accessibility Mathematics](#accessibility-mathematics)
7. [Numerical Analysis and Precision](#numerical-analysis-and-precision)
8. [Functional Programming Mathematical Foundations](#functional-programming-mathematical-foundations)
9. [Detailed Algorithms & Formulas](#detailed-algorithms--formulas)

---

## Color Space Mathematics

### RGB Color Space

The RGB color space represents colors using three components: Red (R), Green (G), and Blue (B), each typically normalized to the range [0, 1].

**Mathematical Definition:**
```
RGB = (R, G, B) where R, G, B ∈ [0, 1]
```

**sRGB Gamma Correction:**
The sRGB color space applies gamma correction using the following piecewise function:

```
f(u) = {
    12.92 * u,                    if u ≤ 0.0031308
    1.055 * u^(1/2.4) - 0.055,   if u > 0.0031308
}
```

**Inverse sRGB Gamma Correction:**
```
f⁻¹(u) = {
    u / 12.92,                        if u ≤ 0.04045
    ((u + 0.055) / 1.055)^2.4,       if u > 0.04045
}
```

### XYZ Color Space

The CIE XYZ color space is a device-independent color space derived from human color perception.

**RGB to XYZ Transformation Matrix (sRGB D65):**
```
| X |   | 0.4124564  0.3575761  0.1804375 | | R |
| Y | = | 0.2126729  0.7151522  0.0721750 | | G |
| Z |   | 0.0193339  0.1191920  0.9503041 | | B |
```

**XYZ to RGB Transformation Matrix:**
```
| R |   |  3.2404542 -1.5371385 -0.4985314 | | X |
| G | = | -0.9692660  1.8760108  0.0415560 | | Y |
| B |   |  0.0556434 -0.2040259  1.0572252 | | Z |
```

### LAB Color Space

The CIE LAB color space (also CIELAB) is designed to be perceptually uniform.

**XYZ to LAB Conversion:**
```
L* = 116 * f(Y/Yn) - 16
a* = 500 * (f(X/Xn) - f(Y/Yn))
b* = 200 * (f(Y/Yn) - f(Z/Zn))
```

Where the function f(t) is defined as:
```
f(t) = {
    t^(1/3),                if t > δ³
    t/(3δ²) + 4/29,        if t ≤ δ³
}
```

Where δ = 6/29, and (Xn, Yn, Zn) are the reference white point coordinates.

**LAB to XYZ Conversion:**
```
X = Xn * f⁻¹((L* + 16)/116 + a*/500)
Y = Yn * f⁻¹((L* + 16)/116)
Z = Zn * f⁻¹((L* + 16)/116 - b*/200)
```

Where:
```
f⁻¹(t) = {
    t³,                     if t > δ
    3δ²(t - 4/29),         if t ≤ δ
}
```

### LCH Color Space

LCH is the cylindrical representation of LAB color space.

**LAB to LCH Conversion:**
```
L = L* (unchanged)
C = √(a*² + b*²)
H = atan2(b*, a*) * 180/π (converted to degrees)
```

**LCH to LAB Conversion:**
```
L* = L (unchanged)
a* = C * cos(H * π/180)
b* = C * sin(H * π/180)
```

---

## Distance Metrics Theory

### Euclidean Distance in LAB Space

The simplest perceptual distance metric in LAB color space:

```
ΔE = √((L₁ - L₂)² + (a₁ - a₂)² + (b₁ - b₂)²)
```

**Properties:**
- Symmetric: d(A, B) = d(B, A)
- Non-negative: d(A, B) ≥ 0
- Identity: d(A, A) = 0
- Triangle inequality: d(A, C) ≤ d(A, B) + d(B, C)

### CIE76 Delta E

The CIE76 formula is equivalent to Euclidean distance in LAB space:

```
ΔE*76 = √((ΔL*)² + (Δa*)² + (Δb*)²)
```

Where:
- ΔL* = L₁* - L₂*
- Δa* = a₁* - a₂*
- Δb* = b₁* - b₂*

### CIE2000 Delta E

The CIEDE2000 formula provides improved perceptual uniformity through complex corrections:

```
ΔE*₀₀ = √((ΔL'/kₗSₗ)² + (ΔC'/kᴄSᴄ)² + (ΔH'/kₕSₕ)² + Rₜ(ΔC'/kᴄSᴄ)(ΔH'/kₕSₕ))
```

**Where:**

**Weighting Functions:**
- kₗ = kᴄ = kₕ = 1 (standard conditions)

**Lightness Weighting:**
```
Sₗ = 1 + (0.015 * (L̄' - 50)²) / √(20 + (L̄' - 50)²)
```

**Chroma Weighting:**
```
Sᴄ = 1 + 0.045 * C̄'
```

**Hue Weighting:**
```
Sₕ = 1 + 0.015 * C̄' * T
```

**Hue Correction Factor:**
```
T = 1 - 0.17cos(h̄' - 30°) + 0.24cos(2h̄') + 0.32cos(3h̄' + 6°) - 0.20cos(4h̄' - 63°)
```

**Rotation Function:**
```
Rₜ = -2sin(2Δθ) * Rᴄ
```

Where:
```
Δθ = 30exp(-((h̄' - 275°)/25)²)
Rᴄ = 2√(C̄'⁷/(C̄'⁷ + 25⁷))
```

### LCH Distance

Distance in LCH space considering cylindrical coordinates:

```
ΔE_LCH = √((ΔL)² + (ΔC)² + (2 * C₁ * C₂ * (1 - cos(ΔH))))
```

Where ΔH is the hue difference in radians.

---

## Color Harmony Mathematical Foundations

### Complementary Colors

**Mathematical Definition:**
Two colors are complementary if their hues differ by 180°:

```
H₂ = (H₁ + 180°) mod 360°
```

**In LCH Space:**
```
Complementary(L, C, H) = (L, C, (H + 180°) mod 360°)
```

### Triadic Harmony

Three colors equally spaced around the color wheel:

```
H₁ = H₀
H₂ = (H₀ + 120°) mod 360°
H₃ = (H₀ + 240°) mod 360°
```

### Split-Complementary Harmony

Base color plus two colors adjacent to its complement:

```
H₁ = H₀
H₂ = (H₀ + 150°) mod 360°
H₃ = (H₀ + 210°) mod 360°
```

### Tetradic (Square) Harmony

Four colors equally spaced around the color wheel:

```
H₁ = H₀
H₂ = (H₀ + 90°) mod 360°
H₃ = (H₀ + 180°) mod 360°
H₄ = (H₀ + 270°) mod 360°
```

### Analogous Harmony

Colors adjacent on the color wheel, typically within 30° intervals:

```
Hᵢ = (H₀ + i * 30°) mod 360° for i ∈ [-1, 0, 1]
```

---

## Gradient Mathematics

### Linear Interpolation

**Definition:**
Linear interpolation between two colors A and B:

```
C(t) = (1 - t) * A + t * B, where t ∈ [0, 1]
```

**Component-wise in LAB space:**
```
L(t) = (1 - t) * L₁ + t * L₂
a(t) = (1 - t) * a₁ + t * a₂
b(t) = (1 - t) * b₁ + t * b₂
```

### Bézier Curve Interpolation

**Cubic Bézier curve:**
```
B(t) = (1-t)³P₀ + 3(1-t)²tP₁ + 3(1-t)t²P₂ + t³P₃
```

**For color gradients with control points:**
```
Color(t) = (1-t)³C₀ + 3(1-t)²tC₁ + 3(1-t)t²C₂ + t³C₃
```

**Quadratic Bézier (most common for easing):**
```
B(t) = (1-t)²P₀ + 2(1-t)tP₁ + t²P₂
```

### Easing Functions

**Mathematical definitions of common easing functions:**

**Linear:**
```
f(t) = t
```

**Ease-in (cubic):**
```
f(t) = t³
```

**Ease-out (cubic):**
```
f(t) = 1 - (1-t)³
```

**Ease-in-out (cubic):**
```
f(t) = {
    4t³,                    if t < 0.5
    1 - 4(1-t)³,           if t ≥ 0.5
}
```

**Custom Bézier easing:**
```
f(t) = 3(1-t)²t*P₁ + 3(1-t)t²*P₂ + t³
```

Where P₁ and P₂ are control points defining the curve shape.

### Perceptually Uniform Gradients

To create perceptually uniform gradients, interpolation should occur in LAB space with equal ΔE steps:

```
Given: Start color A, End color B, Number of steps n
Total distance: D = ΔE(A, B)
Step size: s = D / (n - 1)
Target distances: dᵢ = i * s for i ∈ [0, n-1]
```

**Iterative solution for intermediate colors:**
For each target distance dᵢ, find parameter t such that:
```
ΔE(A, Interpolate(A, B, t)) = dᵢ
```

This typically requires numerical methods (e.g., binary search) since there's no closed-form solution.

---

## Perceptual Color Theory

### Luminance and Brightness

**WCAG Relative Luminance:**
```
L = 0.2126 * R + 0.7152 * G + 0.0722 * B
```

Where R, G, B are the linearized sRGB values (gamma correction removed).

**Perceived Brightness (various models):**

**HSP Model:**
```
Brightness = √(0.299*R² + 0.587*G² + 0.114*B²)
```

**Luma (BT.709):**
```
Y' = 0.2126*R' + 0.7152*G' + 0.0722*B'
```

Where R', G', B' are gamma-corrected values.

### Contrast Ratios

**WCAG Contrast Ratio:**
```
CR = (L₁ + 0.05) / (L₂ + 0.05)
```

Where L₁ is the relative luminance of the lighter color and L₂ is the relative luminance of the darker color.

**Michelson Contrast:**
```
CM = (Lmax - Lmin) / (Lmax + Lmin)
```

**Weber Contrast:**
```
CW = (L - Lb) / Lb
```

Where L is the luminance of the target and Lb is the background luminance.

### Color Appearance Models

**CIECAM02 Lightness:**
```
J = 100 * (A/Aw)^(c*z)
```

**CIECAM02 Chroma:**
```
C = t^0.9 * √(J/100) * (1.64 - 0.29^n)^0.73
```

**CIECAM02 Hue:**
```
H = h + (360/23) * (2 + p₁)
```

Where these formulas involve complex environmental viewing condition parameters.

---

## Accessibility Mathematics

### WCAG Guidelines Mathematical Basis

**Level AA Requirements:**
- Normal text: CR ≥ 4.5:1
- Large text: CR ≥ 3:1

**Level AAA Requirements:**
- Normal text: CR ≥ 7:1
- Large text: CR ≥ 4.5:1

**Large text definition:**
- 18pt or larger
- 14pt bold or larger
- Equivalent CSS: 24px normal, 18.67px bold

### Color Vision Deficiency Simulation

**Protanopia Transformation Matrix:**
```
| R' |   | 0.567  0.433  0.000 | | R |
| G' | = | 0.558  0.442  0.000 | | G |
| B' |   | 0.000  0.242  0.758 | | B |
```

**Deuteranopia Transformation Matrix:**
```
| R' |   | 0.625  0.375  0.000 | | R |
| G' | = | 0.700  0.300  0.000 | | G |
| B' |   | 0.000  0.300  0.700 | | B |
```

**Tritanopia Transformation Matrix:**
```
| R' |   | 0.950  0.050  0.000 | | R |
| G' | = | 0.000  0.433  0.567 | | G |
| B' |   | 0.000  0.475  0.525 | | B |
```

---

## Numerical Analysis and Precision

### Floating Point Considerations

**IEEE 754 Single Precision (f32):**
- Precision: ~7 decimal digits
- Range: ±3.4 × 10³⁸
- Machine epsilon: 2⁻²³ ≈ 1.19 × 10⁻⁷

**IEEE 754 Double Precision (f64):**
- Precision: ~15-16 decimal digits
- Range: ±1.8 × 10³⁰⁸
- Machine epsilon: 2⁻⁵² ≈ 2.22 × 10⁻¹⁶

### Color Space Precision Requirements

**RGB Values:**
- Typical range: [0, 1] for normalized values
- 8-bit: [0, 255] integer values
- Precision requirement: ±1/255 ≈ 0.004

**LAB Values:**
- L*: [0, 100]
- a*, b*: typically [-100, 100], can extend beyond
- Precision requirement: ±0.01 for visual distinction

**Delta E Precision:**
- Just noticeable difference: ΔE ≈ 1.0
- Commercial accuracy: ΔE < 2.0
- Precision requirement: ±0.1

### Numerical Stability

**Avoiding Division by Zero:**
```
safe_divide(a, b) = {
    a / b,          if |b| > ε
    sign(a) * ∞,    if |b| ≤ ε and a ≠ 0
    NaN,            if |b| ≤ ε and a = 0
}
```

**Clamping Functions:**
```
clamp(x, min, max) = {
    min,    if x < min
    max,    if x > max
    x,      otherwise
}
```

**Safe Square Root:**
```
safe_sqrt(x) = {
    √x,     if x ≥ 0
    0,      if x < 0 (for color calculations)
}
```

### Interpolation Accuracy

**Linear interpolation error bounds:**
For f(x) = (1-t)f(a) + tf(b), the error is bounded by:
```
|error| ≤ (1/8) * max|f''(x)| * (b-a)²
```

**Bézier curve approximation error:**
For cubic Bézier approximation of a function, the error is O(h⁴) where h is the step size.

---

## Functional Programming Mathematical Foundations

### Category Theory Concepts

**Functor Laws:**
```
fmap(id) = id
fmap(f ∘ g) = fmap(f) ∘ fmap(g)
```

**Applied to Color transformations:**
```
map_color(identity, color) = color
map_color(f ∘ g, color) = map_color(f, map_color(g, color))
```

**Monad Laws:**
```
return(a) >>= f = f(a)
m >>= return = m
(m >>= f) >>= g = m >>= (λx → f(x) >>= g)
```

**Applied to Color Result types:**
```
Ok(color) >>= transform = transform(color)
Err(e) >>= transform = Err(e)
```

### Algebraic Data Types

**Sum Types (Enums):**
```
ColorSpace = RGB | HSL | LAB | LCH | XYZ
```

**Product Types (Structs):**
```
Color = (ColorSpace, Values, Metadata)
```

**Recursive Types:**
```
ColorTree = Leaf(Color) | Branch(ColorTree, ColorTree)
```

### Pure Function Properties

**Referential Transparency:**
For any pure function f and expression e:
```
f(e) can be replaced by f(value_of(e))
```

**Memoization Validity:**
A function f is memoizable if:
```
∀x: f(x) always produces the same output
```

**Composition Associativity:**
```
(f ∘ g) ∘ h = f ∘ (g ∘ h)
```

### Type Safety Invariants

**Color Space Invariants:**
```
RGB: ∀c ∈ RGB, c.r, c.g, c.b ∈ [0, 1]
LAB: ∀c ∈ LAB, c.l ∈ [0, 100], c.a, c.b ∈ ℝ
```

**Distance Metric Invariants:**
```
∀d: ColorDistance, ∀c₁, c₂: Color
- d(c₁, c₂) ≥ 0
- d(c₁, c₁) = 0
- d(c₁, c₂) = d(c₂, c₁)
```

**Gradient Invariants:**
```
∀g: Gradient, ∀t ∈ [0, 1]
- g(0) = start_color
- g(1) = end_color
- g is continuous
```

---

## Implementation Notes

### Precision Requirements

1. **Color Component Storage**: Use f32 for color components in most cases, f64 for high-precision calculations
2. **Distance Calculations**: Use f64 for Delta E calculations to maintain precision
3. **Matrix Operations**: Use f64 for color space transformation matrices
4. **Gradient Parameters**: Use f64 for gradient interpolation parameters

### Performance Considerations

1. **SIMD Opportunities**: Color component operations can often be vectorized
2. **Lookup Tables**: Pre-computed tables for gamma correction and other nonlinear functions
3. **Approximations**: Fast approximations for less critical calculations (e.g., display purposes)
4. **Cache Efficiency**: Structure data to maximize cache locality

### Numerical Robustness

1. **Input Validation**: Always validate color space ranges
2. **Graceful Degradation**: Handle edge cases and numerical instabilities
3. **Error Propagation**: Use Result types for fallible operations
4. **Precision Loss Warning**: Document where precision loss might occur

---

## References

1. **CIE Standards**: CIE 15:2004, CIE 142:2001
2. **Color Difference Formulas**: CIEDE2000, CIE76, CMC, CIE94
3. **Color Appearance Models**: CIECAM02, Hunt, Nayatani
4. **Web Standards**: W3C CSS Color Module, WCAG 2.1
5. **Functional Programming**: Category Theory for Programmers, Haskell Literature
6. **Numerical Analysis**: Numerical Recipes, IEEE 754 Standard

This document serves as the theoretical foundation for understanding the mathematical principles underlying all color operations in the color-rs library.

---

## Detailed Algorithms & Formulas

Consolidated procedural algorithms and stepwise formula listings formerly residing in `MATH_FORMULAS.md`. Overlapping derivations already covered above are omitted to avoid redundancy.

### Color Space Conversions (Procedural)

#### RGB → HSL
1. Normalize R,G,B ∈ [0,255] to R',G',B' ∈ [0,1].
2. Cmax = max(R',G',B'); Cmin = min(R',G',B'); Δ = Cmax - Cmin.
3. Lightness: L = (Cmax + Cmin)/2.
4. Saturation:
    - If Δ = 0 ⇒ S = 0
    - Else if L > 0.5 ⇒ S = Δ / (2 - Cmax - Cmin)
    - Else ⇒ S = Δ / (Cmax + Cmin)
5. Hue (if Δ = 0 undefined):
    - Cmax = R' ⇒ H = 60° * ((G' - B')/Δ mod 6)
    - Cmax = G' ⇒ H = 60° * ((B' - R')/Δ + 2)
    - Cmax = B' ⇒ H = 60° * ((R' - G')/Δ + 4)
6. Normalize H into [0,360).

#### RGB → XYZ (sRGB D65)
Per component gamma linearization (u in [0,1]): u_lin = u/12.92 (u ≤ 0.04045) else ((u+0.055)/1.055)^{2.4}. Multiply by the standard 3×3 matrix (see Color Space Mathematics) to obtain X,Y,Z.

#### XYZ → LAB (D65)
Normalize X/Xn, Y/Yn, Z/Zn; apply f(t) piecewise (earlier definition); compute L*, a*, b*.

#### LAB ↔ LCH
Forward: C = √(a*² + b*²); H = atan2(b*,a*) * 180/π (add 360 if negative). Inverse: a* = C cos(H π/180); b* = C sin(H π/180).

### Color Distance (Operational Summary)

| Metric | Formula (summary) | Note |
|--------|-------------------|------|
| CIE76 | √((ΔL*)² + (Δa*)² + (Δb*)²) | Euclidean LAB baseline |
| CIE94 | √((ΔL*/kL SL)² + (ΔC*/kC SC)² + (ΔH*/kH SH)²) | Graphic arts k=1 |
| CIEDE2000 | Weighted terms + rotation Rₜ | Most perceptual accuracy |
| LCH Approx | √((ΔL)² + (ΔC)² + 2 C1 C2 (1 - cos ΔH)) | Cylindrical approximation |

ΔH* (CIE94) = √(Δa*² + Δb*² - ΔC*²).

### Hue Operations

Shortest hue distance d(h1,h2) = min(|Δ|, 360 - |Δ|) with Δ = h1 - h2. Range with wrap (min > max) ⇒ h ≥ min OR h ≤ max. Normalize: ((h mod 360)+360) mod 360.

### Interpolation Recipes

LAB: per-component linear interpolation. LCH hue: select shortest angular path (adjust ΔH by ±360). Linear RGB interpolation is non‑perceptual but may be used for speed-critical paths; prefer LAB for perceptual uniformity.

### Contrast & Accessibility Quick Reference

Relative luminance (linear): Y = 0.2126 R + 0.7152 G + 0.0722 B. Contrast ratio: (L1 + 0.05)/(L2 + 0.05). WCAG: 4.5:1 (AA normal), 3:1 (AA large), 7:1 (AAA normal), 4.5:1 (AAA large).

### Gradient Generation (Cubic Bézier)

Bezier easing B(t) = (1-t)^3 P0 + 3(1-t)^2 t P1 + 3(1-t) t^2 P2 + t^3 P3 with P0=(0,0), P3=(1,1), P1=(ease_in,0), P2=(1-ease_out,1). Position(i) = start + span * B(i/steps). Color(t) via perceptual interpolation (LAB preferred).

### Numerical & Performance Implementation Notes (Additions)

1. Suggested hue epsilon ε_h = 1e-6 for equality checks.
2. Precompute 256-entry gamma table for bulk sRGB→linear conversions.
3. Early terminate ΔE accumulation loops when partial sum > threshold ΔE_max.

### Error Handling Summary

Use Result for fallible conversions; propagate domain errors; clamp output gamut for display.

---
End of merged content from former `MATH_FORMULAS.md` (file removed).
