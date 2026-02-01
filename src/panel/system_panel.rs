use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Block, Borders, Gauge, Sparkline},
    Frame,
};

use crate::source::system::SystemSnapshot;
use crate::source::DataSnapshot;

use super::Panel;

const CPU_HISTORY_LEN: usize = 60;

pub struct SystemPanel {
    snapshot: SystemSnapshot,
    cpu_history: Vec<u64>,
}

impl SystemPanel {
    pub fn new() -> Self {
        Self {
            snapshot: SystemSnapshot::default(),
            cpu_history: Vec::with_capacity(CPU_HISTORY_LEN),
        }
    }
}

impl Panel for SystemPanel {
    fn on_data(&mut self, data: &DataSnapshot) {
        if let DataSnapshot::System(snap) = data {
            self.snapshot = snap.clone();

            let avg_cpu = if snap.cpu_usages.is_empty() {
                0.0
            } else {
                snap.cpu_usages.iter().sum::<f64>() / snap.cpu_usages.len() as f64
            };

            self.cpu_history.push(avg_cpu as u64);
            if self.cpu_history.len() > CPU_HISTORY_LEN {
                self.cpu_history.remove(0);
            }
        }
    }

    fn draw(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4), // CPU sparkline
                Constraint::Length(3), // Memory gauge
                Constraint::Length(3), // Disk gauge
                Constraint::Min(0),   // spacer
            ])
            .split(area);

        // CPU Sparkline
        let avg_cpu = if self.snapshot.cpu_usages.is_empty() {
            0.0
        } else {
            self.snapshot.cpu_usages.iter().sum::<f64>() / self.snapshot.cpu_usages.len() as f64
        };

        let cpu_block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" CPU: {avg_cpu:.1}% "));

        let sparkline = Sparkline::default()
            .block(cpu_block)
            .data(&self.cpu_history)
            .max(100)
            .style(Style::default().fg(Color::Green))
            .bar_set(symbols::bar::NINE_LEVELS);

        f.render_widget(sparkline, chunks[0]);

        // Memory Gauge
        let mem_pct = if self.snapshot.total_memory == 0 {
            0.0
        } else {
            self.snapshot.used_memory as f64 / self.snapshot.total_memory as f64
        };

        let mem_used_mb = self.snapshot.used_memory / (1024 * 1024);
        let mem_total_mb = self.snapshot.total_memory / (1024 * 1024);

        let mem_gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Memory "),
            )
            .gauge_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .ratio(mem_pct.min(1.0))
            .label(Span::raw(format!(
                "{mem_used_mb} MB / {mem_total_mb} MB ({:.1}%)",
                mem_pct * 100.0
            )));

        f.render_widget(mem_gauge, chunks[1]);

        // Disk Gauge
        let disk_pct = if self.snapshot.total_disk == 0 {
            0.0
        } else {
            self.snapshot.used_disk as f64 / self.snapshot.total_disk as f64
        };

        let disk_used_gb = self.snapshot.used_disk / (1024 * 1024 * 1024);
        let disk_total_gb = self.snapshot.total_disk / (1024 * 1024 * 1024);

        let disk_gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Disk "),
            )
            .gauge_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .ratio(disk_pct.min(1.0))
            .label(Span::raw(format!(
                "{disk_used_gb} GB / {disk_total_gb} GB ({:.1}%)",
                disk_pct * 100.0
            )));

        f.render_widget(disk_gauge, chunks[2]);
    }
}
