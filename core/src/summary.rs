use colored::Colorize;
use maplit::hashmap;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct MatchedLine {
    pub line_number: usize,
    pub line: String,
    pub matches_indices: Vec<MatchIndices>,
}

#[derive(Debug, PartialEq)]
pub struct MatchIndices {
    pub start: usize,
    pub end: usize,
}

impl Display for MatchedLine {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let line_num = format!("{}", self.line_number).bold().yellow();

        write!(f, "{line_num}:{}", self.line)
    }
}

#[derive(Debug, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub struct SearchSummary(HashMap<String, Vec<MatchedLine>>);

impl SearchSummary {
    pub fn new(filename: String, matches: Vec<MatchedLine>) -> Self {
        Self(hashmap! { filename => matches })
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn merge(&mut self, other: Self) {
        self.0.extend(other.0);
    }

    pub fn files(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(|s| s.as_ref())
    }

    #[cfg(test)]
    pub fn from_map(map: HashMap<String, Vec<MatchedLine>>) -> Self {
        Self(map)
    }
}

impl IntoIterator for SearchSummary {
    type Item = (String, Vec<MatchedLine>);
    type IntoIter = std::collections::hash_map::IntoIter<String, Vec<MatchedLine>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
