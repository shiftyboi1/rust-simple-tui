use std::io;
use simpletui::ui::Menu;
use intuitive::*;
use intuitive::simpletui::ui::{enter_tui, leave_tui};

fn main() -> io::Result<()> {

    let mut foot: Menu = Menu::default();
    foot.bordered_label("--------------", Some("┌-"), Some("-┐"), false);
    foot.bordered_action( "[X]", "" ,Some("| DEMOMENU "), Some(" |"), true);
    foot.bordered_label("--------------", Some("├-"), Some("-┤"), false);
    foot.bordered_action("An action", "one", Some("| "), Some(" |"), true);
    foot.bordered_label("A line", Some("| "), Some(" |"), true);
    foot.bordered_action("Doggodoggo", "doggoprint", Some("| "), Some(" |"), true);
    foot.bordered_label("Label", Some("| "), Some(" |"), true);
    foot.bordered_label("--------------", Some("└-"), Some("-┘"), false);

    enter_tui()?;
    let res=foot.render(3)?;
    leave_tui()?;

    if res == "doggoprint" {
        println!("Doggoprint selected");
    }

    Ok(())
}
