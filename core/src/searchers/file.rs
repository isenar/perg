use crate::config::SearchConfig;
use crate::matcher::Matcher;
use crate::searchers::Searcher;
use crate::summary::{MatchingLineData, SearchSummary};
use crate::Result;

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};

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
                    self.path.to_str().unwrap(), // FIXME
                    MatchingLineData {
                        line_number: Some(line_number + 1),
                        line,
                        matches_idxs: matching_indices,
                    },
                );
            }
        }

        Ok(search_summary)
    }
}

type InputLines = Lines<BufReader<File>>;

fn read_input_lines(path: impl AsRef<Path>) -> Result<InputLines> {
    let file = File::open(path)?;

    Ok(BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::summary::PatternIndices;

    #[test]
    fn search_in_test_data() {
        let file = "test_data/lorem.txt";
        let config = SearchConfig::default();
        let searcher = SingleFileSearcher::new(file.into(), &config);
        let matcher = Matcher::build("in", &config).expect("Failed to build matcher");
        let search_summary = searcher.search(&matcher).expect("Searcher failed");
        let mut expected_summary = SearchSummary::new();
        expected_summary.add_line_data(
            file,
            MatchingLineData {
                line_number: Some(1),
                line: "Lorem ipsum dolor sit amet, consectetur adipiscing elit,".to_string(),
                matches_idxs: vec![PatternIndices { start: 47, end: 49 }],
            },
        );

        assert_eq!(search_summary, expected_summary);
    }
}
