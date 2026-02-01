use std::path::PathBuf;
use std::time::Duration;

use async_trait::async_trait;
use git2::Repository;

use super::{DataSnapshot, DataSource};
use crate::errors::Result;

#[derive(Debug, Clone, Default)]
pub struct GitSnapshot {
    pub repos: Vec<RepoStatus>,
}

#[derive(Debug, Clone)]
pub struct RepoStatus {
    pub name: String,
    pub branch: String,
    pub modified: usize,
    pub staged: usize,
    pub untracked: usize,
    pub ahead: usize,
    pub behind: usize,
}

pub struct GitSource {
    repo_paths: Vec<PathBuf>,
    interval: Duration,
}

impl GitSource {
    pub fn new(repo_paths: Vec<PathBuf>, interval_secs: u64) -> Self {
        Self {
            repo_paths,
            interval: Duration::from_secs(interval_secs),
        }
    }
}

fn collect_repo_status(path: &PathBuf) -> std::result::Result<RepoStatus, git2::Error> {
    let repo = Repository::open(path)?;

    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string_lossy().to_string());

    let branch = match repo.head() {
        Ok(head) => head
            .shorthand()
            .unwrap_or("HEAD")
            .to_string(),
        Err(_) => "no branch".to_string(),
    };

    let statuses = repo.statuses(None)?;

    let mut modified = 0;
    let mut staged = 0;
    let mut untracked = 0;

    for entry in statuses.iter() {
        let s = entry.status();
        if s.intersects(
            git2::Status::WT_MODIFIED
                | git2::Status::WT_DELETED
                | git2::Status::WT_RENAMED
                | git2::Status::WT_TYPECHANGE,
        ) {
            modified += 1;
        }
        if s.intersects(
            git2::Status::INDEX_NEW
                | git2::Status::INDEX_MODIFIED
                | git2::Status::INDEX_DELETED
                | git2::Status::INDEX_RENAMED
                | git2::Status::INDEX_TYPECHANGE,
        ) {
            staged += 1;
        }
        if s.contains(git2::Status::WT_NEW) {
            untracked += 1;
        }
    }

    // Ahead/behind tracking branch
    let (ahead, behind) = match repo.head() {
        Ok(head) => {
            if let Some(local_oid) = head.target() {
                let branch_name = head.shorthand().unwrap_or("");
                let upstream_name = format!("refs/remotes/origin/{branch_name}");
                match repo.refname_to_id(&upstream_name) {
                    Ok(upstream_oid) => {
                        repo.graph_ahead_behind(local_oid, upstream_oid)
                            .unwrap_or((0, 0))
                    }
                    Err(_) => (0, 0),
                }
            } else {
                (0, 0)
            }
        }
        Err(_) => (0, 0),
    };

    Ok(RepoStatus {
        name,
        branch,
        modified,
        staged,
        untracked,
        ahead,
        behind,
    })
}

#[async_trait]
impl DataSource for GitSource {
    async fn collect(&mut self) -> Result<DataSnapshot> {
        let paths = self.repo_paths.clone();

        let repos = tokio::task::spawn_blocking(move || {
            paths
                .iter()
                .filter_map(|p| collect_repo_status(p).ok())
                .collect::<Vec<_>>()
        })
        .await
        .unwrap_or_default();

        Ok(DataSnapshot::Git(GitSnapshot { repos }))
    }

    fn interval(&self) -> Duration {
        self.interval
    }
}
