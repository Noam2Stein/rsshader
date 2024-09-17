use std::{hash::Hash, ops::Range};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Span {
    start: usize,
    end: usize,
}
impl Span {
    pub const EMPTY: Self = Self {
        start: 0,
        end: 0,
    };

    #[inline(always)]
    pub const fn new(start: usize, end: usize) -> Self {
        assert!(end >= start);
        Self {
            start,
            end,
        }
    }
    #[inline(always)]
    pub const fn sized(start: usize, len: usize) -> Self {
        Self {
            start,
            end: start + len,
        }
    }

    #[inline(always)]
    pub const fn start(self) -> usize {
        self.start
    }
    #[inline(always)]
    pub const fn end(self) -> usize {
        self.end
    }
    #[inline(always)]
    pub const fn len(self) -> usize {
        self.end - self.start
    }

    #[inline(always)]
    pub fn connect(self, other: Self) -> Self {
        if self.len() == 0 {
            other
        }
        else if other.len() == 0 {
            self
        }
        else {
            Self {
                start: self.start.min(other.start),
                end: self.end.max(other.end)
            }
        }
    }
    #[inline(always)]
    pub const fn first_byte(self) -> Self {
        if self.len() > 0 {
            Self::sized(self.start, 1)
        }
        else {
            Self::EMPTY
        }
    }
    #[inline(always)]
    pub const fn last_byte(self) -> Self {
        if self.len() > 0 {
            Self::sized(self.end - 1, 1)
        }
        else {
            Self::EMPTY
        }
    }

    #[inline(always)]
    pub const fn intersects(&self, other: &Self) -> bool {
        self.end > other.start && other.end > self.start
    }

    #[inline(always)]
    pub fn into_range(self) -> Range<usize> {
        self.into()
    }
}
impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Self {
            start: value.start,
            end: value.end,
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