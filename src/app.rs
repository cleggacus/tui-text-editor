use std::{io::stdout, time::Duration};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, self};
use crossterm::{event, execute};

use crate::renderer::Renderer;


pub struct App {
    renderer: Renderer
}

impl Drop for App {
    fn drop(&mut self) {
        self.cleanup_term();
    }
}

impl App {
    pub fn new() -> Self {
        App {
            renderer: Renderer::new()
        }
    }

    pub fn start (&mut self) {
        self.setup_term();
        self.start_loop();
    }

    fn draw(&mut self) {
        let (w, h) = self.renderer.get_size();
        self.renderer.clear();
        self.renderer.draw_box(0, 0, w, h);
    }

    fn start_loop (&mut self) {
        loop {
            if event::poll(Duration::from_millis(500)).unwrap() {
                match event::read() {
                    Ok(Event::Resize(w, h)) => self.draw(),
                    _ => continue
                }

                // if let Ok(Event::Key(event)) = event::read() { 
                //     match event {
                //         KeyEvent {
                //             code: KeyCode::Char('q'),
                //             kind: KeyEventKind::Press,
                //             ..
                //         } => break,
                        // KeyEvent {
                        //     code: KeyCode::Up,
                        //     kind: KeyEventKind::Press,
                        //     ..
                        // } => h += 1,
                        // KeyEvent {
                        //     code: KeyCode::Right,
                        //     kind: KeyEventKind::Press,
                        //     ..
                        // } => w += 1,
                        // KeyEvent {
                        //     code: KeyCode::Down,
                        //     kind: KeyEventKind::Press,
                        //     ..
                        // } => h -= 1,
                        // KeyEvent {
                        //     code: KeyCode::Left,
                        //     kind: KeyEventKind::Press,
                        //     ..
                        // } => w -= 1,
                        // _ => continue
                    // }
                // };
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