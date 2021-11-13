
use crate::coordinates::cvec::CVec;

use super::framebuffer::*;
use super::coordinates::bounding_box::*;

impl Framebuffer {
    pub fn draw_line(&mut self, bbox: BBox, color: &Color) -> bool {
        let mut delta = 2*bbox.height() as i32 - bbox.width() as i32;
        let mut y = bbox.start.y;

        if bbox.width() == 0 {
            for y in bbox.iter_y() {
                self.draw_pixel(bbox.start.x, y, &color);
            }

            return true;
        }
    
        for x in bbox.iter_x() {
            self.draw_pixel(x, y, &color);
            if delta > 0 {
                y += 1;
                delta = delta - 2*bbox.width() as i32;
            }
            delta = delta + 2*bbox.height() as i32;
        }

        true
    }

    pub fn draw_vertical_line(&mut self, start: CVec, y1: i32, color: &Color) -> bool {
        for y in start.y..=y1 {
            self.draw_pixel(start.x, y, &color);
        }

        true
    }

    pub fn draw_rect(&mut self, bbox: BBox, color: &Color) -> bool {
        let left_bottom = CVec::new(bbox.start.x, bbox.end.y);
        let right_top   = CVec::new(bbox.end.x, bbox.start.y);

        self.draw_line(BBox::new(bbox.start,  left_bottom), color);
        self.draw_line(BBox::new(left_bottom, bbox.end),    color);
        self.draw_line(BBox::new(right_top,   bbox.end),    color);
        self.draw_line(BBox::new(bbox.start,  right_top),   color);

        true
    }

    pub fn draw_filled_rect(&mut self, bbox: BBox, color: &Color) -> bool {
        for x in bbox.iter_x() {
            self.draw_vertical_line((x, bbox.start.y).into(), bbox.end.y, color);
        }
        
        true
    }

    pub fn draw_circle(&mut self, center: CVec, r: u32, color: &Color) -> bool {
        let mut x = 0;
        let mut y = r as i32;

        let mut draw_i32 = |x, y, c| self.draw_pixel(x, y, c);

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


        true
    }
}