use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvalidAny<'src> {
    s: &'src str,
}
impl<'src> InvalidAny<'src> {
    pub const fn from_str(s: &'src str) -> Self {
        Self {
            s,
        }
    }
}
impl InvalidAny<'static> {
    #[inline(always)]
    pub const fn empty() -> Self {
        Self {
            s: "",
        }
    }
}
impl<'src> Display for InvalidAny<'src> {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.s.fmt(f)
    }
}
impl<'src> RawSpannable for InvalidAny<'src> {
    type Spanned = SpannedInvalidAny<'src>;
}
impl<'src> Describe for InvalidAny<'src> {
    #[inline(always)]
    fn desc(&self) -> Description {
        Description::quote(self.s)
    }
}
impl<'src> TypeDescribe for InvalidAny<'src> {
    #[inline(always)]
    fn type_desc() -> Description {
        Description::new("invalid any")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpannedInvalidAny<'src> {
    inner: InvalidAny<'src>,
    span_start: usize,
}
impl<'src> SpannedInvalidAny<'src> {
    pub fn from_src(src: &'src SrcFile, span: Span) -> Self {
        Self {
            inner: InvalidAny::from_str(&src[span]),
            span_start: span.start(),
        }
    }
}
impl SpannedInvalidAny<'static> {
    pub const fn empty() -> Self {
        Self {
            inner: InvalidAny::empty(),
            span_start: 0,
        }
    }
}
impl<'src> Display for SpannedInvalidAny<'src> {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
impl<'src> Spanned for SpannedInvalidAny<'src> {
    #[inline(always)]
    fn span(&self) -> Span {
        Span::sized(self.span_start, self.inner.s.len())
    }
}
impl<'src> RawSpannedSpannable for SpannedInvalidAny<'src> {
    type Inner = InvalidAny<'src>;
    fn inner(&self) -> &Self::Inner {
        &self.inner
    }
    fn into_inner(self) -> Self::Inner {
        self.inner
    }
}
impl<'src> FromSrc<'src> for SpannedInvalidAny<'src> {
    fn from_src(src: &'src SrcFile, span: Span) -> Option<Self> {
        Some(
            Self::from_src(src, span)
        )
    }
}