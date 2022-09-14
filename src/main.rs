use crossterm::{
    event::{
        DisableMouseCapture,
        EnableMouseCapture,
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
    },
};

use std::{
    error::Error,
    io
};

use crate::ui::widget_ui;

mod ui;

use tui::{
    backend::CrosstermBackend,
    Terminal,
};


fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(widget_ui::ui)?;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}

