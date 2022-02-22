use super::{line::ConstructMethod, line::Line, Drawable};
use crate::coordinates::cvec::Vec2;
use crate::framebuffer::{Color, PixelDraw};

pub struct Triangle {
    vertexes: (Vec2, Vec2, Vec2),
    color: Color,
}

impl Triangle {
    pub fn new(vertexes: (Vec2, Vec2, Vec2), color: Color) -> Self {
        Self { vertexes, color }
    }
}

impl Drawable for Triangle {
    fn draw<C: PixelDraw>(&self, canvas: &mut C) {
        let mut method = ConstructMethod::ByPoints {
            p1: self.vertexes.0,
            p2: self.vertexes.1,
        };
        Line::new(method, self.color).draw(canvas);
        method = ConstructMethod::ByPoints {
            p1: self.vertexes.1,
            p2: self.vertexes.2,
        };
        Line::new(method, self.color).draw(canvas);
        method = ConstructMethod::ByPoints {
            p1: self.vertexes.2,
            p2: self.vertexes.0,
        };
        Line::new(method, self.color).draw(canvas);
    }
}
