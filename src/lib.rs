use crossterm::{
    cursor::{Hide, Show},
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::ResetColor,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::error::Error;
use std::io;
use views::list::ListView;
use views::View;

mod events;
mod views;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let mut current_view: Box<dyn View> = Box::from(ListView::from(&[
        "Option 1",
        "Option 2",
        "Another width option",
    ]));

    enable_raw_mode()?;

    let rx = events::init_events();
    loop {
        let event = rx.recv()?;

        let _ = match event {
            events::Event::KeyPressed(ke) => match ke {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                } => {
                    break;
                }
                other => current_view.as_mut().process_key(other),
            },
            events::Event::Sync => Ok(None),
        };

        current_view.as_ref().draw(&mut stdout, size()?)?;
    }

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, ResetColor, Show)?;

    Ok(())
}
