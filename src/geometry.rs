
use super::framebuffer::*;

#[derive(PartialEq)]
pub enum Axis {
    X,
    Y,
    Both
}
pub struct Coordinates {
    pub x0: i32,
    pub x1: i32,
    pub y0: i32,
    pub y1: i32
}

impl Coordinates {
    pub fn new(x0: i32, x1: i32, y0: i32, y1: i32) -> Self {
        Coordinates {
            x0: x0,
            x1: x1,
            y0: y0,
            y1: y1
        }
    }

    pub fn from_relative(x0: i32, y0: i32, dx: i32, dy: i32) -> Self {
        Coordinates::new(x0, x0 + dx, y0, y0 + dy)
    }

    pub fn dx(&self) -> i32 {
        self.x1 - self.x0
    }

    pub fn dy(&self) -> i32 {
        self.y1 - self.y0
    }

    pub fn x_iter(&self) -> CoordinatesIterator {
        CoordinatesIterator {
            start: (self.x0, self.y0),
            end: (self.x1, self.y1),
            axis: Axis::X,
            index: 0,
        }
    }

    pub fn y_iter(&self) -> CoordinatesIterator {
        CoordinatesIterator {
            start: (self.x0, self.y0),
            end: (self.x1, self.y1),
            axis: Axis::Y,
            index: 0,
        }
    }
}

impl IntoIterator for Coordinates {
    type Item = i32;
    type IntoIter = CoordinatesIterator;

    fn into_iter(self) -> Self::IntoIter {
        CoordinatesIterator {
            start: (self.x0, self.y0),
            end: (self.x1, self.y1),
            axis: Axis::Both,
            index: 0,
        }
    }
}

pub struct CoordinatesIterator {
    start: (i32, i32),
    end: (i32, i32),
    axis: Axis,
    index: i32,
}

impl Iterator for CoordinatesIterator {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        let elem;
        if self.axis == Axis::X {
            elem = self.start.0 + self.index;
            if elem > self.end.0 {
                return None;
            }
        } else {
            elem = self.start.1 + self.index;
            if elem > self.end.1 {
                return None;
            }
        }

        self.index += 1;

        Some(elem)
    }
}

impl Framebuffer {
    pub fn draw_line(&mut self, cords: &Coordinates, color: &Color) -> bool {
        let mut D= 2*cords.dy() - cords.dx();
        let mut y = cords.y0;

        if cords.dx() == 0 {
            for y in cords.y_iter() {
                self.draw_pixel(cords.x0, y, &color);
            }

            return true;
        }
    
        for x in cords.x_iter() {
            self.draw_pixel(x, y, &color);
            if D > 0 {
                y += 1;
                D = D - 2*cords.dx();
            }
            D = D + 2*cords.dy();
        }

        true
    }

    pub fn draw_vertical_line(&mut self, x0: i32, y0: i32, y1: i32, color: &Color) -> bool {
        for y in y0..=y1 {
            self.draw_pixel(x0, y, &color);
        }

        true
    }

    pub fn draw_rect(&mut self, cords: &Coordinates, color: &Color) -> bool {
        self.draw_line(cords, color);
        self.draw_line(cords, color);
        self.draw_line(cords, color);
        self.draw_line(cords, color);

        true
    }

    pub fn draw_filled_rect(&mut self, cords: &Coordinates, color: &Color) -> bool {
        for x in cords.x_iter() {
            self.draw_vertical_line(x, cords.y0, cords.y1, color);
        }
        
        true
    }

    pub fn draw_circle(&mut self, x0: u32, y0: u32, r: u32, color: &Color) -> bool {
        let mut x = 0;
        let mut y = r as i32;
        let x0 = x0 as i32;
        let y0 = y0 as i32;

        let mut draw_i32 = |x, y, c| self.draw_pixel(x, y, c);

        // Calculate the initial decision 
        let mut decision = 3 - (2 * r as i32);

        while x <= y {
            // Put a pixel in each of the 8 segments of the circle.
            draw_i32(x0 + x, y0 + y, color);
            draw_i32(x0 + y, y0 + x, color);
            draw_i32(x0 - y, y0 + x, color);
            draw_i32(x0 - x, y0 + y, color);
            draw_i32(x0 - x, y0 - y, color);
            draw_i32(x0 - y, y0 - x, color);
            draw_i32(x0 + y, y0 - x, color);
            draw_i32(x0 + x, y0 - y, color);

            // Increment value of x.
            x += 1;

            if decision < 0 {
                // The next pixel will be drawn at (x + 1, y).
                decision = decision + (4 * x) + 6;
            }
            else {
                // The next pixel will be drawn at (x + 1, y - 1).
                decision = decision + 4 * (x - y) + 10;
                // Decrement the value of y.
                y -= 1;
            }
        }


        true
    }
}