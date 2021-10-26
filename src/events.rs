use crossterm::event;
use crossterm::event::KeyEvent;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub enum Event {
    Sync,
    KeyPressed(KeyEvent),
}

pub fn init_events() -> mpsc::Receiver<Event> {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(100));
        if let Err(_) = tx.send(Event::Sync) {
            return;
        }
    });

    thread::spawn(move || loop {
        if let event::Event::Key(ke) = event::read().unwrap() {
            if let Err(_) = tx2.send(Event::KeyPressed(ke)) {
                return;
            }
        }
    });

    return rx;
}
