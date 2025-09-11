//go:build integration
// +build integration

package main

import (
	"bytes"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

var binaryPath string

func TestMain(m *testing.M) {
	// Build the binary once before running tests
	tempDir, err := os.MkdirTemp("", "fnorm-test-*")
	if err != nil {
		panic(err)
	}
	defer os.RemoveAll(tempDir)

	binaryPath = filepath.Join(tempDir, "fnorm")

	// Build the binary - build the current directory (cmd/fnorm)
	cmd := exec.Command("go", "build", "-o", binaryPath, ".")
	output, err := cmd.CombinedOutput()
	if err != nil {
		panic(string(output) + ": " + err.Error())
	}

	// Run tests
	exitCode := m.Run()

	// Clean up
	os.Remove(binaryPath)
	os.Exit(exitCode)
}

// runFnorm executes the fnorm binary with given arguments
func runFnorm(args ...string) (string, string, error) {
	cmd := exec.Command(binaryPath, args...)
	var stdout, stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr
	err := cmd.Run()
	return stdout.String(), stderr.String(), err
}

// TestE2EVersion tests the -version flag
func TestE2EVersion(t *testing.T) {
	stdout, _, err := runFnorm("-version")
	if err != nil {
		t.Fatalf("Failed to run fnorm -version: %v", err)
	}

	if !strings.Contains(stdout, "fnorm version") {
		t.Errorf("Expected version output, got: %q", stdout)
	}
}

// TestE2EHelp tests the help output
func TestE2EHelp(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"help flag -h", []string{"-h"}},
		{"help flag --help", []string{"--help"}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// The help flag causes exit(0) which exec treats as success
			stdout, stderr, _ := runFnorm(tt.args...)

			// Help can go to stdout or stderr depending on implementation
			output := stdout + stderr

			expectedParts := []string{
				"fnorm - File name normalizer",
				"Usage: fnorm [flags] file1 [file2 ...]",
				"-dry-run",
				"-version",
				"Examples:",
			}

			for _, part := range expectedParts {
				if !strings.Contains(output, part) {
					t.Errorf("Help output missing %q\nGot: %q", part, output)
				}
			}
		})
	}
}

// TestE2ENoArguments tests behavior when no files are provided
func TestE2ENoArguments(t *testing.T) {
	_, stderr, err := runFnorm()

	// Should exit with error
	if err == nil {
		t.Error("Expected error when no arguments provided")
	}

	// Check exit code
	if exitErr, ok := err.(*exec.ExitError); ok {
		if exitErr.ExitCode() != 1 {
			t.Errorf("Expected exit code 1, got %d", exitErr.ExitCode())
		}
	}

	// Check error message
	if !strings.Contains(stderr, "No files specified") {
		t.Errorf("Expected 'No files specified' error, got: %q", stderr)
	}
}

// TestE2EFileRename tests actual file renaming
func TestE2EFileRename(t *testing.T) {
	tests := []struct {
		name         string
		inputFile    string
		expectedFile string
		shouldRename bool
	}{
		{
			name:         "spaces to hyphens",
			inputFile:    "Test File.txt",
			expectedFile: "test-file.txt",
			shouldRename: true,
		},
		{
			name:         "special characters",
			inputFile:    "File & Name @ 50%.doc",
			expectedFile: "file-and-name-at-50-percent.doc",
			shouldRename: true,
		},
		{
			name:         "already normalized",
			inputFile:    "already-normalized.txt",
			expectedFile: "already-normalized.txt",
			shouldRename: false,
		},
		{
			name:         "case and spaces",
			inputFile:    "Report Summary.txt",
			expectedFile: "report-summary.txt",
			shouldRename: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Create unique temp dir for each test case
			tempDir := t.TempDir()

			// Create test file
			inputPath := filepath.Join(tempDir, tt.inputFile)
			if err := os.WriteFile(inputPath, []byte("test content"), 0644); err != nil {
				t.Fatalf("Failed to create test file: %v", err)
			}

			// Run fnorm
			stdout, stderr, err := runFnorm(inputPath)
			if err != nil {
				t.Fatalf("fnorm failed: %v\nstdout: %s\nstderr: %s", err, stdout, stderr)
			}

			// Check output message
			if tt.shouldRename {
				if !strings.Contains(stdout, "Renamed:") {
					t.Errorf("Expected 'Renamed:' in output, got: %q", stdout)
				}
			} else {
				if !strings.Contains(stdout, "no changes needed") {
					t.Errorf("Expected 'no changes needed' in output, got: %q", stdout)
				}
			}

			// Verify file exists with expected name
			expectedPath := filepath.Join(tempDir, tt.expectedFile)
			if _, err := os.Stat(expectedPath); err != nil {
				t.Errorf("Expected file not found at %s: %v", expectedPath, err)
			}

			// Verify original file doesn't exist if it was renamed
			if tt.shouldRename {
				if _, err := os.Stat(inputPath); !os.IsNotExist(err) {
					t.Errorf("Original file still exists at %s", inputPath)
				}
			}

			// Clean up for next test
			os.Remove(expectedPath)
		})
	}
}

// TestE2EDryRun tests dry-run mode
func TestE2EDryRun(t *testing.T) {
	tempDir := t.TempDir()

	// Create test file
	testFile := "Test Dry Run.txt"
	testPath := filepath.Join(tempDir, testFile)
	if err := os.WriteFile(testPath, []byte("test content"), 0644); err != nil {
		t.Fatalf("Failed to create test file: %v", err)
	}

	// Run fnorm with dry-run
	stdout, _, err := runFnorm("-dry-run", testPath)
	if err != nil {
		t.Fatalf("fnorm dry-run failed: %v", err)
	}

	// Check output
	if !strings.Contains(stdout, "Would rename:") {
		t.Errorf("Expected 'Would rename:' in dry-run output, got: %q", stdout)
	}
	if !strings.Contains(stdout, "test-dry-run.txt") {
		t.Errorf("Expected normalized name in output, got: %q", stdout)
	}

	// Verify file was NOT renamed
	if _, err := os.Stat(testPath); err != nil {
		t.Errorf("Original file should still exist in dry-run mode: %v", err)
	}

	normalizedPath := filepath.Join(tempDir, "test-dry-run.txt")
	if _, err := os.Stat(normalizedPath); !os.IsNotExist(err) {
		t.Errorf("File should not be renamed in dry-run mode")
	}
}

// TestE2EMultipleFiles tests processing multiple files
func TestE2EMultipleFiles(t *testing.T) {
	tempDir := t.TempDir()

	// Create multiple test files
	testFiles := []string{
		"First File.txt",
		"Second File.doc",
		"Third File.pdf",
	}

	var filePaths []string
	for _, file := range testFiles {
		path := filepath.Join(tempDir, file)
		if err := os.WriteFile(path, []byte("content"), 0644); err != nil {
			t.Fatalf("Failed to create test file %s: %v", file, err)
		}
		filePaths = append(filePaths, path)
	}

	// Run fnorm on all files
	stdout, _, err := runFnorm(filePaths...)
	if err != nil {
		t.Fatalf("fnorm failed on multiple files: %v", err)
	}

	// Check that all files were renamed
	expectedFiles := []string{
		"first-file.txt",
		"second-file.doc",
		"third-file.pdf",
	}

	for i, expected := range expectedFiles {
		if !strings.Contains(stdout, "Renamed:") {
			t.Errorf("Expected rename message for file %s", testFiles[i])
		}

		expectedPath := filepath.Join(tempDir, expected)
		if _, err := os.Stat(expectedPath); err != nil {
			t.Errorf("Expected file not found: %s", expectedPath)
		}
	}
}

// TestE2ENonExistentFile tests error handling for non-existent files
func TestE2ENonExistentFile(t *testing.T) {
	_, stderr, err := runFnorm("/nonexistent/file.txt")

	// Should exit with error code 1
	if err == nil {
		t.Error("Expected error for non-existent file")
	}
	if exitErr, ok := err.(*exec.ExitError); ok {
		if exitErr.ExitCode() != 1 {
			t.Errorf("Expected exit code 1, got %d", exitErr.ExitCode())
		}
	} else {
		t.Errorf("Expected ExitError, got: %T", err)
	}

	// Check error message
	if !strings.Contains(stderr, "Error processing") {
		t.Errorf("Expected error message, got: %q", stderr)
	}
}

// TestE2EDirectory tests that directories are skipped
func TestE2EDirectory(t *testing.T) {
	tempDir := t.TempDir()

	// Create a subdirectory
	subDir := filepath.Join(tempDir, "Test Directory")
	if err := os.Mkdir(subDir, 0755); err != nil {
		t.Fatalf("Failed to create directory: %v", err)
	}

	_, stderr, err := runFnorm(subDir)

	// Should exit with error code 1 for directory
	if err == nil {
		t.Error("Expected error when processing directory")
	}
	if exitErr, ok := err.(*exec.ExitError); ok {
		if exitErr.ExitCode() != 1 {
			t.Errorf("Expected exit code 1, got %d", exitErr.ExitCode())
		}
	} else {
		t.Errorf("Expected ExitError, got: %T", err)
	}
	if !strings.Contains(stderr, "Error processing") && !strings.Contains(stderr, "directory") {
		t.Errorf("Expected directory skip message, got stderr: %q, err: %v", stderr, err)
	}
}

// TestE2ETargetExists tests behavior when target file already exists
func TestE2ETargetExists(t *testing.T) {
	tempDir := t.TempDir()

	// Create source file
	sourceFile := "Source File.txt"
	sourcePath := filepath.Join(tempDir, sourceFile)
	if err := os.WriteFile(sourcePath, []byte("source"), 0644); err != nil {
		t.Fatalf("Failed to create source file: %v", err)
	}

	// Create target file that would conflict
	targetFile := "source-file.txt"
	targetPath := filepath.Join(tempDir, targetFile)
	if err := os.WriteFile(targetPath, []byte("target"), 0644); err != nil {
		t.Fatalf("Failed to create target file: %v", err)
	}

	_, stderr, err := runFnorm(sourcePath)

	// Should exit with error code 1 when target exists
	if err == nil {
		t.Error("Expected error when target file already exists")
	}
	if exitErr, ok := err.(*exec.ExitError); ok {
		if exitErr.ExitCode() != 1 {
			t.Errorf("Expected exit code 1, got %d", exitErr.ExitCode())
		}
	} else {
		t.Errorf("Expected ExitError, got: %T", err)
	}
	if !strings.Contains(stderr, "Error processing") {
		t.Errorf("Expected error for existing target, got stderr: %q, err: %v", stderr, err)
	}
	if !strings.Contains(stderr, "already exists") {
		t.Errorf("Expected 'already exists' error, got stderr: %q, err: %v", stderr, err)
	}

	// Verify source file still exists
	if _, err := os.Stat(sourcePath); err != nil {
		t.Error("Source file should still exist after failed rename")
	}
}

// TestE2EComplexFilenames tests edge cases with complex filenames
func TestE2EComplexFilenames(t *testing.T) {
	tests := []struct {
		name         string
		inputFile    string
		expectedFile string
	}{
		{
			name:         "multiple spaces",
			inputFile:    "File   With   Many    Spaces.txt",
			expectedFile: "file-with-many-spaces.txt",
		},
		{
			name:         "leading/trailing spaces",
			inputFile:    "  Trimmed File  .txt",
			expectedFile: "trimmed-file.txt",
		},
		{
			name:         "multiple dots",
			inputFile:    "file.name.with.dots.txt",
			expectedFile: "file.name.with.dots.txt",
		},
		{
			name:         "no extension",
			inputFile:    "Installation Guide",
			expectedFile: "installation-guide",
		},
		// TODO: Add test for hidden files with spaces - currently ".Hidden File" -> ".hidden file"
		// (spaces not converted to hyphens). This may be a bug in the normalization function.
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Create unique temp dir for each test case
			tempDir := t.TempDir()

			// Create test file
			inputPath := filepath.Join(tempDir, tt.inputFile)
			if err := os.WriteFile(inputPath, []byte("test"), 0644); err != nil {
				t.Fatalf("Failed to create test file: %v", err)
			}

			// Run fnorm
			_, stderr, err := runFnorm(inputPath)
			if err != nil {
				t.Fatalf("fnorm failed: %v\nstderr: %s", err, stderr)
			}

			// Verify result
			expectedPath := filepath.Join(tempDir, tt.expectedFile)
			if _, err := os.Stat(expectedPath); err != nil {
				t.Errorf("Expected file not found at %s: %v", expectedPath, err)
			}

			// Clean up
			os.Remove(expectedPath)
		})
	}
}

// TestE2ECaseOnlyRename tests renaming files that only differ in case
// This was previously broken on case-insensitive filesystems like macOS/Windows
func TestE2ECaseOnlyRename(t *testing.T) {
	tests := []struct {
		name         string
		inputFile    string
		expectedFile string
	}{
		{
			name:         "uppercase extension",
			inputFile:    "document.TXT",
			expectedFile: "document.txt",
		},
		{
			name:         "uppercase filename",
			inputFile:    "DOCUMENT.TXT",
			expectedFile: "document.txt",
		},
		{
			name:         "all uppercase",
			inputFile:    "REPORT.DOC",
			expectedFile: "report.doc",
		},
		{
			name:         "mixed case filename",
			inputFile:    "Report.PDF",
			expectedFile: "report.pdf",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Create unique temp dir for each test case
			tempDir := t.TempDir()

			// Create test file
			inputPath := filepath.Join(tempDir, tt.inputFile)
			if err := os.WriteFile(inputPath, []byte("test content"), 0644); err != nil {
				t.Fatalf("Failed to create test file: %v", err)
			}

			// Run fnorm
			stdout, stderr, err := runFnorm(inputPath)
			if err != nil {
				t.Fatalf("fnorm failed: %v\nstdout: %s\nstderr: %s", err, stdout, stderr)
			}

			// Check output message (should show rename)
			if !strings.Contains(stdout, "Renamed:") {
				t.Errorf("Expected 'Renamed:' in output, got: %q", stdout)
			}

			// Verify file exists with expected name
			expectedPath := filepath.Join(tempDir, tt.expectedFile)
			if _, err := os.Stat(expectedPath); err != nil {
				t.Errorf("Expected file not found at %s: %v", expectedPath, err)
			}

			// For case-only renames on case-insensitive filesystems, we can't reliably
			// check that the original path doesn't exist since it may refer to the same file.
			// Instead, verify the file has the correct case by checking the directory listing.
			entries, err := os.ReadDir(tempDir)
			if err != nil {
				t.Fatalf("Failed to read directory: %v", err)
			}

			found := false
			for _, entry := range entries {
				if entry.Name() == tt.expectedFile {
					found = true
					break
				}
			}
			if !found {
				var entryNames []string
				for _, entry := range entries {
					entryNames = append(entryNames, entry.Name())
				}
				t.Errorf("Expected file with correct case not found. Directory contents: %v", entryNames)
			}

			// Verify content is preserved
			content, err := os.ReadFile(expectedPath)
			if err != nil {
				t.Fatalf("Failed to read renamed file: %v", err)
			}
			if string(content) != "test content" {
				t.Errorf("File content changed after rename. Got: %q", string(content))
			}
		})
	}
}

// TestE2ECaseAndContentRename tests files that have both case and content changes
// These should use the regular rename path, not the case-only path
func TestE2ECaseAndContentRename(t *testing.T) {
	tests := []struct {
		name         string
		inputFile    string
		expectedFile string
	}{
		{
			name:         "mixed case with spaces",
			inputFile:    "My Document.TXT",
			expectedFile: "my-document.txt",
		},
		{
			name:         "camel case filename",
			inputFile:    "MyFile.PDF",
			expectedFile: "myfile.pdf",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Create unique temp dir for each test case
			tempDir := t.TempDir()

			// Create test file
			inputPath := filepath.Join(tempDir, tt.inputFile)
			if err := os.WriteFile(inputPath, []byte("test content"), 0644); err != nil {
				t.Fatalf("Failed to create test file: %v", err)
			}

			// Run fnorm
			stdout, stderr, err := runFnorm(inputPath)
			if err != nil {
				t.Fatalf("fnorm failed: %v\nstdout: %s\nstderr: %s", err, stdout, stderr)
			}

			// Check output message (should show rename)
			if !strings.Contains(stdout, "Renamed:") {
				t.Errorf("Expected 'Renamed:' in output, got: %q", stdout)
			}

			// Verify file exists with expected name
			expectedPath := filepath.Join(tempDir, tt.expectedFile)
			if _, err := os.Stat(expectedPath); err != nil {
				t.Errorf("Expected file not found at %s: %v", expectedPath, err)
			}

			// For renames involving case changes on case-insensitive filesystems, we can't reliably
			// check that the original path doesn't exist since it may refer to the same file.
			// Instead, verify the file has the correct name by checking the directory listing.
			entries, err := os.ReadDir(tempDir)
			if err != nil {
				t.Fatalf("Failed to read directory: %v", err)
			}

			found := false
			for _, entry := range entries {
				if entry.Name() == tt.expectedFile {
					found = true
					break
				}
			}
			if !found {
				var entryNames []string
				for _, entry := range entries {
					entryNames = append(entryNames, entry.Name())
				}
				t.Errorf("Expected file with correct name not found. Directory contents: %v", entryNames)
			}

			// Verify content is preserved
			content, err := os.ReadFile(expectedPath)
			if err != nil {
				t.Fatalf("Failed to read renamed file: %v", err)
			}
			if string(content) != "test content" {
				t.Errorf("File content changed after rename. Got: %q", string(content))
			}
		})
	}
}
