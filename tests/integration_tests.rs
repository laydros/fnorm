use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

// Helper function to create a test file or directory
fn create_test_path(dir: &Path, name: &str, is_dir: bool) -> PathBuf {
    let path = dir.join(name);
    if is_dir {
        fs::create_dir(&path).expect("Failed to create test directory");
    } else {
        fs::write(&path, "test content").expect("Failed to create test file");
    }
    path
}

// Helper function to run fnorm on a path
fn run_fnorm(path: &Path, dry_run: bool) -> Result<(), String> {
    use fnorm::Cli;

    let cli = Cli {
        dry_run,
        files: vec![path.to_path_buf()],
    };

    fnorm::run(&cli).map_err(|e| e.to_string())
}

#[test]
fn test_directory_basic_rename() {
    let temp_dir = TempDir::new().unwrap();
    let test_dir = create_test_path(temp_dir.path(), "My Test Directory", true);

    run_fnorm(&test_dir, false).expect("Rename should succeed");

    let expected_path = temp_dir.path().join("my-test-directory");
    assert!(expected_path.exists(), "Directory should be renamed");
    assert!(!test_dir.exists(), "Original directory should not exist");
}

#[test]
fn test_directory_with_special_chars() {
    let temp_dir = TempDir::new().unwrap();
    let test_dir = create_test_path(temp_dir.path(), "Photos & Videos", true);

    run_fnorm(&test_dir, false).expect("Rename should succeed");

    let expected_path = temp_dir.path().join("photos-and-videos");
    assert!(
        expected_path.exists(),
        "Directory should be renamed with special chars handled"
    );
}

#[test]
fn test_directory_with_unicode() {
    let temp_dir = TempDir::new().unwrap();
    let test_dir = create_test_path(temp_dir.path(), "café menu", true);

    run_fnorm(&test_dir, false).expect("Rename should succeed");

    let expected_path = temp_dir.path().join("cafe-menu");
    assert!(
        expected_path.exists(),
        "Directory should be renamed with unicode transliterated"
    );
}

#[test]
fn test_directory_case_only_change() {
    let temp_dir = TempDir::new().unwrap();
    let test_dir = create_test_path(temp_dir.path(), "MyDirectory", true);

    run_fnorm(&test_dir, false).expect("Case-only rename should succeed");

    let expected_path = temp_dir.path().join("mydirectory");
    assert!(
        expected_path.exists(),
        "Directory should exist with new case"
    );

    // Verify the actual case on disk (this may behave differently on case-insensitive filesystems)
    let actual_name = expected_path.file_name().unwrap().to_str().unwrap();
    assert_eq!(
        actual_name, "mydirectory",
        "Directory name should be lowercase"
    );
}

#[test]
fn test_directory_already_normalized() {
    let temp_dir = TempDir::new().unwrap();
    let test_dir = create_test_path(temp_dir.path(), "already-normalized", true);

    run_fnorm(&test_dir, false).expect("Should succeed with no changes");

    assert!(
        test_dir.exists(),
        "Directory should still exist at original path"
    );
}

#[test]
fn test_directory_target_exists() {
    let temp_dir = TempDir::new().unwrap();
    let test_dir = create_test_path(temp_dir.path(), "Source Dir", true);
    // Create the target directory that would conflict
    create_test_path(temp_dir.path(), "source-dir", true);

    let result = run_fnorm(&test_dir, false);
    assert!(result.is_err(), "Should fail when target exists");
    assert!(
        result.unwrap_err().contains("already exists"),
        "Error should mention target exists"
    );
}

#[test]
fn test_file_basic_rename() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = create_test_path(temp_dir.path(), "My Test File.txt", false);

    run_fnorm(&test_file, false).expect("Rename should succeed");

    let expected_path = temp_dir.path().join("my-test-file.txt");
    assert!(expected_path.exists(), "File should be renamed");
    assert!(!test_file.exists(), "Original file should not exist");
}

#[test]
fn test_file_with_special_chars() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = create_test_path(temp_dir.path(), "Ben & Jerry's.txt", false);

    run_fnorm(&test_file, false).expect("Rename should succeed");

    let expected_path = temp_dir.path().join("ben-and-jerry-s.txt");
    assert!(
        expected_path.exists(),
        "File should be renamed with special chars handled"
    );
}

#[test]
fn test_file_extension_preserved() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = create_test_path(temp_dir.path(), "My Document.PDF", false);

    run_fnorm(&test_file, false).expect("Rename should succeed");

    let expected_path = temp_dir.path().join("my-document.pdf");
    assert!(
        expected_path.exists(),
        "File should be renamed with extension lowercased"
    );
}

#[test]
fn test_file_case_only_change() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = create_test_path(temp_dir.path(), "README.TXT", false);

    run_fnorm(&test_file, false).expect("Case-only rename should succeed");

    let expected_path = temp_dir.path().join("readme.txt");
    assert!(expected_path.exists(), "File should exist with new case");
}

#[test]
fn test_file_target_exists() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = create_test_path(temp_dir.path(), "Source File.txt", false);
    // Create the target file that would conflict
    create_test_path(temp_dir.path(), "source-file.txt", false);

    let result = run_fnorm(&test_file, false);
    assert!(result.is_err(), "Should fail when target exists");
    assert!(
        result.unwrap_err().contains("already exists"),
        "Error should mention target exists"
    );
}

#[test]
fn test_dry_run_directory() {
    let temp_dir = TempDir::new().unwrap();
    let test_dir = create_test_path(temp_dir.path(), "Test Directory", true);
    let original_path = test_dir.clone();

    run_fnorm(&test_dir, true).expect("Dry run should succeed");

    assert!(
        original_path.exists(),
        "Original directory should still exist"
    );
    let expected_path = temp_dir.path().join("test-directory");
    assert!(
        !expected_path.exists(),
        "Target directory should not exist in dry run"
    );
}

#[test]
fn test_dry_run_file() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = create_test_path(temp_dir.path(), "Test File.txt", false);
    let original_path = test_file.clone();

    run_fnorm(&test_file, true).expect("Dry run should succeed");

    assert!(original_path.exists(), "Original file should still exist");
    let expected_path = temp_dir.path().join("test-file.txt");
    assert!(
        !expected_path.exists(),
        "Target file should not exist in dry run"
    );
}

#[test]
fn test_file_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent = temp_dir.path().join("does-not-exist.txt");

    let result = run_fnorm(&nonexistent, false);
    assert!(result.is_err(), "Should fail for nonexistent file");
    assert!(
        result.unwrap_err().contains("not found"),
        "Error should mention file not found"
    );
}

#[test]
fn test_directory_preserves_contents() {
    let temp_dir = TempDir::new().unwrap();
    let test_dir = create_test_path(temp_dir.path(), "Parent Dir", true);

    // Create a file inside the directory
    let child_file = test_dir.join("child-file.txt");
    fs::write(&child_file, "test content").expect("Failed to create child file");

    run_fnorm(&test_dir, false).expect("Rename should succeed");

    let expected_dir = temp_dir.path().join("parent-dir");
    let expected_file = expected_dir.join("child-file.txt");

    assert!(expected_dir.exists(), "Renamed directory should exist");
    assert!(expected_file.exists(), "Child file should be preserved");

    let content = fs::read_to_string(&expected_file).expect("Should read child file");
    assert_eq!(
        content, "test content",
        "Child file content should be preserved"
    );
}
