use crate::error::Error;

pub mod config;
pub mod error;
pub mod output;
pub mod searchers;
mod summary;

pub type Result<T> = std::result::Result<T, Error>;

pub fn is_stdin_piped() -> bool {
    atty::isnt(atty::Stream::Stdin)
}
