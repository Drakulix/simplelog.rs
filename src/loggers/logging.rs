use chrono;
use log::Record;
use std::io::{Error, Write};
use Config;

#[inline(always)]
pub fn try_log<W>(config: &Config, record: &Record, write: &mut W) -> Result<(), Error>
where
    W: Write + Sized,
{
    if let Some(time) = config.time {
        if time <= record.level() {
            try!(write_time(write, config));
        }
    }

    if let Some(level) = config.level {
        if level <= record.level() {
            try!(write_level(record, write));
        }
    }

    if let Some(target) = config.target {
        if target <= record.level() {
            try!(write_target(record, write));
        }
    }

    if let Some(location) = config.location {
        if location <= record.level() {
            try!(write_location(record, write));
        }
    }

    try!(write_args(record, write));
    Ok(())
}

#[inline(always)]
pub fn write_time<W>(write: &mut W, config: &Config) -> Result<(), Error>
where
    W: Write + Sized,
{
    let time_format = config.time_format.unwrap_or("%H:%M:%S");
    if config.local_timezone {
        write!(write, "{} ", chrono::Local::now().format(time_format))
    } else {
        write!(write, "{} ", chrono::Utc::now().format(time_format))
    }
}

#[inline(always)]
pub fn write_level<W>(record: &Record, write: &mut W) -> Result<(), Error>
where
    W: Write + Sized,
{
    try!(write!(write, "[{}] ", record.level()));
    Ok(())
}

#[inline(always)]
pub fn write_target<W>(record: &Record, write: &mut W) -> Result<(), Error>
where
    W: Write + Sized,
{
    try!(write!(write, "{}: ", record.target()));
    Ok(())
}

#[inline(always)]
pub fn write_location<W>(record: &Record, write: &mut W) -> Result<(), Error>
where
    W: Write + Sized,
{
    let file = record.file().unwrap_or("<unknown>");
    if let Some(line) = record.line() {
        try!(write!(write, "[{}:{}] ", file, line));
    } else {
        try!(write!(write, "[{}:<unknown>] ", file));
    }
    Ok(())
}

#[inline(always)]
pub fn write_args<W>(record: &Record, write: &mut W) -> Result<(), Error>
where
    W: Write + Sized,
{
    try!(writeln!(write, "{}", record.args()));
    Ok(())
}
