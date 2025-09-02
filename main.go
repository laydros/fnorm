// Package main provides fnorm, a file name normalizer that converts
// file names to lowercase with consistent formatting rules.
package main

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

const (
	version               = "dev" // Fallback version, overridden by ldflags from git tags
	spaceReplacer         = "-"
	forbiddenCharsPattern = `[^a-z0-9\-_.]`
)

var (
	dryRun           = flag.Bool("dry-run", false, "Show what would be renamed without making changes")
	showVersion      = flag.Bool("version", false, "Show version information")
	forbiddenCharsRe = regexp.MustCompile(forbiddenCharsPattern)
	multiHyphenRe    = regexp.MustCompile(`-+`)
)

func main() {
	flag.Usage = showHelp
	flag.Parse()

	if *showVersion {
		fmt.Printf("fnorm version %s\n", version)
		return
	}

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

// showHelp displays usage information for the fnorm command.
func showHelp() {
	fmt.Printf(`fnorm - File name normalizer

Usage: fnorm [flags] file1 [file2 ...]

Normalizes file names according to standards:
  - Spaces become hyphens
  - Converted to lowercase
  - Only letters, numbers, hyphens, underscores, periods allowed

Flags:
  -dry-run    Show what would be renamed without making changes
  -version    Show version information
  -h, --help  Show this help message

Examples:
  fnorm "My Document.PDF"              # -> my-document.pdf
  fnorm -dry-run "File With Spaces.txt"  # Shows preview
  fnorm *.jpg                          # Normalize all JPG files
`)
}

// processFile handles the renaming of a single file, checking for errors
// and respecting the dry-run flag.
func processFile(filePath string) error {
	info, err := os.Stat(filePath)
	if err != nil {
		return fmt.Errorf("file does not exist")
	}
	if info.IsDir() {
		return fmt.Errorf("skipping directory: %s", filePath)
	}

	// Split path into directory and filename
	dir := filepath.Dir(filePath)
	filename := filepath.Base(filePath)

	normalized := normalizeFilename(filename)

	// If no change is needed
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

// normalizeFilename transforms a filename according to the normalization rules:
// spaces to hyphens, lowercase conversion, forbidden character replacement, etc.
func normalizeFilename(filename string) string {

	// Get file extension
	ext := filepath.Ext(filename)
	nameOnly := strings.TrimSuffix(filename, ext)

	// Apply transformations to name only
	result := nameOnly

	// 1. Replace spaces with hyphens
	result = strings.ReplaceAll(result, " ", spaceReplacer)

	// 2. Convert to lowercase
	result = strings.ToLower(result)

	// 3. Replace forbidden characters with hyphens
	// Keep only: letters, numbers, hyphens, underscores, periods
	result = forbiddenCharsRe.ReplaceAllString(result, "-")

	// 4. Clean up multiple consecutive hyphens
	result = multiHyphenRe.ReplaceAllString(result, "-")

	// 5. Trim leading/trailing hyphens
	result = strings.Trim(result, "-")

	// Convert extension to lowercase too
	ext = strings.ToLower(ext)

	return result + ext
}
