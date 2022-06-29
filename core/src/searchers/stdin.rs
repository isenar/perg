use crate::matcher::Matcher;
use crate::searchers::Searcher;
use crate::summary::{MatchedLine, SearchSummary};
use crate::{skip_fail, Result};

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
        let mut search_summary = SearchSummary::new();
        let lines = std::io::stdin().lock().lines();

        for line in lines {
            // skip lines containing non-UTF8 characters (like binary data)
            let line = skip_fail!(line);
            let matching_indices = matcher.find_matches(&line);

            if !matching_indices.is_empty() {
                search_summary.add_line_data(
                    "<stdin>",
                    MatchedLine {
                        line_number: None,
                        line,
                        matches_indicies: matching_indices,
                    },
                );
            }
        }

        Ok(search_summary)
    }
}
