use crate::config::OutputConfig;
use crate::summary::SearchSummary;

use colored::Colorize;

pub struct Printer<'conf> {
    config: &'conf OutputConfig,
}

impl<'conf> Printer<'conf> {
    pub fn new(search_config: &'conf OutputConfig) -> Self {
        Self {
            config: search_config,
        }
    }

    /// Write `summary` to stdout in a ag (silver-searcher) format and color scheme
    pub fn print(&self, summary: SearchSummary) {
        if self.config.only_file_names {
            for file in summary.files() {
                println!("{}", file.bright_green());
            }
        } else {
            for (file, line_data) in summary.into_iter() {
                println!("{}", file.bright_green());

                for line in line_data {
                    println!("{line}");
                }

                println!();
            }
        }
    }
}
