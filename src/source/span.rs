use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Span {
    start: usize,
    len: usize,
}
impl Span {
    #[inline(always)]
    pub const fn start_end(start: usize, end: usize) -> Self {
        assert!(end >= start);
        Self {
            start,
            len: end - start,
        }
    }
    #[inline(always)]
    pub const fn sized(start: usize, len: usize) -> Self {
        Self {
            start,
            len,
        }
    }

    #[inline(always)]
    pub const fn start(&self) -> usize {
        self.start
    }
    #[inline(always)]
    pub const fn end(&self) -> usize {
        self.start + self.len
    }
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub const fn start_span(&self) -> Self {
        Self::sized(self.start, 0)
    }
    #[inline(always)]
    pub const fn end_span(&self) -> Self {
        Self::sized(self.end(), 0)
    }

    #[inline(always)]
    pub fn connect(&self, other: &Self) -> Self {
        if self.len() == 0 {
            *other
        }
        else if other.len() == 0 {
            *self
        }
        else {
            Self::start_end(self.start.min(other.start), self.end().max(other.end()))
        }
    }
    #[inline(always)]
    pub const fn first_byte(&self) -> Self {
        self.with_len(1)
    }
    #[inline(always)]
    pub const fn last_byte(&self) -> Self {
        Self::sized(self.end() - 1, 1)
    }

    #[inline(always)]
    pub const fn intersects(&self, other: &Self) -> bool {
        self.end() > other.start && other.end() > self.start
    }

    #[inline(always)]
    pub const fn with_len(&self, len: usize) -> Self {
        Self {
            start: self.start,
            len,
        }
    }

    #[inline(always)]
    pub const fn range(&self) -> Range<usize> {
        Range {
            start: self.start,
            end: self.end(),
        }
    }
}
impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Self::start_end(value.start, value.end)
    }
}
impl From<Span> for Range<usize> {
    fn from(value: Span) -> Self {
        value.range()
    }
}

pub trait Spanned {
    fn span(&self) -> Span;

    fn srcslice<'src>(&self, srcfile: &'src SrcFile) -> &'src SrcSlice {
        &srcfile[self.span()]
    }
    fn src_fmt(&self, srcfile: &SrcFile, f: &mut Formatter) -> fmt::Result {
        self.srcslice(srcfile).fmt(f)
    }
    fn src_to_string(&self, srcfile: &SrcFile) -> String {
        self.srcslice(srcfile).to_string()
    }
    fn src_desc(&self, srcfile: &SrcFile) -> Description {
        self.srcslice(srcfile).desc()
    }
}