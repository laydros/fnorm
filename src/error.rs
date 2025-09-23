use std::fmt;

#[derive(Debug)]
pub enum FnormError {
    NotAFile(String),
    FileNotFound(String),
    TargetExists(String),
    RenameError {
        from: String,
        to: String,
        source: std::io::Error,
    },
}

impl fmt::Display for FnormError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FnormError::NotAFile(path) => write!(f, "skipping directory: {}: is a directory", path),
            FnormError::FileNotFound(path) => write!(f, "file not found: {}", path),
            FnormError::TargetExists(path) => write!(f, "target file already exists: \"{}\"", path),
            FnormError::RenameError { from, to, source } => {
                write!(f, "failed to rename \"{}\" to \"{}\": {}", from, to, source)
            }
        }
    }
}

impl std::error::Error for FnormError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FnormError::RenameError { source, .. } => Some(source),
            _ => None,
        }
    }
}
