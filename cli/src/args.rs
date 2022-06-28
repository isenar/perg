use clap::Parser;

use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    /// A regular expression used for the search.
    pub pattern: String,
    /// A path to a file or directory to search for.
    /// When provided, this overrides the -g (glob) arg.
    pub path: Option<PathBuf>,

    #[clap(short = 'i', default_value = "false")]
    /// Matches case insensitively
    pub ignore_case: bool,

    #[clap(short = 'v', default_value = "false")]
    /// Inverts matching criteria. Shows lines that do not match provided patterns.
    pub invert_match: bool,

    #[clap(short = 'g')]
    /// Only include the files matching the provided GLOB pattern
    pub glob: Option<String>,
}
