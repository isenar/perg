use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Bad regex for pattern '{1}': {0}")]
    BadRegex(regex::Error, String),

    #[error("WalkDir error: {0}")]
    WalkDir(#[from] walkdir::Error),

    #[error("IO error occurred for file {0}: {1}")]
    Io(PathBuf, std::io::Error),
}
