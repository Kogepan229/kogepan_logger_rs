use chrono::Local;
use log::{LevelFilter, Log};

pub mod kogepanlogger;
pub mod termlogger;
pub mod writelogger;

trait Sharedlogger: Log {
    fn level(&self) -> LevelFilter;
}

fn get_current_time() -> String {
    Local::now().format("%Y/%d/%m %H:%M:%S").to_string()
}
