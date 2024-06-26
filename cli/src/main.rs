mod args;

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).parse_default_env().init();
    args::parse_args();
}
