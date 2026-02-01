use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct AppLayout {
    pub header: Rect,
    pub content: Rect,
    pub statusbar: Rect,
}

impl AppLayout {
    pub fn new(area: Rect) -> Self {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // header / tab bar
                Constraint::Min(1),    // content
                Constraint::Length(1), // status bar
            ])
            .split(area);

        Self {
            header: chunks[0],
            content: chunks[1],
            statusbar: chunks[2],
        }
    }
}
