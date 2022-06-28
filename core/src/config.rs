use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub path: PathBuf,
    pub search: SearchConfig,
    pub output: OutputConfig,
}

#[derive(Debug)]
pub struct SearchConfig {
    pub case_insensitive: bool,
    pub invert_match: bool,
}

#[derive(Debug)]
pub struct OutputConfig {
    pub only_file_names: bool,
}