use std::{rc::Rc, cell::RefCell, slice::{Iter, IterMut}};

use crate::renderer::Renderer;

use super::{Node};

pub struct Label {
    text: String,
    size: (u16, u16),
    pos: (u16, u16),
}

impl Node for Label {
    fn draw(&mut self, renderer: &mut Renderer, parent: Option<(u16, u16)>) {
        let (offset_x, offset_y) = match parent {
            None => (0, 0),
            Some(val) => val
        };

        let (x, y) = (self.pos.0 + offset_x, self.pos.1 + offset_y);

        let w = self.text.len() as u16;

        for i in 0..w {
            renderer.draw_char_at(x+i, y, self.text.chars().nth(i as usize).unwrap());
        }
    }

    fn children(&self) -> Option<Iter<Rc<RefCell<dyn Node>>>> {
        None
    }

    fn children_mut(&mut self) -> Option<IterMut<Rc<RefCell<dyn Node>>>> {
        None
    }

    fn add_child(&mut self, _child: Rc<RefCell<dyn Node>>) {}

    fn set_size(&mut self, size: (u16, u16)) {
        self.size = size;
    }

    fn get_size(&self) -> (u16, u16) {
        self.size
    }

    fn set_pos(&mut self, pos: (u16, u16)) {
        self.pos = pos;
    }

    fn get_pos(&self) -> (u16, u16) {
        self.pos
    }
}

impl Label {
    pub fn new() -> Rc<RefCell<Label>> {
        let label = Label {
            text: String::new(),
            size: (0, 0),
            pos: (0, 0),
        };

        Rc::new(RefCell::new(label))
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = String::from(text);
    }
}