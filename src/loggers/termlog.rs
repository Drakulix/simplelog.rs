//! Module providing the TermLogger Implementation

use log::{
    set_boxed_logger, set_max_level, Level, LevelFilter, Log, Metadata, Record, SetLoggerError,
};
use std::error;
use std::fmt;
use std::io::{Error, Write};
use std::sync::Mutex;
use term;
use term::{color, StderrTerminal, StdoutTerminal, Terminal};

use self::TermLogError::{SetLogger, Term};
use super::logging::*;

use {Config, SharedLogger};

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
        match *self {
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

enum StdTerminal {
    Stderr(Box<StderrTerminal>),
    Stdout(Box<StdoutTerminal>),
}

impl StdTerminal {
    fn flush(&mut self) -> Result<(), Error> {
        match self {
            StdTerminal::Stderr(term) => term.flush(),
            StdTerminal::Stdout(term) => term.flush(),
        }
    }
}

struct OutputStreams {
    err: StdTerminal,
    out: StdTerminal,
}

/// Specifies which streams should be used when logging
pub enum TerminalMode {
    /// Only use Stdout
    Stdout,
    /// Only use Stderr
    Stderr,
    /// Use Stderr for Errors and Stdout otherwise
    Mixed,
}

impl Default for TerminalMode {
    fn default() -> TerminalMode {
        TerminalMode::Mixed
    }
}

/// The TermLogger struct. Provides a stderr/out based Logger implementation
///
/// Supports colored output
pub struct TermLogger {
    level: LevelFilter,
    config: Config,
    streams: Mutex<OutputStreams>,
}

impl TermLogger {
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
    /// let _ = TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed);
    /// # }
    /// ```
    pub fn init(
        log_level: LevelFilter,
        config: Config,
        mode: TerminalMode,
    ) -> Result<(), TermLogError> {
        let logger = try!(TermLogger::new(log_level, config, mode).ok_or(Term));
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
    /// let term_logger = TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap();
    /// # }
    /// ```
    pub fn new(
        log_level: LevelFilter,
        config: Config,
        mode: TerminalMode,
    ) -> Option<Box<TermLogger>> {
        let streams = match mode {
            TerminalMode::Stdout => term::stdout().and_then(|stdout| {
                term::stdout().map(|stdout2| {
                    Mutex::new(OutputStreams {
                        err: StdTerminal::Stdout(stdout),
                        out: StdTerminal::Stdout(stdout2),
                    })
                })
            }),
            TerminalMode::Stderr => term::stderr().and_then(|stderr| {
                term::stderr().map(|stderr2| {
                    Mutex::new(OutputStreams {
                        err: StdTerminal::Stderr(stderr),
                        out: StdTerminal::Stderr(stderr2),
                    })
                })
            }),
            TerminalMode::Mixed => term::stderr().and_then(|stderr| {
                term::stdout().map(|stdout| {
                    Mutex::new(OutputStreams {
                        err: StdTerminal::Stderr(stderr),
                        out: StdTerminal::Stdout(stdout),
                    })
                })
            }),
        };

        streams.map(|streams| {
            Box::new(TermLogger {
                level: log_level,
                config: config,
                streams: streams,
            })
        })
    }

    fn try_log_term<W>(
        &self,
        record: &Record,
        term_lock: &mut Box<Terminal<Output = W> + Send>,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        let color = match record.level() {
            Level::Error => color::RED,
            Level::Warn => color::YELLOW,
            Level::Info => color::BLUE,
            Level::Debug => color::CYAN,
            Level::Trace => color::WHITE,
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

        if let Some(thread) = self.config.thread {
            if thread <= record.level() {
                try!(write_thread_id(&mut *term_lock));
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
            if should_skip(&self.config, record) {
                return Ok(());
            }

            let mut streams = self.streams.lock().unwrap();

            if record.level() == Level::Error {
                match streams.err {
                    StdTerminal::Stderr(ref mut term) => self.try_log_term(record, term),
                    StdTerminal::Stdout(ref mut term) => self.try_log_term(record, term),
                }
            } else {
                match streams.out {
                    StdTerminal::Stderr(ref mut term) => self.try_log_term(record, term),
                    StdTerminal::Stdout(ref mut term) => self.try_log_term(record, term),
                }
            }
        } else {
            Ok(())
        }
    }
}

impl Log for TermLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        let _ = self.try_log(record);
    }

    fn flush(&self) {
        let mut streams = self.streams.lock().unwrap();
        let _ = streams.out.flush();
        let _ = streams.err.flush();
    }
}

impl SharedLogger for TermLogger {
    fn level(&self) -> LevelFilter {
        self.level
    }

    fn config(&self) -> Option<&Config> {
        Some(&self.config)
    }

    fn as_log(self: Box<Self>) -> Box<Log> {
        Box::new(*self)
    }
}
