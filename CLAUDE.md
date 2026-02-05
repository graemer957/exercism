# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Context

This is a personal repository for completing [Exercism](https://exercism.org/) Rust track exercises. Each subdirectory is a standalone Rust crate for a single exercise.

## Repository Structure

Each exercise directory contains:
- `src/lib.rs` - Main implementation file
- `tests/` - Integration tests (some exercises have tests here)
- `Cargo.toml` - Exercise manifest
- `README.md` - Exercise instructions
- `HELP.md` - Rust-specific help

## Working Directory

Always launch Claude Code from within an exercise directory (e.g., `cd triangle && claude`), not from the repo root. This ensures:
- This repo-level `CLAUDE.md` is loaded (via parent directory discovery)
- Any exercise-specific `CLAUDE.md` is also loaded
- The working context is correctly scoped to that exercise

## Common Commands

### Running Tests

```bash
# Test a specific exercise (from exercise directory)
cd <exercise-name>
cargo test

# Test a specific exercise (from repo root)
cargo test --manifest-path <exercise-name>/Cargo.toml

# Run a specific test by name
cargo test test_name

# Run ignored tests (common in Exercism TDD workflow)
cargo test -- --ignored

# Run all tests including ignored ones
cargo test -- --include-ignored
```

### Building and Linting

```bash
# Check code without building
cargo check

# Build an exercise
cargo build

# Run clippy for lints
cargo clippy

# Format code
cargo fmt
```

### Submitting Solutions

```bash
# Submit solution (from exercise directory)
exercism submit src/lib.rs

# Some exercises may require submitting Cargo.toml as well
exercism submit src/lib.rs Cargo.toml
```

## Development Workflow

**Important:** The author's full strategy is documented in the repository's [README.md](./README.md#strategy). Always consult this before assisting with exercises.

Key points:
- Follows a strict TDD approach - write just enough code for each test
- Code must be idiomatic, understandable, and maintainable (written for humans)
- After all tests pass: refactor, add comments, then submit
- Learning is a core goal: community solutions are reviewed and may inform iterations
- The author may request mentoring for specific feedback

**Do not** skip ahead or solve multiple tests at once - the incremental learning process is intentional.

## Code Style

- Prioritize idiomatic, understandable, and maintainable Rust code
- Code is written for humans to understand
- Follow Rust naming conventions and idioms
- Use appropriate trait bounds for generic implementations

## Exercise-Specific Notes

Some exercises use Cargo features (e.g., `generic` feature in triangle exercise):
```bash
cargo test --features generic
```

Check individual exercise Cargo.toml files for available features.
