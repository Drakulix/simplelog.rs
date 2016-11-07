// Copyright 2016 Victor Brekenfeld
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Module providing the SimpleLogger Implementation

use std::io::{stderr, stdout};
use log::{LogLevel, LogLevelFilter, LogMetadata, LogRecord, SetLoggerError, set_logger, Log};
use ::{Config, SharedLogger};
use super::logging::try_log;

/// The SimpleLogger struct. Provides a very basic Logger implementation
pub struct SimpleLogger {
    level: LogLevelFilter,
    config: Config,
}

impl SimpleLogger {

    /// init function. Globally initializes the SimpleLogger as the one and only used log facility.
    ///
    /// Takes the desired `LogLevel` and `Config` as arguments. They cannot be changed later on.
    /// Fails if another Logger was already initialized.
    ///
    /// # Examples
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # fn main() {
    /// let _ = SimpleLogger::init(LogLevelFilter::Info, Config::default());
    /// # }
    /// ```
    pub fn init(log_level: LogLevelFilter, config: Config) -> Result<(), SetLoggerError> {
        set_logger(|max_log_level| {
            max_log_level.set(log_level.clone());
            SimpleLogger::new(log_level, config)
        })
    }

    /// allows to create a new logger, that can be independently used, no matter what is globally set.
    ///
    /// no macros are provided for this case and you probably
    /// dont want to use this function, but `init()`, if you dont want to build a `CombinedLogger`.
    ///
    /// Takes the desired `LogLevel` and `Config` as arguments. They cannot be changed later on.
    ///
    /// # Examples
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # fn main() {
    /// let simple_logger = SimpleLogger::new(LogLevelFilter::Info, Config::default());
    /// # }
    /// ```
    pub fn new(log_level: LogLevelFilter, config: Config) -> Box<SimpleLogger> {
        Box::new(SimpleLogger { level: log_level, config: config })
    }
}

impl Log for SimpleLogger {

    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            match record.level() {
                LogLevel::Error => {
                    let stderr = stderr();
                    let mut stderr_lock = stderr.lock();
                    let _ = try_log(&self.config, record, &mut stderr_lock);
                },
                _ => {
                    let stdout = stdout();
                    let mut stdout_lock = stdout.lock();
                    let _ = try_log(&self.config, record, &mut stdout_lock);
                }
            }
        }
    }
}

impl SharedLogger for SimpleLogger {

    fn level(&self) -> LogLevelFilter {
        self.level
    }

    fn config(&self) -> Option<&Config>
    {
        Some(&self.config)
    }

    fn as_log(self: Box<Self>) -> Box<Log> {
        Box::new(*self)
    }

}
