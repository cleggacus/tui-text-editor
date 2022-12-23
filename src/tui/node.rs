use std::{rc::Rc, cell::RefCell};

use crate::renderer::Renderer;

use super::{style::{Style, display::Display}, drawer::Drawer};

#[derive(Default)]
pub struct Node {
    style: Style,
    children: Vec<Rc<RefCell<Node>>>
}

impl Node {
    pub fn get_style(&mut self) -> &mut Style {
        &mut self.style
    }

    pub fn set_style(&mut self, style: Style) -> &mut Self {
        self.style = style;
        self
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Node>>) -> &mut Self {
        self.children.push(child);
        self
    }

    pub fn draw(&mut self, drawer: &mut Drawer) {
        let mut inner_drawer = drawer.inner_drawer(&self.style);

        match self.style.get_display() {
            Display::Block => self.draw_children_block(&mut inner_drawer),
            Display::Flex => self.draw_children_flex(&mut inner_drawer),
        }

        drawer.draw(&self.style);
    }

    pub fn draw_root(&mut self, renderer: Rc<RefCell<Renderer>>) {
        self.draw(&mut Drawer::new(renderer));
    }

    fn draw_children_block(&mut self, drawer: &mut Drawer) {
        for child in self.children.iter() {
            child.borrow_mut().draw(drawer);
        }
    }

    fn draw_children_flex(&mut self, drawer: &mut Drawer) {
        for child in self.children.iter() {
            child.borrow_mut().draw(drawer);
        }
    }
}
