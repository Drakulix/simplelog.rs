# simplelog [![Build Status](https://travis-ci.org/Drakulix/simplelog.rs.svg?branch=master)](https://travis-ci.org/Drakulix/simplelog.rs) [![Build status](https://ci.appveyor.com/api/projects/status/mgmc2ro2d2pj5v04?svg=true)](https://ci.appveyor.com/project/Drakulix/simplelog.rs) [![Coverage Status](https://coveralls.io/repos/github/Drakulix/simplelog.rs/badge.svg?branch=master)](https://coveralls.io/github/Drakulix/simplelog.rs?branch=master) [![Crates.io](https://img.shields.io/crates/v/simplelog.svg)](https://crates.io/crates/simplelog) [![Crates.io](https://img.shields.io/crates/l/simplelog.svg)](https://crates.io/crates/simplelog)
## A simple and easy-to-use logging facility for Rust's [log](https://crates.io/crates/log) crate

`simplelog` does not aim to provide a rich set of features, nor to provide the
best logging solution. It aims to be a maintainable, easy to integrate facility
for small to medium sized projects, that find [env_logger](https://crates.io/crates/env_logger)
to be somewhat lacking in features. In those cases `simplelog` should provide an
easy alternative.

## Concept
`simplelog` provides a series of logging facilities, that can be easily combined.

- `SimpleLogger` (very basic logger that logs to stdout)
- `TermLogger` (advanced terminal logger, that splits to stdout/err and has color support) (can be excluded on unsupported platforms)
- `FileLogger` (logs to a given file)
- `CombinedLogger` (can be used to form combinations of the above loggers)

## Usage
```rust
#[macro_use]extern crate log;
extern crate simplelog;

use simplelog::{TermLogger, FileLogger, CombinedLogger, LogLevelFilter};

use std::fs::File;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Warn),
            FileLogger::new(LogLevelFilter::Info, File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();

    error!("Bright red error");
    info!("This only appears in the log file");
    debug!("This level is currently not enabled for any logger");
}
```

### Results in
```
$ cargo run --example usage
   Compiling simplelog v0.1.0 (file:///home/drakulix/Projects/simplelog)
     Running `target/debug/examples/usage`
[ERROR] Bright red error
```
and my_rust_binary.log
```
11:13:03 [ERROR] usage: Bright red error
11:13:03 [INFO] usage: This only appears in the log file
```

## Getting Started

Just add
```
[dependencies]
simplelog = "^0.1.0"
```
to your `Cargo.toml`

## [Documentation](https://drakulix.github.io/simplelog.rs/simplelog/index.html)

## Contributing
If you wish to contribute your own logger to `simplelog` or advance/extend existing loggers,
feel free to create a pull request. Just don't blindly assume, that your logger will be accepted.
The rational about this library is, that it is simple to use. This mostly comes down to a small
and easy API, but also includes things like the amount of added dependencies. If you feel unsure
about your plans, feel free to create an issue to talk about your ideas.

### Happy Coding!
