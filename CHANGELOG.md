## v0.5.2
    - Don't interleave stdout and stderr (PR #23, credits to @hansjorg)

## v0.5.1
    - Updated `simplelog` to `log` 0.4.1
    - Updated `simplelog` to `term` 0.5.1
    - Fixed compiler warning about unused `#[macro_use]`

## v0.5.0
    - Update `simplelog` to `log` 0.4

## v0.4.4
    - Fixed building non-default feature sets

## v0.4.3
    - Publically export TermLogger Error type

## v0.4.2
    - Removed a debug println! statement

## v0.4.1
    - Fix `None` config value

## v0.4.0
    - `Config` is not using `LogLevelFilter` anymore but `Option<LogLevel>`
        - `None` represents no logging of the `Config` parameter at all
        - `LogLevelFilter::Off` was supposed to provide this feature, but is actually
          ordered higher then `LogLevelFilter::Error`, and presents *no filtering* instead
          of the incorrectly assumed *filter everything*.

## v0.3.0
    - Merged PullRequest by *Antoni Boucher* - Avoid unwrapping in `TermLogger`:
        - `TermLogger::new` now returns an Option
        - `TermLogger::init` now has its own ErrorType
    - Move FileLogger to WriteLogger
        - More generic, accepts not only `File`, but everything that implements `Write`
    - Loggers now initialize with a `Config` struct, that list, what parts of the typical log message shall be printed, at which levels
        - Migrate from `::init(level, ...)` to `::init(level, Config::default(), ...)`
        - Migrate from `::new(level, ...)` or `::new(level, Config::default(), ...)`
        - Exception: `CombinedLogger` has no use for its own `Config` and does not take the new argument
        - Optionally use rust's `Default` syntax to override some settings
            - E.g. always print location: `Config { location: LogLevelFilter::Error, ..Default::default() }`
            - E.g. never print the target: `Config { target: LogLevelFilter::Off, ..Default::default() }`
    - Target does now by default only log for Debug and lower
    - Added CHANGELOG
    - Removed some internal code duplication

## v0.2.0
    - Local changes that (accidentially) made it to crates.io, but not git
        - Basically a worse version of *Antoni Boucher* 0.3.0 changes
        - Got noticed, when he made a Pull Request

    Sorry, what a mess. If you still have that version, here are the docs:
        https://docs.rs/simplelog/0.2.0/

## v0.1.0
    - Initial Release
