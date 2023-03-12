use log::{set_boxed_logger, set_max_level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::io::Write;

use crate::{get_current_time, Sharedlogger};
use std::sync::Mutex;

pub struct WriteLogger<W: Write + Send + 'static> {
    level: LevelFilter,
    writable: Mutex<W>,
}

impl<W: Write + Send + 'static> WriteLogger<W> {
    pub fn init(level: LevelFilter, writable: W) -> Result<(), SetLoggerError> {
        let logger = WriteLogger::new(level, writable);
        set_max_level(level);
        set_boxed_logger(logger)?;
        Ok(())
    }

    #[must_use]
    pub fn new(level: LevelFilter, writable: W) -> Box<WriteLogger<W>> {
        Box::new(WriteLogger {
            level,
            writable: Mutex::new(writable),
        })
    }
}

impl<W: Write + Send + 'static> Log for WriteLogger<W> {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut write_lock = self.writable.lock().unwrap();
            writeln!(
                &mut *write_lock,
                "{} [{}] {}",
                get_current_time(),
                record.level(),
                record.args()
            )
            .unwrap();
        }
    }

    fn flush(&self) {
        let _ = self.writable.lock().unwrap().flush();
    }
}

impl<W: Write + Send + 'static> Sharedlogger for WriteLogger<W> {
    fn level(&self) -> LevelFilter {
        self.level
    }
}
