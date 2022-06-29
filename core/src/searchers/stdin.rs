use crate::config::SearchConfig;
use crate::searchers::Searcher;
use crate::summary::{MatchingLineData, PatternIndices, SearchSummary};
use crate::Result;
use itertools::Itertools;
use regex::Regex;

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
    fn search(&self, pattern: &str) -> Result<SearchSummary> {
        let matcher = Regex::new(pattern)?;
        let mut search_summary = SearchSummary::new();
        let lines = std::io::stdin().lock().lines();

        for line in lines {
            let line = line?;

            let matching_indices = matcher
                .find_iter(&line)
                .map(|mat| PatternIndices {
                    start: mat.start(),
                    end: mat.end(),
                })
                .collect_vec();

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
