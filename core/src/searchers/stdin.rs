use crate::searchers::Searcher;
use crate::{MatchingLineData, PatternIndices, SearchConfig, SearchSummary};

use itertools::Itertools;
use regex::Regex;

use std::error::Error;
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
    type Output = Result<Vec<SearchSummary>, Box<dyn Error>>;

    fn search(&self, pattern: &str) -> Self::Output {
        let matcher = Regex::new(pattern)?;
        let mut search_summary = SearchSummary::new("<stdin>");
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
                search_summary.add_line_data(MatchingLineData {
                    line_number: 0,
                    line,
                    matching_pattern_idx: matching_indices,
                });
            }
        }

        Ok(vec![search_summary])
    }
}
