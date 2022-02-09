use crate::coordinates::cvec::Vec2;
use super::coordinates::bounding_box::*;
use super::framebuffer::PixelDraw;
use super::framebuffer::*;

pub struct Line {
    bbox: BBox,
    color: Color,
    vertical: bool,
}

pub struct Rect {
    bbox: BBox,
    color: Color,
    filled: bool,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Circle {
    center: Vec2,
    r: u32,
    thickness: u32,
    color: Color,
}

pub struct Ellipse {
    center: Vec2,
    height: u32,
    width: u32,
    thickness: u32,
    color: Color,
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

impl Line {
    pub fn draw<C: PixelDraw>(&self, canvas: &mut C) {
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

impl Rect {
    pub fn draw<C: PixelDraw>(&self, canvas: &mut C) {
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

impl Circle {
    pub fn new(center: Vec2, r: u32, color: Color) -> Self {
        Self {
            center,
            r,
            thickness: 1,
            color,
        }
    }

    /// Creates a circle inscribed in bounding box. If the BBox is not a perfect sqare,
    /// the smallest side would be selected as a base for a square to inscribe.
    pub fn from_bbox(bbox: BBox, color: Color) -> Self {
        let r = (bbox.width().min(bbox.height()) / 2) as i32;
        let x = bbox.start.0 + r;
        let y = bbox.start.1 + r;

        Self {
            center: (x, y),
            r: r as u32,
            thickness: 1,
            color,
        }
    }

    pub fn filled(mut self, filled: bool) -> Self {
        if filled {
            self.thickness = self.r;
        } else {
            self.thickness = 1;
        }

        self
    }

    pub fn thickness(mut self, t: u32) -> Self {
        if t >= 1 {
            self.thickness = t;
        }
        if t >= self.r {
            self.thickness = self.r;
        }

        self
    }
}

impl Circle {
    pub fn draw<C: PixelDraw>(&self, canvas: &mut C) {
        let r = self.r as i32;
        let (x, y) = self.center;
        let t = self.thickness as i32;

        if r == 1 {
            canvas.draw_pixel(x, y, &self.color);
            return;
        }

        // FIXME: On small radiuses (<= 4) this draws a square
        for dx in (-r)..r {
            for dy in (-r)..r {
                if (dx * dx + dy * dy < (r * r) - 1)
                    && (dx * dx + dy * dy > ((r - t) * (r - t)) - 1)
                {
                    canvas.draw_pixel(dx + x, dy + y, &self.color);
                }
            }
        }
    }
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

    pub fn filled(mut self, filled: bool) -> Self {
        if filled {
            self.thickness = self.max_thickness();
        } else {
            self.thickness = 1;
        }

        self
    }

    pub fn max_thickness(&self) -> u32 {
        if self.height >= self.width {
            self.width
        }
        else {
            self.height
        }
    }

    pub fn thickness(mut self, t: u32) -> Self {
        self.thickness = t;
        if t <= self.height && t <= self.height {
            return self;
        }
        self.thickness = self.max_thickness();

        self
    }

}

impl Ellipse {
    pub fn draw<C: PixelDraw>(&self, canvas: &mut C) {
        let t = self.thickness;
        let height_int = self.height as i32;
        let width_int = self.width as i32;
        let height_sqr = (height_int * height_int);
        let width_sqr = (width_int * width_int);
        let internal_width_sqr = (width_int - t as i32) * (width_int - t as i32);
        let internal_height_sqr = (height_int - t as i32) * (height_int - t as i32);

        let (x, y) = self.center;

        for dx in -width_int..=width_int {
            for dy in -height_int..=height_int {
                if t == self.max_thickness() {
                    if dx * dx * height_sqr + dy * dy * width_sqr < height_sqr * width_sqr + 1 {
                        canvas.draw_pixel((dx + x) as i32, (dy + y) as i32, &self.color);
                    }
                } else {
                    if dx * dx * height_sqr + dy * dy * width_sqr < height_sqr * width_sqr + 1
                        && dx * dx * internal_height_sqr + dy * dy * internal_width_sqr > internal_width_sqr * internal_height_sqr {
                        canvas.draw_pixel(x + dx, y + dy, &self.color);
                    }
                }              
                
            }
        }

    }

}