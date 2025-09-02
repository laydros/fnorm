# fnorm - File Name Normalizer

A simple Go tool that normalizes file names according to consistent standards.

## What it does

- Replaces spaces with hyphens
- Converts to lowercase
- Removes/replaces forbidden characters
- Cleans up multiple consecutive hyphens
- Preserves file extensions

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

# Process multiple files
fnorm "File 1.jpg" "Another File.PNG" "Document (Copy).pdf"

# Use with wildcards
fnorm *.jpg
```

## Rules Applied

1. **Spaces → hyphens**: `My File.txt` → `my-file.txt`
2. **Lowercase**: `Document.PDF` → `document.pdf`
3. **Forbidden characters**: Replaced with hyphens
4. **Allowed characters**: Letters, numbers, hyphens (-), underscores (_), periods (.)
5. **Cleanup**: Multiple hyphens become single hyphens

## Examples

| Original | Normalized |
|----------|------------|
| `My Document.PDF` | `my-document.pdf` |
| `File (Copy).txt` | `file-copy.txt` |
| `Report 2025-01-15.xlsx` | `report-2025-01-15.xlsx` |
| `Photo & Video.mov` | `photo-video.mov` |

## Flags

- `-dry-run`: Preview changes without applying them
- `-help`: Show help message

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

This project was developed with AI assistance.