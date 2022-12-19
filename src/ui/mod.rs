use std::{rc::Rc, cell::RefCell, slice::{Iter, IterMut}};

use crate::renderer::Renderer;

pub mod flex;
pub mod label;

pub trait Node {
    fn draw(&mut self, renderer: &mut Renderer, offset_pos: Option<(u16, u16)>);
    fn children(&self) -> Option<Iter<Rc<RefCell<dyn Node>>>>;
    fn children_mut(&mut self) -> Option<IterMut<Rc<RefCell<dyn Node>>>>;
    fn add_child(&mut self, child: Rc<RefCell<dyn Node>>);
    fn set_size(&mut self, size: (u16, u16));
    fn get_size(&self) -> (u16, u16);
    fn set_pos(&mut self, pos: (u16, u16));
    fn get_pos(&self) -> (u16, u16);
}
