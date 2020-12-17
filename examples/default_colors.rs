use log::*;
use simplelog::*;

#[cfg(feature = "termcolor")]
fn main() {
    TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Stdout).unwrap();
    error!("Red error");
    warn!("Yellow warning");
    info!("Blue info");
    debug!("Cyan debug");
    trace!("White trace");
}

#[cfg(not(feature = "termcolor"))]
fn main() {
    println!("this example requires the termcolor feature.");
}