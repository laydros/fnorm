use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use toml::de::Error as TomlError;

#[derive(Debug)]
pub enum ConfigError {
    Io {
        path: PathBuf,
        source: io::Error,
    },
    Parse {
        path: Option<PathBuf>,
        source: TomlError,
    },
    InvalidKey {
        section: &'static str,
        key: String,
    },
}

impl ConfigError {
    pub fn io(path: PathBuf, source: io::Error) -> Self {
        Self::Io { path, source }
    }

    pub fn from_parse_error(source: TomlError) -> Self {
        Self::Parse { path: None, source }
    }

    pub fn from_parse_error_with_path(path: &Path, source: TomlError) -> Self {
        Self::Parse {
            path: Some(path.to_path_buf()),
            source,
        }
    }

    pub fn char_from_key(section: &'static str, key: &str) -> Result<char, Self> {
        let mut chars = key.chars();
        match (chars.next(), chars.next()) {
            (Some(ch), None) => Ok(ch),
            _ => Err(Self::InvalidKey {
                section,
                key: key.to_string(),
            }),
        }
    }
}

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

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io { path, .. } => {
                write!(f, "failed to read config at {}", path.display())
            }
            ConfigError::Parse {
                path: Some(path), ..
            } => {
                write!(f, "failed to parse config at {}", path.display())
            }
            ConfigError::Parse { path: None, .. } => write!(f, "failed to parse config"),
            ConfigError::InvalidKey { section, key } => write!(
                f,
                "invalid key \"{key}\" in {section}; use single-character keys"
            ),
        }
    }
}

impl std::error::Error for FnormError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FnormError::FileNotFound { source, .. } | FnormError::RenameError { source, .. } => {
                Some(source)
            }
            FnormError::TargetExists { .. } => None,
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Io { source, .. } => Some(source),
            ConfigError::Parse { source, .. } => Some(source),
            ConfigError::InvalidKey { .. } => None,
        }
    }
}
