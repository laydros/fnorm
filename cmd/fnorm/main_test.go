package main

import (
	"bytes"
	"flag"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestShowVersion(t *testing.T) {
	// Save original version and restore after test
	originalVersion := version
	defer func() { version = originalVersion }()

	version = "1.2.3-test"

	// Capture stdout
	old := os.Stdout
	r, w, _ := os.Pipe()
	os.Stdout = w

	// Set the version flag and run main
	*showVersion = true
	defer func() { *showVersion = false }()

	main()

	_ = w.Close()
	os.Stdout = old

	var buf bytes.Buffer
	_, _ = buf.ReadFrom(r)
	output := buf.String()

	expected := "fnorm version 1.2.3-test\n"
	if output != expected {
		t.Errorf("Expected %q, got %q", expected, output)
	}
}

func TestShowHelp(t *testing.T) {
	// Capture stdout
	old := os.Stdout
	r, w, _ := os.Pipe()
	os.Stdout = w

	showHelp()

	_ = w.Close()
	os.Stdout = old

	var buf bytes.Buffer
	_, _ = buf.ReadFrom(r)
	output := buf.String()

	// Check for key parts of the help text
	expectedParts := []string{
		"fnorm - File name normalizer",
		"Usage: fnorm [flags] file1 [file2 ...]",
		"-dry-run",
		"-version",
		"Examples:",
	}

	for _, part := range expectedParts {
		if !strings.Contains(output, part) {
			t.Errorf("Help output missing %q", part)
		}
	}
}

func TestProcessFile(t *testing.T) {
	// Create temporary directory for test files
	tempDir := t.TempDir()

	tests := []struct {
		name        string
		filename    string
		dryRun      bool
		expectError bool
		errorType   error
	}{
		{
			name:        "normal file rename",
			filename:    "Test File.txt",
			dryRun:      false,
			expectError: false,
		},
		{
			name:        "file already normalized",
			filename:    "already-normalized.txt",
			dryRun:      false,
			expectError: false,
		},
		{
			name:        "dry run mode",
			filename:    "Dry Run Test.txt",
			dryRun:      true,
			expectError: false,
		},
		{
			name:        "directory should be skipped without --dirs flag",
			filename:    "testdir",
			dryRun:      false,
			expectError: true,
			errorType:   errIsDir,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Set dry-run flag
			originalDryRun := *dryRun
			*dryRun = tt.dryRun
			defer func() { *dryRun = originalDryRun }()

			// Create test file or directory
			testPath := filepath.Join(tempDir, tt.filename)
			if tt.filename == "testdir" {
				err := os.Mkdir(testPath, 0750)
				if err != nil {
					t.Fatalf("Failed to create test directory: %v", err)
				}
			} else {
				file, err := os.Create(testPath) // #nosec G304 -- test file path
				if err != nil {
					t.Fatalf("Failed to create test file: %v", err)
				}
				_ = file.Close()
			}

			// Capture stdout/stderr
			oldStdout := os.Stdout
			oldStderr := os.Stderr
			rOut, wOut, _ := os.Pipe()
			rErr, wErr, _ := os.Pipe()
			os.Stdout = wOut
			os.Stderr = wErr

			err := processFile(testPath)

			_ = wOut.Close()
			_ = wErr.Close()
			os.Stdout = oldStdout
			os.Stderr = oldStderr

			var bufOut, bufErr bytes.Buffer
			_, _ = bufOut.ReadFrom(rOut)
			_, _ = bufErr.ReadFrom(rErr)

			if tt.expectError {
				if err == nil {
					t.Errorf("Expected error but got none")
				}
				if tt.errorType != nil && !strings.Contains(err.Error(), tt.errorType.Error()) {
					t.Errorf("Expected error containing %q, got %q", tt.errorType.Error(), err.Error())
				}
			} else {
				if err != nil {
					t.Errorf("Unexpected error: %v", err)
				}

				// Check output for expected messages
				output := bufOut.String()
				if tt.dryRun && tt.filename == "Dry Run Test.txt" {
					if !strings.Contains(output, "Would rename:") {
						t.Errorf("Expected dry-run output, got: %q", output)
					}
				} else if tt.filename == "already-normalized.txt" {
					if !strings.Contains(output, "no changes needed") {
						t.Errorf("Expected 'no changes needed' output, got: %q", output)
					}
				}
			}
		})
	}
}

func TestProcessFileDirectories(t *testing.T) {
	// Create temporary directory for test directories
	tempDir := t.TempDir()

	tests := []struct {
		name        string
		dirname     string
		dirsFlag    bool
		dryRun      bool
		expectError bool
		errorType   error
	}{
		{
			name:        "directory rename with --dirs flag",
			dirname:     "Test Directory",
			dirsFlag:    true,
			dryRun:      false,
			expectError: false,
		},
		{
			name:        "directory already normalized with --dirs flag",
			dirname:     "already-normalized",
			dirsFlag:    true,
			dryRun:      false,
			expectError: false,
		},
		{
			name:        "directory dry run with --dirs flag",
			dirname:     "Dry Run Directory",
			dirsFlag:    true,
			dryRun:      true,
			expectError: false,
		},
		{
			name:        "directory without --dirs flag should fail",
			dirname:     "Should Fail Directory",
			dirsFlag:    false,
			dryRun:      false,
			expectError: true,
			errorType:   errIsDir,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Set flags
			originalDryRun := *dryRun
			originalDirs := *dirs
			*dryRun = tt.dryRun
			*dirs = tt.dirsFlag
			defer func() {
				*dryRun = originalDryRun
				*dirs = originalDirs
			}()

			// Create test directory
			testPath := filepath.Join(tempDir, tt.dirname)
			err := os.Mkdir(testPath, 0750)
			if err != nil {
				t.Fatalf("Failed to create test directory: %v", err)
			}

			// Capture stdout/stderr
			oldStdout := os.Stdout
			oldStderr := os.Stderr
			rOut, wOut, _ := os.Pipe()
			rErr, wErr, _ := os.Pipe()
			os.Stdout = wOut
			os.Stderr = wErr

			err = processFile(testPath)

			_ = wOut.Close()
			_ = wErr.Close()
			os.Stdout = oldStdout
			os.Stderr = oldStderr

			var bufOut, bufErr bytes.Buffer
			_, _ = bufOut.ReadFrom(rOut)
			_, _ = bufErr.ReadFrom(rErr)

			if tt.expectError {
				if err == nil {
					t.Errorf("Expected error but got none")
				}
				if tt.errorType != nil && !strings.Contains(err.Error(), tt.errorType.Error()) {
					t.Errorf("Expected error containing %q, got %q", tt.errorType.Error(), err.Error())
				}
			} else {
				if err != nil {
					t.Errorf("Unexpected error: %v", err)
				}

				// Check output for expected messages
				output := bufOut.String()
				if tt.dryRun && tt.dirname == "Dry Run Directory" {
					if !strings.Contains(output, "Would rename:") {
						t.Errorf("Expected dry-run output, got: %q", output)
					}
				} else if tt.dirname == "already-normalized" {
					if !strings.Contains(output, "no changes needed") {
						t.Errorf("Expected 'no changes needed' output, got: %q", output)
					}
				} else if !tt.dryRun && tt.dirname == "Test Directory" {
					// Check that the directory was actually renamed
					expectedNewPath := filepath.Join(tempDir, "test-directory")
					if _, err := os.Stat(expectedNewPath); os.IsNotExist(err) {
						t.Errorf("Expected directory to be renamed to %s", expectedNewPath)
					}
					if !strings.Contains(output, "Renamed:") {
						t.Errorf("Expected rename confirmation output, got: %q", output)
					}
				}
			}
		})
	}
}

func TestProcessFileNonExistent(t *testing.T) {
	err := processFile("/nonexistent/file.txt")
	if err == nil {
		t.Error("Expected error for non-existent file")
	}
	if !strings.Contains(err.Error(), "stat") {
		t.Errorf("Expected stat error, got: %v", err)
	}
}

func TestProcessFileTargetExists(t *testing.T) {
	tempDir := t.TempDir()

	// Create source file that needs normalization
	sourceFile := filepath.Join(tempDir, "Source File.txt")
	file, err := os.Create(sourceFile) // #nosec G304 -- test file path
	if err != nil {
		t.Fatalf("Failed to create source file: %v", err)
	}
	_ = file.Close()

	// Create target file that would conflict
	targetFile := filepath.Join(tempDir, "source-file.txt")
	file, err = os.Create(targetFile) // #nosec G304 -- test file path
	if err != nil {
		t.Fatalf("Failed to create target file: %v", err)
	}
	_ = file.Close()

	// Set dry-run to false
	originalDryRun := *dryRun
	*dryRun = false
	defer func() { *dryRun = originalDryRun }()

	err = processFile(sourceFile)
	if err == nil {
		t.Error("Expected error when target file exists")
	}
	if !strings.Contains(err.Error(), "target already exists") {
		t.Errorf("Expected 'target already exists' error, got: %v", err)
	}
}

func TestProcessDirectoryTargetExists(t *testing.T) {
	tempDir := t.TempDir()

	// Create source directory that needs normalization
	sourceDir := filepath.Join(tempDir, "Source Directory")
	err := os.Mkdir(sourceDir, 0750)
	if err != nil {
		t.Fatalf("Failed to create source directory: %v", err)
	}

	// Create target directory that would conflict
	targetDir := filepath.Join(tempDir, "source-directory")
	err = os.Mkdir(targetDir, 0750)
	if err != nil {
		t.Fatalf("Failed to create target directory: %v", err)
	}

	// Set flags
	originalDryRun := *dryRun
	originalDirs := *dirs
	*dryRun = false
	*dirs = true
	defer func() {
		*dryRun = originalDryRun
		*dirs = originalDirs
	}()

	err = processFile(sourceDir)
	if err == nil {
		t.Error("Expected error when target directory exists")
	}
	if !strings.Contains(err.Error(), "target already exists") {
		t.Errorf("Expected 'target already exists' error, got: %v", err)
	}
}

func TestMainLogic(t *testing.T) {
	// Test the main function's logic by testing with flag.Args() directly
	// This avoids the complexity of testing os.Exit behavior

	t.Run("no arguments", func(t *testing.T) {
		// Save and restore flag state
		oldArgs := flag.Args()
		defer func() {
			// Reset os.Args to restore flag.Args()
			os.Args = append([]string{"fnorm"}, oldArgs...)
			flag.Parse()
		}()

		// Clear arguments
		os.Args = []string{"fnorm"}
		flag.CommandLine = flag.NewFlagSet(os.Args[0], flag.ContinueOnError)
		dryRun = flag.Bool("dry-run", false, "Show what would be renamed without making changes")
		showVersion = flag.Bool("version", false, "Show version information")
		flag.Parse()

		// Since we can't easily test os.Exit, we test that flag.Args() returns empty
		args := flag.Args()
		if len(args) != 0 {
			t.Errorf("Expected no args, got %d args", len(args))
		}

		// The main function would call os.Exit(1) here, but we can't test that directly
		// The error message would be printed to stderr, which we've tested in other ways
	})
}

func TestMainWithFiles(t *testing.T) {
	tempDir := t.TempDir()

	// Create test files
	testFiles := []string{"Test File 1.txt", "Test File 2.txt"}
	for _, filename := range testFiles {
		file, err := os.Create(filepath.Join(tempDir, filename)) // #nosec G304 -- test file path
		if err != nil {
			t.Fatalf("Failed to create test file %s: %v", filename, err)
		}
		_ = file.Close()
	}

	// Save original args and flags
	originalArgs := os.Args
	originalShowVersion := *showVersion
	originalDryRun := *dryRun
	defer func() {
		os.Args = originalArgs
		*showVersion = originalShowVersion
		*dryRun = originalDryRun
	}()

	// Reset flag state
	flag.CommandLine = flag.NewFlagSet(os.Args[0], flag.ExitOnError)
	dryRun = flag.Bool("dry-run", false, "Show what would be renamed without making changes")
	showVersion = flag.Bool("version", false, "Show version information")

	// Set up args with dry-run flag and test files
	os.Args = []string{"fnorm", "-dry-run",
		filepath.Join(tempDir, testFiles[0]),
		filepath.Join(tempDir, testFiles[1])}

	// Capture stdout
	oldStdout := os.Stdout
	r, w, _ := os.Pipe()
	os.Stdout = w

	main()

	_ = w.Close()
	os.Stdout = oldStdout

	var buf bytes.Buffer
	_, _ = buf.ReadFrom(r)
	output := buf.String()

	// Should contain dry-run output for both files
	if !strings.Contains(output, "Would rename:") {
		t.Errorf("Expected dry-run output, got: %q", output)
	}

	// Count occurrences - should be one for each file
	count := strings.Count(output, "Would rename:")
	if count != 2 {
		t.Errorf("Expected 2 'Would rename:' messages, got %d", count)
	}
}
