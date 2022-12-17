use std::io::stdout;

use crossterm::{
    terminal::{
        Clear,
        ClearType, self
    },
    cursor::MoveTo, 
    style::Print,
    execute, 
};

const BOX_V: char = '│';
const BOX_H: char = '─';
const BOX_TL: char = '┌';
const BOX_TR: char = '┐';
const BOX_BL: char = '└';
const BOX_BR: char = '┘';
// const BOX_CL: char = '├';
// const BOX_CR: char = '┤';
// const BOX_CT: char = '┬';
// const BOX_CB: char = '┴';
// const BOX_C: char = '┼';

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn clear (&mut self) {
        execute!(stdout(), Clear(ClearType::All))
            .expect("Cound not clear");
    }

    pub fn draw_char_at (&mut self, x: u16, y: u16, c: char) {
        execute!(stdout(), MoveTo(x, y))
            .expect("Could not move cursor");

        execute!(stdout(), Print(c))
            .expect("Could not move cursor");
    }

    pub fn draw_box (&mut self, x: u16, y: u16, w: u16, h: u16) {
        for i in (y+1)..(y+h-1) {
            self.draw_char_at(x, i, BOX_V);
            self.draw_char_at(x+w-1, i, BOX_V);
        }

        for i in (x+1)..(x+w-1) {
            self.draw_char_at(i, y, BOX_H);
            self.draw_char_at(i, y+h-1, BOX_H);
        }

        self.draw_char_at(x, y, BOX_TL);
        self.draw_char_at(x+w-1, y, BOX_TR);
        self.draw_char_at(x, y+h-1, BOX_BL);
        self.draw_char_at(x+w-1, y+h-1, BOX_BR);
    }

    pub fn get_height (&self) -> u16 {
        self.get_size().1
    }

    pub fn get_width (&self) -> u16 {
        self.get_size().0
    }

    pub fn get_size (&self) -> (u16, u16) {
        terminal::size()
            .expect("Could not get size")
    }
}