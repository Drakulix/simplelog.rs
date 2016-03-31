//! Module providing the TermLogger Implementation

use log::{LogLevel, LogLevelFilter, LogMetadata, LogRecord, SetLoggerError, set_logger, Log};
use time;
use term;
use term::{StderrTerminal, StdoutTerminal, Terminal, color};
use std::sync::{Mutex, MutexGuard};
use std::io::{Write, Error};
use super::SharedLogger;

/// The TermLogger struct. Provides a stderr/out based Logger implementation
///
/// Supports colored output
pub struct TermLogger {
    level: LogLevelFilter,
    stderr: Mutex<Box<StderrTerminal>>,
    stdout: Mutex<Box<StdoutTerminal>>,
}

impl TermLogger
{
    /// init function. Globally initializes the TermLogger as the one and only used log facility.
    ///
    /// Takes the desired LogLevel as argument. It cannot be changed later on.
    /// Fails if another Logger was already initialized.
    ///
    /// # Examples
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # fn main() {
    /// let _ = TermLogger::init(LogLevelFilter::Info);
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn init(log_level: LogLevelFilter) -> Result<(), SetLoggerError> {
        set_logger(|max_log_level| {
            max_log_level.set(log_level.clone());
            TermLogger::new(log_level)
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
    /// let term_logger = TermLogger::new(LogLevelFilter::Info);
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn new(log_level: LogLevelFilter) -> Box<TermLogger> {
        Box::new(TermLogger { level: log_level, stderr: Mutex::new(term::stderr().unwrap()), stdout: Mutex::new(term::stdout().unwrap()) })
    }

    fn try_log_term<W>(&self, record: &LogRecord, mut term_lock: MutexGuard<Box<Terminal<Output=W> + Send>>) -> Result<(), Error>
        where W: Write + Sized
    {
        let cur_time = time::now();

        let color = match record.level() {
            LogLevel::Error => color::RED,
            LogLevel::Warn => color::YELLOW,
            LogLevel::Info => color::BLUE,
            LogLevel::Debug => color::CYAN,
            LogLevel::Trace => color::WHITE
        };

        try!(write!(term_lock, "{:02}:{:02}:{:02} [",
                    cur_time.tm_hour,
                    cur_time.tm_min,
                    cur_time.tm_sec));
        try!(term_lock.fg(color));
        try!(write!(term_lock, "{}", record.level()));
        try!(term_lock.reset());
        try!(write!(term_lock, "] "));

        match record.level() {
            LogLevel::Error |
            LogLevel::Warn  |
            LogLevel::Info  |
            LogLevel::Debug => {
                try!(writeln!(term_lock,
                    "{}: {}",
                        record.target(),
                        record.args()
                ));
            },
            LogLevel::Trace => {
                try!(writeln!(term_lock,
                    "{}: [{}:{}] - {}",
                        record.target(),
                        record.location().file(),
                        record.location().line(),
                        record.args()
                ));
            },
        };

        try!(term_lock.flush());

        Ok(())
    }

    fn try_log(&self, record: &LogRecord) -> Result<(), Error> {
        if self.enabled(record.metadata()) {
            if record.level() == LogLevel::Error {
                self.try_log_term(record, self.stderr.lock().unwrap())
            } else {
                self.try_log_term(record, self.stdout.lock().unwrap())
            }
        } else {
            Ok(())
        }
    }
}

impl Log for TermLogger
{
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &LogRecord) {
        let _ = self.try_log(record);
    }
}

impl SharedLogger for TermLogger
{
    fn level(&self) -> LogLevelFilter {
        self.level
    }

    fn as_log(self: Box<Self>) -> Box<Log> {
        Box::new(*self)
    }
}
