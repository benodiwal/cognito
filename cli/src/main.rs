mod args;
mod logger;

fn main() {
    logger::setup();

    let _args = args::parse_args();
    let res = args::read_prompt(&_args);
    println!("{}", res);
}
