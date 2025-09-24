mod error;
mod normalize;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use error::FnormError;
use normalize::normalize;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(
    name = "fnorm",
    version = VERSION,
    about = "Normalize filenames to ASCII-only slug format",
    long_about = "Convert filenames to normalized, ASCII-only slug format while retaining directory location and file extensions."
)]
struct Cli {
    /// Print what would be done without making changes
    #[arg(long)]
    dry_run: bool,

    /// Files to normalize
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle the case where no files are provided
    if cli.files.is_empty() {
        eprintln!("Error: No files specified");
        eprintln!("Use --help for usage information");
        std::process::exit(1);
    }

    let mut has_errors = false;

    for file in &cli.files {
        if let Err(e) = process_file(file, cli.dry_run) {
            eprintln!("Error processing {}: {}", file.display(), e);
            has_errors = true;
        }
    }

    if has_errors {
        std::process::exit(1);
    }

    Ok(())
}

fn process_file(file: &PathBuf, dry_run: bool) -> Result<(), FnormError> {
    // Step 1: Check if file exists and is not a directory
    let metadata = std::fs::metadata(file)
        .map_err(|_| FnormError::FileNotFound(file.display().to_string()))?;

    if metadata.is_dir() {
        return Err(FnormError::NotAFile(file.display().to_string()));
    }

    // Step 2: Get the filename and compute normalized version
    let filename = file
        .file_name()
        .ok_or_else(|| FnormError::FileNotFound("Invalid filename".to_string()))?
        .to_string_lossy();

    let normalized = normalize(&filename);

    // Step 3: Determine if changes are needed
    if filename == normalized {
        // No changes needed
        if !dry_run {
            println!("✓ {} (no changes needed)", filename);
        }
        return Ok(());
    }

    // Step 4: Handle dry-run vs actual rename
    if dry_run {
        println!("Would rename: {} -> {}", filename, normalized);
        return Ok(());
    }

    // Step 5: Construct target path
    let mut target_path = file.clone();
    target_path.set_file_name(&normalized);

    // Step 6: Check for case-only rename
    let is_case_only_rename = filename.to_lowercase() == normalized.to_lowercase();

    if is_case_only_rename {
        // Handle case-only rename with temporary file
        rename_case_only(file, &target_path, &filename, &normalized)?;
    } else {
        // Regular rename - check if target exists first
        if target_path.exists() {
            return Err(FnormError::TargetExists(target_path.display().to_string()));
        }

        std::fs::rename(file, &target_path).map_err(|e| FnormError::RenameError {
            from: file.display().to_string(),
            to: target_path.display().to_string(),
            source: e,
        })?;

        println!("Renamed: {} -> {}", filename, normalized);
    }

    Ok(())
}

/// Handle case-only renames using a temporary file to work on case-insensitive filesystems
fn rename_case_only(
    source: &PathBuf,
    target: &PathBuf,
    old_name: &str,
    new_name: &str,
) -> Result<(), FnormError> {
    let mut temp_path = source.clone();
    temp_path.set_file_name(format!("{}.fnorm-tmp", old_name));

    // Step 1: Rename to temporary
    std::fs::rename(source, &temp_path).map_err(|e| FnormError::RenameError {
        from: source.display().to_string(),
        to: temp_path.display().to_string(),
        source: e,
    })?;

    // Step 2: Rename from temporary to final name
    match std::fs::rename(&temp_path, target) {
        Ok(_) => {
            println!("Renamed: {} -> {}", old_name, new_name);
            Ok(())
        }
        Err(e) => {
            // Restore original name if second step fails
            let _ = std::fs::rename(&temp_path, source);
            Err(FnormError::RenameError {
                from: temp_path.display().to_string(),
                to: target.display().to_string(),
                source: e,
            })
        }
    }
}
