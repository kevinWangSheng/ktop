use crossterm::event::{KeyCode, KeyModifiers};
use tokio::sync::mpsc;

use crate::action::Action;
use crate::config::Config;
use crate::event::{Event, EventHandler};
use crate::panel::git_panel::GitPanel;
use crate::panel::system_panel::SystemPanel;
use crate::panel::Panel;
use crate::source::DataSnapshot;
use crate::tui::Tui;
use crate::ui::layout::AppLayout;
use crate::ui::statusbar::draw_statusbar;
use crate::ui::tabs::draw_tabs;

const TAB_TITLES: &[&str] = &["System", "Git"];

pub struct App {
    running: bool,
    selected_tab: usize,
    system_panel: SystemPanel,
    git_panel: GitPanel,
    data_rx: mpsc::UnboundedReceiver<DataSnapshot>,
    events: EventHandler,
}

impl App {
    pub fn new(config: &Config, data_rx: mpsc::UnboundedReceiver<DataSnapshot>) -> Self {
        let tick_rate = std::time::Duration::from_millis(config.tick_rate_ms);
        Self {
            running: true,
            selected_tab: 0,
            system_panel: SystemPanel::new(),
            git_panel: GitPanel::new(),
            data_rx,
            events: EventHandler::new(tick_rate),
        }
    }

    pub async fn run(&mut self, terminal: &mut Tui) -> color_eyre::Result<()> {
        while self.running {
            // Draw UI
            terminal.draw(|f| {
                let layout = AppLayout::new(f.area());
                draw_tabs(f, layout.header, TAB_TITLES, self.selected_tab);

                match self.selected_tab {
                    0 => self.system_panel.draw(f, layout.content),
                    1 => self.git_panel.draw(f, layout.content),
                    _ => {}
                }

                draw_statusbar(f, layout.statusbar);
            })?;

            // Handle events
            tokio::select! {
                Some(event) = self.events.next() => {
                    if let Some(action) = self.handle_event(event) {
                        self.dispatch(action);
                    }
                }
                Some(snapshot) = self.data_rx.recv() => {
                    self.system_panel.on_data(&snapshot);
                    self.git_panel.on_data(&snapshot);
                }
            }
        }
        Ok(())
    }

    fn handle_event(&self, event: Event) -> Option<Action> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => Some(Action::Quit),
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    Some(Action::Quit)
                }
                KeyCode::Tab => Some(Action::NextTab),
                KeyCode::BackTab => Some(Action::PrevTab),
                KeyCode::Char('r') => Some(Action::Refresh),
                _ => None,
            },
            Event::Tick => None,
            Event::Resize(_, _) => None,
        }
    }

    fn dispatch(&mut self, action: Action) {
        match action {
            Action::Quit => self.running = false,
            Action::NextTab => {
                self.selected_tab = (self.selected_tab + 1) % TAB_TITLES.len();
            }
            Action::PrevTab => {
                self.selected_tab = if self.selected_tab == 0 {
                    TAB_TITLES.len() - 1
                } else {
                    self.selected_tab - 1
                };
            }
            Action::Refresh => {
                // Will trigger data refresh in later phases
            }
        }
    }
}
