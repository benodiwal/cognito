mod prompt;
mod args;
mod repl;
mod client;
mod config;
mod env;
mod constants;

use config::{check_config, set_config};
use prompt::handle_prompt;

pub async fn init() {
    logger::setup();
    env::load();
    
    let _args = args::parse();
    set_config(&_args);

    if check_config() {
        let _pr = prompt::read(&_args);
        handle_prompt().await;

        if _args.repl {
            repl::start();
        }
    }
}
