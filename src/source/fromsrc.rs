use super::*;

pub trait FromSrc<'src>: Sized {
    fn from_src(src: &'src SrcFile, span: Span, errs: &mut Vec<Error>) -> Self;
}
pub trait FromSrcUnchecked<'src>: Sized {
    unsafe fn from_src_unchecked(src: &'src SrcFile, span: Span, _errs: &mut Vec<Error>) -> Self;
}