use crate::{Config, LevelPadding};
use chrono;
use log::{LevelFilter, Record};
use std::io::{Error, Write};
use std::thread;

#[inline(always)]
pub fn try_log<W>(config: &Config, record: &Record<'_>, write: &mut W) -> Result<(), Error>
where
    W: Write + Sized,
{
    if should_skip(config, record) {
        return Ok(());
    }

    if config.time <= record.level() && config.time != LevelFilter::Off {
        write_time(write, config)?;
    }

    if config.level <= record.level() && config.level != LevelFilter::Off {
        write_level(record, write, config)?;
    }

    if config.thread <= record.level() && config.thread != LevelFilter::Off {
        write_thread_id(write)?;
    }

    if config.target <= record.level() && config.target != LevelFilter::Off {
        write_target(record, write)?;
    }

    if config.location <= record.level() && config.location != LevelFilter::Off {
        write_location(record, write)?;
    }

    write_args(record, write)
}

#[inline(always)]
pub fn write_time<W>(write: &mut W, config: &Config) -> Result<(), Error>
where
    W: Write + Sized,
{
    let cur_time = if config.time_local {
        chrono::Local::now().naive_local() + config.time_offset
    } else {
        chrono::Utc::now().naive_utc() + config.time_offset
    };

    write!(write, "{} ", cur_time.format(&*config.time_format))?;
    Ok(())
}

#[inline(always)]
pub fn write_level<W>(record: &Record<'_>, write: &mut W, config: &Config) -> Result<(), Error>
where
    W: Write + Sized,
{
    match config.level_padding {
        LevelPadding::Left => write!(write, "[{: >5}] ", record.level())?,
        LevelPadding::Right => write!(write, "[{: <5}] ", record.level())?,
        LevelPadding::Off => write!(write, "[{}] ", record.level())?,
    };
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
    match (record.module_path(), &*config.filter_allow) {
        (Some(path), allowed) if allowed.len() > 0 => {
            // Check that the module path matches at least one allow filter
            if let None = allowed.iter().find(|v| path.starts_with(&***v)) {
                // If not, skip any further writing
                return true;
            }
        }
        _ => {}
    }

    // If a module path and ignore list are available
    match (record.module_path(), &*config.filter_ignore) {
        (Some(path), ignore) if ignore.len() > 0 => {
            // Check that the module path does not match any ignore filters
            if let Some(_) = ignore.iter().find(|v| path.starts_with(&***v)) {
                // If not, skip any further writing
                return true;
            }
        }
        _ => {}
    }

    return false;
}
