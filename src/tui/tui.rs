use std::io::{self, stdout, Stdout};

use crossterm::{execute, terminal::*};
use ratatui::prelude::*;

use super::app::App;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn setup_tui() -> io::Result<()> {
    let mut terminal = init()?;
    App::new().run(&mut terminal)
}

pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode();
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}