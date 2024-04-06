use std::io::{self, stdout, Stdout};

use crossterm::{execute, terminal::*};
use ratatui::prelude::*;

use crate::structs::Config;

use super::app::App;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Set's up the TUI for usage
pub fn setup_tui(config: &Config) -> io::Result<()> {
    let mut terminal = init()?;
    App::new(config).run(&mut terminal)
}

/// Enabled raw mode and an alternate screen, necessary for TUI's to work
pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Go back to normal terminal mode
pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
