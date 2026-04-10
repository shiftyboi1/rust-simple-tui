use std::io;
use std::io::stdout;
use crossterm::{ExecutableCommand};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use simpletui::ui::Menu;
use rust_simple_tui::*;

// todo add pub function to setup raw mode and enter alt screen so to not add crossterm everywhere

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut foot: Menu = Menu::new();
    foot.label("", "|=", " |");
    foot.label("demomenu", "| ", " |");
    foot.label("", "|=", " |");
    foot.action("An action", "six", "| ", " |");
    foot.label("A line", "| ", " |");
    foot.label("A line", "| ", " |");
    foot.action("Doggoprint", "doggoprint ", "| ", " |");
    foot.label("the labler", "| ", " |");

    let res=foot.render()?;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    if res == "doggoprint" {
        println!("Doggoprint selected");
    }

    Ok(())
}
