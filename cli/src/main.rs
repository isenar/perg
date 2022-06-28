mod args;

use args::Args;

use perg::config::{Config, OutputConfig, SearchConfig};
use perg::output::Printer;
use perg::{is_stdin_piped, search};

use clap::Parser;
use perg::searchers::{Searcher, StdinSearcher};

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Self {
            pattern: args.pattern,
            path: args.path.unwrap_or_else(|| ".".into()),
            search: SearchConfig {
                case_insensitive: args.ignore_case,
                invert_match: args.invert_match,
            },
            output: OutputConfig {
                only_file_names: args.files_with_matches,
            },
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();
    let config = Config::from(args);

    let search_results = if is_stdin_piped() {
        let stdin_searcher = StdinSearcher::new(&config.search);
        stdin_searcher.search(&config.pattern)?
    } else {
        search(config.pattern, &config.path, &config.search)?
    };

    let printer = Printer::new(&config.output);

    printer.print(search_results.iter());

    Ok(())
}
