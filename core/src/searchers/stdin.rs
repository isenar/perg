use crate::config::SearchConfig;
use crate::matcher::Matcher;
use crate::searchers::Searcher;
use crate::summary::{MatchingLineData, SearchSummary};
use crate::Result;

use std::io::BufRead;

#[derive(Debug)]
pub struct StdinSearcher<'conf> {
    _config: &'conf SearchConfig,
}

impl<'conf> StdinSearcher<'conf> {
    pub fn new(config: &'conf SearchConfig) -> Self {
        Self { _config: config }
    }
}

impl<'conf> Searcher for StdinSearcher<'conf> {
    fn search(&self, matcher: &Matcher) -> Result<SearchSummary> {
        let mut search_summary = SearchSummary::new();
        let lines = std::io::stdin().lock().lines();

        for line in lines {
            let line = line?;

            let matching_indices = matcher.find_matches(&line);

            if !matching_indices.is_empty() {
                search_summary.add_line_data(
                    "<stdin>".to_string(),
                    MatchingLineData {
                        line_number: None,
                        line,
                        matching_pattern_idx: matching_indices,
                    },
                );
            }
        }

        Ok(search_summary)
    }
}
