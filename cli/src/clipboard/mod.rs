use clipboard::{ClipboardContext, ClipboardProvider};
use std::io::{self, Write};
use termion::{color, style};

pub fn copy(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(text.to_owned())?;
    Ok(())
}

pub fn prompt_copy_to_clipboard(command: &str) -> io::Result<bool> {
    println!("\n{}", style::Bold);
    println!("{}╔══════════════════════════════════════════", color::Fg(color::Blue));
    println!("{}║ Command 😊: {}{}", color::Fg(color::Blue), color::Fg(color::Green), command);
    println!("{}║", color::Fg(color::Blue));
    println!("{}║ Do you want to copy this command to clipboard?", color::Fg(color::Blue));
    println!("{}╚══════════════════════════════════════════", color::Fg(color::Blue));
    print!("{}", style::Reset);

    print!("{}[Y/n]: {}", color::Fg(color::Yellow), color::Fg(color::Reset));
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_lowercase() != "n")
}
