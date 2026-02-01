use thiserror::Error;

#[derive(Error, Debug)]
pub enum KtopError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config error: {0}")]
    Config(#[from] toml::de::Error),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, KtopError>;
