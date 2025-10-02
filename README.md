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

The Rust port is feature-complete and includes:

- A CLI with `--dry-run` and `--version` flags that processes both files and directories.
- Full support for directory renaming (added in v0.2.0).
- A normalization library with comprehensive unit tests covering all cases from the functional specification.
- Integration tests (15 tests) verifying file/directory rename operations, error handling, and dry-run mode.
- Case-insensitive filesystem support with two-step rename logic for case-only changes.
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

# Install to $CARGO_HOME/bin for local use
cargo install --path .
```

Both build commands produce an executable at `target/debug/fnorm` (or `target/release/fnorm`).

### Run the CLI

`fnorm` accepts one or more file or directory paths. By default it renames paths in place; pass `--dry-run` to preview the changes without touching the filesystem.

If you forget to supply a path, Clap reports the error and displays usage information for you.

```bash
# Show command help
cargo run -- --help

# Show version
cargo run -- --version

# Normalize files in place
cargo run -- path/to/file.txt another/Example.PDF

# Normalize a directory
cargo run -- "My Directory"

# Preview changes without renaming
cargo run -- --dry-run "My Document.PDF"
```

The CLI prints status messages such as:

- `✓ <name> (no changes needed)` when a filename/directory is already normalized.
- `Renamed: <old> -> <new>` for successful renames.
- `Would rename: <old> -> <new>` in dry-run mode.

Errors (missing paths, collisions, rename failures) are reported to standard error and cause a non-zero exit status if any occur.

### Run the test suite

The project includes both unit tests and integration tests:

- **Unit tests** (11 tests) in `src/normalize.rs` cover the normalization algorithm
- **Integration tests** (15 tests) in `tests/integration_tests.rs` verify file/directory operations

Run all tests with:

```bash
cargo test
```

If you need to execute the suite in an offline environment (such as Codex), run
`./codex_setup.sh` once while you still have network access. The script vendors
all dependencies into `./vendor` and writes a `.cargo/config.toml` that points
Cargo at those local copies so `cargo test --offline` can succeed without
contacting crates.io.

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
├── Cargo.toml                   # crate metadata and dependencies
├── src/
│   ├── main.rs                  # CLI entry point
│   ├── lib.rs                   # library interface and file processing logic
│   ├── normalize.rs             # normalization algorithm and unit tests
│   └── error.rs                 # custom error types for user-facing diagnostics
├── tests/
│   └── integration_tests.rs     # integration tests for rename operations
├── functional-spec.md           # authoritative behavior specification
├── IMPLEMENTATION.md            # development log and TODO checklist
└── LICENSE                      # BSD 3-Clause license
```

## Development Workflow

### Implementing new features

1. Update the code under `src/` and extend tests as needed.
2. Add unit tests to `src/normalize.rs` for normalization logic changes.
3. Add integration tests to `tests/integration_tests.rs` for file/directory operation changes.
4. Run `cargo fmt`, `cargo clippy`, and `cargo test` before committing to keep the codebase tidy and verified.
5. Commit logically grouped changes with descriptive messages.

The core functionality is complete. Future enhancements might include recursive directory processing, configuration files, or additional normalization rules.

### Coding standards

- Use idiomatic Rust with clear error messages surfaced via `FnormError`.
- Favor small, testable functions and keep normalization logic pure.
- Ensure new behavior is exercised by unit tests or integration tests.
- The project is structured as both a library (`src/lib.rs`) and binary (`src/main.rs`) to facilitate testing.

## Additional Documentation

- [`functional-spec.md`](functional-spec.md) — Required reading for any feature work; defines the CLI contract and library semantics.
- [`IMPLEMENTATION.md`](IMPLEMENTATION.md) — Tracks porting progress and outstanding tasks.

## License

This project is distributed under the terms of the [BSD 3-Clause License](LICENSE).
