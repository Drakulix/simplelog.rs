fn main() {
    simplelog::TermLogger::init(simplelog::LevelFilter::Debug,
                                simplelog::Config::default(),
                                simplelog::TerminalMode::Mixed,
                                simplelog::ColorChoice::Auto).expect("Failed to start simplelog");

    simplelog::info!("I can write <b>bold</b> text or use tags to <red>color it</>");
}
