use crate::searchers::Searcher;
use crate::{MatchingLineData, PatternIndices, SearchConfig, SearchSummary};
use itertools::Itertools;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::ops::Not;
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
    type Output = Result<Option<SearchSummary>, Box<dyn Error>>;

    fn search(&self, pattern: &str) -> Self::Output {
        let matcher = Regex::new(pattern)?;
        let mut search_summary = SearchSummary::new(self.path.display());
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
                search_summary.add_line_data(MatchingLineData {
                    line_number: line_number + 1,
                    line,
                    matching_pattern_idx: matching_indices,
                });
            }
        }

        Ok(search_summary
            .lines_matching
            .is_empty()
            .not()
            .then(|| search_summary))
    }
}

type InputLines = Lines<BufReader<File>>;

pub fn read_input_lines(path: &PathBuf) -> Result<InputLines, Box<dyn Error>> {
    let file = File::open(path)?;

    Ok(BufReader::new(file).lines())
}
