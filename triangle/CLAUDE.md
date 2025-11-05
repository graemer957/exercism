# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Context

This is an Exercism Rust exercise for the "Triangle" problem. The goal is to classify triangles as equilateral, isosceles, or scalene based on their side lengths, while validating triangle inequality rules.

## Common Commands

### Testing
```bash
# Run all tests
cargo test

# Run tests with generic/floating-point support
cargo test --features generic

# Run a specific test by name
cargo test some_test

# Run only ignored tests
cargo test -- --ignored

# Run all tests including ignored ones
cargo test -- --include-ignored
```

### Building
```bash
# Build the project
cargo build

# Build with release optimizations
cargo build --release
```

### Linting
```bash
# Check code without building
cargo check

# Run clippy for lints
cargo clippy

# Format code
cargo fmt
```

## Code Architecture

The implementation uses Rust generics to support both integer and floating-point triangle types:

- **`src/lib.rs`**: Contains the generic `Triangle<T>` struct with trait bounds `Add<Output = T> + Copy + Default + PartialOrd`
  - `build([T; 3])`: Factory method that validates triangle inequality and returns `Option<Triangle<T>>`
  - `is_equilateral()`, `is_isosceles()`, `is_scalene()`: Classification methods

- **`tests/triangle.rs`**: Integration tests organized into modules by triangle type and validity
  - Tests with `#[cfg(feature = "generic")]` only run when the `generic` feature is enabled

### Key Implementation Details

- Triangle validation requires all sides > 0 and triangle inequality: `a + b ≥ c`, `a + c ≥ b`, `b + c ≥ a`
- The generic implementation works with any type that supports addition, comparison, copying, and has a default (zero) value
- Equilateral triangles are also considered isosceles per exercise requirements

## Exercism Workflow

Submit solutions with:
```bash
exercism submit src/lib.rs Cargo.toml
```
