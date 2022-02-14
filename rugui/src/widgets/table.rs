use crate::coordinates::bounding_box::*;
use crate::framebuffer::*;
use crate::geometry::line::Line;
use crate::geometry::*;

pub struct Table {
    bbox: BBox,
    rows: i32,
    columns: i32,
    color: Color,
}

impl Table {
    pub fn new(bbox: BBox, rows: i32, columns: i32, color: Color) -> Self {
        Self {
            bbox,
            rows,
            columns,
            color,
        }
    }
}

impl Table {
    pub fn draw<C: PixelDraw>(&self, canvas: &mut C) {
        let bbox = self.bbox;
        let color = self.color;

        for x in bbox.iter_x().step_by(bbox.width() / self.columns as usize) {
            let bbox = BBox::new((x, bbox.start.1), bbox.end);
            Line::new_vertical(bbox, color).draw(canvas);
        }

        for y in bbox.iter_y().step_by(bbox.height() / self.rows as usize) {
            Line::new(BBox::new((bbox.start.0, y), (bbox.end.0, y)), color).draw(canvas);
        }
    }
}
