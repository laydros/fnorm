# AGENTS.md

This file provides guidance to AI coding agents (like Claude Code, GitHub Copilot, etc.) when working with the fnorm codebase.

## Project Overview

**fnorm** is a filename normalization utility written in Rust that converts filenames and directory names to ASCII-only slug format while preserving file extensions and directory structure.

**Current Version:** 0.2.0
**Status:** Feature-complete Rust port from Go

## Quick Start

### Building and Running
```bash
# Build the project
cargo build

# Run the CLI
cargo run -- [files...]

# Run with dry-run flag
cargo run -- --dry-run [files...]
```

### Testing
```bash
# Run all tests (26 total: 11 unit + 15 integration)
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_directory_basic_rename
```

### Code Quality
```bash
# Format, lint, and test
cargo fmt && cargo clippy && cargo test
```

## Architecture

### Module Structure
- **src/main.rs** - CLI entry point (thin wrapper)
- **src/lib.rs** - Library interface, file processing logic, and public API
- **src/normalize.rs** - Core normalization algorithm with unit tests
- **src/error.rs** - Custom error types (FnormError, RunError)
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

3. **Error Handling** (`src/error.rs`):
   - `FnormError`: File not found, target exists, rename failures
   - `RunError`: Aggregates multiple errors for batch processing
   - Human-readable error messages

### Testing Strategy

**Unit Tests** (11 tests in `src/normalize.rs`):
- Basic normalization cases
- Extension handling
- Special character substitution
- Unicode transliteration
- Edge cases (hidden files, empty strings, etc.)

**Integration Tests** (15 tests in `tests/integration_tests.rs`):
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
- **Documentation**: Keep README.md, CLAUDE.md, and this file up to date
- **Code style**: Follow idiomatic Rust patterns, keep functions small and testable

### Common Tasks

**Adding a new transliteration rule:**
1. Update the match statement in `normalize_base()` in `src/normalize.rs`
2. Add test case to `test_transliteration()`

**Adding a new special token:**
1. Add match arm in `normalize_base()` in `src/normalize.rs`
2. Add test case to `test_special_tokens()`

**Adding CLI flags:**
1. Update `Cli` struct in `src/lib.rs`
2. Update flag handling in `run()` or `process_file()`
3. Update README.md with usage examples

## Reference Documentation

- **functional-spec.md** - Complete functional specification (authoritative source for behavior)
- **IMPLEMENTATION.md** - Development log and TODO tracking
- **README.md** - User-facing documentation
- **CLAUDE.md** - Claude Code specific guidance

## Dependencies

- **clap 4.4** (with derive feature) - CLI argument parsing
- **tempfile 3.8** (dev-only) - Temporary directories for integration tests

## Known Limitations

1. Hidden files (starting with `.`) treat entire name as extension: `.Hidden File` → `.hidden file`
2. No recursive directory processing (by design - operates on specified paths only)
3. Case-only renames use two-step process on case-insensitive filesystems

## Future Enhancements

Potential areas for extension (not currently planned):
- Recursive directory traversal with `-r` flag
- Configuration file for custom transliteration rules
- Undo/rollback functionality
- Batch rename with pattern matching
- Plugin system for custom normalization rules
