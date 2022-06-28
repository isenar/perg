mod file;
mod stdin;

pub use file::SingleFileSearcher;
pub use stdin::StdinSearcher;

pub trait Searcher {
    type Output;

    fn search(&self, pattern: &str) -> Self::Output;
}
