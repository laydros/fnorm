# fnorm - File Name Normalizer
[![Go Reference](https://pkg.go.dev/badge/github.com/laydros/fnorm.svg)](https://pkg.go.dev/github.com/laydros/fnorm)

A simple Go tool that normalizes file names according to consistent standards.

## What it does

- Replaces spaces with hyphens
- Converts to lowercase
- Removes/replaces forbidden characters
- Cleans up multiple consecutive hyphens
- Preserves file extensions
- Applies special character replacements (`/` → `-or-`, `&` → `-and-`, `@` → `-at-`, `%` → `-percent`)
- Transliterates accented characters and typographic symbols to ASCII equivalents
- Trims leading/trailing spaces and dots

## Project Layout

- `cmd/fnorm/main.go`: command-line entry point
- `internal/normalize/normalize.go`: library package providing `Normalize`
- `internal/normalize/normalize_test.go`: table-driven tests for normalization

## Installation

```bash
git clone https://github.com/yourusername/fnorm
cd fnorm
make build
```

Optionally install to `~/bin`:

```bash
make install
```

## Usage

```bash
# Basic usage
fnorm "My Document.PDF"
# Result: my-document.pdf

# Preview changes without applying
fnorm -dry-run "File With Spaces.txt"

# Check version
fnorm -version

# Process multiple files
fnorm "File 1.jpg" "Another File.PNG" "Document (Copy).pdf"

# Use with wildcards
fnorm *.jpg
```

## Library Usage

```go
package main

import (
    "fmt"

    "github.com/laydros/fnorm/internal/normalize"
)

func main() {
    fmt.Println(normalize.Normalize("My File.PDF"))
    // Output: my-file.pdf
}
```

## Rules Applied

1. **Trim edges**: Leading/trailing spaces and dots are removed
2. **Spaces → hyphens**: `My File.txt` → `my-file.txt`
3. **Lowercase**: `Document.PDF` → `document.pdf`
4. **Special Character Replacements**: uses the following replacements
5. **Transliteration**: Accented characters and common typographic symbols become their ASCII equivalents, e.g. `café.txt` → `cafe.txt`, `rock’n’roll.txt` → `rock-n-roll.txt`
6. **Forbidden characters**: Replaced with hyphens
7. **Cleanup**: Multiple hyphens become single hyphens

| Original | Replacement | Example |
|----------|-------------|---------|
| / (slash) | -or- | `tcp-or-udp-guide.md` |
| & (ampersand) | -and- | `backup-and-restore-process.md` |
| @ (at) | -at- | `meeting-at-headquarters.md` |
| % (percent) | -percent | `cpu-usage-90-percent.txt` |

## Examples

| Original | Normalized |
|----------|------------|
| `My Document.PDF` | `my-document.pdf` |
| `File (Copy).txt` | `file-copy.txt` |
| `Report 2025-01-15.xlsx` | `report-2025-01-15.xlsx` |
| `Photo & Video.mov` | `photo-and-video.mov` |
| `Meeting @ Headquarters.md` | `meeting-at-headquarters.md` |
| `CPU Usage 90%.txt` | `cpu-usage-90-percent.txt` |
| `tcp/udp guide.md` | `tcp-or-udp-guide.md` |
| `Résumé.txt` | `resume.txt` |
| `rock’n’roll.txt` | `rock-n-roll.txt` |

## Benchmarks

```bash
$ go test -bench . ./internal/normalize
BenchmarkNormalize-5      394693              3054 ns/op
```

## Flags

- `-dry-run`: Preview changes without applying them
- `-version`: Show version information
- `-h`, `--help`: Show help message

## Building

```bash
# Local build
make build

# Cross-platform builds
make build-all

# Clean build artifacts
make clean
```

## Development

### Setup

```bash
# Install development tools
make tools

# Run code quality checks
make check  # runs fmt, vet, lint, and test
```

### Available Make Targets

```bash
make build       # Build the binary
make test        # Run tests
make tools       # Install development tools
make lint        # Run golangci-lint
make fmt         # Format code
make vet         # Run go vet
make check       # Run all quality checks
```

### Version Management

Version information is automatically injected at build time using git tags:

```bash
# Version is determined automatically from git
make build  # Uses git describe --tags or "dev" as fallback

# Check version
./fnorm -version
```

**For releases:**

1. Create a git tag: `git tag v1.2.3`
2. Build: `make build` (version will be `v1.2.3`)
3. Push tag: `git push origin v1.2.3`

**For development:**

- Builds without tags show commit hash or "dev"
- Dirty working directory adds "-dirty" suffix

This project was developed with AI assistance.

## CI

A GitHub Actions workflow in `.github/workflows/ci.yml` runs on every push and pull request. It ensures the codebase passes `go fmt`, `go vet`, `golangci-lint run`, and `go test ./...`.

## Contributing

Contributions are welcome. To keep the project consistent and easy to work with:

- Use Go 1.24.3 or later.
- Run `make check` before submitting changes (runs fmt, vet, lint, and test).
- Ensure all text files end with a trailing newline.
- Update both `README.md` and `AGENTS.md` whenever your changes affect project behavior or contributor instructions.

See `AGENTS.md` for the full set of repository guidelines.
