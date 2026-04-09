use std::io;
use std::io::Write;
use crossterm::{event};
use crossterm::event::{Event};

enum MenuEntry {
    Label(String),
    Action{ label: String, id: String },
}

pub struct Menu {
    entries: Vec<MenuEntry>,
    cursor : usize
}

impl Menu {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            cursor: 0,
        }
    }

    pub fn add_label(&mut self, label: String) {
        self.entries.push(MenuEntry::Label(label));
    }

    pub fn add_action(&mut self, label: String, id: String) {
        self.entries.push(MenuEntry::Action{label, id});
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

    fn prepare_render(&mut self) {
        while self.cursor < self.entries.len() && !self.is_selectable(self.cursor) {
            self.cursor += 1;
        }
        if self.cursor >= self.entries.len() {
            self.add_action("No actions provided, idiot.".to_string(), "".to_string());
            self.cursor = 0;
        }
    }

    fn render_frame(&self) -> io::Result<()> {
        let mut out = io::stdout();
        crossterm::queue!(out, crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;
        crossterm::queue!(out, crossterm::cursor::MoveTo(0, 0))?;

        for (i, entry) in self.entries.iter().enumerate() {
            crossterm::queue!(out, crossterm::cursor::MoveTo(0, i as u16))?;
            match entry {
                MenuEntry::Label(label) => write!(out ,"{}", label)?,
                MenuEntry::Action{label, ..} => {
                    if i == self.cursor {
                        write!(out,"> {}", label)?;
                    } else {
                        write!(out,"  {}", label)?;
                    }
                }
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