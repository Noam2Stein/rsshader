use std::{hash::Hash, ops::Range};

use crate::{desc::{Describe, Description, TypeDescribe}, error::Error, src::{FromSrc, SrcFile}, tokenization::{FromTokens, TokenStreamIter}};

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
    pub fn intersects(&self, other: &Self) -> bool {
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

pub trait Spannable: Sized {
    type Spanned: SpannedSpannable<Inner = Self>;
}
pub trait SpannedSpannable: Sized {
    type Inner: Spannable<Spanned = Self>;
    fn into_unspanned(self) -> Self::Inner;
}
impl<T: SpannedSpannable<Inner: TypeDescribe>> TypeDescribe for T {
    #[inline(always)]
    fn type_desc() -> Description {
        T::Inner::type_desc()
    }
}

pub trait RawSpannable {
    type Spanned: RawSpannedSpannable<Inner = Self>;
}
pub trait RawSpannedSpannable {
    type Inner: RawSpannable<Spanned = Self>;
    fn inner(&self) -> &Self::Inner;
    fn into_inner(self) -> Self::Inner;
}
impl<T: RawSpannable> Spannable for T {
    type Spanned = T::Spanned;
}
impl<T: RawSpannedSpannable> SpannedSpannable for T {
    type Inner = T::Inner;
    #[inline(always)]
    fn into_unspanned(self) -> Self::Inner {
        T::into_inner(self)
    }
}
impl<'stream, 'src, T: Spannable<Spanned: FromTokens<'stream, 'src>>> FromTokens<'stream, 'src> for T {
    #[inline(always)]
    fn from_tokens(tokens: &mut TokenStreamIter<'stream, 'src>, errs: &mut Vec<Error>) -> Self {
        T::Spanned::from_tokens(tokens, errs).into_unspanned()
    }
}
impl<T: RawSpannedSpannable<Inner: Describe>> Describe for T {
    #[inline(always)]
    fn desc(&self) -> Description {
        self.inner().desc()
    }
}
impl<'src, T: RawSpannable<Spanned: FromSrc<'src>>> FromSrc<'src> for T {
    #[inline(always)]
    fn from_src(src: &'src SrcFile, span: Span) -> Option<Self> {
        T::Spanned::from_src(src, span).map(|output| output.into_unspanned())
    }
}