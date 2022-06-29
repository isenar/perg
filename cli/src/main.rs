mod args;

use args::Args;
use clap::Parser;
use perg::config::{Config, OutputConfig, SearchConfig};
use perg::is_stdin_piped;
use perg::output::Printer;
use perg::searchers::{RecursiveSearcher, Searcher, SingleFileSearcher, StdinSearcher};

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

fn main() -> perg::Result<()> {
    let args: Args = Args::parse();
    let config = Config::from(args);
    let searcher = select_searcher(&config);
    let search_results = searcher.search(&config.pattern)?;
    let printer = Printer::new(&config.output);

    printer.print(search_results);

    Ok(())
}

fn select_searcher(config: &Config) -> Box<dyn Searcher + '_> {
    if is_stdin_piped() {
        Box::new(StdinSearcher::new(&config.search))
    } else if config.path.is_file() {
        Box::new(SingleFileSearcher::new(config.path.clone(), &config.search))
    } else {
        Box::new(RecursiveSearcher::new(config.path.clone(), &config.search))
    }
}
