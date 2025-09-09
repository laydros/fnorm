// Package normalize provides filename normalization utilities.
package normalize

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
//
// Example:
//
//	normalized := Normalize("My File.PDF")
//	// normalized == "my-file.pdf"
func Normalize(filename string) string {
	if filename == "" {
		return ""
	}

	// Get file extension and base name
	ext := filepath.Ext(filename)
	nameOnly := strings.TrimSuffix(filename, ext)
	if ext == "." {
		ext = ""
	}

	// Trim unwanted characters from the base name
	nameOnly = strings.TrimSpace(nameOnly)
	nameOnly = strings.Trim(nameOnly, ".")

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

	// 4. Transliterate accented characters to ASCII
	result = transliterate(result)

	// 5. Replace forbidden characters with hyphens
	result = forbiddenCharsRe.ReplaceAllString(result, "-")

	// 6. Clean up multiple consecutive hyphens
	result = multiHyphenRe.ReplaceAllString(result, "-")

	// 7. Trim leading hyphens
	result = strings.TrimLeft(result, "-")

	// Convert extension to lowercase too
	ext = strings.ToLower(ext)

	return result + ext
}

var transliterations = map[rune]string{
	'á': "a", 'à': "a", 'â': "a", 'ä': "a", 'ã': "a", 'å': "a",
	'é': "e", 'è': "e", 'ê': "e", 'ë': "e",
	'í': "i", 'ì': "i", 'î': "i", 'ï': "i",
	'ó': "o", 'ò': "o", 'ô': "o", 'ö': "o", 'õ': "o",
	'ú': "u", 'ù': "u", 'û': "u", 'ü': "u",
	'ñ': "n",
	'ç': "c",
	'æ': "ae", 'œ': "oe",
	'ø': "o", 'ß': "ss",
	// Typography
	'–': "-", '—': "-", // en/em dashes
	'‘': "'", '’': "'", // smart single quotes
	'“': "\"", '”': "\"", // smart double quotes
}

func transliterate(s string) string {
	var b strings.Builder
	for _, r := range s {
		if repl, ok := transliterations[r]; ok {
			b.WriteString(repl)
		} else {
			b.WriteRune(r)
		}
	}
	return b.String()
}
