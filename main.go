package main

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

var (
	dryRun = flag.Bool("dry-run", false, "Show what would be renamed without making changes")
)

func main() {
	flag.Usage = showHelp
	flag.Parse()

	args := flag.Args()
	if len(args) == 0 {
		fmt.Fprintf(os.Stderr, "Error: No files specified\n")
		fmt.Fprintf(os.Stderr, "Use -h or --help for usage information\n")
		os.Exit(1)
	}

	for _, arg := range args {
		if err := processFile(arg); err != nil {
			fmt.Fprintf(os.Stderr, "Error processing %s: %v\n", arg, err)
		}
	}
}

func showHelp() {
	fmt.Printf(`fnorm - File name normalizer

Usage: fnorm [flags] file1 [file2 ...]

Normalizes file names according to standards:
  - Spaces become hyphens
  - Converted to lowercase
  - Only letters, numbers, hyphens, underscores, periods allowed

Flags:
  -dry-run    Show what would be renamed without making changes
  -h, --help  Show this help message

Examples:
  fnorm "My Document.PDF"              # -> my-document.pdf
  fnorm -dry-run "File With Spaces.txt"  # Shows preview
  fnorm *.jpg                          # Normalize all JPG files
`)
}

func processFile(filePath string) error {
	dir := filepath.Dir(filePath)
	filename := filepath.Base(filePath)

	normalized := normalizeFilename(filename)

	if filename == normalized {
		if !*dryRun {
			fmt.Printf("✓ %s (no changes needed)\n", filename)
		}
		return nil
	}

	newPath := filepath.Join(dir, normalized)

	if *dryRun {
		fmt.Printf("Would rename: %s -> %s\n", filename, normalized)
		return nil
	}

	// Check if target exists
	if _, err := os.Stat(newPath); err == nil {
		return fmt.Errorf("target file already exists: %s", normalized)
	}

	if err := os.Rename(filePath, newPath); err != nil {
		return fmt.Errorf("failed to rename: %v", err)
	}

	fmt.Printf("Renamed: %s -> %s\n", filename, normalized)
	return nil
}

func normalizeFilename(filename string) string {
	// Get file extension
	ext := filepath.Ext(filename)
	nameOnly := strings.TrimSuffix(filename, ext)

	// Apply transformations to name only
	result := nameOnly

	// 1. Replace spaces with hyphens
	result = strings.ReplaceAll(result, " ", "-")

	// 2. Convert to lowercase
	result = strings.ToLower(result)

	// 3. Replace forbidden characters with hyphens
	// Keep only: letters, numbers, hyphens, underscores, periods
	reg := regexp.MustCompile(`[^a-z0-9\-_.]`)
	result = reg.ReplaceAllString(result, "-")

	// 4. Clean up multiple consecutive hyphens
	reg = regexp.MustCompile(`-+`)
	result = reg.ReplaceAllString(result, "-")

	// 5. Trim leading/trailing hyphens
	result = strings.Trim(result, "-")

	// Convert extension to lowercase too
	ext = strings.ToLower(ext)

	return result + ext
}
