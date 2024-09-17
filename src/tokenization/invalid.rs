use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvalidAny<'src> {
    s: &'src str,
    span_start: usize,
}
impl InvalidAny<'static> {
    pub const fn empty() -> Self {
        Self {
            s: "",
            span_start: 0,
        }
    }
}
impl<'src> Display for InvalidAny<'src> {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.s.fmt(f)
    }
}
impl<'src> Spanned for InvalidAny<'src> {
    #[inline(always)]
    fn span(&self) -> Span {
        Span::sized(self.span_start, self.s.len())
    }
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
impl<'src> FromSrc<'src> for InvalidAny<'src> {
    fn from_src(src: &'src SrcFile, span: Span) -> Option<Self> {
        Some(
            Self {
                s: &src[span],
                span_start: span.start(),
            }
        )
    }
}