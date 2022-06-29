use crate::config::SearchConfig;
use crate::summary::PatternIndices;
use crate::Result;
use regex::{Regex, RegexBuilder};

#[derive(Debug)]
pub struct Matcher(Regex);

impl Matcher {
    pub fn build(pattern: &str, config: &SearchConfig) -> Result<Self> {
        let regex = RegexBuilder::new(pattern)
            .case_insensitive(config.case_insensitive)
            .build()?;

        Ok(Self(regex))
    }

    pub fn find_matches(&self, line: &str) -> Vec<PatternIndices> {
        self.0
            .find_iter(line)
            .map(|mat| PatternIndices {
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
    ) -> Vec<PatternIndices> {
        Matcher::build(pattern, search_config)
            .expect("Failed to build matcher")
            .find_matches(line)
    }

    #[test]
    fn fails_on_bad_regex_pattern() {
        let config = SearchConfig::default();
        let matcher = Matcher::build("[", &config);

        assert_matches!(matcher, Err(crate::Error::BadRegex(_)));
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

        let expected: Vec<_> = expected
            .into_iter()
            .map(|(start, end)| PatternIndices { start, end })
            .collect();

        assert_eq!(matches_found, expected)
    }

    #[test_case("FoO", "foo bar", vec![(0, 3)])]
    #[test_case("hello", "Hello world and hello Rust", vec![(0,5), (16, 21)])]
    fn find_matches_case_insensitive(pattern: &str, line: &str, expected: Vec<(usize, usize)>) {
        let search_config = SearchConfig {
            case_insensitive: true,
            ..Default::default()
        };
        let matches_found = find_matching_indices(pattern, line, &search_config);

        let expected: Vec<_> = expected
            .into_iter()
            .map(|(start, end)| PatternIndices { start, end })
            .collect();

        assert_eq!(matches_found, expected)
    }
}
