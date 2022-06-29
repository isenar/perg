#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Bad regex: {0}")]
    BadRegex(#[from] regex::Error),

    #[error("WalkDir error: {0}")]
    WalkDir(#[from] walkdir::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
