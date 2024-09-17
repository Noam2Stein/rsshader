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
    pub fn from_str(s: &str) -> Result<Self, String> {
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

    pub const fn literal_type_desc(&self) -> Description {
        match self {
            Self::Int(_) => IntLiteral::type_desc(),
            Self::Float(_) => FloatLiteral::type_desc(),
        }
    }
}
impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(literal) => literal.fmt(f),
            Self::Float(literal) => literal.fmt(f),
        }
    }
}
impl Spannable for Literal {
    type Spanned = SpannedLiteral;
}
impl Describe for Literal {
    fn desc(&self) -> Description {
        match self {
            Self::Int(literal) => literal.desc(),
            Self::Float(literal) => literal.desc(),
        }
    }
}
impl TypeDescribe for Literal {
    fn type_desc() -> Description {
        Description::new("a literal")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpannedLiteral<'src> {
    Int(SpannedIntLiteral),
    Float(SpannedFloatLiteral),
    InvalidAny(InvalidAny<'src>),
}
impl<'src> SpannedLiteral<'src> {
    pub const fn literal_type_desc(&self) -> Description {
        match self {
            Self::Int(_) => IntLiteral::type_desc(),
            Self::Float(_) => FloatLiteral::type_desc(),
        }
    }
}
impl<'src> Display for SpannedLiteral<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(literal) => literal.fmt(f),
            Self::Float(literal) => literal.fmt(f),
        }
    }
}
impl<'src> Spanned for SpannedLiteral<'src> {
    fn span(&self) -> Span {
        match self {
            Self::Int(literal) => literal.span(),
            Self::Float(literal) => literal.span(),
        }
    }
}
impl<'src> SpannedSpannable for SpannedLiteral<'src> {
    type Inner = Literal<'src>;
    fn into_unspanned(self) -> Self::Inner {
        match self {
            Self::Int(literal) => Literal::Int(literal.into_unspanned()),
            Self::Float(literal) => Literal::Float(literal.into_unspanned()),
        }
    }
}
impl FromTokens for SpannedLiteral {
    fn from_tokens(tokens: &mut TokenStreamIter, errs: &mut Vec<Error>) -> Self {
        if let Some(token) = tokens.next() {
            if let TokenTree::Literal(tokens) = token {
                *tokens
            }
            else {
                errs.push(Error::from_messages(token.span(), [
                    errm::expected_found(Self::type_desc(), token.token_type_desc())
                ]));

                Self::AnyDefault
            }
        }
        else {
            errs.push(Error::from_messages(tokens.stream().span().last_byte(), [
                errm::unexpected_end_of_file(),
                errm::expected(Self::type_desc())
            ]));
            
            Self::AnyDefault
        }
    }
}