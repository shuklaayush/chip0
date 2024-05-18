use chip8_core::constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crossterm::{
    cursor::{Hide, Show},
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use eyre::{bail, Result};
use ratatui::{backend::CrosstermBackend, layout::Rect, Terminal};
use std::io::{stdout, Error, Stdout};

pub fn setup_terminal(headless: bool) -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;

    if !headless {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen, Hide)?;
        execute!(
            stdout(),
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES),
        )?;

        // Check terminal size
        let Rect { width, height, .. } = terminal.size()?;
        if width < 2 * DISPLAY_WIDTH as u16 {
            bail!(
                "Error: Terminal width {width} less than minimum width {}",
                2 * DISPLAY_WIDTH,
            );
        } else if height < DISPLAY_HEIGHT as u16 {
            bail!("Error: Terminal height {height} less than minimum height {DISPLAY_HEIGHT}");
        }
    }

    Ok(terminal)
}

pub fn restore_terminal(headless: bool) -> Result<(), Error> {
    if !headless {
        execute!(stdout(), PopKeyboardEnhancementFlags)?;
        execute!(stdout(), Show, LeaveAlternateScreen)?;
        disable_raw_mode()?;
    }

    Ok(())
}
