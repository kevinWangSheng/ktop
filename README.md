# ktop

Terminal dashboard built with Ratatui.

## Features

- **System Monitor** — CPU sparkline, memory and disk gauges (via sysinfo)
- **Git Status** — multi-repo status table: branch, modified, staged, untracked, ahead/behind (via git2)
- Tab switching between panels
- TOML config (`ktop.toml`)

## Build & Run

```bash
cargo build
cargo run
```

## Keybindings

| Key | Action |
|-----|--------|
| `q` / `Ctrl+C` | Quit |
| `Tab` | Next panel |
| `Shift+Tab` | Previous panel |
| `r` | Refresh |

## Config

Edit `ktop.toml`:

```toml
tick_rate_ms = 250

[git]
interval_secs = 5
repos = [
    "/path/to/repo1",
    "/path/to/repo2",
]
```
