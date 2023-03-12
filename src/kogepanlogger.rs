use crate::termlogger::TerminalLogger;
use crate::writelogger::WriteLogger;
use crate::Sharedlogger;
use log::{set_boxed_logger, set_max_level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::fs::File;

pub struct KogepanLogger {
    level: LevelFilter,
    term_logger: Box<dyn Sharedlogger>,
    write_logger: Box<dyn Sharedlogger>,
}

impl KogepanLogger {
    pub fn init(
        term_level: LevelFilter,
        write_level: LevelFilter,
        file: File,
    ) -> Result<(), SetLoggerError> {
        let term_logger = TerminalLogger::new(term_level);
        let write_logger = WriteLogger::new(write_level, file);
        let max_level = if term_level >= write_level {
            term_level
        } else {
            write_level
        };

        let logger = Box::new(KogepanLogger {
            level: max_level,
            term_logger: term_logger,
            write_logger: write_logger,
        });

        set_max_level(max_level);
        set_boxed_logger(logger)?;
        Ok(())
    }
}

impl Log for KogepanLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.term_logger.log(record);
            self.write_logger.log(record);
        }
    }

    fn flush(&self) {
        self.term_logger.flush();
        self.write_logger.flush();
    }
}
