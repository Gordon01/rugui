use crate::coordinates::bounding_box::*;
use crate::framebuffer::*;
use crate::geometry::*;
use crate::geometry::{line::Line, rect::Rect};

pub struct Scroller {
    bbox: BBox,
    position: i32,
    width: i32,
    orient: Orientation,
    color: Color,
}

impl Scroller {
    pub fn new(bbox: BBox, position: i32, width: i32, orient: Orientation, color: Color) -> Self {
        Self {
            bbox,
            position,
            width,
            orient,
            color,
        }
    }
}

impl Drawable for Scroller {
    fn draw<C: PixelDraw>(&self, canvas: &mut C) {
        if self.orient == Orientation::Vertical {
            Rect::new_filled(self.bbox, Color::White).draw(canvas);
            let position = self.position.min(100);

            let mid_x = self.bbox.width() / 2;
            Line::new_vertical(self.bbox, self.color);
            let bbox = BBox::new((mid_x as i32, self.bbox.start.1), self.bbox.end);
            Line::new_vertical(bbox, self.color).draw(canvas);
            let position = bbox.start.1
                + ((bbox.end.1 - self.width - bbox.start.1) as f32 * (position as f32 / 100.0))
                    as i32;

            Rect::new(
                BBox::new(
                    (bbox.start.0, position),
                    (bbox.end.0, position + self.width),
                ),
                self.color,
            )
            .draw(canvas);
        }
    }
}
