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
    foot.add_label("demomenu".to_string());
    foot.add_action("An action".to_string(), "six".to_string());
    foot.add_label("A line".to_string());
    foot.add_label("A line".to_string());
    foot.add_action("Doggoprit".to_string(), "doggoprint ".to_string());
    foot.add_label("They call me the labler".to_string());

    let res=foot.render()?;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    if res == "doggoprint" {
        println!("Doggoprint selected");
    }

    Ok(())
}
