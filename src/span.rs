use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}
impl Span {
    pub const ZERO: Self = Self {
        start: 0,
        end: 0,
    };

    #[must_use]
    #[inline(always)]
    pub fn new(start: usize, end: usize) -> Self {
        assert!(end >= start);
        Self {
            start,
            end,
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn sized(start: usize, len: usize) -> Self {
        Self {
            start,
            end: start + len,
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn connect(a: Self, b: Self) -> Self {
        Self {
            start: a.start.min(b.start),
            end: a.end.max(b.end)
        }
    }

    pub fn end(self) -> Self {
        Self {
            start: self.end,
            end: self.end + 1,
        }
    }
}
impl From<Span> for Range<usize> {
    fn from(value: Span) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

pub trait Spanned {
    fn span(&self) -> Span;
}