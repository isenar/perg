use crate::matcher::Matcher;
use crate::searchers::{summarize, Searcher};
use crate::summary::SearchSummary;
use crate::Result;

use std::io::BufRead;

#[derive(Debug, Default)]
pub struct StdinSearcher;

impl StdinSearcher {
    pub fn new() -> Self {
        Self
    }
}

impl Searcher for StdinSearcher {
    fn search(&self, matcher: &Matcher) -> Result<SearchSummary> {
        // filter out non-UTF8 lines from stdin
        let lines = std::io::stdin()
            .lock()
            .lines()
            .map_while(std::io::Result::ok);
        let search_summary = summarize(matcher, "<stdin>", lines);

        Ok(search_summary)
    }
}
