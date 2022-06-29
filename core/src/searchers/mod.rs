mod file;
mod recursive;
mod stdin;

pub use file::SingleFileSearcher;
pub use recursive::RecursiveSearcher;
pub use stdin::StdinSearcher;

use crate::summary::SearchSummary;
use crate::Result;

pub trait Searcher {
    fn search(&self, pattern: &str) -> Result<SearchSummary>;
}
