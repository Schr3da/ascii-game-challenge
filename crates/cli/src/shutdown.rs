use crossterm::event::DisableMouseCapture;
use crossterm::execute;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::LeaveAlternateScreen;
use std::io::{Error, Stdout};
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub fn terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<(), Error> {

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    Ok(())
}
