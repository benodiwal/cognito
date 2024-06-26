mod args;

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).parse_default_env().init();
    let _args = args::parse_args();
    let res = args::read_prompt(&_args);
    println!("{}", res);
}
