// Package main implements the fnorm command-line interface.
package main

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"

	fnorm "github.com/laydros/fnorm/pkg/fnorm" //nolint:depguard // internal package import
)

var (
	version     = "dev" // Fallback version, overridden by ldflags from git tags
	dryRun      = flag.Bool("dry-run", false, "Show what would be renamed without making changes")
	showVersion = flag.Bool("version", false, "Show version information")
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

	normalized := fnorm.Normalize(filename)

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
