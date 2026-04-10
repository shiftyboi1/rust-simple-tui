use std::io;
use std::io::Write;
use std::string::ToString;
use crossterm::{event};
use crossterm::event::{Event};
use crossterm::terminal;

enum MenuEntry {
    Label{label: String, header: String, footer: String},
    Action{ label: String, id: String, header: String, footer: String },
}

pub struct Menu {
    entries: Vec<MenuEntry>,
    cursor : usize,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            cursor: 0,
        }
    }

    pub fn add_label(&mut self, label: String, header: String, footer: String) {
        self.entries.push(MenuEntry::Label{label, header, footer});
    }

    pub fn add_action(&mut self, label: String, id: String, header: String, footer: String) {
        self.entries.push(MenuEntry::Action{label, id, header, footer});
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

    fn prepare_entry(string: &str, max_width: usize, left: &str, right: &str, repeat_border: bool) -> Vec<String> {
        let allowed_strlen = max_width - left.len() - right.len();
        // TODO take up less ram
        let chars : Vec<char> = string.chars().collect();
        let mut output: Vec<String> = Vec::new();

        for (i, chunk) in chars.chunks(allowed_strlen).enumerate() {
            let line_text: String = chunk.iter().collect();
            if (i == 0 || repeat_border) {
                output.push(format!("{}{}{}", left, line_text, right));
                continue
            }
            output.push(format!("{:width$}{}", "", line_text, width = left.len()));
        }
        output
    }

    fn prepare_render(&mut self) {
        while self.cursor < self.entries.len() && !self.is_selectable(self.cursor) {
            self.cursor += 1;
        }
        if self.cursor >= self.entries.len() {
            self.add_action("No actions provided, idiot.".to_string(), "".to_string(), "".to_string(), "".to_string());
            self.cursor = 0;
        }
    }

    fn render_frame(&self) -> io::Result<()> {
        let mut out = io::stdout();
        crossterm::queue!(out, crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;
        crossterm::queue!(out, crossterm::cursor::MoveTo(0, 0))?;

        let (width, _) = terminal::size().unwrap_or((80, 24));
        let mut newline_index = 0;
        for (i, entry) in self.entries.iter().enumerate() {
            let mut print_string : String = String::new();
            let mut header : String = String::new();
            let mut footer : String = String::new();
            match entry {
                MenuEntry::Label{label, ..} => print_string = label.to_string(),
                MenuEntry::Action{label, ..} => {
                    if i == self.cursor {
                        print_string = format!("> {}", label.to_string());
                    } else {
                        print_string = format!("  {}", label.to_string());
                    }
                }
            }
            for line in Self::prepare_entry(&print_string, 24, "| ", "", true) {
                write!(out, "{}", line)?;
                newline_index += 1;
                crossterm::queue!(out, crossterm::cursor::MoveTo(0, newline_index as u16))?
            }
        }
        out.flush()
    }

    pub fn render(&mut self) -> io::Result<String> {
        self.prepare_render();
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