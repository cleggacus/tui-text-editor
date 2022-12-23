#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Border {
    None,
    Line
}

impl Default for Border {
    fn default() -> Self {
        Border::None
    }
}
