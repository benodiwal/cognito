use rustyline::error::ReadlineError;
use log::error;

pub fn start() {
    let mut rl = rustyline::DefaultEditor::new().unwrap();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => println!("Line: {:?}", line),
            Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => break,
            Err(err) => error!("{}", err),
        }
    }

}
