mod args;

use args::Args;
use clap::Parser;
use perg::config::Config;
use perg::is_stdin_piped;
use perg::matcher::Matcher;
use perg::output::Printer;
use perg::searchers::{RecursiveSearcher, Searcher, SingleFileSearcher, StdinSearcher};

fn main() -> perg::Result<()> {
    let args = Args::parse();
    let config = Config::from(args);
    let matcher = Matcher::build(&config.pattern, &config.search)?;
    let searcher = select_searcher(&config);
    let search_results = searcher.search(&matcher)?;
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
