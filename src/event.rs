use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use futures::StreamExt;
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum Event {
    Key(KeyEvent),
    Tick,
    Resize(u16, u16),
}

pub struct EventHandler {
    rx: mpsc::UnboundedReceiver<Event>,
    _tx: mpsc::UnboundedSender<Event>,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let _tx = tx.clone();

        tokio::spawn(async move {
            let mut reader = event::EventStream::new();
            let mut tick_interval = tokio::time::interval(tick_rate);

            loop {
                tokio::select! {
                    _ = tick_interval.tick() => {
                        let _ = tx.send(Event::Tick);
                    }
                    Some(Ok(evt)) = reader.next() => {
                        match evt {
                            CrosstermEvent::Key(key) => {
                                let _ = tx.send(Event::Key(key));
                            }
                            CrosstermEvent::Resize(w, h) => {
                                let _ = tx.send(Event::Resize(w, h));
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        Self { rx, _tx }
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.rx.recv().await
    }
}
