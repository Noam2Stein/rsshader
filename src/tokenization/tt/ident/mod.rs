use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident {
    span: Span,
}
impl Ident {
    #[inline(always)]
    pub(in crate::tokenization) fn new(span: Span) -> Self {
        Self {
            span,
        }
    }
}
impl TypeDescribe for Ident {
    #[inline(always)]
    fn type_desc() -> Description {
        Description::new("an ident")
    }
}
impl Spanned for Ident {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}
impl DisplayWithSrc for Ident {
    fn fmt_with_src(&self, f: &mut Formatter, srcfile: &SrcFile) -> fmt::Result {
        if self.span.len() > 0 {
            srcfile[self.span].s().fmt(f)
        }
        else {
            write!(f, "___")
        }
    }
}
impl UnwrapTokenTree for Ident {
    #[inline(always)]
    fn unwrap_tt(tt: TokenTree, errs: &mut Vec<Error>) -> Self {
        if let TokenTree::Ident(tt) = tt {
            tt
        }
        else {
            errs.push(Error::from_messages(tt.span(), [
                errm::expected_found(Self::type_desc(), tt.token_type_desc())
            ]));

            unsafe {
                Self::tt_default(tt.span())
            }
        }
    }
}
impl TokenDefault for Ident {
    unsafe fn tt_default(span: Span) -> Self {
        Self {
            span: span.with_len(0),
        }
    }
}
impl SubToken for Ident {

}