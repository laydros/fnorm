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
