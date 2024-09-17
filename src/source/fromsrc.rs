pub trait FromSrc<'src>: Sized {
    fn from_src(src: &'src SrcFile, span: Span) -> Option<Self>;
}