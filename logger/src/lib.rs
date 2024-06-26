use std::env;
use env_logger::Builder;

pub fn setup() {
    let mut builder = Builder::new();
    builder.filter_level(log::LevelFilter::Info);

    if let Ok(rust_log) = env::var("RUST_LOG") {
        builder.parse_filters(&rust_log);
    }

    builder.init();
}
