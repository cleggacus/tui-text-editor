#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Display {
    Block,
    Flex(FlexDirection)
}

impl Default for Display {
    fn default() -> Self {
        Display::Block 
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FlexDirection {
    Row,
    Column
}
