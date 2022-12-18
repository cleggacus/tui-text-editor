use std::{io::stdout, time::Duration};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, self};
use crossterm::{event, execute};

use crate::renderer::{Renderer, self};
use crate::ui::flex::{Flex, FlexDirection, FlexSize};
use crate::ui::node::{ContainerNode, Node};


pub struct App {
    renderer: Renderer,
    ui_root: Box<dyn ContainerNode>,
    running: bool,
    term_size: (u16, u16)
}

impl Drop for App {
    fn drop(&mut self) {
        self.cleanup_term();
    }
}

impl App {
    pub fn new() -> Self {
        let renderer = Renderer::new();
        let mut ui_root = Box::new(Flex::new());

        let child0 = Box::new(Flex::new());
        let mut child1 = Box::new(Flex::new());

        let child2 = Box::new(Flex::new());
        let child3 = Box::new(Flex::new());
        let mut child4 = Box::new(Flex::new());

        let child5 = Box::new(Flex::new());
        let child6 = Box::new(Flex::new());

        child4.add_child(child5, FlexSize::Grow);
        child4.add_child(child6, FlexSize::Grow);
        child4.set_direction(FlexDirection::Column);

        child1.add_child(child2, FlexSize::Exact(10));
        child1.add_child(child3, FlexSize::Grow);
        child1.add_child(child4, FlexSize::Grow);

        ui_root.add_child(child0, FlexSize::Exact(1));
        ui_root.add_child(child1, FlexSize::Grow);

        ui_root.set_direction(FlexDirection::Column);

        ui_root.set_size(renderer.get_size());

        let term_size = renderer.get_size();

        App {
            renderer,
            ui_root,
            running: false,
            term_size
        }
    }

    pub fn start (&mut self) {
        self.setup_term();
        self.draw();
        self.start_loop();
    }

    fn draw(&mut self) {
        self.ui_root.set_size(self.renderer.get_size());
        self.renderer.clear();
        self.ui_root.draw(&mut self.renderer);
    }
    
    fn update_term_size(&mut self) -> bool {
        let term_size = self.renderer.get_size();

        if self.term_size != term_size {
            self.term_size = term_size;

            return true;
        }

        false
    }

    fn start_loop (&mut self) {
        self.running = true;

        while self.running {
            self.process_event();

            if self.update_term_size() {
                self.draw();
            }
        }
    }

    fn process_event(&mut self) {
        if event::poll(Duration::from_millis(100)).unwrap() {
            self.process_key_event();
        } 
    }

    fn process_key_event(&mut self) {
        if let Ok(Event::Key(event)) = event::read() { 
            match event {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    kind: KeyEventKind::Press,
                    ..
                } => self.running = false,
                KeyEvent {
                    code: KeyCode::Char('r'),
                    kind: KeyEventKind::Press,
                    ..
                } => self.draw(),
                _ => ()
            }
        };
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