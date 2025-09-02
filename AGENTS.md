# CLAUDE.md

This file provides guidance to AI assistants when working with code in this repository.

## Build and Development Commands

```bash
# Build the binary
make build

# Run tests
make test
go test -v

# Run a specific test
go test -v -run TestNormalizeFilename

# Format and lint code (run before committing)
go fmt ./...
go vet ./...

# Build for all platforms
make build-all

# Install to ~/bin
make install

# Clean build artifacts
make clean

# Run the tool directly without building
go run . [flags] file1 [file2 ...]
```

## Architecture

This is a simple Go CLI tool with a single main package containing:

- **main.go**: Entry point with CLI flag parsing, file processing logic, and the core `normalizeFilename()` function
- **normalize_test.go**: Table-driven tests for the normalization logic

The normalization pipeline in `normalizeFilename()`:

1. Separates extension from filename
2. Replaces spaces with hyphens
3. Converts to lowercase
4. Replaces forbidden characters (anything not alphanumeric, hyphen, underscore, or period) with hyphens
5. Collapses multiple consecutive hyphens
6. Trims leading/trailing hyphens
7. Lowercases the extension

## Key Implementation Details

- Uses Go's `flag` package for CLI argument parsing with built-in `-h`/`--help` support
- The `-dry-run` flag allows previewing changes without applying them
- File operations check for existing targets to prevent accidental overwrites
- Error handling reports issues per-file without stopping batch operations
- Regular expressions handle character filtering and hyphen cleanup

## Testing Approach

Tests use table-driven testing pattern with test cases covering:

- Space replacement
- Case conversion
- Forbidden character handling
- Multiple hyphen collapse
- Extension case normalization

## Project Standards

- Go 1.25 or later required
- All text files must end with a single trailing newline
- Run `go fmt`, `go vet`, and `go test` before committing changes
- **Update both README.md and AGENTS.md when making changes that affect project behavior, usage, or contributor guidelines**
