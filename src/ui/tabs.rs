use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Tabs as RatatuiTabs},
    Frame,
};

pub fn draw_tabs(f: &mut Frame, area: Rect, titles: &[&str], selected: usize) {
    let titles: Vec<Line> = titles.iter().map(|t| Line::from(*t)).collect();

    let tabs = RatatuiTabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(" ktop "))
        .select(selected)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .divider(Span::raw(" | "));

    f.render_widget(tabs, area);
}
