use ratatui::DefaultTerminal;
use std::path::PathBuf;

use crate::{filesystem, ui};
use crate::event::Action;

pub struct App {
    pub running: bool,
    pub current_dir: PathBuf,

    pub current_entries: Vec<filesystem::model::FileEntry>,
    pub parent_entries: Vec<filesystem::model::FileEntry>,

    pub selected: usize,
    pub preview: filesystem::model::Preview,
}

// storing the entries in the app struct is okay, since we are messing with .. and ./entries[i]

impl App {
    pub fn new() -> Self {
        let current_dir = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("Unknown"));

        let entries = filesystem::collector::read_directory(&current_dir)
            .unwrap_or_default();

        let parent_entries = current_dir.parent()
            .and_then(|p| filesystem::collector::read_directory(p).ok())
            .unwrap_or_default();

        Self {
            running: true,
            current_dir: current_dir,
            current_entries: entries,
            parent_entries: parent_entries,
            selected: 0,
            preview: filesystem::model::Preview::None,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while self.running {
            terminal.draw(|frame| ui::render(frame, self))?;

            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                self.handle_event(key.into());
            }
        }

        Ok(())
    }

    pub fn handle_event(&mut self, event: Action) {
        match event {
            Action::Quit => self.running = false,
            Action::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            Action::Down => {
                if self.selected + 1 < self.current_entries.len() {
                    self.selected += 1;
                }
            }

            Action::Enter => {
                // later: open directory
            }

            Action::Unknown => {}
        }
    }

    fn parent_entries(&self) -> Vec<filesystem::model::FileEntry> {
        self.current_dir.parent()
            .and_then(|path| filesystem::collector::read_directory(path).ok())
            .unwrap_or_default()
    }

    fn refresh_entries(&mut self) {
        self.current_entries = filesystem::collector::read_directory(&self.current_dir)
            .unwrap_or_default();

        self.parent_entries = self.parent_entries();

        self.selected = 0;
    }
}