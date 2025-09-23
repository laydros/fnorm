mod error;
mod normalize;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use error::FnormError;

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
    // For now, just simulate the file processing
    println!("Processing: {} (dry_run: {})", file.display(), dry_run);

    // This is where you'll add the actual file processing logic later
    // For now, just verify the file exists and is not a directory
    let metadata = std::fs::metadata(file)
        .map_err(|_| FnormError::FileNotFound(file.display().to_string()))?;

    if metadata.is_dir() {
        return Err(FnormError::NotAFile(file.display().to_string()));
    }

    println!("✓ {} (placeholder - no changes needed)", file.display());
    Ok(())
}
