use std::{io::stdout, time::Duration};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, self};
use crossterm::{event, execute};

pub struct App {}

impl Drop for App {
    fn drop(&mut self) {
        self.cleanup_term();
    }
}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub fn start (&mut self) {
        self.setup_term();
        self.start_loop();
    }

    fn start_loop (&mut self) {
        loop {
            if event::poll(Duration::from_millis(500)).unwrap() {
                if let Ok(Event::Key(event)) = event::read() { 
                    match event {
                        KeyEvent {
                            code: KeyCode::Char('q'),
                            modifiers: event::KeyModifiers::NONE, 
                            kind: KeyEventKind::Press,
                            state: _
                        } => break,
                        _ => continue
                    }
                };
            } 
        }
    }

    fn setup_term (&mut self) {
        execute!(stdout(), EnterAlternateScreen)
            .expect("Could not enter alternate screen");

        terminal::enable_raw_mode()
            .expect("Could not turn on raw mode");
    }

    fn cleanup_term (&mut self) {
        terminal::disable_raw_mode()
            .expect("Could not disable raw mode");

        execute!(stdout(), LeaveAlternateScreen)
            .expect("Could not leave alternate screen");
    }
}