//! Module providing the TermLogger Implementation

use log::{LogLevel, LogLevelFilter, LogMetadata, LogRecord, SetLoggerError, set_logger, Log};
use term;
use term::{StderrTerminal, StdoutTerminal, Terminal, color};
use std::error;
use std::fmt;
use std::sync::{Mutex, MutexGuard};
use std::io::{Write, Error};

use self::TermLogError::{SetLogger, Term};
use super::logging::*;

use ::{Config, SharedLogger};

/// TermLogger error type.
#[derive(Debug)]
pub enum TermLogError {
    ///The type returned by set_logger if set_logger has already been called.
    SetLogger(SetLoggerError),

    ///TermLogger initialization might also fail if stdout or stderr could not be opened,
    ///e.g. when no tty is attached to the process, it runs detached in the background, etc
    /// This is represented by the `Term` Kind
    Term,
}

impl fmt::Display for TermLogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error as FmtError;

        write!(f, "{}", self.description())
    }
}

impl error::Error for TermLogError {
    fn description(&self) -> &str {
        match * self {
            SetLogger(ref err) => err.description(),
            Term => "A terminal could not be opened",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SetLogger(ref err) => Some(err),
            Term => None,
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
    config: Config,
    stderr: Mutex<Box<StderrTerminal>>,
    stdout: Mutex<Box<StdoutTerminal>>,
}

impl TermLogger
{
    /// init function. Globally initializes the TermLogger as the one and only used log facility.
    ///
    /// Takes the desired `LogLevel` and `Config` as arguments. They cannot be changed later on.
    /// Fails if another Logger was already initialized.
    ///
    /// # Examples
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # fn main() {
    /// let _ = TermLogger::init(LogLevelFilter::Info, Config::default());
    /// # }
    /// ```
    pub fn init(log_level: LogLevelFilter, config: Config) -> Result<(), TermLogError> {
        let logger = try!(TermLogger::new(log_level, config).ok_or(Term));
        try!(set_logger(|max_log_level| {
            max_log_level.set(log_level.clone());
            logger
        }));
        Ok(())
    }

    /// allows to create a new logger, that can be independently used, no matter whats globally set.
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
    /// let term_logger = TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap();
    /// # }
    /// ```
    pub fn new(log_level: LogLevelFilter, config: Config) -> Option<Box<TermLogger>> {
        term::stderr().and_then(|stderr|
            term::stdout().map(|stdout| {
                Box::new(TermLogger { level: log_level, config: config, stderr: Mutex::new(stderr), stdout: Mutex::new(stdout) })
            })
        )
    }

    fn try_log_term<W>(&self, record: &LogRecord, mut term_lock: MutexGuard<Box<Terminal<Output=W> + Send>>) -> Result<(), Error>
        where W: Write + Sized
    {
        let color = match record.level() {
            LogLevel::Error => color::RED,
            LogLevel::Warn => color::YELLOW,
            LogLevel::Info => color::BLUE,
            LogLevel::Debug => color::CYAN,
            LogLevel::Trace => color::WHITE
        };

        if let Some(time) = self.config.time {
            if time <= record.level() {
                try!(write_time(&mut *term_lock));
            }
        }

        if let Some(level) = self.config.level {
            if level <= record.level() {
                try!(term_lock.fg(color));
                try!(write_level(record, &mut *term_lock));
                try!(term_lock.reset());
            }
        }

        if let Some(target) = self.config.target {
            if target <= record.level() {
                try!(write_target(record, &mut *term_lock));
            }
        }

        if let Some(location) = self.config.location {
            if location <= record.level() {
                try!(write_location(record, &mut *term_lock));
            }
        }

        try!(write_args(record, &mut *term_lock));
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

    fn config(&self) -> Option<&Config>
    {
        Some(&self.config)
    }

    fn as_log(self: Box<Self>) -> Box<Log> {
        Box::new(*self)
    }
}
