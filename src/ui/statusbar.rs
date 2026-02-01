use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn draw_statusbar(f: &mut Frame, area: Rect) {
    let status = Paragraph::new(Line::from(vec![
        Span::styled(" q", Style::default().fg(Color::Yellow)),
        Span::raw(": Quit  "),
        Span::styled("Tab", Style::default().fg(Color::Yellow)),
        Span::raw(": Next  "),
        Span::styled("Shift+Tab", Style::default().fg(Color::Yellow)),
        Span::raw(": Prev  "),
        Span::styled("r", Style::default().fg(Color::Yellow)),
        Span::raw(": Refresh"),
    ]));

    f.render_widget(status, area);
}
