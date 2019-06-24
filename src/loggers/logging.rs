use chrono;
use chrono::offset::Offset;
use log::Record;
use std::io::{Error, Write};
use std::thread;
use crate::Config;

#[inline(always)]
pub fn try_log<W>(config: &Config, record: &Record<'_>, write: &mut W) -> Result<(), Error>
where
    W: Write + Sized,
{
    if should_skip(config, record) {
        return Ok(());
    }

    if let Some(time) = config.time {
        if time <= record.level() {
            write_time(write, config)?;
        }
    }

    if let Some(level) = config.level {
        if level <= record.level() {
            write_level(record, write)?;
        }
    }

    if let Some(thread) = config.thread {
        if thread <= record.level() {
            write_thread_id(write)?;
        }
    }

    if let Some(target) = config.target {
        if target <= record.level() {
            write_target(record, write)?;
        }
    }

    if let Some(location) = config.location {
        if location <= record.level() {
            write_location(record, write)?;
        }
    }

    write_args(record, write)
}

#[inline(always)]
pub fn write_time<W>(write: &mut W, config: &Config) -> Result<(), Error>
where
    W: Write + Sized,
{
    let offset = match config.offset {
        Some(offset) => {
            offset
        },
        None => {
            chrono::offset::Utc.fix()
        }
    };

    let cur_time = chrono::Utc::now().with_timezone::<chrono::offset::FixedOffset>(
        &chrono::TimeZone::from_offset(&offset));

    write!(
        write,
        "{} ",
        cur_time.format(config.time_format.unwrap_or("%H:%M:%S"))
    )?;
    Ok(())
}

#[inline(always)]
pub fn write_level<W>(record: &Record<'_>, write: &mut W) -> Result<(), Error>
where
    W: Write + Sized,
{
    write!(write, "[{: >5}] ", record.level())?;
    Ok(())
}

#[inline(always)]
pub fn write_target<W>(record: &Record<'_>, write: &mut W) -> Result<(), Error>
where
    W: Write + Sized,
{
    write!(write, "{}: ", record.target())?;
    Ok(())
}

#[inline(always)]
pub fn write_location<W>(record: &Record<'_>, write: &mut W) -> Result<(), Error>
where
    W: Write + Sized,
{
    let file = record.file().unwrap_or("<unknown>");
    if let Some(line) = record.line() {
        write!(write, "[{}:{}] ", file, line)?;
    } else {
        write!(write, "[{}:<unknown>] ", file)?;
    }
    Ok(())
}

pub fn write_thread_id<W>(write: &mut W) -> Result<(), Error>
where
    W: Write + Sized,
{
    let id = format!("{:?}", thread::current().id());
    let id = id.replace("ThreadId(", "");
    let id = id.replace(")", "");
    write!(write, "({}) ", id)?;
    Ok(())
}

#[inline(always)]
pub fn write_args<W>(record: &Record<'_>, write: &mut W) -> Result<(), Error>
where
    W: Write + Sized,
{
    writeln!(write, "{}", record.args())?;
    Ok(())
}

#[inline(always)]
pub fn should_skip(config: &Config, record: &Record<'_>) -> bool {
    // If a module path and allowed list are available
    if let (Some(path), Some(allowed)) = (record.module_path(), config.filter_allow) {
        // Check that the module path matches at least one allow filter
        if let None = allowed.iter().find(|v| path.starts_with(*v)) {
            // If not, skip any further writing
            return true;
        }
    }

    // If a module path and ignore list are available
    if let (Some(path), Some(ignore)) = (record.module_path(), config.filter_ignore) {
        // Check that the module path does not match any ignore filters
        if let Some(_) = ignore.iter().find(|v| path.starts_with(*v)) {
            // If not, skip any further writing
            return true;
        }
    }

    return false;
}
