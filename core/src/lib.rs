use colored::Colorize;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;

// TODO: Show multiple matches in one line on the same line in the output
#[derive(Debug)]
pub struct MatchingLineData {
    line_number: usize,
    line: String,
    start_idx: usize,
    end_idx: usize,
}

impl Display for MatchingLineData {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let line_num = self.line_number;
        let before = &self.line[0..self.start_idx];
        let colored_match = &self.line[self.start_idx..self.end_idx].bold().red();
        let after = &self.line[self.end_idx..];

        writeln!(f, "{line_num}:{before}{colored_match}{after}",)
    }
}

#[derive(Debug)]
pub struct SearchResult {
    file: String,
    lines_matching: Vec<MatchingLineData>,
}

impl SearchResult {
    pub fn new(file: impl ToString) -> Self {
        Self {
            file: file.to_string(),
            lines_matching: vec![],
        }
    }
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if !self.lines_matching.is_empty() {
            writeln!(f, "{}", self.file)?;

            for line_data in &self.lines_matching {
                write!(f, "{line_data}")?;
            }
        }

        Ok(())
    }
}

pub type InputLines = Lines<BufReader<File>>;

pub fn read_input_lines(path: PathBuf) -> Result<InputLines, Box<dyn Error>> {
    let file = File::open(path)?;

    Ok(BufReader::new(file).lines())
}

pub fn search(
    pattern: impl AsRef<str>,
    path: impl Into<PathBuf>,
) -> Result<Vec<SearchResult>, Box<dyn Error>> {
    let pattern = pattern.as_ref();
    let matcher = regex::Regex::new(pattern)?;
    let path = path.into();

    if path.is_file() {
        let mut search_result = SearchResult::new(path.to_string_lossy());
        let lines = read_input_lines(path)?;

        for (line_number, line) in lines.enumerate() {
            let line = line?;

            for matching_text in matcher.find_iter(&line) {
                let matching_data = MatchingLineData {
                    line_number,
                    line: line.clone(),
                    start_idx: matching_text.start(),
                    end_idx: matching_text.end(),
                };

                search_result.lines_matching.push(matching_data);
            }
        }

        Ok(vec![search_result])
    } else {
        todo!("Directory handling not yet implemented!")
    }
}
