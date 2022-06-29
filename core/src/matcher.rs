use crate::config::SearchConfig;
use crate::summary::MatchIndices;
use crate::Result;
use regex::{Regex, RegexBuilder};
use std::borrow::Cow;

/// A matcher that is used to find
#[derive(Debug)]
pub struct Matcher(Regex);

impl Matcher {
    /// Create a matcher based on provided `SearchConfig`.
    /// Creation will fail if `pattern` is an invalid regex.
    pub fn try_create(pattern: &str, config: &SearchConfig) -> Result<Self> {
        let pattern = if config.exact_match {
            Cow::from(format!("\\b{pattern}\\b"))
        } else {
            Cow::from(pattern)
        };

        let regex = RegexBuilder::new(&pattern)
            .case_insensitive(config.case_insensitive)
            .build()
            .map_err(|e| crate::Error::BadRegex(e, pattern.to_string()))?;

        Ok(Self(regex))
    }

    /// Find all the indices (start, end) of the words matching the pattern within `line`.
    pub fn find_matches(&self, line: &str) -> Vec<MatchIndices> {
        self.0
            .find_iter(line)
            .map(|mat| MatchIndices {
                start: mat.start(),
                end: mat.end(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use test_case::test_case;

    fn find_matching_indices(
        pattern: &str,
        line: &str,
        search_config: &SearchConfig,
    ) -> Vec<MatchIndices> {
        Matcher::try_create(pattern, search_config)
            .expect("Failed to build matcher")
            .find_matches(line)
    }

    fn expected_indices(expected: Vec<(usize, usize)>) -> Vec<MatchIndices> {
        expected
            .into_iter()
            .map(|(start, end)| MatchIndices { start, end })
            .collect()
    }

    #[test]
    fn fails_on_bad_regex_pattern() {
        let invalid_pattern = "[";
        let config = SearchConfig::default();
        let matcher = Matcher::try_create(invalid_pattern, &config);

        assert_matches!(matcher, Err(crate::Error::BadRegex(_, pattern)) if pattern == invalid_pattern);
    }

    #[test_case("foo", "foo bar", vec![(0, 3)])]
    #[test_case("world!", "Hello world!", vec![(6, 12)])]
    #[test_case("wild.*", "Some line that will be matched with a wildcard", vec![(38, 46)])]
    #[test_case(".*everything.*", "Here everything should be matched", vec![(0, 33)])]
    #[test_case("foo", "foo multiple matches foo bar", vec![(0, 3), (21, 24)])]
    #[test_case("foo.*foo", "Should match from this foo to this foo", vec![(23, 38)])]
    #[test_case(r"\d{4}", "Current year is 2022", vec![(16, 20)])]
    fn find_matches_case_sensitive(pattern: &str, line: &str, expected: Vec<(usize, usize)>) {
        let search_config = SearchConfig::default();
        let matches_found = find_matching_indices(pattern, line, &search_config);
        let expected = expected_indices(expected);

        assert_eq!(matches_found, expected)
    }

    #[test_case("FoO", "foo bar", vec![(0, 3)])]
    #[test_case("hello", "Hello world and hello Rust", vec![(0, 5), (16, 21)])]
    fn find_matches_case_insensitive(pattern: &str, line: &str, expected: Vec<(usize, usize)>) {
        let search_config = SearchConfig {
            case_insensitive: true,
            ..Default::default()
        };
        let matches_found = find_matching_indices(pattern, line, &search_config);
        let expected = expected_indices(expected);

        assert_eq!(matches_found, expected)
    }

    #[test_case("foo", "simple foo", vec![(7, 10)])]
    #[test_case("foo", "foo but not this fooo", vec![(0,3)])]
    fn find_matches_whole_words(pattern: &str, line: &str, expected: Vec<(usize, usize)>) {
        let search_config = SearchConfig {
            exact_match: true,
            ..Default::default()
        };
        let matches_found = find_matching_indices(pattern, line, &search_config);
        let expected = expected_indices(expected);

        assert_eq!(matches_found, expected)
    }
}
