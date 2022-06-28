pub mod config;
pub mod printer;
pub mod searchers;

use regex::Regex;

use crate::config::SearchConfig;
use itertools::Itertools;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::ops::Not;
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

pub type InputLines = Lines<BufReader<File>>;

pub fn read_input_lines(path: &PathBuf) -> Result<InputLines, Box<dyn Error>> {
    let file = File::open(path)?;

    Ok(BufReader::new(file).lines())
}

pub fn search(
    pattern: impl AsRef<str>,
    path: impl Into<PathBuf>,
    _config: &SearchConfig,
) -> Result<Vec<SearchSummary>, Box<dyn Error>> {
    let matcher = Regex::new(pattern.as_ref())?;
    let path = path.into();

    if path.is_file() {
        Ok(search_file(&path, &matcher)?
            .map(|ss| vec![ss])
            .unwrap_or_default())
    } else {
        // else it's a directory
        let summaries = walkdir::WalkDir::new(path)
            .into_iter()
            .map(|dir| dir.unwrap().into_path())
            .filter_map(|entry_path| {
                if entry_path.is_file() {
                    search_file(&entry_path, &matcher).ok().flatten() // FIXME
                } else {
                    None
                }
            })
            .collect_vec();

        Ok(summaries)
    }
}

fn search_file(path: &PathBuf, matcher: &Regex) -> Result<Option<SearchSummary>, Box<dyn Error>> {
    let mut search_summary = SearchSummary::new(path.display());
    let lines = read_input_lines(path)?;

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

pub fn is_stdin_piped() -> bool {
    atty::isnt(atty::Stream::Stdin)
}
