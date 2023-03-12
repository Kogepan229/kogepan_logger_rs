// Used for manual test
use kogepan_logger_rs::termlogger::TerminalLogger;
use log::*;

fn main() {
    TerminalLogger::init(LevelFilter::Trace).unwrap();
    error!("error");
    warn!("warn");
    info!("info");
    debug!("debug");
    trace!("trace");
}
