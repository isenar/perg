mod args;

use perg::search;

use args::Args;

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let path = args.path.unwrap_or_else(|| ".".into());
    let search_results = search(args.pattern, path)?;

    for result in search_results {
        println!("{result}");
    }

    Ok(())
}
