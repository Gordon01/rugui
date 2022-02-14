use super::{line::Line, Drawable};
use crate::coordinates::bounding_box::BBox;
use crate::framebuffer::{Color, PixelDraw};

pub struct Rect {
    bbox: BBox,
    color: Color,
    filled: bool,
}

impl Rect {
    pub fn new(bbox: BBox, color: Color) -> Self {
        Self {
            bbox,
            color,
            filled: false,
        }
    }

    pub fn new_filled(bbox: BBox, color: Color) -> Self {
        Self {
            bbox,
            color,
            filled: true,
        }
    }
}

impl Drawable for Rect {
    fn draw<C: PixelDraw>(&self, canvas: &mut C) {
        if self.filled {
            for x in self.bbox.iter_x() {
                let bbox = BBox::new((x, self.bbox.start.1), self.bbox.end);
                Line::new_vertical(bbox, self.color).draw(canvas);
            }

            return;
        }

        let left_bottom = (self.bbox.start.0, self.bbox.end.1);
        let right_top = (self.bbox.end.0, self.bbox.start.1);
        let bbox = self.bbox;
        let color = self.color;

        Line::new(BBox::new(bbox.start, left_bottom), color).draw(canvas);
        Line::new(BBox::new(left_bottom, bbox.end), color).draw(canvas);
        Line::new(BBox::new(right_top, bbox.end), color).draw(canvas);
        Line::new(BBox::new(bbox.start, right_top), color).draw(canvas);
    }
}
