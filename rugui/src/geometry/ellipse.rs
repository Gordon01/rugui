use super::Drawable;
use crate::coordinates::{bounding_box::BBox, cvec::Vec2};
use crate::framebuffer::{Color, PixelDraw};

#[derive(PartialEq)]
pub struct Ellipse {
    center: Vec2,
    height: u32,
    width: u32,
    thickness: u32,
    color: Color,
}

impl Ellipse {
    pub fn new(width: u32, height: u32, center: Vec2, color: Color) -> Self {
        Self {
            center,
            height,
            width,
            thickness: 1,
            color,
        }
    }

    pub fn from_bbox(bbox: BBox, color: Color) -> Self {
        let width = (bbox.width() / 2) as u32;
        let height = (bbox.height() / 2) as u32;
        let x = bbox.start.0 + width as i32;
        let y = bbox.start.1 + height as i32;

        Self {
            center: (x, y),
            height,
            width,
            thickness: 1,
            color,
        }
    }

    pub fn filled(mut self, filled: bool) -> Self {
        if filled {
            self.thickness = self.max_thickness();
        } else {
            self.thickness = 1;
        }

        self
    }

    pub fn max_thickness(&self) -> u32 {
        self.height.min(self.width)
    }

    pub fn thickness(mut self, t: u32) -> Self {
        self.thickness = t.min(self.max_thickness());

        self
    }
}

impl Drawable for Ellipse {
    fn draw<C: PixelDraw>(&self, canvas: &mut C) {
        let t = self.thickness;
        let height_int = self.height as i32;
        let width_int = self.width as i32;
        let height_sqr = height_int * height_int;
        let width_sqr = width_int * width_int;
        let internal_width_sqr = (width_int - t as i32) * (width_int - t as i32);
        let internal_height_sqr = (height_int - t as i32) * (height_int - t as i32);

        let (x, y) = self.center;

        for dx in -width_int..=width_int {
            for dy in -height_int..=height_int {
                if dx * dx * height_sqr + dy * dy * width_sqr < height_sqr * width_sqr
                    && dx * dx * internal_height_sqr + dy * dy * internal_width_sqr
                        > internal_width_sqr * internal_height_sqr - 1
                {
                    canvas.draw_pixel(x + dx, y + dy, &self.color);
                }
            }
        }
    }
}
