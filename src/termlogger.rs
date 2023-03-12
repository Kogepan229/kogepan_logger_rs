use colored::*;
use log::{
    set_boxed_logger, set_max_level, Level, LevelFilter, Log, Metadata, Record, SetLoggerError,
};
use std::io::{stdout, Write};

use crate::{get_current_time, Sharedlogger};

pub struct TerminalLogger {
    level: LevelFilter,
}

impl TerminalLogger {
    pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
        let logger = TerminalLogger::new(level);
        set_max_level(level);
        set_boxed_logger(logger)?;
        Ok(())
    }

    #[must_use]
    pub fn new(level: LevelFilter) -> Box<TerminalLogger> {
        Box::new(TerminalLogger { level })
    }
}

impl Log for TerminalLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Error => println!(
                    "{} [{}] {}",
                    get_current_time(),
                    record.level().as_str().red(),
                    record.args()
                ),
                Level::Warn => println!(
                    "{} [{}] {}",
                    get_current_time(),
                    record.level().as_str().yellow(),
                    record.args()
                ),
                Level::Debug => println!(
                    "{} [{} {} {}] {}",
                    get_current_time(),
                    record.level(),
                    record.file().unwrap_or("unknown"),
                    record.line().unwrap_or(0),
                    record.args()
                ),
                _ => println!(
                    "{} [{}] {}",
                    get_current_time(),
                    record.level(),
                    record.args()
                ),
            }
        }
    }

    fn flush(&self) {
        stdout().flush().unwrap();
    }
}

impl Sharedlogger for TerminalLogger {
    fn level(&self) -> LevelFilter {
        self.level
    }
}
