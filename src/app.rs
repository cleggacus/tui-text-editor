use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;
use std::{time::Duration};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, self};
use crossterm::{event, queue};

use crate::renderer::{Renderer};
use crate::ui::{Node};
use crate::ui::flex::{Flex, FlexType, FlexDirection};
use crate::ui::label::Label;

const FPS: u64 = 60;

pub struct App {
    root_ui: Rc<RefCell<dyn Node>>,
    fps_label: Rc<RefCell<Label>>,
    renderer: Renderer,
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
        let renderer = Renderer::new();
        let term_size = renderer.get_size();

        let root_ui = Flex::new();
        root_ui.borrow_mut().set_flex_direction(FlexDirection::Column);

        let flex_0 = Flex::new();
        flex_0.borrow_mut().set_size((0, 3));

        let flex_1 = Flex::new();

        let flex_1_0 = Flex::new();
        flex_1_0.borrow_mut().set_size((5, 0));

        let flex_1_1 = Flex::new();
        let flex_1_2 = Flex::new();
        flex_1_2.borrow_mut().set_flex_direction(FlexDirection::Column);

        let flex_1_2_0 = Flex::new();
        let flex_1_2_1 = Flex::new();
        let flex_1_2_2 = Flex::new();
        let flex_1_2_3 = Flex::new();
        let flex_1_2_4 = Flex::new();

        let fps_label = Label::new();
        fps_label.borrow_mut().set_pos((1, 1));

        flex_1_2.borrow_mut().add_child_flex(flex_1_2_0, FlexType::Flex(1));
        flex_1_2.borrow_mut().add_child_flex(flex_1_2_1, FlexType::Flex(1));
        flex_1_2.borrow_mut().add_child_flex(flex_1_2_2, FlexType::Flex(1));
        flex_1_2.borrow_mut().add_child_flex(flex_1_2_3, FlexType::Flex(1));
        flex_1_2.borrow_mut().add_child_flex(flex_1_2_4, FlexType::Flex(1));

        let label_clone = Rc::clone(&fps_label);
        flex_0.borrow_mut().add_child(label_clone);

        flex_1.borrow_mut().add_child_flex(flex_1_0, FlexType::None);
        flex_1.borrow_mut().add_child_flex(flex_1_1, FlexType::Flex(2));
        flex_1.borrow_mut().add_child_flex(flex_1_2, FlexType::Flex(1));

        root_ui.borrow_mut().add_child_flex(flex_0, FlexType::None);
        root_ui.borrow_mut().add_child_flex(flex_1, FlexType::Flex(1));

        App {
            root_ui,
            fps_label: Rc::clone(&fps_label),
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
        self.renderer.clear();

        let fps = 1_000_000_000 / self.last.elapsed().as_nanos();
        let fps_string = format!(" FPS: {}", fps);
        self.fps_label.borrow_mut().set_text(fps_string.as_str());

        let mut root_ui = self.root_ui.borrow_mut();

        root_ui.set_size(self.renderer.get_size());
        root_ui.draw(&mut self.renderer, None);
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

            self.draw();
            self.renderer.refresh();

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
        queue!(self.renderer.get_stdout_buf(), EnterAlternateScreen)
            .expect("Could not enter alternate screen");

        terminal::enable_raw_mode()
            .expect("Could not turn on raw mode");
    }

    fn cleanup_term (&mut self) {
        terminal::disable_raw_mode()
            .expect("Could not disable raw mode");

        queue!(self.renderer.get_stdout_buf(), LeaveAlternateScreen)
            .expect("Could not leave alternate screen");
    }
}