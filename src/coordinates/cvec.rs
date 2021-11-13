/// A vector of coordinates.
/// May represent coordinates of the point or size of rect.
#[derive(Clone, Copy, Debug)]
pub struct CVec { 
    pub x: i32,
    pub y: i32
}

impl CVec {
    #[inline(always)]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Scale a vector up or down. Positive `scaling` value means scaling up.
    pub fn scale(&self, scaling: i32) -> Self {
        assert!(0 != scaling);

        let mut x = self.x;
        let mut y = self.y;

        if scaling > 0 {
            x *= scaling;
            y *= scaling;
        } else {
            x /= scaling;
            y /= scaling;
        }

        Self { x, y }
    }

    pub fn as_slice(&self) -> [f32; 2] {
        [self.x as f32, self.y as f32]
    }
}

impl From<CVec> for (usize, usize) {
    #[inline(always)]
    fn from(c: CVec) -> Self {
        (c.x as usize, c.y as usize)
    }
}

impl From<CVec> for [f32; 2] {
    #[inline(always)]
    fn from(c: CVec) -> Self {
        [c.x as f32, c.y as f32]
    }
}

impl From<(usize, usize)> for CVec {
    #[inline(always)]
    fn from(c: (usize, usize)) -> Self {
        Self { x: c.0 as i32, y: c.1 as i32}
    }
}

impl From<(i32, i32)> for CVec {
    #[inline(always)]
    fn from(c: (i32, i32)) -> Self {
        Self { x: c.0, y: c.1}
    }
}
