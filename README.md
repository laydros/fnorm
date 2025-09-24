# fnorm

`fnorm` is a command-line tool and Rust library for normalizing filenames into an ASCII-only, slug-style format while preserving their extensions. It is a work-in-progress port of the original Go implementation and ships with a comprehensive functional specification to guide feature completion.

## Table of Contents
- [Project Status](#project-status)
- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
  - [Clone and build](#clone-and-build)
  - [Run the CLI](#run-the-cli)
  - [Run the test suite](#run-the-test-suite)
- [Normalization Rules](#normalization-rules)
- [Repository Layout](#repository-layout)
- [Development Workflow](#development-workflow)
  - [Implementing the remaining features](#implementing-the-remaining-features)
  - [Coding standards](#coding-standards)
- [Additional Documentation](#additional-documentation)
- [License](#license)

## Project Status
The Rust port currently includes:

- A basic CLI with `--dry-run` support that iterates over paths and performs filesystem-safe renames.
- A normalization library with unit tests covering the cases documented in the functional specification.
- Error types that provide human-readable diagnostics for common failure scenarios.

Refer to [`IMPLEMENTATION.md`](IMPLEMENTATION.md) for a detailed checklist of remaining work and historical context for the port.

## Prerequisites

- **Rust toolchain** (stable, 1.75 or newer recommended)
  - Install via [rustup](https://rustup.rs/) if you do not already have Rust and Cargo.
- **Cargo** (bundled with rustup) for building, testing, and running the project.

## Getting Started

### Clone and build

```bash
# Clone the repository
git clone <your-fork-url>
cd fnorm

# Build in debug mode
cargo build

# Build an optimized binary
cargo build --release
```

Both commands produce an executable at `target/debug/fnorm` (or `target/release/fnorm`).

### Run the CLI

`fnorm` accepts one or more file paths. By default it renames files in place; pass `--dry-run` to preview the changes without touching the filesystem.

If you forget to supply a path, Clap reports the error and displays usage information for you.

```bash
# Show command help
cargo run -- --help

# Normalize two files in place
cargo run -- path/to/file.txt another/Example.PDF

# Preview changes without renaming
cargo run -- --dry-run "My Document.PDF"
```

The CLI prints status messages such as:

- `✓ <name> (no changes needed)` when a filename is already normalized.
- `Renamed: <old> -> <new>` for successful renames.
- `Would rename: <old> -> <new>` in dry-run mode.

Errors (missing files, directories, collisions, rename failures) are reported to standard error and cause a non-zero exit status if any occur.

### Run the test suite

Unit tests live alongside the normalization logic. Run them with:

```bash
cargo test
```

Use `cargo test -- --nocapture` to stream any diagnostic output that tests emit.

## Normalization Rules

The normalization pipeline follows the twelve-step algorithm described in detail in [`functional-spec.md`](functional-spec.md). Highlights include:

1. Trim surrounding whitespace and leading/trailing dots from the base name.
2. Replace spaces with hyphens and convert to lowercase.
3. Substitute special tokens: `/` → `-or-`, `&` → `-and-`, `@` → `-at-`, `%` → `-percent-`.
4. Transliterate select accented characters (e.g., `é` → `e`, `ß` → `ss`).
5. Replace any remaining unsupported characters with hyphens and collapse hyphen runs.
6. Lowercase the file extension before reassembling the final name.

The specification also documents known edge cases, such as hidden files whose entire name is treated as an extension (`.Hidden File` → `.hidden file`).

## Repository Layout

```
├── Cargo.toml          # crate metadata and dependencies
├── src/
│   ├── main.rs         # CLI entry point and file processing workflow
│   ├── normalize.rs    # normalization algorithm and tests
│   └── error.rs        # custom error types for user-facing diagnostics
├── functional-spec.md  # authoritative behavior specification
├── IMPLEMENTATION.md   # development log and TODO checklist
└── LICENSE             # BSD 3-Clause license
```

## Development Workflow

### Implementing the remaining features

1. Pick the next unchecked item in [`IMPLEMENTATION.md`](IMPLEMENTATION.md).
2. Update the code under `src/` and extend the tests in `src/normalize.rs` as needed.
3. Run `cargo fmt`, `cargo clippy`, and `cargo test` before committing to keep the codebase tidy and verified.
4. Commit logically grouped changes with descriptive messages.

The current focus areas include finalizing normalization edge cases, expanding test coverage, and completing filesystem behaviors such as collision handling and case-only renames (already implemented in Rust but slated for further verification).

### Coding standards

- Use idiomatic Rust with clear error messages surfaced via `FnormError`.
- Favor small, testable functions and keep normalization logic pure.
- Ensure new behavior is exercised by unit tests or integration tests.

## Additional Documentation

- [`functional-spec.md`](functional-spec.md) — Required reading for any feature work; defines the CLI contract and library semantics.
- [`IMPLEMENTATION.md`](IMPLEMENTATION.md) — Tracks porting progress and outstanding tasks.

## License

This project is distributed under the terms of the [BSD 3-Clause License](LICENSE).
