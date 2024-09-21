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
impl TokenDisplay for Ident {
    fn tt_to_string(&self, srcfile: &SrcFile) -> String {
        srcfile[self.span].s().to_string()
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

            Self::tt_default(tt.span())
        }
    }
}
impl TokenDefault for Ident {
    fn tt_default(span: Span) -> Self {
        Self {
            span: span.with_len(0),
        }
    }
}
impl _ValidatedTokenTree for Ident {

}