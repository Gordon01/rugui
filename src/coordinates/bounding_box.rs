use crate::coordinates::cvec::*;

#[derive(PartialEq)]
pub enum Axis {
    X,
    Y,
}

#[derive(Clone, Copy, Debug)]
pub struct BBox {
    pub start: CVec,
    pub end: CVec,
}

impl BBox {
    pub fn new(start: CVec, end: CVec) -> Self {
        Self { start, end }
    }

    pub fn from_relative(start: CVec, delta: CVec) -> Self {
        Self::new(start.into(), (start.x + delta.x, start.y + delta.y).into())
    }

    /// Make bounding box bigger or smaller in one dimension. 
    /// Positive `delta` value makes bounding box smaller.
    pub fn transform(&self, axis: Axis, delta: i32) -> Self {
        match axis {
            Axis::X => Self::new(
                (self.start.x - delta, self.start.y).into(),
                (self.end.x + delta, self.end.y).into()
            ),
            Axis::Y => Self::new(
                (self.start.x, self.start.y - delta).into(),
                (self.end.x, self.end.y + delta).into()
            )
        }
    }

    /// Make bounding box bigger or smaller in both dimension. 
    /// Positive `delta` value makes bounding box smaller.
    pub fn transform_both(&self, delta: i32) -> Self {
        Self::new(
            (self.start.x - delta, self.start.y - delta).into(),
            (self.end.x + delta, self.end.y + delta).into()
        )
    }

    /// Split bounding box by provided axis
    pub fn split(&self, axis: Axis, at: i32) -> (Self, Self) {
        match axis {
            Axis::X => (
                Self::new(self.start, (at - 1, self.end.y).into()),
                Self::new((at, self.start.y).into(), self.end),
            ),
            Axis::Y => (
                Self::new(self.start, (self.end.x, at - 1).into()),
                Self::new((self.start.x, at).into(), self.end),
            )
        }
    }

    #[inline(always)]
    pub fn width(&self) -> usize {
        (self.end.x - self.start.x) as usize
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        (self.end.y - self.start.y) as usize
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
    start: CVec,
    end: CVec,
    axis: Axis,
    index: i32,
}

impl Iterator for CoordinatesIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let elem;
        if self.axis == Axis::X {
            elem = self.start.x + self.index;
            if elem > self.end.x {
                return None;
            }
        } else {
            elem = self.start.y + self.index;
            if elem > self.end.y {
                return None;
            }
        }

        self.index += 1;

        Some(elem)
    }
}
