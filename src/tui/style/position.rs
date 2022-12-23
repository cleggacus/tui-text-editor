#[derive(Debug, Clone, Copy)]
pub enum Position {
    Auto,
    Exact(u16),
    Percent(f64),
}

impl Default for Position {
    fn default() -> Self {
        Position::Auto
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Position2D (pub Position, pub Position);
