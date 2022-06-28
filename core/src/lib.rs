pub mod config;
pub mod output;
pub mod searchers;

use crate::config::SearchConfig;
use crate::searchers::{Searcher, SingleFileSearcher};
use itertools::Itertools;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug)]
pub struct MatchingLineData {
    line_number: usize,
    line: String,
    matching_pattern_idx: Vec<PatternIndices>,
}

impl MatchingLineData {
    pub fn new(line_number: usize, line: String) -> Self {
        Self {
            line_number,
            line,
            matching_pattern_idx: vec![],
        }
    }

    pub fn has_matches(&self) -> bool {
        !self.matching_pattern_idx.is_empty()
    }

    pub fn add_pattern_indices(&mut self, start: usize, end: usize) {
        self.matching_pattern_idx
            .push(PatternIndices { start, end });
    }
}

#[derive(Debug)]
pub struct PatternIndices {
    start: usize,
    end: usize,
}

impl Display for MatchingLineData {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if self.has_matches() {
            // TODO: fix
            let start = self.matching_pattern_idx[0].start;
            let end = self.matching_pattern_idx[0].end;

            let line_num = self.line_number;
            let before = &self.line[0..start];
            let colored_match = &self.line[start..end];
            let after = &self.line[end..];

            write!(f, "{line_num}:{before}{colored_match}{after}")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct SearchSummary {
    file: String,
    lines_matching: Vec<MatchingLineData>,
}

impl SearchSummary {
    pub fn new(file: impl ToString) -> Self {
        Self {
            file: file.to_string(),
            lines_matching: vec![],
        }
    }

    pub fn add_line_data(&mut self, line_data: MatchingLineData) {
        self.lines_matching.push(line_data);
    }

    pub fn is_empty(&self) -> bool {
        self.lines_matching.is_empty()
    }
}

impl Display for SearchSummary {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.file)?;

        for line_data in &self.lines_matching {
            writeln!(f)?;
            write!(f, "{line_data}")?;
        }

        Ok(())
    }
}

pub fn search(
    pattern: String,
    path: impl Into<PathBuf>,
    config: &SearchConfig,
) -> Result<Vec<SearchSummary>, Box<dyn Error>> {
    let path = path.into();

    if path.is_file() {
        let single_file_searcher = SingleFileSearcher::new(path, config);

        Ok(single_file_searcher
            .search(&pattern)?
            .map(|ss| vec![ss])
            .unwrap_or_default())
    } else {
        // else it's a directory
        let summaries = walkdir::WalkDir::new(path)
            .into_iter()
            .map(|dir| dir.unwrap().into_path())
            .filter_map(|entry_path| {
                if entry_path.is_file() {
                    let single_file_searcher = SingleFileSearcher::new(entry_path, config);
                    single_file_searcher.search(&pattern).ok().flatten() // FIXME
                } else {
                    None
                }
            })
            .collect_vec();

        Ok(summaries)
    }
}

pub fn is_stdin_piped() -> bool {
    atty::isnt(atty::Stream::Stdin)
}
