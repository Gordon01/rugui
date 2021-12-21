use crate::coordinates::cvec::*;

#[derive(PartialEq)]
pub enum Axis {
    X,
    Y,
}

#[derive(Clone, Copy, Debug)]
pub struct BBox {
    pub start: Vec2,
    pub end: Vec2,
}

impl BBox {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self { start, end }
    }

    pub fn from_relative(start: Vec2, delta: Vec2) -> Self {
        Self::new(start, (start.0 + delta.0, start.1 + delta.1))
    }

    /// Make bounding box bigger or smaller in one dimension.
    /// Positive `delta` value makes bounding box smaller.
    pub fn transform(&self, axis: Axis, delta: i32) -> Self {
        match axis {
            Axis::X => Self::new(
                (self.start.0 - delta, self.start.1),
                (self.end.0 + delta, self.end.1),
            ),
            Axis::Y => Self::new(
                (self.start.0, self.start.1 - delta),
                (self.end.0, self.end.1 + delta),
            ),
        }
    }

    /// Make bounding box bigger or smaller in both dimension.
    /// Positive `delta` value makes bounding box smaller.
    pub fn transform_both(&self, delta: i32) -> Self {
        Self::new(
            (self.start.0 - delta, self.start.1 - delta),
            (self.end.0 + delta, self.end.1 + delta),
        )
    }

    /// Split bounding box by provided axis
    pub fn split(&self, axis: Axis, at: i32) -> (Self, Self) {
        match axis {
            Axis::X => (
                Self::new(self.start, (at - 1, self.end.1)),
                Self::new((at, self.start.1), self.end),
            ),
            Axis::Y => (
                Self::new(self.start, (self.end.0, at - 1)),
                Self::new((self.start.0, at), self.end),
            ),
        }
    }

    #[inline(always)]
    pub fn width(&self) -> usize {
        (self.end.0 - self.start.0) as usize
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        (self.end.1 - self.start.1) as usize
    }

    pub fn iter_x(&self) -> CoordinatesIterator {
        CoordinatesIterator {
            start: self.start,
            end: self.end,
            axis: Axis::X,
            index: 0,
        }
    }

    pub fn iter_y(&self) -> CoordinatesIterator {
        CoordinatesIterator {
            start: self.start,
            end: self.end,
            axis: Axis::Y,
            index: 0,
        }
    }
}

pub struct CoordinatesIterator {
    start: Vec2,
    end: Vec2,
    axis: Axis,
    index: i32,
}

impl Iterator for CoordinatesIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let elem;
        if self.axis == Axis::X {
            elem = self.start.0 + self.index;
            if elem > self.end.0 {
                return None;
            }
        } else {
            elem = self.start.1 + self.index;
            if elem > self.end.1 {
                return None;
            }
        }

        self.index += 1;

        Some(elem)
    }
}
