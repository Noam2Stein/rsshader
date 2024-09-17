use std::{fmt::{self, Display, Formatter}, str::FromStr};

use crate::{desc::*, error::*, span::*, tokenization::*};

mod int;
mod float;
pub use int::*;
pub use float::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Literal<'src> {
    Int(IntLiteral),
    Float(FloatLiteral),
    InvalidAny(InvalidAny<'src>),
}
impl<'src> Literal<'src> {
    pub fn from_str(s: &'src str) -> Result<Self, String> {
        if s.contains(".") {
            FloatLiteral::from_str(s).map(|literal| Self::Float(literal))
        }
        else {
            IntLiteral::from_str(s).map(|literal| Self::Int(literal))
        }
    }
    pub unsafe fn from_str_unchecked(s: &str) -> Self {
        if s.contains(".") {
            Self::Float(FloatLiteral::from_str_unchecked(s))
        }
        else {
            Self::Int(IntLiteral::from_str_unchecked(s))
        }
    }

    pub fn literal_type_desc(&self) -> Description {
        match self {
            Self::Int(_) => IntLiteral::type_desc(),
            Self::Float(_) => FloatLiteral::type_desc(),
            Self::InvalidAny(_) => InvalidAny::type_desc(),
        }
    }
}
impl<'src> Display for Literal<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(literal) => literal.fmt(f),
            Self::Float(literal) => literal.fmt(f),
            Self::InvalidAny(literal) => literal.fmt(f),
        }
    }
}
impl<'src> Spannable for Literal<'src> {
    type Spanned = SpannedLiteral<'src>;
}
impl<'src> Describe for Literal<'src> {
    fn desc(&self) -> Description {
        match self {
            Self::Int(literal) => literal.desc(),
            Self::Float(literal) => literal.desc(),
            Self::InvalidAny(literal) => literal.desc(),
        }
    }
}
impl<'src> TypeDescribe for Literal<'src> {
    fn type_desc() -> Description {
        Description::new("a literal")
    }
}
impl<'src> ParseTokens<'src> for Literal<'src> {
    fn parse_tokens(mut tokens: impl Iterator<Item = TokenTree<'src>>, src: &'src SrcFile, errs: &mut Vec<Error>) -> Self {
        if let Some(token) = tokens.next() {
            if let TokenTree::Literal(token) = token {
                token.into_unspanned()
            }
            else {
                errs.push(Error::from_messages(token.span(), [
                    errm::expected_found(Self::type_desc(), token.token_type_desc())
                ]));

                Self::InvalidAny(InvalidAny::from_str(&src[token.span()]))
            }
        }
        else {
            errs.push(Error::from_messages(src.span().last_byte(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]));
            
            Self::InvalidAny(InvalidAny::empty())
        }
    }
}
impl<'src> TokenTypeValidation<'src> for Literal<'src> {
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpannedLiteral<'src> {
    Int(SpannedIntLiteral),
    Float(SpannedFloatLiteral),
    InvalidAny(SpannedInvalidAny<'src>),
}
impl<'src> SpannedLiteral<'src> {
    pub fn literal_type_desc(&self) -> Description {
        match self {
            Self::Int(_) => IntLiteral::type_desc(),
            Self::Float(_) => FloatLiteral::type_desc(),
            Self::InvalidAny(_) => InvalidAny::type_desc(),
        }
    }
}
impl<'src> Display for SpannedLiteral<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(literal) => literal.fmt(f),
            Self::Float(literal) => literal.fmt(f),
            Self::InvalidAny(literal) => literal.fmt(f),
        }
    }
}
impl<'src> Spanned for SpannedLiteral<'src> {
    fn span(&self) -> Span {
        match self {
            Self::Int(literal) => literal.span(),
            Self::Float(literal) => literal.span(),
            Self::InvalidAny(literal) => literal.span(),
        }
    }
}
impl<'src> SpannedSpannable for SpannedLiteral<'src> {
    type Inner = Literal<'src>;
    fn into_unspanned(self) -> Self::Inner {
        match self {
            Self::Int(literal) => Literal::Int(literal.into_unspanned()),
            Self::Float(literal) => Literal::Float(literal.into_unspanned()),
            Self::InvalidAny(literal) => Literal::InvalidAny(literal.into_unspanned()),
        }
    }
}
impl<'src> Describe for SpannedLiteral<'src> {
    fn desc(&self) -> Description {
        match self {
            Self::Int(literal) => literal.desc(),
            Self::Float(literal) => literal.desc(),
            Self::InvalidAny(literal) => literal.desc(),
        }
    }
}
impl<'src> ParseTokens<'src> for SpannedLiteral<'src> {
    fn parse_tokens(mut tokens: impl Iterator<Item = TokenTree<'src>>, src: &'src SrcFile, errs: &mut Vec<Error>) -> Self {
        if let Some(token) = tokens.next() {
            if let TokenTree::Literal(tokens) = token {
                tokens
            }
            else {
                errs.push(Error::from_messages(token.span(), [
                    errm::expected_found(Self::type_desc(), token.token_type_desc())
                ]));

                Self::InvalidAny(SpannedInvalidAny::from_src(src, token.span()))
            }
        }
        else {
            errs.push(Error::from_messages(src.span().last_byte(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]));
            
            Self::InvalidAny(SpannedInvalidAny::empty())
        }   
    }
}
impl<'src> TokenTypeValidation<'src> for SpannedLiteral<'src> {
    
}
impl<'src> SpannedTokenTypeValidation<'src> for SpannedLiteral<'src> {

}