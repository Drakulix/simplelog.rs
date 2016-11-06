#[macro_use]extern crate log;
extern crate simplelog;

use simplelog::{TermLogger, FileLogger, CombinedLogger, LogLevelFilter};

use std::fs::File;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Warn).unwrap(),
            FileLogger::new(LogLevelFilter::Info, File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();

    error!("Bright red error");
    info!("This only appears in the log file");
    debug!("This level is currently not enabled for any logger");
}
