//! Module providing the TermLogger Implementation

use log::{Level, LevelFilter, Metadata, Record, SetLoggerError, set_boxed_logger, set_max_level, Log};
use term;
use term::{StderrTerminal, StdoutTerminal, Terminal, color};
use std::error;
use std::fmt;
use std::sync::Mutex;
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

struct OutputStreams {
    stderr: Box<StderrTerminal>,
    stdout: Box<StdoutTerminal>,
}

/// The TermLogger struct. Provides a stderr/out based Logger implementation
///
/// Supports colored output
pub struct TermLogger {
    level: LevelFilter,
    config: Config,
    streams: Mutex<OutputStreams>,
}

impl TermLogger
{
    /// init function. Globally initializes the TermLogger as the one and only used log facility.
    ///
    /// Takes the desired `Level` and `Config` as arguments. They cannot be changed later on.
    /// Fails if another Logger was already initialized.
    ///
    /// # Examples
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # fn main() {
    /// let _ = TermLogger::init(LevelFilter::Info, Config::default());
    /// # }
    /// ```
    pub fn init(log_level: LevelFilter, config: Config) -> Result<(), TermLogError> {
        let logger = try!(TermLogger::new(log_level, config).ok_or(Term));
        set_max_level(log_level.clone());
        try!(set_boxed_logger(logger));
        Ok(())
    }

    /// allows to create a new logger, that can be independently used, no matter whats globally set.
    ///
    /// no macros are provided for this case and you probably
    /// dont want to use this function, but `init()`, if you dont want to build a `CombinedLogger`.
    ///
    /// Takes the desired `Level` and `Config` as arguments. They cannot be changed later on.
    ///
    /// # Examples
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # fn main() {
    /// let term_logger = TermLogger::new(LevelFilter::Info, Config::default()).unwrap();
    /// # }
    /// ```
    pub fn new(log_level: LevelFilter, config: Config) -> Option<Box<TermLogger>> {
        term::stderr().and_then(|stderr|
            term::stdout().map(|stdout| {
                let streams = Mutex::new( OutputStreams {
                    stderr: stderr,
                    stdout: stdout,
                });
                Box::new(TermLogger { level: log_level, config: config, streams: streams
                })
            })
        )
    }

    fn try_log_term<W>(&self, record: &Record, term_lock: &mut Box<Terminal<Output=W> + Send>) -> Result<(), Error>
        where W: Write + Sized
    {
        let color = match record.level() {
            Level::Error => color::RED,
            Level::Warn => color::YELLOW,
            Level::Info => color::BLUE,
            Level::Debug => color::CYAN,
            Level::Trace => color::WHITE
        };

        if let Some(time) = self.config.time {
            if time <= record.level() {
                try!(write_time(&mut *term_lock, &self.config));
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
        Ok(())
    }

    fn try_log(&self, record: &Record) -> Result<(), Error> {
        if self.enabled(record.metadata()) {

            let mut streams = self.streams.lock().unwrap();

            if record.level() == Level::Error {
                self.try_log_term(record, &mut streams.stderr)
            } else {
                self.try_log_term(record, &mut streams.stdout)
            }
        } else {
            Ok(())
        }
    }
}

impl Log for TermLogger
{
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        let _ = self.try_log(record);
    }

    fn flush(&self) {
        let mut streams = self.streams.lock().unwrap();
        let _ = streams.stdout.flush();
        let _ = streams.stderr.flush();
    }
}

impl SharedLogger for TermLogger
{
    fn level(&self) -> LevelFilter {
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
