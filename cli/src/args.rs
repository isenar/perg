use clap::Parser;

use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    /// A regular expression used for the search.
    pub pattern: String,
    /// A path to a file or directory to search for.
    /// When provided, this overrides the -g (glob) arg.
    pub path: Option<PathBuf>,

    #[clap(short = 'l')]
    /// Print only the names of the files matching the provided criteria.
    pub files_with_matches: bool,

    #[clap(short = 'i')]
    /// Matches case insensitively
    pub ignore_case: bool,

    #[clap(short = 'v')]
    /// Inverts matching criteria. Shows lines that do not match provided patterns.
    pub invert_match: bool,

    #[clap(short = 'S')]
    pub follow_symlinks: bool,

    #[clap(short = 'g')]
    /// Only include the files matching the provided GLOB pattern
    pub glob: Option<String>,
}
