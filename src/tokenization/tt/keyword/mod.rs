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
impl<'src> GetSrcSlice<'src> for Keyword<'src> {
    #[inline(always)]
    fn srcslice(&self) -> &'src SrcSlice {
        unsafe {
            self.start.with_len(self.s().len())
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
    fn default_token(srcslice: &'src SrcSlice) -> Self {
        Self {
            id: 0,
            start: &srcslice.start()
        }
    }
}
impl<'src> ParseTokens<'src> for Keyword<'src> {
    fn parse_tokens(parser: &mut impl TokenParser<'src>, errs: &mut Vec<Error<'src>>) -> Self {
        if let Some(token) = parser.next(errs) {
            if let TokenTree::Keyword(token) = token {
                token
            }
            else {
                errs.push(Error::from_messages(token.srcslice(), [
                    errm::expected_found(Self::type_desc(), token.token_type_desc())
                ]));

                Self::default_token(token.srcslice())
            }
        }
        else {
            errs.push(Error::from_messages(parser.end_srcslice(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]));

            Self::default_token(parser.end_srcslice().with_len(0))
        }
    }
}
impl<'src> _ValidatedToken<'src> for Keyword<'src> {
    
}