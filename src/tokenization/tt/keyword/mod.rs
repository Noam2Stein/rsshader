use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Keyword<'src> {
    id: u8,
    start: &'src SrcSliceStart,
}
impl<'src> Keyword<'src> {
    pub const STRS: &'static [&'static str] = &[
        "pub",
        "const",
        "fn",
        "struct",
        "use",
        "enum",
        "pipeline",
        "loop",
        "return",
        "break",
        "mod",
        "continue",
        "while",
        "for",
        "where",
        "as",
        "in",
    ];

    #[inline(always)]
    pub const fn s(&self) -> &'static str {
        Self::STRS[self.id as usize]
    }
}
impl<'src> Display for Keyword<'src> {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.s().fmt(f)
    }
}
impl<'src> Describe for Keyword<'src> {
    #[inline(always)]
    fn desc(&self) -> Description {
        Description::quote(self.s())
    }
}
impl<'src> TypeDescribe for Keyword<'src> {
    #[inline(always)]
    fn type_desc() -> Description {
        Description::new("a keyword")
    }
}
impl<'src> Spanned for Keyword<'src> {
    #[inline(always)]
    fn span(&self, srcfile: &SrcFile) -> Span {
        unsafe {
            self.start.with_len(self.s().len()).span(srcfile)
        }
    }
}
impl<'src> FromSrc<'src> for Keyword<'src> {
    fn from_src(srcslice: &'src SrcSlice) -> Result<Self, ErrorMessage> {
        if let Some(position) = Self::STRS.iter().position(|keyword| srcslice.s() == *keyword) {
            Ok(
                Self {
                    id: position as u8,
                    start: &srcslice.start()
                }
            )
        }
        else {
            Err(errm::expected_is_not(Self::type_desc(), srcslice.desc()))
        }
    }
}
impl<'src> FromSrcUnchecked<'src> for Keyword<'src> {
    unsafe fn from_src_unchecked(srcslice: &'src SrcSlice) -> Self {
        Self::from_src(srcslice).unwrap()
    }
}
impl<'src> DefaultToken<'src> for Keyword<'src> {
    fn default_token(srcfile: &'src SrcFile, span: Span) -> Self {
        Self {
            id: 0,
            start: &srcfile[span].start()
        }
    }
}
impl<'src> ParseTokens<'src> for Keyword<'src> {
    fn parse_tokens(tokens: &mut impl TokenIterator<'src>, errs: &mut Vec<Error>) -> Self {
        if let Some(token) = tokens.next(errs) {
            if let TokenTree::Keyword(token) = token {
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
impl<'src> _ValidatedToken<'src> for Keyword<'src> {
    
}