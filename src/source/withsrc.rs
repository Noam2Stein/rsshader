use super::*;

pub struct WithSrc<'src, T> {
    pub inner: T,
    pub srcfile: &'src SrcFile,
}
pub trait WithSrcExt<'src>: Sized {
    fn with_src(self, srcfile: &'src SrcFile) -> WithSrc<'src, Self>;
}
impl<'src, T> WithSrcExt<'src> for T {
    fn with_src(self, srcfile: &'src SrcFile) -> WithSrc<'src, Self> {
        WithSrc {
            inner: self,
            srcfile,
        }
    }
}

pub trait DisplayWithSrc {
    fn fmt_with_src(&self, f: &mut Formatter, srcfile: &SrcFile) -> fmt::Result;
}
impl<'src, T: DisplayWithSrc> Display for WithSrc<'src, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.inner.fmt_with_src(f, self.srcfile)
    }
}
impl<'src, T: Display> DisplayWithSrc for T {
    fn fmt_with_src(&self, f: &mut Formatter, _srcfile: &SrcFile) -> fmt::Result {
        self.fmt(f)
    }
}