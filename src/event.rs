use crossterm::event::{KeyCode, KeyEvent};

pub enum Action {
    Quit,
    Up,
    Down,
    Enter,
    Unknown,
}

// j -> down
// k -> up
// h -> parent
// l -> enter
impl From<KeyEvent> for Action {
    fn from(key: KeyEvent) -> Self {
        match key.code {
            KeyCode::Esc   | KeyCode::Char('q') => Action::Quit,
            KeyCode::Up    | KeyCode::Char('k') => Action::Up,
            KeyCode::Down  | KeyCode::Char('j') => Action::Down,
            KeyCode::Enter | KeyCode::Char('l') => Action::Enter,
            _ => Action::Unknown,
        }
    }
}