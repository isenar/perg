#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Bad regex for pattern '{1}': {0}")]
    BadRegex(regex::Error, String),

    #[error("WalkDir error: {0}")]
    WalkDir(#[from] walkdir::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
