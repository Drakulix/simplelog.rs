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

extern crate log;
#[cfg(feature = "term")]
extern crate term;
extern crate time;

mod simplelog;
#[cfg(feature = "term")]
mod termlog;
mod filelog;
mod comblog;

pub use self::simplelog::SimpleLogger;
#[cfg(feature = "term")]
pub use self::termlog::TermLogger;
pub use self::filelog::FileLogger;
pub use self::comblog::CombinedLogger;

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
    /// let logger = SimpleLogger::new(LogLevelFilter::Info);
    /// println!("{}", logger.level());
    /// ```
    fn level(&self) -> LogLevelFilter;

    /// Returns the logger as a Log trait object
    fn as_log(self: Box<Self>) -> Box<Log>;
}
