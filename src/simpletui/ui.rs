use std::io;
use std::io::{stdout, Write};
use std::string::ToString;
use crossterm::{event};
use crossterm::event::{Event};
use crossterm::terminal;
use crossterm::{ExecutableCommand};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

enum MenuEntry {
    Label{label: String, header: String, footer: String},
    Action{ label: String, id: String, header: String, footer: String },
}

#[derive(Default)]
pub struct Menu {
    entries: Vec<MenuEntry>,
    cursor : usize,
}

pub fn enter_tui() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Ok(())
}

pub fn leave_tui() -> io::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

impl Menu {

    pub fn label(&mut self, label: impl Into<String>) {
        self.bordered_label(label, None, None);
    }

    pub fn action(&mut self, label: impl Into<String>, id: impl Into<String>) {
        self.bordered_action(label, id, None, None);
    }

    pub fn bordered_label(&mut self, label: impl Into<String>, header: Option<&str>, footer: Option<&str>) {
        self.entries.push(
            MenuEntry::Label {
                label: label.into(),
                header: header.map(str::to_owned).unwrap_or_default(),
                footer: footer.map(str::to_owned).unwrap_or_default()
            }
        );
    }



    pub fn bordered_action(&mut self, label: impl Into<String>, id: impl Into<String>, header: Option<&str>, footer: Option<&str>) {
        self.entries.push(
            MenuEntry::Action{
                label: label.into(),
                id: id.into(),
                header: header.map(str::to_owned).unwrap_or_default(),
                footer: footer.map(str::to_owned).unwrap_or_default()
            }
        );
    }

    fn select_at_index(&self, index: usize) -> Option<&String> {
        match self.entries[index] {
            MenuEntry::Action{ref id, ..} => Some(id),
            _ => None,
        }
    }

    fn is_selectable(&self, index: usize) -> bool {
        matches!(self.entries[index], MenuEntry::Action{..})
    }

    fn move_cursor(&mut self, direction: i32) {
        let mut new_cursor: i32 = self.cursor as i32;
        loop {
            new_cursor += direction;
            if new_cursor < 0 || (new_cursor as usize) >= self.entries.len() {
                return;
            }
            if self.is_selectable(new_cursor as usize) { break; }

        }
        self.cursor = new_cursor as usize
    }

    fn get_max_entry_width(&self) -> usize {
        // I know magic numbers are evil.
        self.entries.iter().map(|entry| {
            match entry {
                MenuEntry::Label{label, header, ..} => label.chars().count() + header.chars().count(),
                MenuEntry::Action{label, header, ..} => label.chars().count() + header.chars().count() + 2,
            }
        }).max().unwrap_or(6)
    }

    fn prepare_entry(string: &str, max_width: usize, left: &str, right: &str, longest_line: usize, repeat_border: bool) -> Vec<String> {
        let allowed_strlen = max_width - left.chars().count() - right.chars().count();
        if string.is_empty() {
            let padding = longest_line.saturating_sub(left.chars().count());
            return vec![format!("{}{:<padding$}{}", left, "", right, padding = padding)];
        }
        let chars : Vec<char> = string.chars().collect();

        chars.chunks(allowed_strlen)
            .enumerate()
            .map(|(i, chunk)| {
                let line_text: String = chunk.iter().collect();
                if i == 0 || repeat_border {
                    let padding = longest_line.saturating_sub(line_text.chars().count() + left.chars().count());
                    format!("{}{}{:padding$}{}", left, line_text, "", right, padding = padding)
                } else {
                    format!("{:width$}{}", "", line_text, width = left.chars().count())
                }
            }
        ).collect()
    }

    fn prepare_render(&mut self, index: usize) {
        if (index < self.entries.len()) && self.is_selectable(index) {
            self.cursor = index;
        } else {
            self.cursor = 0;
        }
        while self.cursor < self.entries.len() && !self.is_selectable(self.cursor) {
            self.cursor += 1;
        }
        if self.cursor >= self.entries.len() {
            self.entries.push(MenuEntry::Action {label: "[X]".to_string(), id: "".to_string(), header: "".to_string(), footer: "".to_string()});
            self.cursor = 0;
        }
    }

    fn render_frame(&self) -> io::Result<()> {
        let mut out = io::stdout();
        crossterm::queue!(out, terminal::Clear(terminal::ClearType::All))?;
        crossterm::queue!(out, crossterm::cursor::MoveTo(0, 0))?;

        let (width, _) = terminal::size().unwrap_or((80, 24));
        let longest_line = self.get_max_entry_width();
        let mut newline_index = 0;
        for (i, entry) in self.entries.iter().enumerate() {
            let print_string : String;
            let print_header : &String;
            let print_footer : &String ;
            match entry {
                MenuEntry::Label{label, header, footer} => {
                    print_header = header;
                    print_footer = footer;
                    print_string = label.to_string()
                },
                MenuEntry::Action{label, header, footer, .. } => {
                    print_header = header;
                    print_footer = footer;
                    if i == self.cursor {
                        print_string = format!("> {}", label);
                    } else {
                        print_string = format!("  {}", label);
                    }
                }
            }

            if (print_footer.chars().count() + print_header.chars().count() + 3) as u16 >= width {
                crossterm::queue!(out, terminal::Clear(terminal::ClearType::All))?;
                crossterm::queue!(out, crossterm::cursor::MoveTo(0, 0))?;
                write!(out, "Unsupported size. Please resize.")?;
                out.flush()?;
                return Ok(());
            }

            for line in Self::prepare_entry(&print_string, width as usize, print_header, print_footer, longest_line, true) {
                write!(out, "{}", line)?;
                newline_index += 1;
                crossterm::queue!(out, crossterm::cursor::MoveTo(0, newline_index as u16))?
            }
        }
        out.flush()
    }

    pub fn render(&mut self, index: usize) -> io::Result<String> {
        self.prepare_render(index);
        loop {
            self.render_frame()?;
            if let Event::Key(key) = event::read()? {
                match key.code {
                    event::KeyCode::Up => {
                        if self.cursor > 0 {
                            self.move_cursor(-1);
                        }
                    }
                    event::KeyCode::Down => {
                        if self.cursor < self.entries.len() - 1 {
                            self.move_cursor(1);
                        }
                    }
                    event::KeyCode::Enter => {
                        if self.is_selectable(self.cursor) {
                            return Ok(self.select_at_index(self.cursor).unwrap().to_string());
                        }
                    }
                    _ => {}
                }
            };
        }
    }
}