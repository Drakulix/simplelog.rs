use log::LogLevelFilter;

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
/// E.g. to have one part shown always use `LogLevelFilter::Error`. But if you
/// want to show the source line only on `Trace` use that.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config
{
    ///At which level and below the current time shall be logged
    pub time: LogLevelFilter,
    ///At which level and below the level itself shall be logged
    pub level: LogLevelFilter,
    ///At which level and below the target shall be logged
    pub target: LogLevelFilter,
    ///At which level and below a source code reference shall be logged
    pub location: LogLevelFilter,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            time: LogLevelFilter::Error,
            level: LogLevelFilter::Error,
            target: LogLevelFilter::Debug,
            location: LogLevelFilter::Trace,
        }
    }
}
