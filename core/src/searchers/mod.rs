mod file;
mod stdin;

pub use file::FileSearcher;
pub use stdin::StdinSearcher;

pub trait Searcher {
    type Output;

    fn search(&self, pattern: String) -> Self::Output;
}
