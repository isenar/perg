use crate::config::SearchConfig;
use crate::searchers::Searcher;
use crate::summary::{MatchingLineData, PatternIndices, SearchSummary};
use crate::Result;
use itertools::Itertools;
use regex::Regex;

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;

#[derive(Debug)]
pub struct SingleFileSearcher<'conf> {
    path: PathBuf,
    _config: &'conf SearchConfig,
}

impl<'conf> SingleFileSearcher<'conf> {
    pub fn new(path: PathBuf, config: &'conf SearchConfig) -> Self {
        Self {
            path,
            _config: config,
        }
    }
}

impl<'conf> Searcher for SingleFileSearcher<'conf> {
    fn search(&self, pattern: &str) -> Result<SearchSummary> {
        let matcher = Regex::new(pattern)?;
        let mut search_summary = SearchSummary::new();
        let lines = read_input_lines(&self.path)?;

        for (line_number, line) in lines.enumerate() {
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
                    self.path.to_string_lossy().to_string(),
                    MatchingLineData {
                        line_number: Some(line_number + 1),
                        line,
                        matching_pattern_idx: matching_indices,
                    },
                );
            }
        }

        Ok(search_summary)
    }
}

type InputLines = Lines<BufReader<File>>;

fn read_input_lines(path: &PathBuf) -> Result<InputLines> {
    let file = File::open(path)?;

    Ok(BufReader::new(file).lines())
}
