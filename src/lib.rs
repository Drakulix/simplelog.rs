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
//! - `WriteLogger` (logs to a given struct implementing `Write`, e.g. a file)
//! - `CombinedLogger` (can be used to form combinations of the above loggers)
//!
//! Only one Logger should be initialized of the start of your program
//! through the `Logger::init(...)` method. For the actual calling syntax
//! take a look at the documentation of the specific implementation(s) you wanna use.
//!

#![deny(missing_docs)]

#[cfg_attr(test, macro_use)]
extern crate log;
#[cfg(feature = "term")]
extern crate term;
extern crate chrono;

mod config;
mod loggers;

pub use self::config::Config;
pub use self::loggers::{SimpleLogger, WriteLogger, CombinedLogger};
#[cfg(feature = "term")]
pub use self::loggers::{TermLogger, TermLogError};

pub use log::{Level, LevelFilter};

use log::Log;

/// Trait to have a common interface to obtain the Level of Loggers
///
/// Necessary for CombinedLogger to calculate
/// the lowest used Level.
///
pub trait SharedLogger: Log {
    /// Returns the set Level for this Logger
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # fn main() {
    /// let logger = SimpleLogger::new(LevelFilter::Info, Config::default());
    /// println!("{}", logger.level());
    /// # }
    /// ```
    fn level(&self) -> LevelFilter;

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
    /// let logger = SimpleLogger::new(LevelFilter::Info, Config::default());
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
        let mut i = 0;

        CombinedLogger::init(
            {
                let mut vec = Vec::new();
                let mut conf = Config {
                    time: None,
                    level: None,
                    target: None,
                    location: None,
                    time_format: None,
                };

                for elem in vec![None, Some(Level::Trace), Some(Level::Debug), Some(Level::Info), Some(Level::Warn), Some(Level::Error)]
                {
                    conf.location = elem;
                    conf.target = elem;
                    conf.level = elem;
                    conf.time = elem;
                    i += 1;

                    //error
                    vec.push(SimpleLogger::new(LevelFilter::Error, conf) as Box<SharedLogger>);
                    vec.push(TermLogger::new(LevelFilter::Error, conf).unwrap() as Box<SharedLogger>);
                    vec.push(WriteLogger::new(LevelFilter::Error, conf, File::create(&format!("error_{}.log", i)).unwrap()) as Box<SharedLogger>);

                    //warn
                    vec.push(SimpleLogger::new(LevelFilter::Warn, conf) as Box<SharedLogger>);
                    vec.push(TermLogger::new(LevelFilter::Warn, conf).unwrap() as Box<SharedLogger>);
                    vec.push(WriteLogger::new(LevelFilter::Warn, conf, File::create(&format!("warn_{}.log", i)).unwrap()) as Box<SharedLogger>);

                    //info
                    vec.push(SimpleLogger::new(LevelFilter::Info, conf) as Box<SharedLogger>);
                    vec.push(TermLogger::new(LevelFilter::Info, conf).unwrap() as Box<SharedLogger>);
                    vec.push(WriteLogger::new(LevelFilter::Info, conf, File::create(&format!("info_{}.log", i)).unwrap()) as Box<SharedLogger>);

                    //debug
                    vec.push(SimpleLogger::new(LevelFilter::Debug, conf) as Box<SharedLogger>);
                    vec.push(TermLogger::new(LevelFilter::Debug, conf).unwrap() as Box<SharedLogger>);
                    vec.push(WriteLogger::new(LevelFilter::Debug, conf, File::create(&format!("debug_{}.log", i)).unwrap()) as Box<SharedLogger>);

                    //trace
                    vec.push(SimpleLogger::new(LevelFilter::Trace, conf) as Box<SharedLogger>);
                    vec.push(TermLogger::new(LevelFilter::Trace, conf).unwrap() as Box<SharedLogger>);
                    vec.push(WriteLogger::new(LevelFilter::Trace, conf, File::create(&format!("trace_{}.log", i)).unwrap()) as Box<SharedLogger>);
                }

                vec
            }
        ).unwrap();

        error!("Test Error");
        warn!("Test Warning");
        info!("Test Information");
        debug!("Test Debug");
        trace!("Test Trace");

        for j in 1..i
        {
            let mut error = String::new();
            File::open(&format!("error_{}.log", j)).unwrap().read_to_string(&mut error).unwrap();
            let mut warn = String::new();
            File::open(&format!("warn_{}.log", j)).unwrap().read_to_string(&mut warn).unwrap();
            let mut info = String::new();
            File::open(&format!("info_{}.log", j)).unwrap().read_to_string(&mut info).unwrap();
            let mut debug = String::new();
            File::open(&format!("debug_{}.log", j)).unwrap().read_to_string(&mut debug).unwrap();
            let mut trace = String::new();
            File::open(&format!("trace_{}.log", j)).unwrap().read_to_string(&mut trace).unwrap();

            assert!(error.contains("Test Error"));
            assert!(!error.contains("Test Warning"));
            assert!(!error.contains("Test Information"));
            assert!(!error.contains("Test Debug"));
            assert!(!error.contains("Test Trace"));

            assert!(warn.contains("Test Error"));
            assert!(warn.contains("Test Warning"));
            assert!(!warn.contains("Test Information"));
            assert!(!warn.contains("Test Debug"));
            assert!(!warn.contains("Test Trace"));

            assert!(info.contains("Test Error"));
            assert!(info.contains("Test Warning"));
            assert!(info.contains("Test Information"));
            assert!(!info.contains("Test Debug"));
            assert!(!info.contains("Test Trace"));

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
}
