// Package fnorm provides filename normalization utilities.
package fnorm

import (
	"path/filepath"
	"regexp"
	"strings"
)

const (
	spaceReplacer         = "-"
	forbiddenCharsPattern = `[^a-z0-9\-_.]`
)

var (
	forbiddenCharsRe    = regexp.MustCompile(forbiddenCharsPattern)
	multiHyphenRe       = regexp.MustCompile(`-+`)
	specialReplacements = map[string]string{
		"/": "-or-",
		"&": "-and-",
		"@": "-at-",
		"%": "-percent",
	}
)

// Normalize transforms a filename according to the normalization rules:
// spaces to hyphens, lowercase conversion, forbidden character replacement, etc.
func Normalize(filename string) string {
	// Get file extension
	ext := filepath.Ext(filename)
	nameOnly := strings.TrimSuffix(filename, ext)

	// Apply transformations to name only
	result := nameOnly

	// 1. Replace spaces with hyphens
	result = strings.ReplaceAll(result, " ", spaceReplacer)

	// 2. Convert to lowercase
	result = strings.ToLower(result)

	// 3. Apply special character replacements
	for orig, repl := range specialReplacements {
		result = strings.ReplaceAll(result, orig, repl)
	}

	// 4. Replace forbidden characters with hyphens
	// Keep only: letters, numbers, hyphens, underscores, periods
	result = forbiddenCharsRe.ReplaceAllString(result, "-")

	// 5. Clean up multiple consecutive hyphens
	result = multiHyphenRe.ReplaceAllString(result, "-")

	// 6. Trim leading/trailing hyphens
	result = strings.Trim(result, "-")

	// Convert extension to lowercase too
	ext = strings.ToLower(ext)

	return result + ext
}
