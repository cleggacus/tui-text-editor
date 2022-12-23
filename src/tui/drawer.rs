use super::{style::{border::Border, Style, size::{Size2D, Size}, position::{Position2D, Position}, display::Display}};

use std::{rc::Rc, cell::{RefCell}};

use crate::renderer::{Renderer, Rect};

#[derive(Debug, Clone)]
pub struct Drawer {
    pub boundaries: Rect,
    current_position: (u16, u16),
    renderer: Rc<RefCell<Renderer>>,
}

impl Drawer {
    pub fn new(renderer: Rc<RefCell<Renderer>>) -> Self {
        let boundaries = renderer.borrow().boundaries();

        Drawer { 
            boundaries,
            current_position: (0, 0),
            renderer,
        }
    }

    pub fn draw(&mut self, style: &Style) {
        let boundaries = self.calc_onscreen_boundaries(style);

        if style.get_border() == Border::Line {
            self.renderer.borrow_mut().draw_box(boundaries);
        }

        self.current_position = (
            boundaries.x + boundaries.width,
            boundaries.y + boundaries.height,
        );
    }

    pub fn inner_drawer(&mut self, style: &Style) -> Drawer {
        let mut drawer = self.clone();
        drawer.boundaries = self.calc_inner_onscreen_boundaries(style);

        drawer
    }

    fn calc_inner_onscreen_boundaries(&mut self, style: &Style) -> Rect {
        let mut boundaries = self.calc_onscreen_boundaries(style);

        if style.get_border() == Border::Line {
            let width_diff = if boundaries.width < 2 {
                boundaries.width
            } else { 2 };

            let height_diff = if boundaries.height < 2 {
                boundaries.height
            } else { 2 };

            boundaries.x += width_diff / 2;
            boundaries.y += height_diff / 2;
            boundaries.width -= width_diff;
            boundaries.height -= height_diff;
        }

        boundaries
    }

    fn calc_onscreen_boundaries(&mut self, style: &Style) -> Rect {
        let x = self.calc_onscreen_x(style);
        let y = self.calc_onscreen_y(style);
        let width = self.calc_onscreen_width(style);
        let height = self.calc_onscreen_height(style);

        Rect {
            x, y, width, height
        }
    }

    fn calc_onscreen_height(&mut self, style: &Style) -> u16 {
        let Size2D (_, style_height) = style.get_size();
        let height = self.boundaries.height;

        self.calc_onscreen_size(style_height, height)
    }

    fn calc_onscreen_width(&mut self, style: &Style) -> u16 {
        let Size2D (style_width, _) = style.get_size();
        let width = self.boundaries.width;

        self.calc_onscreen_size(style_width, width)
    }

    fn calc_onscreen_size(&self, style_size: Size, boundary_size: u16) -> u16 {
        match style_size {
            Size::Auto => 0,
            Size::Exact(val) => val,
            Size::Percent(val) => {
                if val < 0.0 {
                    return 0;
                }

                let val = val * 0.01 * boundary_size as f64;
                val as u16
            }
        }
    }

    fn calc_onscreen_x(&mut self, style: &Style) -> u16 {
        let Position2D (style_x, _) = style.get_position();
        let x = self.boundaries.x;
        let width = self.boundaries.width;
        let (auto_width, _) = self.current_position;

        self.calc_onscreen_position(style_x, x, auto_width, width)
    }

    fn calc_onscreen_y(&mut self, style: &Style) -> u16 {
        let Position2D (_, style_y) = style.get_position();
        let y = self.boundaries.y;
        let height = self.boundaries.height;
        let (_, auto_height) = self.current_position;

        self.calc_onscreen_position(style_y, y, auto_height, height)
    }

    fn calc_onscreen_position(&self, style_position: Position, boundary_postiion: u16, auto_position: u16, boundary_size: u16) -> u16 {
        boundary_postiion + match style_position {
            Position::Auto => auto_position,
            Position::Exact(val) => val,
            Position::Percent(val) => {
                if val < 0.0 {
                    return 0;
                }

                let val = val * 0.01 * boundary_size as f64;
                val as u16
            }
        }
    }
}
