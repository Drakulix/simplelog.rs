//! Module providing the TermLogger Implementation

use log::{LogLevel, LogLevelFilter, LogMetadata, LogRecord, SetLoggerError, set_logger, Log};
use time;
use term;
use term::{StderrTerminal, StdoutTerminal, Terminal, color};
use std::error;
use std::fmt;
use std::sync::{Mutex, MutexGuard};
use std::io::{Write, Error};
use self::TermLogError::{SetLogger, Term};
use super::SharedLogger;

/// TermLogger error type.
#[derive(Debug)]
pub enum TermLogError {
    SetLogger(SetLoggerError),
    Term(String),
}

impl fmt::Display for TermLogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetLogger(ref err) => err.fmt(f),
            Term(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for TermLogError {
    fn description(&self) -> &str {
        match * self {
            SetLogger(ref err) => err.description(),
            Term(ref err) => &err,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SetLogger(ref err) => Some(err),
            Term(_) => None,
        }
    }
}

impl From<SetLoggerError> for TermLogError {
    fn from(error: SetLoggerError) -> Self {
        SetLogger(error)
    }
}

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
    pub fn init(log_level: LogLevelFilter) -> Result<(), TermLogError> {
        let logger = try!(TermLogger::new(log_level).ok_or(Term("a terminal couldn't be opened".to_string())));
        try!(set_logger(|max_log_level| {
            max_log_level.set(log_level.clone());
            logger
        }));
        Ok(())
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
    /// let term_logger = TermLogger::new(LogLevelFilter::Info).unwrap();
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn new(log_level: LogLevelFilter) -> Option<Box<TermLogger>> {
        term::stderr().and_then(|stderr|
            term::stdout().map(|stdout| {
                Box::new(TermLogger { level: log_level, stderr: Mutex::new(stderr), stdout: Mutex::new(stdout) })
            })
        )
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
