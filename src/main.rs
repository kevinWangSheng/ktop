mod action;
mod app;
mod config;
mod errors;
mod event;
mod panel;
mod source;
mod tui;
mod ui;

use std::path::PathBuf;

use config::Config;
use source::spawn_sources;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let config_path = PathBuf::from("ktop.toml");
    let config = Config::load(&config_path).unwrap_or_default();

    let data_rx = spawn_sources(&config);
    let mut terminal = tui::init()?;
    let mut app = app::App::new(&config, data_rx);

    let result = app.run(&mut terminal).await;

    tui::restore()?;
    result
}
