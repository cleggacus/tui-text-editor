#[derive(Debug, Clone, Copy)]
pub enum Size {
    Auto,
    Exact(u16),
    Percent(f64),
}

impl Default for Size {
    fn default() -> Self {
        Size::Auto
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Size2D (pub Size, pub Size);