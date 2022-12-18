use std::{io::stdout, collections::HashMap};

use crossterm::{
    terminal::{
        Clear,
        ClearType, self
    },
    cursor::MoveTo, 
    style::Print,
    execute, 
};

struct BoxChar {
    pub c: char,
    pub i: usize
}

const BOX_LEFT: usize = 1;
const BOX_BOTTOM: usize = 2;
const BOX_RIGHT: usize = 4;
const BOX_TOP: usize = 8;

const BOX_V: BoxChar = BoxChar { c: '│', i: BOX_TOP | BOX_BOTTOM };
const BOX_H: BoxChar = BoxChar { c: '─', i: BOX_LEFT | BOX_RIGHT };
const BOX_TL: BoxChar = BoxChar { c: '┌', i: BOX_RIGHT | BOX_BOTTOM };
const BOX_TR: BoxChar = BoxChar { c: '┐', i: BOX_LEFT | BOX_BOTTOM };
const BOX_BL: BoxChar = BoxChar { c: '└', i: BOX_RIGHT | BOX_TOP };
const BOX_BR: BoxChar = BoxChar { c: '┘', i: BOX_LEFT | BOX_TOP };
const BOX_CL: BoxChar = BoxChar { c: '├', i: BOX_RIGHT | BOX_TOP | BOX_BOTTOM };
const BOX_CR: BoxChar = BoxChar { c: '┤', i: BOX_LEFT | BOX_TOP | BOX_BOTTOM };
const BOX_CT: BoxChar = BoxChar { c: '┬', i: BOX_LEFT | BOX_RIGHT | BOX_BOTTOM };
const BOX_CB: BoxChar = BoxChar { c: '┴', i: BOX_LEFT | BOX_RIGHT | BOX_TOP };
const BOX_C: BoxChar = BoxChar { c: '┴', i: BOX_LEFT | BOX_RIGHT | BOX_TOP | BOX_BOTTOM };

const BOX_CHARS: [char; 16] = create_box_chars_arr();

const fn create_box_chars_arr () -> [char; 16] {
    let mut char_arr: [char; 16] = [0 as char; 16];

    char_arr[BOX_V.i] = BOX_V.c;
    char_arr[BOX_H.i] = BOX_H.c;
    char_arr[BOX_TL.i] = BOX_TL.c;
    char_arr[BOX_TR.i] = BOX_TR.c;
    char_arr[BOX_BL.i] = BOX_BL.c;
    char_arr[BOX_BR.i] = BOX_BR.c;
    char_arr[BOX_CL.i] = BOX_CL.c;
    char_arr[BOX_CR.i] = BOX_CR.c;
    char_arr[BOX_CT.i] = BOX_CT.c;
    char_arr[BOX_CB.i] = BOX_CB.c;
    char_arr[BOX_C.i] = BOX_C.c;

    char_arr
}

pub struct CharCouple (char, char);

impl PartialEq for CharCouple {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) ||
        (self.0 == other.1 && self.1 == other.0)
    }
}

pub struct Renderer {
    box_chars: HashMap<(u16, u16), usize>
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            box_chars: HashMap::new()
        }
    }

    pub fn clear (&mut self) {
        self.box_chars.clear();

        execute!(stdout(), Clear(ClearType::All))
            .expect("Cound not clear");
    }

    pub fn get_size (&self) -> (u16, u16) {
        terminal::size()
            .expect("Could not get size")
    }

    pub fn draw_char_at (&mut self, x: u16, y: u16, c: char) {
        execute!(stdout(), MoveTo(x, y))
            .expect("Could not move cursor");

        execute!(stdout(), Print(c))
            .expect("Could not move cursor");
    }

    pub fn draw_h_line(&mut self, x_start: u16, x_end: u16, y: u16) {
        for i in x_start..=x_end {
            self.draw_box_char_at(i, y, BOX_H);
        }
    }

    pub fn draw_v_line(&mut self, y_start: u16, y_end: u16, x: u16) {
        for i in y_start..=y_end {
            self.draw_box_char_at(x, i, BOX_V);
        }
    }

    pub fn draw_box (&mut self, x: u16, y: u16, w: u16, h: u16) {
        self.draw_v_line(y+1, y+h, x);
        self.draw_v_line(y+1, y+h, x+w+1);

        self.draw_h_line(x+1, x+w, y);
        self.draw_h_line(x+1, x+w, y+h+1);

        self.draw_box_char_at(x, y, BOX_TL);
        self.draw_box_char_at(x+w+1, y, BOX_TR);
        self.draw_box_char_at(x, y+h+1, BOX_BL);
        self.draw_box_char_at(x+w+1, y+h+1, BOX_BR);

    }

    fn draw_box_char_at(&mut self, x: u16, y: u16, box_char: BoxChar) {
        let combine_layout = match self.box_chars.get(&(x, y)) {
            None => box_char.i,
            Some(val) => box_char.i | val
        };

        let box_char = BOX_CHARS[combine_layout];

        self.draw_char_at(x, y, box_char);
        self.box_chars.insert((x, y), combine_layout);
    }
}
