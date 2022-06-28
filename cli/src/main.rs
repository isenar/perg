mod args;

use perg::search;

use args::Args;

use clap::Parser;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Args: {args:?}");

    let path = args.path.unwrap_or_else(|| ".".into());
    let search_results = search(args.pattern, path)?;

    for result in search_results {
        println!("{result}");
    }

    Ok(())
}
