pub mod git;
pub mod system;

use async_trait::async_trait;
use std::time::Duration;
use tokio::sync::mpsc;

use crate::errors::Result;

#[derive(Debug, Clone)]
pub enum DataSnapshot {
    System(system::SystemSnapshot),
    Git(git::GitSnapshot),
}

#[async_trait]
pub trait DataSource: Send + 'static {
    async fn collect(&mut self) -> Result<DataSnapshot>;
    fn interval(&self) -> Duration;

    async fn run(mut self, tx: mpsc::UnboundedSender<DataSnapshot>)
    where
        Self: Sized,
    {
        let mut interval = tokio::time::interval(self.interval());
        loop {
            interval.tick().await;
            match self.collect().await {
                Ok(snapshot) => {
                    if tx.send(snapshot).is_err() {
                        break;
                    }
                }
                Err(e) => {
                    tracing::warn!("data source error: {e}");
                }
            }
        }
    }
}

pub fn spawn_sources(
    config: &crate::config::Config,
) -> mpsc::UnboundedReceiver<DataSnapshot> {
    let (tx, rx) = mpsc::unbounded_channel();

    // Spawn system source
    let sys_source = system::SystemSource::new();
    let sys_tx = tx.clone();
    tokio::spawn(async move {
        sys_source.run(sys_tx).await;
    });

    // Spawn git source
    let repo_paths: Vec<std::path::PathBuf> = config
        .git
        .repos
        .iter()
        .map(std::path::PathBuf::from)
        .collect();

    if !repo_paths.is_empty() {
        let git_source = git::GitSource::new(repo_paths, config.git.interval_secs);
        let git_tx = tx.clone();
        tokio::spawn(async move {
            git_source.run(git_tx).await;
        });
    }

    let _ = tx;

    rx
}
