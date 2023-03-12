// Used for manual test

// use kogepan_logger_rs::termlogger::TerminalLogger;
// use kogepan_logger_rs::writelogger::WriteLogger;
use kogepan_logger_rs::kogepanlogger::KogepanLogger;
use log::*;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //TerminalLogger::init(LevelFilter::Trace).unwrap();

    // WriteLogger::init(
    //     LevelFilter::Trace,
    //     File::create("kogepanlogger.log").unwrap(),
    // )
    // .unwrap();

    KogepanLogger::init(
        LevelFilter::Trace,
        LevelFilter::Trace,
        File::create("kogepanlogger.log").unwrap(),
    )
    .unwrap();

    error!("error");
    warn!("warn");
    info!("info");
    debug!("debug");
    trace!("trace");

    Ok(())
}
