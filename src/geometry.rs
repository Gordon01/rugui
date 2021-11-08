
use super::framebuffer::*;

#[derive(PartialEq)]
pub enum Axis {
    X,
    Y,
    Both
}

#[derive(Debug)]
pub struct Coordinates {
    pub start: (i32, i32),
    pub end: (i32, i32)
}

impl Coordinates {
    pub fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        Coordinates {
            start: start,
            end: end
        }
    }

    pub fn from_relative(start: (i32, i32), delta: (i32, i32)) -> Self {
        Coordinates::new(start, (start.0 + delta.0, start.1 + delta.1))
    }

    pub fn transform(&self, axis: Axis, delta: i32) -> Self {
        match axis {
            Axis::X    => Coordinates::new((self.start.0 - delta, self.start.1),
                                           (self.end.0 + delta, self.end.1)),
            Axis::Y    => Coordinates::new((self.start.0, self.start.1 - delta),
                                           (self.end.0, self.end.1 + delta)),
            Axis::Both => Coordinates::new((self.start.0 - delta, self.start.1 - delta),
                                           (self.end.0 + delta, self.end.1 + delta)),
        }
    }

    pub fn chop(&self, axis: Axis, at: i32) -> (Self, Self) {
        match axis {
            Axis::X    => (Coordinates::new(self.start,            (at - 1, self.end.1)),
                           Coordinates::new((at, self.start.1),     self.end)),
            Axis::Y    => (Coordinates::new(self.start,            (self.end.0, at - 1)),
                           Coordinates::new((self.start.0, at),     self.end)),
            // LOL sorry we probably need some 
            Axis::Both => (Coordinates::new((0, 0), (0, 0)), Coordinates::new((0, 0), (0, 0)))
        }
    }

    pub fn dx(&self) -> i32 {
        self.end.0 - self.start.0
    }

    pub fn dy(&self) -> i32 {
        self.end.1 - self.start.1
    }

    pub fn x_iter(&self) -> CoordinatesIterator {
        CoordinatesIterator {
            start: self.start,
            end: self.end,
            axis: Axis::X,
            index: 0,
        }
    }

    pub fn y_iter(&self) -> CoordinatesIterator {
        CoordinatesIterator {
            start: self.start,
            end: self.end,
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
            start: self.start,
            end: self.end,
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
        let mut y = cords.start.1;

        if cords.dx() == 0 {
            for y in cords.y_iter() {
                self.draw_pixel(cords.start.0, y, &color);
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
        self.draw_line(&Coordinates::new((cords.start.0, cords.start.1), (cords.start.0, cords.end.1)), color);
        self.draw_line(&Coordinates::new((cords.start.0, cords.end.1), (cords.end.0, cords.end.1)), color);
        self.draw_line(&Coordinates::new((cords.end.0, cords.start.1), (cords.end.0, cords.end.1)), color);
        self.draw_line(&Coordinates::new((cords.start.0, cords.start.1), (cords.end.0, cords.start.1)), color);

        true
    }

    pub fn draw_filled_rect(&mut self, cords: &Coordinates, color: &Color) -> bool {
        for x in cords.x_iter() {
            self.draw_vertical_line(x, cords.start.1, cords.end.1, color);
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