use log::LogLevel;

/// Configuration for the Loggers
///
/// All loggers print the message in the following form:
/// `00:00:00 [LEVEL] crate::module: [lib.rs::100] your_message`
/// Every space delimited part except the actual message is optional.
///
/// Pass this struct to your logger to change when these information shall
/// be logged. Every part can be enabled for a specific LogLevel and is then
/// automatically enable for all lower levels as well.
///
/// The Result is that the logging gets more detailed the more verbose it gets.
/// E.g. to have one part shown always use `LogLevel::Error`. But if you
/// want to show the source line only on `Trace` use that.
/// Passing `None` will completely disable the part.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config
{
    ///At which level and below the current time shall be logged
    pub time: Option<LogLevel>,
    ///At which level and below the level itself shall be logged
    pub level: Option<LogLevel>,
    ///At which level and below the target shall be logged
    pub target: Option<LogLevel>,
    ///At which level and below a source code reference shall be logged
    pub location: Option<LogLevel>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            time: Some(LogLevel::Error),
            level: Some(LogLevel::Error),
            target: Some(LogLevel::Debug),
            location: Some(LogLevel::Trace),
        }
    }
}
