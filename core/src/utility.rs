use super::*;

pub trait Collision<T> {
    fn is_crossing(&self, rhs: &T) -> bool;
    fn contains(&self, inner: &T) -> bool;
}

#[inline]
pub fn is_crossing<T, U>(lhs: &T, rhs: &U) -> bool
where
    T: Collision<U>,
{
    lhs.is_crossing(rhs)
}

#[inline]
pub fn contains<T, U>(outer: &T, inner: &U) -> bool
where
    T: Collision<U>,
{
    outer.contains(inner)
}

impl<T> Collision<LogicalRect<T>> for LogicalPosition<T>
where
    T: num::Num + PartialOrd + Clone
{
    #[inline]
    fn is_crossing(&self, rhs: &LogicalRect<T>) -> bool {
        let lt = rhs.left_top();
        let rb = rhs.right_bottom();
        self.x >= lt.x && self.x <= rb.x && self.y >= lt.y && self.y <= rb.y
    }

    #[inline]
    fn contains(&self, _inner: &LogicalRect<T>) -> bool {
        false
    }
}

impl<T> Collision<LogicalPosition<T>> for LogicalRect<T>
where
    T: num::Num + PartialOrd + Clone
{
    #[inline]
    fn is_crossing(&self, rhs: &LogicalPosition<T>) -> bool {
        rhs.is_crossing(self)
    }

    #[inline]
    fn contains(&self, inner: &LogicalPosition<T>) -> bool {
        self.is_crossing(inner)
    }
}
