use cli::*;
use config::{check_config, set_config};

fn main() {
    logger::setup();
    let _args = args::parse();
        
    set_config(&_args);

    if check_config() {
        prompt::read(&_args);
        if _args.repl {
            repl::start();
        }
    }
    
}
