use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident<'src> {
    srcslice: &'src SrcSlice,
}
impl<'src> Ident<'src> {
    #[inline(always)]
    pub const fn srcslice(&self) -> &'src SrcSlice {
        self.srcslice
    }
    #[inline(always)]
    pub const fn s(&self) -> &'src str {
        self.srcslice.s()
    }
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.srcslice.len()
    }
}
impl<'src> Display for Ident<'src> {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.s().fmt(f)
    }
}
impl<'src> Describe for Ident<'src> {
    fn desc(&self) -> Description {
        Description::quote(&self.s())
    }
}
impl<'src> TypeDescribe for Ident<'src> {
    fn type_desc() -> Description {
        Description::new("an ident")
    }
}
impl<'src> Spanned for Ident<'src> {
    fn span(&self, srcfile: &SrcFile) -> Span {
        self.srcslice.span(srcfile)
    }
}
impl<'src> FromSrc<'src> for Ident<'src> {
    fn from_src(srcslice: &'src SrcSlice) -> Result<Self, ErrorMessage> {
        if srcslice.len() == 0 {
            Err(errm::expected_found(Self::type_desc(), Description::an_empty_str()))
        }
        else if Keyword::STRS.contains(&&srcslice.s()) {
            Err(errm::is_and_thus_cant_be_used_as(srcslice.desc(), Keyword::type_desc(), Self::type_desc()))
        }
        else if srcslice.s().chars().next().map(|c| c.is_ascii_digit() || !c.is_alphanumeric() && c != '_').unwrap() ||
                srcslice.s().chars().any(|c| !c.is_alphanumeric() && c != '_') {
            Err(errm::expected_is_not(Self::type_desc(), srcslice.desc()))
        }
        else {
            Ok(
                Self {
                    srcslice,
                }
            )
        }
    }
}
impl<'src> FromSrcUnchecked<'src> for Ident<'src> {
    unsafe fn from_src_unchecked(srcslice: &'src SrcSlice) -> Self {
        Self {
            srcslice,
        }
    }
}
impl<'src> DefaultToken<'src> for Ident<'src> {
    fn default_token(srcfile: &'src SrcFile, span: Span) -> Self {
        Self {
            srcslice: &srcfile[span.with_len(0)]
        }
    }
}
impl<'src> ParseTokens<'src> for Ident<'src> {
    fn parse_tokens(tokens: &mut impl TokenIterator<'src>, errs: &mut Vec<Error>) -> Self {
        if let Some(token) = tokens.next(errs) {
            if let TokenTree::Ident(token) = token {
                token
            }
            else {
                let srcfile = tokens.srcfile();

                errs.push(Error::from_messages(token.span(srcfile), [
                    errm::expected_found(Self::type_desc(), token.token_type_desc())
                ]));

                Self::default_token(srcfile, token.span(srcfile))
            }
        }
        else {
            let srcfile = tokens.srcfile();

            errs.push(Error::from_messages(srcfile.span().end_span(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]));

            Self::default_token(srcfile, srcfile.span().end_span())
        }
    }
}
impl<'src> _ValidatedToken<'src> for Ident<'src> {

}