# Color-rs Architecture

This document describes the crate and module topology, public vs private modules, and data flow in the color-rs project.

## Module Topology

```mermaid
graph TB
    subgraph "Public API (lib.rs)"
        lib[lib.rs]
        lib --> cli[cli.rs]
        lib --> color[color.rs]
        lib --> error[error.rs]
        lib --> gradient[gradient.rs]
        lib --> image[image.rs]
    end
    
    subgraph "Core Color Operations"
        color --> color_utils[color_utils.rs]
        color --> color_formatter[color_formatter.rs]
        color --> color_operations_facade[color_operations_facade.rs]
        color --> color_distance_strategies[color_distance_strategies.rs]
        color --> color_matching_template[color_matching_template.rs]
    end
    
    subgraph "Color Parsing System"
        color --> color_parser_factory[color_parser_factory.rs]
        color_parser_factory --> color_parser[color_parser/]
        
        subgraph "color_parser/"
            color_parser --> mod_rs[mod.rs]
            color_parser --> types[types.rs]
            color_parser --> parse_utils[parse_utils.rs]
            color_parser --> css_parser[css_parser.rs]
            color_parser --> css_collection[css_collection.rs]
            color_parser --> ral_matcher[ral_matcher.rs]
            color_parser --> ral_classic_collection[ral_classic_collection.rs]
            color_parser --> ral_design_collection[ral_design_collection.rs]
            color_parser --> csv_loader[csv_loader.rs]
            color_parser --> collections[collections.rs]
            color_parser --> unified_manager[unified_manager.rs]
            color_parser --> compat[compat.rs]
        end
    end
    
    subgraph "Gradient System"
        gradient --> gradient_builder[gradient_builder.rs]
        gradient_builder --> gradient
    end
    
    subgraph "Utilities"
        utils[utils.rs]
        config[config.rs]
    end
    
    subgraph "External Data"
        csv_data[color-table/*.csv]
        csv_data --> csv_loader
    end
    
    main[main.rs] --> cli
    cli --> color
    cli --> gradient
    cli --> image
```

## Public vs Private Modules

### Public Modules (Re-exported from lib.rs)
The following modules and types are part of the public API:

- **`cli`**: Command-line interface structures and argument parsing
  - `Cli`, `Commands`, `GradientArgs`, `ColorMatchArgs`
- **`color`**: Core color types and operations
  - `ColorInfo`, `ColorSpace`
- **`color_distance_strategies`**: Pluggable distance calculation algorithms
  - `ColorDistanceStrategy`, `available_strategies()`, `create_strategy()`
- **`color_matching_template`**: Template method for color matching
  - `ColorMatchingTemplate`, `UnifiedColorMatcher`
- **`color_operations_facade`**: Simplified interface for color operations
  - `ColorOperationsFacade`, `ColorAnalysis`
- **`color_parser`**: Universal color parsing system
  - `ColorMatch`, `SearchFilter`, `UnifiedColorManager`, `UniversalColor`
- **`color_parser_factory`**: Factory for creating color parsers
  - `ColorParserFactory`, `ColorParserTrait`, `ColorParserType`, `ColorParserConfig`
- **`color_utils`**: Core color manipulation utilities
  - `ColorUtils`
- **`error`**: Error handling types
  - `ColorError`, `Result`
- **`gradient`**: Gradient calculation and generation
  - `GradientCalculator`, `GradientValue`
- **`gradient_builder`**: Fluent builder for gradient configuration
  - `GradientBuilder`
- **`image`**: Image generation and export
  - `ImageGenerator`, `ImageFormat`

### Private Modules
These modules are implementation details not exposed in the public API:

- **`config`**: Internal configuration constants and settings
- **`main`**: CLI application entry point
- **`utils`**: Internal utility functions
- **`color_formatter`**: Internal formatting logic for color output

### Color Parser Submodules
The `color_parser` module contains several submodules that implement different parsing strategies:

- **`types`**: Core type definitions for color parsing
- **`parse_utils`**: Shared parsing utilities and helper functions
- **`css_parser`**: CSS color specification parser
- **`css_collection`**: CSS named color collection
- **`ral_matcher`**: RAL color matching algorithms
- **`ral_classic_collection`**: RAL Classic color collection (213 colors)
- **`ral_design_collection`**: RAL Design System+ collection (1825 colors)
- **`csv_loader`**: CSV file loading and parsing utilities
- **`collections`**: Unified interface for all color collections
- **`unified_manager`**: Central manager for all color parsing operations
- **`compat`**: Compatibility layer for different color formats

## Data Flow Architecture

### Color Input Processing Pipeline

```mermaid
flowchart TD
    A[User Input] --> B{Input Type?}
    B -->|CLI Command| C[CLI Parser clap]
    B -->|Library Call| D[Public API]
    
    C --> E[ColorMatchArgs / GradientArgs]
    D --> E
    
    E --> F[ColorParserFactory]
    F --> G[UnifiedColorManager]
    
    G --> H{Parse Strategy}
    H -->|HEX| I[Direct RGB conversion]
    H -->|RGB/HSL| J[Palette library conversion]
    H -->|Named Color| K[Collection Lookup]
    H -->|RAL Color| L[RAL Matcher]
    
    I --> M[ColorUtils]
    J --> M
    K --> N[CSV Collections]
    L --> N
    
    N --> O[ColorMatch]
    M --> O
    
    O --> P[Color Operations]
    P --> Q{Output Type}
    Q -->|Analysis| R[ColorFormatter]
    Q -->|Gradient| S[GradientCalculator]
    Q -->|Image| T[ImageGenerator]
    
    R --> U[Terminal Output]
    S --> V[SVG/PNG Files]
    T --> V
```

### Color Collection Data Flow

```mermaid
flowchart LR
    A[CSV Files] --> B[CSVLoader]
    B --> C[Collections]
    
    subgraph "Collections"
        C --> D[CSS Collection<br/>148 colors]
        C --> E[RAL Classic<br/>213 colors]
        C --> F[RAL Design+<br/>1825 colors]
    end
    
    D --> G[UnifiedManager]
    E --> G
    F --> G
    
    G --> H[Color Matching]
    H --> I[Distance Calculation]
    I --> J[Best Match Results]
```

### Gradient Generation Flow

```mermaid
flowchart TD
    A[GradientArgs] --> B[GradientBuilder]
    B --> C[Validation]
    C --> D[Color Parsing]
    D --> E[LAB Conversion]
    E --> F[Bezier Curve Setup]
    F --> G[Step Calculation]
    G --> H{Stop Strategy}
    H -->|Equal| I[Equal Distribution]
    H -->|Mathematical| J[Curve Analysis]
    I --> K[Color Interpolation]
    J --> K
    K --> L[Format Conversion]
    L --> M{Output Format}
    M -->|Table| N[ColorFormatter]
    M -->|SVG| O[SVG Generator]
    M -->|PNG| P[Image Renderer]
```

## Design Patterns in Architecture

### Currently Implemented Patterns

1. **Strategy Pattern**: `ColorDistanceStrategy` allows pluggable distance calculation algorithms
2. **Factory Pattern**: `ColorParserFactory` creates different types of color parsers
3. **Builder Pattern**: `GradientBuilder` provides fluent configuration interface
4. **Facade Pattern**: `ColorOperationsFacade` simplifies complex color operations
5. **Template Method Pattern**: `ColorMatchingTemplate` standardizes matching algorithms

### Architectural Benefits

- **Modularity**: Clear separation between parsing, calculation, and output generation
- **Extensibility**: Easy to add new color formats, distance algorithms, or output formats
- **Type Safety**: Rust's type system prevents invalid color operations
- **Performance**: LAB color space ensures perceptually uniform gradients
- **Maintainability**: Design patterns provide clear structure and responsibilities

## Feature Flags and Configuration

Currently, the crate does not use feature flags but is designed to support them for:

- Optional image generation dependencies (`image`, `tiny-skia`, `usvg`, `resvg`)
- Optional CLI interface (`clap`)
- Different color collection backends
- Performance optimizations for specific use cases

## Memory and Performance Considerations

- **Color Collections**: Loaded once and cached in memory for fast lookups
- **LAB Conversions**: Computed on-demand, cached where beneficial
- **Image Generation**: Uses minimal memory streaming for large images
- **Gradient Calculations**: Vectorized operations where possible using `palette` library optimizations
