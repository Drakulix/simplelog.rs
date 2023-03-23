use crate::config::{TargetPadding, TimeFormat};
use crate::{Config, LevelPadding, ThreadLogMode, ThreadPadding};
use log::{LevelFilter, Record};
use std::io::{Error, Write};
use std::thread;
#[cfg(all(feature = "termcolor", feature = "ansi_term"))]
use termcolor::Color;

#[cfg(all(feature = "termcolor", feature = "ansi_term"))]
pub fn termcolor_to_ansiterm(color: &Color) -> Option<ansi_term::Color> {
    match color {
        Color::Black => Some(ansi_term::Color::Black),
        Color::Red => Some(ansi_term::Color::Red),
        Color::Green => Some(ansi_term::Color::Green),
        Color::Yellow => Some(ansi_term::Color::Yellow),
        Color::Blue => Some(ansi_term::Color::Blue),
        Color::Magenta => Some(ansi_term::Color::Purple),
        Color::Cyan => Some(ansi_term::Color::Cyan),
        Color::White => Some(ansi_term::Color::White),
        _ => None,
    }
}

#[inline(always)]
pub fn try_log<W, SF, RF>(
    config: &Config,
    record: &Record<'_>,
    write: &mut W,
    mut set_color: SF,
    mut reset_color: RF,
) -> Result<(), Error>
where
    W: Write + Sized,
    SF: FnMut(&mut W) -> Result<(), Error>,
    RF: FnMut(&mut W) -> Result<(), Error>,
{
    if should_skip(config, record) {
        return Ok(());
    }

    let (mut have_space, parts) = if config.output_format.format_parts.len() >= 2 {
        let (level, wrap_space, part) = config.output_format.format_parts[0];
        if record.level() <= level {
            write_part(
                record,
                write,
                config,
                part,
                &mut set_color,
                &mut reset_color,
            )?;
        }
        if wrap_space {
            write!(write, " ")?;
        }
        (wrap_space, &config.output_format.format_parts[1..])
    } else {
        (false, &config.output_format.format_parts[..])
    };

    for (level, wrap_space, part) in parts {
        if record.level() > *level {
            continue;
        }

        if *wrap_space && !have_space {
            write!(write, " ")?;
        }

        write_part(
            record,
            write,
            config,
            *part,
            &mut set_color,
            &mut reset_color,
        )?;

        if *wrap_space {
            write!(write, " ")?;
            have_space = true;
        } else {
            have_space = false;
        }
    }

    Ok(())
}

use crate::format::FormatPart;

#[inline(always)]
fn write_part<W, SF, RF>(
    record: &Record<'_>,
    write: &mut W,
    config: &Config,
    part: FormatPart,
    mut set_color: SF,
    mut reset_color: RF,
) -> Result<(), Error>
where
    W: Write + Sized,
    SF: FnMut(&mut W) -> Result<(), Error>,
    RF: FnMut(&mut W) -> Result<(), Error>,
{
    use FormatPart as FP;

    match part {
        FP::Time if config.time <= record.level() && config.time != LevelFilter::Off => {
            write_time(write, config)?;
        }
        FP::Level if config.level <= record.level() && config.level != LevelFilter::Off => {
            set_color(write)?;
            match write_level(record, write, config) {
                Ok(()) => reset_color(write)?,
                Err(e) => {
                    reset_color(write)?;
                    return Err(e);
                }
            };
        }
        FP::Thread if config.thread <= record.level() && config.thread != LevelFilter::Off => {
            match config.thread_log_mode {
                ThreadLogMode::IDs => {
                    write_thread_id(write, config)?;
                }
                ThreadLogMode::Names | ThreadLogMode::Both => {
                    write_thread_name(write, config)?;
                }
            }
        }
        FP::Target if config.target <= record.level() && config.target != LevelFilter::Off => {
            write_target(record, write, config)?
        }
        FP::Location
            if config.location <= record.level() && config.location != LevelFilter::Off =>
        {
            write_location(record, write)?;
        }
        FP::ModulePath => write_module_path(record, write)?,
        FP::Args => write_args(record, write)?,
        FP::Literal(literal) => write!(write, "{}", literal)?,
        _ => (),
    }

    Ok(())
}

#[inline(always)]
fn write_time<W>(write: &mut W, config: &Config) -> Result<(), Error>
where
    W: Write + Sized,
{
    use time::error::Format;
    use time::format_description::well_known::*;

    let time = time::OffsetDateTime::now_utc().to_offset(config.time_offset);
    let res = match config.time_format {
        TimeFormat::Rfc2822 => time.format_into(write, &Rfc2822),
        TimeFormat::Rfc3339 => time.format_into(write, &Rfc3339),
        TimeFormat::Custom(format) => time.format_into(write, &format),
    };
    match res {
        Err(Format::StdIo(err)) => return Err(err),
        Err(err) => panic!("Invalid time format: {}", err),
        _ => {}
    };

    // write!(write, " ")?;
    Ok(())
}

#[inline(always)]
fn write_level<W>(record: &Record<'_>, write: &mut W, config: &Config) -> Result<(), Error>
where
    W: Write + Sized,
{
    #[cfg(all(feature = "termcolor", feature = "ansi_term"))]
    let color = match &config.level_color[record.level() as usize] {
        Some(termcolor) => {
            if config.write_log_enable_colors {
                termcolor_to_ansiterm(termcolor)
            } else {
                None
            }
        }
        None => None,
    };

    let level = match config.level_padding {
        LevelPadding::Left => format!("{: >5}", record.level()),
        LevelPadding::Right => format!("{: <5}", record.level()),
        LevelPadding::Off => format!("{}", record.level()),
    };

    #[cfg(all(feature = "termcolor", feature = "ansi_term"))]
    match color {
        Some(c) => write!(write, "{}", c.paint(level))?,
        None => write!(write, "{}", level)?,
    };

    #[cfg(not(feature = "ansi_term"))]
    write!(write, "{}", level)?;

    Ok(())
}

#[inline(always)]
fn write_target<W>(record: &Record<'_>, write: &mut W, config: &Config) -> Result<(), Error>
where
    W: Write + Sized,
{
    // dbg!(&config.target_padding);
    match config.target_padding {
        TargetPadding::Left(pad) => {
            write!(write, "{target:>pad$}", pad = pad, target = record.target())?;
        }
        TargetPadding::Right(pad) => {
            write!(write, "{target:<pad$}", pad = pad, target = record.target())?;
        }
        TargetPadding::Off => {
            write!(write, "{}", record.target())?;
        }
    }

    Ok(())
}

#[inline(always)]
fn write_location<W>(record: &Record<'_>, write: &mut W) -> Result<(), Error>
where
    W: Write + Sized,
{
    let file = record.file().unwrap_or("<unknown>");
    if let Some(line) = record.line() {
        write!(write, "{}:{}", file, line)?;
    } else {
        write!(write, "{}:<unknown>", file)?;
    }
    Ok(())
}

fn write_thread_name<W>(write: &mut W, config: &Config) -> Result<(), Error>
where
    W: Write + Sized,
{
    if let Some(name) = thread::current().name() {
        match config.thread_padding {
            ThreadPadding::Left { 0: qty } => {
                write!(write, "({name:>0$})", qty, name = name)?;
            }
            ThreadPadding::Right { 0: qty } => {
                write!(write, "({name:<0$})", qty, name = name)?;
            }
            ThreadPadding::Off => {
                write!(write, "({})", name)?;
            }
        }
    } else if config.thread_log_mode == ThreadLogMode::Both {
        write_thread_id(write, config)?;
    }

    Ok(())
}

fn write_thread_id<W>(write: &mut W, config: &Config) -> Result<(), Error>
where
    W: Write + Sized,
{
    let id = format!("{:?}", thread::current().id());
    let id = id.replace("ThreadId(", "");
    let id = id.replace(")", "");
    match config.thread_padding {
        ThreadPadding::Left { 0: qty } => {
            write!(write, "({id:>0$} ", qty, id = id)?;
        }
        ThreadPadding::Right { 0: qty } => {
            write!(write, "({id:<0$})", qty, id = id)?;
        }
        ThreadPadding::Off => {
            write!(write, "({})", id)?;
        }
    }
    Ok(())
}

#[inline(always)]
fn write_module_path<W>(record: &Record<'_>, write: &mut W) -> Result<(), Error>
where
    W: Write + Sized,
{
    writeln!(write, "{}", record.module_path().unwrap_or("<unknown>"))?;
    Ok(())
}

#[inline(always)]
fn write_args<W>(record: &Record<'_>, write: &mut W) -> Result<(), Error>
where
    W: Write + Sized,
{
    writeln!(write, "{}", record.args())?;
    Ok(())
}

#[inline(always)]
pub fn should_skip(config: &Config, record: &Record<'_>) -> bool {
    // If a module path and allowed list are available
    match (record.target(), &*config.filter_allow) {
        (path, allowed) if !allowed.is_empty() => {
            // Check that the module path matches at least one allow filter
            if !allowed.iter().any(|v| path.starts_with(&**v)) {
                // If not, skip any further writing
                return true;
            }
        }
        _ => {}
    }

    // If a module path and ignore list are available
    match (record.target(), &*config.filter_ignore) {
        (path, ignore) if !ignore.is_empty() => {
            // Check that the module path does not match any ignore filters
            if ignore.iter().any(|v| path.starts_with(&**v)) {
                // If not, skip any further writing
                return true;
            }
        }
        _ => {}
    }

    false
}
