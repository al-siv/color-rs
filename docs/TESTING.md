# Color-rs Testing Strategy

This document describes the testing strategy, test organization, quality gates, benchmarking, and code coverage for color-rs.

## Table of Contents

- [Testing Philosophy](#testing-philosophy)
- [Test Organization](#test-organization)
- [Unit Tests](#unit-tests)
- [Integration Tests](#integration-tests)
- [Property-Based Testing](#property-based-testing)
- [Benchmarking](#benchmarking)
- [Code Coverage](#code-coverage)
- [Continuous Integration](#continuous-integration)

## Testing Philosophy

### Quality Gates

Color-rs maintains high quality through multiple testing layers:

1. **Compilation**: Rust's type system prevents many runtime errors
2. **Unit Tests**: Fast, isolated tests for individual components
3. **Integration Tests**: End-to-end functionality verification
4. **Property Tests**: Generative testing for edge cases
5. **Benchmarks**: Performance regression detection
6. **Manual Testing**: CLI and library usage validation

### Testing Principles

- **Fast Feedback**: Unit tests run in milliseconds
- **Deterministic**: Tests produce consistent results
- **Isolated**: Tests don't depend on external resources
- **Comprehensive**: Cover both happy paths and error cases
- **Maintainable**: Tests are easy to understand and modify

## Test Organization

### Current Test Structure

```
color-rs/
├── src/
│   ├── lib.rs
│   ├── color.rs
│   │   └── #[cfg(test)] mod tests { ... }    # Unit tests
│   ├── gradient.rs
│   │   └── #[cfg(test)] mod tests { ... }    # Unit tests
│   ├── output_filter.rs                      # Output filtering system (v0.15.4)
│   │   └── #[cfg(test)] mod tests { ... }    # Filtering unit tests
│   └── ...
├── tests/                                     # Integration tests (planned)
│   ├── cli_tests.rs
│   ├── gradient_generation.rs
│   ├── color_parsing.rs
│   └── output_filtering.rs                  # Filtering integration tests (v0.14.1+)
├── benches/                                   # Benchmarks (planned)
│   ├── color_conversion.rs
│   ├── gradient_generation.rs
│   ├── collection_loading.rs
│   └── output_filtering.rs                  # Filtering performance tests (v0.14.1+)
└── examples/
    ├── library_usage.rs                      # Manual testing examples
    └── ...
```

### Test Categories

#### 1. Unit Tests (Internal)
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hex_to_rgb_conversion() {
        // Test individual function behavior
    }
}
```

#### 2. Integration Tests (External)
```rust
// tests/cli_tests.rs
use std::process::Command;

#[test]
fn test_gradient_cli_basic() {
    let output = Command::new("color-rs")
        .args(&["gradient", "red", "blue"])
        .output()
        .expect("Failed to execute color-rs");
    
    assert!(output.status.success());
}
```

#### 3. Property Tests (Generative)
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_lab_rgb_roundtrip(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
        let lab = ColorUtils::rgb_to_lab([r, g, b]);
        let rgb_back = ColorUtils::lab_to_rgb(lab);
        
        // Allow small rounding errors
        prop_assert!((rgb_back[0] as i16 - r as i16).abs() <= 1);
    }
}
```

## Unit Tests

### Test Coverage by Module

#### color.rs
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hex_color_parsing() {
        let result = ColorProcessor::parse_hex_color("#FF0000");
        assert!(result.is_ok());
        
        let lab = result.unwrap();
        // Verify red color in LAB space
        assert!((lab.l - 53.24).abs() < 0.1);
    }
    
    #[test]
    fn test_invalid_hex_format() {
        let result = ColorProcessor::parse_hex_color("#GG0000");
        assert!(result.is_err());
        
        if let Err(ColorError::ParseError(msg)) = result {
            assert!(msg.contains("Invalid HEX"));
        }
    }
    
    #[test]
    fn test_lab_to_hex_conversion() {
        let lab = Lab::new(53.24, 80.09, 67.20); // Red in LAB
        let hex = ColorProcessor::lab_to_hex(lab);
        assert_eq!(hex, "#FF0000");
    }
}
```

#### gradient.rs
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gradient_calculation() {
        let calculator = GradientCalculator::new();
        let start_lab = [53.24, 80.09, 67.20]; // Red
        let end_lab = [32.30, 79.19, -107.86]; // Blue
        
        let steps = vec![0.0, 0.5, 1.0];
        let gradient = calculator.calculate(
            start_lab, end_lab, steps, 0.0, 1.0
        );
        
        assert_eq!(gradient.len(), 3);
        assert_eq!(gradient[0].position, 0);
        assert_eq!(gradient[1].position, 50);
        assert_eq!(gradient[2].position, 100);
    }
    
    #[test]
    fn test_bezier_easing() {
        let calculator = GradientCalculator::new();
        // Test with ease-in-out curve
        let steps = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let gradient = calculator.calculate(
            [0.0, 0.0, 0.0], [100.0, 0.0, 0.0],
            steps, 0.42, 0.58
        );
        
        // Verify easing curve shape
        assert!(gradient[1].lab[0] < 25.0); // Slow start
        assert!(gradient[3].lab[0] > 75.0); // Fast finish
    }
}
```

#### color_parser/
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_css_color_parsing() {
        let parser = CssColorParser::new().unwrap();
        
        // Test named color
        let result = parser.parse("red");
        assert!(result.is_ok());
        
        let (lab, format) = result.unwrap();
        assert_eq!(format, "CSS Named");
        assert!((lab[0] - 53.24).abs() < 0.1);
    }
    
    #[test]
    fn test_rgb_function_parsing() {
        let parser = CssColorParser::new().unwrap();
        
        let result = parser.parse("rgb(255, 0, 0)");
        assert!(result.is_ok());
        
        let (lab, format) = result.unwrap();
        assert_eq!(format, "RGB");
    }
    
    #[test]
    fn test_invalid_color_format() {
        let parser = CssColorParser::new().unwrap();
        
        let result = parser.parse("invalid-color");
        assert!(result.is_err());
    }
}
```

### Helper Functions and Utilities

```rust
#[cfg(test)]
mod test_helpers {
    use super::*;
    
    pub fn assert_lab_near(actual: [f64; 3], expected: [f64; 3], tolerance: f64) {
        for i in 0..3 {
            assert!(
                (actual[i] - expected[i]).abs() < tolerance,
                "LAB component {} differs: actual={}, expected={}, tolerance={}",
                i, actual[i], expected[i], tolerance
            );
        }
    }
    
    pub fn create_test_gradient_args() -> GradientArgs {
        GradientArgs {
            start_color: "red".to_string(),
            end_color: "blue".to_string(),
            start_position: 0,
            end_position: 100,
            ease_in: 0.65,
            ease_out: 0.35,
            svg: false,
            png: false,
            no_legend: false,
            width: 1000,
            svg_name: "test.svg".to_string(),
            png_name: "test.png".to_string(),
            grad_step: 5,
            grad_stops: None,
            grad_stops_simple: None,
        }
    }
}
```

## Integration Tests

### CLI Integration Tests

```rust
// tests/cli_tests.rs
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_basic_gradient_generation() {
    let output = Command::new("color-rs")
        .args(&["gradient", "red", "blue"])
        .output()
        .expect("Failed to execute color-rs");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("RGB"));
    assert!(stdout.contains("Lab"));
}

#[test]
fn test_gradient_with_svg_output() {
    let temp_dir = TempDir::new().unwrap();
    let svg_path = temp_dir.path().join("test.svg");
    
    let output = Command::new("color-rs")
        .args(&[
            "gradient", "red", "blue",
            "--svg",
            "--svg-name", svg_path.to_str().unwrap()
        ])
        .output()
        .expect("Failed to execute color-rs");
    
    assert!(output.status.success());
    assert!(svg_path.exists());
    
    let svg_content = std::fs::read_to_string(&svg_path).unwrap();
    assert!(svg_content.contains("<svg"));
    assert!(svg_content.contains("gradient"));
}

#[test]
fn test_color_match_ral_colors() {
    let output = Command::new("color-rs")
        .args(&["color-match", "RAL 3020"])
        .output()
        .expect("Failed to execute color-rs");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Traffic Red"));
    assert!(stdout.contains("RAL"));
}

#[test]
fn test_invalid_color_input() {
    let output = Command::new("color-rs")
        .args(&["gradient", "invalid-color", "blue"])
        .output()
        .expect("Failed to execute color-rs");
    
    assert!(!output.status.success());
    
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Unable to parse color"));
}

#[test]
fn test_output_filtering_hex_only() {
    let output = Command::new("color-rs")
        .args(&["gradient", "red", "blue", "--func", "hex"])
        .output()
        .expect("Failed to execute color-rs");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("#"));
    assert!(!stdout.contains("RGB"));
    assert!(!stdout.contains("HSL"));
    assert!(!stdout.contains("Lab"));
}

#[test]
fn test_output_filtering_multiple_formats() {
    let output = Command::new("color-rs")
        .args(&["gradient", "red", "blue", "--func", "hex,rgb"])
        .output()
        .expect("Failed to execute color-rs");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("#"));
    assert!(stdout.contains("RGB"));
    assert!(!stdout.contains("HSL"));
    assert!(!stdout.contains("Lab"));
}

#[test]
fn test_invalid_filter_expression() {
    let output = Command::new("color-rs")
        .args(&["gradient", "red", "blue", "--func", "invalid"])
        .output()
        .expect("Failed to execute color-rs");
    
    assert!(!output.status.success());
    
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Invalid filter"));
}
```

### Library Integration Tests

```rust
// tests/library_integration.rs
use color_rs::*;

#[test]
fn test_gradient_builder_integration() {
    let args = GradientBuilder::new()
        .start_color("red")
        .end_color("blue")
        .ease_in_out()
        .steps(10)
        .build()
        .unwrap();
    
    let color_rs = ColorRs::new();
    let result = color_rs.generate_gradient(args);
    
    assert!(result.is_ok());
}

#[test]
fn test_color_operations_facade() {
    let facade = ColorOperationsFacade::new();
    
    // Test hex to RGB conversion
    let rgb = facade.hex_to_rgb("#FF0000").unwrap();
    assert_eq!(rgb, [255, 0, 0]);
    
    // Test contrast calculation
    let contrast = facade.calculate_contrast("#FFFFFF", "#000000").unwrap();
    assert!((contrast - 21.0).abs() < 0.1);
    
    // Test color analysis
    let analysis = facade.analyze_color("#FF0000").unwrap();
    assert_eq!(analysis.rgb, [255, 0, 0]);
    assert_eq!(analysis.hex, "#FF0000");
}

#[test]
fn test_output_filtering_integration() {
    // Test FilterEngine integration with color analysis
    let config = FilterConfig::from_expression("hex,hsl").unwrap();
    let engine = FilterEngine::new(config);
    
    let facade = ColorOperationsFacade::new();
    let analysis = facade.analyze_color("#FF0000").unwrap();
    let filtered = engine.apply(&analysis);
    
    assert!(filtered.hex.is_some());
    assert!(filtered.hsl.is_some());
    assert!(filtered.rgb.is_none());
    assert!(filtered.lab.is_none());
}

#[test]
fn test_filtered_gradient_output() {
    let args = GradientBuilder::new()
        .start_color("red")
        .end_color("blue")
        .steps(5)
        .filter_expression("hex,rgb")
        .build()
        .unwrap();
    
    let color_rs = ColorRs::new();
    let result = color_rs.generate_gradient(args);
    
    assert!(result.is_ok());
    // Each gradient step should only contain hex and rgb outputs
}

#[test]
fn test_color_parser_factory() {
    let parser = ColorParserFactory::create_comprehensive().unwrap();
    
    let (lab, format) = parser.parse("#FF0000").unwrap();
    assert_eq!(format, "HEX");
    
    let (lab, format) = parser.parse("red").unwrap();
    assert_eq!(format, "CSS Named");
    
    let (lab, format) = parser.parse("RAL 3020").unwrap();
    assert_eq!(format, "RAL Classic");
}
```

#### output_filter.rs (v0.14.1+)
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_filter_config_creation() {
        let config = FilterConfig::from_expression("hex,rgb").unwrap();
        assert!(config.include_hex);
        assert!(config.include_rgb);
        assert!(!config.include_hsl);
        assert!(!config.include_lab);
    }
    
    #[test]
    fn test_filter_engine_basic_filtering() {
        let config = FilterConfig::from_expression("hex,hsl").unwrap();
        let engine = FilterEngine::new(config);
        
        let analysis = ColorAnalysisOutput {
            hex: "#FF0000".to_string(),
            rgb: [255, 0, 0],
            hsl: [0.0, 100.0, 50.0],
            lab: [53.24, 80.09, 67.20],
            distance: Some(0.0),
            contrast: Some(21.0),
        };
        
        let filtered = engine.apply(&analysis);
        
        // Should only include hex and hsl
        assert!(filtered.hex.is_some());
        assert!(filtered.hsl.is_some());
        assert!(filtered.rgb.is_none());
        assert!(filtered.lab.is_none());
    }
    
    #[test]
    fn test_filter_expression_parsing() {
        // Test valid expressions
        assert!(FilterConfig::from_expression("hex").is_ok());
        assert!(FilterConfig::from_expression("hex,rgb,hsl,lab").is_ok());
        assert!(FilterConfig::from_expression("RGB,HSL").is_ok()); // Case insensitive
        
        // Test invalid expressions
        assert!(FilterConfig::from_expression("invalid").is_err());
        assert!(FilterConfig::from_expression("hex,invalid,rgb").is_err());
        assert!(FilterConfig::from_expression("").is_err());
    }
    
    #[test]
    fn test_filter_engine_preserves_metadata() {
        let config = FilterConfig::from_expression("hex").unwrap();
        let engine = FilterEngine::new(config);
        
        let analysis = ColorAnalysisOutput {
            hex: "#FF0000".to_string(),
            rgb: [255, 0, 0],
            hsl: [0.0, 100.0, 50.0],
            lab: [53.24, 80.09, 67.20],
            distance: Some(5.5),
            contrast: Some(4.2),
        };
        
        let filtered = engine.apply(&analysis);
        
        // Metadata should be preserved even if color formats are filtered
        assert_eq!(filtered.distance, Some(5.5));
        assert_eq!(filtered.contrast, Some(4.2));
    }
}
```

## Property-Based Testing

### Proptest Integration

```toml
[dev-dependencies]
proptest = "1.0"
```

### Color Conversion Properties

```rust
// src/color_utils.rs
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_rgb_lab_roundtrip_property(
            r in 0u8..=255,
            g in 0u8..=255, 
            b in 0u8..=255
        ) {
            let lab = ColorUtils::rgb_to_lab([r, g, b]);
            let rgb_back = ColorUtils::lab_to_rgb(lab);
            
            // Allow for small rounding errors in conversion
            prop_assert!((rgb_back[0] as i16 - r as i16).abs() <= 2);
            prop_assert!((rgb_back[1] as i16 - g as i16).abs() <= 2);
            prop_assert!((rgb_back[2] as i16 - b as i16).abs() <= 2);
        }
        
        #[test]
        fn test_hex_conversion_property(hex_color in "[0-9A-F]{6}") {
            let hex_with_hash = format!("#{}", hex_color);
            let result = ColorUtils::hex_to_rgb(&hex_with_hash);
            
            prop_assert!(result.is_ok());
            
            let rgb = result.unwrap();
            let hex_back = ColorUtils::rgb_to_hex(rgb);
            
            prop_assert_eq!(hex_back, hex_with_hash);
        }
        
        #[test]
        fn test_luminance_range_property(
            r in 0u8..=255,
            g in 0u8..=255,
            b in 0u8..=255
        ) {
            let luminance = ColorUtils::calculate_luminance([r, g, b]);
            
            // Luminance should always be between 0 and 1
            prop_assert!(luminance >= 0.0);
            prop_assert!(luminance <= 1.0);
        }
        
        #[test]
        fn test_contrast_ratio_property(
            r1 in 0u8..=255, g1 in 0u8..=255, b1 in 0u8..=255,
            r2 in 0u8..=255, g2 in 0u8..=255, b2 in 0u8..=255
        ) {
            let ratio = ColorUtils::contrast_ratio([r1, g1, b1], [r2, g2, b2]);
            
            // Contrast ratio should be between 1 and 21
            prop_assert!(ratio >= 1.0);
            prop_assert!(ratio <= 21.0);
            
            // Contrast should be symmetric
            let ratio_reverse = ColorUtils::contrast_ratio([r2, g2, b2], [r1, g1, b1]);
            prop_assert!((ratio - ratio_reverse).abs() < 0.001);
        }
    }
    
    // Output filtering property tests (v0.14.1+)
    proptest! {
        #[test]
        fn test_filter_preserves_included_formats(
            include_hex in any::<bool>(),
            include_rgb in any::<bool>(),
            include_hsl in any::<bool>(),
            include_lab in any::<bool>()
        ) {
            // Skip if no formats are included (invalid configuration)
            prop_assume!(include_hex || include_rgb || include_hsl || include_lab);
            
            let config = FilterConfig {
                include_hex,
                include_rgb,
                include_hsl,
                include_lab,
            };
            let engine = FilterEngine::new(config);
            
            let analysis = ColorAnalysisOutput {
                hex: "#FF0000".to_string(),
                rgb: [255, 0, 0],
                hsl: [0.0, 100.0, 50.0],
                lab: [53.24, 80.09, 67.20],
                distance: Some(0.0),
                contrast: Some(21.0),
            };
            
            let filtered = engine.apply(&analysis);
            
            // Check that included formats are present
            prop_assert_eq!(filtered.hex.is_some(), include_hex);
            prop_assert_eq!(filtered.rgb.is_some(), include_rgb);
            prop_assert_eq!(filtered.hsl.is_some(), include_hsl);
            prop_assert_eq!(filtered.lab.is_some(), include_lab);
            
            // Metadata should always be preserved
            prop_assert_eq!(filtered.distance, Some(0.0));
            prop_assert_eq!(filtered.contrast, Some(21.0));
        }
        
        #[test]
        fn test_filter_expression_parsing_property(
            expression in "[a-zA-Z]{1,10}(,[a-zA-Z]{1,10}){0,5}"
        ) {
            let result = FilterConfig::from_expression(&expression);
            
            // Valid expressions should only contain hex, rgb, hsl, lab
            let valid_parts: Vec<&str> = expression
                .split(',')
                .filter(|part| ["hex", "rgb", "hsl", "lab"].contains(&part.to_lowercase().as_str()))
                .collect();
            
            if valid_parts.len() == expression.split(',').count() && !expression.is_empty() {
                prop_assert!(result.is_ok());
            } else {
                prop_assert!(result.is_err());
            }
        }
    }
}
```

### Gradient Generation Properties

```rust
// src/gradient.rs
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_gradient_monotonic_property(
            step_count in 2usize..=20,
            ease_in in 0.0..=1.0,
            ease_out in 0.0..=1.0
        ) {
            let calculator = GradientCalculator::new();
            let steps: Vec<f64> = (0..step_count)
                .map(|i| i as f64 / (step_count - 1) as f64)
                .collect();
            
            let gradient = calculator.calculate(
                [0.0, 0.0, 0.0],    // Black
                [100.0, 0.0, 0.0],  // White in LAB
                steps,
                ease_in,
                ease_out
            );
            
            // Positions should be monotonically increasing
            for i in 1..gradient.len() {
                prop_assert!(gradient[i].position >= gradient[i-1].position);
            }
            
            // First and last positions should be at extremes
            prop_assert_eq!(gradient[0].position, 0);
            prop_assert_eq!(gradient[gradient.len()-1].position, 100);
        }
    }
}
```

## Benchmarking

### Criterion Setup

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "color_conversion"
harness = false

[[bench]]
name = "gradient_generation"  
harness = false

[[bench]]
name = "collection_loading"
harness = false
```

### Color Conversion Benchmarks

```rust
// benches/color_conversion.rs
use color_rs::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_rgb_to_lab(c: &mut Criterion) {
    c.bench_function("rgb_to_lab", |b| {
        b.iter(|| {
            ColorUtils::rgb_to_lab(black_box([255, 128, 64]))
        })
    });
}

fn benchmark_lab_to_rgb(c: &mut Criterion) {
    c.bench_function("lab_to_rgb", |b| {
        b.iter(|| {
            ColorUtils::lab_to_rgb(black_box([53.24, 80.09, 67.20]))
        })
    });
}

fn benchmark_hex_parsing(c: &mut Criterion) {
    c.bench_function("hex_to_rgb", |b| {
        b.iter(|| {
            ColorUtils::hex_to_rgb(black_box("#FF8040"))
        })
    });
}

fn benchmark_distance_calculation(c: &mut Criterion) {
    let mut group = c.benchmark_group("distance_algorithms");
    
    let color1 = [53.24, 80.09, 67.20];
    let color2 = [32.30, 79.19, -107.86];
    
    group.bench_function("delta_e_76", |b| {
        let strategy = create_strategy("delta-e-76");
        b.iter(|| {
            strategy.calculate_distance(black_box(color1), black_box(color2))
        })
    });
    
    group.bench_function("delta_e_2000", |b| {
        let strategy = create_strategy("delta-e-2000");
        b.iter(|| {
            strategy.calculate_distance(black_box(color1), black_box(color2))
        })
    });
    
    group.bench_function("euclidean_lab", |b| {
        let strategy = create_strategy("euclidean-lab");
        b.iter(|| {
            strategy.calculate_distance(black_box(color1), black_box(color2))
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_rgb_to_lab,
    benchmark_lab_to_rgb,
    benchmark_hex_parsing,
    benchmark_distance_calculation
);
criterion_main!(benches);
```

### Gradient Generation Benchmarks

```rust
// benches/gradient_generation.rs
use color_rs::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn benchmark_gradient_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("gradient_generation");
    
    let calculator = GradientCalculator::new();
    let start_lab = [53.24, 80.09, 67.20];
    let end_lab = [32.30, 79.19, -107.86];
    
    for step_count in [5, 10, 20, 50, 100].iter() {
        let steps: Vec<f64> = (0..*step_count)
            .map(|i| i as f64 / (*step_count - 1) as f64)
            .collect();
        
        group.bench_with_input(
            BenchmarkId::new("equal_steps", step_count),
            step_count,
            |b, _| {
                b.iter(|| {
                    calculator.calculate(
                        black_box(start_lab),
                        black_box(end_lab), 
                        black_box(steps.clone()),
                        black_box(0.65),
                        black_box(0.35)
                    )
                })
            }
        );
    }
    
    group.finish();
}

criterion_group!(benches, benchmark_gradient_sizes);
criterion_main!(benches);
```

### Output Filtering Benchmarks (v0.14.1+)

```rust
// benches/output_filtering.rs
use color_rs::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn benchmark_filter_engine_creation(c: &mut Criterion) {
    c.bench_function("filter_config_creation", |b| {
        b.iter(|| {
            FilterConfig::from_expression(black_box("hex,rgb,hsl,lab"))
        })
    });
}

fn benchmark_filter_application(c: &mut Criterion) {
    let mut group = c.benchmark_group("filter_application");
    
    let analysis = ColorAnalysisOutput {
        hex: "#FF0000".to_string(),
        rgb: [255, 0, 0],
        hsl: [0.0, 100.0, 50.0],
        lab: [53.24, 80.09, 67.20],
        distance: Some(0.0),
        contrast: Some(21.0),
    };
    
    // Test different filter configurations
    let filter_configs = [
        ("single_format", "hex"),
        ("two_formats", "hex,rgb"),
        ("three_formats", "hex,rgb,hsl"),
        ("all_formats", "hex,rgb,hsl,lab"),
    ];
    
    for (name, expression) in filter_configs.iter() {
        let config = FilterConfig::from_expression(expression).unwrap();
        let engine = FilterEngine::new(config);
        
        group.bench_with_input(
            BenchmarkId::new("filter_apply", name),
            &analysis,
            |b, analysis| {
                b.iter(|| {
                    engine.apply(black_box(analysis))
                })
            }
        );
    }
    
    group.finish();
}

fn benchmark_gradient_with_filtering(c: &mut Criterion) {
    let mut group = c.benchmark_group("filtered_gradient_generation");
    
    let calculator = GradientCalculator::new();
    let start_lab = [53.24, 80.09, 67.20];
    let end_lab = [32.30, 79.19, -107.86];
    let steps: Vec<f64> = (0..10).map(|i| i as f64 / 9.0).collect();
    
    let filter_configs = [
        ("no_filter", None),
        ("hex_only", Some("hex")),
        ("hex_rgb", Some("hex,rgb")),
        ("all_formats", Some("hex,rgb,hsl,lab")),
    ];
    
    for (name, filter_expr) in filter_configs.iter() {
        group.bench_with_input(
            BenchmarkId::new("gradient_with_filter", name),
            filter_expr,
            |b, filter_expr| {
                b.iter(|| {
                    let gradient = calculator.calculate(
                        black_box(start_lab),
                        black_box(end_lab),
                        black_box(steps.clone()),
                        black_box(0.65),
                        black_box(0.35)
                    );
                    
                    if let Some(expr) = filter_expr {
                        let config = FilterConfig::from_expression(expr).unwrap();
                        let engine = FilterEngine::new(config);
                        
                        // Apply filtering to each gradient step
                        for step in &gradient {
                            let analysis = ColorAnalysisOutput {
                                hex: format!("#{:02X}{:02X}{:02X}", 
                                    step.rgb[0], step.rgb[1], step.rgb[2]),
                                rgb: step.rgb,
                                hsl: [0.0, 0.0, 0.0], // Simplified for benchmark
                                lab: step.lab,
                                distance: None,
                                contrast: None,
                            };
                            black_box(engine.apply(&analysis));
                        }
                    }
                })
            }
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_filter_engine_creation,
    benchmark_filter_application,
    benchmark_gradient_with_filtering
);
criterion_main!(benches);
```

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench color_conversion

# Generate HTML reports
cargo bench -- --output-format html

# Compare with baseline
cargo bench -- --save-baseline main
# ... make changes ...
cargo bench -- --baseline main
```

## Code Coverage

### Tarpaulin Setup

```toml
[dev-dependencies]
cargo-tarpaulin = "0.27"
```

### Coverage Commands

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# Coverage with all features
cargo tarpaulin --all-features --out Html

# Exclude integration tests from coverage
cargo tarpaulin --bin color-rs --out Html

# Upload to codecov (CI)
cargo tarpaulin --out Xml
bash <(curl -s https://codecov.io/bash)
```

### Coverage Targets

- **Unit Tests**: Target 90%+ line coverage
- **Integration Tests**: Target 80%+ end-to-end coverage
- **Critical Paths**: 100% coverage for color conversions and gradient calculations

## Continuous Integration

### GitHub Actions Test Matrix

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta, nightly]
        features: [
          "--all-features",
          "--no-default-features",
          "--features cli"
        ]
    
    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: ${{ matrix.rust }}
          override: true
      
      - name: Run tests
        run: cargo test ${{ matrix.features }} --verbose
      
      - name: Run doctests
        run: cargo test ${{ matrix.features }} --doc
```

### Quality Gates

```yaml
  quality:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
          components: rustfmt, clippy
      
      - name: Check formatting
        run: cargo fmt --all -- --check
      
      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings
      
      - name: Generate documentation
        run: cargo doc --all-features --no-deps
      
      - name: Run benchmarks
        run: cargo bench --no-run
```

### Coverage in CI

```yaml
  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      
      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin
      
      - name: Generate coverage
        run: cargo tarpaulin --all-features --out Xml
      
      - name: Upload to codecov
        uses: codecov/codecov-action@v3
```

## Test Execution Guide

### Local Development

```bash
# Quick test during development
cargo check                          # Fast syntax check
cargo test                          # Run unit tests
cargo test --lib                    # Library tests only
cargo test color                    # Tests matching "color"
cargo test filter                   # Tests matching "filter" (v0.14.1+)

# Comprehensive testing
cargo test --all-features           # All features enabled
cargo test --release                # Release mode testing
cargo clippy --all-targets         # Linting
cargo fmt --all                     # Formatting

# Documentation testing
cargo test --doc                    # Test documentation examples
cargo doc --all-features           # Generate documentation
```

### Pre-Commit Testing

```bash
# Complete pre-commit check
./scripts/pre-commit.sh             # Custom script (planned)

# Manual pre-commit steps
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
cargo doc --all-features
```

### Performance Testing

```bash
# Run benchmarks
cargo bench

# Profile performance
cargo build --release
perf record --call-graph=dwarf ./target/release/color-rs gradient red blue
perf report
```

This comprehensive testing strategy ensures color-rs maintains high quality, performance, and reliability across all supported platforms and use cases.
