use crate::error::Error;

pub mod config;
pub mod error;
pub mod matcher;
pub mod output;
pub mod searchers;
pub mod summary;

pub type Result<T> = std::result::Result<T, Error>;

/// Helper function to check whether perg is currently being piped to
pub fn is_stdin_piped() -> bool {
    atty::isnt(atty::Stream::Stdin)
}
