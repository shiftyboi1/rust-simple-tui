use std::io;
use simpletui::ui::Menu;
use rust_simple_tui::*;
use rust_simple_tui::simpletui::ui::{enter_tui, leave_tui};

fn main() -> io::Result<()> {

    let mut foot: Menu = Menu::default();
    foot.bordered_label("--------------", Some("┌-"), Some("-┐"), false);
    foot.bordered_action( "[X]", "" ,Some("| DEMOMENU "), Some(" |"), false);
    foot.bordered_label("--------------", Some("├-"), Some("-┤"), false);
    foot.bordered_action("An action", "six", Some("| "), Some(" |"), true);
    foot.bordered_label("A line", Some("| "), Some(" |"), true);
    foot.bordered_label("A line", Some("| "), Some(" |"), true);
    foot.bordered_action("Doggoprinter", "doggoprint ", Some("| "), Some(" |"), true);
    foot.bordered_label("the labler", Some("| "), Some(" |"), true);
    foot.bordered_label("--------------", Some("└-"), Some("-┘"), false);

    enter_tui()?;
    let res=foot.render(3)?;
    leave_tui()?;

    if res == "doggoprint" {
        println!("Doggoprint selected");
    }

    Ok(())
}
