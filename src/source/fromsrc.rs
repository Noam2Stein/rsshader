use super::*;

pub trait FromSrc<'src>: Sized {
    type Err: Sized;
    fn from_src(s: &'src str) -> Result<Self, Self::Err>;
}
pub trait FromSrcSpan<'src>: Sized {
    type Err: Sized;
    fn from_src_span(src: &'src SrcFile, span: Span) -> Result<Self, Self::Err>;
}
impl<'src, T: FromSrc<'src>> FromSrcSpan<'src> for T {
    type Err = T::Err;
    fn from_src_span(src: &'src SrcFile, span: Span) -> Result<Self, Self::Err> {
        
    }
}