use clap::Parser;
use utils::HELP_MESSAGES;

mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, short, help = HELP_MESSAGES[0].as_str())]
    pub model_path: String,

    #[arg(long, short, default_value = None, help = HELP_MESSAGES[1].as_str())]
    pub prompt: Option<String>,

    #[arg(long, short = 'f', default_value = None, help = HELP_MESSAGES[2].as_str())]
    pub prompt_file: Option<String>,

   #[arg(long, short, default_value_t = 4, help = HELP_MESSAGES[3].as_str())]
    pub threads: usize,
}

pub fn parse_args() -> Args {
    Args::parse()
}
