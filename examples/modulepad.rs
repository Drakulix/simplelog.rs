use log::*;
use simplelog::*;

mod a {
    use log::*;

    pub(crate) fn print(client_id: &str, connection_id: usize) {
        info!(
            "{:15.15}[I] {:20} id = {}",
            client_id, "connect", connection_id
        );
    }
}

mod aaaa {
    use log::*;

    pub fn print(client_id: &str, connection_id: usize) {
        info!(
            "{:15.15}[I] {:20} id = {}",
            client_id, "connect", connection_id
        );
    }
}

fn main() {
    let mut config = simplelog::ConfigBuilder::new();
    config
        .set_time_format("".to_owned())
        .set_location_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Error)
        .set_target_padding(TargetPadding::Right(15))
        .set_thread_level(LevelFilter::Error)
        .set_thread_padding(ThreadPadding::Left(2))
        .set_level_color(Level::Trace, Some(Color::Cyan))
        .set_level_padding(LevelPadding::Right);

    let loggers = TermLogger::new(
        LevelFilter::Debug,
        config.build(),
        TerminalMode::Mixed,
        ColorChoice::Always,
    );
    CombinedLogger::init(vec![loggers]).unwrap();

    error!("Bright red error");
    info!("This only appears in the log file");
    debug!("This level is currently not enabled for any logger");

    a::print("hello", 1);
    aaaa::print("world", 10);
}
