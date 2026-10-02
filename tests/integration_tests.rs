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
        config: None,
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
fn test_file_case_only_change_target_exists() {
    let temp_dir = TempDir::new().unwrap();
    let source = create_test_path(temp_dir.path(), "Foo.txt", false);
    let target = temp_dir.path().join("foo.txt");

    // Only meaningful where Foo.txt and foo.txt can coexist
    if target.exists() {
        return;
    }
    fs::write(&target, "original target").unwrap();

    let result = run_fnorm(&source, false);
    assert!(result.is_err(), "Should fail when a distinct target exists");
    assert!(
        result.unwrap_err().contains("already exists"),
        "Error should mention target exists"
    );
    assert!(source.exists(), "Source should be left in place");
    assert_eq!(
        fs::read_to_string(&target).unwrap(),
        "original target",
        "Existing target must not be overwritten"
    );
}

#[cfg(unix)]
#[test]
fn test_file_target_is_dangling_symlink() {
    let temp_dir = TempDir::new().unwrap();
    let source = create_test_path(temp_dir.path(), "Source File.txt", false);
    let target = temp_dir.path().join("source-file.txt");
    std::os::unix::fs::symlink(temp_dir.path().join("missing"), &target).unwrap();

    let result = run_fnorm(&source, false);
    assert!(
        result.is_err(),
        "Should fail when target is a dangling symlink"
    );
    assert!(source.exists(), "Source should be left in place");
    assert!(
        target.symlink_metadata().unwrap().file_type().is_symlink(),
        "Symlink must not be overwritten"
    );
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

#[test]
fn test_cli_reports_errors_with_display_and_exit_code() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let missing = temp_dir.path().join("Missing File.txt");

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_fnorm"))
        .arg(&missing)
        .output()
        .expect("Failed to run fnorm binary");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("file not found"), "stderr: {stderr}");
    assert!(!stderr.contains("RunError"), "stderr: {stderr}");
}

fn run_fnorm_binary(args: &[&std::ffi::OsStr]) -> (Option<i32>, String) {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_fnorm"))
        .args(args)
        .output()
        .expect("Failed to run fnorm binary");
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

#[test]
fn test_config_parse_error_shows_reason() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config = temp_dir.path().join("bad.toml");
    fs::write(&config, "bad = [").expect("Failed to write config");
    let file = create_test_path(temp_dir.path(), "Some File.txt", false);

    let (code, stderr) =
        run_fnorm_binary(&["--config".as_ref(), config.as_os_str(), file.as_os_str()]);

    assert_eq!(code, Some(1));
    assert!(
        stderr.contains("failed to parse config"),
        "stderr: {stderr}"
    );
    assert!(stderr.contains("line 1"), "stderr: {stderr}");
    assert!(
        file.exists(),
        "File must not be processed when config fails"
    );
}

#[test]
fn test_config_parse_error_reason_is_indented_under_header() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config = temp_dir.path().join("bad.toml");
    fs::write(&config, "bad = [").expect("Failed to write config");
    let file = create_test_path(temp_dir.path(), "Some File.txt", false);

    let (_, stderr) =
        run_fnorm_binary(&["--config".as_ref(), config.as_os_str(), file.as_os_str()]);

    let reason_lines: Vec<&str> = stderr.trim_end().lines().skip(1).collect();
    assert!(reason_lines.len() > 1, "stderr: {stderr}");
    assert!(
        reason_lines.iter().all(|line| line.starts_with("  ")),
        "stderr: {stderr}"
    );
}

#[test]
fn test_config_read_error_shows_os_error() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config = temp_dir.path().join("missing.toml");
    let file = create_test_path(temp_dir.path(), "Some File.txt", false);

    let (code, stderr) =
        run_fnorm_binary(&["--config".as_ref(), config.as_os_str(), file.as_os_str()]);

    assert_eq!(code, Some(1));
    assert!(stderr.contains("failed to read config"), "stderr: {stderr}");
    assert!(
        stderr.contains("No such file or directory"),
        "stderr: {stderr}"
    );
}

#[test]
fn test_file_not_found_names_path_once() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let missing = temp_dir.path().join("Missing File.txt");

    let (code, stderr) = run_fnorm_binary(&[missing.as_os_str()]);

    assert_eq!(code, Some(1));
    let shown = missing.display().to_string();
    assert_eq!(stderr.matches(&shown).count(), 1, "stderr: {stderr}");
}

#[cfg(unix)]
#[test]
fn test_permission_error_is_not_reported_as_not_found() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let locked = create_test_path(temp_dir.path(), "locked", true);
    let file = create_test_path(&locked, "Some File.txt", false);
    fs::set_permissions(&locked, fs::Permissions::from_mode(0o000)).expect("chmod");
    let readable = fs::metadata(&file).is_ok();

    let (code, stderr) = run_fnorm_binary(&[file.as_os_str()]);
    fs::set_permissions(&locked, fs::Permissions::from_mode(0o755)).expect("chmod");

    // Running as root ignores directory permissions, so there is no error to check
    if readable {
        return;
    }
    assert_eq!(code, Some(1));
    assert!(!stderr.contains("file not found"), "stderr: {stderr}");
    assert!(stderr.contains("Permission denied"), "stderr: {stderr}");
}

#[cfg(unix)]
#[test]
fn test_rename_failure_shows_os_error_once() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let readonly = create_test_path(temp_dir.path(), "readonly", true);
    let file = create_test_path(&readonly, "Some File.txt", false);
    fs::set_permissions(&readonly, fs::Permissions::from_mode(0o555)).expect("chmod");

    let (code, stderr) = run_fnorm_binary(&[file.as_os_str()]);
    fs::set_permissions(&readonly, fs::Permissions::from_mode(0o755)).expect("chmod");

    // Running as root ignores directory permissions, so the rename succeeds
    if code == Some(0) {
        return;
    }
    assert_eq!(code, Some(1));
    assert!(stderr.contains("failed to rename"), "stderr: {stderr}");
    assert_eq!(
        stderr.matches("Permission denied").count(),
        1,
        "stderr: {stderr}"
    );
}

#[test]
fn test_no_files_is_an_error() {
    let (code, stderr) = run_fnorm_binary(&[]);

    assert_eq!(code, Some(2), "stderr: {stderr}");
    assert!(stderr.contains("Usage:"), "stderr: {stderr}");
}

#[test]
fn test_name_that_normalizes_to_nothing_is_left_alone() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file = create_test_path(temp_dir.path(), "!!!", false);

    let (code, stderr) = run_fnorm_binary(&[file.as_os_str()]);

    assert_eq!(code, Some(1));
    assert!(stderr.contains("empty name"), "stderr: {stderr}");
    assert!(file.exists(), "Original file should be untouched");
}

#[test]
fn test_name_with_only_extension_left_is_not_made_hidden() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file = create_test_path(temp_dir.path(), "!!!.txt", false);

    let (code, stderr) = run_fnorm_binary(&[file.as_os_str()]);

    assert_eq!(code, Some(1));
    assert!(stderr.contains("empty name"), "stderr: {stderr}");
    assert!(file.exists(), "Original file should be untouched");
    assert!(
        !temp_dir.path().join(".txt").exists(),
        "File must not become a hidden file"
    );
}

#[test]
fn test_dry_run_reports_name_that_normalizes_to_nothing() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file = create_test_path(temp_dir.path(), "!!!.txt", false);

    let (code, stderr) = run_fnorm_binary(&["--dry-run".as_ref(), file.as_os_str()]);

    assert_eq!(code, Some(1));
    assert!(stderr.contains("empty name"), "stderr: {stderr}");
}
