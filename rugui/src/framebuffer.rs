#[derive(Clone, Copy)]
pub enum Color {
    Black,
    White,
}

#[derive(PartialEq)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

pub struct Framebuffer<'a> {
    width: i32,
    height: i32,
    row_height: i32,
    frame: &'a mut [u8],
}

pub trait PixelDraw {
    fn draw_pixel(&mut self, x: i32, y: i32, color: &Color) -> bool;
}

impl<'a> PixelDraw for Framebuffer<'a> {
    fn draw_pixel(&mut self, x: i32, y: i32, color: &Color) -> bool {
        if self.width <= x || self.height <= y {
            return false;
        }

        let pos = (y / self.row_height * self.width) + x;
        let mut byte = self.frame[pos as usize];
        let pixel = y % self.row_height;

        byte = match color {
            Color::Black => byte | 1 << pixel,
            Color::White => byte & !(1 << pixel),
        };

        self.frame[pos as usize] = byte;

        true
    }
}

impl<'a> Framebuffer<'a> {
    pub fn new(width: i32, height: i32, frame: &'a mut [u8]) -> Option<Self> {
        Some(Framebuffer {
            width,
            height,
            row_height: 8,
            frame,
        })
    }

    pub fn get_byte(self, num: usize) -> u8 {
        self.frame[num]
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Color {
        let pos = (y / self.row_height * self.width) + x;
        let byte = self.frame[pos as usize];
        let pixel = y % self.row_height;

        if byte & (1 << pixel) != 0 {
            return Color::Black;
        } else {
            return Color::White;
        }
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }
}