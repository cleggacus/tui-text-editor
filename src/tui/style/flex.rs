#[derive(Debug, Clone, Copy)]
pub enum Flex {
    None,
    Value(f64)
}

impl Default for Flex {
    fn default() -> Self {
        Flex::None
    }
}
