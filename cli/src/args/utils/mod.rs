use lazy_static::lazy_static;

lazy_static! {
    pub static ref HELP_MESSAGES: Vec<String> = vec![
        String::from("Config: set your OPENAI_KEY"),
        String::from("Prompt to use"),
        String::from("File containing the prompt"),
        String::from("Run in REPL mode"),
        String::from("Number of threads to use"),
    ];
}
