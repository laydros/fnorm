# AGENTS.md

This file provides guidance to AI coding agents (like Claude Code, GitHub Copilot, etc.) when working with the fnorm codebase.

## Project Overview

**fnorm** is a filename normalization utility written in Rust that converts filenames and directory names to ASCII-only slug format while preserving file extensions and directory structure.

## Quick Start

### Building and Running
```bash
# Build the project
cargo build

# Build with optimizations
cargo build --release

# Run the CLI
cargo run -- [files...]

# Run with dry-run flag
cargo run -- --dry-run [files...]
```

### Testing
```bash
# Run all tests (unit + integration)
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_directory_basic_rename
```

### Code Quality
```bash
# Check code without building
cargo check

# Format, lint, and test
cargo fmt && cargo clippy && cargo test
```

## Architecture

### Module Structure
- **src/main.rs** - CLI entry point (thin wrapper)
- **src/lib.rs** - Library interface, file processing logic, and public API
- **src/normalize.rs** - Core normalization algorithm with unit tests
- **src/error.rs** - Custom error types (FnormError, ConfigError)
- **tests/integration_tests.rs** - Integration tests for file/directory operations

### Key Features

1. **Normalization Algorithm** (`src/normalize.rs`):
   - 12-step process defined in `functional-spec.md`
   - Handles extension detection, whitespace, case conversion, transliteration
   - Special character substitution: `/` → `-or-`, `&` → `-and-`, `@` → `-at-`, `%` → `-percent-`
   - Comprehensive character transliteration (é→e, ß→ss, etc.)

2. **File Operations** (`src/lib.rs`):
   - Processes both files and directories
   - Case-insensitive filesystem support via two-step rename
   - Collision detection (target exists)
   - Dry-run mode for preview

3. **Error Handling** (`src/error.rs`, `src/lib.rs`):
   - `FnormError`: Missing or unreadable path, target exists, rename failures
   - `ConfigError`: Config file read, parse and invalid-key failures
   - `RunError` (in `src/lib.rs`): Aggregates per-path errors into one summary printed at the end
   - Human-readable error messages

### Testing Strategy

**Unit Tests** (in `src/normalize.rs`):
- Basic normalization cases
- Extension handling
- Special character substitution
- Unicode transliteration
- Edge cases (hidden files, empty strings, etc.)

**Integration Tests** (in `tests/integration_tests.rs`):
- File rename operations
- Directory rename operations
- Case-only renames
- Error conditions (target exists, file not found)
- Dry-run mode
- Directory contents preservation

## Development Guidelines

### When Making Changes

1. **Algorithm changes**: Update `src/normalize.rs` and add unit tests
2. **File operations**: Update `src/lib.rs` and add integration tests
3. **CLI changes**: Update `src/main.rs` and relevant documentation
4. **Always run**: `cargo fmt && cargo clippy && cargo test` before committing

### Important Conventions

- **Error handling**: Use `FnormError` for specific errors, `RunError` for aggregation
- **Testing**: Ensure all new functionality has corresponding tests
- **Documentation**: Keep README.md and this file up to date
- **Code style**: Follow idiomatic Rust patterns, keep functions small and testable

### Common Tasks

**Adding a new transliteration rule:**
1. Add an entry to the `transliterations` map in `NormalizationConfig::default()` in `src/normalize.rs`
2. Add test case to `test_transliteration()`

**Adding a new special token:**
1. Add an entry to the `special_tokens` map in `NormalizationConfig::default()` in `src/normalize.rs`
2. Add test case to `test_special_tokens()`

**Adding CLI flags:**
1. Update `Cli` struct in `src/lib.rs`
2. Update flag handling in `run()` or `process_file()`
3. Update README.md with usage examples

## Reference Documentation

- **functional-spec.md** - Complete functional specification (authoritative source for behavior)
- **README.md** - User-facing documentation

## Dependencies

- **clap 4.4** (with derive feature) - CLI argument parsing
- **serde 1** (with derive feature) and **toml 0.8** - Reading the `--config` file
- **tempfile 3.8** (dev-only) - Temporary directories for integration tests

## Known Limitations

1. No recursive directory processing; fnorm operates on the specified paths only
2. Case-only renames use two-step process on case-insensitive filesystems

## Future Enhancements

Planned enhancements are tracked as GitHub issues.
