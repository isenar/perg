use crate::Result;

use std::fs::File;
use std::io::{BufRead, BufReader, IsTerminal, Lines};
use std::path::Path;

/// Helper function to check whether perg is currently being piped to.
pub fn is_stdin_piped() -> bool {
    !std::io::stdin().is_terminal()
}

pub fn read_lines(path: impl AsRef<Path>) -> Result<Lines<BufReader<File>>> {
    let path = path.as_ref();
    let file = File::open(path).map_err(|e| crate::Error::Io(path.into(), e))?;

    Ok(BufReader::new(file).lines())
}
