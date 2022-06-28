use regex::Regex;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
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
}

impl Display for SearchSummary {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if !self.lines_matching.is_empty() {
            writeln!(f, "{}", self.file)?;

            for line_data in &self.lines_matching {
                writeln!(f, "{line_data}")?;
            }
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
) -> Result<Vec<SearchSummary>, Box<dyn Error>> {
    let matcher = Regex::new(pattern.as_ref())?;
    let path = path.into();

    if path.is_file() {
        let single_summary = search_file(&path, &matcher)?;

        Ok(vec![single_summary])
    } else if path.is_dir() {
        let mut summaries = vec![];

        let dirs = walkdir::WalkDir::new(path);

        for entry in dirs {
            let entry_path = entry?.into_path();
            if entry_path.is_file() {
                let summary = search_file(&entry_path, &matcher)
                    .map_err(|e| format!("Failed for {entry_path:?}: {e}"))?;

                summaries.push(summary);
            }
        }

        Ok(summaries)
    } else {
        todo!()
    }
}

fn search_file(path: &PathBuf, matcher: &Regex) -> Result<SearchSummary, Box<dyn Error>> {
    let mut search_summary = SearchSummary::new(path.to_string_lossy());
    let lines = read_input_lines(path)?;

    for (line_number, line) in lines.enumerate() {
        let line = line?;
        let mut matching_data = MatchingLineData::new(line_number, line.clone());

        for matching_text in matcher.find_iter(&line) {
            matching_data.add_pattern_indices(matching_text.start(), matching_text.end());
        }

        if matching_data.has_matches() {
            search_summary.add_line_data(matching_data);
        }
    }

    Ok(search_summary)
}
