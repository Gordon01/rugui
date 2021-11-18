
use crate::coordinates::cvec::CVec;

use super::framebuffer::*;
use super::coordinates::bounding_box::*;
use super::framebuffer::PixelDraw;

pub struct Line {
    bbox: BBox,
    color: Color,
    vertical: bool
}

impl Line {
    pub fn new(bbox: BBox, color: Color) -> Self {
        Self { bbox, color, vertical: false }
    }

    pub fn new_vertical(bbox: BBox, color: Color) -> Self {
        Self { bbox, color, vertical: true }
    }
}

impl Line {
    pub fn draw<C: PixelDraw>(&self, canvas: &mut C) {
        if self.vertical {
            for y in self.bbox.start.y..=self.bbox.end.y {
                canvas.draw_pixel(self.bbox.start.x, y, &self.color);
            }

            return;
        }

        let bbox = self.bbox;
        let color = self.color;

        let mut delta = 2*bbox.height() as i32 - bbox.width() as i32;
        let mut y = bbox.start.y;

        if bbox.width() == 0 {
            for y in bbox.iter_y() {
                canvas.draw_pixel(bbox.start.x, y, &color);
            }

            return;
        }
    
        for x in bbox.iter_x() {
            canvas.draw_pixel(x, y, &color);
            if delta > 0 {
                y += 1;
                delta = delta - 2*bbox.width() as i32;
            }
            delta = delta + 2*bbox.height() as i32;
        }
    }
}

pub struct Rect {
    bbox: BBox,
    color: Color,
    filled: bool
}

impl Rect {
    pub fn new(bbox: BBox, color: Color) -> Self {
        Self { bbox, color, filled: false }
    }

    pub fn new_filled(bbox: BBox, color: Color) -> Self {
        Self { bbox, color, filled: true }
    }
}

impl Rect {
    pub fn draw<C: PixelDraw>(&self, canvas: &mut C) {
        if self.filled {
            for x in self.bbox.iter_x() {
                let bbox = BBox::new((x, self.bbox.start.y).into(), self.bbox.end);
                Line::new_vertical(bbox, self.color).draw(canvas);
            }

            return;
        }

        let left_bottom = CVec::new(self.bbox.start.x, self.bbox.end.y  );
        let right_top   = CVec::new(self.bbox.end.x,   self.bbox.start.y);
        let bbox = self.bbox;
        let color = self.color;

        Line::new(BBox::new(bbox.start,  left_bottom), color).draw(canvas);
        Line::new(BBox::new(left_bottom, bbox.end),    color).draw(canvas);
        Line::new(BBox::new(right_top,   bbox.end),    color).draw(canvas);
        Line::new(BBox::new(bbox.start,  right_top),   color).draw(canvas);
    }
}

pub struct Circle {
    center: CVec,
    r: u32,
    color: Color
}


impl Circle {
    pub fn new(center: CVec, r: u32, color: Color) -> Self {
        Self { center, r, color }
    }
}

impl Circle {
    pub fn draw<C: PixelDraw>(&self, canvas: &mut C) {
        let center = self.center;
        let r = self.r;
        let color = self.color;
        let mut x = 0;
        let mut y = r as i32;

        let mut draw_i32 = |x, y, c| canvas.draw_pixel(x, y, &c);

        // Calculate the initial decision 
        let mut decision = 3 - (2 * r as i32);

        while x <= y {
            // Put a pixel in each of the 8 segments of the circle.
            draw_i32(center.x + x, center.y + y, color);
            draw_i32(center.x + y, center.y + x, color);
            draw_i32(center.x - y, center.y + x, color);
            draw_i32(center.x - x, center.y + y, color);
            draw_i32(center.x - x, center.y - y, color);
            draw_i32(center.x - y, center.y - x, color);
            draw_i32(center.x + y, center.y - x, color);
            draw_i32(center.x + x, center.y - y, color);

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

    }
}
