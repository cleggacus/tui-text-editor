#[derive(Debug, Clone, Copy)]
pub enum Display {
    Block,
    Flex
}

impl Default for Display {
    fn default() -> Self {
        Display::Block 
    }
}