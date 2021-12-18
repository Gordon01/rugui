/// A vector of coordinates.
/// May represent coordinates of the point or size of rect.
pub type Vec2 = (i32, i32);

pub trait Vector2 {
    fn scale(&self, scaling: i32) -> Self;
    fn div(&self, scaling: i32) -> Self;
}

impl Vector2 for Vec2 {

    /// Scale a vector up or down. Positive `scaling` value means scaling up.
    fn scale(&self, scalar: i32) -> Self {
        let (x, y) = self;
        (x * scalar, y * scalar)
    }

    fn div(&self, scalar: i32) -> Self {
        let (x, y) = self;
        (x / scalar, y / scalar)
    }

}
