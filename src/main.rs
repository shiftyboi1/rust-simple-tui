use std::io;
use std::io::stdout;
use crossterm::{ExecutableCommand};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use simple_terminal_select::*;

// SHIFT YOU BETTER READ THIS 
// todo add pub function to setup raw mode and enter alt screen so to not add crossterm everywhere
// todo add command to exit too

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut foot: Menu = Menu::new();
    foot.add_label("demomenu".to_string());
    foot.add_action("An action".to_string(), "six".to_string());
    foot.add_label("A line".to_string());
    foot.add_label("A line".to_string());
    foot.add_action("Slutprint".to_string(), "seven".to_string());
    foot.add_label("They call me the labler".to_string());
    foot.add_action("Slutprint".to_string(), "seven".to_string());

    let res=foot.render()?;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    if res == "seven" {
        println!("Slutprint selected");
    }

    // i feel horrible but c:

    Ok(())
}
