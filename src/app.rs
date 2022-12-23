use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;
use std::{time::Duration};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, self};
use crossterm::{event, queue};

use crate::renderer::{Renderer};
use crate::tui::node::Node;
use crate::tui::style::Style;
use crate::tui::style::border::Border;
use crate::tui::style::display::{Display, FlexDirection};
use crate::tui::style::flex::Flex;
use crate::tui::style::position::{Position2D, Position};
use crate::tui::style::size::{Size2D, Size};

const FPS: u64 = 60;

pub struct App {
    root_ui: Node,
    renderer: Rc<RefCell<Renderer>>,
    term_size: (u16, u16),
    running: bool,
    last: Instant
}

impl Drop for App {
    fn drop(&mut self) {
        self.cleanup_term();
    }
}

impl App {
    pub fn new() -> Self {
        let renderer = Rc::new(RefCell::new(Renderer::new()));
        let boundaries = renderer.borrow().boundaries();
        let term_size = ( boundaries.width, boundaries.height );

        let child_style = *Style::default()
            .set_size(Size2D(
                Size::Auto, 
                Size::Percent(50.0)
            ))
            .set_flex(Flex::Value(1.0))
            .set_border(Border::Line);

        let mut root_ui = Node::default();

        root_ui.get_style()
            .set_display(Display::Flex(FlexDirection::Row))
            .set_size(Size2D(
                Size::Percent(100.0), 
                Size::Percent(100.0)
            ))
            .set_border(Border::Line)
            .set_flex_border(Border::Line);

        let child_0 = Rc::new(RefCell::new(Node::default()));
        let child_1 = Rc::new(RefCell::new(Node::default()));

        child_0.borrow_mut().set_style(child_style);
        child_1.borrow_mut().set_style(child_style);

        root_ui
            .add_child(child_0)
            .add_child(child_1);

        App {
            root_ui,
            renderer,
            term_size,
            running: false,
            last: Instant::now(),
        }
    }

    pub fn start (&mut self) {
        self.setup_term();
        self.draw();
        self.start_loop();
    }

    fn draw(&mut self) {
        self.renderer.borrow_mut().clear();
        self.root_ui.draw_root(self.renderer.clone());
    }
    
    fn update_term_size(&mut self) -> bool {
        let boundaries = self.renderer.borrow().boundaries();
        let term_size = ( boundaries.width, boundaries.height );

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

            self.draw();
            self.renderer.borrow_mut().refresh();

            self.last = Instant::now();
        }
    }

    fn process_event(&mut self) {
        if event::poll(Duration::from_nanos(1_000_000_000 / FPS)).unwrap() {
        // if event::poll(Duration::ZERO).unwrap() {
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
        queue!(self.renderer.borrow_mut().get_stdout_buf(), EnterAlternateScreen)
            .expect("Could not enter alternate screen");

        terminal::enable_raw_mode()
            .expect("Could not turn on raw mode");
    }

    fn cleanup_term (&mut self) {
        terminal::disable_raw_mode()
            .expect("Could not disable raw mode");

        queue!(self.renderer.borrow_mut().get_stdout_buf(), LeaveAlternateScreen)
            .expect("Could not leave alternate screen");
    }
}