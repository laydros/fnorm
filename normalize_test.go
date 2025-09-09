package fnorm

import "testing"

func TestNormalize(t *testing.T) {
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
			input:    "file!name.txt",
			expected: "file-name.txt",
		},
		{
			name:     "slash replaced with or",
			input:    "tcp/udp guide.md",
			expected: "tcp-or-udp-guide.md",
		},
		{
			name:     "ampersand replaced with and",
			input:    "Backup & Restore Process.txt",
			expected: "backup-and-restore-process.txt",
		},
		{
			name:     "at sign replaced with at",
			input:    "Meeting @ Headquarters.md",
			expected: "meeting-at-headquarters.md",
		},
		{
			name:     "percent sign replaced with percent",
			input:    "CPU Usage 90%.txt",
			expected: "cpu-usage-90-percent.txt",
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
		{
			name:     "empty string",
			input:    "",
			expected: "",
		},
		{
			name:     "file without extension",
			input:    "README",
			expected: "readme",
		},
		{
			name:     "multiple dots preserved",
			input:    "file.name.txt",
			expected: "file.name.txt",
		},
		{
			name:     "unicode characters transliterated",
			input:    "café.txt",
			expected: "cafe.txt",
		},
		{
			name:     "typographic dashes transliterated",
			input:    "foo–bar—baz.txt",
			expected: "foo-bar-baz.txt",
		},
		{
			name:     "curly apostrophes transliterated",
			input:    "rock’n’roll.txt",
			expected: "rock-n-roll.txt",
		},
		{
			name:     "smart quotes transliterated",
			input:    "test“quote”file.txt",
			expected: "test-quote-file.txt",
		},
		{
			name:     "leading and trailing spaces",
			input:    " file ",
			expected: "file",
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			got := Normalize(tc.input)
			if got != tc.expected {
				t.Fatalf("expected %q, got %q", tc.expected, got)
			}
		})
	}
}
