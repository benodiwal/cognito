use cli::*;
use config::{check_config, set_config};

#[tokio::main]
async fn main() {
    logger::setup();
    env::load();
    
    let _args = args::parse();
    set_config(&_args);

    if check_config() {
        prompt::read(&_args);
        if _args.repl {
            repl::start();
        }
    } 
}
