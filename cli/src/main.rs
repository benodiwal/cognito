mod prompt;
mod args;
mod logger;
mod repl;

fn main() {
    logger::setup();

    let _args = args::parse();
    prompt::read(&_args);

    if _args.repl {
        repl::start();
    }
}
