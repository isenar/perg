use crate::config::SearchConfig;
use crate::matcher::Matcher;
use crate::searchers::Searcher;
use crate::summary::{MatchingLineData, SearchSummary};
use crate::Result;

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;

#[derive(Debug)]
pub struct SingleFileSearcher<'conf> {
    path: PathBuf,
    config: &'conf SearchConfig,
}

impl<'conf> SingleFileSearcher<'conf> {
    pub fn new(path: PathBuf, config: &'conf SearchConfig) -> Self {
        Self { path, config }
    }
}

impl<'conf> Searcher for SingleFileSearcher<'conf> {
    fn search(&self, matcher: &Matcher) -> Result<SearchSummary> {
        let mut search_summary = SearchSummary::new();

        if self.path.is_symlink() && !self.config.follow_symlinks {
            return Ok(search_summary);
        }

        let lines = read_input_lines(&self.path)?;

        for (line_number, line) in lines.enumerate() {
            let line = line?;

            let matching_indices = matcher.find_matches(&line);

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
