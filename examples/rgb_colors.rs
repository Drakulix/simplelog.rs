use log::*;
use simplelog::*;

#[cfg(all(not(target_family = "windows"), feature = "termcolor"))]
fn main() {
    let config = ConfigBuilder::new()
        .set_level_color(Level::Error, Color::Rgb(191, 0, 0))
        .set_level_color(Level::Warn,  Color::Rgb(255, 127, 0))
        .set_level_color(Level::Info,  Color::Rgb(192, 192, 0))
        .set_level_color(Level::Debug, Color::Rgb(63, 127, 0))
        .set_level_color(Level::Trace, Color::Rgb(127, 127, 255))
        .build();

    TermLogger::init(LevelFilter::Trace, config, TerminalMode::Stdout).unwrap();
    error!("Red error");
    warn!("Orange warning");
    info!("Yellow info");
    debug!("Dark green debug");
    trace!("Light blue trace");
}

#[cfg(any(target_family = "windows", not(feature = "termcolor")))]
fn main() {
    println!("this example requires the termcolor feature and a non-Windows OS.");
}