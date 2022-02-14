use super::Drawable;
use crate::coordinates::bounding_box::BBox;
use crate::framebuffer::{Color, PixelDraw};

pub struct Line {
    bbox: BBox,
    color: Color,
    vertical: bool,
}

impl Line {
    pub fn new(bbox: BBox, color: Color) -> Self {
        Self {
            bbox,
            color,
            vertical: false,
        }
    }

    pub fn new_vertical(bbox: BBox, color: Color) -> Self {
        Self {
            bbox,
            color,
            vertical: true,
        }
    }
}

impl Drawable for Line {
    fn draw<C: PixelDraw>(&self, canvas: &mut C) {
        if self.vertical {
            for y in self.bbox.start.1..=self.bbox.end.1 {
                canvas.draw_pixel(self.bbox.start.0, y, &self.color);
            }

            return;
        }

        let bbox = self.bbox;
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
}
