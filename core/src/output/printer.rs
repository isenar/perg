use crate::config::OutputConfig;
use crate::SearchSummary;

pub struct Printer<'conf> {
    _config: &'conf OutputConfig,
}

impl<'conf> Printer<'conf> {
    pub fn new(search_config: &'conf OutputConfig) -> Self {
        Self {
            _config: search_config,
        }
    }

    pub fn print(&self, summaries: impl Iterator<Item = &'conf SearchSummary>) {
        for summary in summaries {
            println!("{summary}");
        }
    }
}
