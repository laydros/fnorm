package main

import "testing"

func TestNormalizeFilename(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "spaces replaced with hyphens",
			input:    "My File.txt",
			expected: "my-file.txt",
		},
		{
			name:     "case converted to lowercase",
			input:    "HELLO.txt",
			expected: "hello.txt",
		},
		{
			name:     "forbidden characters replaced",
			input:    "file@name!.txt",
			expected: "file-name.txt",
		},
		{
			name:     "multiple hyphens collapsed",
			input:    "file--name---test.txt",
			expected: "file-name-test.txt",
		},
		{
			name:     "extension lowercased",
			input:    "report.PDF",
			expected: "report.pdf",
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			got := normalizeFilename(tc.input)
			if got != tc.expected {
				t.Fatalf("expected %q, got %q", tc.expected, got)
			}
		})
	}
}
