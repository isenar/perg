use crate::error::Error;

pub mod config;
pub mod error;
pub mod matcher;
pub mod printer;
pub mod searchers;
pub mod summary;
pub mod utils;

pub type Result<T> = std::result::Result<T, Error>;
