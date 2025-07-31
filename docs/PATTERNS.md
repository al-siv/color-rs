# Programming Patterns with Functional Emphasis

## Overview

This document serves as the **SECONDARY** programming patterns catalog for the color-rs project, focusing on the evaluation and migration away from traditional object-oriented patterns toward modern functional programming approaches in Rust. While functional patterns (documented in `PATTERNS_FUNCTIONAL.md`) are the primary approach, this document provides guidance on when traditional patterns might still be appropriate and how to migrate away from problematic OOP patterns.

## Philosophy: Functional First, Careful OOP Evaluation

Traditional Gang of Four (GoF) design patterns were created to address limitations in languages like C++ and Java that lacked modern functional programming features. Rust's rich type system, ownership model, and functional capabilities make many of these patterns unnecessary or even counterproductive. This document helps identify when to avoid traditional patterns and provides functional alternatives.

## ⚠️ Deprecated Patterns - Avoid or Replace

The following traditional OOP patterns should be **avoided** or **replaced** with functional alternatives in color-rs:

### 1. Singleton Pattern - ❌ DEPRECATED

**Problem**: Global mutable state, difficult testing, hidden dependencies.

**Why deprecated in Rust**: 
- Violates Rust's ownership principles
- Creates hidden global state that's hard to test
- Thread safety issues with mutable globals
- Dependency injection is clearer and more testable

**Color-rs specific issues**:
```rust
// ❌ AVOID: Global color configuration singleton
struct ColorConfig {
    precision: usize,
    default_space: ColorSpace,
}

impl ColorConfig {
    fn instance() -> &'static mut ColorConfig {
        // Dangerous global mutable state
        todo!("Singleton implementation")
    }
}
```

**Functional alternatives**:
```rust
// ✅ PREFER: Immutable configuration with dependency injection
#[derive(Clone)]
pub struct ColorConfig {
    pub precision: usize,
    pub default_space: ColorSpace,
}

impl ColorConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(ColorConfig {
            precision: env::var("COLOR_PRECISION")?.parse()?,
            default_space: ColorSpace::Lab,
        })
    }
}

// Usage: Pass configuration explicitly
pub fn analyze_color(input: &str, config: &ColorConfig) -> Result<ColorAnalysisResult> {
    // Function uses injected configuration
}
```

### 2. Strategy Pattern - ⚠️ REQUIRES MIGRATION

**Current usage in color-rs**: Distance calculation algorithms
**Status**: **REQUIRES MIGRATION** to functional approach

**Existing OOP approach**:
```rust
// ❌ PROBLEMATIC: Traditional strategy with trait objects
pub trait ColorDistanceStrategy: Send + Sync {
    fn calculate_distance(&self, color1: Lab, color2: Lab) -> f64;
    fn name(&self) -> &str;
}

pub fn create_strategy(name: &str) -> Box<dyn ColorDistanceStrategy> {
    match name {
        "lch" => Box::new(LchStrategy),
        "delta-e-2000" => Box::new(DeltaE2000Strategy),
        _ => Box::new(LchStrategy),
    }
}
```

**Functional alternative**:
```rust
// ✅ PREFER: Function-based strategy
pub type DistanceFunction = fn([f64; 3], [f64; 3]) -> f64;

pub fn get_distance_function(name: &str) -> DistanceFunction {
    match name {
        "lch" => lch_distance,
        "delta-e-2000" => delta_e_2000_distance,
        "euclidean" => euclidean_distance,
        _ => lch_distance,
    }
}

// Pure functions for distance calculations
pub fn lch_distance(color1: [f64; 3], color2: [f64; 3]) -> f64 {
    // Implementation
}

pub fn delta_e_2000_distance(color1: [f64; 3], color2: [f64; 3]) -> f64 {
    // Implementation  
}

// Usage with higher-order functions
pub fn find_closest_color<F>(target: [f64; 3], colors: &[[f64; 3]], distance_fn: F) -> Option<usize>
where
    F: Fn([f64; 3], [f64; 3]) -> f64,
{
    colors
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            distance_fn(target, **a)
                .partial_cmp(&distance_fn(target, **b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(i, _)| i)
}
```

### 3. Factory Pattern - ❌ DEPRECATED

**Problem**: Complex object creation hierarchies, hidden dependencies.

**Why deprecated**: Rust's associated functions and enum variants provide better alternatives.

**Color-rs specific issues**:
```rust
// ❌ AVOID: Complex factory hierarchy
trait ColorParserFactory {
    fn create_parser(&self) -> Box<dyn ColorParser>;
}

struct CssColorParserFactory;
struct RalColorParserFactory;
```

**Functional alternatives**:
```rust
// ✅ PREFER: Enum variants and associated functions
#[derive(Debug, Clone)]
pub enum ColorParser {
    Css(CssParser),
    Ral(RalParser),
    Full(FullParser),
}

impl ColorParser {
    pub fn css() -> Self {
        ColorParser::Css(CssParser::new())
    }
    
    pub fn ral() -> Self {
        ColorParser::Ral(RalParser::new())
    }
    
    pub fn full() -> Self {
        ColorParser::Full(FullParser::new())
    }
    
    pub fn parse(&self, input: &str) -> Result<ParsedColor> {
        match self {
            ColorParser::Css(parser) => parser.parse(input),
            ColorParser::Ral(parser) => parser.parse(input),
            ColorParser::Full(parser) => parser.parse(input),
        }
    }
}

// Or even better: direct function approach
pub fn parse_css_color(input: &str) -> Result<ParsedColor> {
    // Direct implementation
}

pub fn parse_ral_color(input: &str) -> Result<ParsedColor> {
    // Direct implementation
}
```

### 4. Template Method Pattern - ❌ DEPRECATED

**Problem**: Inheritance-based code reuse, rigid structure.

**Color-rs specific issues**:
```rust
// ❌ AVOID: Template method with trait inheritance
trait ColorMatchingTemplate {
    fn find_closest_color(&self, target: &str) -> Result<ColorMatch> {
        let parsed = self.parse_input(target)?;
        let collection = self.get_collection();
        let best = self.find_best_match(&parsed, collection)?;
        self.format_result(best)
    }
    
    fn parse_input(&self, input: &str) -> Result<ParsedColor>;
    fn get_collection(&self) -> &[Color];
    fn find_best_match(&self, target: &ParsedColor, collection: &[Color]) -> Result<&Color>;
    fn format_result(&self, color: &Color) -> Result<ColorMatch>;
}
```

**Functional alternative**:
```rust
// ✅ PREFER: Higher-order functions and composition
pub fn find_closest_color<P, C, M, F>(
    target: &str,
    parse_fn: P,
    collection_fn: C,
    match_fn: M,
    format_fn: F,
) -> Result<ColorMatch>
where
    P: Fn(&str) -> Result<ParsedColor>,
    C: Fn() -> &'static [Color],
    M: Fn(&ParsedColor, &[Color]) -> Result<&Color>,
    F: Fn(&Color) -> Result<ColorMatch>,
{
    let parsed = parse_fn(target)?;
    let collection = collection_fn();
    let best = match_fn(&parsed, collection)?;
    format_fn(best)
}

// Or even simpler: direct function pipeline
pub fn find_closest_css_color(target: &str) -> Result<ColorMatch> {
    let parsed = parse_css_color(target)?;
    let best = find_best_match_in_collection(&parsed, &CSS_COLORS)?;
    format_color_match(best)
}
```

## Color-rs Specific Pattern Assessment

### Current Pattern Usage Analysis Required:

1. **Strategy Pattern** (distance calculations) - **REQUIRES MIGRATION**
2. **Template Method Pattern** (color matching) - **REQUIRES MIGRATION**  
3. **Factory Pattern** (parser creation) - **REQUIRES MIGRATION**
4. **Facade Pattern** (main API) - **REVIEW AND SIMPLIFY**
5. **Builder Pattern** (gradient configuration) - **REVIEW FOR FUNCTIONAL APPROACH**

### Migration Strategy

#### Phase 1: Assessment
1. Identify all current GoF pattern usage in color-rs
2. Categorize patterns by deprecation status  
3. Document specific instances requiring migration

#### Phase 2: Functional Replacement
1. Replace deprecated patterns with functional alternatives
2. Maintain API compatibility where possible
3. Update tests to verify functional equivalence

#### Phase 3: Cleanup
1. Remove deprecated pattern implementations
2. Update documentation to reflect functional approach
3. Add examples of functional pattern usage

### Patterns Requiring Immediate Attention

**Files to analyze for pattern migration**:
- `src/color_distance_strategies.rs` - Strategy pattern migration
- `src/color_matching_template.rs` - Template method migration  
- `src/color_parser_factory.rs` - Factory pattern migration
- `src/color_operations_facade.rs` - Facade pattern review
- `src/gradient_builder.rs` - Builder pattern functional conversion

This document guides the transition from traditional OOP patterns to functional programming approaches that better align with Rust's capabilities and color-rs's domain requirements.
