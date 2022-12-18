use crate::renderer::{Renderer};

use super::{node::{ContainerNode, Node}};

#[derive(PartialEq)]
pub enum FlexDirection {
    Row,
    Column
}

pub enum FlexSize {
    Exact(u16),
    Grow
}

pub struct Flex {
    children: Vec<Box<dyn Node>>,
    direction: FlexDirection,
    layout: Vec<FlexSize>,
    size: (u16, u16),
    pos: (u16, u16)
}

impl Node for Flex {
    fn draw(&mut self, renderer: &mut Renderer) {
        self.update_layout();
        self.draw_children(renderer);

        let (x, y) = self.pos;
        let (width, height) = self.size;

        renderer.draw_box(x, y, width, height);
    }

    fn get_size(&self) -> (u16, u16) {
        self.size
    }

    fn set_size(&mut self, size: (u16, u16)) {
        self.size = size;
    }

    fn get_pos(&self) -> (u16, u16) {
        self.pos
    }

    fn set_pos(&mut self, pos: (u16, u16)) {
        self.pos = pos;
    }
}

impl ContainerNode for Flex {
    fn get_children(&self) -> &Vec<Box<dyn super::node::Node>> {
        &self.children
    }
} 

impl Flex {
    pub fn new() -> Self {
        Flex {
            children: Vec::new(),
            direction: FlexDirection::Row,
            layout: Vec::new(),
            size: (0, 0),
            pos: (0, 0)
        }
    }

    pub fn add_child(&mut self, node: Box<dyn Node>, size: FlexSize) {
        self.children.push(node);
        self.layout.push(size);
    }

    pub fn set_direction(&mut self, direction: FlexDirection) {
        self.direction = direction;
    }

    fn draw_children(&mut self, renderer: &mut Renderer) {
        for i in 0..self.children.len() {
            let child = self.children.get_mut(i)
                .expect("Could not get child");

            child.draw(renderer);
        }
    }

    fn update_layout(&mut self) {
        let (width, height) = self.size;
        let (x, y) = self.pos;
        let layout_len = self.layout.len();

        if layout_len > 0 {
            let mut grow_count = 0;
            let mut exact_count = 0;

            for flex_size in &self.layout {
                match flex_size {
                    FlexSize::Exact(val) => exact_count += val,
                    FlexSize::Grow => grow_count += 1
                }
            }

            let max_size = if self.direction == FlexDirection::Row {
                self.size.0 
            } else {
                self.size.1
            };

            let grow_size = (max_size - exact_count - (layout_len as u16) + 1) / grow_count;

            let len = if self.children.len() < layout_len {
                self.children.len()
            } else {
                layout_len
            };

            let mut offset: u16 = 0;

            for i in 0..len {
                let child = self.children.get_mut(i).unwrap();
                let flex_size = self.layout.get(i).unwrap();

                let size = match flex_size {
                    FlexSize::Exact(val) => *val,
                    FlexSize::Grow => grow_size
                };

                if self.direction == FlexDirection::Row {
                    child.set_size((size, height));
                    child.set_pos((x + offset, y));
                } else {
                    child.set_size((width, size));
                    child.set_pos((x, y + offset));
                }

                offset += size + 1;
            }
        }
    }
}