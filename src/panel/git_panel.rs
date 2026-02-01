use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::source::git::GitSnapshot;
use crate::source::DataSnapshot;

use super::Panel;

pub struct GitPanel {
    snapshot: GitSnapshot,
}

impl GitPanel {
    pub fn new() -> Self {
        Self {
            snapshot: GitSnapshot::default(),
        }
    }
}

impl Panel for GitPanel {
    fn on_data(&mut self, data: &DataSnapshot) {
        if let DataSnapshot::Git(snap) = data {
            self.snapshot = snap.clone();
        }
    }

    fn draw(&self, f: &mut Frame, area: Rect) {
        let header_cells = ["Repo", "Branch", "Modified", "Staged", "Untracked", "Ahead", "Behind"]
            .iter()
            .map(|h| {
                Cell::from(*h).style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            });
        let header = Row::new(header_cells).height(1);

        let rows: Vec<Row> = self
            .snapshot
            .repos
            .iter()
            .map(|repo| {
                let cells = vec![
                    Cell::from(repo.name.clone()),
                    Cell::from(repo.branch.clone()).style(Style::default().fg(Color::Cyan)),
                    Cell::from(repo.modified.to_string()).style(if repo.modified > 0 {
                        Style::default().fg(Color::Red)
                    } else {
                        Style::default().fg(Color::Green)
                    }),
                    Cell::from(repo.staged.to_string()).style(if repo.staged > 0 {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default().fg(Color::Green)
                    }),
                    Cell::from(repo.untracked.to_string()).style(if repo.untracked > 0 {
                        Style::default().fg(Color::Magenta)
                    } else {
                        Style::default().fg(Color::Green)
                    }),
                    Cell::from(repo.ahead.to_string()),
                    Cell::from(repo.behind.to_string()),
                ];
                Row::new(cells)
            })
            .collect();

        let widths = [
            ratatui::layout::Constraint::Min(15),
            ratatui::layout::Constraint::Min(15),
            ratatui::layout::Constraint::Length(10),
            ratatui::layout::Constraint::Length(8),
            ratatui::layout::Constraint::Length(11),
            ratatui::layout::Constraint::Length(7),
            ratatui::layout::Constraint::Length(8),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Git Repositories "),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(table, area);
    }
}
