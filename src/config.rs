use log::Level;

pub use chrono::offset::{FixedOffset, Local, Offset, TimeZone, Utc};

/// Configuration for the Loggers
///
/// All loggers print the message in the following form:
/// `00:00:00 [LEVEL] crate::module: [lib.rs::100] your_message`
/// Every space delimited part except the actual message is optional.
///
/// Pass this struct to your logger to change when these information shall
/// be logged. Every part can be enabled for a specific Level and is then
/// automatically enable for all lower levels as well.
///
/// The Result is that the logging gets more detailed the more verbose it gets.
/// E.g. to have one part shown always use `Level::Error`. But if you
/// want to show the source line only on `Trace` use that.
/// Passing `None` will completely disable the part.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    ///At which level and below the current time shall be logged
    pub time: Option<Level>,
    ///At which level and below the level itself shall be logged
    pub level: Option<Level>,
    ///At which level and below the thread id shall be logged. Default DEBUG
    pub thread: Option<Level>,
    ///At which level and below the target shall be logged
    pub target: Option<Level>,
    ///At which level and below a source code reference shall be logged
    pub location: Option<Level>,
    ///A chrono strftime string. See: https://docs.rs/chrono/0.4.0/chrono/format/strftime/index.html#specifiers
    pub time_format: Option<&'static str>,
    /// Chrono Offset used for logging time (default is UTC)
    pub offset: Option<FixedOffset>,
    /// Allowed module filters.
    /// If specified, only records from modules starting with one of these entries will be printed
    ///
    /// For example, `filter_allow: Some(&["tokio::uds"])` would allow only logging from the `tokio` crates `uds` module.
    pub filter_allow: Option<&'static [&'static str]>,
    /// Denied module filters.
    /// If specified, records from modules starting with one of these entries will be ignored
    ///
    /// For example, `filter_ignore: Some(&["tokio::uds"])` would deny logging from the `tokio` crates `uds` module.
    pub filter_ignore: Option<&'static [&'static str]>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            time: Some(Level::Error),
            level: Some(Level::Error),
            thread: Some(Level::Debug),
            target: Some(Level::Debug),
            location: Some(Level::Trace),
            time_format: None,
            offset: Some(Utc.fix()),
            filter_allow: None,
            filter_ignore: None,
        }
    }
}
