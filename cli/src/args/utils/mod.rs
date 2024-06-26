use lazy_static::lazy_static;

lazy_static! {
    pub static ref HELP_MESSAGES: Vec<String> = vec![
        String::from("Where to load the model path from"),
        String::from("Prompt to use"),
        String::from("File containing the prompt"),
        String::from("Number of threads to use"),
    ];
}
