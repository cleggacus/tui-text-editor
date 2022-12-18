use crate::renderer::Renderer;

pub trait Node {
    fn draw(&mut self, renderer: &mut Renderer);
    fn get_size(&self) -> (u16, u16);
    fn set_size(&mut self, size: (u16, u16));
    fn get_pos(&self) -> (u16, u16);
    fn set_pos(&mut self, pos: (u16, u16));
}

pub trait ContainerNode: Node {
    fn get_children(&self) -> &Vec<Box<dyn Node>>;
}
