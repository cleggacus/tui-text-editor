use std::{io::{stdout, Stdout, BufWriter, Write}, collections::HashMap};

use crossterm::{
    terminal::{
        self
    },
    cursor::MoveTo, 
    style::Print,
    queue, 
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
const BOX_C: BoxChar = BoxChar { c: '┼', i: BOX_LEFT | BOX_RIGHT | BOX_TOP | BOX_BOTTOM };

const BOX_TL_CURVE: BoxChar = BoxChar { c: '╭', i: BOX_RIGHT | BOX_BOTTOM };
const BOX_TR_CURVE: BoxChar = BoxChar { c: '╮', i: BOX_LEFT | BOX_BOTTOM };
const BOX_BL_CURVE: BoxChar = BoxChar { c: '╰', i: BOX_RIGHT | BOX_TOP };
const BOX_BR_CURVE: BoxChar = BoxChar { c: '╯', i: BOX_LEFT | BOX_TOP };

const BOX_CHARS: [char; 16] = create_box_chars_arr();

const fn create_box_chars_arr () -> [char; 16] {
    let mut char_arr: [char; 16] = [0 as char; 16];

    char_arr[BOX_V.i] = BOX_V.c;
    char_arr[BOX_H.i] = BOX_H.c;
    char_arr[BOX_TL.i] = BOX_TL_CURVE.c;
    char_arr[BOX_TR.i] = BOX_TR_CURVE.c;
    char_arr[BOX_BL.i] = BOX_BL_CURVE.c;
    char_arr[BOX_BR.i] = BOX_BR_CURVE.c;
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

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug)]
pub struct Renderer {
    box_chars: HashMap<(u16, u16), usize>,
    offscreen_buf: Vec<Vec<char>>,
    stdout_buf: BufWriter<Stdout>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            offscreen_buf: Vec::new(),
            stdout_buf: BufWriter::with_capacity(u16::MAX as usize, stdout()),
            box_chars: HashMap::new(),
        }
    }

    pub fn get_stdout_buf(&mut self) -> &mut BufWriter<Stdout> {
        &mut self.stdout_buf
    }

    pub fn refresh(&mut self) {
        for line in 0..self.offscreen_buf.len() {
            queue!(self.stdout_buf, MoveTo(0, line as u16))
                .expect("Could not move cursor");

            for c in self.offscreen_buf[line].iter() {
                queue!(self.stdout_buf, Print(c))
                    .expect("Could not move cursor");
            }
        }

        self.stdout_buf.flush().unwrap();
    }

    pub fn clear (&mut self) {
        self.box_chars.clear();

        let boundaries = self.boundaries();
        let width = boundaries.width;
        let height = boundaries.height;

        self.offscreen_buf.resize(height as usize, Vec::new());

        for line in self.offscreen_buf.iter_mut() {
            line.resize(width as usize, ' ');
            line.fill(' ');
        }
    }

    pub fn boundaries (&self) -> Rect {
        let (width, height) = terminal::size()
            .expect("Could not get size");

        Rect {
            x: 0,
            y: 0,
            width,
            height
        }
    }

    pub fn draw_char_at (&mut self, x: u16, y: u16, c: char) {
        let x = x as usize;
        let y = y as usize;

        let offscreen_height = self.offscreen_buf.len();
        let offscreen_width: usize = match self.offscreen_buf.get(0) {
            None => 0,
            Some(val) => val.len()
        };

        if x < offscreen_width && y < offscreen_height {
            self.offscreen_buf[y][x] = c;
        }
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

    pub fn draw_box (&mut self, rect: Rect) {
        let x = rect.x;
        let y = rect.y;
        let w = rect.width;
        let h = rect.height;

        if w < 1 || h < 1 {
            return
        }

        self.draw_v_line(y+1, y+h-2, x);
        self.draw_v_line(y+1, y+h-2, x+w-1);

        self.draw_h_line(x+1, x+w-2, y);
        self.draw_h_line(x+1, x+w-2, y+h-1);

        self.draw_box_char_at(x, y, BOX_TL);
        self.draw_box_char_at(x+w-1, y, BOX_TR);
        self.draw_box_char_at(x, y+h-1, BOX_BL);
        self.draw_box_char_at(x+w-1, y+h-1, BOX_BR);

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
