#[allow(missing_docs)]
#[macro_export]
macro_rules! info {
    ($($args:tt)+) => {
        log::info!("{}", paris::formatter::colorize_string(format!($($args)*)));
    };
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! debug {
    ($($args:tt)+) => {
        log::debug!("{}", paris::formatter::colorize_string(format!($($args)*)));
    };
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! trace {
    ($($args:tt)+) => {
        log::trace!("{}", paris::formatter::colorize_string(format!($($args)*)));
    };
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! warn {
    ($($args:tt)+) => {
        log::warn!("{}", paris::formatter::colorize_string(format!($($args)*)));
    };
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! error {
    ($($args:tt)+) => {
        log::error!("{}", paris::formatter::colorize_string(format!($($args)*)));
    };
}
