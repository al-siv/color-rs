# Color-rs v0.8.1 Release Assets

This directory contains release assets for color-rs v0.8.1.

## Windows Executable

The Windows executable `color-rs.exe` is built with release optimizations and includes all dependencies.

### File Details
- Size: ~8.1 MB
- Architecture: x86_64
- Dependencies: Statically linked
- No installation required

### Usage
1. Download `color-rs.exe`
2. Run directly from command line or place in PATH
3. Use `color-rs.exe --help` to get started

### Examples
```bash
# Generate gradient
color-rs.exe gradient --start-color red --end-color blue

# Analyze color
color-rs.exe color-match "#FF5733"

# Create SVG
color-rs.exe gradient --start-color "#FF0000" --end-color "#0000FF" --svg
```
