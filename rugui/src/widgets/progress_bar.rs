use crate::coordinates::bounding_box::*;
use crate::framebuffer::*;
use crate::geometry::rect::Rect;
use crate::geometry::*;

pub struct ProgressBar {
    bbox: BBox,
    progress: u8,
    color: Color,
}

impl ProgressBar {
    pub fn new(bbox: BBox, progress: u8, color: Color) -> Self {
        Self {
            bbox,
            progress,
            color,
        }
    }
}

impl Drawable for ProgressBar {
    fn draw<C: PixelDraw>(&self, canvas: &mut C) {
        Rect::new(self.bbox, self.color).draw(canvas);

        let bbox = self.bbox.transform_both(-1);
        let progress = self.progress.min(100);

        let position = bbox.start.0 + (bbox.width() as f32 * (progress as f32 / 100.0)) as i32;
        let (filled, empty) = bbox.split(Axis::X, position + 1);
        Rect::new_filled(filled, self.color).draw(canvas);
        Rect::new_filled(empty, Color::White).draw(canvas);
    }
}
