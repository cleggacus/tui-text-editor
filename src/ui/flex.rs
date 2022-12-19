use std::{rc::Rc, cell::RefCell, slice::{Iter, IterMut}};

use crate::renderer::Renderer;

use super::{Node};

pub enum FlexType {
    Flex(u16),
    None
}

pub enum FlexDirection {
    Row, Column
}

pub struct Flex {
    children: Vec<Rc<RefCell<dyn Node>>>,
    flex_direction: FlexDirection,
    flexes: Vec<FlexType>,
    size: (u16, u16),
    pos: (u16, u16),
}

impl Node for Flex {
    fn draw(&mut self, renderer: &mut Renderer, parent: Option<(u16, u16)>) {
        let (offset_x, offset_y) = match parent {
            None => (0, 0),
            Some(val) => val
        };

        let (w, h) = self.size;
        let (x, y) = (self.pos.0 + offset_x, self.pos.1 + offset_y);

        self.update_layout();

        for child in self.children.iter_mut() {
            child.borrow_mut().draw(renderer, Some((x, y)));
        }

        renderer.draw_box(x, y, w, h);
    }

    fn children(&self) -> Option<Iter<Rc<RefCell<dyn Node>>>> {
        Some(self.children.iter())
    }

    fn children_mut(&mut self) -> Option<IterMut<Rc<RefCell<dyn Node>>>> {
        Some(self.children.iter_mut())
    }

    fn add_child(&mut self, child: Rc<RefCell<dyn Node>>) {
        self.children.push(child);
        self.flexes.push(FlexType::None);
    }

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

impl Flex {
    pub fn new() -> Rc<RefCell<Flex>> {
        let flex = Flex {
            flex_direction: FlexDirection::Row,
            children: Vec::new(),
            flexes: Vec::new(),
            size: (0, 0),
            pos: (0, 0),
        };

        Rc::new(RefCell::new(flex))
    }

    pub fn add_child_flex(&mut self, child: Rc<RefCell<dyn Node>>, flex_type: FlexType) {
        self.add_child(child);
        let last_index = self.flexes.len() - 1;
        self.flexes[last_index] = flex_type;
    }

    pub fn set_flex_direction(&mut self, flex_direction: FlexDirection) {
        self.flex_direction = flex_direction; 
    }

    fn update_layout(&mut self) {
        let full_size = match self.flex_direction {
            FlexDirection::Row => self.get_size().0,
            FlexDirection::Column => self.get_size().1
        };

        let mut flex_total = 0;
        let mut none_total = 0;


        for i in 0..self.children.len() {
            let flex = &self.flexes[i];
            let child = self.children[i].borrow();

            match flex {
                FlexType::None => none_total += match self.flex_direction {
                    FlexDirection::Row => child.get_size().0,
                    FlexDirection::Column => child.get_size().1
                },
                FlexType::Flex(val) => {
                    flex_total += val;
                }
            }
        }

        let flex_space = full_size - none_total + self.children.len() as u16 - 1 * !self.children.is_empty() as u16;

        let (flex_scale, mut flex_rem) = if flex_total != 0 {
            ( flex_space / flex_total,
              flex_space % flex_total )
        } else {
            (0, 0)
        };

        let mut current_pos = 0;
        for i in 0..self.children.len() {
            let flex = &self.flexes[i];
            let (child_width, child_height) = self.children[i].borrow().get_size();
            let (child_x, child_y) = self.children[i].borrow().get_pos();

            let mut child = self.children[i].borrow_mut();

            let mut size: u16 = match self.flex_direction {
                FlexDirection::Row => child_width,
                FlexDirection::Column => child_height
            };

            if let FlexType::Flex(val) = flex {
                size = (flex_scale * val) as u16;

                if flex_rem > 0 {
                    size += 1;
                    flex_rem -= 1;
                }

                match self.flex_direction {
                    FlexDirection::Row => child.set_size((size, self.get_size().1)),
                    FlexDirection::Column => child.set_size((self.get_size().0, size))
                }
            }

            match self.flex_direction {
                FlexDirection::Row => child.set_pos((current_pos, child_y)),
                FlexDirection::Column => child.set_pos((child_x, current_pos))
            }

            current_pos += size - (size > 1) as u16 * 1;
        }
    }
}