use colored::Colorize;

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct MatchedLine {
    pub line_number: Option<usize>,
    pub line: String,
    pub matches_indicies: Vec<MatchIndices>,
}

#[derive(Debug, PartialEq)]
pub struct MatchIndices {
    pub start: usize,
    pub end: usize,
}

impl Display for MatchedLine {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let line_num = self
            .line_number
            .map(|num| format!("{num}:"))
            .unwrap_or_default()
            .bold()
            .yellow();

        write!(f, "{line_num}{}", self.line)
    }
}

#[derive(Debug, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub struct SearchSummary(BTreeMap<String, Vec<MatchedLine>>);

impl SearchSummary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_line_data(&mut self, filename: &str, line_data: MatchedLine) {
        self.0.entry(filename.into()).or_default().push(line_data);
    }

    pub fn merge(&mut self, other: Self) {
        self.0.extend(other.0);
    }

    /// Return an iterator over file names
    pub fn files(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(|s| s as &str)
    }

    #[cfg(test)]
    pub fn from_map(map: BTreeMap<String, Vec<MatchedLine>>) -> Self {
        Self(map)
    }
}

impl IntoIterator for SearchSummary {
    type Item = (String, Vec<MatchedLine>);
    type IntoIter = std::collections::btree_map::IntoIter<String, Vec<MatchedLine>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
