use self::{position::Position2D, size::Size2D, border::{Border}, display::Display, flex::Flex};

pub mod border;
pub mod position;
pub mod size;
pub mod display;
pub mod flex;

#[derive(Default, Clone, Copy)]
pub struct Style {
    display: Display,
    position: Position2D,
    size: Size2D,
    border: Border,
    flex_border: Border,
    flex: Flex,
}

impl Style {
    pub fn set_flex(&mut self, flex: Flex) -> &mut Self {
        self.flex = flex;
        self 
    }

    pub fn get_flex(&self) -> Flex {
        self.flex
    }

    pub fn set_display(&mut self, display: Display) -> &mut Self {
        self.display = display;
        self 
    }

    pub fn get_display(&self) -> Display {
        self.display
    }

    pub fn set_position(&mut self, position: Position2D) -> &mut Self {
        self.position = position;
        self
    }

    pub fn get_position(&self) -> Position2D {
        self.position
    }

    pub fn set_size(&mut self, size: Size2D) -> &mut Self {
        self.size = size;
        self
    }

    pub fn get_size(&self) -> Size2D {
        self.size
    }

    pub fn set_flex_border(&mut self, flex_border: Border) -> &mut Self {
        self.flex_border = flex_border;
        self
    }

    pub fn get_flex_border(&self) -> Border {
        self.flex_border
    }

    pub fn set_border(&mut self, border: Border) -> &mut Self {
        self.border = border;
        self
    }

    pub fn get_border(&self) -> Border {
        self.border
    }
}