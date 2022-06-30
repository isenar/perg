mod file;
mod recursive;
mod stdin;

pub use file::SingleFileSearcher;
pub use recursive::RecursiveSearcher;
pub use stdin::StdinSearcher;

use crate::matcher::Matcher;
use crate::summary::{MatchedLine, SearchSummary};
use crate::Result;

use std::ops::Not;

pub trait Searcher {
    fn search(&self, matcher: &Matcher) -> Result<SearchSummary>;
}

pub(crate) fn summarize(
    matcher: &Matcher,
    file_path: impl Into<String>,
    lines: impl IntoIterator<Item = String>,
) -> SearchSummary {
    let matching_lines: Vec<_> = lines
        .into_iter()
        .enumerate()
        .filter_map(|(line_num, line)| {
            let matches_indices = matcher.find_matches(&line);

            matches_indices.is_empty().not().then(|| MatchedLine {
                line_number: line_num + 1,
                line,
                matches_indices,
            })
        })
        .collect();

    if matching_lines.is_empty() {
        SearchSummary::empty()
    } else {
        SearchSummary::new(file_path.into(), matching_lines)
    }
}
