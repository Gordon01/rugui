use super::coordinates::bounding_box::*;
use super::geometry::*;
use super::framebuffer::*;
use super::geometry::{Line, Rect};

pub struct ProgressBar {
    bbox: BBox,
    progress: u8,
    color: Color,
}

pub struct Scroller {
    bbox: BBox,
    position: i32,
    width: i32,
    orient: Orientation,
    color: Color,
}

pub struct Table {
    bbox: BBox,
    rows: i32,
    columns: i32,
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
