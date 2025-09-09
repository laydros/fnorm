# AGENTS.md

This file provides guidance to AI assistants when working with code in this repository.

## Build and Development Commands

```bash
# Build the binary (with version injection from git tags)
make build

# Install development tools (run once)
make tools

# Run all quality checks (format, vet, lint, test)
make check

# Individual checks
make test        # Run unit tests
make coverage    # Run tests with coverage profile (library: 96%, CLI: 82.5%)
make lint        # Run golangci-lint
make fmt         # Format code
make vet         # Run go vet -all

# Run tests
go test -v
go test -v -run TestNormalize  # Run specific test

# Integration tests (end-to-end CLI testing)
go test -v -tags integration ./cmd/fnorm

# Build for all platforms
make build-all

# Install to ~/bin
make install

# Clean build artifacts
make clean

# Run the tool directly without building
go run ./cmd/fnorm [flags] file1 [file2 ...]

# Check version information
./fnorm -version
```

## Architecture

This project follows idiomatic Go structure with library-first design:

- **normalize.go**: Library package (package fnorm) exporting the `Normalize` function - can be imported by other Go projects
- **cmd/fnorm/main.go**: CLI entry point with flag parsing and file processing logic
- **cmd/fnorm/main_test.go**: Unit tests for CLI functions and logic (82.5% coverage)
- **cmd/fnorm/integration_test.go**: End-to-end integration tests using compiled binary
- **normalize_test.go**: Table-driven tests for the normalization logic (96% coverage)
- **example_test.go**: Example usage tests for documentation
- **normalize_bench_test.go**: Performance benchmarks
- **tools.go**: Development tool dependencies (build tag: tools)
- **.golangci.yml**: Linter configuration

Library import path: `github.com/laydros/fnorm`

The normalization pipeline in `Normalize()`:

1. Separates extension from filename and trims leading/trailing spaces and dots
2. Replaces spaces with hyphens
3. Converts to lowercase
4. Applies special character replacements (`/` → `-or-`, `&` → `-and-`, `@` → `-at-`, `%` → `-percent`)
5. Transliterates accented characters and common typographic symbols (en/em dashes, smart quotes) to their ASCII equivalents
6. Replaces forbidden characters (anything not alphanumeric, hyphen, underscore, or period) with hyphens
7. Collapses multiple consecutive hyphens
8. Trims leading hyphens
9. Lowercases the extension

## Key Implementation Details

- Uses Go's `flag` package for CLI argument parsing with built-in `-h`/`--help` support
- The `-dry-run` flag allows previewing changes without applying them
- The `-version` flag shows version information (injected at build time via ldflags)
- File operations check for existing targets to prevent accidental overwrites
- Error handling reports issues per-file without stopping batch operations
- **Exit codes**: Exits 0 for success, 1 for any file processing failures (proper Unix CLI behavior)
- Regular expressions handle character filtering and hyphen cleanup
- Explicit mappings handle special character replacements (`/` → `-or-`, `&` → `-and-`, `@` → `-at-`, `%` → `-percent`)
- Version management uses git tags with ldflags injection for builds

## Testing Approach

The project includes comprehensive testing at multiple levels:

### Unit Tests
- **Library tests** (`normalize_test.go`): Table-driven tests for the normalization logic with 96% coverage
- **CLI tests** (`cmd/fnorm/main_test.go`): Unit tests for CLI functions with 82.5% coverage
- **Example tests** (`example_test.go`): Documentation examples that verify API usage

### Integration Tests
- **End-to-end tests** (`cmd/fnorm/integration_test.go`): Complete workflow testing using the compiled binary
- Tests actual file operations, error handling, exit codes, and CLI behavior
- Uses `os/exec` to build and run the actual binary as subprocess for true end-to-end testing
- Run with: `go test -v -tags integration ./cmd/fnorm`

### Test Coverage
- Space replacement, case conversion, forbidden character handling
- Multiple hyphen collapse, extension case normalization
- Special character replacements (`/` → `-or-`, `&` → `-and-`, etc.)
- Edge cases (empty strings, files without extensions, multiple dots, unicode, leading/trailing spaces)
- CLI error scenarios, exit codes, dry-run mode, version display
- File system operations and conflict handling

### Known Test Limitations
- Hidden files with spaces currently normalize incorrectly (`.Hidden File` → `.hidden file` instead of `.hidden-file`)
- Case-insensitive filesystem testing challenges on macOS/Windows for filename case changes

## Development Setup

New contributors should run:

```bash
make tools   # Install development tools like golangci-lint
make check   # Verify everything works
```

## Project Standards

- Go 1.24 or later required
- All text files must end with a single trailing newline
- Run `make check` before committing changes (includes fmt, vet, lint, and test)
- **Update both README.md and AGENTS.md when making changes that affect project behavior, usage, or contributor guidelines**

## CI

A GitHub Actions workflow (`.github/workflows/ci.yml`) runs `go fmt`, `go vet -all`, `golangci-lint run`, and `make coverage` on every push and pull request, uploading `coverage.out` as an artifact.
