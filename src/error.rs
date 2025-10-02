use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum FnormError {
    FileNotFound {
        path: PathBuf,
        source: io::Error,
    },
    TargetExists {
        path: PathBuf,
    },
    RenameError {
        from: PathBuf,
        to: PathBuf,
        source: io::Error,
    },
}

impl fmt::Display for FnormError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FnormError::FileNotFound { path, .. } => {
                write!(f, "file not found: {}", path.display())
            }
            FnormError::TargetExists { path } => {
                write!(f, "target file already exists: \"{}\"", path.display())
            }
            FnormError::RenameError { from, to, source } => {
                write!(
                    f,
                    "failed to rename \"{}\" to \"{}\": {}",
                    from.display(),
                    to.display(),
                    source
                )
            }
        }
    }
}

impl std::error::Error for FnormError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FnormError::FileNotFound { source, .. } => Some(source),
            FnormError::RenameError { source, .. } => Some(source),
            FnormError::TargetExists { .. } => None,
        }
    }
}
