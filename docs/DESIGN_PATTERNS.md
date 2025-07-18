# Color-rs Design Patterns

This document catalogs the Gang of Four design patterns used in the color-rs codebase, explaining why each was chosen, where it's implemented, how it's expressed in Rust, and the trade-offs involved.

## Table of Contents

- [Currently Implemented Patterns](#currently-implemented-patterns)
- [Pattern Catalog](#pattern-catalog)
- [Unused Patterns](#unused-patterns)
- [Rust-Specific Considerations](#rust-specific-considerations)

## Currently Implemented Patterns

### 1. Strategy Pattern ⭐

**Problem & Forces**: Need to support multiple color distance calculation algorithms with different performance and accuracy characteristics. Users may want to choose between fast approximations and perceptually accurate calculations.

**Where it lives**: 
- Module: `src/color_distance_strategies.rs`
- Trait: `ColorDistanceStrategy`
- Concrete implementations: `DeltaE76Strategy`, `DeltaE2000Strategy`, `EuclideanLabStrategy`

**How it's expressed in Rust**:
```rust
pub trait ColorDistanceStrategy: Send + Sync {
    fn calculate_distance(&self, color1: [f64; 3], color2: [f64; 3]) -> f64;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

// Factory function for strategy creation
pub fn create_strategy(name: &str) -> Box<dyn ColorDistanceStrategy> {
    match name {
        "delta-e-76" => Box::new(DeltaE76Strategy),
        "delta-e-2000" => Box::new(DeltaE2000Strategy),
        "euclidean-lab" => Box::new(EuclideanLabStrategy),
        _ => Box::new(DeltaE2000Strategy), // Default
    }
}
```

**Trade-offs**:
- ✅ **Pros**: Easy to add new algorithms, runtime selection, clean separation of concerns
- ❌ **Cons**: Slight performance overhead from dynamic dispatch, more complex than direct function calls
- **Alternatives**: Function pointers, enum dispatch, compile-time generics

---

### 2. Builder Pattern ⭐

**Problem & Forces**: Gradient configuration has many optional parameters with sensible defaults. Users need a fluent, discoverable API that prevents invalid configurations and supports method chaining.

**Where it lives**:
- Module: `src/gradient_builder.rs`
- Type: `GradientBuilder`
- Usage: Fluent interface for constructing `GradientArgs`

**How it's expressed in Rust**:
```rust
pub struct GradientBuilder {
    start_color: Option<String>,
    end_color: Option<String>,
    ease_in: f64,
    ease_out: f64,
    // ... other fields
}

impl GradientBuilder {
    pub fn new() -> Self { /* ... */ }
    
    // Ownership-taking methods for fluent chaining
    pub fn start_color<S: AsRef<str>>(mut self, color: S) -> Self {
        self.start_color = Some(color.as_ref().to_string());
        self
    }
    
    pub fn ease_in_out(mut self) -> Self {
        self.ease_in = 0.42;
        self.ease_out = 0.58;
        self
    }
    
    // Consuming build method
    pub fn build(self) -> Result<GradientArgs> { /* validation & construction */ }
}
```

**Trade-offs**:
- ✅ **Pros**: Discoverable API, prevents invalid states, fluent syntax, strong defaults
- ❌ **Cons**: More complex than direct struct construction, additional validation logic
- **Alternatives**: Direct struct construction, macro-based DSL, config files

---

### 3. Factory Pattern ⭐

**Problem & Forces**: Need to create different types of color parsers with varying capabilities (CSS-only, full-featured, custom configurations). Parser creation involves complex initialization and validation.

**Where it lives**:
- Module: `src/color_parser_factory.rs`
- Type: `ColorParserFactory`
- Products: Various implementations of `ColorParserTrait`

**How it's expressed in Rust**:
```rust
pub enum ColorParserType {
    Css,
    Full, 
    Custom,
}

pub struct ColorParserFactory;

impl ColorParserFactory {
    pub fn create_parser(parser_type: ColorParserType) -> Result<Box<dyn ColorParserTrait>> {
        match parser_type {
            ColorParserType::Css => Ok(Box::new(CssColorParser::new()?)),
            ColorParserType::Full => Ok(Box::new(FullColorParser::new()?)),
            ColorParserType::Custom => Ok(Box::new(CustomColorParser::new()?)),
        }
    }
    
    // Convenience factory methods
    pub fn create_fast() -> Result<Box<dyn ColorParserTrait>> { /* ... */ }
    pub fn create_comprehensive() -> Result<Box<dyn ColorParserTrait>> { /* ... */ }
}
```

**Trade-offs**:
- ✅ **Pros**: Centralizes complex creation logic, easy to add new parser types, configuration abstraction
- ❌ **Cons**: Indirection overhead, runtime type decisions, trait object limitations
- **Alternatives**: Enum-based parsers, direct construction, dependency injection

---

### 4. Facade Pattern ⭐

**Problem & Forces**: The color manipulation API is complex with many interdependent modules. Users need a simplified interface for common operations without understanding internal complexity.

**Where it lives**:
- Module: `src/color_operations_facade.rs`
- Type: `ColorOperationsFacade`
- Simplifies: Color parsing, conversion, analysis, and calculation operations

**How it's expressed in Rust**:
```rust
pub struct ColorOperationsFacade {
    parser: Box<dyn ColorParserTrait>,
    distance_strategy: Box<dyn ColorDistanceStrategy>,
}

impl ColorOperationsFacade {
    pub fn new() -> Self { /* ... */ }
    
    // Simplified high-level operations
    pub fn hex_to_rgb(&self, hex: &str) -> Result<[u8; 3]> { /* ... */ }
    
    pub fn calculate_contrast(&self, color1: &str, color2: &str) -> Result<f64> { /* ... */ }
    
    pub fn analyze_color(&self, color: &str) -> Result<ColorAnalysis> { /* ... */ }
    
    pub fn mix_colors(&self, color1: &str, color2: &str, ratio: f64) -> Result<String> { /* ... */ }
}
```

**Trade-offs**:
- ✅ **Pros**: Simple API for common tasks, hides complexity, good for beginners
- ❌ **Cons**: May limit advanced usage, potential performance overhead, can become bloated
- **Alternatives**: Direct module usage, multiple specialized facades, free functions

---

### 5. Template Method Pattern ⭐

**Problem & Forces**: Color matching algorithms follow a common pattern (parse input → get collection → calculate distances → format results) but differ in specific implementations. Need to enforce the algorithm structure while allowing customization.

**Where it lives**:
- Module: `src/color_matching_template.rs`
- Trait: `ColorMatchingTemplate`
- Implementations: `CssColorMatcher`, `RalClassicMatcher`, `UnifiedColorMatcher`

**How it's expressed in Rust**:
```rust
pub trait ColorMatchingTemplate {
    // Template method - defines algorithm skeleton
    fn find_closest_color(&self, target: &str) -> Result<ColorMatch> {
        let parsed = self.parse_input(target)?;
        let collection = self.get_collection();
        let mut best_match = None;
        let mut best_distance = f64::INFINITY;
        
        for color in collection {
            let distance = self.calculate_distance(&parsed, color);
            if distance < best_distance {
                best_distance = distance;
                best_match = Some(color);
            }
        }
        
        match best_match {
            Some(color) => Ok(self.format_result(color, best_distance)),
            None => Err(ColorError::NoMatchFound),
        }
    }
    
    // Abstract methods - implemented by concrete classes
    fn parse_input(&self, input: &str) -> Result<UniversalColor>;
    fn get_collection(&self) -> &[UniversalColor];
    fn calculate_distance(&self, color1: &UniversalColor, color2: &UniversalColor) -> f64;
    fn format_result(&self, color: &UniversalColor, distance: f64) -> ColorMatch;
}
```

**Trade-offs**:
- ✅ **Pros**: Code reuse, enforced algorithm structure, extensible
- ❌ **Cons**: Inheritance-like complexity, trait object overhead
- **Alternatives**: Free functions with higher-order functions, composition over inheritance

## Pattern Catalog

### Creational Patterns

#### Abstract Factory (Recommended for Implementation)

**Problem**: Need to create families of related color objects (parser + matcher + formatter) for different color systems.

**Proposed Implementation**:
```rust
trait ColorSystemFactory {
    fn create_parser(&self) -> Box<dyn ColorParserTrait>;
    fn create_matcher(&self) -> Box<dyn ColorMatchingTemplate>;
    fn create_formatter(&self) -> Box<dyn ColorFormatter>;
}

struct CssColorSystemFactory;
struct RalClassicSystemFactory;
struct RalDesignSystemFactory;
```

**Benefits**: Would ensure consistency between related components and simplify adding new color systems.

#### Prototype (Low Priority)

**Problem**: Expensive color object creation could benefit from cloning existing instances.

**Current Status**: Not needed - color objects are lightweight and `Clone` trait provides sufficient functionality.

#### Singleton (Medium Priority)

**Problem**: Color collection loading is expensive and should be shared globally.

**Rust Implementation Considerations**: 
- Use `lazy_static` or `std::sync::Once` for thread-safe initialization
- Consider `Arc<Mutex<T>>` for shared mutable state
- Rust's ownership model makes singleton less necessary than in other languages

### Structural Patterns

#### Adapter (High Priority for Implementation)

**Problem**: Different color format representations need unified interfaces.

**Proposed Implementation**:
```rust
trait ColorAdapter {
    fn to_universal(&self) -> UniversalColor;
    fn from_universal(&self, color: &UniversalColor) -> Self;
}

struct HexColorAdapter(String);
struct RgbColorAdapter([u8; 3]);
struct HslColorAdapter([f64; 3]);

impl ColorAdapter for HexColorAdapter {
    fn to_universal(&self) -> UniversalColor { /* ... */ }
    fn from_universal(&self, color: &UniversalColor) -> Self { /* ... */ }
}
```

**Benefits**: Would unify color format handling and reduce conversion boilerplate.

#### Bridge (Medium Priority)

**Problem**: Color representation could be separated from color operations.

**Potential Application**: Separate abstract color interface from concrete palette library implementation.

#### Composite (Medium Priority)

**Problem**: Complex gradients with multiple segments could be represented as trees.

**Potential Application**:
```rust
trait GradientComponent {
    fn render(&self, position: f64) -> [f64; 3];
}

struct SimpleGradient { /* ... */ }
struct CompositeGradient {
    segments: Vec<Box<dyn GradientComponent>>,
}
```

#### Decorator (High Priority for Implementation)

**Problem**: Color operations need optional features like caching, logging, validation without modifying core implementations.

**Proposed Implementation**:
```rust
struct CachingColorParser {
    inner: Box<dyn ColorParserTrait>,
    cache: std::collections::HashMap<String, UniversalColor>,
}

struct ValidatingColorParser {
    inner: Box<dyn ColorParserTrait>,
}

struct LoggingColorParser {
    inner: Box<dyn ColorParserTrait>,
}
```

**Benefits**: Would allow modular addition of features without changing core parser implementations.

#### Flyweight (Low Priority)

**Problem**: Memory usage for large color collections could be optimized.

**Current Status**: Not critical - color data is relatively small and loading time is acceptable.

#### Proxy (Medium Priority)

**Problem**: Color collection loading could benefit from lazy initialization and caching.

**Potential Application**: Lazy-loading proxies for RAL color collections.

### Behavioral Patterns

#### Chain of Responsibility (High Priority for Implementation)

**Problem**: Color parsing should try multiple strategies in sequence with fallback logic.

**Proposed Implementation**:
```rust
trait ColorParsingHandler {
    fn handle(&self, input: &str) -> Option<UniversalColor>;
    fn set_next(&mut self, next: Box<dyn ColorParsingHandler>);
}

struct HexParsingHandler { next: Option<Box<dyn ColorParsingHandler>> }
struct RgbParsingHandler { next: Option<Box<dyn ColorParsingHandler>> }
struct NamedColorHandler { next: Option<Box<dyn ColorParsingHandler>> }
```

**Benefits**: Would create a flexible parsing pipeline with clear precedence and easy extensibility.

#### Command (High Priority for Implementation)

**Problem**: Color operations could benefit from encapsulation for undo/redo, logging, and batch processing.

**Proposed Implementation**:
```rust
trait ColorCommand {
    fn execute(&self) -> Result<ColorCommandResult>;
    fn undo(&self) -> Result<()>;
    fn description(&self) -> &str;
}

struct GenerateGradientCommand { args: GradientArgs }
struct AnalyzeColorCommand { color: String }
struct ConvertColorCommand { from: String, to: ColorSpace }
```

**Benefits**: Would enable command history, batch operations, and reversible actions.

#### Interpreter (Low Priority)

**Problem**: Complex color expressions like "mix(red, blue, 30%)" could benefit from a DSL.

**Current Status**: Current string-based parsing is sufficient for current needs.

#### Iterator (Already Satisfied)

Rust's built-in `Iterator` trait provides excellent iteration capabilities that surpass the traditional Iterator pattern.

#### Mediator (Medium Priority)

**Problem**: Complex interactions between parser, matcher, and formatter could be coordinated.

**Potential Application**: Central coordinator for color processing pipeline.

#### Memento (Low Priority)

**Problem**: Color/gradient state snapshots could enable undo functionality.

**Current Status**: Not needed for current CLI-focused use case.

#### Observer (Medium Priority)

**Problem**: Color operations could notify interested parties about changes.

**Potential Application**: Reactive updates when color collections are reloaded.

#### State (Low Priority)

**Problem**: Color parser behavior could change based on configuration state.

**Current Status**: Current parser factory approach is sufficient.

#### Visitor (Low Priority)

**Problem**: Different operations on color objects without modifying them.

**Current Status**: Rust's trait system provides better alternatives through extension traits.

## Unused Patterns

### Why Certain Patterns Are Not Relevant

#### Abstract Factory
**Status**: Partially used (Factory pattern), could be enhanced
**Note**: Current factory is sufficient for single-family creation

#### Prototype  
**Status**: Not relevant
**Reason**: Rust's `Clone` trait provides better primitive for object copying

#### Singleton
**Status**: Language feature replaces it
**Reason**: Rust's module system and `lazy_static` provide better alternatives

#### Bridge
**Status**: Not relevant currently
**Reason**: Current abstraction level is appropriate

#### Composite
**Status**: Future consideration
**Reason**: Current gradient model doesn't require tree structures

#### Flyweight
**Status**: Not relevant
**Reason**: Memory usage is not a concern with current data sizes

#### Interpreter
**Status**: Not relevant
**Reason**: Simple string parsing is sufficient

#### Mediator
**Status**: Future consideration  
**Reason**: Current module interactions are not complex enough

#### Memento
**Status**: Not relevant
**Reason**: CLI tool doesn't require state history

#### Observer
**Status**: Language feature replaces it
**Reason**: Rust's channels and async/await provide better alternatives

#### State
**Status**: Language feature replaces it
**Reason**: Rust's enum pattern matching provides better state handling

#### Visitor
**Status**: Language feature replaces it
**Reason**: Rust's trait system and pattern matching provide better alternatives

## Rust-Specific Considerations

### Pattern Adaptations for Rust

1. **Ownership Model**: Many patterns need adaptation for Rust's ownership rules
2. **Trait Objects**: Used for runtime polymorphism instead of inheritance
3. **Enum Matching**: Often replaces State and Visitor patterns
4. **Zero-Cost Abstractions**: Generics preferred over trait objects where possible
5. **Error Handling**: `Result` type integration in all pattern implementations

### Performance Considerations

1. **Dynamic Dispatch**: Trait objects have runtime cost - use sparingly
2. **Memory Layout**: Avoid unnecessary heap allocations in pattern implementations
3. **Compile-time Optimization**: Prefer generics over trait objects where runtime flexibility isn't needed

### Best Practices

1. **Start Simple**: Implement patterns only when complexity justifies them
2. **Rust Idioms**: Use Rust's strengths (ownership, pattern matching, traits) rather than forcing OOP patterns
3. **Documentation**: Each pattern implementation should explain why it was chosen over alternatives
4. **Testing**: Pattern implementations should have comprehensive unit tests

## Future Pattern Implementation Roadmap

### Phase 1: High-Priority Patterns
1. **Adapter Pattern**: Unify color format conversions
2. **Chain of Responsibility**: Flexible color parsing pipeline
3. **Decorator Pattern**: Add caching and validation features
4. **Command Pattern**: Encapsulate operations for better testing

### Phase 2: Medium-Priority Patterns
1. **Abstract Factory**: Enhanced factory for color system families
2. **Composite**: Complex gradient support
3. **Observer**: Reactive updates for collections

### Phase 3: Evaluation
1. Review implemented patterns for effectiveness
2. Consider additional patterns based on new requirements
3. Refactor existing code to use proven pattern implementations
