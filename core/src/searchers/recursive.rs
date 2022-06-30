use crate::config::SearchConfig;
use crate::matcher::Matcher;
use crate::searchers::{Searcher, SingleFileSearcher};
use crate::summary::SearchSummary;
use crate::Result;

use std::path::PathBuf;

#[derive(Debug)]
pub struct RecursiveSearcher<'conf> {
    path: PathBuf,
    config: &'conf SearchConfig,
}

impl<'conf> RecursiveSearcher<'conf> {
    pub fn new(path: PathBuf, config: &'conf SearchConfig) -> Self {
        Self { path, config }
    }
}

impl<'conf> Searcher for RecursiveSearcher<'conf> {
    fn search(&self, pattern: &Matcher) -> Result<SearchSummary> {
        let mut summary = SearchSummary::empty();

        for entry in walkdir::WalkDir::new(&self.path) {
            let path = entry?.into_path();

            if path.is_file() {
                let single_file_searcher = SingleFileSearcher::new(path, self.config);
                let result = single_file_searcher.search(pattern)?;
                summary.merge(result);
            }
        }

        Ok(summary)
    }
}
