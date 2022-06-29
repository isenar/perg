use clap::Parser;

use perg::config::{Config, OutputConfig, SearchConfig};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    /// A regular expression used for the search.
    pub pattern: String,
    /// A path to a file or directory to search for.
    /// If none is given, perg will search for the <PATTERN> recursively in current directory
    pub path: Option<PathBuf>,

    #[clap(short = 'l')]
    /// Print only the names of the files matching the provided criteria.
    /// (Disabled by default)
    pub files_with_matches: bool,

    #[clap(short = 'i')]
    /// Matches case insensitively. (Disabled by default)
    pub ignore_case: bool,

    #[clap(short = 'S')]
    /// Follow symbolic links when traversing the directories. (Disabled by default)
    pub follow_symlinks: bool,

    #[clap(short = 'w')]
    /// Only match whole words. (Disabled by default)
    pub whole_words: bool,
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Self {
            pattern: args.pattern,
            path: args.path.unwrap_or_else(|| ".".into()),
            search: SearchConfig {
                case_insensitive: args.ignore_case,
                follow_symlinks: args.follow_symlinks,
                exact_match: args.whole_words,
            },
            output: OutputConfig {
                only_file_names: args.files_with_matches,
            },
        }
    }
}
