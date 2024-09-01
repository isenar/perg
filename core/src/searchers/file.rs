use crate::config::SearchConfig;
use crate::matcher::Matcher;
use crate::searchers::{summarize, Searcher};
use crate::summary::SearchSummary;
use crate::utils::read_lines;
use crate::Result;

use std::path::PathBuf;

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
        // skip handling symlinks if the flag is disabled
        if self.path.is_symlink() && !self.config.follow_symlinks {
            return Ok(SearchSummary::empty());
        }

        let lines = read_lines(&self.path)?.map_while(std::io::Result::ok);
        let path = self.path.to_string_lossy();
        let search_summary = summarize(matcher, path, lines);

        Ok(search_summary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::summary::{MatchIndices, MatchedLine};
    use maplit::hashmap;

    fn expected_summary(file: &str) -> SearchSummary {
        SearchSummary::from_map(hashmap! {
            file.to_owned() => vec![
                MatchedLine {
                    line_number: 1,
                    line: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_owned(),
                    matches_indices: vec![MatchIndices { start: 12, end: 17 }],
                },
                MatchedLine {
                    line_number: 2,
                    line: "Aenean commodo ligula eget dolor.".to_owned(),
                    matches_indices: vec![MatchIndices { start: 27, end: 32 }],
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
        let expected_summary = SearchSummary::empty();

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
