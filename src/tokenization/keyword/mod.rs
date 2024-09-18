use super::*;

#[inline(always)]
pub const fn keyword(s: &str) -> Keyword {
    Keyword::from_str_panic(s)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Keyword {
    id: u8,
}
impl Keyword {
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

    pub const fn from_str_panic(s: &str) -> Self {
        let mut position = 0;
        while position < Self::STRS.len() {
            //if s == Self::STRS[position] { // can't compare &str s in const context
            if {
                let a = s.as_bytes();
                let b = Self::STRS[position].as_bytes();

                if a.len() == b.len() {
                    let mut i = 0;
                    loop {
                        if a[i] != b[i] {
                            break false;
                        }
                        
                        i += 1;
                        if i >= a.len() {
                            break true;
                        }
                    }
                }
                else {
                    false
                }
            } {
                return Self {
                    id: position as u8,
                };
            }
    
            position += 1;
        }
    
        panic!("not a keyword")
    }

    #[inline(always)]
    pub const fn str(&self) -> &'static str {
        Self::STRS[self.id as usize]
    }
}
impl Display for Keyword {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.str().fmt(f)
    }
}
impl FromStr for Keyword {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(position) = Self::STRS.into_iter().position(|keyword| s == *keyword) {
            Ok(
                Self {
                    id: position as u8,
                }
            )
        }
        else {
            Err(format!("'{s}' is not {}", Self::type_desc()))
        }
    }
}
impl WrapSpannable for Keyword {
    type Wrapper = SpannedKeyword;
}
impl Describe for Keyword {
    #[inline(always)]
    fn desc(&self) -> Description {
        Description::quote(self.str())
    }
}
impl TypeDescribe for Keyword {
    #[inline(always)]
    fn type_desc() -> Description {
        Description::new("a keyword")
    }
}
impl<'src> ValidatedTokenType<'src> for Keyword {
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpannedKeyword {
    inner: Keyword,
    span_start: usize,
}
impl SpannedKeyword {
    pub const STRS: &'static [&'static str] = <Self as SpannedSpannable>::Unspanned::STRS;

    #[inline(always)]
    pub const fn str(&self) -> &'static str {
        self.inner.str()
    }
}
impl Display for SpannedKeyword {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}
impl Spanned for SpannedKeyword {
    #[inline(always)]
    fn span(&self) -> Span {
        Span::sized(self.span_start, self.str().len())
    }
}
impl WrapSpanned for SpannedKeyword {
    type Inner = Keyword;
    #[inline(always)]
    fn inner(&self) -> &Self::Inner {
        &self.inner
    }
    #[inline(always)]
    fn into_inner(self) -> Self::Inner {
        self.inner
    }
}
impl<'src> FromSrc<'src> for SpannedKeyword {
    fn from_src(src: &'src SrcFile, span: Span, errs: &mut Vec<Error>) -> Self {
        let s = &src[span];
        if let Some(position) = Self::STRS.iter().position(|keyword| s == *keyword) {
            Self {
                inner: Keyword {
                    id: position as u8,
                },
                span_start: span.start()
            }
        }
        else {
            errs.push(Error::from_messages(span, [
                errm::expected(Self::type_desc()),
                errm::is_not(Description::quote(s), Self::type_desc())
            ]));

            Self {
                inner: Keyword { id: 0 },
                span_start: span.start()
            }
        }
    }
}
impl<'src> FromSrcUnchecked<'src> for SpannedKeyword {
    #[inline(always)]
    unsafe fn from_src_unchecked(src: &'src SrcFile, span: Span, errs: &mut Vec<Error>) -> Self {
        Self::from_src(src, span, errs)
    }
}
impl<'src> FromRawToken<'src> for SpannedKeyword {
    #[inline(always)]
    unsafe fn from_raw_token(src: &'src SrcFile, raw_token: RawToken, errs: &mut Vec<Error>) -> Self {
        Self::from_src_unchecked(src, raw_token.span, errs)
    }
}
impl<'src> TryFromRawToken<'src> for SpannedKeyword {
    unsafe fn try_from_raw_token(src: &'src SrcFile, raw_token: RawToken, _errs: &mut Vec<Error>) -> Option<Self> {
        let s = &src[raw_token.span];
        Self::STRS.iter().position(|keyword| s == *keyword).map(|position|
            Self {
                inner: Keyword {
                    id: position as u8,
                },
                span_start: raw_token.span.start()
            }
        )
    }
}
impl<'src> ParseTokens<'src> for SpannedKeyword {
    fn parse_tokens(mut tokens: impl Iterator<Item = TokenTree<'src>>, src: &'src SrcFile, errs: &mut Vec<Error>) -> Self {
        if let Some(token) = tokens.next() {
            if let TokenTree::Keyword(token) = token {
                token
            }
            else {
                errs.push(Error::from_messages(token.span(), [
                    errm::expected_found(Self::type_desc(), token.token_type_desc())
                ]));

                Self { inner: Keyword { id: 0 }, span_start: token.span().start() }
            }
        }
        else {
            errs.push(Error::from_messages(src.span().last_byte(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]));

            Self { inner: Keyword { id: 0 }, span_start: src.span().last_byte().start() }
        }   
    }
}
impl<'src> ValidatedSpannedTokenType<'src> for SpannedKeyword {

}