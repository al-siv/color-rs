# Migration Guide: v0.15.x to v0.16.0

This guide helps users migrate from the GoF design patterns used in v0.15.x to the new functional programming approach in v0.16.0.

## Overview

Version 0.16.0 introduces a major architectural change from GoF patterns (Factory, Strategy, Command) to functional programming patterns. To ease migration, backward compatibility shims are provided with deprecation warnings.

## Deprecated Patterns and Their Replacements

### 1. Color Parser Factory Pattern → Functional Parsing

**Old Code (v0.15.x):**
```rust
use color_rs::{ColorParserFactory, ColorParserType};

// Old factory pattern
let parser = ColorParserFactory::create_parser(ColorParserType::Css)?;
let color = parser.parse("#FF0000")?;
let name = parser.get_color_name((255, 0, 0));
```

**New Code (v0.16.0):**
```rust
use color_rs::color_parser_functional::{parse_color_functional, get_color_name_functional, ParserType, ParsingConfig};

// New functional approach
let config = ParsingConfig::new()
    .with_parser_type(ParserType::Css)
    .with_color_tolerance(10.0);
    
let color = parse_color_functional("#FF0000", &config)?;
let name = get_color_name_functional([255, 0, 0], &config);
```

**Compatibility Shim (Temporary):**
```rust
use color_rs::{create_parser, ColorParserType}; // Deprecated

// This still works but shows deprecation warnings
let parser = create_parser(ColorParserType::Css)?;
let color = parser.parse("#FF0000")?;
```

### 2. Command Pattern → Functional Commands

**Old Code (v0.15.x):**
```rust
use color_rs::{Command, GradientCommand};

// Old command pattern
let cmd = GradientCommand::new(args);
let result = cmd.execute()?;
```

**New Code (v0.16.0):**
```rust
use color_rs::command_functional::{CommandType, execute_command_functional, ExecutionContext};

// New functional approach
let command_type = CommandType::GenerateGradient {
    args: gradient_args,
    output_path: None,
};

let context = ExecutionContext::new(command_type)
    .with_pre_hook(PreHookStep::ValidateParameters)
    .with_post_hook(PostHookStep::FormatOutput);
    
let result = execute_command_functional(&context)?;
```

**Compatibility Shim (Temporary):**
```rust
use color_rs::{execute_legacy_command, CommandType}; // Deprecated

// This still works but shows deprecation warnings
let result = execute_legacy_command(CommandType::GenerateGradient { args, output_path: None })?;
```

### 3. Strategy Pattern → Function Composition

**Old Code (v0.15.x):**
```rust
use color_rs::{DistanceStrategy, DeltaE2000Strategy};

// Old strategy pattern
let strategy = DeltaE2000Strategy::new();
let distance = strategy.calculate_distance(color1, color2);
```

**New Code (v0.16.0):**
```rust
use color_rs::color_distance_strategies::{calculate_distance_functional, DistanceAlgorithm};

// New functional approach
let distance = calculate_distance_functional(
    color1, 
    color2, 
    DistanceAlgorithm::DeltaE2000
)?;
```

## Migration Steps

### Step 1: Update Dependencies
Ensure you're using v0.16.0:
```toml
[dependencies]
color-rs = "0.16.0"
```

### Step 2: Identify Usage Patterns
Search your code for:
- `ColorParserFactory`
- `ColorParserType` 
- `Command` implementations
- Strategy pattern usage

### Step 3: Gradual Migration
1. **Phase 1**: Update imports to use compatibility shims (immediate compilation)
2. **Phase 2**: Replace with functional equivalents (removes deprecation warnings)
3. **Phase 3**: Remove compatibility shim usage entirely

### Step 4: Update Imports

**Before:**
```rust
use color_rs::{
    ColorParserFactory, 
    ColorParserType, 
    Command,
    DistanceStrategy
};
```

**After:**
```rust
use color_rs::{
    color_parser_functional::{parse_color_functional, ParsingConfig, ParserType},
    command_functional::{execute_command_functional, CommandType, ExecutionContext},
    color_distance_strategies::{calculate_distance_functional, DistanceAlgorithm}
};
```

## Key Benefits of the New Approach

1. **Better Testability**: Functions are easier to test than objects
2. **Improved Performance**: No dynamic dispatch overhead
3. **Functional Composition**: Chain operations more naturally
4. **Reduced Complexity**: Less boilerplate code
5. **Better Error Handling**: More explicit error types

## Compatibility Timeline

- **v0.16.0**: Compatibility shims available with deprecation warnings
- **v0.17.0**: Compatibility shims will be removed (breaking change)

## Common Migration Issues

### Issue 1: Missing Configuration
**Problem**: Functional APIs require explicit configuration
**Solution**: Create appropriate config objects:
```rust
let config = ParsingConfig::new()
    .with_parser_type(ParserType::Css)
    .with_color_tolerance(10.0);
```

### Issue 2: Different Error Types
**Problem**: Error types may have changed
**Solution**: Update error handling:
```rust
match result {
    Ok(color) => { /* handle success */ },
    Err(e) => { /* handle specific error types */ },
}
```

### Issue 3: Changed Function Signatures
**Problem**: Parameters might be in different order
**Solution**: Check the API documentation for exact signatures

## Examples

### Complete Migration Example

**Before (v0.15.x):**
```rust
use color_rs::{ColorParserFactory, ColorParserType, GradientCommand};

fn process_colors() -> Result<(), Box<dyn std::error::Error>> {
    // Parse color
    let parser = ColorParserFactory::create_parser(ColorParserType::Css)?;
    let color = parser.parse("#FF0000")?;
    
    // Generate gradient
    let cmd = GradientCommand::new(gradient_args);
    let result = cmd.execute()?;
    
    Ok(())
}
```

**After (v0.16.0):**
```rust
use color_rs::{
    color_parser_functional::{parse_color_functional, ParsingConfig, ParserType},
    command_functional::{execute_command_functional, CommandType, ExecutionContext}
};

fn process_colors() -> Result<(), Box<dyn std::error::Error>> {
    // Parse color
    let config = ParsingConfig::new().with_parser_type(ParserType::Css);
    let color = parse_color_functional("#FF0000", &config)?;
    
    // Generate gradient
    let command_type = CommandType::GenerateGradient {
        args: gradient_args,
        output_path: None,
    };
    let context = ExecutionContext::new(command_type);
    let result = execute_command_functional(&context)?;
    
    Ok(())
}
```

## Support

If you encounter issues during migration:
1. Check this guide for common patterns
2. Review the API documentation
3. Use compatibility shims temporarily
4. File issues on GitHub for unhandled cases

Remember: The compatibility shims will be removed in v0.17.0, so complete your migration by then.
