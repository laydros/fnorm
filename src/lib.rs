// Re-export the main functionality for integration tests
mod error;
mod normalize;

use clap::{ArgAction, Parser};
use std::error::Error as StdError;
use std::fmt;
use std::path::{Path, PathBuf};

pub use error::FnormError;
pub use normalize::normalize;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(
    name = "fnorm",
    version = VERSION,
    about = "Normalize filenames to ASCII-only slug format",
    long_about = "Convert filenames to normalized, ASCII-only slug format while retaining directory location and file extensions.",
    disable_version_flag = true
)]
pub struct Cli {
    /// Print the version
    #[arg(short = 'v', long = "version", action = ArgAction::Version)]
    pub _version: bool,

    /// Print what would be done without making changes
    #[arg(long)]
    pub dry_run: bool,

    /// Files to normalize
    #[arg(value_name = "FILE", num_args = 1..)]
    pub files: Vec<PathBuf>,
}

/// Handle case-only renames using a temporary file to work on case-insensitive filesystems
fn rename_case_only(
    source: &Path,
    target: &Path,
    old_name: &str,
    new_name: &str,
) -> Result<(), FnormError> {
    let mut temp_path = source.to_path_buf();
    temp_path.set_file_name(format!("{}.fnorm-tmp", old_name));

    // Step 1: Rename to temporary
    std::fs::rename(source, &temp_path).map_err(|e| FnormError::RenameError {
        from: source.to_path_buf(),
        to: temp_path.clone(),
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
                from: temp_path,
                to: target.to_path_buf(),
                source: e,
            })
        }
    }
}

fn process_file(path: &Path, dry_run: bool) -> Result<(), FnormError> {
    use std::fs;
    use std::io::{self, ErrorKind};

    fs::metadata(path).map_err(|source| FnormError::FileNotFound {
        path: path.to_path_buf(),
        source,
    })?;

    let filename = path
        .file_name()
        .ok_or_else(|| FnormError::FileNotFound {
            path: path.to_path_buf(),
            source: io::Error::new(ErrorKind::InvalidInput, "invalid filename"),
        })?
        .to_string_lossy();

    let normalized = normalize(&filename);

    if filename == normalized {
        if !dry_run {
            println!("✓ {} (no changes needed)", filename);
        }
        return Ok(());
    }

    if dry_run {
        println!("Would rename: {} -> {}", filename, normalized);
        return Ok(());
    }

    let mut target_path = path.to_path_buf();
    target_path.set_file_name(&normalized);

    if filename.to_lowercase() == normalized.to_lowercase() {
        rename_case_only(path, &target_path, &filename, &normalized)?;
        return Ok(());
    }

    if target_path.exists() {
        return Err(FnormError::TargetExists { path: target_path });
    }

    std::fs::rename(path, &target_path).map_err(|source| FnormError::RenameError {
        from: path.to_path_buf(),
        to: target_path.clone(),
        source,
    })?;

    println!("Renamed: {} -> {}", filename, normalized);

    Ok(())
}

pub fn run(cli: Cli) -> Result<(), RunError> {
    let mut errors = RunError::default();

    for file in &cli.files {
        if let Err(error) = process_file(file, cli.dry_run) {
            errors.push(file.clone(), error);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[derive(Debug)]
struct RunErrorEntry {
    path: PathBuf,
    error: FnormError,
}

#[derive(Debug, Default)]
pub struct RunError {
    entries: Vec<RunErrorEntry>,
}

impl RunError {
    fn push(&mut self, path: PathBuf, error: FnormError) {
        self.entries.push(RunErrorEntry { path, error });
    }

    fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.entries.len() == 1 {
            writeln!(f, "failed to process 1 path:")?;
        } else {
            writeln!(f, "failed to process {} paths:", self.entries.len())?;
        }

        for entry in &self.entries {
            writeln!(f, "  {}: {}", entry.path.display(), entry.error)?;
            if let Some(source) = StdError::source(&entry.error) {
                writeln!(f, "    caused by: {}", source)?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for RunError {}
