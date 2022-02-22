pub mod circle;
pub mod ellipse;
pub mod line;
pub mod rect;
pub mod triangle;

mod geometry_tests;

use crate::framebuffer::PixelDraw;

pub trait Drawable {
    fn draw<C: PixelDraw>(&self, canvas: &mut C);
}
