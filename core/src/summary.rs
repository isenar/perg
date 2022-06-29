use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct MatchingLineData {
    pub line_number: Option<usize>,
    pub line: String,
    pub matching_pattern_idx: Vec<PatternIndices>,
}

#[derive(Debug, PartialEq)]
pub struct PatternIndices {
    pub start: usize,
    pub end: usize,
}

impl Display for MatchingLineData {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        // TODO: fix
        let start = self.matching_pattern_idx[0].start;
        let end = self.matching_pattern_idx[0].end;
        let line_num = self
            .line_number
            .map(|num| format!("{num}:"))
            .unwrap_or_default();
        let before = &self.line[0..start];
        let colored_match = &self.line[start..end];
        let after = &self.line[end..];

        write!(f, "{line_num}{before}{colored_match}{after}")
    }
}

#[derive(Debug, Default)]
pub struct SearchSummary(pub BTreeMap<String, Vec<MatchingLineData>>);

impl SearchSummary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_line_data(&mut self, file: String, line_data: MatchingLineData) {
        self.0.entry(file).or_default().push(line_data);
    }

    pub fn merge(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}

impl IntoIterator for SearchSummary {
    type Item = (String, Vec<MatchingLineData>);
    type IntoIter = std::collections::btree_map::IntoIter<String, Vec<MatchingLineData>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
