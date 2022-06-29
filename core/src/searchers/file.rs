use crate::config::SearchConfig;
use crate::matcher::Matcher;
use crate::searchers::Searcher;
use crate::summary::{MatchedLine, SearchSummary};
use crate::{skip_fail, Result};

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

        // skip handling symlinks when the flag is disabled
        if self.path.is_symlink() && !self.config.follow_symlinks {
            return Ok(search_summary);
        }

        let lines = read_lines(&self.path)?;

        for (line_number, line) in lines.enumerate() {
            let line = skip_fail!(line);
            let matching_indices = matcher.find_matches(&line);

            if !matching_indices.is_empty() {
                search_summary.add_line_data(
                    self.path.to_string_lossy().as_ref(),
                    MatchedLine {
                        line_number: Some(line_number + 1),
                        line,
                        matches_indicies: matching_indices,
                    },
                );
            }
        }

        Ok(search_summary)
    }
}

fn read_lines(path: impl AsRef<Path>) -> Result<Lines<BufReader<File>>> {
    let path = path.as_ref();
    let file = File::open(path).map_err(|e| crate::Error::Io(path.into(), e))?;

    Ok(BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::summary::MatchIndices;
    use maplit::hashmap;

    fn expected_summary(file: &str) -> SearchSummary {
        SearchSummary::from_map(hashmap! {
            file.to_owned() => vec![
                MatchedLine {
                    line_number: Some(1),
                    line: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_owned(),
                    matches_indicies: vec![MatchIndices { start: 12, end: 17 }],
                },
                MatchedLine {
                    line_number: Some(2),
                    line: "Aenean commodo ligula eget dolor.".to_owned(),
                    matches_indicies: vec![MatchIndices { start: 27, end: 32 }],
                },
            ]
        })
    }

    #[test]
    fn search_in_test_data() {
        let file = "test_data/lorem.txt";
        let config = SearchConfig::default();
        let searcher = SingleFileSearcher::new(file.into(), &config);
        let matcher = Matcher::try_create("dolor", &config).expect("Failed to build matcher");
        let search_summary = searcher.search(&matcher).expect("Searcher failed");
        let expected_summary = expected_summary(file);

        assert_eq!(search_summary, expected_summary);
    }

    #[test]
    fn search_in_symlink_with_feature_disabled_yields_nothing() {
        let file = "test_data/lorem_symlink";
        let config = SearchConfig::default();
        let searcher = SingleFileSearcher::new(file.into(), &config);
        let matcher = Matcher::try_create("dolor", &config).expect("Failed to build matcher");
        let search_summary = searcher.search(&matcher).expect("Searcher failed");
        let expected_summary = SearchSummary::new();

        assert_eq!(search_summary, expected_summary);
    }

    #[test]
    fn search_in_symlink_yields_linked_file_contents() {
        let file = "test_data/lorem_symlink";
        let config = SearchConfig {
            follow_symlinks: true,
            ..SearchConfig::default()
        };
        let searcher = SingleFileSearcher::new(file.into(), &config);
        let matcher = Matcher::try_create("dolor", &config).expect("Failed to build matcher");
        let search_summary = searcher.search(&matcher).expect("Searcher failed");
        let expected_summary = expected_summary(file);

        assert_eq!(search_summary, expected_summary);
    }
}
