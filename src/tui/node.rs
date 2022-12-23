use std::{rc::Rc, cell::RefCell};

use crate::renderer::{Renderer, Rect};

use super::{style::{Style, display::{Display, FlexDirection}, flex::Flex, size::{Size2D, Size}, position::{Position2D, Position}, border::Border}, drawer::Drawer};

#[derive(Default)]
pub struct Node {
    style: Style,
    children: Vec<Rc<RefCell<Node>>>
}

impl Node {
    pub fn get_style(&mut self) -> &mut Style {
        &mut self.style
    }

    pub fn set_style(&mut self, style: Style) -> &mut Self {
        self.style = style;
        self
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Node>>) -> &mut Self {
        self.children.push(child);
        self
    }

    pub fn draw(&mut self, drawer: &mut Drawer) {
        let mut inner_drawer = drawer.inner_drawer(&self.style);

        if let Display::Flex(direction) = self.style.get_display()  {
            match direction {
                FlexDirection::Row => self.apply_flex_row(drawer),
                FlexDirection::Column => self.apply_flex_column(),
            }
        }

        for child in self.children.iter() {
            child.borrow_mut().draw(&mut inner_drawer);
        }
         
        drawer.draw(&self.style);
    }

    pub fn draw_root(&mut self, renderer: Rc<RefCell<Renderer>>) {
        self.draw(&mut Drawer::new(renderer));
    }

    fn apply_flex_row(&mut self, drawer: &mut Drawer) {
        let boundaries = drawer.inner_drawer(&self.style).boundaries;

        let total_width = boundaries.width;

        let mut flex_count = 0;
        let mut flex_total = 0.0;
        let mut non_flex_width = 0;

        for child in self.children.iter() {
            let style = child.borrow().style;

            match style.get_flex() {
                Flex::None => {
                    let Size2D (width, _) = style.get_size();
                    let size = Drawer::calc_onscreen_size(width, total_width);
                    non_flex_width += size;
                }
                Flex::Value(val) => {
                    flex_count += 1;
                    flex_total += val;
                }
            }
        }

        let mut flex_width = total_width - non_flex_width;

        if self.get_style().get_flex_border() != Border::None {
            flex_width -= flex_count - 1;
        }

        let flex_scale = flex_width as f64 / flex_total;
        let mut x = 0;

        for child in self.children.iter() {
            let mut child = child.borrow_mut();

            if let Flex::Value(val) = child.get_style().get_flex() {
                let width = (val * flex_scale) as u16;
                let Size2D (_, height) = child.get_style().get_size();

                child.get_style()
                    .set_size(Size2D (Size::Exact(width), height));
            }

            let Size2D (width, _) = child.get_style().get_size();

            child.get_style()
                .set_position(Position2D (Position::Exact(x), Position::Exact(0)));

            x += Drawer::calc_onscreen_size(width, total_width);

            if self.style.get_flex_border() != Border::None {
                if x < total_width {
                    drawer.draw_flex_border(&self.style, x);
                }

                x += 1;
            }
        }
    }

    fn apply_flex_column(&mut self) {
        let child_styles: Vec<Style> = self.children.iter().map(
            |child| child.borrow_mut().style
        ).collect();
    }
}
