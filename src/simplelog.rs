// Copyright 2016 Victor Brekenfeld
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Module providing the SimpleLogger Implementation

use std::io::{stderr, Write};
use log::{LogLevel, LogLevelFilter, LogMetadata, LogRecord, SetLoggerError, set_logger, Log};
use time;
use super::SharedLogger;

/// The SimpleLogger struct. Provides a very basic Logger implementation
pub struct SimpleLogger {
    level: LogLevelFilter,
}

impl SimpleLogger {

    /// init function. Globally initializes the SimpleLogger as the one and only used log facility.
    ///
    /// Takes the desired `LogLevel` as argument. It cannot be changed later on.
    /// Fails if another Logger was already initialized.
    ///
    /// # Examples
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # fn main() {
    /// let _ = SimpleLogger::init(LogLevelFilter::Info);
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn init(log_level: LogLevelFilter) -> Result<(), SetLoggerError> {
        set_logger(|max_log_level| {
            max_log_level.set(log_level.clone());
            SimpleLogger::new(log_level)
        })
    }

    /// allows to create a new logger, that can be independently used, no matter whats globally set.
    ///
    /// no macros are provided for this case and you probably
    /// dont want to use this function, but `init()``, if you dont want to build a `CombinedLogger`.
    ///
    /// Takes the desired LogLevel as argument. It cannot be changed later on.
    ///
    /// # Examples
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # fn main() {
    /// let simple_logger = SimpleLogger::new(LogLevelFilter::Info);
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn new(log_level: LogLevelFilter) -> Box<SimpleLogger> {
        Box::new(SimpleLogger { level: log_level })
    }

}

impl Log for SimpleLogger {

    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {

            let stderr = stderr();

            let mut stderr_lock = stderr.lock();

            let cur_time = time::now();

            let _ = match record.level() {
                LogLevel::Trace =>
                    writeln!(stderr_lock,
                        "{:02}:{:02}:{:02} [{}] {}: [{}:{}] {}",
                            cur_time.tm_hour,
                            cur_time.tm_min,
                            cur_time.tm_sec,
                            record.level(),
                            record.target(),
                            record.location().file(),
                            record.location().line(),
                            record.args()
                    ),
                _ =>
                    writeln!(stderr_lock,
                        "{:02}:{:02}:{:02} [{}] {}: {}",
                            cur_time.tm_hour,
                            cur_time.tm_min,
                            cur_time.tm_sec,
                            record.level(),
                            record.target(),
                            record.args()
                    ),
            };

            stderr_lock.flush().unwrap();
        }
    }
}

impl SharedLogger for SimpleLogger {

    fn level(&self) -> LogLevelFilter {
        self.level
    }

    fn as_log(self: Box<Self>) -> Box<Log> {
        Box::new(*self)
    }

}
