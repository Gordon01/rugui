use super::Drawable;
use crate::coordinates::{bounding_box::BBox, cvec::Vec2};
use crate::framebuffer::{Color, PixelDraw};

pub enum ConstructMethod {
    FromBbox { bbox: BBox, vertical: bool },
    ByPoints { p1: Vec2, p2: Vec2 },
}

pub struct Line {
    method: ConstructMethod,
    color: Color,
}

impl Line {
    pub fn new(method: ConstructMethod, color: Color) -> Self {
        Self { method, color }
    }

    fn draw_from_bbox<C: PixelDraw>(&self, canvas: &mut C, bbox: BBox, vertical: bool) {
        if vertical {
            for y in bbox.start.1..=bbox.end.1 {
                canvas.draw_pixel(bbox.start.0, y, &self.color);
            }

            return;
        }

        let color = self.color;

        let mut delta = 2 * bbox.height() as i32 - bbox.width() as i32;
        let mut y = bbox.start.1;

        if bbox.width() == 0 {
            for y in bbox.iter_y() {
                canvas.draw_pixel(bbox.start.0, y, &color);
            }

            return;
        }

        for x in bbox.iter_x() {
            canvas.draw_pixel(x, y, &color);
            if delta > 0 {
                y += 1;
                delta -= 2 * bbox.width() as i32;
            }
            delta += 2 * bbox.height() as i32;
        }
    }

    fn draw_by_points<C: PixelDraw>(&self, canvas: &mut C, p1: Vec2, p2: Vec2) {
        let dx = (p2.0 - p1.0).abs();
        let sx = if p1.0 < p2.0 { 1 } else { -1 };
        let dy = -(p2.1 - p1.1).abs();
        let sy = if p1.1 < p2.1 { 1 } else { -1 };

        let mut err = dx + dy;
        let mut e2;

        let (mut x, mut y) = (p1.0, p1.1);

        loop {
            canvas.draw_pixel(x, y, &self.color);
            if x == p2.0 && y == p2.1 {
                break;
            }
            e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}

impl Drawable for Line {
    fn draw<C: PixelDraw>(&self, canvas: &mut C) {
        match self.method {
            ConstructMethod::FromBbox { bbox, vertical } => {
                self.draw_from_bbox(canvas, bbox, vertical)
            }
            ConstructMethod::ByPoints { p1, p2 } => self.draw_by_points(canvas, p1, p2),
        }
    }
}
