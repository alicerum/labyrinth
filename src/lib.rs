use crossterm::{
    cursor::{Hide, Show},
    event::KeyCode,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::error::Error;
use std::io;

mod events;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        Hide,
        SetForegroundColor(Color::Yellow),
        SetBackgroundColor(Color::Blue),
        Print("Hello World"),
        ResetColor,
    )?;

    enable_raw_mode()?;

    let rx = events::init_events();
    loop {
        let event = rx.recv()?;

        match event {
            events::Event::KeyPressed(ke) => match ke.code {
                KeyCode::Char(c) if c == 'q' => {
                    break;
                }
                _ => {
                    // TODO: process more keyboard events
                }
            },
            events::Event::Sync => {
                // TODO: process tick here
            }
        }
    }

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, ResetColor, Show)?;

    Ok(())
}
