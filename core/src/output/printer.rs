use crate::config::OutputConfig;
use crate::summary::SearchSummary;
use crate::Result;

use std::io::Write;

pub struct Printer<'conf> {
    config: &'conf OutputConfig,
}

impl<'conf> Printer<'conf> {
    pub fn new(search_config: &'conf OutputConfig) -> Self {
        Self {
            config: search_config,
        }
    }

    pub fn print(&self, summary: SearchSummary, writer: &mut impl Write) -> Result<()> {
        if self.config.only_file_names {
            for (file, _) in summary {
                writeln!(writer, "{file}")?;
            }
        } else {
            for (file, line_data) in summary {
                writeln!(writer, "{file}")?;

                for line in line_data {
                    writeln!(writer, "{line}")?;
                }

                writeln!(writer)?;
            }
        }

        Ok(())
    }
}
