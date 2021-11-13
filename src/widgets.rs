use super::coordinates::bounding_box::*;
use super::framebuffer::*;

impl Framebuffer {
    pub fn progress_bar(&mut self, bbox: BBox, progress: u32, color: &Color) -> bool {
        if progress > 100 {
            return false;
        }

        self.draw_rect(bbox, color);
        let bbox = bbox.transform_both(-1);
        let position = bbox.start.x + (bbox.width() as f32 * (progress as f32 / 100.0)) as i32;
        let (filled, empty) = bbox.split(Axis::X, position + 1);
        self.draw_filled_rect(filled, color);
        self.draw_filled_rect(empty, &Color::White);

        true
    }

    pub fn scroller(
        &mut self,
        bbox: BBox,
        position: i32,
        width: i32,
        orient: Orientation,
        color: &Color,
    ) -> bool {
        if orient == Orientation::Vertical {
            self.draw_filled_rect(bbox, &Color::White);

            let mid_x = bbox.width() / 2;
            self.draw_vertical_line((mid_x as i32, bbox.start.y).into(), bbox.end.y, color);
            let position = bbox.start.y
                + ((bbox.end.y - width - bbox.start.y) as f32 * (position as f32 / 100.0)) as i32;

            self.draw_rect(
                BBox::new(
                    (bbox.start.x, position).into(),
                    (bbox.end.x, position + width).into(),
                ),
                color,
            );
        }

        true
    }

    pub fn table(&mut self, bbox: &BBox, rows: i32, columns: i32, color: &Color) -> bool {
        for x in bbox.iter_x().step_by(bbox.width() / columns as usize) {
            self.draw_vertical_line((x, bbox.start.y).into(), bbox.end.y, color);
        }

        for y in bbox.iter_y().step_by(bbox.height() / rows as usize) {
            self.draw_line(
                BBox::new((bbox.start.x, y).into(), (bbox.end.x, y).into()),
                color,
            );
        }

        true
    }
}
