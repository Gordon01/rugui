use super::Drawable;
use crate::coordinates::{bounding_box::BBox, cvec::Vec2};
use crate::framebuffer::{Color, PixelDraw};

#[derive(Eq, PartialEq, Debug)]
pub struct Circle {
    center: Vec2,
    r: u32,
    thickness: u32,
    color: Color,
}

impl Circle {
    pub fn new(center: Vec2, r: u32, color: Color) -> Self {
        Self {
            center,
            r,
            thickness: 1,
            color,
        }
    }

    /// Creates a circle inscribed in bounding box. If the BBox is not a perfect sqare,
    /// the smallest side would be selected as a base for a square to inscribe.
    pub fn from_bbox(bbox: BBox, color: Color) -> Self {
        let r = (bbox.width().min(bbox.height()) / 2) as i32;
        let x = bbox.start.0 + r;
        let y = bbox.start.1 + r;

        Self {
            center: (x, y),
            r: r as u32,
            thickness: 1,
            color,
        }
    }

    pub fn filled(mut self, filled: bool) -> Self {
        if filled {
            self.thickness = self.r;
        } else {
            self.thickness = 1;
        }

        self
    }

    pub fn thickness(mut self, t: u32) -> Self {
        if t >= 1 {
            self.thickness = t;
        }
        if t >= self.r {
            self.thickness = self.r;
        }

        self
    }
}

impl Drawable for Circle {
    fn draw<C: PixelDraw>(&self, canvas: &mut C) {
        let r = self.r as i32;
        let (x, y) = self.center;
        let t = self.thickness as i32;

        if r == 1 {
            canvas.draw_pixel(x, y, &self.color);
            return;
        }

        // FIXME: On small radiuses (<= 4) this draws a square
        for dx in (-r)..r {
            for dy in (-r)..r {
                if (dx * dx + dy * dy < (r * r) - 1)
                    && (dx * dx + dy * dy > ((r - t) * (r - t)) - 1)
                {
                    canvas.draw_pixel(dx + x, dy + y, &self.color);
                }
            }
        }
    }
}
