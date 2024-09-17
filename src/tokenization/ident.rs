use std::fmt::{self, Display, Formatter};

use crate::{desc::*, error::*, span::*, src::*, tokenization::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident<'src> {
    s: &'src str,
}
impl<'src> Ident<'src> {
    pub fn from_str(s: &'src str) -> Result<Self, String> {
        if Keyword::STRS.contains(&s) {
            Err(format!("'{s}' is a keyword and thus is an invalid ident"))
        }
        else if s.len() == 0 {
            Err(format!("'' is an invalid ident"))
        }
        else if s.chars().any(|c| !c.is_ascii_alphabetic() && !c.is_ascii_digit() && !['_'].contains(&c)) {
            Err(format!("'{s}' is an invalid ident because it contains invalid chars"))
        }
        else if s.chars().next().unwrap().is_ascii_digit() {
            Err(format!("'{s}' is an invalid ident because it starts with a digit"))
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
impl<'src> RawSpannable for Ident<'src> {
    type Spanned = SpannedIdent<'src>;
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
impl<'src> TokenTypeValidation<'src> for Ident<'src> {

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
impl<'src> RawSpannedSpannable for SpannedIdent<'src> {
    type Inner = Ident<'src>;
    fn inner(&self) -> &Self::Inner {
        &self.inner
    }
    fn into_inner(self) -> Self::Inner {
        self.inner
    }
}
impl<'src> FromSrc<'src> for SpannedIdent<'src> {
    fn from_src(src: &'src SrcFile, span: Span) -> Option<Self> {
        Ident::from_str(&src[span]).ok().map(|inner| Self {
            inner,
            span_start: span.start(),
        })
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
impl<'src> TokenTypeValidation<'src> for SpannedIdent<'src> {
    
}
impl<'src> SpannedTokenTypeValidation<'src> for SpannedIdent<'src> {

}