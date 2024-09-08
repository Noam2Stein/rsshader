use std::ops::Range;

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
    pub fn new(start: usize, end: usize) -> Self {
        assert!(end >= start);
        Self {
            start,
            end,
        }
    }
    #[inline(always)]
    pub fn sized(start: usize, len: usize) -> Self {
        Self {
            start,
            end: start + len,
        }
    }

    #[inline(always)]
    pub fn start(self) -> usize {
        self.start
    }
    #[inline(always)]
    pub fn end(self) -> usize {
        self.end
    }
    #[inline(always)]
    pub fn len(self) -> usize {
        self.end - self.start
    }

    #[inline(always)]
    pub fn connect(self, rhs: Self) -> Self {
        if self.len() == 0 {
            rhs
        }
        else if rhs.len() == 0 {
            self
        }
        else {
            Self {
                start: self.start.min(rhs.start),
                end: self.end.max(rhs.end)
            }
        }
    }
    #[inline(always)]
    pub fn first_byte(self) -> Self {
        if self.len() > 0 {
            Self::sized(self.start, 1)
        }
        else {
            Self::EMPTY
        }
    }
    #[inline(always)]
    pub fn last_byte(self) -> Self {
        if self.len() > 0 {
            Self::sized(self.end - 1, 1)
        }
        else {
            Self::EMPTY
        }
    }

    #[inline(always)]
    pub fn intersects(&self, rhs: &Self) -> bool {
        self.end > rhs.start && rhs.end > self.start
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