pub mod git_panel;
pub mod system_panel;

use ratatui::{layout::Rect, Frame};

use crate::source::DataSnapshot;

pub trait Panel {
    fn on_data(&mut self, snapshot: &DataSnapshot);
    fn draw(&self, f: &mut Frame, area: Rect);
}
