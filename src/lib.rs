// Copyright 2016 Victor Brekenfeld
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//!
//! `simplelog` provides a series of logging facilities, that can be easily combined.
//!
//! - `SimpleLogger` (very basic logger that logs to stdout)
//! - `TermLogger` (advanced terminal logger, that splits to stdout/err and has color support) (can be excluded on unsupported platforms)
//! - `FileLogger` (logs to a given file)
//! - `CombinedLogger` (can be used to form combinations of the above loggers)
//!
//! Only one Logger should be initialized of the start of your program
//! through the `Logger::init(...)` method. For the actual calling syntax
//! take a look at the documentation of the specific implementation(s) you wanna use.
//!

#![deny(missing_docs)]

#[macro_use] extern crate log;
#[cfg(feature = "term")]
extern crate term;
extern crate time;

mod config;
mod loggers;

pub use self::config::Config;
pub use self::loggers::{SimpleLogger, WriteLogger, CombinedLogger};
#[cfg(feature = "term")]
pub use self::loggers::TermLogger;

pub use log::LogLevelFilter;

use log::Log;

/// Trait to have a common interface to obtain the LogLevel of Loggers
///
/// Necessary for CombinedLogger to calculate
/// the lowest used LogLevel.
///
pub trait SharedLogger: Log {
    /// Returns the set LogLevel for this Logger
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # fn main() {
    /// let logger = SimpleLogger::new(LogLevelFilter::Info, Config::default());
    /// println!("{}", logger.level());
    /// # }
    /// ```
    fn level(&self) -> LogLevelFilter;

    /// Inspect the config of a running Logger
    ///
    /// An Option is returned, because some Logger may not contain a Config
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # fn main() {
    /// let logger = SimpleLogger::new(LogLevelFilter::Info, Config::default());
    /// println!("{:?}", logger.config());
    /// # }
    /// ```
    fn config(&self) -> Option<&Config>;

    /// Returns the logger as a Log trait object
    fn as_log(self: Box<Self>) -> Box<Log>;
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use std::fs::File;

    use super::*;

    #[test]
    fn test() {
        CombinedLogger::init(
            vec![
                //error
                SimpleLogger::new(LogLevelFilter::Error, Config::default()),
                TermLogger::new(LogLevelFilter::Error, Config::default()).unwrap(),
                WriteLogger::new(LogLevelFilter::Error, Config::default(), File::create("error.log").unwrap()),

                //warn
                SimpleLogger::new(LogLevelFilter::Warn, Config::default()),
                TermLogger::new(LogLevelFilter::Warn, Config::default()).unwrap(),
                WriteLogger::new(LogLevelFilter::Warn, Config::default(), File::create("warn.log").unwrap()),

                //info
                SimpleLogger::new(LogLevelFilter::Info, Config::default()),
                TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap(),
                WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create("info.log").unwrap()),

                //debug
                SimpleLogger::new(LogLevelFilter::Debug, Config::default()),
                TermLogger::new(LogLevelFilter::Debug, Config::default()).unwrap(),
                WriteLogger::new(LogLevelFilter::Debug, Config::default(), File::create("debug.log").unwrap()),

                //trace
                SimpleLogger::new(LogLevelFilter::Trace, Config::default()),
                TermLogger::new(LogLevelFilter::Trace, Config::default()).unwrap(),
                WriteLogger::new(LogLevelFilter::Trace, Config::default(), File::create("trace.log").unwrap()),
            ]
        ).unwrap();

        error!("Test Error");
        warn!("Test Warning");
        info!("Test Information");
        debug!("Test Debug");
        trace!("Test Trace");

        let mut error = String::new();
        File::open("error.log").unwrap().read_to_string(&mut error).unwrap();
        let mut warn = String::new();
        File::open("warn.log").unwrap().read_to_string(&mut warn).unwrap();
        let mut info = String::new();
        File::open("info.log").unwrap().read_to_string(&mut info).unwrap();
        let mut debug = String::new();
        File::open("debug.log").unwrap().read_to_string(&mut debug).unwrap();
        let mut trace = String::new();
        File::open("trace.log").unwrap().read_to_string(&mut trace).unwrap();

        assert!(error.contains("Test Error"));
        assert!(!error.contains("Test Warning"));

        assert!(warn.contains("Test Error"));
        assert!(warn.contains("Test Warning"));
        assert!(!warn.contains("Test Information"));

        assert!(info.contains("Test Error"));
        assert!(info.contains("Test Warning"));
        assert!(info.contains("Test Information"));
        assert!(!info.contains("Test Debug"));

        assert!(debug.contains("Test Error"));
        assert!(debug.contains("Test Warning"));
        assert!(debug.contains("Test Information"));
        assert!(debug.contains("Test Debug"));
        assert!(!debug.contains("Test Trace"));

        assert!(trace.contains("Test Error"));
        assert!(trace.contains("Test Warning"));
        assert!(trace.contains("Test Information"));
        assert!(trace.contains("Test Debug"));
        assert!(trace.contains("Test Trace"));
    }
}
