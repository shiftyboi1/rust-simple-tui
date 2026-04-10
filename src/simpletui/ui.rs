use std::cmp::max;
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

    pub fn label(&mut self, label: impl Into<String>, header: impl Into<String>, footer: impl Into<String>) {
        self.entries.push(MenuEntry::Label { label: label.into(), header: header.into(), footer: footer.into()});
    }

    pub fn action(&mut self, label: impl Into<String>, id: impl Into<String>, header: impl Into<String>, footer: impl Into<String>) {
        self.entries.push(MenuEntry::Action{label: label.into(), id: id.into(), header: header.into(), footer: footer.into()});
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
                MenuEntry::Label{label, header, ..} => label.len() + header.len() + 2,
                MenuEntry::Action{label, header, ..} => label.len() + header.len() + 2,
            }
        }).max().unwrap_or(6)
    }

    fn prepare_entry(string: &str, max_width: usize, left: &str, right: &str, longest_line: usize, repeat_border: bool) -> Vec<String> {
        let allowed_strlen = max_width - left.len() - right.len();
        // TODO take up less ram
        let chars : Vec<char> = string.chars().collect();
        let mut output: Vec<String> = Vec::new();

        for (i, chunk) in chars.chunks(allowed_strlen).enumerate() {
            let line_text: String = chunk.iter().collect();
            if i == 0 || repeat_border {
                let padding: usize = max(longest_line - line_text.len() - left.len(), 0);
                output.push(format!("{}{}{:padding$}{}", left, line_text, "",right));
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
            self.entries.push(MenuEntry::Action {label: "Exit".to_string(), id: "".to_string(), header: "".to_string(), footer: "".to_string()});
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

            if (print_footer.len() + print_header.len() + 3) as u16 >= width {
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