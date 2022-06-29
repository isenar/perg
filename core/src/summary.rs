use colored::Colorize;

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use vec1::Vec1;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct MatchingLineData {
    pub line_number: Option<usize>,
    pub line: String,
    pub matches_idxs: Vec1<PatternIndices>,
}

#[derive(Debug, PartialEq)]
pub struct PatternIndices {
    pub start: usize,
    pub end: usize,
}

impl Display for MatchingLineData {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        // FIXME
        let start = self.matches_idxs[0].start;
        let end = self.matches_idxs[0].end;
        let line_num = self
            .line_number
            .map(|num| format!("{num}:"))
            .unwrap_or_default()
            .bold()
            .yellow();
        let before = &self.line[0..start];
        let colored_match = &self.line[start..end].black().on_bright_yellow();
        let after = &self.line[end..];

        write!(f, "{line_num}{before}{colored_match}{after}")
    }
}

#[derive(Debug, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub struct SearchSummary(BTreeMap<String, Vec<MatchingLineData>>);

impl SearchSummary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_line_data(&mut self, filename: &str, line_data: MatchingLineData) {
        let elem = self.0.get_mut(filename);
        match elem {
            Some(data) => data.push(line_data),
            None => {
                self.0.insert(filename.to_owned(), vec![line_data]);
            }
        }
    }

    pub fn merge(&mut self, other: Self) {
        self.0.extend(other.0);
    }

    pub fn files(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(|s| s as &str)
    }
}

impl IntoIterator for SearchSummary {
    type Item = (String, Vec<MatchingLineData>);
    type IntoIter = std::collections::btree_map::IntoIter<String, Vec<MatchingLineData>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
