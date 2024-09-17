use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident<'src> {
    s: &'src str,
}
impl<'src> Ident<'src> {
    pub fn from_str(s: &'src str) -> Result<Self, ErrorMessage> {
        if Keyword::STRS.contains(&s) {
            Err(errm::is_and_thus_cant_be_used_as(
                Description::quote(s),
                Keyword::type_desc(),
                Self::type_desc()
            ))
        }
        else if s.len() == 0 {
            Err(errm::expected_found(Self::type_desc(), Description::quote(s)))
        }
        else if s.chars().any(|c| !c.is_ascii_alphabetic() && !c.is_ascii_digit() && !['_'].contains(&c)) {
            Err(errm::is_not_because(Description::quote(s), Self::type_desc(), "it contains invalid chars"))
        }
        else if s.chars().next().unwrap().is_ascii_digit() {
            Err(errm::is_not_because(Description::quote(s), Self::type_desc(), "it starts with a digit"))
        }
        else {
            Ok(
                Self {
                    s,
                }
            )
        }
    }
    #[inline(always)]
    pub const unsafe fn from_str_unchecked(s: &'src str) -> Self {
        Self {
            s
        }
    }

    #[inline(always)]
    pub const fn str(&self) -> &str {
        &self.s
    }
}
impl<'src> Display for Ident<'src> {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.s.fmt(f)
    }
}
impl<'src> WrapSpannable for Ident<'src> {
    type Wrapper = SpannedIdent<'src>;
}
impl<'src> Describe for Ident<'src> {
    fn desc(&self) -> Description {
        Description::quote(&self.s)
    }
}
impl<'src> TypeDescribe for Ident<'src> {
    fn type_desc() -> Description {
        Description::new("an ident")
    }
}
impl<'src> UnspannedTokenTypeValidation<'src> for Ident<'src> {

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpannedIdent<'src> {
    inner: Ident<'src>,
    span_start: usize,
}
impl<'src> SpannedIdent<'src> {
    #[inline(always)]
    pub const fn str(&self) -> &str {
        &self.inner.str()
    }
}
impl<'src> Display for SpannedIdent<'src> {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
impl<'src> Spanned for SpannedIdent<'src> {
    fn span(&self) -> Span {
        Span::sized(self.span_start, self.inner.s.len())
    }
}
impl<'src> WrapSpanned for SpannedIdent<'src> {
    type Inner = Ident<'src>;
    fn inner(&self) -> &Self::Inner {
        &self.inner
    }
    fn into_inner(self) -> Self::Inner {
        self.inner
    }
}
impl<'src> FromSrc<'src> for SpannedIdent<'src> {
    fn from_src(src: &'src SrcFile, span: Span, errs: &mut Vec<Error>) -> Self {
        match Ident::from_str(&src[span]) {
            Ok(inner) => Self {
                inner,
                span_start: span.start()
            },
            Err(err) => {
                errs.push(Error::from_messages(span, [
                    err
                ]));

                Self {
                    inner: unsafe { Ident::from_str_unchecked("_") },
                    span_start: span.start()
                }
            }
        }
    }
}
impl<'src> FromSrcUnchecked<'src> for SpannedIdent<'src> {
    unsafe fn from_src_unchecked(src: &'src SrcFile, span: Span, _errs: &mut Vec<Error>) -> Self {
        Self {
            inner: Ident::from_str_unchecked(&src[span]),
            span_start: span.start(),
        }
    }
}
impl<'src> ParseTokens<'src> for SpannedIdent<'src> {
    fn parse_tokens(mut tokens: impl Iterator<Item = TokenTree<'src>>, src: &'src SrcFile, errs: &mut Vec<Error>) -> Self {
        if let Some(token) = tokens.next() {
            if let TokenTree::Ident(token) = token {
                token
            }
            else {
                errs.push(Error::from_messages(token.span(), [
                    errm::expected_found(Self::type_desc(), token.token_type_desc())
                ]));

                Self {
                    inner: Ident { s: "" },
                    span_start: token.span().start(),
                }
            }
        }
        else {
            errs.push(Error::from_messages(src.span().last_byte(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]));

            Self {
                inner: Ident { s: "" },
                span_start: 0,
            }
        }   
    }
}
impl<'src> SpannedTokenTypeValidation<'src> for SpannedIdent<'src> {

}