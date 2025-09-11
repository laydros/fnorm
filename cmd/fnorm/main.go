// Package main implements the fnorm command-line interface.
package main

import (
	"errors"
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/laydros/fnorm" //nolint:depguard // allowed internal module import
)

var (
	version     = "dev" // Fallback version, overridden by ldflags from git tags
	dryRun      = flag.Bool("dry-run", false, "Show what would be renamed without making changes")
	showVersion = flag.Bool("version", false, "Show version information")
	errIsDir    = errors.New("is a directory")
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

	// Track whether any operations failed
	var hasErrors bool
	for _, arg := range args {
		if err := processFile(arg); err != nil {
			fmt.Fprintf(os.Stderr, "Error processing %s: %v\n", arg, err)
			hasErrors = true
		}
	}

	// Exit with appropriate code
	if hasErrors {
		os.Exit(1)
	}
}

// showHelp displays usage information for the fnorm command.
//
// Example:
//
//	flag.Usage = showHelp
//	showHelp()
func showHelp() {
	fmt.Printf(`fnorm - File name normalizer

Usage: fnorm [flags] file1 [file2 ...]

Normalizes file names to safe, consistent format:
  - Spaces become hyphens
  - Converted to lowercase
  - Trims leading/trailing spaces and dots
  - Special characters: / → -or-, & → -and-, @ → -at-, %% → -percent
  - Accented letters and typographic symbols simplified to ASCII
  - Forbidden characters replaced with hyphens
  - Multiple consecutive hyphens collapsed to single hyphens
  - Leading hyphens trimmed

Flags:
  -dry-run    Show what would be renamed without making changes
  -version    Show version information
  -h, --help  Show this help message

Examples:
  fnorm "My Document.PDF"              # -> my-document.pdf
  fnorm "Photo & Video.mov"            # -> photo-and-video.mov
  fnorm "Meeting @ HQ.md"              # -> meeting-at-hq.md
  fnorm "tcp/udp guide.txt"            # -> tcp-or-udp-guide.txt
  fnorm "CPU Usage 90%%.log"            # -> cpu-usage-90-percent.log
  fnorm -dry-run "File With Spaces.txt"  # Shows preview without changes
  fnorm *.jpg                          # Normalize all JPG files
`)
}

// processFile handles the renaming of a single file, checking for errors
// and respecting the dry-run flag.
//
// Example:
//
//	if err := processFile("My File.txt"); err != nil {
//	        log.Fatal(err)
//	}
func processFile(filePath string) error {
	info, err := os.Stat(filePath)
	if err != nil {
		return fmt.Errorf("stat %s: %w", filePath, err)
	}
	if info.IsDir() {
		return fmt.Errorf("skipping directory %s: %w", filePath, errIsDir)
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

	// Check if this is a case-only change
	if isCaseOnlyChange(filePath, newPath) {
		// Use two-step rename for case-only changes to work on case-insensitive filesystems
		if err := performCaseOnlyRename(filePath, newPath); err != nil {
			return err
		}
	} else {
		// Check if target exists (but only for non-case-only changes)
		if _, err := os.Stat(newPath); err == nil {
			return fmt.Errorf("target file already exists %q: %w", normalized, os.ErrExist)
		}

		if err := os.Rename(filePath, newPath); err != nil {
			return fmt.Errorf("failed to rename %q to %q: %w", filePath, newPath, err)
		}
	}

	fmt.Printf("Renamed: %s -> %s\n", filename, normalized)
	return nil
}

// isCaseOnlyChange returns true if the old and new paths differ only in case.
// This helps detect when a rename is just changing case on case-insensitive filesystems.
func isCaseOnlyChange(oldPath, newPath string) bool {
	return strings.EqualFold(oldPath, newPath) && oldPath != newPath
}

// generateTempName creates a temporary filename by appending a suffix to avoid conflicts.
func generateTempName(originalPath string) string {
	return originalPath + ".fnorm-tmp"
}

// performCaseOnlyRename handles renaming files when only the case changes.
// This uses a two-step process to work around case-insensitive filesystem limitations.
func performCaseOnlyRename(oldPath, newPath string) error {
	tempPath := generateTempName(oldPath)

	// Step 1: Rename to temporary file
	if err := os.Rename(oldPath, tempPath); err != nil {
		return fmt.Errorf("failed to rename to temporary file %q: %w", tempPath, err)
	}

	// Step 2: Rename from temporary to final name
	if err := os.Rename(tempPath, newPath); err != nil {
		// Try to restore original file if second step fails
		if restoreErr := os.Rename(tempPath, oldPath); restoreErr != nil {
			return fmt.Errorf("failed to rename to %q and failed to restore original: %v (restore error: %v)", newPath, err, restoreErr)
		}
		return fmt.Errorf("failed to rename %q to %q: %w", tempPath, newPath, err)
	}

	return nil
}
