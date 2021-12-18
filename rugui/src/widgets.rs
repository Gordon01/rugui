use super::coordinates::bounding_box::*;
use super::framebuffer::*;
use super::geometry::{Line, Rect};

impl Framebuffer {
    pub fn progress_bar(&mut self, bbox: BBox, progress: u32, color: Color) -> bool {
        if progress > 100 {
            return false;
        }

        Rect::new(bbox, color).draw(self);
        let bbox = bbox.transform_both(-1);
        let position = bbox.start.0 + (bbox.width() as f32 * (progress as f32 / 100.0)) as i32;
        let (filled, empty) = bbox.split(Axis::X, position + 1);
        Rect::new_filled(filled, color).draw(self);
        Rect::new_filled(empty, Color::White).draw(self);

        true
    }

    pub fn scroller(
        &mut self,
        bbox: BBox,
        position: i32,
        width: i32,
        orient: Orientation,
        color: Color,
    ) -> bool {
        if orient == Orientation::Vertical {
            Rect::new_filled(bbox, Color::White).draw(self);

            let mid_x = bbox.width() / 2;
            Line::new_vertical(bbox, color);
            let bbox = BBox::new((mid_x as i32, bbox.start.1).into(), bbox.end);
            Line::new_vertical(bbox, color).draw(self);
            let position = bbox.start.1
                + ((bbox.end.1 - width - bbox.start.1) as f32 * (position as f32 / 100.0)) as i32;

            Rect::new(
                BBox::new(
                    (bbox.start.0, position).into(),
                    (bbox.end.0, position + width).into(),
                ),
                color,
            ).draw(self);
        }

        true
    }

    pub fn table(&mut self, bbox: &BBox, rows: i32, columns: i32, color: Color) -> bool {
        for x in bbox.iter_x().step_by(bbox.width() / columns as usize) {
            let bbox = BBox::new((x, bbox.start.1).into(), bbox.end);
            Line::new_vertical(bbox, color).draw(self);
        }

        for y in bbox.iter_y().step_by(bbox.height() / rows as usize) {
            Line::new(
                BBox::new((bbox.start.0, y).into(), (bbox.end.0, y).into()),
                color,
            ).draw(self);
        }

        true
    }
}
